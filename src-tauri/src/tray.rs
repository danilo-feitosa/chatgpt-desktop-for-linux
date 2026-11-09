use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, Runtime};

use crate::window;

pub fn setup_tray<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<()> {
    let open = MenuItem::with_id(app, "tray.open", "Abrir ChatGPT", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "tray.hide", "Ocultar janela", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "tray.quit", "Sair", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&open, &hide, &quit])?;

    let mut builder = TrayIconBuilder::with_id("main-tray")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .tooltip("ChatGPT Desktop")
        .on_menu_event(|app, event| match event.id.as_ref() {
            "tray.open" => window::show_main_window(app),
            "tray.hide" => {
                if let Some(w) = app.get_webview_window(window::MAIN_WINDOW_LABEL) {
                    let _ = w.hide();
                }
            }
            "tray.quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                window::show_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    // A bandeja pode não ser suportada em todos os ambientes (especialmente
    // algumas sessões Wayland/GNOME). Falha não deve impedir o app de iniciar.
    match builder.build(app) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("[chatgpt-desktop] bandeja do sistema indisponível: {e}");
            Ok(())
        }
    }
}
