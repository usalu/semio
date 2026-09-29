/** 🗂️ The hub-document sweep inside `s`: every kind the hub can create, as a signed-in person creates and edits it.
 *
 * One persistent browser profile per run (a later kind of the same plugin installs from this device's store, exactly as a
 * person's second document does). Per creatable kind of the space's creation catalog: sign in → OPEN the sweep space from
 * the hub workspace's Space Browser (the only lane that mounts the space index as the shell's own base session) → the
 * index's `createArtifact` with a name and the kind picked by its kind id → a new index row → the creation saga opens the
 * document by itself (its program installed from the hub catalog; bounded wait, longer for puzzle kinds' CPU-bound
 * genesis) → the kind's pinned rail verb → undo → redo, judged by the History ledger, `Check in (n)` and the render digest
 * (clean round trip `[e, e+1, e, e+1]`) → 0 fault lines. Optional: `--reopen` reloads and reopens the document from the
 * index (the module then comes from the device's store), `--cancel install|open` cancels the program install band or the
 * execution-target open once it runs (nothing may be committed). Every transient notice, creation status,
 * execution-target stage and install band the shell shows is recorded per kind.
 *
 * Promoted from the session-12/13 ticket harness `wp-s16/s16-hub-journey.mjs` + `s16-b2-sweep.sh` (= S15's, adapted from
 * S12's probe; S16 06:3x version: windowed `artifact:<id>` rows, saga wait 300 s / 900 s for puzzle kinds). The diagnostic
 * toggles of the ticket script (net log, plugin blocking, store eviction, hold/open traces) stay in the ticket.
 * @see ../🧮️program-matrix/🟦️.ts — verb pins and the round-trip witness
 */

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import type { BrowserContext, Page } from "playwright";
import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🔌️plugin/🏗️build/📋️plan/🟦️.ts";
import { ensureParityPlaywrightBrowsersPath } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { FAULT, NOISE, awaitBeacon, click, dismissIntroduction, mutateUndoRedo, readMatrixPins, readShell, unfoldActionsRail, withDevServe, type MatrixRenderedEdit } from "../🧮️program-matrix/🟦️.ts";
import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "../../../../../🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts";

//#region 🔖️Journey
type Notice = { code: string; text: string };

/** 🧯️ Records every transient notice, creation status, execution-target stage, install band and dialog as it appears
 * (a transient notice auto-dismisses after 4 s, so polling would lose the very refusal the shell showed). */
const installNoticeRecorder = (context: BrowserContext): Promise<unknown> =>
  context.addInitScript(() => {
    const seen: { code: string; text: string }[] = [];
    (globalThis as unknown as { __hubSweepNotices: typeof seen }).__hubSweepNotices = seen;
    const text = (element: Element): string => (element.textContent ?? "").replace(/\s+/gu, " ").trim();
    const push = (code: string, value: string): void => {
      if (!seen.some((other) => other.code === code && other.text === value)) seen.push({ code, text: value.slice(0, 160) });
    };
    const scan = (): void => {
      for (const element of document.querySelectorAll("[data-semio-transient-notice]")) push(element.getAttribute("data-notice-code") ?? "", text(element));
      for (const element of document.querySelectorAll("[data-semio-artifact-creation]")) push(`creation:${element.getAttribute("data-semio-artifact-creation") ?? ""}`, text(element));
      for (const element of document.querySelectorAll("[data-semio-execution-target-status]")) {
        const progress = element.querySelector("progress");
        push(`execution-target:${element.getAttribute("data-semio-execution-target-stage") ?? element.getAttribute("role")}`, `${text(element).slice(0, 120)}${progress ? ` [${progress.getAttribute("value")}/${progress.getAttribute("max")}]` : ""}`);
      }
      for (const element of document.querySelectorAll("[data-semio-plugin-install]")) {
        const progress = element.querySelector("[data-semio-plugin-install-progress]");
        push(`plugin-install:${element.getAttribute("data-plugin-install-ids") ?? ""}`, `${text(element).slice(0, 120)}${progress ? ` [${progress.getAttribute("value")}/${progress.getAttribute("max")}]` : ""}`);
      }
      for (const element of document.querySelectorAll('[role="dialog"]')) push("dialog", text(element));
    };
    const start = (): void => {
      scan();
      new MutationObserver(scan).observe(document.body, { subtree: true, childList: true, attributes: true });
    };
    if (document.body) start();
    else document.addEventListener("DOMContentLoaded", start);
  });

