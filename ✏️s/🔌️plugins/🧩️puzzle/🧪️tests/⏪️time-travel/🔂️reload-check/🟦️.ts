/** 🔂️ The reload half of step 5: the folder archive, the reconnect band and the forgotten folder. */
import { join } from "node:path";
import { mkdirSync, readdirSync } from "node:fs";
import type { Ctx } from "../🌄️drag-journey/🟦️.ts";
import { OUT, stamp, currentLocale, setExpectedReloads, expectedReloads, setProbeReloadAt, page, renderer } from "@semio-tech/framework/time-travel/testing/state";
import { positions, movedIds, cameraOf, vitals, dragBy, toScreen, paneBox, offsetOf } from "../🎲️board/🟦️.ts";
import { closePanels, waitForBoot, openPanelTabIds } from "@semio-tech/framework/time-travel/testing/chrome";
import { attachFolder, readAlternatives, isCurrentAlternative, pickSyncFolder } from "@semio-tech/framework/time-travel/testing/history-session";
import { verdict, emit, note, log, atStep } from "@semio-tech/framework/time-travel/testing/report";
import { waitUntil, evalSafe, shot, sleep } from "@semio-tech/framework/time-travel/testing/page";
import { allHistoryRows, documentEntries, documentEditIds, expandEntry, findMutationRow, openHistory, readsAsOpText } from "@semio-tech/framework/time-travel/testing/history";
import { harvestNotices, installBandTrace, installNoticeTrace, prepareChord } from "@semio-tech/framework/time-travel/testing/band";
import { armReprojection, reprojectionSettled, reprojectionSummary, disarmReprojection, reprojectionShellVerdicts } from "@semio-tech/framework/time-travel/testing/reprojection";
import { wgpuPress, mirror } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";
import { consoleRows } from "@semio-tech/framework/time-travel/testing/console";
import { COPY } from "../🌍️copy/🟦️.ts";

//#region 🔖️Steps

/** 🔁️ Step 5's reload half, taken after step 8. A `?plugin=` playground opens no space, so
 * `resolveDocumentOpeningBindings` (`🏛️ShellHost/🧭️opening/🟦️.ts`) answers no binding and a bare reload re-runs Set
 * Active Example; the one local-only route is the sync card's folder attach, remembered per device in
 * `os.config.local-folders` by the document's store id. The folder is attached at boot (`--folder-at=1`) or here with one
 * edit to trigger the write (`--folder-at=5`); then: see the archive on disk, reload, take the reconnect band's offer
 * (`🏛️ShellHost/📁️local-folders`, fallback: attach again by hand), compare head, rows and edit ids with the ones before the
 * reload, and finally detach and reload once more — a forgotten folder must not be offered. */
