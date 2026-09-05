import "./shell";

if (new URLSearchParams(window.location.search).get("demo") === "1") {
  window.location.replace("/demo/");
}
