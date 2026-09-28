/** 👥️ Two humans, one hub, every kind the hub can create — driven in two isolated browser profiles of the React `s` shell.
 *
 * Per kind the hub's creation catalog offers: A creates it from the Space app (the creation saga opens it for A), B opens it
 * from the Space index, both presence rosters hold two peers, every document window takes focus with no owner fault, A
 * edits → B sees it, B edits → A sees it, A undoes their OWN edit (B's stays), B undoes theirs (both back to the start), B
 * redoes (both see it again), both reload and reopen and converge. One row per kind; per-kind faults; screenshots on failure.
 * The document is witnessed as the human sees it (window-body text plus element / SVG / canvas counts and a key digest)
 * and, when an admin capability file is given, as the hub holds it (`headSeq`). Locale: the browser language both profiles
 * boot in (`en` → `en-US`, `de` → `de-DE`).
 *
 * Promoted from the session-12/13 ticket harness `wp-c10`/`wp-c11` (`c11-collab-matrix.mjs`, `c11-lib.mjs`,
 * `c11-journey.mjs`, ticket 26/09/23); the edit verbs and staged arguments are the program matrix's pins.
 * @see ../🧮️program-matrix/🟦️.ts
 * @see ../🤝️collaboration/🟦️.ts — the fresh-local-hub collaboration story
 */

import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import type { Browser, BrowserContext, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { FAULT, NOISE, clickUncovered, fillStagedArgument, readMatrixPins, readShell, submitStagedVerb, unfoldActionsRail, withDevServe } from "../🧮️program-matrix/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";
import { hubProbeCall, hubProbeOpenDocument, hubProbeSignIn } from "../../../../../../../🌎️hub/🤝️integration-harness/🟦️.ts";
import { createSpaceCommandV1 } from "../../../📇️directory/🏘️spaces/🟦️.ts";
import { directoryCommandRequestJson, sealDirectoryCommandRequestV1, type DirectorySpaceRole } from "../../../📇️directory/🧬️schema/🟦️.ts";

//#region 🔖️Sessions
/** 🔑️ One human: a hub credential. */
export type Human = Readonly<{ label: string; email: string; password: string }>;

/** 🌐️ One human's isolated browser profile on one serve, with its console and hub traffic lines. */
export type Session = { human: Human; url: string; context: BrowserContext; page: Page; lines: string[] };

const started = Date.now();
const ms = (): number => Date.now() - started;
const pause = (session: Session, duration: number): Promise<void> => session.page.waitForTimeout(duration);
const originOf = (session: Session): string => new URL(session.url).origin;
export const faultsSince = (session: Session, cursor: number): string[] => session.lines.slice(cursor).filter((line) => FAULT.test(line) && !NOISE.test(line)).map((line) => line.slice(0, 240));
const short = (text: string): string => (text.length <= 160 ? text : `${text.slice(0, 80)}…${text.slice(-70)}`);

export async function openSessions(browser: Browser, urls: readonly string[], humans: readonly Human[], locale: string): Promise<Session[]> {
  const sessions: Session[] = [];
  for (const [index, human] of humans.entries()) {
    const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale });
    const page = await context.newPage();
    const lines: string[] = [];
    page.on("console", (message) => {
      const text = message.text();
      if (/Download the React DevTools|\[vite\]|agent-bridge|status of 404/u.test(text)) return;
      lines.push(`${ms()} ${message.type()} ${text.slice(0, 900)}`);
    });
    page.on("pageerror", (error) => lines.push(`${ms()} pageerror ${String(error).slice(0, 900)}`));
    page.on("websocket", (socket) => socket.on("close", () => lines.push(`${ms()} ws-closed ${socket.url().replace(/^wss?:\/\/[^/]+/u, "").slice(0, 160)}`)));
    page.on("request", (request) => {
      if (request.url().includes("/directory/event-page/v1?after=0")) lines.push(`${ms()} request /directory/event-page/v1?after=0`);
    });
    sessions.push({ human, url: urls[index] ?? urls[0]!, context, page, lines });
  }
  return sessions;
}

async function until<T>(probe: () => Promise<T | null>, deadlineMs: number, stepMs = 500): Promise<T | null> {
  const deadline = Date.now() + deadlineMs;
  let value = await probe();
  while (!value && Date.now() < deadline) {
    await new Promise((resolveDelay) => setTimeout(resolveDelay, stepMs));
    value = await probe();
  }
  return value;
}

export async function boot(session: Session): Promise<void> {
  await session.page.goto(session.url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await session.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
}

export async function signIn(session: Session): Promise<void> {
  const page = session.page;
  await page.locator('[data-semio-hub-sign-in=""]').first().click();
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 30_000 });
  await form.locator('input[type="email"]').fill(session.human.email);
  await form.locator('input[type="password"]').fill(session.human.password);
  await form.locator('[id="os.hub.signIn.submit"]').click();
  await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 60_000 });
  await page.locator('[data-semio-hub-workspace] [id="os.hub.signIn.cancel"]').click();
  await page.locator("[data-semio-hub-workspace]").waitFor({ state: "hidden", timeout: 15_000 });
}

/** ⌨️ Keyboard activation of a UI node by its stable `data-ui-node-key`. */
export async function activate(page: Page, key: string): Promise<void> {
  const node = page.locator(`[data-ui-node-key="${key}"]`).first();
  await node.waitFor({ state: "attached", timeout: 30_000 });
  await node.focus();
  await node.press("Enter");
}

export const dialog = (page: Page) => page.locator('[role="dialog"][data-slot="dialog-content"]');

export async function selectOption(page: Page, triggerId: string, option: RegExp): Promise<void> {
  await page.locator(`[id="${triggerId}"]`).click();
  await page.getByRole("option", { name: option }).first().click();
  await page.waitForTimeout(200);
}

export async function submitDialog(page: Page): Promise<void> {
  await page.locator('[id="ui.dialog.submit"]').click();
  await dialog(page).waitFor({ state: "hidden", timeout: 20_000 });
}

async function rowIds(page: Page, prefix: string): Promise<Set<string>> {
  return new Set(await page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements) => elements.map((element) => element.getAttribute("data-ui-node-key") ?? "")));
}

async function waitNewRow(page: Page, prefix: string, before: ReadonlySet<string>, deadlineMs: number): Promise<string> {
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    for (const id of await rowIds(page, prefix)) if (!before.has(id)) return id.slice(prefix.length + 1);
    await page.waitForTimeout(400);
  }
  throw new Error(`no new ${prefix} row within ${deadlineMs} ms`);
}

/** 🧭️ Pages every windowed table (`table-window-scroll`) top to bottom, as a human scrolls, until `found` answers. */
export async function pageWindowedTables<T>(page: Page, found: () => Promise<T | null>): Promise<T | null> {
  let hit = await found();
  const scrollers = page.locator('[data-slot="table-window-scroll"]');
  for (let index = 0, count = await scrollers.count(); index < count && hit === null; index += 1) {
    for (let top = 0; ; ) {
      const metrics = await scrollers
        .nth(index)
        .evaluate((element, y) => {
          element.scrollTop = y;
          return { top: element.scrollTop, scrollHeight: element.scrollHeight, clientHeight: element.clientHeight };
        }, top)
        .catch(() => null);
      await page.waitForTimeout(400);
      hit = await found();
      if (metrics === null || hit !== null || metrics.top + metrics.clientHeight >= metrics.scrollHeight) break;
      top = metrics.top + Math.max(1, Math.floor(metrics.clientHeight * 0.8));
    }
  }
  return hit;
}

