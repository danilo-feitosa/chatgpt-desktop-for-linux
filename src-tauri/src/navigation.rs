use tauri::{Runtime, Url};
use tauri_plugin_opener::OpenerExt;

/// Decide se uma navegação deve permanecer na WebView.
///
/// Política inicial (Fase 2): conservadora e **a validar na Fase 3**.
/// - Permite domínios necessários ao ChatGPT/OpenAI.
/// - URLs externas são abertas no navegador padrão (quando possível) e bloqueadas na WebView.
///
/// Observação: não intercepta, lê ou modifica conteúdo/sessão.
pub fn should_allow_navigation(url: &Url) -> bool {
    let scheme = url.scheme();
    if scheme == "about" {
        // Permite about:blank/srcdoc usados por fluxos web; navegações reais
        // continuam restritas pela allowlist de hosts.
        return url.path() == "blank" || url.path() == "srcdoc" || url.as_str() == "about:blank";
    }

    let Some(host) = url.host_str() else {
        return false;
    };

    let host = host.to_ascii_lowercase();

    if scheme != "https" && scheme != "http" {
        return false;
    }

    // Domínios principais esperados. A lista final DEVE ser validada na Fase 3
    // com testes reais de login/conversação, sem coleta de dados sensíveis.
    // Inclui provedores de autenticação comuns usados pelo ChatGPT; manter mínima.
    let allowed = host == "chatgpt.com"
        || host.ends_with(".chatgpt.com")
        || host == "openai.com"
        || host.ends_with(".openai.com")
        || host == "auth.openai.com"
        || host == "auth0.openai.com"
        || host.ends_with(".auth0.com")
        || host == "oaistatic.com"
        || host.ends_with(".oaistatic.com")
        || host == "oaiusercontent.com"
        || host.ends_with(".oaiusercontent.com")
        // Provedores de login (validar/minimizar na Fase 3)
        || host == "accounts.google.com"
        || host == "accounts.google.com.br"
        || host == "accounts.youtube.com"
        || host == "consent.google.com"
        || host == "myaccount.google.com"
        || host == "www.google.com"
        || host.ends_with(".googleusercontent.com")
        || host.ends_with(".gstatic.com")
        || host == "appleid.apple.com"
        || host.ends_with(".appleid.apple.com")
        || host == "login.microsoftonline.com"
        || host == "login.live.com"
        || host.ends_with(".microsoftonline.com")
        || host.ends_with(".live.com");

    allowed
}

/// Abre URL externa no navegador padrão. Falhas são ignoradas para não quebrar a UI.
pub fn open_external<R: Runtime>(app: &tauri::AppHandle<R>, url: &Url) {
    if url.scheme() == "about" {
        return;
    }
    // Log de depuração (host/esquema) para diagnosticar bloqueios na Fase 3.
    if let Some(host) = url.host_str() {
        eprintln!("[chatgpt-desktop] abrindo externamente: {host}");
    } else {
        eprintln!("[chatgpt-desktop] abrindo externamente: {}", url.scheme());
    }
    let _ = app.opener().open_url(url.as_str(), None::<&str>);
}
