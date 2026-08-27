import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

test("landing page is accessible and has no runtime errors", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  page.on("pageerror", (error) => errors.push(error.message));
  await page.goto("/");
  await expect(page).toHaveTitle(/Secret Sync Preflight/);
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.getByRole("heading", { name: "Know your secrets line up before deploy." })).toBeVisible();
  // axe's bundled Playwright types can lag the runner patch while the runtime API remains compatible.
  const accessibility = await new AxeBuilder({ page: page as never }).analyze();
  expect(accessibility.violations.filter((issue) => ["serious", "critical"].includes(issue.impact ?? ""))).toEqual([]);
  expect(errors).toEqual([]);
});

test("seeded demo catches missing, extra, rename, and limit states", async ({ page }) => {
  await page.goto("/#demo");
  await expect(page.getByRole("heading", { name: "Unsafe to deploy" })).toBeVisible();
  await expect(page.locator("#missing-metric")).toHaveText("1");
  await expect(page.locator("#extra-metric")).toHaveText("2");
  await expect(page.locator("#rename-metric")).toHaveText("1");
  await expect(page.getByText("SESION_KEY → SESSION_KEY")).toBeVisible();
  await expect(page.getByText("1 key(s) beyond provider maximum")).toBeVisible();
});

test("demo supports clean, empty, error, and offline states", async ({ page, context }) => {
  await page.goto("/#demo");
  await page.locator("#desired").fill("");
  await page.locator("#current").fill("");
  await page.locator("#limit").fill("100");
  await page.getByRole("button", { name: "Run preflight" }).click();
  await expect(page.getByRole("heading", { name: "Safe to deploy" })).toBeVisible();
  await expect(page.getByText("All declared keys are present; no excess keys found.")).toBeVisible();

  await page.locator("#current").fill("TOKEN=must-not-appear");
  await page.getByRole("button", { name: "Run preflight" }).click();
  await expect(page.getByText("Input needs attention")).toBeVisible();
  await expect(page.locator("body")).not.toContainText("must-not-appear", { useInnerText: true });

  await context.setOffline(true);
  await page.evaluate(() => window.dispatchEvent(new Event("offline")));
  await expect(page.locator("#connection")).toContainText("Offline · demo still works");
  await page.getByRole("button", { name: "Run preflight" }).click();
});

test("primary demo path is keyboard operable", async ({ page }) => {
  await page.goto("/#demo");
  await page.locator("#desired").focus();
  await page.keyboard.press("Tab");
  await expect(page.locator("#current")).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.locator("#limit")).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.locator("#policy")).toBeFocused();
  await page.keyboard.press("Tab");
  await expect(page.getByRole("button", { name: "Run preflight" })).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(page.getByRole("heading", { name: "Unsafe to deploy" })).toBeVisible();
});

test("mobile layout has no horizontal overflow and legal pages are sound", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile", "mobile-only layout assertion");
  await page.goto("/");
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  await page.goto("/privacy/");
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("main")).toBeVisible();
  await page.goto("/terms/");
  await expect(page.locator("h1")).toHaveCount(1);
});
