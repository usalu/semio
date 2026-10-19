/** 🖱️ Collaboration e2e UI driving and row waits: the toolbar, dialog and option driving, the artifact creation, the row and editor waits, the check-in, the space opening and reopening and the screenshot helper. */
import { join } from "node:path";
import { spaceE2eAssert } from "@semio-tech/framework-os-dev/testing/studio";
import { COLLAB_E2E_LOCALES } from "../⚙️constants-relay-and-recorder/🟦️.ts";
import { collabOutDir } from "../🧭️scope-and-dev-servers/🟦️.ts";

//#region 🔖️CollabE2e
//#region 🔖️CollabE2eDom
/** 🕹️ Activates a shell-frozen toolbar button (contract §C0: `s-home-create-space`, `s-space-create-artifact`) by its
 * `data-ui-node-key` — the DOM id is window-scoped (`window:<window>/<key>`) — from the keyboard: focus + Enter, the
 * accessible path a keyboard user takes. A pointer cannot reach the button while the window's folded `Actions` chip
 * floats over the body's first line (measured `elementFromPoint`, ticket 26/09/23 C10); that layout defect is the
 * chrome owner's, and the keyboard path exercises the same `onAction` → dialog route. */
export async function collabClickToolbarButton(page: import("playwright").Page, elementId: string): Promise<void> {
  const button = page.locator(`[data-ui-node-key="${elementId}"]`).first();
  spaceE2eAssert((await button.count()) > 0, `toolbar button #${elementId} does not exist`);
  for (let attempt = 1; attempt <= 3; attempt += 1) {
    await button.focus();
    await button.press("Enter");
    if (await page.locator('[role="dialog"][data-slot="dialog-content"]').first().waitFor({ state: "visible", timeout: 5_000 }).then(() => true).catch(() => false)) return;
    console.log(`[collab-e2e] #${elementId} opened no dialog (attempt ${attempt}); pressing again`);
  }
}

export async function collabWaitForDialog(page: import("playwright").Page): Promise<void> {
  await page.locator('[role="dialog"][data-slot="dialog-content"]').waitFor({ state: "visible", timeout: 15_000 });
}

export async function collabSubmitDialog(page: import("playwright").Page): Promise<void> {
  await page.locator('[id="ui.dialog.submit"]').click();
  await page.locator('[role="dialog"][data-slot="dialog-content"]').waitFor({ state: "hidden", timeout: 15_000 });
}

/** 🕹️ Opens a `<Select id={triggerId}>` (Radix, portal-rendered) and clicks the option named `option` — exact text,
 * or a pattern for catalog-derived labels the hub's artifact-creation catalog words. */
export async function collabSelectOption(page: import("playwright").Page, triggerId: string, option: string | RegExp): Promise<void> {
  await page.locator(`#${triggerId}`).click();
  await page.getByRole("option", typeof option === "string" ? { name: option, exact: true } : { name: option }).first().click();
  await page.waitForTimeout(150);
}

/** 🏷️ The hub creation catalog's kind ids of the three collaboration editors STEP 3 and STEP 14 create. The picker is
 * driven by the kind id inside the option's encoded choice (`data-value`), never by its label: the hub labels a kind with its editor
 * app's label, so writer, draw and puzzle all read "Editor" (ticket 26/09/23 S15 finding c). */
export const COLLAB_E2E_KINDS = { writer: "text.document", draw: "2d.drawing", puzzle3d: "3d.puzzle" } as const;

/** 🌱️ Creates one artifact of catalog kind `kindId` in the open Space through `#s-space-create-artifact` and its
 * `createArtifact` dialog (`name` + the `kindChoice` catalog picker, opened from the keyboard), returning the new row's
 * bare id.
 * @see ../../../../../../../✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs */
export async function collabCreateArtifact(page: import("playwright").Page, name: string, kindId: string): Promise<string> {
  const before = await collabRowIds(page, "artifact");
  await collabClickToolbarButton(page, "s-space-create-artifact");
  await collabWaitForDialog(page);
  await page.locator("#name").fill(name);
  const option = page.locator(`[role="option"][data-value*='"kindId":"${kindId}"']`).first();
  const deadline = Date.now() + 90_000;
  for (;;) {
    await page.locator("#kindChoice").focus();
    await page.keyboard.press("Enter");
    if (await option.waitFor({ state: "visible", timeout: 5_000 }).then(() => true).catch(() => false)) break;
    if (Date.now() > deadline) throw new Error(`the kind picker never offered ${kindId} (catalog kinds load with the dialog)`);
  }
  await option.click();
  await collabSubmitDialog(page);
  return collabWaitForNewRow(page, "artifact", before, 300_000);
}