async function waitRow(page: Page, prefix: string, id: string, deadlineMs: number): Promise<void> {
  const row = page.locator(`[data-ui-node-key="${prefix}:${id}"]`).first();
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    if ((await pageWindowedTables(page, async () => ((await row.count()) > 0 ? true : null))) !== null) return;
    await page.waitForTimeout(500);
  }
  throw new Error(`timeout waiting for [data-ui-node-key="${prefix}:${id}"] (${deadlineMs} ms, windowed tables paged)`);
}

export async function waitNamedRow(page: Page, prefix: string, name: string, deadlineMs: number): Promise<string> {
  const find = (): Promise<string | null> => page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements, wanted) => elements.find((element) => (element.textContent ?? "").includes(wanted))?.getAttribute("data-ui-node-key") ?? null, name);
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const key = await pageWindowedTables(page, find);
    if (key !== null) return key.slice(prefix.length + 1);
    await page.waitForTimeout(500);
  }
  throw new Error(`no ${prefix} row named ${JSON.stringify(name)} within ${deadlineMs} ms (windowed tables paged)`);
}

export async function clickRowAction(page: Page, prefix: string, id: string, pattern: RegExp): Promise<string> {
  const buttons = page.locator(`[data-ui-node-key="${prefix}:${id}"] button`);
  for (let index = 0, count = await buttons.count(); index < count; index += 1) {
    const button = buttons.nth(index);
    const name = `${(await button.getAttribute("aria-label")) ?? ""} ${(await button.getAttribute("title")) ?? ""} ${(await button.textContent()) ?? ""}`;
    if (pattern.test(name)) {
      await button.focus();
      await button.press("Enter");
      return name.trim();
    }
  }
  throw new Error(`row ${prefix}:${id} has no action matching ${pattern}`);
}

/** 🪞️ What the shell publishes about presence and the execution target of the open document. */
const readStatus = (page: Page): Promise<{ peers: string[]; peerLabels: string[]; executionTarget: string[]; windows: string[]; error: string | null }> =>
  page.evaluate(() => {
    const text = (element: Element | null): string => ((element as HTMLElement | null)?.innerText ?? "").replace(/\s+/gu, " ").trim();
    const peerNodes = [...document.querySelectorAll('[id="s-presence-peers"] [data-row-id^="peer:"]:not([data-row-id="peer:overflow"])')];
    return {
      peers: peerNodes.map((element) => element.getAttribute("data-row-id") ?? ""),
      peerLabels: peerNodes.map((element) => text(element).slice(0, 64)),
      executionTarget: [...document.querySelectorAll("[data-semio-execution-target-status]")].map((element) => `${element.getAttribute("data-semio-execution-target-status")}:${text(element).slice(0, 80)}`),
      windows: [...document.querySelectorAll('[data-slot="window"]')].map((element) => element.id).slice(0, 12),
      error: document.documentElement.getAttribute("data-semio-os-error"),
    };
  });
//#endregion 🔖️Sessions

//#region 🔖️Journey
/** 🧩️ Kind ids whose dialect does not name its plugin (the catalog names the dialect only). */
const PLUGIN_BY_KIND: Readonly<Record<string, string>> = { "2d.drawing": "draw", "text.document": "writer", "2d.puzzle": "puzzle", "3d.puzzle": "puzzle", "5d.puzzle": "puzzle", "2d.block": "block", "3d.block": "block", "5d.block": "block", "3d.wfcgrid3d": "wfc", "animate.presentation": "animate" };

/** 🪟️ The shell's own chrome windows — a document is mounted once a window other than these shows. */
const SHELL_WINDOWS = new Set(["framework.window.table", "s-home-main"]);

/** ⏱️ How often a mount is polled; a mount is stamped when the poll that sees it runs, before the panels are unfolded, so a
 * timing carries at most one poll interval of slack (it used to carry a 1 s poll plus ~2 s of panel clicks and pauses). */
const MOUNT_POLL_MS = 200;

async function awaitMounted(session: Session, deadlineMs: number) {
  const mounted = await until(async () => {
    const shell = await readShell(session.page);
    const status = await readStatus(session.page);
    const documentWindows = shell.windowIds.filter((id) => !SHELL_WINDOWS.has(id));
    const painted = await session.page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"]')].some((body) => body.querySelectorAll("*").length > 20));
    return documentWindows.length > 0 && painted && status.executionTarget.length === 0 && !new URL(session.page.url()).pathname.endsWith("/") ? { ...shell, windowIds: documentWindows, mountedAt: Date.now() } : null;
  }, deadlineMs, MOUNT_POLL_MS);
  if (!mounted) throw new Error(`not mounted within ${deadlineMs} ms: ${JSON.stringify(await readStatus(session.page))}`);
  await unfoldActionsRail(session.page);
  await session.page.locator('[data-slot="panel-tab-button"][id="framework.panel.history"]').first().click({ force: true }).catch(() => undefined);
  await pause(session, 1_000);
  return mounted;
}

/** ⏳️ A hard load mounts the Space app, then the restored identity re-opens the space seconds later: waits until the
 * load has been quiet for `quietMs` and the Space app is mounted again. */
async function settleAfterLoad(session: Session, cursor: number, quietMs = 12_000, deadlineMs = 90_000): Promise<boolean> {
  const create = session.page.locator('[data-ui-node-key="s-space-create-artifact"]').first();
  const begun = Date.now();
  let quietSince = Date.now();
  let seen = cursor;
  while (Date.now() - begun < deadlineMs) {
    const fresh = session.lines.slice(seen);
    seen = session.lines.length;
    if (fresh.some((line) => /space index opening failed|event-page\/v1\?after=0/u.test(line))) quietSince = Date.now();
    if (Date.now() - quietSince >= quietMs && (await create.count()) > 0) return true;
    await session.page.waitForTimeout(500);
  }
  return false;
}

/** 🏠️ Sign-in re-bootstraps Home's directory seconds later (a second `event-page/v1?after=0` under a new socket grant), and a
 * dialog opened before that is closed by it: waits until Home has been quiet for `quietMs` and is mounted. */
export async function settleHome(session: Session, quietMs = 8_000, deadlineMs = 60_000): Promise<boolean> {
  const create = session.page.locator('[data-ui-node-key="s-home-create-space"]').first();
  const begun = Date.now();
  let quietSince = Date.now();
  let seen = 0;
  while (Date.now() - begun < deadlineMs) {
    const fresh = session.lines.slice(seen);
    seen = session.lines.length;
    if (fresh.some((line) => /event-page\/v1\?after=0|ws-closed \/directory\/socket/u.test(line))) quietSince = Date.now();
    if (Date.now() - quietSince >= quietMs && (await create.count()) > 0) return true;
    await session.page.waitForTimeout(500);
  }
  return false;
}

/** 🏘️ Opens the Space app at `/spaces/<id>`, retrying a hard-load miss up to three times; every miss is recorded. */
async function openSpace(session: Session, spaceId: string, misses: unknown[]): Promise<void> {
  const create = session.page.locator('[data-ui-node-key="s-space-create-artifact"]').first();
  if (new URL(session.page.url()).pathname === `/spaces/${spaceId}` && (await create.count()) > 0) return;
  for (let attempt = 1; attempt <= 3; attempt += 1) {
    const cursor = session.lines.length;
    await session.page.goto(`${originOf(session)}/spaces/${spaceId}`, { waitUntil: "domcontentloaded" });
    if ((await create.waitFor({ state: "attached", timeout: 60_000 }).then(() => true).catch(() => false)) && (await settleAfterLoad(session, cursor))) return;
    misses.push({ human: session.human.label, spaceId, attempt, windows: (await readStatus(session.page)).windows, at: new Date().toISOString() });
  }
  throw new Error(`the Space app never mounted at /spaces/${spaceId} in 3 loads`);
}

