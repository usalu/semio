/** 🌄️ Steps 1 to 5: boot, drag, edit, accept and finalize of one drag mutation. */
import { join } from "node:path";
import { mkdirSync } from "node:fs";
import { vitals, positions, movedIds, frameBoard, pickNodes, selectionIds, cameraOf, dragBy, offsetOf, near, toScreen, paneBox, placed } from "../🎲️board/🟦️.ts";
import type { Positions } from "../🎲️board/🟦️.ts";
import { openHistory, findMutationRow, pressRowAction, historyInventory, newestEntrySeq, allHistoryRows, historyCommandsTotal, documentEntries, expandEntry, readHistory, rowActionState } from "@semio-tech/framework/time-travel/testing/history";
import { band, prepareChord, installBandTrace, bandOverlapVerdict, bandOverlaps, clearBandTrace, pressBand, bandTrace, finalizeDialog, dialogChoose } from "@semio-tech/framework/time-travel/testing/band";
import type { Band } from "@semio-tech/framework/time-travel/testing/band";
import { log, verdict, note, emit } from "@semio-tech/framework/time-travel/testing/report";
import { COPY, dragLabel } from "../🌍️copy/🟦️.ts";
import { currentLocale, renderer, folderAt, OUT, stamp, page } from "@semio-tech/framework/time-travel/testing/state";
import { waitUntil, shot, evalSafe } from "@semio-tech/framework/time-travel/testing/page";
import { editorArrived, attachFolder } from "@semio-tech/framework/time-travel/testing/history-session";
import { wgpuPressedTabsWithoutPanel, presentKey, textOfKey, closePanels, openPanelTabIds, HISTORY_TAB, indicators, presenceText } from "@semio-tech/framework/time-travel/testing/chrome";
import { mirror, WGPU_BAND_CONTROLS } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";
import { focusRead, focusIsOnAnEditorInput, editorRevealed, readEditor, typeEditorNumber, keyEditorNumber, editor } from "@semio-tech/framework/time-travel/testing/editor";
import { ariaVerdict } from "@semio-tech/framework/time-travel/testing/aria-oracle";

//#region 🔖️Steps

export type Ctx = {
  a?: string;
  b?: string;
  c?: string;
  p0?: Positions;
  p1?: Positions;
  pc?: Positions;
  dx?: number;
  dy?: number;
  cOffset?: [number, number];
  dragEntry?: string;
  dragMutation?: string;
  cMutation?: string;
  afterOverwrite?: Positions;
  dyEdited?: number;
  alternative?: string;
  folder?: string;
  noFolder?: boolean;
  g4?: { drag: string; p: string; q: string };
};

/** 🏹️ Finds the drag's `drag-selection` mutation row by its stable mutation id (paging the windowed body, expanding its
 * entry) and presses Edit on it; answers the band once `editing`. */
export const beginDragEdit = async (ctx: Ctx) => {
  await openHistory();
  const mutation = ctx.dragMutation ? await findMutationRow(ctx.dragMutation) : null;
  if (!mutation) return { entry: null as string | null, via: "absent", band: null as Band | null, waitedMs: 0, editorWaitedMs: 0 };
  if (mutation.parent !== ctx.dragEntry) log(`drag row moved ${ctx.dragEntry} → ${mutation.parent}`);
  const via = await pressRowAction(mutation.id, COPY[currentLocale].edit);
  const settled = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  const editorWaitedMs = settled.ok ? await editorArrived(mutation.label) : 0;
  return { entry: mutation.parent, via, band: settled.value, waitedMs: settled.waitedMs, editorWaitedMs };
};

