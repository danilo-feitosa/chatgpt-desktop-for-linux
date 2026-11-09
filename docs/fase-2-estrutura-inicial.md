# Fase 2 — Estrutura Inicial
**Data:** 2026-11-08  
**Status:** Concluída (estrutura criada; compilação bloqueada por dependências de sistema faltantes)

## Implementado
- Scaffold Tauri 2 (template `vanilla-ts`, `--tauri-version 2`) mesclado ao workspace, preservando `docs/`.
- Identidade do app atualizada: **ChatGPT Desktop** (`com.danilo.chatgptdesktop`), descrição deixando claro que **não é oficial/OpenAI**.
- `tauri.conf.json`: `withGlobalTauri: false`, janelas criadas programaticamente (array `windows` vazio), metadados de bundle atualizados.
- Capability da janela principal (`main`): **sem permissões** (`permissions: []`) para não expor IPC/comandos Tauri ao conteúdo remoto.
- Estrutura modular Rust: `window.rs`, `navigation.rs`, `settings.rs`, `menu.rs` (placeholder), `tray.rs` (placeholder).
- Janela principal configurada para carregar `https://chatgpt.com` (`WebviewUrl::External`) com título/tamanho iniciais.
- Política inicial de navegação no lado nativo (`navigation.rs`): allowlist para domínios ChatGPT/OpenAI/recursos relacionados esperados; demais navegações bloqueadas na WebView (abertura externa via opener a refinar/validar). **A validar na Fase 3.**
- Configurações locais não sensíveis (`settings.rs`): zoom inicial, comportamento de início/fechamento/bandeja (persistência JSON em diretório de config do app). **Sem dados sensíveis.**
- UI local mínima (Vite/TS/CSS) com identidade visual e aviso de não oficialidade.
- Ícones próprios gerados (SVG + PNG/ICO; ICNS pode requerer ajuste em ambiente macOS) — marca original (balão de chat), sem logotipos oficiais.
- `packaging/chatgpt-desktop.desktop` e README atualizados.

## Decisões técnicas
- Janela `main` = conteúdo remoto ChatGPT, sem capabilities Tauri.
- Plugin `opener` mantido para uso nativo (Rust), não exposto à janela remota.
- `greet`/invoke handlers de demonstração removidos.
- Menu/tray como módulos placeholder; implementação funcional na Fase 4.
- Zoom persistente, restauração de geometria e comportamento de fechamento avançado: Fase 4.

## Problemas / bloqueios
- `cargo check` falhou no ambiente atual: `pkg-config` não encontrado e dependências de sistema/dev (WebKitGTK 4.1, GTK, dbus, libsoup etc.) não instaladas/indisponíveis para compilação. É necessário instalar pacotes com `sudo` (ver README/Fase 1) antes de compilar/executar `tauri dev`.

## Testes executados
- `pnpm install` (sucesso).
- `cargo fmt --all` em `src-tauri` (sucesso).
- `cargo check` em `src-tauri` (**falhou** por dependências de sistema/pkg-config ausentes).
- Verificação de estrutura de arquivos e conteúdo de configuração/capabilities.

## Próximos passos (após aprovação)
1. Instalar dependências de sistema `-dev` (requer `sudo`).
2. Validar `cargo check` e `pnpm tauri dev`.
3. Iniciar **Fase 3** (PoC): carregar `chatgpt.com`, testar login, persistência de sessão, navegação, upload/mídia e ajustar política de domínios/navegação com segurança.