async function openRow(session: Session, spaceId: string, artifactId: string, misses: unknown[]): Promise<void> {
  await openSpace(session, spaceId, misses);
  await waitRow(session.page, "artifact", artifactId, 90_000);
  await clickRowAction(session.page, "artifact", artifactId, /^(open|öffnen)\b/iu);
}

type CreatableKind = { value: string; label: string; kindId: string };

/** 🗂️ The creation catalog's kinds as the Space app's picker offers them (each option's value carries the kind id). */
async function creatableKinds(session: Session): Promise<CreatableKind[]> {
  await activate(session.page, "s-space-create-artifact");
  await dialog(session.page).waitFor({ state: "visible", timeout: 20_000 });
  const offered = await until(async () => {
    await session.page.locator('[id="kindChoice"]').focus();
    await session.page.keyboard.press("Enter");
    return session.page.locator('[role="option"]').first().waitFor({ state: "visible", timeout: 5_000 }).then(() => true).catch(() => null);
  }, 120_000, 1_000);
  if (!offered) throw new Error("the Space app's kind picker offered no artifact kind within 120 s");
  const options = await session.page.locator('[role="option"]').evaluateAll((elements) => elements.map((element) => ({ value: element.getAttribute("data-value") ?? "", label: (element.textContent ?? "").trim() })));
  await session.page.keyboard.press("Escape");
  await session.page.keyboard.press("Escape");
  await dialog(session.page).waitFor({ state: "hidden", timeout: 20_000 }).catch(() => undefined);
  return options.map((option) => ({ ...option, kindId: String((JSON.parse(option.value) as { kindId?: unknown }).kindId ?? "") }));
}

async function createArtifact(session: Session, name: string, kind: CreatableKind, budgetMs: number): Promise<string> {
  const before = await rowIds(session.page, "artifact");
  await activate(session.page, "s-space-create-artifact");
  await dialog(session.page).waitFor({ state: "visible", timeout: 20_000 });
  await session.page.locator("#name").fill(name);
  await session.page.locator('[id="kindChoice"]').focus();
  await session.page.keyboard.press("Enter");
  await session.page.locator(`[role="option"][data-value="${kind.value.replace(/"/gu, '\\"')}"]`).first().click();
  await submitDialog(session.page);
  return waitNewRow(session.page, "artifact", before, budgetMs);
}

/** 📄️ The document as the human sees it: every window body's text plus element / SVG / canvas counts and an FNV digest
 * of its keyed nodes, action panes and presence overlays removed. */
const docText = (session: Session): Promise<string> =>
  session.page.evaluate(() =>
    [...document.querySelectorAll('[data-slot="window-body"]')]
      .map((body) => {
        const clone = body.cloneNode(true) as Element;
        clone.querySelectorAll('[data-slot="window-action-pane"], [data-slot="window-action-pane-overlay"], [data-slot="utility-bar"], [data-slot="canvas-presence-overlay"]').forEach((element) => element.remove());
        const keys = [...clone.querySelectorAll("[data-ui-node-key], [data-node-id], [data-row-id], svg [id]")].map((element) => element.getAttribute("data-ui-node-key") ?? element.getAttribute("data-node-id") ?? element.getAttribute("data-row-id") ?? element.id).sort().join("|");
        let hash = 0x811c9dc5;
        for (let index = 0; index < keys.length; index += 1) hash = Math.imul(hash ^ keys.charCodeAt(index), 0x01000193) >>> 0;
        return `${(clone.textContent ?? "").replace(/\s+/gu, " ").trim()} #${clone.querySelectorAll("*").length}/${clone.querySelectorAll("svg *").length}/${clone.querySelectorAll("canvas").length}/${hash.toString(16)}`;
      })
      .join(" ¦ "),
  );

async function awaitText(session: Session, predicate: (now: string) => boolean, deadlineMs: number): Promise<string | { missed: string }> {
  const seen = await until(async () => {
    const now = await docText(session);
    return predicate(now) ? now : null;
  }, deadlineMs);
  return seen ?? { missed: await docText(session) };
}

const seenText = (value: string | { missed: string }): string => (typeof value === "string" ? value : value.missed);

/** ✏️ Dispatches the plugin's pinned document verb once (staged arguments filled); answers whether the author's own view
 * moved within 20 s. A still-open staged form is submitted directly. */
/** 👥️ The pinned arguments of `plugin`'s verb made distinct per human: a text value gains the human's label, a numeric one
 * (a seed, a count) is offset by 6 for the second human, a JSON string literal gains it inside its quotes, a live id stays
 * live — so two humans never write the same value and
 * each edit is a visible change (a set-style verb would otherwise rewrite the first human's value unchanged). */
export function personalArgs(pinned: Readonly<Record<string, string>>, label: string, liveId: string): Record<string, string> {
  const personal = (value: string): string => {
    if (value === liveId) return value;
    if (/^-?\d+(\.\d+)?$/u.test(value)) return String(Number(value) + (label === "user2" ? 6 : 0));
    if (/^".*"$/su.test(value)) return JSON.stringify(`${JSON.parse(value) as string} ${label}`);
    return `${value} ${label}`;
  };
  return Object.fromEntries(Object.entries(pinned).map(([key, value]) => [key, personal(value)]));
}

async function edit(session: Session, plugin: string, pins: ReturnType<typeof readMatrixPins>) {
  const verb = pins.pluginVerbs[plugin];
  if (verb === undefined) throw new Error(`no pinned document verb for plugin ${plugin}`);
  const before = await docText(session);
  const staged = await session.page.locator(`[id$=".action.${verb}.execute"]`).first().isVisible().catch(() => false);
  if (!staged) await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="action.${verb}"]`);
  await pause(session, 1_000);
  for (const [key, value] of Object.entries(personalArgs(pins.pluginArgs[`${plugin}.${verb}`] ?? pins.pluginArgs[verb] ?? {}, session.human.label, pins.liveId))) await fillStagedArgument(session.page, key, value, pins.liveId);
  await submitStagedVerb(session.page, verb);
  const after = await awaitText(session, (now) => now !== before, 20_000);
  return { verb, after: seenText(after), applied: typeof after === "string" };
}

async function undo(session: Session, verb: "undo" | "redo" = "undo") {
  const before = await docText(session);
  await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="action.${verb}"]`);
  const after = await awaitText(session, (now) => now !== before, 20_000);
  return { after: seenText(after), undone: typeof after === "string" };
}

/** 🔑️ The capability the local hub launcher publishes in `admin-capability.json`, and its refresh contract: an admin-relay
 * capability lives 15 minutes; writing the sibling `admin-request` file asks the launcher (the `startLocalHub` hold) for a
 * fresh one, which it writes into the same capability file. */
const readCapability = (file: string): string => String((JSON.parse(readFileSync(file, "utf8")) as { capability?: unknown }).capability ?? "");

async function refreshCapability(file: string): Promise<boolean> {
  const before = readCapability(file);
  writeFileSync(join(dirname(file), "admin-request"), "");
  for (const deadline = Date.now() + 15_000; Date.now() < deadline; ) {
    await new Promise((resolveDelay) => setTimeout(resolveDelay, 500));
    const now = (() => {
      try {
        return readCapability(file);
      } catch {
        return before;
      }
    })();
    if (now !== before && now.length > 0) return true;
  }
  return false;
}

