import "@fontsource/inter/latin-400.css";
import "@fontsource/inter/latin-600.css";
import "@fontsource/ibm-plex-mono/latin-500.css";
import "./styles.css";

const path = window.location.pathname;

function current(route: string): string {
  return path.startsWith(route) ? ' aria-current="page"' : "";
}

const headerPlaceholder = document.querySelector<HTMLElement>("[data-site-header]");
if (headerPlaceholder) {
  const header = document.createElement("header");
  header.className = "site-header";
  header.innerHTML = `
    <a class="wordmark" href="/" aria-label="Secret Sync Preflight home"><span class="mark" aria-hidden="true"></span><span>SSP<span class="wordmark-muted">/flight</span></span></a>
    <nav aria-label="Primary navigation">
      <a href="/#how">How it works</a>
      <a href="/demo/"${current("/demo")}>Demo</a>
      <a href="/privacy/"${current("/privacy")}>Privacy</a>
      <a class="nav-install" href="/#install">Install</a>
    </nav>`;
  headerPlaceholder.replaceWith(header);
}

const footerPlaceholder = document.querySelector<HTMLElement>("[data-site-footer]");
if (footerPlaceholder) {
  const footer = document.createElement("footer");
  footer.innerHTML = `
    <a class="wordmark" href="/" aria-label="Secret Sync Preflight home"><span class="mark" aria-hidden="true"></span><span>SSP<span class="wordmark-muted">/flight</span></span></a>
    <p>Checks expected key names against destination exports. Key names can be sensitive.</p>
    <div class="footer-links"><span>Built by Param Factory · v0.1.0</span><nav aria-label="Footer navigation"><a href="/privacy/"${current("/privacy")}>Privacy</a><a href="/terms/"${current("/terms")}>Terms</a><a href="https://github.com/B-Divyesh/sf-secret-sync-preflight">Source <span class="sr-only">(opens GitHub)</span></a></nav></div>`;
  footerPlaceholder.replaceWith(footer);
}

const announcer = document.createElement("div");
announcer.className = "sr-only";
announcer.id = "route-announcer";
announcer.setAttribute("aria-live", "polite");
announcer.setAttribute("aria-atomic", "true");
document.body.append(announcer);

function announceRoute(): void {
  const heading = document.querySelector<HTMLElement>("h1");
  if (!heading) return;
  heading.tabIndex = -1;
  heading.focus({ preventScroll: true });
  announcer.textContent = `${document.title}. ${heading.textContent?.trim() ?? ""}`;
}

window.addEventListener("pageshow", () => window.requestAnimationFrame(announceRoute));

const copyButton = document.querySelector<HTMLButtonElement>("#copy-command");
copyButton?.addEventListener("click", async () => {
  const status = document.querySelector<HTMLElement>("#copy-status");
  try {
    await navigator.clipboard.writeText(copyButton.dataset.copy ?? "");
    copyButton.textContent = "Copied";
    if (status) status.textContent = "Install command copied to clipboard.";
  } catch {
    copyButton.textContent = "Copy unavailable";
    if (status) status.textContent = "Select the command above to copy it.";
  }
  window.setTimeout(() => { copyButton.textContent = "Copy install command"; }, 2000);
});

if ("serviceWorker" in navigator && import.meta.env.PROD) {
  window.addEventListener("load", () => navigator.serviceWorker.register("/sw.js").catch(() => undefined));
}
