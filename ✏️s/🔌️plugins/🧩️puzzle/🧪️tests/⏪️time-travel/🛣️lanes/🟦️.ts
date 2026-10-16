/** 🛣️ Steps 16 to 19: long history, two peers, history steps, configuration and list inputs. */
import { join } from "node:path";
import { mkdirSync, readdirSync } from "node:fs";
import { COPY, dragLabel } from "../🌍️copy/🟦️.ts";
import { currentLocale, renderer, page, stamp, longHistory, OUT, currentStep, mod } from "@semio-tech/framework/time-travel/testing/state";
import { pageHistory, documentEntries, expandEntry, treeWindow, reachWindowEnd, readHistory, scrollHistoryBy, pressAuthored, pressRowAction, allHistoryRows, findMutationRow, openHistory, newestEntrySeq, historyCommandsTotal, isShellChromeRow, newMutation } from "@semio-tech/framework/time-travel/testing/history";
import type { HistoryRow, TreeWindowRead } from "@semio-tech/framework/time-travel/testing/history";
import { evalSafe, sleep, onFreshPages, DESKTOP, waitUntil, shot, use } from "@semio-tech/framework/time-travel/testing/page";
import { inside, closePanels, presenceText, textOfKey } from "@semio-tech/framework/time-travel/testing/chrome";
import { beginEditOf, exitSession, attachFolder, finalizeOverwrite, readAlternatives, isCurrentAlternative, switchAlternative } from "@semio-tech/framework/time-travel/testing/history-session";
import { note, verdict, emit } from "@semio-tech/framework/time-travel/testing/report";
import { typeEditorText, readNumberControl, typeEditorNumber, revealHistory, readEditor } from "@semio-tech/framework/time-travel/testing/editor";
import { armReplay, disarmReplay, EDIT_SETTLE_MS } from "@semio-tech/framework/time-travel/testing/replay-arm";
import type { ArmLog } from "@semio-tech/framework/time-travel/testing/replay-arm";
import type { Ctx } from "../🌄️drag-journey/🟦️.ts";
import { vitals, positions, movedIds, frameBoard, pickNodes, dragBy, offsetOf, cameraOf, placed, near, paneBox, pickCloneSource } from "../🎲️board/🟦️.ts";
import { band, pressBand, shownNoticeCodes, installBandTrace, finalizeDialog, dialogFillName, dialogChoose, shownNotices, prepareChord } from "@semio-tech/framework/time-travel/testing/band";
import { runPaletteCommand, selectNodes, rowsOf } from "../🧮️gaps/🟦️.ts";
import { armReprojection, reprojectionSummary, disarmReprojection, reprojectionShellVerdicts, reprojectionSettled, reprojection } from "@semio-tech/framework/time-travel/testing/reprojection";
import { consoleRows } from "@semio-tech/framework/time-travel/testing/console";
import { mirrorFind, mirror } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";

//#region 🔖️Steps
//#region 🔖️Gaps
//#region 🔖️LongHistory

/** 📈️ Growth rounds a long-history step may add (one palette Set Active Example each, several hundred mutations) when the
 * replay of the current history finishes before a frame shows it. */
const LONG_HISTORY_GROWTH_ROUNDS = 2;

/** 📚️ The example load's transaction of a fresh document — the OLDEST entry row reading Set Active Example, paged from the end
 * of the windowed body, expanded — with the mutation rows the panel lists for it. */
const exampleTransaction = async () => {
  const copy = COPY[currentLocale];
  let entry: HistoryRow | null = null;
  await pageHistory("up", async (rows) => {
    const hits = documentEntries(rows).filter((row) => copy.example.test(row.label)).sort((left, right) => Number(left.key) - Number(right.key));
    entry = hits[0] ?? entry;
    return entry !== null;
  });
  const found = entry as HistoryRow | null;
  if (!found) return { entry: null as HistoryRow | null, mutations: [] as HistoryRow[], window: null as TreeWindowRead | null };
  const expanded = await expandEntry(found.id);
  return { entry: found as HistoryRow | null, mutations: expanded.mutations, window: await treeWindow(`framework.history.entry.${found.key}`) };
};

/** 🔚️ The LAST mutation row of the history row `entry` (gap N1). React scrolls the row's tree window until it materialises its
 * last row (`offset + length = total`) and answers the window. wgpu, whose mirror publishes no window size, pages the body down
 * and keeps, in reading order, every mutation row between `entry` and the next history row — across pages, so a page that
 * starts inside the row (its entry scrolled off) still counts — until `expected` were seen or nothing new shows. */
const lastMutationOf = async (entry: HistoryRow, expected: number) => {
  if (renderer !== "wgpu") {
    const end = await reachWindowEnd(`framework.history.entry.${entry.key}`);
    const rows = (await readHistory()).filter((row) => row.kind === "mutation" && row.parent === entry.id);
    const last = rows[rows.length - 1] ?? null;
    const lastIndex = last ? await evalSafe((id) => {
      const el = document.getElementById(id);
      const index = el?.getAttribute("data-tree-window-row") ?? el?.closest("[data-tree-window-row]")?.getAttribute("data-tree-window-row") ?? el?.querySelector("[data-tree-window-row]")?.getAttribute("data-tree-window-row");
      return index === null || index === undefined ? null : Number(index);
    }, null as number | null, last.id) : null;
    return { total: end?.total ?? null, atEnd: end?.atEnd ?? false, scrolls: end?.scrolls ?? 0, seen: rows.length, last, lastIndex, createNodes: rows.filter((row) => COPY[currentLocale].createNodeRow.test(row.label)).length };
  }
  const seen = new Map<string, HistoryRow>();
  let inside = false;
  let idle = 0;
  for (let page = 0; page < 80 && seen.size < expected && idle < 4; page++) {
    const before = seen.size;
    const rows = await readHistory();
    for (const row of rows) {
      if (row.kind === "entry") inside = row.id === entry.id;
      else if (inside) seen.set(row.key, row);
    }
    idle = seen.size > before ? 0 : idle + 1;
    if (!(await scrollHistoryBy(0.8))) break;
    await sleep(400);
  }
  const rows = [...seen.values()];
  return { total: null as number | null, atEnd: seen.size >= expected, scrolls: 0, seen: rows.length, last: rows[rows.length - 1] ?? null, lastIndex: rows.length ? rows.length - 1 : null, createNodes: rows.filter((row) => COPY[currentLocale].createNodeRow.test(row.label)).length };
};

/** 🖌️ Begins a session on the first editable of `candidates` and drafts a real change: the manifest id (a text input), else
 * a created node's x (+10, a grid-snapped stepper), else Withdraw. */
