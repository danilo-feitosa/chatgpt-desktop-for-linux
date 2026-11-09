# Fase 5 — Segurança e Testes (revisão estática)
**Data:** 2026-11-08  
**Status:** Revisão de código/configuração concluída; testes de runtime/plataforma parcialmente dependem do usuário

## Revisão de segurança (código/configuração)

### Controles presentes
- `app.withGlobalTauri: false` em `src-tauri/tauri.conf.json` — evita expor `__TAURI__` globalmente.
- Capabilities (`src-tauri/capabilities/default.json`) com `permissions: []` para janelas conhecidas (`main`, `about`). Nenhum comando Tauri exposto ao conteúdo remoto.
- Sem `invoke_handler`/comandos Rust expostos ao frontend.
- `tauri-plugin-opener` usado apenas no lado nativo (Rust) para abrir URLs externas; não concedido via capability à WebView.
- Navegação remota filtrada no lado nativo (`on_navigation`, `on_new_window`) com allowlist; domínios fora da allowlist são bloqueados na WebView e encaminhados ao navegador padrão (política a validar/minimizar).
- `on_permission_request` concede microfone/câmera/geolocalização/notificações apenas para origens consideradas permitidas.
- Sem injeção de scripts no ChatGPT para reload/zoom/versão (usa APIs nativas `reload`/`set_zoom`).
- Sem armazenamento manual de credenciais/cookies/tokens/sessão; persistência delegada ao WebView.
- TLS/certificados não desabilitados no código do projeto.

### Achados / recomendações
| Item | Severidade | Observação |
|---|---|---|
| Allowlist de navegação ampla para OAuth | Média | Inclui `*.gstatic.com`, `*.googleusercontent.com`, `www.google.com`, `accounts.youtube.com` etc. Necessário validar o mínimo real e reduzir. Domínios amplos aumentam superfície se carregados como navegação principal. |
| Janelas dinâmicas `chatgpt-*`/`popup-*` sem capability explícita | Baixa (com postura atual) | Não recebem permissões concedidas; `withGlobalTauri` desabilitado. Revisar se futuras features locais exigirem capabilities específicas. |
| `on_new_window` cria popups in-app para domínios permitidos | Média (funcional/segurança) | Necessário para OAuth, mas popups tornam-se superfícies web controladas pela allowlist. Manter sem IPC e monitorar comportamento. |
| Logs de debug com hosts/URLs | Baixa | Úteis na Fase 3; podem vazar URLs em logs locais. Reduzir após estabilização. |
| `app.security.csp` nulo | Baixa | UI local é mínima; avaliar CSP para páginas locais se a UI local crescer. |

## Verificações executadas
- `cargo fmt --check` (após formatação) OK.
- `cargo check` (`src-tauri`) OK.
- `cargo clippy --all-targets -p chatgpt-desktop` executado; avisos menores (`collapsible_if`) — sem erros.
- `pnpm build` OK.

## Testes de runtime/plataforma (status)
| Teste | Status | Evidência |
|---|---|---|
| Janela independente / carga `chatgpt.com` | OK (usuário) | Relato Fase 3/4 |
| Navegação/chat/envio-recebimento | OK (usuário) | Relato |
| Upload arquivos/imagens | OK (usuário) | Relato |
| Login Google/Apple OAuth completo | **Pendente/falha reportada** | Navegações permitidas registradas; fluxo não conclui (diagnóstico visual incompleto) |
| Persistência de sessão | **Pendente** | Depende de login concluído |
| Áudio/microfone | **Falha no ambiente** | Timeout PipeWire reportado |
| Menu/atalhos/zoom/fullscreen/multi-janelas/tray | **Pendente validação** | Implementado; requer teste GUI |
| X11/Wayland, GNOME/KDE/XFCE | **Pendente** | Não executado neste agente |
| Falhas de conexão / encerramento / resoluções | **Pendente** | Não executado |

## Conclusão da Fase 5
A revisão estática não encontrou exposição óbvia de IPC/comandos Tauri ao conteúdo remoto nem bypass de TLS/injeção. Os principais riscos remanescentes são a **allowlist ampla para OAuth** (minimizar após evidência) e a **validação incompleta de OAuth/sessão/microfone/plataformas**, que dependem de testes manuais no ambiente alvo. Recomendado concluir diagnóstico de login antes da Fase 6 ou registrar claramente a limitação no release.
