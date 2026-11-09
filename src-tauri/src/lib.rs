mod menu;
mod navigation;
mod settings;
mod tray;
mod window;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(window::ZoomState(std::sync::Mutex::new(1.0)))
        .setup(|app| {
            let handle = app.handle();
            let app_settings = settings::load(handle);

            if let Some(state) = app.try_state::<window::ZoomState>() {
                if let Ok(mut z) = state.0.lock() {
                    *z = app_settings.zoom_level.clamp(0.5, 3.0);
                }
            }

            match menu::build_menu(handle) {
                Ok(m) => {
                    let _ = app.set_menu(m);
                }
                Err(e) => eprintln!("[chatgpt-desktop] falha ao criar menu: {e}"),
            }

            if let Err(e) = tray::setup_tray(handle) {
                eprintln!("[chatgpt-desktop] falha ao configurar bandeja: {e}");
            }

            window::create_main_window(handle, &app_settings)
                .map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

            Ok(())
        })
        .on_menu_event(|app, event| {
            menu::handle_menu_event(app, event.id().as_ref());
        })
        .run(tauri::generate_context!())
        .expect("erro ao executar o aplicativo Tauri");
}
