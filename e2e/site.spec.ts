import { spawnSync } from "node:child_process";
import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const origin = "http://127.0.0.1:4173";

function normalizedDemoOutput(output: string): string {
  return output.replace(/^Demo files: .*$/m, "Demo files: /tmp/sspf-demo-<id>").trim();
}

async function expectRouteMetadata(page: Page, path: string, title: string): Promise<void> {
  await page.goto(path);
  await expect(page).toHaveTitle(title);
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("h1")).toBeFocused();
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.locator('link[rel="canonical"]')).toHaveCount(1);
  await expect(page.locator('link[rel="apple-touch-icon"]')).toHaveCount(1);
  for (const property of ["og:title", "og:description", "og:image"]) {
    await expect(page.locator(`meta[property="${property}"]`)).toHaveCount(1);
  }
  for (const name of ["twitter:card", "twitter:title", "twitter:description", "twitter:image"]) {
    await expect(page.locator(`meta[name="${name}"]`)).toHaveCount(1);
  }
}

test("landing first screen names the job, audience, action, privacy, offline use, and price", async ({ page }, testInfo) => {
  await page.goto("/");
  await expect(page).toHaveTitle("Secret Sync Preflight — check secret key drift");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Check secret key drift before deployment.");
  await expect(page.getByText("For DevOps teams syncing configuration across CI and hosting services.")).toBeVisible();
  await expect(page.getByRole("link", { name: "Try it with sample data" }).first()).toHaveAttribute("href", "/demo/");
  await expect(page.getByText("Opens a sample drift report in this browser.")).toBeVisible();
  await expect(page.getByText("No provider login", { exact: true })).toBeVisible();
  await expect(page.getByText("Demo works offline after the first visit", { exact: true })).toBeVisible();
  await expect(page.getByText("Free under the MIT License", { exact: true })).toBeVisible();
  if (testInfo.project.name === "mobile") {
    const lastFact = await page.getByText("Free under the MIT License", { exact: true }).boundingBox();
    expect(lastFact && lastFact.y + lastFact.height).toBeLessThanOrEqual(844);
    expect(await page.evaluate(() => window.scrollY)).toBe(0);
  }
});

test("@claim:demo-entry the landing action opens a populated sample report", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Try it with sample data" }).first().click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await expect(page.locator("#missing-metric")).toHaveText("1");
  await expect(page.locator("#extra-metric")).toHaveText("2");
  await expect(page.locator("#rename-metric")).toHaveText("1");
  await expect(page.getByText("SESION_KEY → SESSION_KEY")).toBeVisible();
  await expect(page.getByText("4 keys / 3 maximum · over limit")).toBeVisible();
  await expect(page.locator('input[type="password"]')).toHaveCount(0);
});

test("@claim:cli-recording landing recording matches the real demo command", async ({ page }) => {
  const command = spawnSync("cargo", ["run", "--quiet", "--", "demo"], {
    cwd: process.cwd(),
    encoding: "utf8"
  });
  expect(command.status).toBe(1);
  expect(command.stderr).toBe("");
  const expectedOutput = normalizedDemoOutput(command.stdout);

  await page.goto("/");
  const recording = page.locator('.recording-frame img[src="/cli-demo.svg"]');
  await expect(recording).toBeVisible();
  expect(await recording.evaluate((image: HTMLImageElement) => image.complete && image.naturalWidth === 1200)).toBeTruthy();
  await page.getByText("Read the CLI transcript").click();
  expect(normalizedDemoOutput(await page.locator("#cli-recording-output").innerText())).toBe(expectedOutput);

  const svgLines = await page.evaluate(async () => {
    const source = await fetch("/cli-demo.svg").then((response) => response.text());
    const document = new DOMParser().parseFromString(source, "image/svg+xml");
    return [...document.querySelectorAll('[data-output="line"]')].map((line) => line.textContent).join("\n");
  });
  expect(normalizedDemoOutput(svgLines)).toBe(expectedOutput);
  await expect(page.getByText("The sample files are written to a new")).toBeVisible();
});