/** 🌄️ Step 1 — the board is parsed and populated, and the history panel speaks the locale under test. */
export const step1 = async (ctx: Ctx) => {
  const parsed = await waitUntil(vitals, (v) => v?.parsed === "true", 60000);
  verdict("fixture-parsed", parsed.ok, { parsed: parsed.value?.parsed, waitedMs: parsed.waitedMs });
  const populated = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
  const p = await positions();
  verdict("board-has-nodes", (populated.value?.nodes ?? 0) > 0 && Object.keys(p).length > 0, { nodes: populated.value?.nodes, edges: populated.value?.edges, positions: Object.keys(p).length, camera: populated.value?.camera });
  if (renderer === "wgpu") {
    const pressed = await wgpuPressedTabsWithoutPanel();
    const windows = Array.from(new Set((await mirror()).map((node) => node.window)));
    await shot("wgpu-boot-chrome");
    note("wgpu-pressed-panel-tabs-without-a-mirrored-panel", { pressedWithoutPanel: pressed, mirroredWindows: windows, reading: "coordinator decision (2026-10-05, F15): `aria-pressed=true` on a dock tab names the active tab of its dock group, as on React and in the shared fixture — not a shown panel; recorded, not judged. The probe reads open panels from the mirrored body windows" });
  }
  const opened = await openHistory();
  const commands = await presentKey("framework.history.commands");
  const commandsText = await textOfKey("framework.history.commands");
  verdict("history-panel-speaks-the-locale", opened && commandsText.toLowerCase().includes(COPY[currentLocale].commands.toLowerCase()), { opened, commands, commandsText: commandsText.slice(0, 80), expected: COPY[currentLocale].commands, lang: await evalSafe(() => navigator.language, "") });
  emit({ kind: "inventory", tag: "boot-history", rows: await historyInventory() });
  await shot("boot");
  await closePanels();
  if (folderAt === 1) {
    ctx.folder = join(OUT, `folder-${stamp}-${currentLocale}`);
    mkdirSync(ctx.folder, { recursive: true });
    const before = await positions();
    const attached = await attachFolder(ctx.folder);
    const after = await positions();
    verdict("local-folder-attach-keeps-the-document", attached.typed === ctx.folder && attached.attachButton && movedIds(before, after, 1e-6).length === 0, { attached, drift: movedIds(before, after, 1e-6).length, reading: "the document is bound to a local folder (sync card → Folder → Attach, `persistedLocalOnly`) before any edit, so every later edit is persisted; step 5's reload check re-attaches it" });
    if (renderer === "wgpu" && !attached.card) {
      ctx.noFolder = true;
      note("wgpu-browser-build-offers-no-folder", { attached, reading: "S2-W2C decision (`📓️w2-c-report.md` S2.1): the browser wgpu build serves no folder transport, so it never attaches, remembers or offers a folder; the reload check then measures a plain reload" });
    }
    await closePanels();
  }
};

