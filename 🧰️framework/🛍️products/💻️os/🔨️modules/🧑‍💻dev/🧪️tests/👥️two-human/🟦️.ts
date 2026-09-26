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
import { join, resolve } from "node:path";
import type { Browser, BrowserContext, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { FAULT, NOISE, clickUncovered, fillStagedArgument, readMatrixPins, readShell, submitStagedVerb, unfoldActionsRail } from "../🧮️program-matrix/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

//#region 🔖️Sessions
/** 🔑️ One human: a hub credential. */
type Human = Readonly<{ label: string; email: string; password: string }>;

/** 🌐️ One human's isolated browser profile on one serve, with its console and hub traffic lines. */
type Session = { human: Human; url: string; context: BrowserContext; page: Page; lines: string[] };

const started = Date.now();
const ms = (): number => Date.now() - started;
const pause = (session: Session, duration: number): Promise<void> => session.page.waitForTimeout(duration);
const originOf = (session: Session): string => new URL(session.url).origin;
const faultsSince = (session: Session, cursor: number): string[] => session.lines.slice(cursor).filter((line) => FAULT.test(line) && !NOISE.test(line)).map((line) => line.slice(0, 240));
const short = (text: string): string => (text.length <= 160 ? text : `${text.slice(0, 80)}…${text.slice(-70)}`);

async function openSessions(browser: Browser, urls: readonly string[], humans: readonly Human[], locale: string): Promise<Session[]> {
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

async function boot(session: Session): Promise<void> {
  await session.page.goto(session.url, { waitUntil: "domcontentloaded", timeout: 180_000 });
  await session.page.locator('[data-ui-node-key="s-home-create-space"]').first().waitFor({ state: "attached", timeout: 180_000 });
}

async function signIn(session: Session): Promise<void> {
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
async function activate(page: Page, key: string): Promise<void> {
  const node = page.locator(`[data-ui-node-key="${key}"]`).first();
  await node.waitFor({ state: "attached", timeout: 30_000 });
  await node.focus();
  await node.press("Enter");
}

const dialog = (page: Page) => page.locator('[role="dialog"][data-slot="dialog-content"]');

async function selectOption(page: Page, triggerId: string, option: RegExp): Promise<void> {
  await page.locator(`[id="${triggerId}"]`).click();
  await page.getByRole("option", { name: option }).first().click();
  await page.waitForTimeout(200);
}

async function submitDialog(page: Page): Promise<void> {
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
async function pageWindowedTables<T>(page: Page, found: () => Promise<T | null>): Promise<T | null> {
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

async function waitNamedRow(page: Page, prefix: string, name: string, deadlineMs: number): Promise<string> {
  const find = (): Promise<string | null> => page.locator(`[data-ui-node-key^="${prefix}:"]`).evaluateAll((elements, wanted) => elements.find((element) => (element.textContent ?? "").includes(wanted))?.getAttribute("data-ui-node-key") ?? null, name);
  const deadline = Date.now() + deadlineMs;
  while (Date.now() < deadline) {
    const key = await pageWindowedTables(page, find);
    if (key !== null) return key.slice(prefix.length + 1);
    await page.waitForTimeout(500);
  }
  throw new Error(`no ${prefix} row named ${JSON.stringify(name)} within ${deadlineMs} ms (windowed tables paged)`);
}

async function clickRowAction(page: Page, prefix: string, id: string, pattern: RegExp): Promise<string> {
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

async function awaitMounted(session: Session, deadlineMs: number) {
  const mounted = await until(async () => {
    const shell = await readShell(session.page);
    const status = await readStatus(session.page);
    const documentWindows = shell.windowIds.filter((id) => !SHELL_WINDOWS.has(id));
    const painted = await session.page.evaluate(() => [...document.querySelectorAll('[data-slot="window-body"]')].some((body) => body.querySelectorAll("*").length > 20));
    return documentWindows.length > 0 && painted && status.executionTarget.length === 0 && !new URL(session.page.url()).pathname.endsWith("/") ? { ...shell, windowIds: documentWindows } : null;
  }, deadlineMs, 1_000);
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

async function createArtifact(session: Session, name: string, kind: CreatableKind): Promise<string> {
  const before = await rowIds(session.page, "artifact");
  await activate(session.page, "s-space-create-artifact");
  await dialog(session.page).waitFor({ state: "visible", timeout: 20_000 });
  await session.page.locator("#name").fill(name);
  await session.page.locator('[id="kindChoice"]').focus();
  await session.page.keyboard.press("Enter");
  await session.page.locator(`[role="option"][data-value="${kind.value.replace(/"/gu, '\\"')}"]`).first().click();
  await submitDialog(session.page);
  return waitNewRow(session.page, "artifact", before, 240_000);
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
async function edit(session: Session, plugin: string, pins: ReturnType<typeof readMatrixPins>) {
  const verb = pins.pluginVerbs[plugin];
  if (verb === undefined) throw new Error(`no pinned document verb for plugin ${plugin}`);
  const before = await docText(session);
  const staged = await session.page.locator(`[id$=".action.${verb}.execute"]`).first().isVisible().catch(() => false);
  if (!staged) await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="action.${verb}"]`);
  await pause(session, 1_000);
  for (const [key, value] of Object.entries(pins.pluginArgs[`${plugin}.${verb}`] ?? pins.pluginArgs[verb] ?? {})) await fillStagedArgument(session.page, key, value, pins.liveId);
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

/** 🗄️ The hub's own head sequence of a document through the admin API, when an admin capability file was given. */
async function hubHead(hub: string, adminCapabilityFile: string | null, artifactId: string): Promise<number | string | null> {
  if (!adminCapabilityFile) return null;
  const { capability } = JSON.parse(readFileSync(adminCapabilityFile, "utf8")) as { capability: string };
  const response = await fetch(`${hub}/admin/api/documents`, { headers: { authorization: `Bearer ${capability}` } }).catch(() => null);
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

//#region 🔖️TwoHuman
/** 🎛️ One two-human run. */
export type TwoHumanOptions = Readonly<{
  hub: string;
  serves: readonly [string, string];
  humans: readonly [Human, Human];
  locale: "en" | "de";
  kinds: readonly string[];
  spaceId: string | null;
  tag: string;
  outDir: string;
  adminCapabilityFile: string | null;
  signal: AbortSignal;
}>;

type KindRow = { kindId: string; label: string; plugin?: string; artifactId?: string; checks: Record<string, { pass: boolean; detail: unknown }>; faults: { A: string[]; B: string[] }; pass: boolean };

/** 📊️ The run's report, rewritten after every kind. */
export type TwoHumanReport = { tag: string; hub: string; serves: readonly string[]; locale: string; startedAt: string; finishedAt?: string; spaceId: string | null; kinds: string[]; rows: KindRow[]; misses: unknown[]; fatal?: string; cancelled?: boolean };

/** 👥️ Runs the two-human journey over every selected creatable kind; the report is flushed after every kind and the
 * signal ends the run after the current kind. */
export async function runTwoHuman(options: TwoHumanOptions): Promise<TwoHumanReport> {
  const pins = readMatrixPins();
  const outDir = join(options.outDir, options.tag);
  mkdirSync(outDir, { recursive: true });
  const browserLocale = options.locale === "de" ? "de-DE" : "en-US";
  const report: TwoHumanReport = { tag: options.tag, hub: options.hub, serves: options.serves, locale: browserLocale, startedAt: new Date().toISOString(), spaceId: options.spaceId, kinds: [], rows: [], misses: [] };
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
    await pause(A, 5_000);
    let spaceId = options.spaceId;
    if (spaceId === null) {
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
      await selectOption(A.page, "role", /author|autor/iu);
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
        const artifactId = await createArtifact(A, `Two Human ${kind.kindId} ${Date.now() % 100000}`, kind);
        row.artifactId = artifactId;
        const shellA = await awaitMounted(A, 300_000);
        const plugin = /^s\.([a-z0-9-]+)\./u.exec(String((JSON.parse(kind.value) as { dialect?: { artifactKind?: string } }).dialect?.artifactKind ?? ""))?.[1] ?? PLUGIN_BY_KIND[kind.kindId] ?? "";
        row.plugin = plugin;
        check("A creates + opens", true, { artifactId, windows: shellA.windowIds });
        await openRow(B, spaceId, artifactId, report.misses);
        const shellB = await awaitMounted(B, 300_000);
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
        const aUndo = await undo(A);
        const bAfterAUndo = await awaitText(B, (now) => now !== afterBoth[1], 30_000);
        check("A undoes own (B's stays)", aUndo.undone && typeof bAfterAUndo === "string" && aUndo.after !== initial[0], { a: short(aUndo.after), b: short(seenText(bAfterAUndo)) });
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
        await openRow(A, spaceId, artifactId, report.misses);
        await openRow(B, spaceId, artifactId, report.misses);
        await awaitMounted(A, 300_000);
        await awaitMounted(B, 300_000);
        const converged = await until(async () => {
          const [a, b] = [await docText(A), await docText(B)];
          return a === settled[0] && b === settled[1] ? { a: short(a) } : null;
        }, 45_000);
        check("reload converges", converged !== null, { settled: settled.map(short), afterReload: [short(await docText(A)), short(await docText(B))], hubHead: await hubHead(options.hub, options.adminCapabilityFile, artifactId) });
      } catch (error) {
        check("journey", false, String(error instanceof Error ? error.message : error).slice(0, 600));
        await shot(A, `${kind.kindId}-fail`);
        await shot(B, `${kind.kindId}-fail`);
      }
      row.faults = { A: faultsSince(A, cursors[0]), B: faultsSince(B, cursors[1]) };
      row.pass = Object.values(row.checks).every((entry) => entry.pass) && Object.keys(row.checks).length >= 10;
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

/** 🚪️ `verify two-human --hub <url> --serve <url> [--serve-b <url>] [--locale en|de] [--kinds <kindId,…>] [--space <id>]
 * [--tag <t>] [--out <dir>] [--admin-capability <file>] [--users <json>]` — runs the journey, writes `report.json`,
 * `console.txt` and failure screenshots under `<out>/<tag>/`, publishes the acceptance record, exits non-zero unless every
 * kind passes. */
export async function runTwoHumanCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const hub = flagValue(segments, "--hub");
  const serve = flagValue(segments, "--serve");
  if (!hub || !serve) throw new Error("usage: verify two-human --hub <url> --serve <url> [--serve-b <url>] [--locale en|de] [--kinds …] [--space <id>] [--tag <t>] [--out <dir>] [--admin-capability <file>] [--users <json>]");
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const tag = flagValue(segments, "--tag") ?? `two-human-${locale}`;
  const controller = new AbortController();
  const cancel = (): void => controller.abort();
  process.once("SIGINT", cancel);
  process.once("SIGTERM", cancel);
  const startedAt = new Date();
  try {
    const outDir = resolve(flagValue(segments, "--out") ?? defaultOutDir);
    const report = await runTwoHuman({
      hub: hub.replace(/\/$/u, ""),
      serves: [serve, flagValue(segments, "--serve-b") ?? serve],
      humans: readHumans(segments),
      locale,
      kinds: (flagValue(segments, "--kinds") ?? "").split(",").filter(Boolean),
      spaceId: flagValue(segments, "--space") ?? null,
      tag,
      outDir,
      adminCapabilityFile: flagValue(segments, "--admin-capability") ?? null,
      signal: controller.signal,
    });
    const passed = report.rows.filter((row) => row.pass).length;
    const total = report.rows.length;
    const failing = report.rows.filter((row) => !row.pass).map((row) => `${row.kindId}: ${Object.entries(row.checks).filter(([, entry]) => !entry.pass).map(([name]) => name).join(", ") || "faults"}`);
    const status = report.fatal || report.cancelled || total === 0 ? "fail" : passed === total ? "pass" : "fail";
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check: "two-human",
        status,
        startedAt,
        measured: { locale, kinds: total, passed, failed: total - passed, routeMisses: report.misses.length, fatal: Boolean(report.fatal), cancelled: Boolean(report.cancelled) },
        summary: {
          en: `${passed}/${total} kinds pass the two-human journey in ${locale}${failing.length ? `; failing: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; fatal: ${report.fatal.slice(0, 160)}` : ""}`,
          de: `${passed}/${total} Arten bestehen den Zwei-Personen-Weg in ${locale}${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; Abbruch: ${report.fatal.slice(0, 160)}` : ""}`,
        },
        evidence: [join(outDir, tag, "report.json")],
      }),
    );
    console.log(`[two-human] === ${tag}: PASS ${passed}/${total} → ${join(outDir, tag)} ===`);
    if (status !== "pass") process.exitCode = 1;
  } finally {
    process.removeListener("SIGINT", cancel);
    process.removeListener("SIGTERM", cancel);
  }
}
//#endregion 🔖️TwoHuman
