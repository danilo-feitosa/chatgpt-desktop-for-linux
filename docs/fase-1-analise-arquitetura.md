# Fase 1 — Análise e Planejamento
**Projeto:** ChatGPT Desktop for Linux  
**Data da análise:** 2026-11-08  
**Status:** Concluída (aguardando aprovação para Fase 2)

## 1. Ambiente verificado

| Item | Resultado observado |
|---|---|
| Sistema | Ubuntu 26.04.1 LTS (Resolute Raccoon), x86_64 |
| Kernel | Linux 7.0.0-43-generic (SMP PREEMPT_DYNAMIC) |
| Workspace | `/home/danilo/chatgpt-desktop` (vazio antes desta fase) |
| Repositório Git | Não inicializado |
| Rust | `rustc 1.99.1`, `cargo 1.99.1` (instalados em `~/.cargo/bin`) |
| Node.js / npm / pnpm | Node `v24.22.2`, npm `11.19.1`, pnpm disponível via fnm |
| Tauri CLI global | Não encontrado no `PATH` |
| WebKitGTK runtime | `libwebkit2gtk-4.1-0` **2.52.6** instalado |
| GTK3 runtime | `libgtk-3-0t64` 3.24.52 instalado |
| AppIndicator runtime | `libayatana-appindicator3-1` instalado |
| librsvg runtime | `librsvg2-2` 2.61.3 instalado |
| `build-essential` | Instalado |
| Pacotes `-dev` WebKit/GTK/AppIndicator/OpenSSL | **Não detectados** por `dpkg`/`pkg-config` no momento |

**Implicação:** o ambiente já possui toolchains Rust/Node e bibliotecas runtime relevantes, mas a preparação para compilar Tauri 2 no Linux exigirá instalação explícita dos pacotes de desenvolvimento antes da Fase 2 (ver seção 3).

## 2. Objetivo técnico validado (alto nível)

Criar um cliente desktop Linux com **Tauri 2 + Rust + WebKitGTK** que abra `https://chatgpt.com` em uma janela nativa independente, **sem**:

- implementar chatbot próprio;
- usar API alternativa;
- recriar funcionalidades do ChatGPT;
- usar scraping, automação não autorizada ou engenharia reversa;
- injetar scripts no site;
- capturar credenciais, cookies ou dados de sessão;
- expor comandos/IPC Tauri ao conteúdo remoto.

A aplicação deve atuar como **invólucro nativo** (janela, menu, atalhos, bandeja, configurações locais, empacotamento) enquanto o **ChatGPT Web oficial** permanece responsável pela experiência de conversação.

## 3. Tecnologias e dependências

### 3.1 Stack recomendada

| Camada | Escolha | Justificativa |
|---|---|---|
| Shell desktop | **Tauri 2** | Requisito do projeto; menor footprint que Electron; integra Rust + WebView do sistema. |
| Backend nativo | **Rust** | Gerencia janelas, menu, tray, atalhos globais/locais conforme viável, persistência de configurações locais e comandos seguros. |
| Renderização Linux | **WebKitGTK 4.1** (via WRY/Tauri) | Stack padrão do Tauri 2 no Linux; runtime 4.1 já presente no ambiente. |
| UI complementar nativa | HTML/CSS/JS mínimo (apenas telas locais: splash/carregamento/erro/configurações/sobre) | Evita reimplementar o ChatGPT; conteúdo local fica isolado do remoto. |
| Configurações locais | **JSON em diretório de configuração do app** (fase inicial) ou SQLite se houver necessidade real | SQLite só se houver volume/consultas; para preferências simples, JSON é mais simples e sustentável. |
| Build/empacotamento | Tauri bundler + `tauri-action`/scripts conforme fase 6 | Suporta AppImage/DEB/RPM; Flatpak será avaliado separadamente. |

**Decisão:** não usar Electron. Só reconsiderar se houver bloqueio técnico comprovado e documentado em WebKitGTK/Tauri 2 que impeça requisitos essenciais.

### 3.2 Dependências Linux (desenvolvimento) — a instalar/validar na Fase 2

Base Debian/Ubuntu (Tauri 2 / WebKitGTK 4.1):

```bash
sudo apt update
sudo apt install -y \
  libwebkit2gtk-4.1-dev \
  build-essential \
  curl wget file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  pkg-config \
  libjavascriptcoregtk-4.1-dev \
  libsoup-3.0-dev \
  libgtk-3-dev
```

> Nota: alguns pacotes podem já estar presentes como runtime; a lista acima prioriza completude para compilação. A Fase 2 deve confirmar versões e ajustar ao estado real do Ubuntu 26.04.

### 3.3 Dependências por distribuição (documentação alvo)

