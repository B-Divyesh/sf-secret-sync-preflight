import "@fontsource/inter/latin-400.css";
import "@fontsource/inter/latin-600.css";
import "@fontsource/ibm-plex-mono/latin-500.css";
import "./styles.css";

if (new URLSearchParams(window.location.search).get("demo") === "1") {
  window.location.replace("/demo/");
}

if ("serviceWorker" in navigator && import.meta.env.PROD) {
  window.addEventListener("load", () => navigator.serviceWorker.register("/sw.js").catch(() => undefined));
}

// Each static document is a real route. Moving focus to its page heading gives
// keyboard and screen-reader users the same clear route boundary as a client router.
window.addEventListener("load", () => {
  const heading = document.querySelector<HTMLElement>("h1");
  if (heading && new URLSearchParams(window.location.search).has("from")) {
    heading.tabIndex = -1;
    heading.focus({ preventScroll: true });
  }
});
