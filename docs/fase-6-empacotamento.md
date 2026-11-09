# Fase 6 — Empacotamento
**Data:** 2026-11-08  
**Status:** DEB e AppImage gerados com sucesso; RPM/Flatpak pendentes por ferramentas ausentes

## Artefatos gerados
| Formato | Status | Arquivo (exemplo gerado) |
|---|---|---|
| DEB | OK | `src-tauri/target/release/bundle/deb/ChatGPT Desktop_0.1.0_amd64.deb` (~2.4 MiB) |
| AppImage | OK | `src-tauri/target/release/bundle/appimage/ChatGPT Desktop_0.1.0_amd64.AppImage` (~84 MiB) |
| RPM | Não gerado | `rpmbuild` não encontrado no ambiente |
| Flatpak | Não avaliado como viável aqui | `flatpak-builder` não encontrado; requer manifesto/fluxo específico |

## Comandos de build
```bash
cd /home/danilo/chatgpt-desktop
pnpm install
pnpm tauri build --bundles deb --ci
pnpm tauri build --bundles appimage --ci
# ou: pnpm tauri build --ci  (usa targets configurados: deb, appimage)
```

## Dependências de runtime (DEB)
O pacote DEB declara dependências como `libayatana-appindicator3-1`, `libwebkit2gtk-4.1-0`, `libgtk-3-0` (conforme bundler Tauri). O usuário final precisa de WebKitGTK 4.1 compatível.

## Instalação

### DEB (Ubuntu/Debian/Mint/Pop!_OS)
```bash
sudo apt install "./src-tauri/target/release/bundle/deb/ChatGPT Desktop_0.1.0_amd64.deb"
```
Ou:
```bash
sudo dpkg -i "./src-tauri/target/release/bundle/deb/ChatGPT Desktop_0.1.0_amd64.deb"
sudo apt -f install
```

### AppImage
```bash
chmod +x "./src-tauri/target/release/bundle/appimage/ChatGPT Desktop_0.1.0_amd64.AppImage"
"./src-tauri/target/release/bundle/appimage/ChatGPT Desktop_0.1.0_amd64.AppImage"
```

## Atualização
- **DEB:** instalar a nova versão do `.deb` (o gerenciador de pacotes substitui).
- **AppImage:** substituir o arquivo AppImage pela nova versão.
- Atualizador automático do app (`tauri-plugin-updater`) não está configurado nesta versão.

## Metadados / configuração
- `src-tauri/tauri.conf.json`: `productName`, `version`, `identifier`, descrição, ícones, `bundle.targets` (`deb`, `appimage`), `bundle.category` (`Utility` — `Network` causou erro de categoria no bundler neste ambiente), seção DEB `net`.
- Ícones em `src-tauri/icons/`.
- Desktop entry gerado pelo bundler; template de referência em `packaging/chatgpt-desktop.desktop`.

## Observações
- O nome do arquivo gerado contém espaço (`ChatGPT Desktop_...`) devido ao `productName`; usar aspas no shell.
- AppImage é maior por empacotar dependências necessárias; DEB usa bibliotecas do sistema.
- RPM: instalar `rpm-build`/ferramentas RPM e executar `pnpm tauri build --bundles rpm --ci`.
- Flatpak: requer manifesto, runtime adequado e `flatpak-builder`; avaliar separadamente.
- Antes de release público, validar: login/sessão OAuth, microfone/portais, tray em GNOME/KDE/XFCE, Wayland/X11, e minimizar allowlist de navegação com evidência.