const draftLongHistoryTarget = async (candidates: HistoryRow[]) => {
  const copy = COPY[currentLocale];
  const ordered = [...candidates.filter((row) => copy.manifest.test(row.label)), ...candidates.filter((row) => copy.createNodeRow.test(row.label)), ...candidates].filter((row, index, all) => all.findIndex((other) => other.key === row.key) === index);
  for (const target of ordered.slice(0, 3)) {
    const begun = await beginEditOf(target.key);
    if (begun.band?.stage !== "editing") {
      note("g9-candidate-did-not-open", { target: target.label, via: begun.via });
      continue;
    }
    if (copy.manifest.test(target.label)) return { target, kind: "manifest", draft: await typeEditorText("newManifestId", `probe-long-history-${stamp.slice(11, 19)}`) as Record<string, unknown> };
    if (copy.createNodeRow.test(target.label)) {
      const x = await readNumberControl("node.x");
      if (x?.valueNow !== null && x?.valueNow !== undefined) return { target, kind: "node-x", draft: { from: x.valueNow, ...(await typeEditorNumber("node.x", Math.round(x.valueNow + 10))) } as Record<string, unknown> };
    }
    return { target, kind: "withdraw", draft: (await pressAuthored("framework.history.editor.withdraw")) as Record<string, unknown> };
  }
  return null;
};

/** ⏱️ The band states an armed replay sampled, summarised: whether a `replaying` frame showed, the largest progress total, the
 * time from the arm to the review and the press the arm made. */
const replaySummary = (armLog: ArmLog | null) => {
  const samples = armLog?.samples ?? [];
  const replaying = samples.filter((sample) => sample.stage === "replaying");
  const totals = replaying.map((sample) => sample.total ?? 0);
  const reviewedAt = samples.find((sample, index) => index > 0 && sample.stage === "reviewing")?.t ?? null;
  const target = armLog?.target ?? [];
  const refusedAt = target.find((row) => row.stage === "replaying" && row.disabled === true) ?? null;
  return { sawReplaying: replaying.length > 0, frames: replaying.length, total: totals.length ? Math.max(...totals) : null, firstReplayingAt: replaying[0]?.t ?? null, reviewedAt, acted: armLog?.acted ?? null, samples: samples.slice(0, 40), target: target.slice(0, 24), disabledWhileReplaying: refusedAt ? { afterMs: refusedAt.t - (replaying[0]?.t ?? refusedAt.t), reason: refusedAt.reason } : null };
};

/** 🗻️ Step 16 — G9 long history in a fresh document: the example load is one transaction of several hundred mutations. Edit its
 * first mutation (a real change), Accept — the replay shows progress over at least `--long-history` mutations and Cancel stops
 * it (review "Replay needed", Replay again offered); Replay again with Edit pressed on another mutation while it replays — the
 * press is refused (disabled, or the `timeTravel.illegal` notice) and the session keeps its target; the review completes; Exit
 * leaves zero trace. Presses land in the first frame that shows the replay (a person reacting to the progress bar). While no
 * frame shows a replay the history grows by one Set Active Example (palette) and the session is retried. */