const notices = (page: Page): Promise<Notice[]> => page.evaluate(() => (globalThis as unknown as { __hubSweepNotices?: Notice[] }).__hubSweepNotices ?? []);
const windowIds = (page: Page): Promise<string[]> => page.evaluate(() => [...document.querySelectorAll("[data-window-id]")].map((element) => element.getAttribute("data-window-id")).filter((id): id is string => typeof id === "string"));

async function signIn(page: Page, email: string, password: string): Promise<string | null> {
  const badge = page.locator('[data-semio-hub-sign-in=""]').first();
  if ((await badge.count()) === 0) return "no sign-in badge";
  await badge.click({ force: true }).catch(() => undefined);
  const form = page.locator("[data-semio-hub-workspace]");
  await form.waitFor({ state: "visible", timeout: 60_000 }).catch(() => undefined);
  if ((await form.count()) === 0) return "hub workspace never opened";
  await form.locator('input[type="email"]').fill(email);
  await form.locator('input[type="password"]').fill(password);
  const inForm = form.locator('form:has(input[type="password"]) button[type="submit"]').first();
  await ((await inForm.count()) > 0 ? inForm : form.locator('button[type="submit"]').first()).click({ force: true }).catch(() => undefined);
  const signedIn = await page.waitForFunction(() => !document.querySelector('[data-semio-hub-sign-in=""]'), undefined, { timeout: 120_000 }).then(() => true, () => false);
  await page.waitForTimeout(6_000);
  return signedIn ? null : "the sign-in badge never left (credential refused or hub unreachable)";
}

const spaceRows = (page: Page): Promise<{ id: string | null; text: string }[]> =>
  page.evaluate(() => [...document.querySelectorAll("[data-semio-hub-workspace] li[data-space-id]")].map((row) => ({ id: row.getAttribute("data-space-id"), text: (row.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 80) })));

/** 🏘️ The sweep space: the listed space whose name matches, else a new one created from the hub workspace. */
async function ensureSpace(page: Page, name: string): Promise<{ id: string | null; text: string }[]> {
  let rows = await spaceRows(page);
  if (rows.some((row) => row.text.includes(name))) return rows;
  const input = page.locator('[data-element-alias="os.hub.spaces.createName"], #os\\.hub\\.spaces\\.createName').first();
  if ((await input.count()) === 0) return rows;
  await input.fill(name).catch(() => undefined);
  await page.locator('#os\\.hub\\.spaces\\.createSubmit, [id="os.hub.spaces.createSubmit"]').first().click({ force: true }).catch(() => undefined);
  const deadline = Date.now() + 90_000;
  while (Date.now() < deadline) {
    rows = await spaceRows(page);
    if (rows.some((row) => row.text.includes(name))) return rows;
    await page.waitForTimeout(2_000);
  }
  return rows;
}

/** 🗂️ The space index's rows (windowed tables key their rows `artifact:<id>`). */
const indexRows = (page: Page): Promise<{ id: string | null; text: string }[]> =>
  page.evaluate(() =>
    [...document.querySelectorAll('[data-slot="window-body"] tr, [data-slot="window-body"] [role="row"], [data-slot="window-body"] [data-row-id]')]
      .map((element) => ({ id: element.getAttribute("data-row-id") ?? (/^artifact:(.+)$/u.exec(element.getAttribute("data-ui-node-key") ?? "")?.[1] ?? null), text: (element.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 120) }))
      .filter((row) => row.text.length > 0)
      .slice(0, 60),
  );

