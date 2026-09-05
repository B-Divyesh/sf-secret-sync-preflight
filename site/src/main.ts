import "./shell";

const KEY_PATTERN = /^[A-Za-z0-9_.\-/:]{1,255}$/;

type ParseResult = { keys: string[]; error?: string };

function parseKeys(value: string): ParseResult {
  const keys: string[] = [];
  const seen = new Set<string>();
  const lines = value.split(/\r?\n/);
  for (let index = 0; index < lines.length; index += 1) {
    const candidate = lines[index].trim();
    if (!candidate || candidate.startsWith("#")) continue;
    if (candidate.includes("=") || /\s/.test(candidate) || !KEY_PATTERN.test(candidate)) {
      return { keys: [], error: `Line ${index + 1} must contain one key name. Remove values or spaces.` };
    }
    if (seen.has(candidate)) return { keys: [], error: `Line ${index + 1} duplicates an earlier key.` };
    seen.add(candidate);
    keys.push(candidate);
  }
  return { keys };
}

function distance(a: string, b: string): number {
  let previous = Array.from({ length: b.length + 1 }, (_, index) => index);
  for (let i = 0; i < a.length; i += 1) {
    const current = [i + 1];
    for (let j = 0; j < b.length; j += 1) {
      current.push(Math.min(previous[j + 1] + 1, current[j] + 1, previous[j] + Number(a[i].toLowerCase() !== b[j].toLowerCase())));
    }
    previous = current;
  }
  return previous[b.length];
}

function likelyRenames(missing: string[], extra: string[]): Array<{ current: string; desired: string }> {
  const candidates = missing.flatMap((desired, m) => extra.map((current, e) => ({ desired, current, m, e, score: distance(desired, current) })))
    .filter(({ score }) => score > 0 && score <= 2)
    .sort((a, b) => a.score - b.score || a.desired.localeCompare(b.desired));
  const usedMissing = new Set<number>();
  const usedExtra = new Set<number>();
  return candidates.filter(({ m, e }) => {
    if (usedMissing.has(m) || usedExtra.has(e)) return false;
    usedMissing.add(m); usedExtra.add(e); return true;
  }).map(({ current, desired }) => ({ current, desired }));
}

const form = document.querySelector<HTMLFormElement>("#preflight-form")!;
const desiredInput = document.querySelector<HTMLTextAreaElement>("#expected")!;
const currentInput = document.querySelector<HTMLTextAreaElement>("#current")!;
const limitInput = document.querySelector<HTMLInputElement>("#limit")!;
const policyInput = document.querySelector<HTMLSelectElement>("#policy")!;
const findings = document.querySelector<HTMLElement>("#findings")!;

function text(selector: string, value: string): void {
  const element = document.querySelector<HTMLElement>(selector);
  if (element) element.textContent = value;
}

function finding(kind: "danger" | "warning" | "safe", label: string, message: string): HTMLElement {
  const row = document.createElement("div");
  row.className = `finding ${kind}`;
  const strong = document.createElement("strong");
  strong.textContent = label;
  const span = document.createElement("span");
  span.textContent = message;
  row.append(strong, span);
  return row;
}

function setFieldState(input: HTMLTextAreaElement, result: ParseResult): void {
  const id = input.id;
  input.setAttribute("aria-invalid", String(Boolean(result.error)));
  text(`#${id}-error`, result.error ?? "");
  text(`#${id}-count`, result.error ? "Fix the key list" : `${result.keys.length} ${result.keys.length === 1 ? "key" : "keys"}`);
}

