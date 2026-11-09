use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::{AppHandle, Manager, Runtime, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_opener::OpenerExt;

use crate::window;

pub fn build_menu<R: Runtime>(app: &AppHandle<R>) -> tauri::Result<Menu<R>> {
    // Arquivo
    let new_window = MenuItem::with_id(
        app,
        "file.new_window",
        "Nova janela",
        true,
        Some("CmdOrCtrl+N"),
    )?;
    let new_window_alt = MenuItem::with_id(
        app,
        "file.new_window_alt",
        "Nova janela",
        true,
        Some("CmdOrCtrl+Shift+N"),
    )?;
    let reload = MenuItem::with_id(
        app,
        "file.reload",
        "Recarregar ChatGPT",
        true,
        Some("CmdOrCtrl+R"),
    )?;
    let close = PredefinedMenuItem::close_window(app, Some("Fechar janela"))?;
    let quit = MenuItem::with_id(app, "file.quit", "Sair", true, Some("CmdOrCtrl+Q"))?;
    let file_menu = Submenu::with_items(
        app,
        "Arquivo",
        true,
        &[&new_window, &new_window_alt, &reload, &close, &quit],
    )?;

    // Editar (operações padrão do webview/sistema)
    let cut = PredefinedMenuItem::cut(app, Some("Recortar"))?;
    let copy = PredefinedMenuItem::copy(app, Some("Copiar"))?;
    let paste = PredefinedMenuItem::paste(app, Some("Colar"))?;
    let select_all = PredefinedMenuItem::select_all(app, Some("Selecionar tudo"))?;
    let edit_menu = Submenu::with_items(app, "Editar", true, &[&cut, &copy, &paste, &select_all])?;

    // Exibir
    let zoom_in = MenuItem::with_id(
        app,
        "view.zoom_in",
        "Zoom aumentar",
        true,
        Some("CmdOrCtrl+="),
    )?;
    let zoom_out = MenuItem::with_id(
        app,
        "view.zoom_out",
        "Zoom diminuir",
        true,
        Some("CmdOrCtrl+-"),
    )?;
    let zoom_reset = MenuItem::with_id(
        app,
        "view.zoom_reset",
        "Zoom padrão",
        true,
        Some("CmdOrCtrl+0"),
    )?;
    let fullscreen = MenuItem::with_id(app, "view.fullscreen", "Tela cheia", true, Some("F11"))?;
    let view_menu = Submenu::with_items(
        app,
        "Exibir",
        true,
        &[&zoom_in, &zoom_out, &zoom_reset, &fullscreen],
    )?;

    // Ajuda
    let about = MenuItem::with_id(app, "help.about", "Sobre o aplicativo", true, None::<&str>)?;
    let version = MenuItem::with_id(app, "help.version", "Verificar versão", true, None::<&str>)?;
    let official = MenuItem::with_id(
        app,
        "help.official",
        "Abrir site oficial do ChatGPT",
        true,
        None::<&str>,
    )?;
    let help_menu = Submenu::with_items(app, "Ajuda", true, &[&about, &version, &official])?;

    Menu::with_items(app, &[&file_menu, &edit_menu, &view_menu, &help_menu])
}

pub fn handle_menu_event<R: Runtime>(app: &AppHandle<R>, id: &str) {
    match id {
        "file.new_window" | "file.new_window_alt" => {
            let _ = window::create_additional_chat_window(app);
        }
        "file.reload" => {
            if let Some(w) = window::active_chat_window(app) {
                // Recarrega sem injetar scripts na página.
                let _ = w.reload();
            }
        }
        "file.quit" => {
            app.exit(0);
        }
        "view.zoom_in" => {
            let z = (window::current_zoom(app) + 0.1).min(3.0);
            window::set_zoom_all(app, z);
        }
        "view.zoom_out" => {
            let z = (window::current_zoom(app) - 0.1).max(0.5);
            window::set_zoom_all(app, z);
        }
        "view.zoom_reset" => {
            window::set_zoom_all(app, 1.0);
        }
        "view.fullscreen" => {
            if let Some(w) = window::active_chat_window(app) {
                let current = w.is_fullscreen().unwrap_or(false);
                let _ = w.set_fullscreen(!current);
            }
        }
        "help.about" | "help.version" => {
            if let Some(w) = app.get_webview_window("about") {
                let _ = w.set_focus();
            } else {
                let _ =
                    WebviewWindowBuilder::new(app, "about", WebviewUrl::App("index.html".into()))
                        .title("Sobre - ChatGPT Desktop")
                        .inner_size(560.0, 420.0)
                        .resizable(false)
                        .build();
            }
        }
        "help.official" => {
            let _ = app.opener().open_url("https://chatgpt.com", None::<&str>);
        }
        _ => {}
    }
}