/** 🪪️ Whether the space index lists a row naming the document within the deadline (a kind whose genesis waits on a cold
 * component install lists its row only once the guest answered). */
async function awaitIndexRow(page: Page, name: string, ms: number): Promise<boolean> {
  const deadline = Date.now() + ms;
  for (;;) {
    if ((await indexRows(page)).some((entry) => entry.text.includes(name))) return true;
    if (Date.now() >= deadline) return false;
    await page.waitForTimeout(3_000);
  }
}

/** 🗄️ The device's persisted plugin module store: record keys, blob count, and whether its service worker controls the page. */
const storeState = (page: Page): Promise<unknown> =>
  page
    .evaluate(async () => {
      const cache = await caches.open("semio-plugin-module-store-v1");
      const keys = (await cache.keys()).map((request) => decodeURIComponent(new URL(request.url).pathname));
      return { controlled: Boolean(navigator.serviceWorker?.controller), records: keys.filter((key) => key.includes("/record/")).length, blobs: keys.filter((key) => key.includes("/blob/")).length };
    })
    .catch((error: unknown) => ({ error: String(error).slice(0, 120) }));

/** 🔎️ One encoded kind choice (kind id + dialect) as the sweep drives it; null for any other option value. */
function kindChoiceOf(value: string): { kindId: string; plugin: string; value: string } | null {
  if (!value.startsWith("{")) return null;
  try {
    const choice = JSON.parse(value) as { kindId?: string; dialect?: { artifactKind?: string } };
    const kindId = String(choice.kindId ?? "");
    return kindId.length === 0 ? null : { kindId, plugin: /^s\.([a-z0-9-]+)\./u.exec(String(choice.dialect?.artifactKind ?? ""))?.[1] ?? "", value };
  } catch {
    return null;
  }
}

/** 🔎️ The kinds the space index's staged `createArtifact` form offers: each option's encoded choice, read from a native
 * `<select>` or — when the form renders the choice as a combobox — from the listbox it opens (closed again after). */
export async function stagedKinds(page: Page): Promise<{ kindId: string; plugin: string; value: string }[]> {
  const native = await page.evaluate(() => [...document.querySelectorAll('[data-slot="window-action-pane"] select option, select option')].map((element) => (element as HTMLOptionElement).value));
  const fromNative = native.map(kindChoiceOf).filter((choice) => choice !== null);
  if (fromNative.length > 0) return fromNative;
  const trigger = page.locator('[data-slot="window-action-pane"] [id$=".arg.kindChoice"] [role="combobox"], [data-slot="window-action-pane"] button#kindChoice').first();
  if ((await trigger.count()) === 0) return [];
  await trigger.click({ force: true, timeout: 8_000 }).catch(() => undefined);
  await page.locator('[role="option"]').first().waitFor({ state: "visible", timeout: 8_000 }).catch(() => undefined);
  const listed = await page.evaluate(() => [...document.querySelectorAll('[role="option"]')].map((option) => option.getAttribute("data-value") ?? ""));
  await page.keyboard.press("Escape");
  return listed.map(kindChoiceOf).filter((choice) => choice !== null);
}
//#endregion 🔖️Journey

//#region 🔖️Sweep
/** 🎛️ One sweep. */
export type HubDocumentSweepOptions = Readonly<{
  baseUrl: string;
  locale: "en" | "de";
  email: string;
  password: string;
  spaceName: string;
  kinds: readonly string[];
  sagaMs: number;
  puzzleSagaMs: number;
  reopen: boolean;
  cancel: "install" | "open" | null;
  profileDir: string;
  outDir: string;
  signal: AbortSignal;
}>;

/** 🧾️ One kind's row. */
export type HubSweepRow = Record<string, unknown> & { kindId: string; plugin: string; pass: boolean; faults: string[]; notices: Notice[] };