function runPreflight(): void {
  const desiredResult = parseKeys(desiredInput.value);
  const currentResult = parseKeys(currentInput.value);
  setFieldState(desiredInput, desiredResult);
  setFieldState(currentInput, currentResult);
  const limit = Number(limitInput.value);
  const limitError = !Number.isInteger(limit) || limit < 1 || limit > 10000;
  limitInput.setAttribute("aria-invalid", String(limitError));
  if (desiredResult.error || currentResult.error || limitError) {
    findings.replaceChildren(finding("danger", "Input error", limitError ? "Destination limit must be between 1 and 10,000." : "Fix the highlighted key list, then run again."));
    text("#result-title", "Fix the key list");
    setStatus("Blocked", "danger");
    text("#result-note", "Entries such as KEY=value are rejected before values can appear in the result.");
    return;
  }

  const desired = new Set(desiredResult.keys);
  const current = new Set(currentResult.keys);
  const missing = [...desired].filter((key) => !current.has(key)).sort();
  const extra = [...current].filter((key) => !desired.has(key)).sort();
  const renames = likelyRenames(missing, extra);
  const overLimit = desired.size > limit || current.size > limit;
  const atLimit = desired.size === limit || current.size === limit;
  const extrasBlock = extra.length > 0 && policyInput.value === "block";
  const blocked = missing.length > 0 || overLimit || extrasBlock;

  text("#missing-metric", String(missing.length));
  text("#extra-metric", String(extra.length));
  text("#rename-metric", String(renames.length));
  text("#result-title", blocked ? "Unsafe to deploy" : extra.length || atLimit ? "Review before deployment" : "Safe to deploy");
  setStatus(blocked ? "Blocked" : extra.length || atLimit ? "Warning" : "Passed", blocked ? "danger" : extra.length || atLimit ? "warning" : "safe");

  const capacity = `${Math.max(desired.size, current.size)} keys / ${limit} maximum${overLimit ? " · over limit" : atLimit ? " · at limit" : " · within limit"}`;
  text("#capacity-label", capacity);
  const meter = document.querySelector<HTMLElement>(".capacity-track")!;
  meter.setAttribute("aria-valuemax", String(limit));
  meter.setAttribute("aria-valuenow", String(Math.min(limit, Math.max(desired.size, current.size))));
  const fill = document.querySelector<HTMLElement>("#capacity-fill")!;
  fill.style.width = `${Math.min(100, Math.max(desired.size, current.size) / limit * 100)}%`;
  fill.style.background = overLimit ? "var(--coral)" : atLimit ? "var(--amber)" : "var(--signal)";

  const rows: HTMLElement[] = [];
  renames.forEach(({ current: from, desired: to }) => rows.push(finding("warning", "Rename?", `${from} → ${to}`)));
  missing.forEach((key) => rows.push(finding("danger", "Missing", key)));
  extra.forEach((key) => {
    const policyMessage = policyInput.value === "block"
      ? "Deletion is blocked."
      : `${policyInput.value === "warn" ? "Warn" : "Allow"} policy; no change is made.`;
    rows.push(finding(extrasBlock ? "danger" : "warning", "Extra", `${key}. ${policyMessage}`));
  });
  if (overLimit) {
    const difference = Math.max(desired.size, current.size) - limit;
    rows.push(finding("danger", "Over limit", `${difference} ${difference === 1 ? "key" : "keys"} above the destination maximum.`));
  }
  if (rows.length === 0) rows.push(finding("safe", "No drift", "All expected keys are present. No extra keys were found."));
  findings.replaceChildren(...rows);
  text("#result-note", "No changes were made. Review the report. Then update the expected keys or destination export.");
}

function setStatus(label: string, kind: "danger" | "warning" | "safe"): void {
  const badge = document.querySelector<HTMLElement>("#status-badge")!;
  badge.textContent = label;
  badge.className = `status-badge ${kind}`;
}

form.addEventListener("submit", (event) => { event.preventDefault(); runPreflight(); });
[desiredInput, currentInput].forEach((input) => input.addEventListener("input", () => setFieldState(input, parseKeys(input.value))));

const connection = document.querySelector<HTMLElement>("#connection")!;
let offlinePrepared = false;
function updateConnection(): void {
  connection.classList.toggle("offline", !navigator.onLine);
  connection.lastChild!.textContent = navigator.onLine
    ? offlinePrepared ? "Available offline" : "Preparing offline demo"
    : "Offline · demo still works";
}
window.addEventListener("online", updateConnection);
window.addEventListener("offline", updateConnection);
updateConnection();
if ("serviceWorker" in navigator) {
  navigator.serviceWorker.ready.then(() => {
    offlinePrepared = true;
    updateConnection();
  }).catch(() => undefined);
}

const resetButton = document.querySelector<HTMLButtonElement>("#reset-demo");
resetButton?.addEventListener("click", () => {
  desiredInput.value = "API_URL\nDATABASE_URL\nSESSION_KEY";
  currentInput.value = "API_URL\nDATABASE_URL\nSESION_KEY\nOLD_WEBHOOK_TOKEN";
  limitInput.value = "3";
  policyInput.value = "block";
  runPreflight();
  resetButton.textContent = "Demo reset";
  window.setTimeout(() => { resetButton.textContent = "Reset demo"; }, 1600);
});

runPreflight();
