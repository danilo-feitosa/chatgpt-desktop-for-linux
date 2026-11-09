use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};

use tauri::webview::{NewWindowResponse, PermissionKind, PermissionResponse};
use tauri::{AppHandle, Manager, Runtime, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::navigation;
use crate::settings::AppSettings;

pub const MAIN_WINDOW_LABEL: &str = "main";
pub const CHATGPT_URL: &str = "https://chatgpt.com";

static POPUP_COUNTER: AtomicU32 = AtomicU32::new(1);
static CHAT_WINDOW_COUNTER: AtomicU32 = AtomicU32::new(2);

#[derive(Default)]
pub struct ZoomState(pub Mutex<f64>);

fn next_popup_label() -> String {
    let id = POPUP_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("popup-{id}")
}

fn next_chat_label() -> String {
    let id = CHAT_WINDOW_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("chatgpt-{id}")
}

fn is_allowed_origin<R: Runtime>(webview: &WebviewWindow<R>) -> bool {
    webview
        .url()
        .ok()
        .and_then(|u| Url::parse(u.as_str()).ok())
        .as_ref()
        .is_some_and(navigation::should_allow_navigation)
}

fn permission_response<R: Runtime>(
    webview: &WebviewWindow<R>,
    kind: PermissionKind,
) -> PermissionResponse {
    let allowed = is_allowed_origin(webview);
    match kind {
        PermissionKind::Microphone
        | PermissionKind::Camera
        | PermissionKind::Geolocation
        | PermissionKind::Notifications => {
            if allowed {
                PermissionResponse::Allow
            } else {
                PermissionResponse::Deny
            }
        }
        _ => PermissionResponse::Default,
    }
}

fn configure_window<'a, R: Runtime>(
    builder: WebviewWindowBuilder<'a, R, AppHandle<R>>,
    app: &AppHandle<R>,
) -> WebviewWindowBuilder<'a, R, AppHandle<R>> {
    let nav_handle = app.clone();
    let new_window_handle = app.clone();
    let perm_window_handle = app.clone();

    builder
        .on_navigation(move |url| {
            if navigation::should_allow_navigation(url) {
                return true;
            }
            if let Some(host) = url.host_str() {
                eprintln!("[chatgpt-desktop] navegação bloqueada na WebView: {host}");
            } else {
                eprintln!("[chatgpt-desktop] navegação bloqueada na WebView: {url}");
            }
            navigation::open_external(&nav_handle, url);
            false
        })
        .on_new_window(move |url, features| {
            let is_about_blank = url.scheme() == "about";
            if is_about_blank || navigation::should_allow_navigation(&url) {
                let label = next_popup_label();
                let popup_app = new_window_handle.clone();
                let popup_builder = WebviewWindowBuilder::new(
                    &popup_app,
                    &label,
                    WebviewUrl::External(url.clone()),
                )
                .title("ChatGPT Desktop")
                .inner_size(520.0, 700.0)
                .resizable(true)
                .visible(true)
                .focused(true)
                .window_features(features)
                .on_document_title_changed(|window, title| {
                    let _ = window.set_title(&title);
                });

                let popup_builder = configure_window(popup_builder, &popup_app);
                match popup_builder.build() {
                    Ok(window) => NewWindowResponse::Create { window },
                    Err(e) => {
                        eprintln!("[chatgpt-desktop] falha ao criar popup: {e}");
                        if !is_about_blank {
                            navigation::open_external(&new_window_handle, &url);
                        }
                        NewWindowResponse::Deny
                    }
                }
            } else {
                if let Some(host) = url.host_str() {
                    eprintln!("[chatgpt-desktop] popup bloqueado (domínio não permitido): {host}");
                } else {
                    eprintln!("[chatgpt-desktop] popup bloqueado: {url}");
                }
                navigation::open_external(&new_window_handle, &url);
                NewWindowResponse::Deny
            }
        })
        .on_permission_request(move |webview, kind| {
            if let Some(window) = perm_window_handle.get_webview_window(webview.label()) {
                permission_response(&window, kind)
            } else {
                PermissionResponse::Default
            }
        })
}

fn build_chat_window<'a, R: Runtime>(
    app: &'a AppHandle<R>,
    label: &str,
    settings: &AppSettings,
    visible: bool,
) -> tauri::Result<WebviewWindow<R>> {
    let url = CHATGPT_URL
        .parse()
        .expect("CHATGPT_URL constante deve ser válida");
    let builder = WebviewWindowBuilder::new(app, label, WebviewUrl::External(url))
        .title("ChatGPT Desktop")
        .inner_size(1200.0, 800.0)
        .min_inner_size(480.0, 360.0)
        .resizable(true)
        .visible(visible);
    let builder = configure_window(builder, app);
    let window = builder.build()?;
    let _ = window.set_zoom(settings.zoom_level.clamp(0.5, 3.0));
    Ok(window)
}

pub fn create_main_window<R: Runtime>(
    app: &AppHandle<R>,
    settings: &AppSettings,
) -> tauri::Result<()> {
    if app.get_webview_window(MAIN_WINDOW_LABEL).is_some() {
        return Ok(());
    }
    let _ = build_chat_window(app, MAIN_WINDOW_LABEL, settings, !settings.start_minimized)?;
    Ok(())
}

pub fn create_additional_chat_window<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let label = next_chat_label();
    let settings = crate::settings::load(app);
    let window = build_chat_window(app, &label, &settings, true)?;
    let _ = window.set_focus();
    Ok(())
}

pub fn active_chat_window<R: Runtime>(app: &AppHandle<R>) -> Option<WebviewWindow<R>> {
    for (_, w) in app.webview_windows() {
        let l = w.label();
        if (l == MAIN_WINDOW_LABEL || l.starts_with("chatgpt-")) && w.is_focused().unwrap_or(false)
        {
            return Some(w);
        }
    }
    if let Some(w) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        return Some(w);
    }
    app.webview_windows().into_values().find(|w| {
        let l = w.label();
        l == MAIN_WINDOW_LABEL || l.starts_with("chatgpt-")
    })
}

pub fn set_zoom_all<R: Runtime>(app: &AppHandle<R>, zoom: f64) {
    let zoom = zoom.clamp(0.5, 3.0);
    for (_, w) in app.webview_windows() {
        let l = w.label();
        if l == MAIN_WINDOW_LABEL || l.starts_with("chatgpt-") || l.starts_with("popup-") {
            let _ = w.set_zoom(zoom);
        }
    }
    if let Some(state) = app.try_state::<ZoomState>() {
        if let Ok(mut z) = state.0.lock() {
            *z = zoom;
        }
    }
    let mut s = crate::settings::load(app);
    s.zoom_level = zoom;
    let _ = crate::settings::save(app, &s);
}

pub fn current_zoom<R: Runtime>(app: &AppHandle<R>) -> f64 {
    app.try_state::<ZoomState>()
        .and_then(|s| s.0.lock().ok().map(|z| *z))
        .unwrap_or(1.0)
}

pub fn show_main_window<R: Runtime>(app: &AppHandle<R>) {
    if let Some(w) = app.get_webview_window(MAIN_WINDOW_LABEL) {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    } else {
        let settings = crate::settings::load(app);
        let _ = create_main_window(app, &settings);
    }
}