- **Ubuntu/Debian/Mint/Pop!_OS:** `libwebkit2gtk-4.1-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `libssl-dev`, build tools.
- **Fedora:** `webkit2gtk4.1-devel`, `openssl-devel`, `libappindicator-gtk3-devel`, `librsvg2-devel`, toolchain C/C++.
- **Arch:** `webkit2gtk-4.1`, `base-devel`, `openssl`, `libappindicator-gtk3`, `librsvg`, `xdotool`.
- **Runtime empacotado (DEB):** tipicamente depende de `libwebkit2gtk-4.1-0`, `libgtk-3-0` e `libayatana-appindicator3-1` quando tray é usado (validar com bundler Tauri 2).

## 4. Arquitetura recomendada

### 4.1 Visão geral

Arquitetura em **duas superfícies claramente separadas**:

1. **Superfície nativa (confiável, Rust/Tauri):**
   - ciclo de vida da aplicação;
   - criação/gerenciamento de janelas;
   - menu nativo;
   - atalhos nativos (apenas os que não conflitam ou que são de escopo da janela/app);
   - bandeja do sistema (opcional, tolerante a ausência);
   - zoom/tela cheia/recarga;
   - persistência de configurações locais não sensíveis;
   - abertura de URLs externas no navegador padrão.

2. **Superfície remota (não confiável, conteúdo web):**
   - `https://chatgpt.com` carregado no WebView;
   - **sem acesso a comandos Tauri/IPC**;
   - **sem capabilities de plugins perigosos** para origem remota;
   - navegação restrita a domínios necessários; links externos abertos fora da WebView.

### 4.2 Modelo de janela

- **Janela principal:** pode iniciar em uma página local mínima (splash/loader) e, após inicialização, navegar/carregar a WebView principal para `https://chatgpt.com` **ou** usar uma janela dedicada ao conteúdo remoto conforme o modelo mais seguro/simples suportado por Tauri 2.
- **Múltiplas janelas:** cada janela cria seu próprio contexto de WebView gerenciado pela app; perfil/persistência de dados do WebView deve ser avaliado para não compartilhar estado de forma insegura e para preservar sessão quando apropriado (validar comportamento real do WRY/WebKitGTK no PoC).
- **Estado de janela:** posição, tamanho, maximização e zoom salvos localmente (configuração do app), com validação de limites de tela.

### 4.3 Isolamento e IPC

- **Regra central:** conteúdo remoto (`https://chatgpt.com` e domínios relacionados necessários) **não deve** receber capability para invocar comandos Tauri.
- Páginas locais (sobre/configurações/splash), se usadas, podem ter capabilities mínimas e explícitas.
- Evitar `dangerous*`/exposição ampla de APIs. Não habilitar acesso remoto a filesystem, shell, dialog, opener irrestrito etc., exceto o estritamente necessário para UI local — e **nunca** para origem remota.
- Qualquer abertura de URL externa deve passar por validação no lado nativo e usar navegador padrão.

### 4.4 Navegação e domínios

- Permitir navegação interna necessária ao funcionamento do ChatGPT (domínio principal e subdomínios/autenticação/CDN estritamente necessários — **a ser mapeado empiricamente na Fase 3** sem interceptação de dados sensíveis).
- Bloquear ou delegar ao navegador padrão domínios não confiáveis/desnecessários.
- Não desabilitar verificação TLS/certificados.

### 4.5 Persistência de sessão

- **Não** armazenar senhas/cookies manualmente.
- Confiar nos mecanismos oficiais do WebView/WebKitGTK para cookies, armazenamento local e sessão, desde que o perfil de dados da aplicação seja persistente e configurado corretamente.
- **Ponto crítico de validação (Fase 3):** confirmar que login e sessão persistem entre reinicializações no Linux com Tauri 2 + WebKitGTK 4.1.

## 5. Estrutura inicial proposta (ajustada ao Tauri 2)

```text
chatgpt-desktop/
├── docs/
│   ├── fase-1-analise-arquitetura.md
│   ├── decisoes-tecnicas.md            # a partir da Fase 2
│   └── problemas-conhecidos.md         # a partir da Fase 2/3
├── src/                                # UI local mínima (Vite/vanilla ou framework leve), se necessária
│   ├── assets/
│   ├── styles/
│   └── pages/                          # splash, erro offline, sobre, configurações
├── src-tauri/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── window.rs                   # criação/configuração de janelas
│   │   ├── menu.rs
│   │   ├── tray.rs
│   │   ├── shortcuts.rs                # se necessário
│   │   ├── settings.rs                 # preferências locais não sensíveis
│   │   └── navigation.rs               # política de navegação/URLs externas
│   ├── capabilities/                   # permissões mínimas; separar local vs eventual remoto
│   ├── icons/
│   ├── tauri.conf.json
│   ├── Cargo.toml
│   └── build.rs
├── scripts/
├── README.md
└── LICENSE
```

A estrutura final será confirmada após `create-tauri-app` / scaffold oficial do Tauri 2 na Fase 2, para aderir ao layout e convenções atuais.

## 6. Limitações conhecidas e riscos