export async function openSweepSpace(page: Page, options: HubDocumentSweepOptions, row: HubSweepRow): Promise<string | null> {
  await page.goto(new URL("/hub", options.baseUrl).href, { waitUntil: "commit", timeout: 300_000 });
  row.beacon = await awaitBeacon(page, Date.now() + 300_000);
  await dismissIntroduction(page);
  if (await page.locator('[data-semio-hub-sign-in=""]').count()) {
    const refused = await signIn(page, options.email, options.password);
    if (refused) return refused;
  }
  await page
    .waitForFunction(() => { const phase = document.querySelector("[data-semio-hub-spaces-phase]")?.getAttribute("data-semio-hub-spaces-phase"); return phase !== undefined && phase !== null && phase !== "loading"; }, undefined, { timeout: 60_000 })
    .catch(() => undefined);
  const spaces = await ensureSpace(page, options.spaceName);
  const space = spaces.find((entry) => entry.text.includes(options.spaceName));
  if (!space?.id) return `the hub workspace lists no space named ${JSON.stringify(options.spaceName)} (${spaces.length} listed)`;
  row.spaceId = space.id;
  await click(page, `[data-semio-hub-workspace] li[data-space-id="${space.id}"] button`);
  const deadline = Date.now() + 180_000;
  while (Date.now() < deadline) {
    const shell = await readShell(page);
    const uri = await page.evaluate(() => location.pathname);
    if (uri.includes(`/spaces/${space.id}`) && shell.windowIds.some((id) => id !== "s-home-main")) {
      await page.waitForTimeout(8_000);
      return null;
    }
    await page.waitForTimeout(2_000);
  }
  return `the space index never mounted at /spaces/${space.id}`;
}

/** 🌱️ Stages `createArtifact` with a name and the kind picked by its encoded choice; answers the submit outcome. */
export async function createKind(page: Page, kindValue: string, name: string): Promise<string> {
  const createRow = page.locator('[data-slot="window-action-pane"] [id="action.createArtifact"]').first();
  if ((await createRow.count()) === 0) return "no action.createArtifact row";
  await createRow.click({ force: true }).catch(() => undefined);
  await page.waitForTimeout(2_000);
  await page.evaluate(() => {
    for (const element of document.querySelectorAll('[data-slot="window-action-pane"] [aria-expanded="false"]')) if (/create artifact|artefakt anlegen/iu.test(element.textContent ?? "")) (element as HTMLElement).click();
  });
  await page.waitForTimeout(800);
  const input = page.locator('[data-slot="window-action-pane"] [id$=".arg.name"]:is(input,textarea), [data-slot="window-action-pane"] [id$=".arg.name"] :is(input,textarea)').first();
  if ((await input.count()) === 0) return "no name field";
  await input.fill(name);
  const select = page.locator('[data-slot="window-action-pane"] select[id$=".arg.kindChoice"], select').filter({ has: page.locator(`option[value="${kindValue.replace(/"/gu, '\\"')}"]`) }).first();
  if ((await select.count()) > 0) await select.selectOption(kindValue).catch(() => undefined);
  else {
    const trigger = page.locator('[data-slot="window-action-pane"] [id$=".arg.kindChoice"] [role="combobox"], [data-slot="window-action-pane"] button#kindChoice').first();
    await trigger.focus().catch(() => undefined);
    await page.keyboard.press("Enter");
    await page.waitForTimeout(800);
    await page.locator(`[role="option"][data-value="${kindValue.replace(/"/gu, '\\"')}"]`).first().click({ force: true }).catch(() => undefined);
  }
  await page.waitForTimeout(500);
  for (const selector of ['[data-slot="window-action-pane"] [id$=".action.createArtifact.execute"]', '[id$=".action.createArtifact.execute"]', '[id$="createArtifact.execute"]']) {
    const outcome = await click(page, selector);
    if (outcome !== "absent") return outcome;
  }
  return "no execute control";
}

