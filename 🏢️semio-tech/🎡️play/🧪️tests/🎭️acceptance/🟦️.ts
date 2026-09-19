// #region 🧲️Header
// 💻️ 🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts
// Specs: End-to-end acceptance coverage for semio-tech play — every app of the grid boots.
// Summary: The overview must list one card per authored pane. Then, for every pane, deep-links to
// `/#<paneId>` (which boots exactly that pane immediately), waits for its own `FrameworkOsShell` to report
// an outcome through the per-shell `data-shell-ready`/`data-shell-error`/`data-shell-not-found` beacon,
// requires "ready", lets the boot example announcement settle, requires the pane's error boundary to stay
// silent and fails on any page error, non-404 console error or refused input. The overview button must
// return to the overview.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

// #region 🔌️Adapters
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { expect, test, type Page } from "@playwright/test";
// #endregion 🔌️Adapters

//#region 🪪️PlayPanes
type PlayPane = { readonly variant: string; readonly label: string };

/** @emoji 🪪️ Reads the pane catalog as data instead of importing app modules, which drag the React runtime into Node's loader. */
function playPanes(): readonly PlayPane[] {
  const catalog = JSON.parse(readFileSync(join(import.meta.dirname, "..", "..", "🔨️modules", "🧩️runtime", "🔣️.json"), "utf8")) as { readonly groups: readonly { readonly panes: readonly PlayPane[] }[] };
  return catalog.groups.flatMap(group => group.panes);
}
//#endregion 🪪️PlayPanes

/** @emoji ⏱️ Cold wasm plugin boots can be slow — generous so the suite reports real defects, not infrastructure latency. */
const SHELL_READY_TIMEOUT_MS = 120_000;
const TEST_TIMEOUT_MS = 180_000;

/** @emoji 🕊️ Quiet window after the network settles: the shell announces the boot example after "ready",
 * and a refused announcement is only logged once the guest answers. */
const BOOT_SETTLE_MS = 2_000;

/** @emoji 🔇️ Drops resource 404s and the repo's `[DEBUG] `-prefixed temporary diagnostics. */
function significantConsoleErrors(messages: readonly string[]): string[] {
  return messages.filter(text => !/Failed to load resource:.*\b40[0-9]\b/i.test(text) && !text.startsWith("[DEBUG] "));
}

/** @emoji 👂️ Collects page errors, console errors and refused inputs — a refused input is the shell's
 * "The input could not be delivered" notice, logged only as a warning. */
function collectErrors(page: Page): { readonly pageErrors: string[]; readonly consoleErrors: string[]; readonly refusedInputs: string[] } {
  const pageErrors: string[] = [], consoleErrors: string[] = [], refusedInputs: string[] = [];
  page.on("pageerror", error => pageErrors.push(`${error.name}: ${error.message}`));
  page.on("console", message => {
    if (message.type() === "error") consoleErrors.push(message.text());
    else if (/^input #\d+ .* refused: /.test(message.text())) refusedInputs.push(message.text());
  });
  return { pageErrors, consoleErrors, refusedInputs };
}

type ShellOutcome = "ready" | "error" | "notFound";

/** @emoji 🚦️ Waits for the pane's own `[data-shell-id]` root to report an outcome. */
async function waitForPaneShellOutcome(page: Page, paneId: string): Promise<ShellOutcome> {
  await page.waitForFunction(id => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null;
    return !!el && (el.dataset.shellReady !== undefined || el.dataset.shellError !== undefined || el.dataset.shellNotFound !== undefined);
  }, paneId, { timeout: SHELL_READY_TIMEOUT_MS });
  return page.evaluate(id => {
    const el = document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement;
    return el.dataset.shellReady !== undefined ? "ready" : el.dataset.shellError !== undefined ? "error" : "notFound";
  }, paneId) as Promise<ShellOutcome>;
}

test.describe("semio-tech play", () => {
  test("lists one overview card per app", async ({ page }) => {
    const errors = collectErrors(page);
    const panes = playPanes();
    await page.goto("./");
    await page.keyboard.press("Escape");
    await expect(page.locator("[data-play-pane-card]")).toHaveCount(panes.length);
    await expect(page.locator('[data-slot="play-app-count"]')).toHaveText(`${panes.length} apps`);
    for (const pane of panes) await expect(page.locator(`[data-play-pane-card][data-pane-id="${pane.variant}"]`)).toHaveAttribute("aria-label", new RegExp(`^Open ${pane.label.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")}: `));
    expect(errors.pageErrors).toEqual([]);
    expect(significantConsoleErrors(errors.consoleErrors)).toEqual([]);
  });

  for (const pane of playPanes()) {
    test(`boots ${pane.label} (${pane.variant})`, async ({ page }) => {
      test.setTimeout(TEST_TIMEOUT_MS);
      const errors = collectErrors(page);
      await page.goto(`./#${pane.variant}`);
      const outcome = await waitForPaneShellOutcome(page, pane.variant);
      const detail = await page.evaluate(id => (document.querySelector(`[data-shell-id="${id}"]`) as HTMLElement | null)?.dataset.shellError ?? "", pane.variant);
      expect(outcome, `${pane.variant} shell outcome ${detail}`).toBe("ready");
      await page.waitForLoadState("networkidle");
      await page.waitForTimeout(BOOT_SETTLE_MS);
      await expect(page.locator(`[data-play-pane="${pane.variant}"] [data-play-pane-error]`)).toHaveCount(0);
      await expect(page.locator("[data-play-overview-button]")).toBeVisible();
      expect(errors.pageErrors).toEqual([]);
      expect(significantConsoleErrors(errors.consoleErrors)).toEqual([]);
      expect(errors.refusedInputs).toEqual([]);
      await page.locator("[data-play-overview-button]").click();
      await expect(page.locator("[data-play-overview]")).toBeVisible();
    });
  }
});
