# Empacotamento: RPM e Flatpak (receitas/preparação)

## RPM (Tauri bundler)

O RPM é gerado pelo bundler do Tauri (não por um `.spec` manual neste projeto).

### Pré-requisitos (Fedora/RHEL/openSUSE, conforme distro)
- Dependências de build do Tauri/WebKitGTK 4.1
- Ferramentas RPM (ex.: `rpm-build`)

### Configuração
`src-tauri/tauri.conf.json` pode incluir ajustes em `bundle.linux.rpm` (opcional), por exemplo metadados específicos. O projeto já define identificador, versão, descrição e ícones.

### Geração
```bash
cd /home/danilo/chatgpt-desktop
pnpm install
pnpm tauri build --bundles rpm --ci
```

### Observações
- Neste ambiente, `rpmbuild` não estava instalado, então o RPM não foi gerado.
- Validar dependências runtime em distribuições alvo (WebKitGTK 4.1, GTK3, AppIndicator/Ayatana quando aplicável).
- Testar login/sessão, tray, Wayland/X11 e microfone no pacote instalado.

## Flatpak (manifesto de referência)

Arquivo: `packaging/flatpak/com.danilo.chatgptdesktop.yml`

Este manifesto é um **ponto de partida** e **requer validação**. Empacotar Tauri/WebKitGTK em Flatpak envolve escolhas importantes:
- runtime/SDK (GNOME vs Freedesktop) e versão;
- WebKitGTK e bibliotecas compatíveis;
- permissões mínimas (rede para `chatgpt.com`, display, áudio/portal para microfone, notificações);
- estratégia de build reproduzível (fontes + toolchains Rust/Node no sandbox) vs. empacotar binário já validado.

### Permissões sugeridas (mínimas práticas)
- `--share=network`: necessário para o ChatGPT Web.
- `--socket=wayland` / `--socket=fallback-x11` / `--share=ipc`: interface gráfica.
- `--socket=pulseaudio` (e/ou integração PipeWire/portal conforme runtime): áudio/microfone — **validar**.
- Evitar `--filesystem=home` amplo; preferir portais para upload/abertura de arquivos.

### Build (após validar manifesto/fontes)
```bash
flatpak-builder --force-clean build-dir packaging/flatpak/com.danilo.chatgptdesktop.yml
# ou instalação local para teste
flatpak-builder --user --install --force-clean build-dir packaging/flatpak/com.danilo.chatgptdesktop.yml
```

### Segurança
- Manter as mesmas restrições do app: sem IPC Tauri para conteúdo remoto, sem captura de credenciais/cookies, sem bypass de TLS.
- Revisar `finish-args` antes de distribuição (princípio do menor privilégio).
- Testar uploads via portal, microfone, tray (pode exigir permissões/portais específicos) e persistência de sessão.

## Próximos passos recomendados
1. Gerar RPM em ambiente com `rpmbuild`.
2. Refinar manifesto Flatpak com build reproduzível e runtime testado.
3. Testar ambos em Fedora/Ubuntu-based + GNOME/KDE (X11/Wayland).
4. Minimizar allowlist de navegação do app com evidência dos fluxos de login.