export const reloadCheck = async (ctx: Ctx) => {
  if (!ctx.afterOverwrite && !ctx.g4) return;
  if (ctx.noFolder) return plainReloadCheck(ctx);
  const early = ctx.folder !== undefined;
  const folder = ctx.folder ?? join(OUT, `folder-${stamp}-${currentLocale}`);
  mkdirSync(folder, { recursive: true });
  const beforeAttach = await positions();
  await closePanels();
  const attached = early ? { card: true, typed: folder, attachButton: true } : await attachFolder(folder);
  const afterAttach = await positions();
  if (!early) verdict("folder-attach-keeps-the-head", movedIds(beforeAttach, afterAttach, 1e-6).length === 0, { attached, drift: movedIds(beforeAttach, afterAttach, 1e-6).slice(0, 6) });
  const files = () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => `${entry.parentPath ?? ""}/${entry.name}`.replace(folder, ""));
  if (!early && ctx.c && afterAttach[ctx.c]) {
    await closePanels();
    const zoom = cameraOf(await vitals()).zoom;
    await dragBy(toScreen(afterAttach[ctx.c], cameraOf(await vitals()), await paneBox()), 10 * zoom, 10 * zoom);
    await waitUntil(positions, (p) => offsetOf(afterAttach, p, ctx.c!)?.some((v) => Math.abs(v) > 0.5) === true, 15000);
  }
  const written = await waitUntil(async () => files(), (list) => list.length > 0, 30000, 1000);
  verdict("folder-attach-writes-the-document-archive", attached.typed === folder && written.ok, { files: written.value.slice(0, 8), folder, waitedMs: written.waitedMs, reading: "the archive is written on the bound document's outbound mutations (`archivePersistence` in `bindDocumentBackbone.send`), so one edit follows the attach" });
  const before = await positions();
  const historyBefore = await allHistoryRows();
  const rowsBefore = documentEntries(historyBefore).map((row) => row.label.slice(0, 60));
  const editsBefore = await documentEditIds(historyBefore);
  emit({ kind: "rows", tag: "before-reload", editIds: editsBefore, rows: historyBefore.filter((row) => row.kind === "entry").map((row) => ({ key: row.key, label: row.label, text: row.text.slice(0, 160), expandable: row.expandable })) });
  const warned = historyBefore.find((row) => row.kind === "entry" && /Set Active Example|Beispiel/i.test(row.label) && /Warning|Warnung/.test(row.text));
  if (warned) {
    const expanded = await expandEntry(warned.id);
    note("example-row-warning-after-attach", { label: warned.text.slice(0, 120), flagged: expanded.mutations.filter((row) => /Warning|Warnung/.test(row.text)).slice(0, 8).map((row) => row.text.slice(0, 200)), shown: expanded.mutations.length });
  }
  const listedBefore = await waitUntil(readAlternatives, (rows) => rows.length > 0, 40000, 2000);
  const alternativesBefore = listedBefore.value;
  if (!listedBefore.ok) note("no-alternatives-listed-before-the-reload", { waitedMs: listedBefore.waitedMs, commits: historyBefore.filter((row) => row.kind === "entry" && /Commit Checkpoint|Kontrollpunkt/i.test(row.label)).length, reading: "the History body shows its Alternatives section (the main line) only once the document holds a checkpoint; a folder-bound document commits one by itself about 20 s after an edit, and none came within the wait — the before/after comparison of the alternatives cannot be judged for this document" });
  else if (listedBefore.waitedMs > 3000) note("alternatives-listed-after-the-first-checkpoint", { waitedMs: listedBefore.waitedMs, rows: alternativesBefore.map((row) => row.text.slice(0, 50)) });
  const currentBefore = alternativesBefore.find(isCurrentAlternative);
  const g4Before = ctx.g4 ? (await findMutationRow(ctx.g4.drag))?.text ?? null : null;
  emit({ kind: "alternatives", tag: "before-reload", rows: alternativesBefore, current: currentBefore?.key ?? null, g4: g4Before });
  await closePanels();
  await harvestNotices();
  setExpectedReloads(expectedReloads + 1);
  setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebooted = await waitForBoot("reload", 60);
  await installBandTrace();
  await installNoticeTrace();
  const bare = await positions();
  note("bare-reload-reruns-the-example", { driftCount: movedIds(before, bare, 1e-6).length, route: await evalSafe(() => location.search, "") });
  const example = (await allHistoryRows()).filter((row) => row.kind === "entry" && /Set Active Example|Beispiel/i.test(row.label));
  if (example[0]) {
    const expanded = await expandEntry(example[0].id);
    note("post-reload-example-row", { label: example[0].label.slice(0, 120), text: example[0].text.slice(0, 200), flagged: expanded.mutations.filter((row) => /Warning|Warnung|Error|Fehler/.test(row.text)).slice(0, 6).map((row) => row.text.slice(0, 160)) });
  }
  await closePanels();
  const folderName = folder.split("/").pop() ?? folder;
  const offer = await waitUntil(reconnectBand, (state) => state?.state === "offered", 20000, 500);
  const offered = offer.value;
  verdict("folder-reconnect-offered", Boolean(offered && offered.state === "offered" && offered.role === "status" && offered.live === "polite" && offered.message.includes(folderName) && offered.reconnect && offered.forget), { band: offered, folderName, waitedMs: offer.waitedMs });
  let reattached: Record<string, unknown>;
  if (offered?.reconnect) {
    await openHistory();
    await armReprojection("observe");
    const via = renderer === "wgpu" ? await wgpuPress("s-folder-reconnect") : await page.locator("#s-folder-reconnect").click({ timeout: 4000 }).then(() => "button").catch(() => null);
    const busy = await waitUntil(reconnectBand, (state) => state === null || state.state === "reconnecting", 10000, 100);
    const settled = await waitUntil(reconnectBand, (state) => state === null, 60000, 500);
    await reprojectionSettled(60000);
    const load = reprojectionSummary(await disarmReprojection());
    reattached = { via, sawBusy: busy.value?.state === "reconnecting", bandGone: settled.ok, waitedMs: settled.waitedMs, load };
    verdict("folder-reconnect-attaches-and-closes-the-band", Boolean(via) && settled.ok, reattached);
    if (load.seen) {
      verdict("stepped-load-shows-its-progress", load.kinds.includes("load") && (load.total ?? 0) > 0, { load, reading: "§20.8: a whole-document load rides the history wire (`HistoryPatch.reprojection {kind: load}`) — \"Document load\" / \"Loading document: d of t\" in the shell status and the body's `framework.history.reprojection` section" });
      reprojectionShellVerdicts(load, "-load");
    } else note("stepped-load-finished-within-one-refresh", { load, reading: "the reconnect's archive load finished before anything re-rendered (progress refreshes ≥ 100 ms); step 18 loads a grown archive" });
  } else {
    reattached = { fallback: await attachFolder(folder) };
    note("folder-reconnect-fallback-manual-attach", reattached);
  }
  const restored = await waitUntil(positions, (p) => movedIds(before, p, 1e-6).length === 0, 60000, 1000);
  const drift = movedIds(before, restored.value, 1e-6);
  verdict("positions-persist-after-reload", rebooted && restored.ok, { rebooted, reattached, a: ctx.a ? restored.value[ctx.a] : null, expected: ctx.a ? before[ctx.a] : null, driftCount: drift.length, drift: drift.slice(0, 6), folder });
  note("folder-backbone-requests", { requests: consoleRows.filter((row) => row.locale === currentLocale && row.type === "backbone").slice(-30).map((row) => `${row.t}s ${row.text}`), sessions: await evalSafe(() => Array.from(document.querySelectorAll("[data-surface-id]")).map((el) => el.getAttribute("data-surface-id")).slice(0, 3), [] as (string | null)[]) });
  const hydrated = await waitUntil(allHistoryRows, (all) => documentEntries(all).length >= rowsBefore.length, 20000, 1000);
  const historyAfter = hydrated.value;
  const editsAfter = await documentEditIds(historyAfter);
  emit({ kind: "rows", tag: "after-reload-reattach", editIds: editsAfter, rows: historyAfter.filter((row) => row.kind === "entry").map((row) => ({ key: row.key, label: row.label, text: row.text.slice(0, 160), expandable: row.expandable })) });
  verdict("edit-ids-survive-the-reload", editsBefore.length > 0 && editsBefore.every((id) => editsAfter.includes(id)), { before: editsBefore.length, after: editsAfter.length, lost: editsBefore.filter((id) => !editsAfter.includes(id)).slice(0, 12) });
  const labels = documentEntries(historyAfter).map((row) => row.label.slice(0, 60));
  const lost = rowsBefore.filter((label) => !labels.includes(label));
  verdict("document-rows-survive-the-reload", rowsBefore.length > 0 && lost.length === 0 && labels.length >= rowsBefore.length, { before: rowsBefore.slice(0, 24), after: labels.slice(0, 24), lost: lost.slice(0, 16), counts: [rowsBefore.length, labels.length], waitedMs: hydrated.waitedMs, reading: "G5: every pre-reload row comes back with its localized label (rows whose command emits no description included: `Edit.verb`)" });
  const opText = labels.filter(readsAsOpText);
  verdict("no-row-reads-op-text-after-reload", opText.length === 0, { opText: opText.slice(0, 8), reading: "G5: no row falls back to the locale-invariant op text (`create-node node {…}`)" });
  const overwrite = COPY[currentLocale].row("edit", null, 1);
  if (rowsBefore.some((label) => label.startsWith(overwrite))) verdict("overwrite-row-survives-the-reload", labels.some((label) => label.startsWith(overwrite)), { labels: labels.slice(0, 10) });
  else note("overwrite-row-absent-before-the-reload", { before: rowsBefore.slice(0, 12), reading: "the overwrite row was no longer in the document when the reload check ran (a dev reload reset it earlier)" });
  const alternativesAfter = await readAlternatives();
  const currentAfter = alternativesAfter.find(isCurrentAlternative);
  emit({ kind: "alternatives", tag: "after-reload-reattach", rows: alternativesAfter, current: currentAfter?.key ?? null });
  const missingAlternatives = alternativesBefore.filter((row) => !alternativesAfter.some((other) => other.key === row.key));
  if (alternativesBefore.length > 0) verdict("alternatives-survive-the-reload", missingAlternatives.length === 0, { before: alternativesBefore.map((row) => `${row.key}=${row.text.slice(0, 50)}`), after: alternativesAfter.map((row) => `${row.key}=${row.text.slice(0, 50)}`), missing: missingAlternatives.map((row) => row.key) });
  verdict("main-line-listed-after-the-reload", alternativesAfter.some((row) => row.text.includes(COPY[currentLocale].trunk)), { after: alternativesAfter.map((row) => row.text.slice(0, 50)), expected: COPY[currentLocale].trunk });
  if (alternativesBefore.length > 0) verdict("current-alternative-restored-after-the-reload", Boolean(currentBefore && currentAfter && currentAfter.key === currentBefore.key), { before: currentBefore?.key ?? null, after: currentAfter?.key ?? null });
  if (ctx.g4) {
    const g4After = (await findMutationRow(ctx.g4.drag))?.text ?? null;
    atStep(13, () => verdict("warning-row-survives-the-reload", Boolean(g4After?.includes(COPY[currentLocale].warningPartial)), { before: g4Before, after: g4After, expected: COPY[currentLocale].warningPartial, reading: "G4: the warning an edit introduced persists in history through finalize and a folder reload + re-attach" }));
  }
  await shot("reloaded");
  await closePanels();
  const detached = await detachFolder();
  await harvestNotices();
  setExpectedReloads(expectedReloads + 1);
  setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebootedAgain = await waitForBoot("reload-after-detach", 60);
  await installBandTrace();
  await installNoticeTrace();
  const stray = await waitUntil(reconnectBand, (state) => state !== null, 15000, 500);
  verdict("a-forgotten-folder-is-not-offered-after-reload", rebootedAgain && !stray.ok, { detached, band: stray.value, waitedMs: stray.waitedMs, reading: "Detach on the sync card forgets `os.config.local-folders`' binding, so the next load offers nothing" });
};