/** 🗄️ The hub's own head sequence of a document through the admin API, when an admin capability file was given; an expired
 * capability (401) is refreshed once through the launcher's `admin-request` contract and the read retried. */
export async function hubHead(hub: string, adminCapabilityFile: string | null, artifactId: string): Promise<number | string | null> {
  if (!adminCapabilityFile) return null;
  const read = (): Promise<Response | null> => fetch(`${hub}/admin/api/documents`, { headers: { authorization: `Bearer ${readCapability(adminCapabilityFile)}` }, signal: AbortSignal.timeout(60_000) }).catch(() => null);
  let response = await read();
  if (response?.status === 401 && (await refreshCapability(adminCapabilityFile))) response = await read();
  if (response === null || !response.ok) return `admin ${response?.status ?? "unreachable"}`;
  const body = (await response.json()) as { rows: { descriptor: { documentId: string }; headSeq: number }[] };
  return body.rows.find((row) => row.descriptor.documentId === artifactId)?.headSeq ?? null;
}

/** 🏠️ Waits until `session`'s Home lists `spaceId`, reloading once when the hub never tells it (recorded as a miss). */
async function awaitSharedSpace(session: Session, spaceId: string, misses: unknown[]): Promise<void> {
  if (await waitRow(session.page, "space", spaceId, 45_000).then(() => true).catch(() => false)) return;
  misses.push({ human: session.human.label, spaceId, home: true, at: new Date().toISOString() });
  await session.page.reload({ waitUntil: "domcontentloaded" });
  await waitRow(session.page, "space", spaceId, 120_000);
}
//#endregion 🔖️Journey

//#region 🔖️Journeys
/** 🧭️ What one kind's journey gets once A created and opened the document and B opened it from the Space index. */
type JourneyContext = Readonly<{ A: Session; B: Session; check: (name: string, pass: boolean, detail: unknown) => void; plugin: string; pins: ReturnType<typeof readMatrixPins>; options: TwoHumanOptions; spaceId: string; artifactId: string; misses: unknown[] }>;

/** ✏️ Both humans author: A edits → B sees, B edits → A sees, A undoes their OWN edit (B's stays), B undoes theirs (both back
 * to the start), B redoes (both see it again), both reload and reopen and converge. */
async function editJourney({ A, B, check, plugin, pins, options, spaceId, artifactId, misses }: JourneyContext): Promise<void> {
    const initial = [await docText(A), await docText(B)] as const;
    const head0 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
    const aEdit = await edit(A, plugin, pins);
    const bSawA = await awaitText(B, (now) => now !== initial[1], 30_000);
    check("A edits → B sees", aEdit.applied && typeof bSawA === "string", { verb: aEdit.verb, a: short(aEdit.after), b: short(seenText(bSawA)), hubHead: [head0, await hubHead(options.hub, options.adminCapabilityFile, artifactId)] });
    const afterA = [await docText(A), await docText(B)] as const;
    const bEdit = await edit(B, plugin, pins);
    const aSawB = await awaitText(A, (now) => now !== afterA[0], 30_000);
    check("B edits → A sees", bEdit.applied && typeof aSawB === "string", { b: short(bEdit.after), a: short(seenText(aSawB)) });
    const afterBoth = [await docText(A), await docText(B)] as const;
    const headBeforeAUndo = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
    const undoCursor = [A.lines.length, B.lines.length] as const;
    const aUndo = await undo(A);
    const agreed = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a === b ? a : null;
    }, 30_000);
    const headAfterAUndo = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
    const undoFaults = [...faultsSince(A, undoCursor[0]), ...faultsSince(B, undoCursor[1])];
    const headAdvanced = typeof headBeforeAUndo !== "number" || typeof headAfterAUndo !== "number" || headAfterAUndo > headBeforeAUndo;
    const additive = aUndo.undone;
    check("A undoes own (B's stays)", agreed !== null && agreed !== initial[0] && undoFaults.length === 0 && headAdvanced && (additive || agreed === afterBoth[0]), { kind: additive ? "additive (A's element removed)" : "set (B's later value stands)", a: short(await docText(A)), b: short(await docText(B)), faults: undoFaults.slice(0, 3), hubHead: [headBeforeAUndo, headAfterAUndo] });
    const bUndo = await undo(B);
    const backToInitial = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a === initial[0] && b === initial[1] ? { a, b } : null;
    }, 30_000);
    check("B undoes own (both back to the start)", bUndo.undone && backToInitial !== null, { a: short(await docText(A)), b: short(await docText(B)), initial: initial.map(short) });
    const bRedo = await undo(B, "redo");
    const redone = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a !== initial[0] && b !== initial[1] && a === b ? { a } : null;
    }, 30_000);
    check("B redoes own (both see it again)", bRedo.undone && redone !== null, { b: short(bRedo.after), a: short(await docText(A)) });
    const settled = [await docText(A), await docText(B)] as const;
    for (const session of [A, B]) await session.page.reload({ waitUntil: "domcontentloaded" });
    for (const session of [A, B]) await boot(session);
    await openRow(A, spaceId, artifactId, misses);
    await openRow(B, spaceId, artifactId, misses);
    await awaitMounted(A, options.mountBudgetMs);
    await awaitMounted(B, options.mountBudgetMs);
    const converged = await until(async () => {
      const [a, b] = [await docText(A), await docText(B)];
      return a === settled[0] && b === settled[1] ? { a: short(a) } : null;
    }, 45_000);
    check("reload converges", converged !== null, { settled: settled.map(short), afterReload: [short(await docText(A)), short(await docText(B))], hubHead: await hubHead(options.hub, options.adminCapabilityFile, artifactId) });
}

/** 🏷️ The document surfaces a human's shell mounted (`data-surface-id`, `<dialect>#editor|#viewer`), the navbar role chip
 * (`data-role` + its localized text) and the transient notices it shows. */
const surfaceReading = (page: Page): Promise<{ surfaces: string[]; chips: string[]; notices: string[] }> =>
  page.evaluate(() => ({
    surfaces: [...new Set([...document.querySelectorAll("[data-surface-id]")].map((element) => element.getAttribute("data-surface-id") ?? "").filter((id) => id.includes("#")))],
    chips: [...document.querySelectorAll('[data-slot="surface-role-chip"]')].map((element) => `${element.getAttribute("data-role")}:${(element.textContent ?? "").trim()}`),
    notices: [...document.querySelectorAll("[data-semio-transient-notice]")].map((element) => (element.textContent ?? "").replace(/\s+/gu, " ").trim()).filter(Boolean),
  }));

/** 🎛️ The ids among `ids` a human can press: present, visible and neither `disabled` nor `aria-disabled`. */
const pressableControls = (page: Page, ids: readonly string[]): Promise<string[]> =>
  page.evaluate(
    (wanted) =>
      wanted.filter((id) =>
        [...document.querySelectorAll(`[id="${id}"]`)].some((element) => element instanceof HTMLElement && element.offsetParent !== null && !element.hasAttribute("disabled") && element.getAttribute("aria-disabled") !== "true"),
      ),
    [...ids],
  );

/** 👁️ Every way a viewer can try to change the document: typing and deleting in the first document window body (text
 * editors, focused canvases), the plugin's pinned verb and Undo/Redo when the pane lists them, and the undo shortcut. */