async function sweepKind(page: Page, kind: { kindId: string; plugin: string; value: string }, options: HubDocumentSweepOptions, faults: string[]): Promise<HubSweepRow> {
  const started = Date.now();
  const faultCursor = faults.length;
  const row: HubSweepRow = { kindId: kind.kindId, plugin: kind.plugin, pass: false, faults: [], notices: [] };
  const pins = readMatrixPins();
  const blocked = await openSweepSpace(page, options, row);
  if (blocked) {
    row.detail = blocked;
    return row;
  }
  const noticeCursor = (await notices(page)).length;
  await unfoldActionsRail(page);
  row.documentName = `Hub Sweep ${kind.kindId} ${Date.now() % 100000}`;
  row.submitted = await createKind(page, kind.value, String(row.documentName));
  if (options.cancel === "install" || options.cancel === "open") {
    const selector = options.cancel === "install" ? "[data-semio-plugin-install] button" : "[data-semio-execution-target-cancel]";
    await page.locator(selector).first().waitFor({ state: "visible", timeout: 180_000 }).catch(() => undefined);
    row.cancelled = await page.locator(selector).first().click({ timeout: 10_000 }).then(() => "ok", (error: unknown) => String(error).split("\n")[0]!.slice(0, 80));
    await page.waitForTimeout(8_000);
    row.afterCancel = { windows: await windowIds(page), store: await storeState(page) };
  }
  row.created = await awaitIndexRow(page, String(row.documentName), 120_000);
  const sagaMs = kind.kindId.includes("puzzle") ? options.puzzleSagaMs : options.sagaMs;
  const sagaDeadline = Date.now() + sagaMs;
  while (Date.now() < sagaDeadline && !options.signal.aborted) {
    const ids = await windowIds(page);
    if (!ids.includes("framework.window.table") && ids.length > 0) break;
    const creating = await page.evaluate(() => [...document.querySelectorAll('[role="status"], [role="alert"]')].some((element) => /Creating artifact|Artefakt wird erstellt|Opening the artifact|wird geöffnet/iu.test(element.textContent ?? "")));
    if (!creating && Date.now() - started > 30_000) break;
    await page.waitForTimeout(1_000);
  }
  row.createToOpenMs = Date.now() - started;
  const opened = (await windowIds(page)).filter((id) => id !== "framework.window.table" && id !== "s-home-main");
  row.openedWindows = opened;
  if (opened.length > 0 && options.cancel === null) {
    const verb = pins.hubKindVerbs[kind.kindId] ?? pins.pluginVerbs[kind.plugin];
    const pre = pins.hubKindPre[kind.kindId];
    if (pre) {
      await unfoldActionsRail(page);
      row.preVerb = `${pre}:${await click(page, `[data-slot="window-action-pane"] [id="action.${pre}"]`)}`;
      await page.waitForTimeout(2_000);
    }
    const args: Record<string, Readonly<Record<string, string>>> = {};
    for (const [argKey, value] of Object.entries(pins.pluginArgs)) if (argKey.startsWith(`${kind.plugin}.`)) args[`${kind.kindId}.${argKey.slice(kind.plugin.length + 1)}`] = value;
    const renderedEdits: Record<string, MatrixRenderedEdit> = {};
    for (const [editKey, edit] of Object.entries(pins.pluginEdits)) if (editKey.startsWith(`${kind.plugin}.`)) renderedEdits[`${kind.kindId}.${editKey.slice(kind.plugin.length + 1)}`] = edit;
    const refusals: string[] = [];
    const result = await mutateUndoRedo(page, refusals, kind.kindId, verb ? { [kind.kindId]: verb } : {}, args, renderedEdits, pins.liveId, 6);
    const edits = ("edits" in result ? result.edits : []) as number[];
    Object.assign(row, { verb: result.mutation, verbDetail: result.mutationDetail, railRows: result.railRows, edits, refusals: refusals.slice(0, 5), ledgerTail: (await readShell(page)).ledger.slice(-4).map((entry) => entry.label) });
    row.cleanRoundTrip = edits.length === 4 && edits[1]! >= 1 && edits[2] === edits[1]! - 1 && edits[3] === edits[1];
    if (options.reopen) {
      await page.reload({ waitUntil: "commit" });
      await awaitBeacon(page, Date.now() + 300_000);
      await page.waitForTimeout(6_000);
      const target = (await indexRows(page)).find((entry) => entry.id !== null && entry.text.includes(String(row.documentName)));
      if (target?.id) {
        const locator = page.locator(`[data-slot="window-body"] [data-ui-node-key="artifact:${target.id}"]`).first();
        await locator.dblclick({ force: true }).catch(() => undefined);
        const reopenDeadline = Date.now() + 150_000;
        while (Date.now() < reopenDeadline && (await windowIds(page)).filter((id) => id !== "framework.window.table" && id !== "s-home-main").length === 0) await page.waitForTimeout(1_000);
      }
      row.reopenedWindows = (await windowIds(page)).filter((id) => id !== "framework.window.table" && id !== "s-home-main");
    }
  }
  await page.screenshot({ path: join(options.outDir, `${kind.kindId.replace(/[^A-Za-z0-9]+/gu, "-")}.png`) }).catch(() => undefined);
  row.store = await storeState(page);
  if (row.created !== true && options.cancel === null) {
    row.createdLate = (await openSweepSpace(page, options, row)) === null && (await awaitIndexRow(page, String(row.documentName), 60_000));
    row.created = row.createdLate;
  }
  row.notices = (await notices(page)).slice(noticeCursor);
  row.faults = faults.slice(faultCursor, faultCursor + 8);
  row.totalMs = Date.now() - started;
  row.pass =
    options.cancel !== null
      ? row.cancelled === "ok" && (row.openedWindows as string[]).length === 0
      : row.created === true && (row.openedWindows as string[]).length > 0 && row.cleanRoundTrip === true && (!options.reopen || ((row.reopenedWindows as string[] | undefined) ?? []).length > 0) && faults.length === faultCursor;
  return row;
}