/** 🔂️ The reload half on a build that offers no folder route (the browser wgpu shell, note
 * `wgpu-browser-build-offers-no-folder`): one plain reload, then whether the head and the rows persisted. */
const plainReloadCheck = async (_ctx: Ctx) => {
  const before = await positions();
  const rowsBefore = documentEntries(await allHistoryRows()).map((row) => row.label.slice(0, 60));
  await closePanels();
  await harvestNotices();
  setExpectedReloads(expectedReloads + 1);
  setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 }).catch((error) => log(`reload failed ${String(error).split("\n")[0]}`));
  const rebooted = await waitForBoot("reload", 60);
  await installBandTrace();
  await installNoticeTrace();
  const after = await waitUntil(positions, (p) => Object.keys(p).length > 0 && movedIds(before, p, 1e-6).length === 0, 30000, 1000);
  const drift = movedIds(before, after.value, 1e-6);
  const rowsAfter = documentEntries(await allHistoryRows()).map((row) => row.label.slice(0, 60));
  verdict("reload-restores-the-edited-document", rebooted && drift.length === 0 && rowsBefore.every((label) => rowsAfter.includes(label)), { rebooted, driftCount: drift.length, drift: drift.slice(0, 6), rowsBefore: rowsBefore.slice(0, 16), rowsAfter: rowsAfter.slice(0, 16), reading: "no folder route on this build: G5 (reload persistence) needs a persisted-local route for the browser wgpu shell (owner S2-W2C)" });
  await shot("reloaded");
  await closePanels();
};

