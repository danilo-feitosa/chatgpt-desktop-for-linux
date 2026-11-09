# Fase 3 — Integração ChatGPT (status)
**Data:** 2026-11-08  
**Status:** Parcialmente validada em runtime (há limitações a tratar)

## Ambiente
- Dependências de sistema instaladas pelo usuário (compilação passou após instalação de `pkg-config`/`libdbus-1-dev`/WebKitGTK/GTK/libsoup etc.).
- `pnpm tauri dev` compila e executa o binário.

## Validação reportada pelo usuário
| Item | Resultado |
|---|---|
| Janela abre independentemente | OK |
| `chatgpt.com` carrega | OK |
| Navegação entre conversas / enviar/receber mensagens | OK (conforme relato) |
| Upload de arquivos/imagens | OK |
| Upload/áudio (microfone) | **Falha** — terminal mostra timeout PipeWire (`Unable to open pipewire remote. Error: O tempo limite foi alcançado`); também depende de portal/serviços de áudio do sistema |
| Login com Google | **Não conclui** no app; fluxo navega (permitido) por `auth.openai.com` → `accounts.google.com` → `accounts.youtube.com` → `accounts.google.com` e para sem completar (evidência: logs; falta detalhe visual do ponto de falha) |
| Login com Apple | **Não conclui** no app; navega para `appleid.apple.com` e para (evidência: logs; falta detalhe visual) |
| Sessão persistente após login | **Não validada** (login não concluído nos testes reportados) |
| Links externos / bloqueios | Política ativa; domínios de login iterativamente permitidos com base em logs de debug |

## Implementado nesta fase (código)
- Correção de tipo: `navigation::open_external<R: Runtime>`.
- `on_navigation` com abertura externa para domínios fora da allowlist e logs de diagnóstico.
- `on_new_window`: popups `about:blank` e domínios permitidos criados como janelas Tauri seguras (`visible/focused`, sem capabilities concedidas à `main`; navegação filtrada nas popups); domínios não permitidos abrem no navegador padrão.
- `on_permission_request`: concede `Microphone/Camera/Geolocation/Notifications` apenas para origens permitidas pelo app (comportamento real no Linux depende de WebKitGTK/portais/PipeWire).
- Allowlist expandida para domínios ChatGPT/OpenAI/OAuth comuns (Google/Apple/Microsoft, `gstatic`, `googleusercontent`, `accounts.youtube.com`, `www.google.com` etc.) — **deve ser revisada/minimizada com base em evidência**.
- Tratamento de `about:blank/srcdoc` para evitar abertura indevida no navegador.

## Limitações / pendências
- **OAuth Google/Apple não concluindo**: navegação permitida, mas fluxo não finaliza (causa exata ainda não confirmada — pode envolver WebKitGTK/cookies/storage/redirects, comportamento do provedor, ou requisito de popup/comunicação específica). Necessita diagnóstico visual detalhado e, se for limitação do WebKitGTK, decisão técnica documentada.
- **Microfone**: bloqueio provável no ambiente (PipeWire/portal); validar após corrigir sessão de áudio do usuário e confirmar comportamento no WebKitGTK.
- **Sessão persistente**: validar após conseguir login bem-sucedido.
- **Reutilizar sessão Google do navegador**: **não suportado/não implementado** (perfis isolados; extração/cópia de cookies/sessão é proibida pelos requisitos de segurança).
- Logs de debug (`navegação permitida`) estão verbosos — reduzir após estabilização.

## Próximos passos possíveis
1. Coletar evidência visual/terminal do ponto exato de falha no OAuth (Google/Apple) e testar login por e-mail/senha, se disponível.
2. Ajustar política/comportamento do webview com segurança, se o diagnóstico indicar causa corrigível.
3. Se houver limitação comprovada do WebKitGTK para OAuth do ChatGPT, documentar e propor alternativa segura **sem** scraping/automação/bypass (aguardando decisão).
4. Avançar para **Fase 4** (menu, atalhos, zoom, fullscreen, multi-janelas, tray, configurações) em paralelo apenas se não conflitar com a depuração da Fase 3 — mediante aprovação.