/** ✋️ Step 2 — select two nodes, drag them by (+80,+40) world units, one new row; then a downstream drag of a third node. */
export const step2 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  const totalBefore = await historyCommandsTotal();
  await closePanels();
  const { camera, picked } = await pickNodes(3, [[80, 40], [-60, 30]], [], 50);
  if (picked.length < 3) {
    verdict("three-clickable-nodes", false, { picked: picked.map((row) => row.id), camera });
    return;
  }
  const [a, b, c] = picked;
  ctx.a = a.id;
  ctx.b = b.id;
  ctx.c = c.id;
  log(`picked a=${a.id}@${JSON.stringify(a.at)} b=${b.id}@${JSON.stringify(b.at)} c=${c.id}@${JSON.stringify(c.at)} zoom=${camera.zoom}`);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await page.mouse.click(a.at.x, a.at.y);
  const first = await waitUntil(vitals, (v) => selectionIds(v).includes(a.id), 15000);
  await page.keyboard.down("Shift");
  await page.mouse.click(b.at.x, b.at.y);
  await page.keyboard.up("Shift");
  const both = await waitUntil(vitals, (v) => selectionIds(v).includes(a.id) && selectionIds(v).includes(b.id), 15000);
  verdict("two-nodes-selected", first.ok && both.ok && selectionIds(both.value).length === 2, { selection: selectionIds(both.value), transform: both.value?.transform?.slice(0, 200) });
  ctx.p0 = await positions();
  const zoom = cameraOf(await vitals()).zoom;
  await dragBy(a.at, 80 * zoom, 40 * zoom);
  const moved = await waitUntil(positions, (p) => offsetOf(ctx.p0!, p, a.id)?.some((v) => Math.abs(v) > 0.5) === true && offsetOf(ctx.p0!, p, b.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  ctx.p1 = moved.value;
  const offA = offsetOf(ctx.p0, ctx.p1, a.id);
  const offB = offsetOf(ctx.p0, ctx.p1, b.id);
  ctx.dx = offA?.[0] ?? 0;
  ctx.dy = offA?.[1] ?? 0;
  const others = movedIds(ctx.p0, ctx.p1).filter((id) => id !== a.id && id !== b.id);
  verdict("both-nodes-moved-by-one-common-offset", moved.ok && Boolean(offA && offB && near(offA[0], offB[0], 1e-6) && near(offA[1], offB[1], 1e-6)), { offA, offB, zoom, waitedMs: moved.waitedMs });
  verdict("offset-is-80-40-world-units", Boolean(offA && near(offA[0], 80, 0.005) && near(offA[1], 40, 0.005)), { offA, screenDelta: [80 * zoom, 40 * zoom], note: "screen delta = world offset × zoom; a pixel-quantised pointer makes this inexact" });
  verdict("no-other-node-moved", others.length === 0, { others: others.slice(0, 8) });
  const expected = dragLabel(2, ctx.dx, ctx.dy);
  const grew = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 1, 30000, 1000);
  const added = grew.value;
  verdict("exactly-one-new-history-row", added.length === 1, { added: added.map((row) => `${row.id}=${row.label}`), waitedMs: grew.waitedMs });
  const totalAfter = await historyCommandsTotal();
  if (totalBefore === null) note("n1-commands-window-total-not-published", { reading: "no window size is published (React `data-tree-window-total`, wgpu `aria-setsize` of the entry rows); the new row is proven by its sequence above" });
  else {
    const newer = (await allHistoryRows()).filter((entry) => entry.kind === "entry" && Number(entry.key) > newestBefore);
    const totalNow = await historyCommandsTotal();
    verdict("n1-commands-window-total-grows-by-one-per-edit", totalBefore !== null && totalNow !== null && totalNow - totalBefore === newer.length && added.length === 1, { totalBefore, totalAfter, totalNow, newRows: newer.map((entry) => `${entry.key}=${entry.label}${entry.expandable ? " (document)" : ""}`), documentRows: added.length, reading: "N1 / W1E-6: the Commands section is a tree window whose `total` counts every row, materialised or not — the total grows by exactly the rows that are newer than the read before the drag (the drag's one document row plus the chrome commands of the gesture and of the probe's own panel toggles, which are command rows without mutations)" });
  }
  const row = added.find((entry) => entry.label.startsWith(expected) || entry.text.includes(expected)) ?? added[0];
  verdict("row-labelled-from-the-drag-mutation", Boolean(row && (row.label === expected || row.label.startsWith(expected))), { expected, label: row?.label, text: row?.text?.slice(0, 160) });
  if (row) {
    ctx.dragEntry = row.id;
    const expanded = await expandEntry(row.id);
    const mutation = expanded.mutations.find((entry) => entry.label.startsWith(expected)) ?? expanded.mutations[0];
    ctx.dragMutation = mutation?.key;
    verdict("row-expands-to-its-drag-selection-mutation", Boolean(mutation && mutation.label.startsWith(expected)), { state: expanded.state, mutations: expanded.mutations.map((entry) => `${entry.key}=${entry.label}`).slice(0, 8) });
    verdict("s19-the-transaction-row-reads-its-intent-leaf", Boolean(mutation && row.label === mutation.label && !expanded.mutations.some((entry) => entry.key !== mutation.key && row.label === entry.label)), { row: row.label, intent: mutation?.label, children: expanded.mutations.map((entry) => entry.label).slice(0, 8), reading: "design §19.1: a tool transaction's row is labelled by its declared intent leaf (the drag), never by a structural support leaf that rides with it (proximity `connect-handles`); a one-leaf transaction holds it trivially" });
  }
  await shot("dragged");
  await closePanels();
  const beforeC = await positions();
  const cAt = toScreen(beforeC[c.id], cameraOf(await vitals()), await paneBox());
  await dragBy(cAt, -60 * zoom, 30 * zoom);
  const cMoved = await waitUntil(positions, (p) => offsetOf(beforeC, p, c.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  ctx.pc = cMoved.value;
  ctx.cOffset = offsetOf(beforeC, ctx.pc, c.id) ?? [0, 0];
  const cOthers = movedIds(beforeC, ctx.pc).filter((id) => id !== c.id);
  verdict("downstream-drag-moves-only-c", cMoved.ok && cOthers.length === 0, { cOffset: ctx.cOffset, others: cOthers.slice(0, 6) });
  const cRows = await waitUntil(async () => documentEntries(await allHistoryRows()).filter((entry) => Number(entry.key) > newestBefore && entry.id !== ctx.dragEntry), (rows) => rows.length >= 1, 30000, 1000);
  const cRow = cRows.value[0];
  if (cRow) {
    const expanded = await expandEntry(cRow.id);
    ctx.cMutation = expanded.mutations[0]?.key;
    verdict("downstream-drag-is-its-own-row", cRows.value.length === 1 && cRow.label.startsWith(dragLabel(1, ctx.cOffset[0], ctx.cOffset[1])), { label: cRow.label, expected: dragLabel(1, ctx.cOffset[0], ctx.cOffset[1]), mutation: ctx.cMutation });
  } else verdict("downstream-drag-is-its-own-row", false, { rows: cRows.value.length });
  await closePanels();
};

/** 🎬️ Step 3 — Edit the drag: band `editing`, preview = state before + draft, downstream pending, dx/dy steppers and targets. */
export const step3 = async (ctx: Ctx) => {
  if (!ctx.dragEntry || !ctx.dragMutation || !ctx.p0 || !ctx.a || !ctx.b || !ctx.c || !ctx.pc) {
    verdict("precondition-step-2", false, { ctx: Object.keys(ctx) });
    return;
  }
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  const b = begun.band;
  verdict("edit-opens-the-band-in-editing", b?.stage === "editing", { via: begun.via, stage: b?.stage, waitedMs: begun.waitedMs, text: b?.text?.slice(0, 200) });
  verdict("band-is-a-polite-status-region", b?.role === "status" && b?.live === "polite", { role: b?.role, live: b?.live });
  verdict("band-names-stage-and-target", Boolean(b && b.text.includes(COPY[currentLocale].stageEditing) && (b.target ?? "").includes(dragLabel(2, ctx.dx!, ctx.dy!))), { text: b?.text?.slice(0, 200), target: b?.target });
  verdict("band-offers-accept-discard-exit", JSON.stringify(b?.controls.map((c) => c.control)) === JSON.stringify(["accept", "discard", "exit"]), { controls: b?.controls });
  const controlIds = (b?.controls ?? []).map((c) => [c.control, c.id ?? "", WGPU_BAND_CONTROLS.find(([name]) => name === c.control)?.[1] ?? ""] as const);
  verdict("band-controls-carry-their-control-ids", controlIds.length > 0 && controlIds.every(([, id, expected]) => id !== "" && id === expected), { controls: controlIds.map(([control, id, expected]) => ({ control, id, expected })), reading: "every band control carries the corpus `controlId` as its id on both renderers (React DOM id = wgpu mirror key, `TimeTravelControl::control_id`)" });
  await bandOverlapVerdict("band-overlaps-no-other-text");
  if (renderer !== "wgpu") {
    const seat = await bandOverlaps();
    verdict("band-sits-in-the-subfooter-bands", Boolean(seat?.inBottomBands && seat.inSubfooter), { seat, reading: "bands live in `[data-slot=layout-subfooter] > [data-semio-bottom-bands]` — the bands row under the footer (in flow, own surface; S5-UI r3)" });
  }
  const focus = await waitUntil(focusRead, focusIsOnAnEditorInput, 6000, 150);
  verdict("focus-moves-to-the-editor", focus.ok, { focus: focus.value, waitedMs: focus.waitedMs, reading: "design §16 / G13: a draft that starts puts keyboard focus on its first input (`timeTravelTransitionV1` → `editor`; React `timeTravelFocusElementV1`, wgpu `resolve_time_travel_focus`), never on its row" });
  const revealed = await waitUntil(editorRevealed, (state) => state.revealed, 6000, 200);
  const tabs = await openPanelTabIds();
  verdict("history-panel-reveals-on-session-start", tabs.includes(HISTORY_TAB) && revealed.ok, { tabs, revealed: revealed.value, waitedMs: revealed.waitedMs, reading: "the edge into a session opens the History panel with the band and the editor's first input in view (no probe scroll; React: inside the viewport and uncovered, wgpu: projected). In one tab a session can only begin from that panel's own Edit row action — a closed-panel begin needs the agent/MCP gateway, which this serve does not run" });
  const indicator = await indicators();
  verdict("windows-wear-the-time-travel-indicator", indicator.length > 0, { indicator: indicator.slice(0, 4) });
  const roster = await presenceText();
  verdict("presence-roster-shows-no-editing-peer", roster === null || !/⏪|is editing|bearbeitet .* im Verlauf|bearbeitet den Verlauf/.test(roster), { roster, reading: "one tab, no peer: the ⏪ badge and the \"is editing\" notes (`🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-peers`) must stay absent; a live second peer needs a presence transport (hub), which this serve does not run" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], ctx.dx!, ctx.dy!) && placed(p, ctx.c!, ctx.p0![ctx.c!], 0, 0), 15000);
  verdict("preview-is-state-before-target-plus-draft", placed(preview.value, ctx.a, ctx.p0[ctx.a], ctx.dx!, ctx.dy!) && placed(preview.value, ctx.b, ctx.p0[ctx.b], ctx.dx!, ctx.dy!), {
    a: preview.value[ctx.a],
    aBefore: ctx.p0[ctx.a],
    draft: [ctx.dx, ctx.dy],
    design: "§4: the preview is the document as of the edited mutation with the DRAFT applied (Begin drafts the original input), so the dragged nodes read pre-drag + draft",
  });
  verdict("downstream-not-applied-while-editing", placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { c: preview.value[ctx.c], cBeforeItsDrag: ctx.p0[ctx.c], cHead: ctx.pc[ctx.c], waitedMs: preview.waitedMs });
  const pendingRows = await waitUntil(readHistory, (rows) => rows.some((row) => row.kind === "mutation" && row.key === ctx.cMutation && row.text.includes(COPY[currentLocale].pending)), 10000);
  const cRow = pendingRows.value.find((row) => row.kind === "mutation" && row.key === ctx.cMutation);
  verdict("downstream-row-reads-not-applied", Boolean(cRow?.text.includes(COPY[currentLocale].pending)), { cRow: cRow?.text?.slice(0, 160), expected: COPY[currentLocale].pending });
  const e = await waitUntil(readEditor, (value) => Boolean(value?.numbers.dx && value?.numbers.dy && value?.targets), 15000);
  const ed = e.value;
  const steppers = renderer === "wgpu" ? Boolean(ed?.numbers.dx?.stepper && ed?.numbers.dy?.stepper) : Boolean(ed?.numbers.dx?.stepper && ed?.numbers.dy?.stepper && ed.numbers.dx.step === "1" && ed.numbers.dy.step === "1" && ed.numbers.dx.plus && ed.numbers.dx.minus);
  verdict("editor-shows-dx-dy-steppers-with-grid-snap-step", steppers, { dx: ed?.numbers.dx, dy: ed?.numbers.dy, note: renderer === "wgpu" ? "the mirror announces a spinbutton; its snap step is proven by ArrowUp/ArrowDown in step 4" : "snapSource {config: gridFactor} resolves to the grid factor (default 1) as the stepper step" });
  verdict("editor-dx-dy-read-the-original-input", Boolean(ed?.numbers.dx && ed?.numbers.dy && near(Number(ed.numbers.dx.value), ctx.dx!, 0.005) && near(Number(ed.numbers.dy.value), ctx.dy!, 0.005)), { dx: ed?.numbers.dx?.value, dy: ed?.numbers.dy?.value, expected: [ctx.dx, ctx.dy] });
  verdict("editor-shows-the-targets-reference-list", Boolean(ed?.targets && ed.targets.chips.length === 2 && ed.targets.chips.every((chip) => chip.trim().length > 0) && ed.targets.useSelection), { targets: ed?.targets, ids: [ctx.a, ctx.b], inputs: ed?.inputs, reading: "the reference row is a tree window: \"Use selection\" (`….targets.useSelection`), then one chip row per referenced id (`….targets.chip.<i>.row`) named by the entity's label (`ArtifactApp::entity_label`)" });
  await ariaVerdict("aria-band-editor-and-history-have-no-structural-findings", "session");
  emit({ kind: "inventory", tag: "editing", rows: await historyInventory(), editor: ed, band: b });
  await shot("editing");
};

/** 🎼️ Step 4 — dx → 120 through the stepper (keyboard), Accept, replay, review `ready`, head at +120 with downstream. */
export const step4 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || (await band())?.stage !== "editing") {
    verdict("precondition-editing-session", false, { band: (await band())?.stage ?? null });
    return;
  }
  const typed = await typeEditorNumber("dx", 120);
  verdict("dx-stepper-control-reachable", typed.present && typed.typed === "120", { typed, authored: "framework.history.editor.input.dx" });
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!) && placed(p, ctx.b!, ctx.p0![ctx.b!], 120, ctx.dy!), 20000);
  verdict("dx-120-updates-the-preview", preview.ok && placed(preview.value, ctx.c, ctx.p0[ctx.c], 0, 0), { typed, a: preview.value[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dy!], c: preview.value[ctx.c], waitedMs: preview.waitedMs });
  if (typed.present) {
    await keyEditorNumber("dx", "ArrowUp");
    const up = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 121, ctx.dy!), 15000);
    await keyEditorNumber("dx", "ArrowDown");
    const down = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!), 15000);
    verdict("arrow-keys-step-dx-by-the-snap-step", up.ok && down.ok, { up: up.value[ctx.a], down: down.value[ctx.a], dxField: (await editor())?.numbers.dx?.value });
  }
  const accepted = (await editor())?.numbers.dx?.value ?? null;
  if (typed.present && ctx.cMutation) {
    const copy = COPY[currentLocale];
    const cRow = await findMutationRow(ctx.cMutation);
    const state = cRow ? await rowActionState(cRow.id, copy.edit) : null;
    verdict("n15-edit-is-refused-while-a-changed-draft-is-open-naming-why", Boolean(state?.present && state.disabled === true && state.native !== true && state.reason.includes(copy.refusalBlocked)), { row: cRow?.label ?? null, state, expected: copy.refusalBlocked, reading: "gap N15: `TimeTravelSession::begin_refusal` is Blocked while the open draft differs from its original, so Edit on every other mutation row is disabled and names why (`RowAction::disabled_because` → React `aria-describedby`, wgpu the mirror button's description)" });
    verdict("n15-the-refusal-reason-is-visible", Boolean(state?.shown?.includes(copy.refusalBlocked)), { shown: state?.shown ?? null, reveal: state?.reveal ?? null, reason: state?.reason ?? null, reading: renderer === "wgpu" ? "wgpu: the mirror button's description (its painted tooltip is outside the DOM)" : "W1E-1: `[data-slot=row-action-reason][data-revealed]` shows the reason after a hover (≈ 400 ms), on keyboard focus and on a press, hides it on leave, Escape and blur, and is always the button's `aria-describedby`" });
  }
  await clearBandTrace();
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  const trace = await bandTrace();
  const stages = [...new Set(trace.map((row) => row.key.split("|")[0]))];
  const progress = trace.filter((row) => row.key.split("|")[2]);
  verdict("accept-replays-then-reviews-ready", review.value?.stage === "reviewing" && review.value.review === "ready", { via, dxField: accepted, band: review.value?.text?.slice(0, 200), review: review.value?.review, waitedMs: review.waitedMs });
  const bandFocus = await waitUntil(focusRead, (focus) => focus.isBand, 6000, 150);
  verdict("focus-moves-to-the-band-on-review", bandFocus.ok, { focus: bandFocus.value, waitedMs: bandFocus.waitedMs, reading: "G13: a replay or a review puts focus on the band (React `[data-semio-time-travel]` tabIndex −1, wgpu `shell.time-travel.status`)" });
  if (stages.includes("replaying") || progress.length) verdict("replay-stage-rendered", true, { trace: trace.slice(0, 20) });
  else note("replay-stage-not-rendered", { trace: trace.slice(0, 20), reading: "the two-mutation replay finished inside one reactor turn, so the band went editing → reviewing without a replaying frame (progress is throttled ≥ 100 ms / ≥ 5 %)" });
  verdict("review-line-reads-ready", Boolean(review.value?.text.includes(COPY[currentLocale].reviewReady)), { text: review.value?.text?.slice(0, 200) });
  const finalize = review.value?.controls.find((c) => c.control === "finalize");
  verdict("finalize-enabled-when-ready", finalize?.disabled === false, { controls: review.value?.controls });
  const head = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, ctx.dy!) && placed(p, ctx.c!, ctx.pc![ctx.c!], 0, 0), 15000);
  verdict("reviewed-head-at-plus-120-with-downstream-reapplied", head.ok && placed(head.value, ctx.b, ctx.p0[ctx.b], 120, ctx.dy!), { a: head.value[ctx.a], b: head.value[ctx.b], c: head.value[ctx.c], cHead: ctx.pc?.[ctx.c] });
  await shot("reviewing");
};

