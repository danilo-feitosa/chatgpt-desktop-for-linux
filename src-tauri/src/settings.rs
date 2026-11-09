use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

/// Configurações locais não sensíveis do aplicativo.
/// Nunca armazena credenciais, cookies, tokens ou conteúdo de conversas.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub zoom_level: f64,
    pub start_minimized: bool,
    pub minimize_to_tray: bool,
    pub close_behavior: CloseBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloseBehavior {
    Ask,
    Exit,
    MinimizeToTray,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            zoom_level: 1.0,
            start_minimized: false,
            minimize_to_tray: false,
            close_behavior: CloseBehavior::Ask,
        }
    }
}

impl Default for CloseBehavior {
    fn default() -> Self {
        Self::Ask
    }
}

fn settings_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| format!("falha ao obter diretório de configuração: {e}"))?;
    fs::create_dir_all(&dir)
        .map_err(|e| format!("falha ao criar diretório de configuração: {e}"))?;
    Ok(dir.join("settings.json"))
}

pub fn load<R: Runtime>(app: &AppHandle<R>) -> AppSettings {
    let Ok(path) = settings_path(app) else {
        return AppSettings::default();
    };
    let Ok(content) = fs::read_to_string(path) else {
        return AppSettings::default();
    };
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save<R: Runtime>(app: &AppHandle<R>, settings: &AppSettings) -> Result<(), String> {
    let path = settings_path(app)?;
    let content = serde_json::to_string_pretty(settings)
        .map_err(|e| format!("falha ao serializar configurações: {e}"))?;
    fs::write(path, content).map_err(|e| format!("falha ao salvar configurações: {e}"))
}