/** 🧭️ Waits until the Home (`s-home-create-space`) or Space (`s-space-create-artifact`) app is mounted, by the toolbar
 * button every later step clicks — an empty directory or space renders its empty state, never a table host. */
export async function collabWaitForApp(page: import("playwright").Page, toolbarId: "s-home-create-space" | "s-space-create-artifact", timeout: number): Promise<void> {
  await page.locator(`[data-ui-node-key="${toolbarId}"]`).first().waitFor({ state: "visible", timeout });
}

async function collabRowIds(page: import("playwright").Page, prefix: "space" | "artifact"): Promise<Set<string>> {
  const ids = await page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements) => elements.map((element) => element.getAttribute("data-ui-node-key") ?? ""));
  return new Set(ids);
}

/** ⏳️ Polls `page` until a row `data-ui-node-key` with `prefix` appears that was not in `before`, returning the
 * bare id (prefix stripped). Used for both same-page ("the row appears") and cross-page ("user2 sees the
 * same row") assertions — the caller decides which page to poll. */
async function collabWaitForNewRow(page: import("playwright").Page, prefix: "space" | "artifact", before: ReadonlySet<string>, deadlineMs: number): Promise<string> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const current = await collabRowIds(page, prefix);
    for (const id of current) {
      if (!before.has(id)) return id.slice(prefix.length + 1);
    }
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for a new ${prefix}: row`);
}

/** ⏳️ Polls `page` until a `prefix` row whose text holds `name` appears, returning its bare id: a freshly signed-in Home
 * streams its existing rows in over several pages, so "a row that was not there before" can be an OLD space. */
export async function collabWaitForNamedRow(page: import("playwright").Page, prefix: "space" | "artifact", name: string, deadlineMs: number): Promise<string> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const key = await collabFindRow(page, prefix, (row) => row.text.includes(name));
    if (key !== null) return key.slice(prefix.length + 1);
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for a ${prefix}: row named ${JSON.stringify(name)}`);
}

/** 🧭️ Finds a `prefix` row of a WINDOWED table — only the rows inside its scroll window exist in the DOM ("Rows 1–47 of
 * 53" measured on a Home with 52 hub spaces): the rendered rows first, then every `table-window-scroll` container paged from
 * top to bottom, as a human scrolls. Leaves the table scrolled to the row it found. */
async function collabFindRow(page: import("playwright").Page, prefix: "space" | "artifact", matches: (row: { readonly key: string; readonly text: string }) => boolean): Promise<string | null> {
  const rendered = async (): Promise<string | null> =>
    (await page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements) => elements.map((element) => ({ key: element.getAttribute("data-ui-node-key") ?? "", text: element.textContent ?? "" })))).find(matches)?.key ?? null;
  const shown = await rendered();
  if (shown !== null) return shown;
  const scrollers = page.locator('[data-slot="table-window-scroll"]');
  for (let index = 0, count = await scrollers.count(); index < count; index += 1) {
    const scroller = scrollers.nth(index);
    for (let top = 0; ; ) {
      const metrics = await scroller.evaluate((element, y) => {
        element.scrollTop = y;
        return { top: element.scrollTop, scrollHeight: element.scrollHeight, clientHeight: element.clientHeight };
      }, top);
      await page.waitForTimeout(400);
      const found = await rendered();
      if (found !== null) return found;
      if (metrics.top + metrics.clientHeight >= metrics.scrollHeight) break;
      top = metrics.top + Math.max(1, Math.floor(metrics.clientHeight * 0.8));
    }
  }
  return null;
}

/** 🏘️ Every load of `/spaces/<id>` that did not mount the Space app, with the window fault it showed instead — printed in
 * the summary, never silently absorbed (a hard load measured landing on a faulted window: `actor-activation.revoked`). */
export const collabRouteMisses: string[] = [];

/** 🖱️ Activates one named row action (`Open: <id>` / `Öffnen: <id>`, `Share: <name>` / `Teilen: <name>`, case-insensitive) from the
 * keyboard — the ACTIONS column can sit past the window's right edge, where a pointer click times out. The label is the one of
 * the shell's locale, so the selector accepts every locale's label of the verb. */
