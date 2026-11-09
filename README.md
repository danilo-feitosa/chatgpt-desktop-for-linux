# ChatGPT Desktop for Linux

Cliente desktop Linux independente que carrega o **ChatGPT Web** (`https://chatgpt.com`) em uma janela nativa usando **Tauri 2 + Rust + WebKitGTK**.

> **Aviso:** este projeto **não é oficial** e **não é afiliado à OpenAI**. Ele não implementa um chatbot próprio, não usa API alternativa, não recria funcionalidades do ChatGPT e não utiliza scraping, automação não autorizada ou engenharia reversa. A experiência de conversa é fornecida pelo site oficial.

## Status

- **Fase 1 (concluída):** análise de ambiente, arquitetura, dependências, riscos e segurança. [`docs/fase-1-analise-arquitetura.md`](docs/fase-1-analise-arquitetura.md).
- **Fase 2 (concluída):** estrutura Tauri 2, identidade visual, módulos Rust, configuração segura. [`docs/fase-2-estrutura-inicial.md`](docs/fase-2-estrutura-inicial.md).
- **Fase 3 (parcial):** integração carrega `chatgpt.com`; navegação/chat/uploads validados pelo usuário; login OAuth (Google/Apple) e microfone/PipeWire com pendências. [`docs/fase-3-integracao-status.md`](docs/fase-3-integracao-status.md).
- **Fase 4 (implementação inicial):** menu, atalhos, zoom nativo, fullscreen, múltiplas janelas, bandeja opcional. [`docs/fase-4-recursos-nativos.md`](docs/fase-4-recursos-nativos.md). Validação runtime pendente.
- **Fase 5 (revisão estática concluída):** sem IPC/comandos expostos ao remoto; `withGlobalTauri` desabilitado; capabilities vazias; achados registrados (allowlist OAuth ampla, testes de plataforma/OAuth pendentes). [`docs/fase-5-seguranca-testes.md`](docs/fase-5-seguranca-testes.md).
- **Fase 6 (concluída parcialmente):** DEB e AppImage gerados; RPM/Flatpak dependem de ferramentas/manifesto. [`docs/fase-6-empacotamento.md`](docs/fase-6-empacotamento.md).

## Arquitetura (resumo)

- **Janela nativa** gerenciada por Rust/Tauri.
- **Conteúdo remoto**: `https://chatgpt.com` carregado na WebView principal.
- **Segurança**: `withGlobalTauri: false`; capability da janela principal sem permissões Tauri; política de navegação inicial no lado nativo (a validar na Fase 3); URLs externas destinadas ao navegador padrão.
- **Dados sensíveis**: o app não armazena senhas/cookies/tokens manualmente.

## Pré-requisitos (Ubuntu/Debian)

Dependências de desenvolvimento (Tauri 2 / WebKitGTK 4.1):

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev build-essential curl wget file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev \
  pkg-config libjavascriptcoregtk-4.1-dev libsoup-3.0-dev libgtk-3-dev
```

Toolchains esperadas:

- Rust/Cargo (`rustup`)
- Node.js + gerenciador de pacotes (pnpm/npm)
- Tauri CLI 2 (`cargo install tauri-cli --version '^2'` ou via scripts do projeto)

## Desenvolvimento

```bash
pnpm install
pnpm tauri dev
```

> Em ambientes sem os pacotes `-dev` do WebKitGTK/GTK, a compilação do backend Tauri pode falhar. Consulte `docs/fase-1-analise-arquitetura.md`.

## Empacotamento (Fase 6)

Artefatos gerados neste ambiente:
- DEB: `src-tauri/target/release/bundle/deb/ChatGPT Desktop_0.1.0_amd64.deb`
- AppImage: `src-tauri/target/release/bundle/appimage/ChatGPT Desktop_0.1.0_amd64.AppImage`

```bash
pnpm tauri build --bundles deb --ci
pnpm tauri build --bundles appimage --ci
```

Instalação DEB: `sudo apt install "./src-tauri/target/release/bundle/deb/ChatGPT Desktop_0.1.0_amd64.deb"`  
AppImage: `chmod +x` e execute o arquivo.

Detalhes: [`docs/fase-6-empacotamento.md`](docs/fase-6-empacotamento.md).  
RPM/Flatpak (receitas): [`docs/packaging-rpm-flatpak.md`](docs/packaging-rpm-flatpak.md), manifesto em [`packaging/flatpak/com.danilo.chatgptdesktop.yml`](packaging/flatpak/com.danilo.chatgptdesktop.yml).

## Estrutura

```text
chatgpt-desktop/
├── docs/
├── packaging/
├── src/                    # UI local mínima (splash/identidade local)
├── src-tauri/
│   ├── capabilities/       # permissões mínimas (janela principal sem IPC remoto)
│   ├── icons/
│   └── src/
│       ├── lib.rs
│       ├── main.rs
│       ├── window.rs
│       ├── navigation.rs
│       ├── settings.rs
│       ├── menu.rs         # estrutura (Fase 4)
│       └── tray.rs         # estrutura (Fase 4)
├── package.json
└── README.md
```

## Segurança

- Não desabilitar validação TLS.
- Não injetar scripts no ChatGPT Web.
- Não capturar credenciais, cookies ou mensagens.
- Não conceder permissões Tauri desnecessárias ao conteúdo remoto.
- Restringir navegação a domínios necessários; abrir externo no navegador.

## Licença

MIT — veja [`LICENSE`](LICENSE). Este projeto não é afiliado à OpenAI.
