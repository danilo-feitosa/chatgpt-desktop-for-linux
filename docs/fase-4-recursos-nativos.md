# Fase 4 — Recursos Nativos (status)
**Data:** 2026-11-08  
**Status:** Implementação inicial (compila; validação runtime pendente pelo usuário)

## Implementado
- **Menu nativo** (Tauri 2): Arquivo (Nova janela, Recarregar, Fechar janela, Sair), Editar (Recortar/Copiar/Colar/Selecionar tudo via itens predefinidos), Exibir (Zoom +/-, Zoom padrão, Tela cheia/F11), Ajuda (Sobre, Verificar versão, Abrir site oficial do ChatGPT).
- **Atalhos (aceleradores de menu)**: `Ctrl/Cmd+N`, `Ctrl/Cmd+Shift+N` (nova janela), `Ctrl/Cmd+R` (recarregar), `Ctrl/Cmd+Q` (sair), `Ctrl/Cmd+=`/`-`/`0` (zoom), `F11` (tela cheia). Podem conflitar com atalhos do site dependendo do foco/plataforma — validar.
- **Zoom**: aplicado via `WebviewWindow::set_zoom` (nativo, sem injeção de script), persistido em `settings.json` como `zoom_level`.
- **Tela cheia**: alternância via `is_fullscreen`/`set_fullscreen`.
- **Múltiplas janelas**: "Nova janela" cria janelas `chatgpt-N` carregando `https://chatgpt.com`, com as mesmas políticas de navegação/permissão. Janelas dinâmicas não recebem capabilities adicionais (sem IPC).
- **Bandeja do sistema** (feature `tray-icon`): menu Abrir/Ocultar/Sair e clique esquerdo para mostrar janela principal. Falha na criação da bandeja é tratada como não fatal (degradação graciosa).
- **Sobre/Versão**: abre janela local `about` (`index.html`) sem injeção no conteúdo remoto.
- **Recarregar**: usa `WebviewWindow::reload()` (sem `eval`).

## Segurança
- Nenhum `eval`/injeção de script no conteúdo remoto do ChatGPT para zoom/reload/versão.
- Conteúdo remoto continua sem capabilities Tauri (`withGlobalTauri: false`, capability vazia para janelas conhecidas).

## Pendente / limitações
- **Lembrar posição/tamanho/maximização da janela**: não implementado completamente (requer listeners de move/resize e persistência cuidadosa) — pendente.
- **Comportamento ao fechar / minimizar para bandeja**: configuração existe em `settings`, mas integração completa com evento de fechamento por janela/tray está parcial — validar e completar.
- **Iniciar minimizado / abrir ao iniciar sistema / verificar atualizações do app**: não implementados (atualização exigiria `tauri-plugin-updater`; auto-start exige integração de sistema). Requer decisão/escopo.
- **AppImage/DEB/RPM/Flatpak**: Fase 6.
- Atalhos podem ser capturados pelo menu antes do site ou vice-versa; validar UX.
- Em alguns ambientes (GNOME Wayland sem AppIndicator adequado), a bandeja pode não aparecer — comportamento esperado (opcional).

## Validação sugerida (usuário)
1. Menu Arquivo/Editar/Exibir/Ajuda aparece e itens funcionam.
2. `Ctrl+R` recarrega, `F11` alterna tela cheia, `Ctrl+Q` sai.
3. `Ctrl+=`/`-`/`0` alteram zoom e persistem ao reiniciar.
4. `Ctrl+N` abre nova janela do ChatGPT.
5. Bandeja (se suportada): abrir/ocultar/sair.
6. Verificar se login/sessão/fluxos OAuth continuam funcionando após mudanças.