test("@claim:demo-browser-isolation sample edits reset without changing real browser data", async ({ browser }) => {
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto("/");
  await page.evaluate(() => {
    localStorage.setItem("real:sentinel", "keep-local");
    sessionStorage.setItem("real:sentinel", "keep-session");
  });
  await page.getByRole("link", { name: "Try it with sample data" }).first().click();
  await page.locator("#expected").fill("ONE_KEY");
  await page.locator("#current").fill("ANOTHER_KEY");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.locator("#expected")).toHaveValue("API_URL\nDATABASE_URL\nSESSION_KEY");
  await expect(page.locator("#current")).toHaveValue("API_URL\nDATABASE_URL\nSESION_KEY\nOLD_WEBHOOK_TOKEN");
  expect(await page.evaluate(async () => ({
    local: { ...localStorage },
    session: { ...sessionStorage },
    cookies: document.cookie,
    databases: indexedDB.databases ? (await indexedDB.databases()).length : 0
  }))).toEqual({
    local: { "real:sentinel": "keep-local" },
    session: { "real:sentinel": "keep-session" },
    cookies: "",
    databases: 0
  });
  await page.getByRole("link", { name: "Start for real" }).click();
  await expect(page).toHaveURL(`${origin}/`);
  expect(await page.evaluate(() => [localStorage.getItem("real:sentinel"), sessionStorage.getItem("real:sentinel")])).toEqual(["keep-local", "keep-session"]);
  await page.goto("/demo/");
  await page.locator("#expected").fill("UNSAVED_EDIT");
  await page.reload();
  await expect(page.locator("#expected")).toHaveValue("API_URL\nDATABASE_URL\nSESSION_KEY");
  await context.close();
});

test("@claim:demo-browser-network the full sample flow stays on site and works offline", async ({ browser }) => {
  const context = await browser.newContext();
  const requests: Array<{ url: string; method: string }> = [];
  context.on("request", (request) => requests.push({ url: request.url(), method: request.method() }));
  const page = await context.newPage();
  await page.goto("/demo/");
  await expect(page.locator("#connection")).toContainText("Available offline", { timeout: 15000 });
  await page.reload();
  await page.locator("#expected").fill("API_URL\nDATABASE_URL\nSESSION_KEY");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await context.setOffline(true);
  await page.reload();
  await expect(page.locator("#connection")).toContainText("Offline · demo still works");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await expect(page.getByRole("heading", { name: "Unsafe to deploy" })).toBeVisible();
  expect(requests.length).toBeGreaterThan(0);
  expect(requests.every((request) => new URL(request.url).origin === origin)).toBeTruthy();
  expect(requests.every((request) => request.method === "GET")).toBeTruthy();
  const requestedPaths = requests.map((request) => new URL(request.url).pathname);
  expect(requestedPaths.some((path) => /analytics|track|beacon/i.test(path))).toBeFalsy();
  await context.setOffline(false);
  await context.close();
});

test("demo handles invalid, clean, boundary, warning, and reset paths", async ({ page }) => {
  await page.goto("/demo/");
  await page.locator("#expected").fill("TOKEN=value-that-must-not-render");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await expect(page.locator("#expected-error")).toHaveText("Line 1 must contain one key name. Remove values or spaces.");
  await expect(page.getByRole("heading", { name: "Fix the key list" })).toBeVisible();
  await expect(page.locator("body")).not.toContainText("value-that-must-not-render");

  await page.locator("#expected").fill("ONLY_KEY");
  await page.locator("#current").fill("ONLY_KEY");
  await page.locator("#limit").fill("10000");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await expect(page.getByRole("heading", { name: "Safe to deploy" })).toBeVisible();
  await expect(page.getByText("No drift", { exact: true })).toBeVisible();

  await page.locator("#current").fill("ONLY_KEY\nOLD_KEY");
  await page.locator("#policy").selectOption("warn");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await expect(page.getByRole("heading", { name: "Review before deployment" })).toBeVisible();
  await expect(page.locator("#status-badge")).toHaveText("Warning");

  await page.locator("#limit").fill("0");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await expect(page.getByText("Destination limit must be between 1 and 10,000.")).toBeVisible();
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.getByRole("heading", { name: "Unsafe to deploy" })).toBeVisible();
});

test("normal route links, Back, and deep links focus and announce each heading", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("h1")).toBeFocused();
  await page.getByRole("link", { name: "Demo", exact: true }).click();
  await expect(page.locator("h1")).toBeFocused();
  await expect(page.locator("#route-announcer")).toContainText("Demo — Secret Sync Preflight");
  await page.getByRole("link", { name: "Privacy", exact: true }).first().click();
  await expect(page.locator("h1")).toBeFocused();
  await expect(page.locator("#route-announcer")).toContainText("Privacy — Secret Sync Preflight");
  await page.goBack();
  await expect(page.locator("h1")).toBeFocused();
  await expect(page).toHaveURL(/\/demo\/$/);
  await page.goBack();
  await expect(page.locator("h1")).toBeFocused();
  await expect(page).toHaveURL(`${origin}/`);
  await page.goto("/terms/");
  await expect(page.locator("h1")).toBeFocused();
});

