import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("landing first screen names the job and provides the demo action", async ({ page }) => {
  await page.goto("/");
  await expect(page).toHaveTitle("Secret Sync Preflight — check secret key drift");
  await expect(page.locator("html")).toHaveAttribute("lang", "en");
  await expect(page.locator("h1")).toHaveText("Check secret key drift before deployment.");
  await expect(page.getByRole("link", { name: "Try it with sample data" }).first()).toHaveAttribute("href", "/?demo=1");
  await expect(page.getByRole("main")).toBeVisible();
});

test("demo route seeds the real report, resets, and transfers focus", async ({ page }) => {
  await page.goto("/demo/");
  await expect(page).toHaveTitle("Demo — Secret Sync Preflight");
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
  await expect(page.locator("#missing-metric")).toHaveText("1");
  await expect(page.locator("#extra-metric")).toHaveText("2");
  await expect(page.getByText("SESION_KEY → SESSION_KEY")).toBeVisible();
  await page.locator("#desired").fill("API_URL");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.locator("#desired")).toHaveValue("API_URL\nDATABASE_URL\nSESSION_KEY");
  await page.getByRole("link", { name: "Start for real" }).click();
  await expect(page.locator("h1")).toBeFocused();
});

test("query demo path enters the isolated sample", async ({ page }) => {
  await page.goto("/?demo=1");
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
});

test("@claim:demo-browser-isolation browser sample edits are resettable and unsaved", async ({ page }) => {
  await page.goto("/demo/");
  await page.locator("#desired").fill("ONE_KEY");
  await page.getByRole("button", { name: "Show drift report" }).click();
  const storage = await page.evaluate(() => ({ local: localStorage.length, session: sessionStorage.length, cookies: document.cookie }));
  expect(storage).toEqual({ local: 0, session: 0, cookies: "" });
  await page.reload();
  await expect(page.locator("#desired")).toHaveValue("API_URL\nDATABASE_URL\nSESSION_KEY");
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.locator("#current")).toHaveValue(/SESION_KEY/);
});

test("@claim:demo-browser-network browser sample stays same-origin and works offline", async ({ page, context }) => {
  const origins: string[] = [];
  page.on("request", request => origins.push(new URL(request.url()).origin));
  await page.goto("/demo/");
  expect(origins.every(origin => origin === "http://127.0.0.1:4173")).toBeTruthy();
  await page.evaluate(() => navigator.serviceWorker.ready);
  await page.reload();
  await context.setOffline(true);
  await page.reload();
  await expect(page.locator("#connection")).toContainText("Offline · demo still works");
  await page.getByRole("button", { name: "Show drift report" }).click();
  await expect(page.getByRole("heading", { name: "Unsafe to deploy" })).toBeVisible();
});

test("keyboard, mobile layout, legal pages, and accessibility are sound", async ({ page }, testInfo) => {
  await page.goto("/demo/");
  await page.locator("#desired").focus();
  await page.keyboard.press("Tab");
  await expect(page.locator("#current")).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.locator("#limit")).toBeFocused();
  const results = await new AxeBuilder({ page: page as never }).analyze();
  expect(results.violations.filter(issue => ["serious", "critical"].includes(issue.impact ?? ""))).toEqual([]);
  if (testInfo.project.name === "mobile") {
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
    expect(overflow).toBeLessThanOrEqual(1);
  }
  await page.goto("/privacy/");
  await expect(page.locator("h1")).toHaveCount(1);
  await page.goto("/terms/");
  await expect(page.locator("main")).toBeVisible();
});

test("unknown paths are a designed 404 and direct demo reload remains available", async ({ page }) => {
  const missing = await page.goto("/definitely-not-a-real-route");
  expect(missing?.status()).toBe(404);
  await expect(page).toHaveTitle("Page not found — Secret Sync Preflight");
  await expect(page.getByRole("link", { name: "Return home" })).toBeVisible();
  await page.goto("/demo/");
  await page.reload();
  await expect(page.getByText("Demo — sample data, nothing is saved")).toBeVisible();
});
