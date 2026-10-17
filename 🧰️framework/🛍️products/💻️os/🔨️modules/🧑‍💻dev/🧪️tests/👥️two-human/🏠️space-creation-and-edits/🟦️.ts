/** 🏠️ The Space app steps of the two-human harness: mounting and settling the shell, opening the Space and its rows, creating a document, and reading, editing and undoing its text with the hub head as witness. */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { clickUncovered, fillStagedArgument, readMatrixPins, readShell, submitStagedVerb, unfoldActionsRail } from "../../🧮️program-matrix/🟦️.ts";
import { activate, clickRowAction, dialog, originOf, pause, readStatus, rowIds, type Session, submitDialog, until, waitNewRow, waitRow } from "../🔑️sessions/🟦️.ts";

//#region 🔖️Journey
/** 🧩️ Kind ids whose dialect does not name its plugin (the catalog names the dialect only). */
export const PLUGIN_BY_KIND: Readonly<Record<string, string>> = { "2d.drawing": "draw", "text.document": "writer", "2d.puzzle": "puzzle", "3d.puzzle": "puzzle", "5d.puzzle": "puzzle", "2d.block": "block", "3d.block": "block", "5d.block": "block", "3d.wfcgrid3d": "wfc", "animate.presentation": "animate" };

/** 🪟️ The shell's own chrome windows — a document is mounted once a window other than these shows. */
const SHELL_WINDOWS = new Set(["framework.window.table", "s-home-main"]);

/** ⏱️ How often a mount is polled; a mount is stamped when the poll that sees it runs, before the panels are unfolded, so a
 * timing carries at most one poll interval of slack (it used to carry a 1 s poll plus ~2 s of panel clicks and pauses). */
const MOUNT_POLL_MS = 200;

export async function awaitMounted(session: Session, deadlineMs: number) {
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
export async function openSpace(session: Session, spaceId: string, misses: unknown[]): Promise<void> {
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
export async function creatableKinds(session: Session): Promise<CreatableKind[]> {
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

export async function createArtifact(session: Session, name: string, kind: CreatableKind, budgetMs: number): Promise<string> {
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
export const docText = (session: Session): Promise<string> =>
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

/** ⏱️ How often an edit's arrival is polled while it is being timed: an edit round trip is a few hundred ms, so the 500 ms
 * default poll would be its own error bar. */
const EDIT_POLL_MS = 40;

/** ⏱️ {@link awaitText} with the epoch ms of the poll that first saw the change (`null` when it never came). */
async function awaitTextAt(session: Session, predicate: (now: string) => boolean, deadlineMs: number): Promise<{ seen: string | { missed: string }; at: number | null }> {
  const seen = await until(async () => {
    const now = await docText(session);
    return predicate(now) ? { now, at: Date.now() } : null;
  }, deadlineMs, EDIT_POLL_MS);
  return seen === null ? { seen: { missed: await docText(session) }, at: null } : { seen: seen.now, at: seen.at };
}

/** ⏱️ Every distinct document text `session` shows from now on, each with the epoch ms of the poll that first saw it — a
 * peer's view also moves for presence and renders its own selection, so an edit's arrival is the first time after the
 * submit that the peer shows the text it settles on. */
export function watchText(session: Session, deadlineMs: number): { readonly stop: () => Promise<readonly { readonly text: string; readonly at: number }[]> } {
  const seen: { text: string; at: number }[] = [];
  let running = true;
  const done = (async () => {
    for (const deadline = Date.now() + deadlineMs; running && Date.now() < deadline; await new Promise((resolveDelay) => setTimeout(resolveDelay, EDIT_POLL_MS))) {
      const text = await docText(session).catch(() => "");
      if (seen.at(-1)?.text !== text) seen.push({ text, at: Date.now() });
    }
  })();
  return {
    stop: async () => {
      running = false;
      await done;
      return seen;
    },
  };
}

/** ⏱️ Submit → the first poll at which the peer showed the text it settled on (`null` when the edit never applied or the
 * peer never moved after the submit). */
export const arrivalMs = (texts: readonly { readonly text: string; readonly at: number }[], edit: { readonly applied: boolean; readonly submittedAt: number }): number | null => {
  const settled = texts.at(-1);
  const hit = edit.applied && settled !== undefined && settled.at >= edit.submittedAt ? texts.find((entry) => entry.text === settled.text && entry.at >= edit.submittedAt) : undefined;
  return hit === undefined ? null : hit.at - edit.submittedAt;
};

/** ⏱️ How long a peer's view must hold still before the text it shows counts as settled. */
export const SETTLE_MS = 1_500;

export async function awaitText(session: Session, predicate: (now: string) => boolean, deadlineMs: number): Promise<string | { missed: string }> {
  const seen = await until(async () => {
    const now = await docText(session);
    return predicate(now) ? now : null;
  }, deadlineMs);
  return seen ?? { missed: await docText(session) };
}

export const seenText = (value: string | { missed: string }): string => (typeof value === "string" ? value : value.missed);

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

export async function edit(session: Session, plugin: string, pins: ReturnType<typeof readMatrixPins>, overrides: Readonly<Record<string, string>> = EDIT_ARGS[plugin] ?? {}) {
  const verb = pins.pluginVerbs[plugin];
  if (verb === undefined) throw new Error(`no pinned document verb for plugin ${plugin}`);
  const before = await docText(session);
  const staged = await session.page.locator(`[id$=".action.${verb}.execute"]`).first().isVisible().catch(() => false);
  if (!staged) await clickUncovered(session.page, `[data-slot="window-action-pane"] [id="action.${verb}"]`);
  await pause(session, 1_000);
  for (const [key, value] of Object.entries({ ...personalArgs(pins.pluginArgs[`${plugin}.${verb}`] ?? pins.pluginArgs[verb] ?? {}, session.human.label, pins.liveId), ...overrides })) await fillStagedArgument(session.page, key, value, pins.liveId);
  const submittedAt = Date.now();
  await submitStagedVerb(session.page, verb);
  const after = await awaitTextAt(session, (now) => now !== before, 20_000);
  return { verb, after: seenText(after.seen), applied: typeof after.seen === "string", submittedAt, localMs: after.at === null ? null : after.at - submittedAt };
}

export async function undo(session: Session, verb: "undo" | "redo" = "undo") {
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
export async function awaitSharedSpace(session: Session, spaceId: string, misses: unknown[]): Promise<void> {
  if (await waitRow(session.page, "space", spaceId, 45_000).then(() => true).catch(() => false)) return;
  misses.push({ human: session.human.label, spaceId, home: true, at: new Date().toISOString() });
  await session.page.reload({ waitUntil: "domcontentloaded" });
  await waitRow(session.page, "space", spaceId, 120_000);
}

//#endregion 🔖️Journey
//#region 🔖️Journeys
/** 🖍️ Arguments a journey's edit pins beyond the program matrix's (never personalized): draw's `addLayer` defaults to an empty
 * `path` node the pen draws into — no geometry, nothing on the canvas, nothing to select — so a journey adds a filled shape. */
const EDIT_ARGS: Readonly<Record<string, Readonly<Record<string, string>>>> = { draw: { kind: "shape:rect" } };
//#endregion 🔖️Journeys