export const step16 = async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const opened = await open(DESKTOP, "long-history");
    verdict("g9-fresh-document-boots", opened !== null, {});
    if (!opened) return;
    const board = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
    const created = (board.value?.nodes ?? 0) + Math.max(0, board.value?.edges ?? 0);
    const tx = await exampleTransaction();
    const nodes = board.value?.nodes ?? 0;
    const end = tx.entry ? await lastMutationOf(tx.entry, created) : null;
    const reachable = renderer === "wgpu" ? Boolean(end?.atEnd && end.last && end.createNodes >= nodes) : Boolean(tx.window && tx.window.total >= created && end?.atEnd && end.last && end.total !== null && end.lastIndex === end.total - 1);
    verdict("g9-every-mutation-of-the-long-transaction-is-reachable", reachable, { entry: tx.entry?.label ?? null, window: tx.window, end: end ? { ...end, last: end.last?.label ?? null } : null, createdAtLeast: created, firstRows: tx.mutations.map((row) => row.label).slice(0, 6), reading: renderer === "wgpu" ? "acceptance item 1 / N1: the wgpu mirror publishes no window size, so the body is paged until the row's mutations seen reach one create-node per node (counted by label) and nodes + edges in all" : "acceptance item 1 / N1: the example's row is a tree window whose `total` counts every mutation (≥ one create-node per node + one connect-handles per edge); scrolled to its end it materialises row `total − 1` (`data-tree-window-row`) — W1E-6: counts come from `window.total`, never from DOM rows" });
    if (!tx.entry || !tx.mutations.length) return;
    if (end?.last) {
      const lastRow = end.last;
      const via = await pressRowAction(lastRow.id, copy.edit);
      const opened = await waitUntil(band, (b) => b?.stage === "editing" && (b.target ?? b.text).includes(lastRow.label.slice(0, 12)), 20000);
      verdict("n1-the-last-mutation-of-the-long-transaction-opens-for-editing", opened.ok, { via, row: lastRow.label, key: lastRow.key, band: opened.value?.text?.slice(0, 160) });
      if (opened.value) await exitSession();
    }
    const headBefore = await positions();
    let rowsBefore = documentEntries(await allHistoryRows()).map((row) => row.label);
    let session: Awaited<ReturnType<typeof draftLongHistoryTarget>> = null;
    let first: ReturnType<typeof replaySummary> | null = null;
    const rounds: Record<string, unknown>[] = [];
    for (let round = 0; round <= LONG_HISTORY_GROWTH_ROUNDS; round++) {
      const current = await exampleTransaction();
      session = await draftLongHistoryTarget(current.mutations);
      if (!session) break;
      await sleep(900);
      await armReplay({ kind: "cancel" });
      const via = await pressBand("accept");
      const settled = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 240000, 100);
      first = replaySummary(await disarmReplay());
      rounds.push({ round, kind: session.kind, via, review: settled.value?.review ?? null, ...first, target: session.target.label, samples: first.samples.slice(0, 12) });
      const cancelLandedInTime = first.acted?.via === "cancelReplay" && settled.value?.review === "needsReplay";
      if ((first.sawReplaying && cancelLandedInTime) || round === LONG_HISTORY_GROWTH_ROUNDS) break;
      note("g9-replay-too-short-to-cancel", { round, sawReplaying: first.sawReplaying, total: first.total, replayMs: first.firstReplayingAt !== null && first.reviewedAt !== null ? first.reviewedAt - first.firstReplayingAt : null, acted: first.acted, review: settled.value?.review ?? null, reading: "the replay finished before a Cancel pressed in its first rendered frame reached the guest (or before any frame showed it); the history grows by one example load and the step retries" });
      await exitSession();
      const examplesBefore = documentEntries(await allHistoryRows()).filter((row) => copy.example.test(row.label)).length;
      const grown = await runPaletteCommand(copy.exampleQuery, copy.example, "setActiveExample");
      const landed = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => copy.example.test(row.label)).length, (count) => count > examplesBefore, 120000, 2000);
      note("g9-history-grew-by-one-example-load", { round, run: grown, examples: [examplesBefore, landed.value], waitedMs: landed.waitedMs, nodes: (await vitals())?.nodes });
      rowsBefore = documentEntries(await allHistoryRows()).map((row) => row.label);
    }
    emit({ kind: "long-history", rounds });
    if (!session || !first) {
      verdict("g9-a-long-history-mutation-opens-for-editing", false, { rounds, reading: "no listed mutation of the example transaction opened a session" });
      return;
    }
    verdict("g9-the-replay-shows-progress-over-the-long-history", first.sawReplaying && (first.total ?? 0) >= longHistory, { total: first.total, frames: first.frames, firstReplayingAt: first.firstReplayingAt, reviewedAt: first.reviewedAt, minimum: longHistory, rounds: rounds.length, reading: "the band shows `replaying` with progress done/total (React `<progress>`, wgpu `shell.time-travel.status` aria-valuenow/max) over at least --long-history mutations" });
    const afterCancel = await band();
    const cancelLanded = first.acted?.via === "cancelReplay" && afterCancel?.review === "needsReplay";
    verdict("g9-cancel-stops-the-replay", cancelLanded && Boolean(afterCancel?.text.includes(copy.replayCancelled)), { acted: first.acted, review: afterCancel?.review, text: afterCancel?.text?.slice(0, 200), expected: copy.replayCancelled, reading: "design §4: ReplayCancelled → Reviewing with the cancelled fault, review needsReplay (\"Replay needed: later mutations are not checked yet\")" });
    const rerunControl = afterCancel?.controls.find((control) => control.control === "rerun");
    if (cancelLanded) verdict("g9-replay-again-is-offered-after-cancel", rerunControl?.disabled === false, { controls: afterCancel?.controls });
    await shot("g9-cancelled");
    const tx2 = await exampleTransaction();
    const other = tx2.mutations.find((row) => row.key !== session!.target.key) ?? null;
    let prepared = "";
    if (other && !cancelLanded) {
      const again = await beginEditOf(session.target.key);
      const redraft = session.kind === "manifest" ? await typeEditorText("newManifestId", `probe-long-history-${stamp.slice(11, 19)}-b`) : session.kind === "node-x" ? await typeEditorNumber("node.x", Math.round(Number((session.draft as { from?: number }).from ?? 0) + 20)) : { present: false, typed: null };
      await sleep(900);
      prepared = `begin ${again.band?.stage ?? "?"} → redraft ${JSON.stringify(redraft).slice(0, 80)} → `;
    }
    const otherRow = other ? await findMutationRow(other.key) : null;
    if (!otherRow) {
      verdict("g9-edit-during-replay-is-refused", false, { reason: "no second mutation row of the example transaction to press Edit on" });
    } else {
      await armReplay({ kind: "edit", rowId: otherRow.id, source: copy.edit.source, flags: copy.edit.flags });
      const started = cancelLanded ? `rerun via ${await pressBand("rerun")}` : `${prepared}accept via ${await pressBand("accept")}`;
      const completed = await waitUntil(band, (b) => b?.stage === "reviewing" && b.review !== null && b.review !== "needsReplay", 240000, 100);
      const second = replaySummary(await disarmReplay());
      const noticed = second.acted?.disabled === true ? null : await waitUntil(shownNoticeCodes, (shown) => shown.includes("timeTravel.illegal"), 2500, 250);
      const codes = await shownNoticeCodes();
      const editedAfter = (second.samples ?? []).some((sample) => sample.stage === "editing" && sample.t > (second.acted?.t ?? Number.POSITIVE_INFINITY));
      const replayMs = second.firstReplayingAt !== null && second.reviewedAt !== null ? second.reviewedAt - second.firstReplayingAt : null;
      verdict("g9-edit-during-replay-is-refused", second.acted !== null && second.acted.via !== "absent" && (second.acted.disabled === true || codes.includes("timeTravel.illegal")) && !editedAfter, { started, acted: second.acted, codes, noticeWaitedMs: noticed?.waitedMs ?? null, editedAfter, frames: second.frames, replayMs, disabledWhileReplaying: second.disabledWhileReplaying, target: second.target, row: otherRow.label, reading: "design §4: Begin is legal only from Inactive and Reviewing — while Replaying the press is refused (`timeTravel.illegal`, a warning notice) or the Edit action is disabled (N15); a press that neither opens a session nor is refused in words is a silently dropped command" });
      const refusedFrames = second.target.filter((row) => row.stage === "replaying" && row.disabled === true && !row.how.includes("aria-busy"));
      const pendingFrames = second.target.filter((row) => row.disabled === true && row.how.includes("aria-busy"));
      if (second.acted && second.acted.via !== "absent") verdict("n15-edit-is-disabled-while-replaying-naming-why", second.acted.via === "action" && refusedFrames.length > 0 && refusedFrames.every((row) => row.reason.includes(copy.refusalIllegal) && !row.how.includes("disabled attribute")), { judgedOn: `every sampled frame of the replay in which the Edit action reads disabled by the guest (aria-disabled, never the native attribute, not aria-busy) with the reason as its description. The press is a settled one: once the action reads disabled, else ${EDIT_SETTLE_MS} ms after the band turned replaying. Frames that read aria-busy are the host's pending state of the probe's own press and are listed, not judged; no guest-disabled frame at all means Edit stayed enabled for the settle time of a running replay`, acted: second.acted, replayMs, refusedFrames: refusedFrames.length, pendingFrames: pendingFrames.length, disabledWhileReplaying: second.disabledWhileReplaying, target: second.target, expected: copy.refusalIllegal, reading: "gap N15: while a replay runs every Edit row action is disabled (`RowAction::disabled_because`) and names why — an aria-disabled button with its reason as the accessible description, never an enabled control whose press is refused afterwards" });
      const firstReplaying = second.target.find((row) => row.stage === "replaying") ?? null;
      if (second.acted && second.acted.via !== "absent") verdict("n15-edit-reads-disabled-with-its-reason-in-the-first-replaying-frame", firstReplaying !== null && firstReplaying.disabled === true && firstReplaying.reason.includes(copy.refusalIllegal) && !firstReplaying.how.includes("disabled attribute"), { firstReplayingFrame: firstReplaying, pressSettledMs: second.acted.settledMs, pressedDisabled: second.acted.disabled, reasonAtThePress: second.acted.reason, target: second.target.slice(0, 8), expected: copy.refusalIllegal, reading: "the unsettled read: the page's MutationObserver samples the Edit row action in the very DOM commit in which the band first reads `replaying` — the action must read aria-disabled with the reason as its description in that same frame (S5-UI `f3`: row actions are refused from the session stage)" });
      verdict("g9-the-replay-completes-the-review", completed.ok && (completed.value?.review === "ready" || completed.value?.review === "blocked"), { started, review: completed.value?.review, text: completed.value?.text?.slice(0, 200), total: second.total, reviewedAt: second.reviewedAt });
    }
    await shot("g9-reviewed");
    const exited = await exitSession();
    const zero = await waitUntil(positions, (p) => movedIds(headBefore, p, 1e-6).length === 0, 20000, 500);
    const rowsAfter = documentEntries(await allHistoryRows()).map((row) => row.label);
    verdict("g9-exit-leaves-zero-trace", exited.ok && rowsAfter.length === rowsBefore.length && (rounds.length > 1 || zero.ok), { band: exited.value?.stage ?? null, drift: movedIds(headBefore, zero.value, 1e-6).length, rows: [rowsBefore.length, rowsAfter.length], grown: rounds.length > 1, reading: "Exit discards every accepted draft: no new row; the head is unchanged unless the history grew (then only the rows are compared)" });
  });