export async function collabRowAction(page: import("playwright").Page, prefix: "space" | "artifact", id: string, verb: "open" | "share"): Promise<void> {
  const labels = [...new Set(Object.values(COLLAB_E2E_LOCALES).map((locale) => locale[verb]))];
  const action = page.locator(labels.map((label) => `[data-ui-node-key="${prefix}:${id}"] button[aria-label^="${label}:" i]`).join(", ")).first();
  await action.waitFor({ state: "attached", timeout: 30_000 });
  await action.focus();
  await action.press("Enter");
}

/** ⏳️ Waits for an editable text surface — a hub document's editor mounts only after its plugin module is installed from
 * the hub catalog (the "Loading plugin …" band), which takes tens of seconds on a fresh device. */
export async function collabWaitForEditor(page: import("playwright").Page, deadlineMs: number): Promise<boolean> {
  return collabTextEditor(page).waitFor({ state: "attached", timeout: deadlineMs }).then(() => true).catch(() => false);
}

/** ⌨️ The input sink of the document's text editor (`TextEditorHost`'s hidden textarea, which mirrors the session's buffer): never
 * the first textarea of the page, which after a reload can be a panel field such as the History panel's check-in message
 * (collab STEP 8 typed its marker there in run `c12collab-1`, ticket 26/09/23 C12). */
export function collabTextEditor(page: import("playwright").Page): import("playwright").Locator {
  return page.locator(".semio-text-editor-host textarea").first();
}

/** 🔁️ When each page last re-opened its Space app or re-bootstrapped its directory (see {@link collabSettleAfterLoad}). */
export const collabSpaceReopenedAt = new WeakMap<import("playwright").Page, number>();

/** ⏳️ A hard load mounts the Space app, then the restored identity re-establishes the session and the route re-opens the
 * space ~5–7 s later ("space index opening failed: document closed", routed to U5 on 26/09/26), closing whatever was opened
 * in between: waits until the page has been quiet for `quietMs` and the Space app is mounted again. */
async function collabSettleAfterLoad(page: import("playwright").Page, quietMs = 12_000, deadlineMs = 90_000): Promise<boolean> {
  const started = Date.now();
  while (Date.now() - started < deadlineMs) {
    const quietSince = Math.max(started, collabSpaceReopenedAt.get(page) ?? 0);
    if (Date.now() - quietSince >= quietMs && (await page.locator('[data-ui-node-key="s-space-create-artifact"]').count()) > 0) return true;
    await page.waitForTimeout(500);
  }
  return false;
}

/** 📌️ Opens the History panel of the document open on `page` and unfolds its History section (a tree section that renders
 * closed), returning the `#s-checkin` control. */
export async function collabOpenCheckin(page: import("playwright").Page): Promise<import("playwright").Locator> {
  const historyTab = page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"]');
  spaceE2eAssert((await historyTab.count()) > 0, "no framework.panel.history tab found — cannot reach #s-checkin");
  await historyTab.first().click();
  const checkin = page.locator('[id="s-checkin"]');
  const deadline = Date.now() + 20_000;
  while (Date.now() < deadline && !(await checkin.first().isVisible().catch(() => false))) {
    const closed = page.locator('[data-slot="tree-section-row"]:not([data-state="open"])').filter({ hasText: /^\s*(history|verlauf)\b/iu });
    if ((await closed.count()) > 0) await closed.first().click();
    await page.waitForTimeout(500);
  }
  await checkin.first().waitFor({ state: "visible", timeout: 1_000 });
  return checkin.first();
}

/** 🕐️ The minute (epoch minutes, UTC) an artifact row's "updated" cell shows — en `2026-09-28 21:58 UTC`, de
 * `28.09.2026, 21:58 UTC` — or `null` when the row shows none. */
function collabRowUpdatedMinute(text: string): number | null {
  const en = /(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}) UTC/u.exec(text);
  const de = /(\d{2})\.(\d{2})\.(\d{4}), (\d{2}):(\d{2}) UTC/u.exec(text);
  const parts = en ? [en[1], en[2], en[3], en[4], en[5]] : de ? [de[3], de[2], de[1], de[4], de[5]] : null;
  return parts === null ? null : Math.floor(Date.UTC(Number(parts[0]), Number(parts[1]) - 1, Number(parts[2]), Number(parts[3]), Number(parts[4])) / 60_000);
}

/** 🕐️ Waits until `page`'s row for `artifactId` shows `minute` or later in its "updated" cell (bounded); answers the last row read. */
export async function collabAwaitRowUpdatedMinute(page: import("playwright").Page, artifactId: string, minute: number, deadlineMs: number): Promise<{ readonly text: string; readonly minute: number | null }> {
  const deadline = Date.now() + deadlineMs;
  let text = "";
  while (Date.now() < deadline) {
    text = (await page.locator(`[data-ui-node-key="artifact:${artifactId}"]`).innerText().catch(() => "")) ?? "";
    const shown = collabRowUpdatedMinute(text);
    if (shown !== null && shown >= minute) return { text, minute: shown };
    await page.waitForTimeout(1_000);
  }
  return { text, minute: collabRowUpdatedMinute(text) };
}