test("every route has complete metadata and the shared site shell", async ({ page }) => {
  for (const [path, title] of [
    ["/", "Secret Sync Preflight — check secret key drift"],
    ["/demo/", "Demo — Secret Sync Preflight"],
    ["/privacy/", "Privacy — Secret Sync Preflight"],
    ["/terms/", "Terms — Secret Sync Preflight"]
  ]) {
    await expectRouteMetadata(page, path, title);
    await expect(page.locator("header .wordmark")).toBeVisible();
    await expect(page.locator("footer .wordmark")).toBeVisible();
    await expect(page.locator("footer")).toContainText("Checks expected key names against destination exports.");
    await expect(page.locator("footer")).toContainText("Built by Param Factory · v0.1.0");
  }
});

test("@claim:site-deployment build serves security headers and a designed 404", async ({ page, request }) => {
  const home = await request.get("/");
  expect(home.status()).toBe(200);
  expect(home.headers()["content-security-policy"]).toContain("frame-ancestors 'none'");
  expect(home.headers()["referrer-policy"]).toBe("no-referrer");
  expect(home.headers()["x-content-type-options"]).toBe("nosniff");
  expect((await request.get("/demo")).status()).toBe(200);
  expect((await request.get("/demo/")).status()).toBe(200);
  const missing = await page.goto("/definitely-not-a-real-route");
  expect(missing?.status()).toBe(404);
  await expectRouteMetadata(page, "/definitely-not-a-real-route", "Page not found — Secret Sync Preflight");
  await expect(page.getByRole("link", { name: "Return home" })).toBeVisible();
  await expect(page.locator("header .wordmark")).toBeVisible();
  await expect(page.locator("footer .wordmark")).toBeVisible();
});

test("keyboard order, focus treatment, touch targets, motion, and route accessibility pass", async ({ page }) => {
  const browserErrors: string[] = [];
  page.on("pageerror", (error) => browserErrors.push(String(error)));
  page.on("console", (message) => {
    if (message.type() === "error") browserErrors.push(message.text());
  });
  await page.emulateMedia({ reducedMotion: "reduce" });
  for (const path of ["/", "/demo/", "/privacy/", "/terms/", "/missing-page"]) {
    await page.goto(path);
    const results = await new AxeBuilder({ page: page as never }).analyze();
    expect(results.violations.filter((issue) => ["serious", "critical"].includes(issue.impact ?? ""))).toEqual([]);
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    expect(overflow).toBeLessThanOrEqual(1);
    const smallTargets = await page.locator("header a, footer a, main a, main button, main input, main select, main textarea, main summary").evaluateAll((elements) => elements
      .filter((element) => {
        const rect = element.getBoundingClientRect();
        const style = getComputedStyle(element);
        return style.display !== "none" && style.visibility !== "hidden" && (rect.width < 44 || rect.height < 44);
      })
      .map((element) => ({ text: element.textContent?.trim(), rect: element.getBoundingClientRect().toJSON() })));
    expect(smallTargets).toEqual([]);
  }

  await page.goto("/demo/");
  await page.locator("h1").press("Tab");
  await expect(page.locator("#expected")).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.locator("#current")).toBeFocused();
  const outline = await page.locator("#current").evaluate((element) => getComputedStyle(element).outline);
  expect(outline).toContain("3px");

  await page.goto("/");
  const heroMotion = await page.locator(".hero-visual").evaluate((element) => ({
    transform: getComputedStyle(element).transform,
    duration: getComputedStyle(element).animationDuration
  }));
  expect(["none", "matrix(1, 0, 0, 1, 0, 0)"]).toContain(heroMotion.transform);
  expect(["0.00001s", "1e-05s", "0s"]).toContain(heroMotion.duration);
  expect(browserErrors).toEqual([]);
});

test("copy failure gives one clear recovery action", async ({ browser }) => {
  const context = await browser.newContext();
  await context.addInitScript(() => {
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: { writeText: () => Promise.reject(new Error("denied")) }
    });
  });
  const page = await context.newPage();
  await page.goto("/");
  await page.getByRole("button", { name: "Copy install command" }).click();
  await expect(page.getByRole("button", { name: "Copy unavailable" })).toBeVisible();
  await expect(page.locator("#copy-status")).toHaveText("Select the command above to copy it.");
  await context.close();
});

test("all internal links and anchors resolve", async ({ page, request }) => {
  await page.goto("/");
  const hrefs = await page.locator("a").evaluateAll((links) => links.map((link) => link.getAttribute("href")).filter((href): href is string => Boolean(href)));
  for (const href of new Set(hrefs)) {
    if (href.startsWith("http") || href.startsWith("mailto:")) continue;
    const url = new URL(href, origin);
    if (url.hash) {
      await page.goto(url.pathname);
      await expect(page.locator(url.hash)).toHaveCount(1);
    } else {
      expect((await request.get(url.pathname)).status()).toBe(200);
    }
  }
});
