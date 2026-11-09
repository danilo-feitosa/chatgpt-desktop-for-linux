// UI local mínima (Fase 2). A janela principal do app carrega https://chatgpt.com.
// Esta página pode ser usada futuramente para splash/erro/configurações locais.
const status = document.querySelector<HTMLDivElement>("#status");
if (status) {
  status.textContent = "Janela principal preparada. O ChatGPT Web será aberto na janela do aplicativo.";
}

export {};