async function attemptViewerEdits(session: Session, verb: string): Promise<string[]> {
  const tried: string[] = [];
  const body = session.page.locator('[data-slot="window-body"]').last();
  const box = await body.boundingBox().catch(() => null);
  if (box !== null) {
    await session.page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
    tried.push("click");
  }
  const editable = session.page.locator('[data-slot="window-body"] textarea, [data-slot="window-body"] [contenteditable="true"], [data-slot="window-body"] input:not([type="hidden"])').first();
  if ((await editable.count()) > 0) {
    await editable.focus().catch(() => undefined);
    tried.push("focus-editable");
  }
  await session.page.keyboard.type("viewer-attempt", { delay: 30 });
  await session.page.keyboard.press("Backspace");
  await session.page.keyboard.press("Delete");
  tried.push("keys");
  for (const id of [`action.${verb}`, "action.undo", "action.redo"]) {
    if ((await session.page.locator(`[data-slot="window-action-pane"] [id="${id}"]`).count()) === 0) continue;
    tried.push(`${id}:${await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="${id}"]`)}`);
    await pause(session, 600);
  }
  await session.page.keyboard.press(process.platform === "darwin" ? "Meta+z" : "Control+z");
  tried.push("undo-shortcut");
  return tried;
}

/** 👁️ B is a Spectator of the space: B's open lands on the kind's viewer surface and the navbar says so in B's language, B is
 * offered no edit control (neither the kind's verbs nor Undo/Redo nor the navbar's switch to the editor), every edit B attempts leaves both views and the hub head unchanged, B still sees A's edit live,
 * and a write crafted with B's own credential straight onto the document socket is refused by the hub (the viewer's plan
 * grants no write) while A's view stays put. */
async function viewerJourney({ A, B, check, plugin, pins, options, spaceId, artifactId }: JourneyContext): Promise<void> {
  const verb = pins.pluginVerbs[plugin] ?? "";
  const chipText = options.locale === "de" ? "Betrachter" : "Viewer";
  const readingB = await surfaceReading(B.page);
  check("B holds the viewer surface", readingB.surfaces.length > 0 && readingB.surfaces.every((id) => id.endsWith("#viewer")) && readingB.chips.includes(`viewer:${chipText}`), { b: readingB, a: (await surfaceReading(A.page)).surfaces });
  const offered = await pressableControls(B.page, [`action.${verb}`, "action.undo", "action.redo", "playground.navbar.roles.editor"]);
  check("viewer offers no edit control", offered.length === 0, { offered, actions: (await readShell(B.page)).actions.slice(0, 16) });
  const before = [await docText(A), await docText(B)] as const;
  const head0 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const cursor = B.lines.length;
  const tried = await attemptViewerEdits(B, verb);
  await pause(B, 3_000);
  const after = [await docText(A), await docText(B)] as const;
  const head1 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const notices = (await surfaceReading(B.page)).notices;
  check("viewer edit attempts change nothing", after[0] === before[0] && after[1] === before[1] && head1 === head0, { tried, a: [short(before[0]), short(after[0])], b: [short(before[1]), short(after[1])], hubHead: [head0, head1], faults: faultsSince(B, cursor).slice(0, 4) });
  check("viewer is told why, in its language", readingB.chips.includes(`viewer:${chipText}`) && (notices.some((text) => (options.locale === "de" ? /schreibgeschützt/u : /read-only/iu).test(text)) || offered.length === 0), { chips: readingB.chips, notices, offered });
  const aEdit = await edit(A, plugin, pins);
  const bSaw = await awaitText(B, (now) => now !== after[1], 30_000);
  check("A edits → viewer sees it live", aEdit.applied && typeof bSaw === "string", { verb: aEdit.verb, a: short(aEdit.after), b: short(seenText(bSaw)) });
  const settled = [await docText(A), await docText(B)] as const;
  const head2 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const crafted = await hubProbeSignIn(options.hub, B.human.email, B.human.password, "twohumanviewer")
    .then((token) => hubProbeOpenDocument(options.hub, token, spaceId, artifactId, `two-human-viewer-${Date.now()}`))
    .then(async (socket) => {
      const answered = await socket.submit(0, "");
      socket.close();
      return { role: String(socket.plan?.surface?.role), write: Boolean(socket.plan?.grant?.write), accepted: answered.accepted, stages: JSON.stringify(answered.ack.stages).slice(0, 300) };
    })
    .catch((error: unknown) => ({ role: "unopened", write: false, accepted: false, stages: String(error instanceof Error ? error.message : error).slice(0, 300) }));
  await pause(A, 3_000);
  const head3 = await hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const aAfterCraft = await docText(A);
  check("hub refuses a write crafted with the viewer's credential", crafted.role === "viewer" && !crafted.write && !crafted.accepted && head3 === head2 && aAfterCraft === settled[0], { crafted, hubHead: [head2, head3], a: short(aAfterCraft) });
}

/** ⏪️ A `Revert` history transition naming `mutationIds`, shaped the way a replica sends one (`semio.history.transition`:
 * tag 0, varint count, varint-length UTF-8 ids) — what another actor would have to send to undo someone else's operations.
 * @see ../../../../../../🔨️modules/📡️replication/🔗️causal/🔀️transition/🦀️.rs */
function revertTransitionEnvelope(documentId: string, mutationIds: readonly string[]): Record<string, unknown> {
  const payload: number[] = [0];
  const varint = (value: number): void => {
    let rest = value;
    while (rest >= 0x80) {
      payload.push((rest & 0x7f) | 0x80);
      rest >>>= 7;
    }
    payload.push(rest);
  };
  varint(mutationIds.length);
  for (const id of mutationIds) {
    const utf8 = new TextEncoder().encode(id);
    varint(utf8.length);
    payload.push(...utf8);
  }
  return { mutation_id: `transition-crafted-${crypto.randomUUID()}`, document_id: documentId, actor: "", dependencies: [...mutationIds], observed: null, target: [], diff: { schema: "semio.history.transition", payload }, inverse: { schema: "semio.history.transition", payload: [] }, timestamp: { actor: 1, physical_ms: Date.now(), logical: 0 } };
}

/** ⏪️ Undo and redo across two authors (row 3.11): each human's undo withdraws only their own newest edit and redo restores
 * only their own — B's undo takes B's edit back and a second one finds nothing of A's to take; A's undo under B's later edit
 * keeps B's edit (the later edits replay on the state without A's, a later value of the same field stands) — both views
 * converge after every step and the hub head advances with every committed transition; and a `Revert` crafted by another
 * actor that names A's operations changes nothing on either view (an undo belongs to its author). */
async function crossUndoJourney({ A, B, check, plugin, pins, options, spaceId, artifactId }: JourneyContext): Promise<void> {
  const crafted = await hubProbeSignIn(options.hub, B.human.email, B.human.password, "twohumanundo").then((token) => hubProbeOpenDocument(options.hub, token, spaceId, artifactId, `two-human-undo-${Date.now()}`));
  const head = (): Promise<number | string | null> => hubHead(options.hub, options.adminCapabilityFile, artifactId);
  const both = async (): Promise<readonly [string, string]> => [await docText(A), await docText(B)] as const;
  const agreeOn = (expected: readonly [string, string]) => until(async () => {
    const [a, b] = await both();
    return a === expected[0] && b === expected[1] ? a : null;
  }, 30_000);
  const advanced = (before: number | string | null, after: number | string | null): boolean => typeof before !== "number" || typeof after !== "number" || after > before;
  try {
    const initial = await both();
    const relayedBeforeA = crafted.relayedEnvelopes().length;
    const aEdit = await edit(A, plugin, pins);
    const bSawA = await awaitText(B, (now) => now !== initial[1], 30_000);
    check("A edits → B sees", aEdit.applied && typeof bSawA === "string", { verb: aEdit.verb, a: short(aEdit.after), b: short(seenText(bSawA)) });
    await pause(A, 2_000);
    const aOperations = crafted.relayedEnvelopes().slice(relayedBeforeA).filter((envelope) => envelope?.diff?.schema !== "semio.history.transition").map((envelope) => String(envelope.mutation_id));
    const afterA = await both();
    const bEdit = await edit(B, plugin, pins);
    const aSawB = await awaitText(A, (now) => now !== afterA[0], 30_000);
    check("B edits → A sees", bEdit.applied && typeof aSawB === "string", { b: short(bEdit.after), a: short(seenText(aSawB)) });
    const afterBoth = await both();
    const cursor = [A.lines.length, B.lines.length] as const;
    const head0 = await head();
    const bUndo = await undo(B);
    const backToA = await agreeOn(afterA);
    const head1 = await head();
    check("B's undo withdraws only B's edit", bUndo.undone && backToA !== null && advanced(head0, head1), { a: short(await docText(A)), b: short(await docText(B)), expected: afterA.map(short), hubHead: [head0, head1] });
    const bUndoAgain = await undo(B);
    const stillA = await both();
    const head2 = await head();
    check("B's next undo leaves A's edit alone", !bUndoAgain.undone && stillA[0] === afterA[0] && stillA[1] === afterA[1] && head2 === head1, { a: short(stillA[0]), b: short(stillA[1]), hubHead: [head1, head2], faults: [...faultsSince(A, cursor[0]), ...faultsSince(B, cursor[1])].slice(0, 3) });
    const bRedo = await undo(B, "redo");
    const redone = await agreeOn(afterBoth);
    check("B's redo restores only B's edit", bRedo.undone && redone !== null, { a: short(await docText(A)), b: short(await docText(B)), expected: afterBoth.map(short) });
    const head3 = await head();
    await undo(A);
    const agreed = await until(async () => {
      const [a, b] = await both();
      return a === b && a !== afterBoth[0] ? a : a === b && a === afterBoth[0] && advanced(head3, await head()) ? a : null;
    }, 30_000);
    const head4 = await head();
    check("A's undo under B's later edit keeps B's edit", agreed !== null && agreed !== initial[0] && agreed !== afterA[0] && advanced(head3, head4), { kind: agreed === afterBoth[0] ? "set (B's later value stands)" : "additive (A's element withdrawn, B's replayed)", a: short(await docText(A)), b: short(await docText(B)), hubHead: [head3, head4] });
    await undo(A, "redo");
    const aRedone = await agreeOn(afterBoth);
    check("A's redo restores A's edit under B's", aRedone !== null, { a: short(await docText(A)), b: short(await docText(B)), expected: afterBoth.map(short) });
    const head5 = await head();
    const answered = aOperations.length === 0 ? null : await crafted.submitEnvelopes(1, [revertTransitionEnvelope(artifactId, aOperations)]);
    await pause(A, 6_000);
    const afterCraft = await both();
    check("another actor's crafted undo of A's edit changes nothing", aOperations.length > 0 && afterCraft[0] === afterBoth[0] && afterCraft[1] === afterBoth[1], { aOperations, hubAccepted: answered?.accepted ?? null, stages: answered === null ? null : JSON.stringify(answered.ack.stages).slice(0, 240), a: short(afterCraft[0]), b: short(afterCraft[1]), expected: afterBoth.map(short), hubHead: [head5, await head()] });
  } finally {
    crafted.close();
  }
}

/** 🌱️ Creates a public studio as `owner` and makes `memberEmail` a member with `role`, through the same sealed directory
 * commands the Home dialogs post (a journey whose subject is not the space dialogs does not depend on them). */
async function seedSharedSpaceThroughHub(hub: string, owner: Human, memberEmail: string, role: DirectorySpaceRole, name: string): Promise<string> {
  const token = await hubProbeSignIn(hub, owner.email, owner.password, "twohumanseed");
  const command = (body: Parameters<typeof sealDirectoryCommandRequestV1>[1]) => hubProbeCall(hub, "POST", "/directory/commands", token, directoryCommandRequestJson(sealDirectoryCommandRequestV1(crypto.randomUUID().replaceAll("-", ""), body)));
  const created = await command(createSpaceCommandV1(name, "studio", "public"));
  const spaceId = created.json?.events?.find((event: { body?: { kind?: string } }) => event?.body?.kind === "space.created")?.body?.spaceId;
  if (created.status !== 202 || typeof spaceId !== "string") throw new Error(`seed create-space ${created.status} ${created.text.slice(0, 200)}`);
  const shared = await command({ kind: "upsert-member", spaceId, email: memberEmail, role });
  if (shared.status !== 202) throw new Error(`seed upsert-member ${shared.status} ${shared.text.slice(0, 200)}`);
  return spaceId;
}

/** 🗺️ The journeys a run can drive: how the shared space is set up (the Home dialogs, or the hub's directory commands), the
 * role B is invited with, and how many checks a kind passes with. */
const JOURNEYS: Readonly<Record<TwoHumanJourney, Readonly<{ run: (context: JourneyContext) => Promise<void>; seed: "ui" | "hub"; shareRole: RegExp; memberRole: DirectorySpaceRole; checks: number; check: string }>>> = {
  edit: { run: editJourney, seed: "ui", shareRole: /author|autor/iu, memberRole: "author", checks: 10, check: "two-human" },
  viewer: { run: viewerJourney, seed: "hub", shareRole: /spectator|betrachter/iu, memberRole: "spectator", checks: 10, check: "two-human-viewer" },
  "cross-undo": { run: crossUndoJourney, seed: "hub", shareRole: /author|autor/iu, memberRole: "author", checks: 12, check: "two-human-cross-undo" },
};
//#endregion 🔖️Journeys

//#region 🔖️TwoHuman
/** 🧭️ The per-kind journey of a run: both humans author (`edit`), B is a Spectator (`viewer`), or both author and undo/redo
 * across each other's edits (`cross-undo`). */
export type TwoHumanJourney = "edit" | "viewer" | "cross-undo";

/** 🎛️ One two-human run. */
export type TwoHumanOptions = Readonly<{
  journey: TwoHumanJourney;
  hub: string;
  serves: readonly [string, string];
  humans: readonly [Human, Human];
  locale: "en" | "de";
  kinds: readonly string[];
  spaceId: string | null;
  tag: string;
  outDir: string;
  adminCapabilityFile: string | null;
  mountBudgetMs: number;
  signal: AbortSignal;
}>;

type KindRow = { kindId: string; label: string; plugin?: string; artifactId?: string; timings?: { createToMountedMs?: number; openRowToMountedMs?: number; createStartedAtMs?: number; openStartedAtMs?: number }; checks: Record<string, { pass: boolean; detail: unknown }>; faults: { A: string[]; B: string[] }; pass: boolean };

/** 📊️ The run's report, rewritten after every kind. */
export type TwoHumanReport = { tag: string; journey: TwoHumanJourney; hub: string; serves: readonly string[]; locale: string; startedAt: string; finishedAt?: string; spaceId: string | null; kinds: string[]; rows: KindRow[]; misses: unknown[]; fatal?: string; cancelled?: boolean };

/** 👥️ Runs the two-human journey over every selected creatable kind; the report is flushed after every kind and the
 * signal ends the run after the current kind. */
export async function runTwoHuman(options: TwoHumanOptions): Promise<TwoHumanReport> {
  const pins = readMatrixPins();
  const outDir = join(options.outDir, options.tag);
  mkdirSync(outDir, { recursive: true });
  const browserLocale = options.locale === "de" ? "de-DE" : "en-US";
  const report: TwoHumanReport = { tag: options.tag, journey: options.journey, hub: options.hub, serves: options.serves, locale: browserLocale, startedAt: new Date().toISOString(), spaceId: options.spaceId, kinds: [], rows: [], misses: [] };
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal", "--ignore-gpu-blocklist"] });
  const sessions = await openSessions(browser, options.serves, options.humans, browserLocale);
  const [A, B] = sessions as [Session, Session];
  const flush = (): void => {
    writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
    writeFileSync(join(outDir, "console.txt"), sessions.flatMap((session) => session.lines.map((line) => `${session.human.label} ${line}`)).join("\n"));
  };
  const shot = (session: Session, name: string): Promise<void> => session.page.screenshot({ path: join(outDir, `${name}-${session.human.label}.png`) }).then(() => undefined, () => undefined);
  const log = (line: string): void => console.log(`[two-human] ${line}`.slice(0, 1600));
  try {
    for (const session of [A, B]) await boot(session);
    for (const session of [A, B]) await signIn(session);
    for (const session of [A, B]) await settleHome(session);
    let spaceId = options.spaceId;
    if (spaceId === null && JOURNEYS[options.journey].seed === "hub") {
      spaceId = await seedSharedSpaceThroughHub(options.hub, A.human, B.human.email, JOURNEYS[options.journey].memberRole, `Two Human ${options.tag} ${Date.now() % 100000}`);
    } else if (spaceId === null) {
      const spaceName = `Two Human ${options.tag} ${Date.now() % 100000}`;
      await activate(A.page, "s-home-create-space");
      await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
      await A.page.locator("#name").fill(spaceName);
      await selectOption(A.page, "kind", /studio/iu);
      await selectOption(A.page, "visibility", /public|öffentlich/iu);
      await submitDialog(A.page);
      spaceId = await waitNamedRow(A.page, "space", spaceName, 90_000);
      await clickRowAction(A.page, "space", spaceId, /^(share|teilen)\b/iu);
      await dialog(A.page).waitFor({ state: "visible", timeout: 20_000 });
      await A.page.locator("#email").fill(B.human.email);
      await selectOption(A.page, "role", JOURNEYS[options.journey].shareRole);
      await submitDialog(A.page);
      await awaitSharedSpace(B, spaceId, report.misses);
    }
    report.spaceId = spaceId;
    log(`space ${spaceId}`);
    await openSpace(A, spaceId, report.misses);
    await openSpace(B, spaceId, report.misses);
    const kinds = (await creatableKinds(A)).filter((kind) => options.kinds.length === 0 || options.kinds.includes(kind.kindId));
    report.kinds = kinds.map((kind) => kind.kindId);
    log(`kinds ${report.kinds.join(", ")}`);
    flush();
    for (const [index, kind] of kinds.entries()) {
      if (options.signal.aborted) {
        report.cancelled = true;
        log("cancelled; the report keeps every finished kind");
        break;
      }
      const cursors = [A.lines.length, B.lines.length] as const;
      const row: KindRow = { kindId: kind.kindId, label: kind.label, checks: {}, faults: { A: [], B: [] }, pass: false };
      const check = (name: string, pass: boolean, detail: unknown): void => {
        row.checks[name] = { pass, detail };
        log(`  ${kind.kindId} ${name}: ${pass ? "PASS" : "FAIL"} ${typeof detail === "string" ? detail : JSON.stringify(detail)}`);
      };
      try {
        await openSpace(A, spaceId, report.misses);
        const createdAt = Date.now();
        const artifactId = await createArtifact(A, `Two Human ${kind.kindId} ${Date.now() % 100000}`, kind, options.mountBudgetMs);
        row.artifactId = artifactId;
        const shellA = await awaitMounted(A, options.mountBudgetMs);
        row.timings = { createToMountedMs: shellA.mountedAt - createdAt, createStartedAtMs: createdAt - started };
        const plugin = /^s\.([a-z0-9-]+)\./u.exec(String((JSON.parse(kind.value) as { dialect?: { artifactKind?: string } }).dialect?.artifactKind ?? ""))?.[1] ?? PLUGIN_BY_KIND[kind.kindId] ?? "";
        row.plugin = plugin;
        check("A creates + opens", true, { artifactId, windows: shellA.windowIds });
        await openSpace(B, spaceId, report.misses);
        await waitRow(B.page, "artifact", artifactId, 90_000);
        const openedAt = Date.now();
        await clickRowAction(B.page, "artifact", artifactId, /^(open|öffnen)\b/iu);
        const shellB = await awaitMounted(B, options.mountBudgetMs);
        row.timings = { ...row.timings, openRowToMountedMs: shellB.mountedAt - openedAt, openStartedAtMs: openedAt - started };
        check("B opens via Space index", true, { windows: shellB.windowIds });
        const presence = await until(async () => {
          const [a, b] = [await readStatus(A.page), await readStatus(B.page)];
          return a.peers.length === 2 && b.peers.length === 2 ? { a: a.peerLabels, b: b.peerLabels } : null;
        }, 45_000);
        check("presence 2/2", presence !== null, presence ?? { a: (await readStatus(A.page)).peers, b: (await readStatus(B.page)).peers });
        const windowCursor = [A.lines.length, B.lines.length] as const;
        for (const session of [A, B])
          for (const windowId of shellA.windowIds) {
            await session.page.locator(`[data-window-id="${windowId}"]`).first().click({ position: { x: 40, y: 8 }, force: true }).catch(() => undefined);
            await pause(session, 800);
          }
        const windowFaults = [...faultsSince(A, windowCursor[0]), ...faultsSince(B, windowCursor[1])];
        check("every window takes focus and commands", windowFaults.length === 0 && shellA.windowIds.length > 0, { windows: shellA.windowIds, faults: windowFaults.slice(0, 4) });
        await JOURNEYS[options.journey].run({ A, B, check, plugin, pins, options, spaceId, artifactId, misses: report.misses });
      } catch (error) {
        check("journey", false, String(error instanceof Error ? error.message : error).slice(0, 600));
        await shot(A, `${kind.kindId}-fail`);
        await shot(B, `${kind.kindId}-fail`);
      }
      row.faults = { A: faultsSince(A, cursors[0]), B: faultsSince(B, cursors[1]) };
      row.pass = Object.values(row.checks).every((entry) => entry.pass) && Object.keys(row.checks).length >= JOURNEYS[options.journey].checks;
      report.rows.push(row);
      flush();
      log(`${index + 1}/${kinds.length} ${row.pass ? "PASS" : "FAIL"} ${kind.kindId} (${Object.values(row.checks).filter((entry) => entry.pass).length}/${Object.keys(row.checks).length} checks, faults A ${row.faults.A.length} B ${row.faults.B.length})`);
    }
  } catch (error) {
    report.fatal = String(error instanceof Error ? (error.stack ?? error.message) : error).slice(0, 1200);
    log(`FATAL ${report.fatal}`);
    await shot(A, "run-fail");
    await shot(B, "run-fail");
  } finally {
    report.finishedAt = new Date().toISOString();
    flush();
    await browser.close();
  }
  return report;
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🔑️ The local development hub's two credential users, the ones every hub gate signs in as by default. */
const DEVELOPMENT_HUMANS: readonly [Human, Human] = [
  { label: "user1", email: "user1@semio.dev", password: "gm1-local-dev-pass-1" },
  { label: "user2", email: "user2@semio.dev", password: "gm1-local-dev-pass-2" },
];

/** 🔑️ The two humans: `SEMIO_TWO_HUMAN_USER{1,2}_{EMAIL,PASSWORD}` (what the goal gate hands over), else a non-empty
 * `--users` JSON file `{"users":[{"email","password"},…]}`, else the development hub's two users. Never an argument. */
function readHumans(segments: readonly string[]): readonly [Human, Human] {
  const fromEnv = [1, 2].map((index) => ({ label: `user${index}`, email: process.env[`SEMIO_TWO_HUMAN_USER${index}_EMAIL`] ?? "", password: process.env[`SEMIO_TWO_HUMAN_USER${index}_PASSWORD`] ?? "" }));
  if (fromEnv.every((human) => human.email && human.password)) return fromEnv as unknown as readonly [Human, Human];
  const file = flagValue(segments, "--users");
  if (!file) return DEVELOPMENT_HUMANS;
  const users = (JSON.parse(readFileSync(resolve(file), "utf8")) as { users?: { email: string; password: string }[] }).users ?? [];
  if (users.length < 2) throw new Error(`--users ${file} must list two users`);
  return [{ label: "user1", ...users[0]! }, { label: "user2", ...users[1]! }];
}

/** 🚪️ `verify two-human --hub <url> --serve <url> [--serve-b <url>] [--journey edit|viewer|cross-undo] [--locale en|de] [--kinds <kindId,…>]
 * [--space <id>] [--tag <t>] [--out <dir>] [--admin-capability <file>] [--users <json>] [--max-create-to-mounted-ms <n>]
 * [--max-open-to-mounted-ms <n>] [--mount-budget-ms <n>]` — runs the journey (a creation or an open waits up to
 * `--mount-budget-ms`, default 15 min, for the document to mount; the latency bounds judge it), writes `report.json`,
 * `console.txt` and failure screenshots under `<out>/<tag>/`, publishes the acceptance record, exits non-zero unless every
 * kind passes. */
export async function runTwoHumanCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const hub = flagValue(segments, "--hub");
  const serve = flagValue(segments, "--serve");
  if (!hub || !serve) throw new Error("usage: verify two-human --hub <url> --serve <url> [--serve-b <url>] [--journey edit|viewer|cross-undo] [--locale en|de] [--kinds …] [--space <id>] [--tag <t>] [--out <dir>] [--admin-capability <file>] [--users <json>]");
  const journey = flagValue(segments, "--journey") ?? "edit";
  if (!(journey in JOURNEYS)) throw new Error(`verify two-human: unknown --journey ${journey} (${Object.keys(JOURNEYS).join("|")})`);
  const { check } = JOURNEYS[journey as TwoHumanJourney];
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const tag = flagValue(segments, "--tag") ?? `${check}-${locale}`;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  const serveB = flagValue(segments, "--serve-b") ?? serve;
  const hubUrl = hub.replace(/\/$/u, "");
  const served = (url: string, run: (baseUrl: string) => Promise<void>): Promise<void> => withDevServe(repoRoot, check, { serveUrl: url, hubUrl, locale, signal: controller.signal, startedAt }, run);
  const body = async (serveUrl: string, serveBUrl: string): Promise<void> => {
    const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
    const report = await runTwoHuman({
      journey: journey as TwoHumanJourney,
      hub: hubUrl,
      serves: [serveUrl, serveBUrl],
      humans: readHumans(segments),
      locale,
      kinds: (flagValue(segments, "--kinds") ?? "").split(",").filter(Boolean),
      spaceId: flagValue(segments, "--space") ?? null,
      tag,
      outDir,
      adminCapabilityFile: flagValue(segments, "--admin-capability") ?? null,
      mountBudgetMs: Number(flagValue(segments, "--mount-budget-ms") ?? 900_000),
      signal: controller.signal,
    });
    const passed = report.rows.filter((row) => row.pass).length;
    const total = report.rows.length;
    const failing = report.rows.filter((row) => !row.pass).map((row) => `${row.kindId}: ${Object.entries(row.checks).filter(([, entry]) => !entry.pass).map(([name]) => name).join(", ") || "faults"}`);
    const unreachable = total === 0 && /ERR_CONNECTION_REFUSED|ECONNREFUSED|Unable to connect/u.test(report.fatal ?? "");
    const bound = (flag: string): number | null => (flagValue(segments, flag) === undefined ? null : Number(flagValue(segments, flag)));
    const createBoundMs = bound("--max-create-to-mounted-ms");
    const openBoundMs = bound("--max-open-to-mounted-ms");
    const createTimes = report.rows.map((row) => row.timings?.createToMountedMs).filter((value): value is number => value !== undefined).sort((left, right) => left - right);
    const openTimes = report.rows.map((row) => row.timings?.openRowToMountedMs).filter((value): value is number => value !== undefined).sort((left, right) => left - right);
    const p50 = (values: readonly number[]): number => (values.length ? values[Math.floor((values.length - 1) / 2)]! : -1);
    const slow = report.rows.filter((row) => (createBoundMs !== null && (row.timings?.createToMountedMs ?? 0) > createBoundMs) || (openBoundMs !== null && (row.timings?.openRowToMountedMs ?? 0) > openBoundMs)).map((row) => row.kindId);
    const status = unreachable ? "blocked" : report.fatal || report.cancelled || total === 0 ? "fail" : passed === total && slow.length === 0 ? "pass" : "fail";
    const latency = { en: `create→mounted p50 ${p50(createTimes)} ms max ${createTimes.at(-1) ?? -1} ms${createBoundMs === null ? "" : ` (bound ${createBoundMs})`}, open→mounted p50 ${p50(openTimes)} ms max ${openTimes.at(-1) ?? -1} ms${openBoundMs === null ? "" : ` (bound ${openBoundMs})`}${slow.length ? `; over bound: ${slow.join(", ")}` : ""}`, de: `Anlegen→eingebunden p50 ${p50(createTimes)} ms max ${createTimes.at(-1) ?? -1} ms${createBoundMs === null ? "" : ` (Grenze ${createBoundMs})`}, Öffnen→eingebunden p50 ${p50(openTimes)} ms max ${openTimes.at(-1) ?? -1} ms${openBoundMs === null ? "" : ` (Grenze ${openBoundMs})`}${slow.length ? `; über der Grenze: ${slow.join(", ")}` : ""}` };
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check,
        status,
        startedAt,
        measured: { locale, kinds: total, passed, failed: total - passed, routeMisses: report.misses.length, fatal: Boolean(report.fatal), cancelled: Boolean(report.cancelled), createToMountedP50Ms: p50(createTimes), createToMountedMaxMs: createTimes.at(-1) ?? -1, openRowToMountedP50Ms: p50(openTimes), openRowToMountedMaxMs: openTimes.at(-1) ?? -1, createBoundMs: createBoundMs ?? -1, openBoundMs: openBoundMs ?? -1, overBound: slow.length },
        summary: {
          en: `${passed}/${total} kinds pass the two-human ${journey} journey in ${locale}; ${latency.en}${failing.length ? `; failing: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; fatal: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
          de: `${passed}/${total} Arten bestehen den Zwei-Personen-Weg (${journey}) in ${locale}; ${latency.de}${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; Abbruch: ${report.fatal.split("\n")[0]!.slice(0, 160)}` : ""}`,
        },
        evidence: [join(outDir, tag, "report.json")],
      }),
    );
    console.log(`[two-human] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===`);
    if (status !== "pass") process.exitCode = 1;
  };
  await withAcceptanceRecord(repoRoot, check, () => served(serve, (serveUrl) => (serveB === serve ? body(serveUrl, serveUrl) : served(serveB, (serveBUrl) => body(serveUrl, serveBUrl)))));
  process.removeListener("SIGINT", cancel);
  process.removeListener("SIGTERM", cancel);
}
//#endregion 🔖️TwoHuman