| Risco/Limitação | Impacto | Mitigação/validação |
|---|---|---|
| WebKitGTK pode diferir do Chromium em compatibilidade com recursos web modernos | Recursos do ChatGPT podem falhar ou degradar | PoC na Fase 3: login, chat, upload, imagens, áudio, navegação; documentar lacunas. |
| Carregar site remoto como experiência principal exige cuidado com capabilities/IPC | Risco de segurança se mal configurado | Capabilities mínimas; origem remota sem comandos Tauri; revisão de config. |
| Sessão/cookies persistentes dependem do comportamento do webview/data dir | Usuário pode precisar relogar | Validar persistência entre restarts; ajustar data directory se suportado com segurança. |
| Uploads/diálogos de arquivo/drag-and-drop variam no WebKitGTK | Upload pode não funcionar como no Chrome | Testes manuais focados na Fase 3. |
| Wayland/X11 e drivers (ex.: NVIDIA/WebKitGTK/EGL/DMABUF) | Janela branca, flicker, falhas gráficas | Testar X11/Wayland; documentar workarounds de ambiente (variáveis WebKit) sem embutir hacks inseguros por padrão. |
| Bandeja do sistema inconsistente entre GNOME/KDE/XFCE/Wayland | Ícone/comportamento pode variar | Tornar tray opcional e degradar graciosamente. |
| Mudanças no ChatGPT Web (domínios, fluxos, requisitos) | Quebras futuras | Política de navegação configurável com cautela; atualizações do app; sem gambiarras de scraping. |
| Ubuntu 26.04 é recente | Possíveis incompatibilidades de pacotes/tooling | Validar build e runtime no ambiente alvo; registrar versões. |
| Atalhos globais/conflitos com o site | UX inconsistente | Preferir atalhos de escopo da aplicação; preservar atalhos do site quando possível. |

## 7. Estratégia de segurança

1. **HTTPS obrigatório** para conteúdo principal (`https://chatgpt.com`).
2. **Sem bypass de TLS/certificados.**
3. **Sem injeção de JS/CSS** para alterar comportamento do ChatGPT.
4. **Sem captura de credenciais, tokens, cookies, storage ou mensagens.**
5. **Capabilities mínimas** (princípio do menor privilégio) em `src-tauri/capabilities/`.
6. **Conteúdo remoto sem IPC Tauri:** não expor comandos ao `chatgpt.com`.
7. **Política de navegação:** permitir apenas domínios necessários; abrir externo no navegador padrão.
8. **Dependências e APIs:** usar APIs suportadas no Tauri 2; evitar crates/plugins desnecessários.
9. **Sem contorno de proteções da OpenAI.**
10. **Revisão de segurança** antes de empacotar (Fase 5): capabilities, CSP/local pages se houver, comandos expostos, tratamento de URLs.

## 8. Plano de implementação (próximas fases)

### Fase 2 — Estrutura inicial (após aprovação)
- Instalar dependências `-dev` faltantes.
- Instalar/confirmar Tauri CLI 2 (`cargo install tauri-cli --version '^2'` ou uso via `cargo tauri`/npm conforme convenção).
- Scaffold do projeto Tauri 2 no workspace (preservando `docs/`).
- Configurar metadados do app, identificador, ícones provisórios, janela principal.
- Adicionar documentação de setup/build.

### Fase 3 — Integração ChatGPT (PoC crítico)
- Carregar `https://chatgpt.com` com configuração segura.
- Testes manuais: login, persistência de sessão, navegação entre conversas, envio/recebimento, upload/imagens/áudio quando aplicável.
- Mapear domínios necessários observáveis sem coleta de dados sensíveis.
- Decidir ajustes de data directory/política de navegação com base em evidência.

### Fase 4 — Recursos nativos
- Menu completo funcional.
- Atalhos, zoom, tela cheia.
- Múltiplas janelas.
- Bandeja opcional.
- Configurações locais.

### Fase 5 — Segurança e testes
- Auditoria de permissões/capabilities.
- Testes X11/Wayland, resoluções, falhas de rede, encerramento.
- Correções e documentação de limitações.

### Fase 6 — Empacotamento
- AppImage e DEB primeiro; RPM em seguida; Flatpak se viável.
- Metadados, `.desktop`, ícones finais, instruções.

## 9. Decisões técnicas registradas (Fase 1)

1. **Tauri 2 confirmado como base**; Electron descartado.
2. **ChatGPT Web oficial como fonte de funcionalidade**; app atua como wrapper nativo.
3. **Sem IPC para origem remota** como premissa de arquitetura.
4. **Configurações locais não sensíveis**; sem persistência manual de credenciais/sessão.
5. **PoC de viabilidade na Fase 3** é obrigatório antes de expandir recursos nativos, porque a compatibilidade WebKitGTK ↔ ChatGPT Web é o principal risco.

## 10. Critérios para aprovação da Fase 1

A Fase 1 está pronta para revisão se esta análise cobrir:

- ambiente e dependências observadas;
- arquitetura e isolamento de segurança;
- limitações/riscos realistas;
- estrutura inicial;
- plano faseado com validação de viabilidade.

**Nenhuma funcionalidade da aplicação foi implementada nesta fase.** Apenas documentação de análise/planejamento foi criada.
