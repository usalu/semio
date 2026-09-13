/** 🧩️ Semantic studio verification owner. */

import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";



//#region 🔖️SpaceE2eVerify
/** 🎭️ Playwright end-to-end workflow verification for the `s` studio shell (folded in from the former `.🧬semio/🦑️repo/🎫️tickets/26/07/04/RUST-PLUGIN-FRAMEWORK-MIGRATION/s-studio-e2e-verify.mjs`). */
const STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS = ["NoCompatibleDevice"];

function spaceE2eAssert(condition: boolean, message: string): void {
  if (!condition) throw new Error(message);
}

function isIgnorableStudioE2ePageError(message: string): boolean {
  return STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS.some((fragment) => message.includes(fragment));
}

async function waitForStudioE2eCondition(page: import("playwright").Page, predicate: (state: { text: string; children: number }) => boolean, label: string, deadline: number): Promise<{ text: string; children: number }> {
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const children = await page.locator("#root *").count().catch(() => 0);
    if (predicate({ text, children })) return { text, children };
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for ${label}`);
}

async function openStudioE2e(page: import("playwright").Page, deadline: number): Promise<{ text: string; children: number }> {
  await page.keyboard.press("Meta+n");
  while (Date.now() < deadline) {
    const text = await page
      .locator("body")
      .innerText()
      .catch(() => "");
    const path = new URL(page.url()).pathname;
    const children = await page.locator("#root *").count().catch(() => 0);
    if (/Catalogue/i.test(text) && /Parameters/i.test(text) && path.startsWith("/spaces/")) {
      return { text, children };
    }
    await page.waitForTimeout(500);
  }
  throw new Error("timeout waiting for studio workspace");
}

async function activateStudioE2eWorkflowWindow(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(200);
}

async function expandStudioE2eWorkflowEngagement(page: import("playwright").Page): Promise<void> {
  await activateStudioE2eWorkflowWindow(page);
  await page.evaluate(() => document.getElementById("framework.window.sWorkflow.search.toggle")?.click());
  await page.waitForSelector("#s-media-catalogue-hint", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromEngagement(page: import("playwright").Page): Promise<string> {
  await expandStudioE2eWorkflowEngagement(page);
  const engagementInput = page.locator("#s-media-catalogue-hint");
  await engagementInput.fill("draw draw");
  await engagementInput.press("Enter");
  await page.waitForTimeout(1500);
  return "engagement";
}

async function openStudioE2eCommandPalette(page: import("playwright").Page): Promise<void> {
  await page.locator(".semio-node-graph-host").first().click({ force: true });
  await page.waitForTimeout(100);
  await page.keyboard.press("Meta+p");
  await page.waitForSelector("[role='dialog'] [data-slot='command-input']", { timeout: 10_000 });
}

async function spawnStudioE2eDrawFromPalette(page: import("playwright").Page): Promise<string | null> {
  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("draw");
  await page.waitForTimeout(400);
  const drawSpawn = page
    .locator('[data-slot="command-item"]')
    .filter({ hasText: /Spawn Draw/i })
    .first();
  if (await drawSpawn.count()) {
    await drawSpawn.click();
    return "palette";
  }
  await page.keyboard.press("Escape");
  return null;
}

async function runStudioE2eVerify(baseUrl: string, timeoutMs: number): Promise<void> {
  const { chromium } = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const pageErrors: string[] = [];
  page.on("pageerror", (err) => pageErrors.push(String(err)));

  console.log(`navigating to ${baseUrl}`);
  await page.goto(baseUrl, { waitUntil: "domcontentloaded", timeout: 120_000 });
  await page.waitForFunction(() => /home/i.test(document.body.innerText) && /Demo Studio|New Studio/i.test(document.body.innerText) && document.querySelectorAll("#root *").length > 150, { timeout: 120_000 });

  const deadline = Date.now() + timeoutMs;
  const booted = await waitForStudioE2eCondition(page, ({ text }) => /Home/i.test(text) && /Studios|Search/i.test(text) && /Demo Studio|New Studio/i.test(text), "home shell with studios", deadline);
  console.log(`home loaded (${booted.children} nodes)`);
  spaceE2eAssert(/Demo Studio|Studios/i.test(booted.text), "home studios vfs should list seeded studio");

  await openStudioE2e(page, deadline);
  const pathAfterCreate = await page.evaluate(() => location.pathname);
  console.log(`studio loaded at ${pathAfterCreate}`);
  spaceE2eAssert(pathAfterCreate.startsWith("/spaces/"), "studio uri should be under /spaces/");

  await page.waitForFunction(() => document.querySelector(".semio-node-graph-host") != null, { timeout: 30_000 });

  const bodyText = await page.locator("body").innerText();
  spaceE2eAssert(!/Missing window:/i.test(bodyText), "all studio windows should render");
  spaceE2eAssert((await page.locator(".semio-node-graph-host").count()) > 0, "node graph host should render");
  spaceE2eAssert((await page.locator(".semio-text-editor-host").count()) > 0, "compiled dag editor should render");
  console.log("three studio windows rendered");

  let spawnMode: string | null = null;
  try {
    spawnMode = await spawnStudioE2eDrawFromEngagement(page);
    console.log(`spawn via ${spawnMode}`);
  } catch {
    spawnMode = await spawnStudioE2eDrawFromPalette(page);
    spaceE2eAssert(spawnMode === "palette", "draw spawn should work via engagement rail or command palette");
    console.log(`spawn via ${spawnMode}`);
  }

  await page.keyboard.press("Meta+z");
  await page.waitForTimeout(1500);
  console.log("undo issued");

  await openStudioE2eCommandPalette(page);
  const paletteInput = page.locator("[role='dialog'] [data-slot='command-input']").first();
  await paletteInput.fill("undo");
  await page.waitForTimeout(300);
  spaceE2eAssert((await page.locator('[data-slot="command-item"]').filter({ hasText: "Undo" }).count()) > 0, "undo should be in command palette");
  await paletteInput.fill("checkpoint");
  await page.waitForTimeout(300);
  spaceE2eAssert(
    (await page
      .locator('[data-slot="command-item"]')
      .filter({ hasText: /checkpoint/i })
      .count()) > 0,
    "checkpoint command should be in command palette",
  );
  console.log("studio commands in palette");
  await page.keyboard.press("Escape");

  await page.keyboard.press("Meta+f");
  await page.waitForTimeout(500);
  spaceE2eAssert((await page.locator("[role='dialog'] [data-slot='command-input']").count()) > 0, "find palette should open");
  console.log("find palette available");
  await page.keyboard.press("Escape");

  await page.getByRole("button", { name: "← Home" }).click({ force: true });
  await waitForStudioE2eCondition(page, ({ text }) => text.includes("Demo Studio") || text.includes("New Studio"), "home via studio bar", deadline);
  console.log("studio home bar navigation works");

  const demoStudioRow = page.locator('[data-row-id="studio:default"]');
  if (await demoStudioRow.count()) {
    await demoStudioRow.dblclick({ force: true });
    await page.waitForFunction(() => location.pathname.startsWith("/spaces/"), { timeout: 15_000 });
    await waitForStudioE2eCondition(page, ({ text }) => /Catalogue/i.test(text), "opened studio from home vfs", deadline);
    console.log("home vfs open studio works");
  }

  const criticalErrors = pageErrors.filter((message) => !isIgnorableStudioE2ePageError(message));
  if (criticalErrors.length !== pageErrors.length) {
    console.log(`ignored headless gpu errors: ${pageErrors.filter(isIgnorableStudioE2ePageError).join(" | ")}`);
  }
  spaceE2eAssert(criticalErrors.length === 0, `page errors: ${criticalErrors.join(" | ")}`);

  await browser.close();
  console.log("PASS: S studio end-to-end workflows verified");
}

export { STUDIO_E2E_HEADLESS_GPU_ERROR_FRAGMENTS, activateStudioE2eWorkflowWindow, expandStudioE2eWorkflowEngagement, isIgnorableStudioE2ePageError, openStudioE2e, openStudioE2eCommandPalette, runStudioE2eVerify, spaceE2eAssert, spawnStudioE2eDrawFromEngagement, spawnStudioE2eDrawFromPalette, waitForStudioE2eCondition };