type ReconnectBand = { state: string; role: string | null; live: string | null; label: string | null; message: string; reconnect: boolean; forget: boolean };

/** 📁️ The folder-reconnect band a reload offers for a document bound to a local folder (`🏛️ShellHost/📁️local-folders`):
 * `role=status`, polite, `data-semio-folder-reconnect=offered|reconnecting`, its message and the Reconnect / Forget buttons
 * (wgpu: the same ids through the mirror). */
const reconnectBand = async (): Promise<ReconnectBand | null> => {
  if (renderer === "wgpu") {
    const nodes = await mirror();
    const reconnect = nodes.find((node) => node.key === "s-folder-reconnect");
    if (!reconnect) return null;
    const message = nodes.find((node) => node.key === "s-folder-reconnect-message");
    return { state: reconnect.disabled ? "reconnecting" : "offered", role: "status", live: "polite", label: null, message: message?.label ?? "", reconnect: true, forget: nodes.some((node) => node.key === "s-folder-forget") };
  }
  return evalSafe(() => {
    const el = document.querySelector("[data-semio-folder-reconnect]");
    if (!el) return null;
    return {
      state: el.getAttribute("data-semio-folder-reconnect") ?? "",
      role: el.getAttribute("role"),
      live: el.getAttribute("aria-live"),
      label: el.getAttribute("aria-label"),
      message: document.getElementById("s-folder-reconnect-message")?.textContent ?? "",
      reconnect: Boolean(document.getElementById("s-folder-reconnect")),
      forget: Boolean(document.getElementById("s-folder-forget")),
    };
  }, null as ReconnectBand | null);
};

/** ✂️ Detaches the document from its folder through the sync card (Detach / Trennen; wgpu `framework.sync.detach`). */
const detachFolder = async () => {
  if (renderer === "wgpu") {
    if (!(await openPanelTabIds()).includes("s-sync-status")) await wgpuPress("s-sync-status");
    await sleep(600);
    await pickSyncFolder();
    await sleep(600);
    const via = await wgpuPress("framework.sync.detach");
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    return via;
  }
  if (!(await openPanelTabIds()).includes("s-sync-status")) await page.locator('[id="s-sync-status"]').first().click({ timeout: 4000 }).catch(() => {});
  await sleep(600);
  await pickSyncFolder();
  const detach = page.locator("button").filter({ hasText: /^\s*(detach|trennen)\s*$/i }).first();
  const present = await detach.count().catch(() => 0);
  if (present) await detach.click({ timeout: 4000 }).catch(() => {});
  await sleep(1000);
  await page.keyboard.press("Escape").catch(() => {});
  return present ? "button" : null;
};
//#endregion 🔖️Steps