/** 🗂️ Runs the sweep over every selected creatable kind in one persistent profile; the report is rewritten after every kind
 * and the signal ends the run after the current kind. */
export async function runHubDocumentSweep(options: HubDocumentSweepOptions): Promise<{ rows: HubSweepRow[]; kinds: string[]; cancelled: boolean; fatal?: string }> {
  mkdirSync(options.outDir, { recursive: true });
  ensureParityPlaywrightBrowsersPath();
  const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
  const context = await chromium.launchPersistentContext(options.profileDir, { headless: true, args: ["--use-angle=metal"], viewport: { width: 1600, height: 1000 }, locale: options.locale === "de" ? "de-DE" : "en-US" });
  await installNoticeRecorder(context);
  const page = context.pages()[0] ?? (await context.newPage());
  page.setDefaultNavigationTimeout(180_000);
  const faults: string[] = [];
  page.on("pageerror", (error) => faults.push(`pageerror: ${String(error)}`.slice(0, 240)));
  page.on("console", (message) => {
    const text = message.text();
    if (FAULT.test(text) && !NOISE.test(text) && !/WebSocket connection to 'ws:\/\/127\.0\.0\.1:\d+\/bridge' failed/u.test(text)) faults.push(`${message.type()}: ${text}`.slice(0, 240));
  });
  const report: { baseUrl: string; locale: string; started: string; finished: string | null; kinds: string[]; rows: HubSweepRow[]; cancelled: boolean; fatal?: string } = { baseUrl: options.baseUrl, locale: options.locale, started: new Date().toISOString(), finished: null, kinds: [], rows: [], cancelled: false };
  const flush = (): void => writeFileSync(join(options.outDir, "hub-sweep.json"), JSON.stringify(report, null, 1));
  try {
    const probe: HubSweepRow = { kindId: "-", plugin: "-", pass: false, faults: [], notices: [] };
    const blocked = await openSweepSpace(page, options, probe);
    if (blocked) throw new Error(blocked);
    await unfoldActionsRail(page);
    await click(page, '[data-slot="window-action-pane"] [id="action.createArtifact"]');
    await page.waitForTimeout(2_000);
    const offered = await stagedKinds(page);
    const kinds = offered.filter((kind) => options.kinds.length === 0 || options.kinds.includes(kind.kindId));
    report.kinds = kinds.map((kind) => kind.kindId);
    console.log(`[hub-sweep] space ${String(probe.spaceId)}; ${offered.length} creatable kinds, ${kinds.length} selected: ${report.kinds.join(", ")}`);
    flush();
    for (const [index, kind] of kinds.entries()) {
      if (options.signal.aborted) {
        report.cancelled = true;
        break;
      }
      const row = await sweepKind(page, kind, options, faults).catch((error: unknown) => ({ kindId: kind.kindId, plugin: kind.plugin, pass: false, faults: [], notices: [], detail: `probe error: ${String(error).split("\n")[0]!.slice(0, 200)}` }) as HubSweepRow);
      report.rows.push(row);
      flush();
      console.log(`[hub-sweep] ${index + 1}/${kinds.length} ${row.pass ? "PASS" : "FAIL"} ${kind.kindId} (${kind.plugin}) created=${String(row.created ?? "-")} opened=${((row.openedWindows as string[] | undefined) ?? []).length} verb=${String(row.verb ?? "-")} edits=${JSON.stringify(row.edits ?? null)} faults=${row.faults.length} ${String(row.detail ?? row.verbDetail ?? "")} ${Math.round(Number(row.totalMs ?? 0) / 1000)}s`);
    }
  } catch (error) {
    report.fatal = String(error instanceof Error ? error.message : error).split("\n")[0]!.slice(0, 400);
    console.log(`[hub-sweep] FATAL ${report.fatal}`);
  } finally {
    report.finished = new Date().toISOString();
    flush();
    await context.close();
  }
  return report;
}