//#endregion 🔖️LongHistory
//#region 🔖️Peers

/** 🎫️ The mutation row of the newest document row whose label starts with `prefix` (paged, expanded), or null. */
const mutationLabelled = async (prefix: string) => {
  const entry = documentEntries(await allHistoryRows()).filter((row) => row.label.startsWith(prefix)).sort((left, right) => Number(right.key) - Number(left.key))[0];
  if (!entry) return null;
  return (await expandEntry(entry.id)).mutations.find((row) => row.label.startsWith(prefix)) ?? null;
};

/** 🤝️ Step 17 — G10 two peers on one local folder: peer A (a fresh context) attaches a fresh folder and drags one node — the
 * archive lands in the folder; peer B (another fresh context) attaches the same folder and reads A's document; A begins a
 * history edit of its drag; B drags another node, which reaches A over the folder's change stream (`backbone.folder`) as a base
 * move — A's session keeps editing, the remote drag is listed downstream and not applied; A changes dx, Accept replays B's drag
 * too, Finalize overwrite reaches B. Presence (the roster's ⏪ badge, "is editing") travels only through a hub's presence
 * frames, never through a folder binding, so it is recorded as a note. */
export const step17 = async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const folder = join(OUT, `folder-${stamp}-${currentLocale}-peers`);
    mkdirSync(folder, { recursive: true });
    const a = await open(DESKTOP, "peer-a");
    verdict("g10-first-peer-boots", a !== null, {});
    if (!a) return;
    await closePanels();
    const attachedA = await attachFolder(folder);
    if (!attachedA.card) {
      note("g10-no-folder-route-on-this-build", { attachedA, reading: "two peers need a shared persisted route: this build offers no folder card" });
      return;
    }
    verdict("g10-first-peer-attaches-the-shared-folder", attachedA.typed === folder && attachedA.attachButton, { attachedA, folder });
    await closePanels();
    await frameBoard(6);
    const { camera, picked } = await pickNodes(2, [[60, 30], [-50, 40]], [], 90);
    if (picked.length < 2) {
      verdict("g10-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
      return;
    }
    const [n1, n2] = [picked[0].id, picked[1].id];
    await selectNodes([picked[0]]);
    const p0 = await positions();
    await dragBy(picked[0].at, 60 * camera.zoom, 30 * camera.zoom);
    const movedA = await waitUntil(positions, (p) => offsetOf(p0, p, n1)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    const aOff = offsetOf(p0, movedA.value, n1);
    if (!aOff) {
      verdict("g10-first-peer-drag-moves-the-node", false, { n1 });
      return;
    }
    const aRowRead = await waitUntil(() => mutationLabelled(dragLabel(1, aOff[0], aOff[1])), (row) => row !== null, 15000, 750);
    const aRow = aRowRead.value;
    const files = () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => entry.name);
    const written = await waitUntil(async () => files(), (list) => list.length > 0, 30000, 1000);
    verdict("g10-first-peer-edit-is-written-to-the-folder", Boolean(aRow) && written.ok, { aOff, row: aRow?.label ?? null, files: written.value.slice(0, 6) });
    if (!aRow) return;
    const headA = await positions();
    const b = await open(DESKTOP, "peer-b");
    verdict("g10-second-peer-boots", b !== null, {});
    if (!b) return;
    await closePanels();
    const attachedB = await attachFolder(folder);
    const shared = await waitUntil(positions, (p) => Object.keys(p).length > 0 && movedIds(headA, p, 1e-6).length === 0, 45000, 1000);
    const bSeesARead = await waitUntil(() => mutationLabelled(dragLabel(1, aOff[0], aOff[1])), (row) => row !== null, 20000, 1000);
    const bSeesA = bSeesARead.value;
    verdict("g10-second-peer-opens-the-shared-document", attachedB.typed === folder && shared.ok && Boolean(bSeesA), { rowWaitedMs: bSeesARead.waitedMs, attachedB, drift: movedIds(headA, shared.value, 1e-6).slice(0, 6), driftCount: movedIds(headA, shared.value, 1e-6).length, row: bSeesA?.label ?? null, reading: "attaching a folder that holds an archive reads it (`documentArchiveReplaced`): B now shows A's document and A's drag row" });
    use(a);
    await installBandTrace();
    const begun = await beginEditOf(aRow.key);
    verdict("g10-first-peer-begins-a-history-edit", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
    if (begun.band?.stage !== "editing") return;
    await armReprojection("observe");
    note("g10-presence-travels-only-through-a-hub", { a: await presenceText(), reading: "the roster's ⏪ badge and the \"is editing\" notes come from presence frames (presence bit 13), which only a hub socket carries (`🏪️store/👷️worker` `presenceAuthority` is set from hub frames); a folder binding carries none — the live two-peer presence case needs a hub-backed space serve (`🚀️local-hub` + `?space=`)" });
    use(b);
    await closePanels();
    await frameBoard(4);
    const n2At = (await rowsOf([n2]))[0];
    if (!n2At) {
      verdict("g10-second-peer-drags-another-node", false, { n2 });
      return;
    }
    await selectNodes([n2At]);
    const beforeB = await positions();
    const zoomB = cameraOf(await vitals()).zoom;
    await dragBy(n2At.at, -50 * zoomB, 40 * zoomB);
    const movedB = await waitUntil(positions, (p) => offsetOf(beforeB, p, n2)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    const bOff = offsetOf(beforeB, movedB.value, n2);
    verdict("g10-second-peer-drags-another-node", Boolean(bOff), { n2, bOff });
    if (!bOff) return;
    await openHistory();
    await armReprojection("observe");
    use(a);
    const bLabel = dragLabel(1, bOff[0], bOff[1]);
    const arrived = await waitUntil(() => mutationLabelled(bLabel), (row) => row !== null, 45000, 1500);
    verdict("g10-a-remote-edit-arrives-while-editing", arrived.ok, { expected: bLabel, row: arrived.value?.text?.slice(0, 160) ?? null, waitedMs: arrived.waitedMs, reading: "B's write reaches A over the folder's change stream; A's store takes the new history while the session edits" });
    const during = await band();
    verdict("g10-the-session-survives-the-base-move", during?.stage === "editing" && Boolean((during.target ?? during.text).includes(aRow.label)), { stage: during?.stage, target: during?.target, text: during?.text?.slice(0, 160), reading: "design §7: remote ingests keep arriving while frozen → BaseMoved; in Editing the preview is re-shown, the draft stays" });
    const whileEditing = reprojectionSummary(await disarmReprojection());
    if (whileEditing.seen) reprojectionShellVerdicts(whileEditing, "-during-a-session");
    else note("g10-the-base-move-replayed-within-one-refresh", { whileEditing });
    const previewA = await positions();
    verdict("g10-the-remote-edit-stays-downstream-and-unapplied-while-editing", placed(previewA, n2, p0[n2], 0, 0, 0.05) && Boolean(arrived.value?.text.includes(copy.pending)), { n2: previewA[n2], n2Before: p0[n2], row: arrived.value?.text?.slice(0, 160) ?? null, expected: copy.pending });
    const dxNew = Math.round(aOff[0] + 20);
    const typed = await typeEditorNumber("dx", dxNew);
    await pressBand("accept");
    const review = await waitUntil(band, (b2) => b2?.stage === "reviewing" && Boolean(b2.review), 60000, 100);
    const replayed = await waitUntil(positions, (p) => placed(p, n1, p0[n1], dxNew, aOff[1], 0.05) && placed(p, n2, p0[n2], bOff[0], bOff[1], 0.05), 20000);
    verdict("g10-accept-replays-the-remote-edit-too", review.value?.review === "ready" && replayed.ok, { typed, review: review.value?.review, n1: replayed.value[n1], n2: replayed.value[n2], expected: { n1: [p0[n1][0] + dxNew, p0[n1][1] + aOff[1]], n2: [p0[n2][0] + bOff[0], p0[n2][1] + bOff[1]] } });
    if (review.value?.review !== "ready") {
      await exitSession();
      return;
    }
    const finalized = await finalizeOverwrite();
    verdict("g10-finalize-closes-the-session", finalized.closed, finalized);
    use(b);
    const converged = await waitUntil(positions, (p) => placed(p, n1, p0[n1], dxNew, aOff[1], 0.05), 45000, 1000);
    const adopted = reprojectionSummary(await disarmReprojection());
    const overwriteRead = await waitUntil(async () => documentEntries(await allHistoryRows()).find((row) => row.label.startsWith(copy.row("edit", null, 1))) ?? null, (row) => row !== null, 20000, 1000);
    const overwriteRow = overwriteRead.value;
    verdict("g10-the-second-peer-sees-the-finalized-edit", converged.ok && Boolean(overwriteRow), { n1: converged.value[n1], n2: converged.value[n2], row: overwriteRow?.label ?? null, waitedMs: converged.waitedMs });
    if (adopted.seen) {
      verdict("g10-the-second-peer-shows-what-it-replays", adopted.kinds.length > 0, { adopted, reading: "N17 / §20.8: the peer names the change it replays (`HistoryPatch.reprojection {kind: remote|load}`) with its progress in words while it adopts the finalized edit" });
      reprojectionShellVerdicts(adopted, "-peer");
    } else note("g10-the-second-peer-adopted-within-one-refresh", { adopted, reading: "the folder change stream replaced the archive and the peer adopted it before its body re-rendered (progress refreshes ≥ 100 ms)" });
    await shot("g10-peers");
    const stateOf = async () => {
      const now = await positions();
      return { n1: now[n1] ?? null, n2: now[n2] ?? null, rows: documentEntries(await allHistoryRows()).map((row) => row.label.slice(0, 60)), band: (await band())?.stage ?? null };
    };
    const finalB = await stateOf();
    use(a);
    const finalA = await stateOf();
    await shot("g10-peer-a-final");
    const agree = JSON.stringify([finalA.n1, finalA.n2]) === JSON.stringify([finalB.n1, finalB.n2]) && JSON.stringify(finalA.rows) === JSON.stringify(finalB.rows);
    verdict("g10-both-peers-converge-after-the-finalize", agree, { a: finalA, b: finalB, reading: "after the first peer finalized, both peers hold the same document (the two nodes) and list the same document rows" });
    const survived = Boolean(finalA.n2 && finalB.n2 && near(finalA.n2[0], p0[n2][0] + bOff[0], 0.05) && near(finalA.n2[1], p0[n2][1] + bOff[1], 0.05) && near(finalB.n2[0], p0[n2][0] + bOff[0], 0.05) && near(finalB.n2[1], p0[n2][1] + bOff[1], 0.05));
    note("g10-the-second-peers-drag-after-the-finalize", { survived, n2: { a: finalA.n2, b: finalB.n2, withTheDrag: [p0[n2][0] + bOff[0], p0[n2][1] + bOff[1]], withoutIt: p0[n2] }, reading: "design §22.25: a remote tail edit is adopted at ingest and listed \"Not applied while editing\"; until the folder route merges a fetched pair (S5-LOAD / S5-STORE) a read-back is last-writer-wins, so whether the second peer's drag survives the first peer's finalize is recorded, not judged" });
    note("g10-final-states-of-both-peers", {
      a: finalA,
      b: finalB,
      agree,
      expected: { n1: [p0[n1][0] + dxNew, p0[n1][1] + aOff[1]], n2: [p0[n2][0] + bOff[0], p0[n2][1] + bOff[1]] },
      folderRequests: consoleRows.filter((row) => row.locale === currentLocale && row.step === currentStep && row.type === "backbone").map((row) => `${row.t}s ${/HTTP \d+ \w+/.exec(row.text)?.[0] ?? row.text.slice(0, 20)}`),
      reading: "what each peer holds at the end (the first peer edited history while the second peer dragged), and every folder request of the step with its run-clock time: a PUT is a peer writing the archive, the GETs after it are the peers reading it",
    });
  });
//#endregion 🔖️Peers
//#region 🔖️HistorySteps

/** 🧗️ Step 18 — N17 history steps and the §20.8 stepped document load, in fresh documents. Page A, bound to a fresh folder (its
 * archive lands there), grows by a second example load and finalizes an edit of its first example's first mutation as a new
 * alternative; switching back to the main line is a deferred local step the body shows as "History step" / "Replaying
 * history: d of t mutations" — Undo meanwhile is refused (`history.replaying`), Cancel replay drops the step with zero trace
 * (the alternative stays current) — and switching again completes it; one drag then marks A's head. Page B attaches A's folder:
 * the archive loads stepwise ("Document load" / "Loading document: d of t"), Undo meanwhile is refused (`document.loading`) and
 * Cancel replay keeps B's own document; uncancelled, B shows A's head. Presses land the frame the status shows (a person
 * reacting to it); a step or a load that finishes before any frame shows it is recorded as such. */
export const step18 = async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const folder = join(OUT, `folder-${stamp}-${currentLocale}-steps`);
    mkdirSync(folder, { recursive: true });
    const a = await open(DESKTOP, "history-step");
    verdict("n17-fresh-document-boots", a !== null, {});
    if (!a) return;
    await closePanels();
    const attached = await attachFolder(folder);
    await closePanels();
    const examples = async () => documentEntries(await allHistoryRows()).filter((row) => copy.example.test(row.label)).length;
    const examplesBefore = await examples();
    const grown = await runPaletteCommand(copy.exampleQuery, copy.example, "setActiveExample");
    const landed = await waitUntil(examples, (count) => count > examplesBefore, 120000, 2000);
    verdict("n17-history-grows-by-a-second-example-load", landed.ok, { run: grown, examples: [examplesBefore, landed.value], waitedMs: landed.waitedMs });
    await closePanels();
    const tx = await exampleTransaction();
    const session = tx.mutations.length ? await draftLongHistoryTarget(tx.mutations) : null;
    verdict("n17-a-mutation-of-the-first-example-opens-with-a-draft", session !== null, { entry: tx.entry?.label ?? null, kind: session?.kind ?? null, target: session?.target.label ?? null });
    if (!session) return;
    await sleep(900);
    await pressBand("accept");
    const review = await waitUntil(band, (b) => b?.stage === "reviewing" && b.review !== null && b.review !== "needsReplay", 240000, 200);
    if (review.value?.review !== "ready") {
      verdict("n17-the-edit-reviews-ready", false, { review: review.value?.review ?? null, text: review.value?.text?.slice(0, 200) });
      await exitSession();
      return;
    }
    const name = `probe steps ${currentLocale} ${stamp.slice(11, 19)}`;
    await pressBand("finalize");
    await waitUntil(finalizeDialog, (d) => d !== null, 15000);
    const typedName = await dialogFillName(name);
    await dialogChoose("submit");
    const closed = await waitUntil(band, (b) => b === null, 60000);
    const listed = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(name)) && rows.some((row) => row.text.includes(copy.trunk)), 20000, 1000);
    verdict("n17-the-edit-is-kept-as-a-new-alternative", closed.ok && listed.ok, { typedName, alternatives: listed.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${isCurrentAlternative(row) ? " [current]" : ""}`) });
    if (!listed.ok) return;
    const onAlternative = await positions();
    const rowsOnAlternative = documentEntries(await allHistoryRows()).map((row) => row.label);
    const trunkRow = async () => (await readAlternatives()).find((row) => row.text.includes(copy.trunk)) ?? null;
    const currentIsTrunk = async () => Boolean((await readAlternatives()).find(isCurrentAlternative)?.text.includes(copy.trunk));
    const trunk = await trunkRow();
    await armReprojection("undo-then-cancel");
    const via = trunk ? await switchAlternative(trunk) : "absent";
    await sleep(800);
    await reprojectionSettled(180000);
    const step = reprojectionSummary(await disarmReprojection());
    emit({ kind: "history-step", via, step });
    if (!step.seen) note("n17-the-history-step-adopted-within-one-refresh", { via, step, reading: "the switch's replay finished before the body re-rendered (progress refreshes ≥ 100 ms), so neither the refusal nor Cancel could be pressed while it ran" });
    else {
      verdict("n17-a-history-step-shows-its-progress", step.kinds.includes("step") && (step.total ?? 0) > 0, { step, reading: "N17: an alternative switch away from the applied tail is a deferred local step — \"History step\" / \"Replaying history: d of t mutations\" in the shell status and the body's `framework.history.reprojection` section (`HistoryPatch.reprojection {kind: step}`)" });
      reprojectionShellVerdicts(step, "-step");
      const refused = (await shownNotices()).find((row) => row.code === "history.replaying");
      if (step.undo) verdict("n17-a-second-history-step-is-refused-while-one-replays", Boolean(refused?.text.includes(copy.noticeReplaying)), { undo: step.undo, notice: refused ?? null, expected: copy.noticeReplaying, reading: "`VcsError::HistoryReplaying`: every further history step (here Undo) is refused while one replays, with the localized notice" });
      if (step.cancel) {
        const back = await waitUntil(positions, (p) => movedIds(onAlternative, p, 1e-6).length === 0, 15000, 500);
        const rowsAfter = documentEntries(await allHistoryRows()).map((row) => row.label);
        verdict("n17-cancel-replay-drops-the-history-step-with-zero-trace", step.cancel.via !== "absent" && !(await currentIsTrunk()) && back.ok && rowsAfter.length === rowsOnAlternative.length, { cancel: step.cancel, drift: movedIds(onAlternative, back.value, 1e-6).slice(0, 6), rows: [rowsOnAlternative.length, rowsAfter.length], reading: "`discard_local_step`: Cancel replay drops a local step with zero trace — the alternative stays current and nothing is recorded" });
      }
    }
    if (!(await currentIsTrunk())) {
      const again = await trunkRow();
      if (again) await switchAlternative(again);
      await reprojectionSettled(180000);
    }
    const done = await waitUntil(currentIsTrunk, (ok) => ok, 60000, 1000);
    verdict("n17-the-history-step-completes-on-the-main-line", done.ok, { alternatives: (await readAlternatives()).map((row) => `${row.text.slice(0, 50)}${isCurrentAlternative(row) ? " [current]" : ""}`) });
    await closePanels();
    await frameBoard(4);
    const { camera, picked } = await pickNodes(1, [[60, 30]], [], 40);
    if (picked[0]) {
      await selectNodes(picked);
      const before = await positions();
      await dragBy(picked[0].at, 60 * camera.zoom, 30 * camera.zoom);
      await waitUntil(positions, (p) => offsetOf(before, p, picked[0]!.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    }
    const headA = await positions();
    await closePanels();
    if (!attached.card) {
      note("n17-no-folder-route-for-the-stepped-load", { attached, reading: "this build offers no folder card (browser wgpu), so the stepped archive load is reached only through a hub or a file import" });
      return;
    }
    const written = await waitUntil(async () => readdirSync(folder, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).length, (count) => count > 0, 30000, 1000);
    const b = await open(DESKTOP, "document-load");
    verdict("n17-second-page-boots", b !== null, { archiveFiles: written.value });
    if (!b) return;
    const own = await positions();
    await closePanels();
    await armReprojection("undo-then-cancel");
    const attachedB = await attachFolder(folder, () => openHistory());
    await sleep(800);
    const settledLoad = await reprojectionSettled(90000);
    const load = reprojectionSummary(await disarmReprojection());
    emit({ kind: "document-load", load, attachedB, settled: settledLoad });
    const shown = await positions();
    const isOwn = movedIds(own, shown, 1e-6).length === 0;
    const isA = movedIds(headA, shown, 1e-6).length === 0;
    if (!load.seen) {
      note("n17-the-document-load-finished-within-one-refresh", { load, reading: "the archive load finished before the body re-rendered (progress refreshes ≥ 100 ms)" });
      verdict("n17-attaching-the-folder-loads-the-shared-document", isA, { drift: movedIds(headA, shown, 1e-6).slice(0, 6), attachedB });
      return;
    }
    verdict("n17-the-stepped-load-shows-its-progress", load.kinds.includes("load") && (load.total ?? 0) > 0, { load, reading: "§20.8: a whole-document load is stepped on the history wire (`HistoryPatch.reprojection {kind: load}`), never one `LoadDocument` — \"Document load\" / \"Loading document: d of t\" in the shell status and the body" });
    reprojectionShellVerdicts(load, "-load");
    const refused = (await shownNotices()).find((row) => row.code === "document.loading");
    if (load.undo) verdict("n17-a-command-during-the-load-is-refused-naming-the-load", Boolean(refused?.text.includes(copy.noticeLoading)), { undo: load.undo, notice: refused ?? null, expected: copy.noticeLoading, reading: "N17: any command while a whole-document load is live is refused with the localized `document.loading` notice" });
    if (load.cancel) verdict("n17-cancel-replay-keeps-the-previous-document", isOwn, { cancel: load.cancel, drift: movedIds(own, shown, 1e-6).slice(0, 6), reading: "`historyEditCancelReplay` without a session cancels a live load: the previous document stays, zero trace" });
    else verdict("n17-attaching-the-folder-loads-the-shared-document", isA, { drift: movedIds(headA, shown, 1e-6).slice(0, 6), attachedB });
    if (load.cancel && load.cancel.via !== "absent") {
      await openHistory();
      const stale = await reprojection();
      verdict("n17-a-cancelled-load-clears-its-status", settledLoad.ok && stale === null, { settled: settledLoad, status: stale, reading: "after Cancel replay nothing loads any more, so the History body's `framework.history.reprojection` section (and the shell status) must go away; a status that stays reads as a load that never ends" });
      await shot("load-cancelled");
      const noticesBefore = (await shownNotices()).length;
      await closePanels();
      await frameBoard(6);
      const { camera, picked } = await pickNodes(1, [[30, 20]], [], 90);
      if (picked.length) {
        await selectNodes([picked[0]]);
        const beforeDrag = await positions();
        await dragBy(picked[0].at, 30 * camera.zoom, 20 * camera.zoom);
        const dragged = await waitUntil(positions, (now) => offsetOf(beforeDrag, now, picked[0].id)?.some((value) => Math.abs(value) > 0.5) === true, 10000);
        verdict("n17-the-document-is-editable-after-a-cancelled-load", dragged.ok, { node: picked[0].id, offset: offsetOf(beforeDrag, dragged.value, picked[0].id), notices: (await shownNotices()).slice(noticesBefore), reading: "a cancelled load leaves the previous document as it was — a person goes on working in it; a drag that is refused (`document.loading`) means the cancelled load still holds the document" });
      } else note("n17-no-node-to-drag-after-the-cancelled-load", { picked: 0 });
    }
  });
//#endregion 🔖️HistorySteps
//#region 🔖️ConfigAndLists

/** 🛤️ Scrolls the tree window `windowKey` slice by slice from its start until the row `authored` is materialised and in view —
 * how a person scrolls a long editor to one of its rows (wgpu: the body pages until the mirror projects it). */
export const revealWindowRow = async (windowKey: string, authored: string) => {
  for (let attempt = 0; attempt < 40; attempt++) {
    if (renderer === "wgpu" ? Boolean(mirrorFind(await mirror(), authored)) : await revealHistory(authored)) return true;
    const moved = renderer === "wgpu"
      ? await scrollHistoryBy(0.5)
      : await evalSafe(
          (arg) => {
            const el = Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).find((node) => {
              const key = node.getAttribute("data-tree-window-key") ?? "";
              return key === arg.key || key.endsWith(`/${arg.key}`);
            });
            const rows = Array.from(el?.children ?? []).filter((child) => !child.hasAttribute("data-tree-window-spacer"));
            const anchor = (arg.first ? el?.firstElementChild : rows[rows.length - 1]) as HTMLElement | null | undefined;
            anchor?.scrollIntoView({ block: "start" });
            return Boolean(anchor);
          },
          false,
          { key: windowKey, first: attempt === 0 },
        );
    if (!moved) return false;
    await sleep(500);
  }
  return false;
};

/** 📊️ The item count a list input's row names (`Items: n` / `Elemente: n`), scrolling its row into view only when it is not
 * materialised; null while it cannot be reached. */
const listItemCount = async (pointerKey: string) => {
  const pattern = new RegExp(COPY[currentLocale].itemCount(0).replace("0", "(\\d+)"));
  const read = async () => {
    const match = pattern.exec(await textOfKey(`${pointerKey}.row`));
    return match ? Number(match[1]) : null;
  };
  return (await read()) ?? ((await revealWindowRow("framework.history.editor.inputs", `${pointerKey}.row`)) ? read() : null);
};

/** ⚙️ Step 19 — on the main document: L4 (§20.13) — camera zoom and pan add no history row at all; N1 — one drag adds exactly
 * one row, the Commands window `total` grows by one, and §19.1 — the row reads its intent leaf; L4 again — Undo after a camera
 * move takes back the drag and leaves the camera, Redo re-applies it; N2 — Edit of a duplicated node's `create-node`: its
 * handles list names its item count, Add item drafts one more item, Remove item drops it, Exit leaves zero trace. */
export const step19 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  const cameraNow = async () => cameraOf(await vitals());
  const sameCamera = (left: { x: number; y: number; zoom: number }, right: { x: number; y: number; zoom: number }) => near(left.x, right.x, 1e-3) && near(left.y, right.y, 1e-3) && near(left.zoom, right.zoom, 1e-4);
  await closePanels();
  await frameBoard(6);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  const totalBefore = await historyCommandsTotal();
  await closePanels();
  const cameraBefore = await cameraNow();
  const box = await paneBox();
  const centre = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
  const byGesture: Record<string, string[]> = {};
  let newestSeen = newestBefore;
  const rowsOfGesture = async (gesture: string) => {
    const rows = (await allHistoryRows()).filter((row) => row.kind === "entry" && Number(row.key) > newestSeen);
    byGesture[gesture] = rows.filter((row) => !isShellChromeRow(row)).map((row) => `${row.key}=${row.label}`);
    newestSeen = Math.max(newestSeen, newestEntrySeq(rows));
    await closePanels();
    await page.mouse.move(centre.x, centre.y);
  };
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.wheel(0, -240);
  await sleep(900);
  await rowsOfGesture("wheel zoom in");
  await page.mouse.down({ button: "middle" });
  for (let index = 1; index <= 10; index++) await page.mouse.move(centre.x + 6 * index, centre.y + 4 * index);
  await page.mouse.up({ button: "middle" });
  await sleep(900);
  await rowsOfGesture("middle-button pan");
  await page.mouse.wheel(0, 120);
  await sleep(1500);
  const cameraAfter = await cameraNow();
  const rowsAfter = await allHistoryRows();
  const totalAfter = await historyCommandsTotal();
  byGesture["wheel zoom out"] = rowsAfter.filter((row) => row.kind === "entry" && Number(row.key) > newestSeen && !isShellChromeRow(row)).map((row) => `${row.key}=${row.label}`);
  const added = rowsAfter.filter((row) => row.kind === "entry" && Number(row.key) > newestBefore);
  const foreign = added.filter((row) => !isShellChromeRow(row));
  verdict("l4-camera-moves-are-never-history-rows", !sameCamera(cameraBefore, cameraAfter) && foreign.length === 0 && (totalBefore === null || (totalAfter !== null && totalAfter - totalBefore === added.length)), { cameraBefore, cameraAfter, rowsFromTheCameraMoves: foreign.map((row) => `${row.key}=${row.label}${row.expandable ? " (document)" : ""}`), byGesture, probePanelRows: added.length - foreign.length, totals: [totalBefore, totalAfter], reading: "L4 / §20.13: camera zoom and pan stream through the window transient and commit config edits, which the history projection never lists — not as a row, not as a mutation (the law observes ALL rows; the panel toggles the probe itself makes to read the body are shell chrome rows and are told apart by their icon)" });
  await closePanels();
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { picked } = await pickNodes(1, [[50, 30]], exclude, 40);
  if (!picked[0]) {
    verdict("l4-one-clickable-node", false, {});
    return;
  }
  const node = picked[0].id;
  const newestBeforeDrag = newestEntrySeq(await allHistoryRows());
  const totalBeforeDrag = await historyCommandsTotal();
  const [target] = await rowsOf([node]);
  await selectNodes(target ? [target] : picked);
  const p0 = await positions();
  const zoom = (await cameraNow()).zoom;
  const [grab] = await rowsOf([node]);
  await dragBy((grab ?? picked[0]).at, 50 * zoom, 30 * zoom);
  const moved = await waitUntil(positions, (p) => offsetOf(p0, p, node)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const offset = offsetOf(p0, moved.value, node);
  if (!offset) {
    verdict("l4-the-drag-moves-the-node", false, { node });
    return;
  }
  const row = await newMutation(newestBeforeDrag, (entry) => entry.label.startsWith(dragLabel(1, offset[0], offset[1])));
  const totalAfterDrag = await historyCommandsTotal();
  if (totalBeforeDrag === null) note("n1-commands-window-total-not-published", { reading: "no window size is published (React `data-tree-window-total`, wgpu `aria-setsize` of the entry rows)" });
  else {
    const newer = (await allHistoryRows()).filter((entry) => entry.kind === "entry" && Number(entry.key) > newestBeforeDrag);
    const totalNow = await historyCommandsTotal();
    const documents = newer.filter((entry) => entry.expandable);
    verdict("n1-one-drag-adds-exactly-one-row-to-the-window-total", Boolean(row.entry) && totalBeforeDrag !== null && totalNow !== null && totalNow - totalBeforeDrag === newer.length && documents.length === 1, { totals: [totalBeforeDrag, totalAfterDrag, totalNow], row: row.entry?.label ?? null, newRows: newer.map((entry) => `${entry.key}=${entry.label}${entry.expandable ? " (document)" : ""}`), documentRows: documents.length, reading: "N1: the Commands window `total` grows by exactly the rows newer than the read before the drag — the drag's ONE document row plus command rows without mutations (the selection click's board events, the probe's own panel toggles)" });
  }
  verdict("s19-the-drag-row-reads-its-intent-leaf", Boolean(row.entry && row.mutation && row.entry.label === row.mutation.label), { row: row.entry?.label ?? null, intent: row.mutation?.label ?? null, expected: dragLabel(1, offset[0], offset[1]) });
  await closePanels();
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.wheel(0, -180);
  await sleep(1500);
  const zoomed = await cameraNow();
  await prepareChord();
  await page.keyboard.press(`${mod}+z`).catch(() => {});
  const undone = await waitUntil(async () => placed(await positions(), node, p0[node], 0, 0), (ok) => ok, 12000, 400);
  await sleep(600);
  const cameraUndone = await cameraNow();
  verdict("l4-undo-takes-back-the-drag-not-the-camera", undone.ok && sameCamera(zoomed, cameraUndone), { node, at: (await positions())[node], before: p0[node], zoomed, cameraUndone, reading: "L4: undo of the artifact never steps over config edits — one Undo after a camera move takes back the drag, the camera stays" });
  await prepareChord();
  await page.keyboard.press(`${mod}+Shift+z`).catch(() => {});
  const redone = await waitUntil(async () => placed(await positions(), node, p0[node], offset[0], offset[1]), (ok) => ok, 12000, 400);
  verdict("l4-redo-re-applies-the-drag", redone.ok, { node, at: (await positions())[node], expected: [p0[node]![0] + offset[0], p0[node]![1] + offset[1]] });
  await closePanels();
  await frameBoard(4);
  const spot = await pickCloneSource([...exclude, node]);
  if (!spot) {
    verdict("n2-clone-source-with-free-space", false, {});
    return;
  }
  const idsBefore = Object.keys(await positions());
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const newestBeforeClone = newestEntrySeq(await allHistoryRows());
  const [source] = await rowsOf([spot.row.id]);
  await selectNodes(source ? [source] : [spot.row]);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  const createRow = clone ? await newMutation(newestBeforeClone, (entry) => entry.label.startsWith(copy.createNode(clone))) : { entry: null, mutation: null, waitedMs: 0 };
  verdict("n2-duplicate-adds-a-create-node-row", Boolean(createRow.mutation), { clone, mutation: createRow.mutation?.label ?? null });
  if (!createRow.mutation) return;
  const head = await positions();
  const rowsPre = documentEntries(await allHistoryRows()).map((entry) => entry.label);
  const begun = await beginEditOf(createRow.mutation.key);
  verdict("n2-edit-opens-the-create-node-session", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const list = "framework.history.editor.input.node.handles";
  await readEditor();
  const count = await listItemCount(list);
  const handlesBefore = (await vitals())?.handles ?? null;
  await revealWindowRow("framework.history.editor.inputs", `${list}.row`);
  const add = await pressAuthored(`${list}.add`, false);
  const grew = await waitUntil(() => listItemCount(list), (now) => count !== null && now === count + 1, 10000, 400);
  verdict("n2-add-item-drafts-one-more-list-item", count !== null && add.present && add.disabled !== true && grew.ok, { add, count: [count, grew.value], handles: [handlesBefore, (await vitals())?.handles ?? null], reading: "N2: a list input's row names its item count and holds Add item (`historyEditInput{path: …/handles/-, edit: insert}`), which drafts the item's default" });
  if (count !== null && grew.ok) {
    await revealWindowRow("framework.history.editor.inputs", `${list}.${count}.row`);
    const remove = await pressAuthored(`${list}.${count}.remove`, false);
    const shrank = await waitUntil(() => listItemCount(list), (now) => now === count, 10000, 400);
    verdict("n2-remove-item-drops-the-drafted-item", remove.present && remove.disabled !== true && shrank.ok, { remove, count: [grew.value, shrank.value], reading: "N2: each list item's first row holds Remove item (`edit: remove` at its pointer)" });
  }
  note("n2-long-option-rows-not-offered-by-puzzle-2d", { reading: "a select or segmented input with MORE than `UI_FIXED_LIST_ITEMS` (32) options becomes windowed option rows (`….option.<i>`, `role=treeitem` with `aria-selected=true|false`, the chosen row `data-selected`); no puzzle 2d leaf offers that many, so the contract is proven by the React `🧺️ListInputs` and wgpu `the_guest_editor_offers_list_and_chip_edits_within_their_bounds` laws (S4-UI), not live" });
  const exited = await exitSession();
  const restored = await waitUntil(positions, (p) => movedIds(head, p, 1e-6).length === 0, 15000, 500);
  const rowsSettled = await waitUntil(async () => documentEntries(await allHistoryRows()).map((entry) => entry.label), (rows) => rows.length === rowsPre.length, 8000, 800);
  const rowsPost = rowsSettled.value;
  verdict("n2-exit-leaves-zero-trace", exited.ok && restored.ok && rowsPost.length === rowsPre.length, { band: exited.value?.stage ?? null, drift: movedIds(head, restored.value, 1e-6).slice(0, 6), rows: [rowsPre.length, rowsPost.length], rowsWaitedMs: rowsSettled.waitedMs });
  await shot("config-and-lists");
  await closePanels();
};
//#endregion 🔖️ConfigAndLists
//#endregion 🔖️Gaps
//#endregion 🔖️Steps