/** 🏁️ Step 5 — Finalize → dialog → Overwrite → band gone, overwrite row, head persists across a reload. */
export const step5 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || (await band())?.review !== "ready") {
    verdict("precondition-ready-review", false, { band: await band() });
    return;
  }
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  const d = dialog.value;
  const copy = COPY[currentLocale];
  verdict("finalize-opens-the-dialog", Boolean(d && d.title.includes(copy.dialogTitle)), { title: d?.title, waitedMs: dialog.waitedMs });
  const promptFocus = await waitUntil(focusRead, (focus) => focus.inDialog, 6000, 150);
  verdict("focus-moves-into-the-finalize-prompt", promptFocus.ok, { focus: promptFocus.value, waitedMs: promptFocus.waitedMs, reading: "G13: choosing puts focus on the prompt's first control (its Alternative name field)" });
  verdict("dialog-offers-destructive-overwrite", Boolean(d?.overwrite && d.overwrite.text.includes(copy.overwrite) && (renderer === "wgpu" || d.overwrite.destructive === "true") && (d.overwrite.description ?? "").includes(copy.overwriteDescription)), { overwrite: d?.overwrite, expectedDescription: copy.overwriteDescription, reading: renderer === "wgpu" ? "the mirror carries no tone; the destructive choice is told by its description" : "data-destructive + the choice description" });
  verdict("dialog-offers-new-alternative-with-a-name-field", Boolean(d?.submit && d.submit.text.includes(copy.newAlternative) && d.name && d.name.value === copy.defaultName), { submit: d?.submit, name: d?.name });
  if (renderer === "wgpu") note("band-while-the-dialog-is-open", { band: (await band())?.stage ?? null, reading: "the wgpu chrome projects only the modal dialog's nodes while it is open (`chrome_accessibility_nodes`), so the band's status is not announced then" });
  else verdict("band-reads-choosing-while-the-dialog-is-open", (await band())?.stage === "choosing", { band: (await band())?.stage });
  const behind = renderer === "wgpu" ? null : (await readHistory()).find((row) => row.kind === "mutation" && row.key === ctx.cMutation);
  if (behind) {
    const refused = await waitUntil(() => rowActionState(behind.id, copy.edit, false), (read) => read.present && read.disabled === true, 5000, 200);
    const state = refused.value;
    verdict("n15-edit-is-refused-while-choosing-naming-why", state.present && state.disabled === true && state.native !== true && state.reason.includes(copy.refusalIllegal), { row: behind.label, state, waitedMs: refused.waitedMs, expected: copy.refusalIllegal, reading: "gap N15: Begin is illegal while the finalize prompt is open (Choosing), so every Edit behind it is disabled and names why" });
  } else note("n15-edit-while-choosing-not-observable", { reading: renderer === "wgpu" ? "the wgpu chrome projects only the modal prompt's nodes while it is open" : "the downstream mutation row is not materialised behind the prompt" });
  await ariaVerdict("aria-finalize-prompt-has-no-structural-findings", "prompt");
  await shot("dialog");
  await dialogChoose("overwrite");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict("overwrite-closes-the-session", gone.ok && (await finalizeDialog()) === null, { waitedMs: gone.waitedMs, band: gone.value?.stage ?? null });
  const expected = copy.row("edit", null, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict("overwrite-row-appears", rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  ctx.afterOverwrite = await positions();
  verdict("head-keeps-plus-120-after-overwrite", placed(ctx.afterOverwrite, ctx.a, ctx.p0[ctx.a], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.b, ctx.p0[ctx.b], 120, ctx.dy!) && placed(ctx.afterOverwrite, ctx.c, ctx.pc![ctx.c], 0, 0), { a: ctx.afterOverwrite[ctx.a], c: ctx.afterOverwrite[ctx.c] });
  await shot("overwritten");
  note("reload-check-deferred", { reason: "the reload is taken after step 8 and before step 9 (attributed to step 5), so steps 6–8 keep the document it would otherwise replace and step 9 digests its console" });
};
//#endregion 🔖️Steps