function flagValue(segments: readonly string[], flag: string): string | undefined {
  const index = segments.indexOf(flag);
  const value = index >= 0 ? segments[index + 1] : undefined;
  return value === undefined || value.startsWith("--") ? undefined : value;
}

/** 🚪️ `verify hub-sweep --serve <url> --hub <url> [--locale en|de] [--space <name>] [--kinds <kindId,…>] [--saga-ms <n>]
 * [--puzzle-saga-ms <n>] [--reopen] [--cancel install|open] [--profile <dir>] [--tag <t>] [--out <dir>]` — runs against the
 * serve `--serve` names, joined to the hub `--hub` names (reused, or started and stopped by `withDevServe`; a scratch browser
 * profile is used and removed unless `--profile` names one to keep, e.g. to measure a later session);
 * credentials `OS_HUB_PROBE_EMAIL` / `OS_HUB_PROBE_PASSWORD` (default the development hub's first user). Writes
 * `hub-sweep.json` + one screenshot per kind under `<out>/<tag>/`, publishes the acceptance record, exits non-zero unless
 * every selected kind passes. */
export async function runHubDocumentSweepCli(repoRoot: string, defaultOutDir: string, segments: readonly string[]): Promise<void> {
  const serveUrl = flagValue(segments, "--serve");
  const hubUrl = flagValue(segments, "--hub");
  if (!serveUrl || !hubUrl) throw new Error("usage: verify hub-sweep --serve <url> --hub <url> [--locale en|de] [--space <name>] [--kinds …] [--saga-ms <n>] [--puzzle-saga-ms <n>] [--reopen] [--cancel install|open] [--profile <dir>] [--tag <t>] [--out <dir>]");
  const locale = flagValue(segments, "--locale") === "de" ? "de" : "en";
  const tag = flagValue(segments, "--tag") ?? `hub-sweep-${locale}`;
  const cancelMode = flagValue(segments, "--cancel");
  if (cancelMode !== undefined && cancelMode !== "install" && cancelMode !== "open") throw new Error("--cancel accepts install | open");
  const controller = new AbortController();
  const abort = (): void => controller.abort();
  process.once("SIGINT", abort);
  process.once("SIGTERM", abort);
  const startedAt = new Date();
  const profile = flagValue(segments, "--profile");
  const scratchProfile = profile === undefined ? mkdtempSync(join(tmpdir(), "semio-hub-sweep-profile-")) : undefined;
  await withAcceptanceRecord(repoRoot, "hub-document-sweep", () => withDevServe(repoRoot, "hub-document-sweep", { serveUrl, hubUrl, locale, signal: controller.signal, startedAt }, async (baseUrl) => {
    const outDir = join(resolve(flagValue(segments, "--out") ?? defaultOutDir), tag);
    const report = await runHubDocumentSweep({
      baseUrl,
      locale,
      email: process.env.OS_HUB_PROBE_EMAIL ?? "user1@semio.dev",
      password: process.env.OS_HUB_PROBE_PASSWORD ?? "gm1-local-dev-pass-1",
      spaceName: flagValue(segments, "--space") ?? "Hub Document Sweep",
      kinds: (flagValue(segments, "--kinds") ?? "").split(",").filter(Boolean),
      sagaMs: Number(flagValue(segments, "--saga-ms") ?? 300_000),
      puzzleSagaMs: Number(flagValue(segments, "--puzzle-saga-ms") ?? 900_000),
      reopen: segments.includes("--reopen"),
      cancel: (cancelMode as "install" | "open" | undefined) ?? null,
      profileDir: profile ?? scratchProfile!,
      outDir,
      signal: controller.signal,
    });
    const passed = report.rows.filter((row) => row.pass).length;
    const total = report.rows.length;
    const failing = report.rows.filter((row) => !row.pass).map((row) => `${row.kindId}: ${String(row.detail ?? row.verbDetail ?? (row.cleanRoundTrip === false ? `edits ${JSON.stringify(row.edits)}` : row.faults[0] ?? "not opened"))}`.slice(0, 120));
    const unreachable = total === 0 && /ERR_CONNECTION_REFUSED|never left|lists no space/u.test(report.fatal ?? "");
    const status = unreachable ? "blocked" : report.fatal || report.cancelled || total === 0 ? "fail" : passed === total ? "pass" : "fail";
    publishAcceptanceCheckResult(
      repoRoot,
      acceptanceCheckResult({
        check: "hub-document-sweep",
        status,
        startedAt,
        measured: { locale, kinds: total, offered: report.kinds.length, passed, failed: total - passed, cancelMode: cancelMode ?? "none", reopen: segments.includes("--reopen"), fatal: Boolean(report.fatal) },
        summary: {
          en: `${passed}/${total} hub-creatable kinds created, opened by the creation saga and round-tripped in ${locale}${failing.length ? `; failing: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; fatal: ${report.fatal.slice(0, 160)}` : ""}`,
          de: `${passed}/${total} auf dem Hub anlegbare Arten angelegt, vom Anlage-Ablauf geöffnet und hin und zurück bearbeitet in ${locale}${failing.length ? `; fehlgeschlagen: ${failing.slice(0, 4).join("; ")}` : ""}${report.fatal ? `; Abbruch: ${report.fatal.slice(0, 160)}` : ""}`,
        },
        evidence: [join(outDir, "hub-sweep.json")],
      }),
    );
    console.log(`[hub-sweep] === ${tag}: PASS ${passed}/${total} → ${outDir} ===`);
    if (status !== "pass") process.exitCode = 1;
  }));
  if (scratchProfile) rmSync(scratchProfile, { recursive: true, force: true });
  process.removeListener("SIGINT", abort);
  process.removeListener("SIGTERM", abort);
}
//#endregion 🔖️Sweep