/** 📌️ The focused program's active checkpoint id, from the shell root's `data-history-json` (`null` before any). */
export async function collabCurrentCheckpointId(page: import("playwright").Page): Promise<string | null> {
  return page.evaluate(() => (JSON.parse(document.querySelector("[data-history-json]")?.getAttribute("data-history-json") ?? "null") as { readonly currentCheckpointId?: string | null } | null)?.currentCheckpointId ?? null);
}

/** ⏳️ The hub Check In's terminal status sentence on `page` (`#s-checkin-status`: ready, cancelled or a refusal), or the last
 * sentence it showed when `deadlineMs` ran out — a running check-in reads "Checking in… n/m". */
export async function collabAwaitCheckinStatus(page: import("playwright").Page, deadlineMs: number): Promise<string> {
  const status = page.locator('[id="s-checkin-status"]').first();
  const deadline = Date.now() + deadlineMs;
  let text = "";
  while (Date.now() < deadline) {
    text = ((await status.innerText().catch(() => "")) ?? "").trim();
    if (text !== "" && (await page.locator('[id="s-checkin-abort"]').count()) === 0) return text;
    await page.waitForTimeout(250);
  }
  return text;
}

/** 🚪️ Re-opens the writer document of steps 3–13 for both humans from the Space index (a step before navigated away) and
 * waits until both editors mounted. */
export async function collabReopenWriter(opts: { readonly user1: import("playwright").Page; readonly user2: import("playwright").Page; readonly spaceId: string; readonly artifactId: string; readonly purpose: string }): Promise<void> {
  for (const [label, page] of [["user1", opts.user1], ["user2", opts.user2]] as const) {
    await collabOpenSpace(page, opts.spaceId);
    await collabWaitForRow(page, "artifact", opts.artifactId, 30_000);
    await collabRowAction(page, "artifact", opts.artifactId, "open").catch(() => undefined);
    spaceE2eAssert(await collabWaitForEditor(page, 120_000), `${label}'s writer editor never re-mounted for ${opts.purpose}`);
  }
}

/** 🪪️ The hub user id of the human signed in on `page`: the connection report names users only by id, and the shell
 * remembers its minted capability (`semio.os.hub-session-capability.v1`) with that id. */
export async function collabHubUserId(page: import("playwright").Page): Promise<string | null> {
  return page.evaluate(() => {
    for (const storage of [globalThis.sessionStorage, globalThis.localStorage]) {
      const stored = storage?.getItem("semio.os.hub-session-capability.v1");
      if (stored) return (JSON.parse(stored) as { readonly userId?: string }).userId ?? null;
    }
    return null;
  });
}

/** 🏘️ Loads `/spaces/<id>` until the Space app mounts and settles (at most three loads), recording every miss. */
export async function collabOpenSpace(page: import("playwright").Page, spaceId: string): Promise<void> {
  for (let attempt = 1; attempt <= 3; attempt += 1) {
    await page.goto(`${new URL(page.url()).origin}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
    if (await collabWaitForApp(page, "s-space-create-artifact", 30_000).then(() => true).catch(() => false) && (await collabSettleAfterLoad(page))) return;
    const shown = await page.evaluate(() => (document.querySelector('[data-slot="window-body"]')?.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 160)).catch(() => "");
    collabRouteMisses.push(`/spaces/${spaceId} load ${attempt}: ${JSON.stringify(shown)}`);
  }
  throw new Error(`the Space app never mounted at /spaces/${spaceId} in 3 loads`);
}

export async function collabWaitForRow(page: import("playwright").Page, prefix: "space" | "artifact", id: string, deadlineMs: number): Promise<void> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    if ((await collabFindRow(page, prefix, (row) => row.key === `${prefix}:${id}`)) !== null) return;
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for [data-ui-node-key="${prefix}:${id}"]`);
}

export async function collabScreenshot(page: import("playwright").Page, label: string): Promise<void> {
  try {
    await page.screenshot({ path: join(collabOutDir(), `🧪️3-c-${label}.png`) });
  } catch {
  }
}
//#endregion 🔖️CollabE2eDom
//#endregion 🔖️CollabE2e
