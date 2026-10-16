/** 🕹️ Steps 20 to 23: row actions, the goal sentence, handles and the pointer journeys. */
import type { Ctx } from "../🌄️drag-journey/🟦️.ts";
import { COPY, dragLabel } from "../🌍️copy/🟦️.ts";
import { currentLocale, page, mod, renderer } from "@semio-tech/framework/time-travel/testing/state";
import type { Point } from "@semio-tech/framework/time-travel/testing/state";
import { closePanels, presentKey, inside } from "@semio-tech/framework/time-travel/testing/chrome";
import { frameBoard, pickCloneSource, vitals, positions, selectionIds, cameraOf, toScreen, paneBox, dragBy, offsetOf, movedIds, pickNodes, near, placed, OVERVIEW, layout } from "../🎲️board/🟦️.ts";
import { verdict, note, log } from "@semio-tech/framework/time-travel/testing/report";
import { newestEntrySeq, allHistoryRows, documentEntries, expandEntry, rowActionState, pressRowAction, findMutationRow, newMutation, pressAuthored, openHistory, readHistory, scrollHistory } from "@semio-tech/framework/time-travel/testing/history";
import type { HistoryRow } from "@semio-tech/framework/time-travel/testing/history";
import { waitUntil, shot, evalSafe, sleep } from "@semio-tech/framework/time-travel/testing/page";
import { prepareChord, installBandTrace, band, pressBand, shownNoticeCodes } from "@semio-tech/framework/time-travel/testing/band";
import { exitSession, beginEditOf, finalizeOverwrite, documentRowsSince } from "@semio-tech/framework/time-travel/testing/history-session";
import { selectNodes, rowsOf, runPaletteCommand } from "../🧮️gaps/🟦️.ts";
import { readEditor, keyEditorNumber, pressUseSelection, typeEditorNumber, editor } from "@semio-tech/framework/time-travel/testing/editor";
import { mirror, mirrorFind, mirrorAwait } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";
import { revealWindowRow } from "../🛣️lanes/🟦️.ts";

//#region 🔖️Steps

/** 🧿 Step 20 — row actions (design §22.1, §22.20), on the main document: a node is duplicated and its clone dragged (two rows:
 * `create-node`, then its drag). Every mutation row offers Edit and Withdraw. Withdraw on the create-node's ROW opens a session
 * with a withdrawn draft (no editor) → the replay leaves the drag blocking ("Target missing"); the row now holds an accepted
 * draft and offers Restore in place of Withdraw. Withdraw on the BLOCKING drag's row — a row action needs no input schema, so it
 * works for a mutation whose inputs cannot be edited too; the row's Edit state is recorded — makes the review `ready` with two
 * accepted changes. Restore on the drag's row drops that draft (blocked again, one accepted change). Exit leaves zero trace. */
export const step20 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const spot = await pickCloneSource(exclude);
  if (!spot) {
    verdict("r22-clone-source-with-free-space", false, { reason: "no node whose clone and drag destination are free" });
    return;
  }
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(spot.row.at.x, spot.row.at.y);
  await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === spot.row.id, 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  if (!clone) {
    verdict("r22-duplicate-adds-one-clone", false, { nodes: [nodesBefore, (await vitals())?.nodes ?? null] });
    return;
  }
  await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  await dragBy({ x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 }, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const cloneOffset = offsetOf(beforeDrag, dragged.value, clone);
  const head = dragged.value;
  const added = (await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 2, 30000, 1000)).value;
  let createRow: HistoryRow | undefined;
  let dragRow: HistoryRow | undefined;
  for (const entry of added) {
    const expanded = await expandEntry(entry.id);
    createRow ??= expanded.mutations.find((row) => row.label.startsWith(copy.createNode(clone)));
    if (cloneOffset) dragRow ??= expanded.mutations.find((row) => row.label.startsWith(dragLabel(1, cloneOffset[0], cloneOffset[1])));
  }
  verdict("r22-duplicate-and-drag-rows-carry-their-mutations", Boolean(createRow && dragRow), { added: added.map((row) => row.label), create: createRow?.label, drag: dragRow?.label });
  if (!createRow || !dragRow) return;
  const rowsPre = documentEntries(await allHistoryRows()).map((row) => row.label);
  const offered = { createEdit: await rowActionState(createRow.id, copy.edit, false), createWithdraw: await rowActionState(createRow.id, copy.withdraw, false), dragEdit: await rowActionState(dragRow.id, copy.edit, false), dragWithdraw: await rowActionState(dragRow.id, copy.withdraw, false) };
  verdict("r22-every-mutation-row-offers-edit-and-withdraw", Object.values(offered).every((state) => state.present) && offered.createWithdraw.disabled !== true && offered.dragWithdraw.disabled !== true, { offered, reading: "design §22.1: a mutation row always carries `[Edit, Withdraw]` (a withdrawn row `[Edit]`, a row with an accepted draft `[Edit, Restore]`); an action that cannot run is disabled with its reason, never absent" });
  await installBandTrace();
  const viaWithdraw = await pressRowAction(createRow.id, copy.withdraw);
  let afterWithdraw = await waitUntil(band, (b) => b !== null && (b.stage === "editing" || (b.stage === "reviewing" && Boolean(b.review))), 20000, 100);
  const stageAfterPress = afterWithdraw.value?.stage ?? null;
  if (afterWithdraw.value?.stage === "editing") {
    await pressBand("accept");
    afterWithdraw = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  }
  verdict("r22-withdraw-from-the-row-opens-a-session-with-a-withdrawn-draft", viaWithdraw === "action" && afterWithdraw.value?.stage === "reviewing" && afterWithdraw.value.review === "blocked", { via: viaWithdraw, stageAfterPress, review: afterWithdraw.value?.review ?? null, text: afterWithdraw.value?.text?.slice(0, 200), reading: "Withdraw pressed on a mutation row (`historyEditWithdraw {mutationId}`) opens or stacks a session whose draft withdraws that mutation — no editor; here the clone's drag then fails downstream, so the review blocks" });
  const gone = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("r22-the-review-shows-the-document-without-the-withdrawn-node", gone.ok, { hasClone: Boolean(gone.value[clone]) });
  const createNow = await findMutationRow(createRow.key);
  const restoreOffer = createNow ? await rowActionState(createNow.id, copy.restore, false) : null;
  const withdrawOffer = createNow ? await rowActionState(createNow.id, copy.withdraw, false) : null;
  verdict("r22-a-row-with-an-accepted-draft-offers-restore-in-place-of-withdraw", Boolean(restoreOffer?.present && restoreOffer.disabled !== true && withdrawOffer && !withdrawOffer.present), { row: createNow?.text?.slice(0, 160) ?? null, restore: restoreOffer, withdraw: withdrawOffer });
  const failing = await findMutationRow(dragRow.key);
  const failingEdit = failing ? await rowActionState(failing.id, copy.edit, false) : null;
  const viaBlocking = failing ? await pressRowAction(failing.id, copy.withdraw) : "absent";
  let ready = await waitUntil(band, (b) => b !== null && (b.stage === "editing" || (b.stage === "reviewing" && b.review === "ready")), 30000, 100);
  if (ready.value?.stage === "editing") {
    await pressBand("accept");
    ready = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  }
  verdict("r22-withdrawing-the-blocking-mutation-from-its-row-makes-the-review-ready", viaBlocking === "action" && ready.value?.review === "ready" && Boolean(ready.value.text.includes(copy.accepted(2))), { via: viaBlocking, blockingRow: failing?.text?.slice(0, 160) ?? null, editOfTheBlockingRow: failingEdit ? { disabled: failingEdit.disabled, reason: failingEdit.reason } : null, review: ready.value?.review ?? null, text: ready.value?.text?.slice(0, 200), expected: copy.accepted(2), reading: "design §22.1: Withdraw needs no input schema, so a blocking mutation is withdrawn from its row whether or not its inputs can be edited (the row's Edit state is recorded: disabled with \"" + copy.refusalNotEditable + "\" for a non-editable one)" });
  const dragNow = await findMutationRow(dragRow.key);
  const viaRestore = dragNow ? await pressRowAction(dragNow.id, copy.restore) : "absent";
  const restored = await waitUntil(band, (b) => b?.stage === "reviewing" && b.review === "blocked" && b.text.includes(copy.accepted(1)), 30000, 100);
  verdict("r22-restore-from-the-row-drops-the-accepted-draft", viaRestore === "action" && restored.ok, { via: viaRestore, review: restored.value?.review ?? null, text: restored.value?.text?.slice(0, 200), expected: `${copy.reviewBlocked} … ${copy.accepted(1)}`, reading: "Restore (`historyEditRestore {mutationId}`) drops that mutation's accepted draft: the drag is applied again, fails again, and one accepted change (the withdrawn create-node) remains" });
  await shot("row-actions");
  const exited = await exitSession();
  const back = await waitUntil(positions, (p) => movedIds(head, p, 1e-6).length === 0, 20000, 500);
  const rowsSettled = await waitUntil(async () => documentEntries(await allHistoryRows()).map((row) => row.label), (rows) => rows.length === rowsPre.length, 8000, 800);
  verdict("r22-exit-leaves-zero-trace", exited.ok && back.ok && rowsSettled.ok, { band: exited.value?.stage ?? null, drift: movedIds(head, back.value, 1e-6).slice(0, 6), rows: [rowsPre.length, rowsSettled.value.length] });
  await closePanels();
};

/** 🥅️ Step 21 — the goal's own sentence, one verdict set on the main document: two selected nodes are dragged (ONE history row
 * holding the `drag-selection` leaf), a third node is dragged afterwards (downstream). Edit on the drag → the editor offers the
 * SELECTION as a reference list — Use selection plus one chip per target, each with its Remove — and the OFFSET as dx / dy
 * steppers that step by the grid snap. Each is changed through its own control and the preview follows: ArrowUp on dx, Remove on
 * one target, the board selection through Use selection. Accept re-applies the downstream drag (review ready); Exit, zero trace. */
export const step21 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const { camera, picked } = await pickNodes(4, [[60, 40], [-50, 30], [61, 40]], exclude, 60);
  if (picked.length < 4) {
    verdict("goal-four-clickable-nodes", false, { picked: picked.map((row) => row.id), reading: "the set needs two nodes to drag, one for the downstream drag and one to take over as the target" });
    return;
  }
  const [a, b, c, d] = picked.map((row) => row.id);
  await selectNodes([picked[0], picked[1]]);
  const p0 = await positions();
  await dragBy(picked[0].at, 60 * camera.zoom, 40 * camera.zoom);
  const moved = await waitUntil(positions, (now) => offsetOf(p0, now, a)?.some((value) => Math.abs(value) > 0.5) === true && offsetOf(p0, now, b)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const offset = offsetOf(p0, moved.value, a) ?? [0, 0];
  const [dx, dy] = offset;
  const drag = await newMutation(newestBefore, (row) => row.label.startsWith(dragLabel(2, dx, dy)));
  const dragEntries = documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore);
  const leaves = drag.entry ? (await expandEntry(drag.entry.id)).mutations : [];
  verdict("goal-one-drag-of-the-selection-is-one-history-row", moved.ok && dragEntries.length === 1 && Boolean(drag.mutation) && leaves.filter((row) => row.label.startsWith(dragLabel(2, dx, dy))).length === 1, { offset, rows: dragEntries.map((row) => row.label), leaves: leaves.map((row) => row.label), expected: dragLabel(2, dx, dy), reading: "the select tool is a state machine: one gesture = one transaction = one History row, holding one `drag-selection` mutation for the whole selection" });
  if (!drag.mutation) return;
  await closePanels();
  const [cRow] = await rowsOf([c]);
  const beforeC = await positions();
  await selectNodes(cRow ? [cRow] : [picked[2]]);
  await dragBy((cRow ?? picked[2]).at, -50 * camera.zoom, 30 * camera.zoom);
  const cMoved = await waitUntil(positions, (now) => offsetOf(beforeC, now, c)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const cOffset = offsetOf(beforeC, cMoved.value, c) ?? [0, 0];
  const head = cMoved.value;
  const rowsPre = documentEntries(await allHistoryRows()).map((row) => row.label);
  await installBandTrace();
  const begun = await beginEditOf(drag.mutation.key);
  verdict("goal-edit-opens-the-drag-mutation", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const slow = renderer === "wgpu" ? 45000 : 15000;
  const editorRead = await waitUntil(readEditor, (value) => Boolean(value?.numbers.dx && value?.numbers.dy && value?.targets), slow);
  const ed = editorRead.value;
  if (editorRead.waitedMs > 5000) note("the-editor-arrived-late", { waitedMs: editorRead.waitedMs, complete: editorRead.ok, reading: "time from the band reading `editing` to the editor's inputs (targets, dx, dy) being readable" });
  if (renderer === "wgpu") note("wgpu-editor-mirror-nodes", { nodes: (await mirror()).filter((node) => /framework\.history\.(editor|timeTravel)|shell\.time-travel|ui\.timeTravel/u.test(node.key)).map((node) => `${node.key}|${node.tag}${node.inputType ? `[${node.inputType}]` : ""}:${node.role}|${node.label.slice(0, 50)}|value=${node.value ?? node.valueNow ?? ""} step=${node.step ?? ""} min=${node.min ?? ""} max=${node.max ?? ""}${node.disabled ? " disabled" : ""}${node.actionable ? " actionable" : ""}${node.description ? ` described=${node.description.slice(0, 50)}` : ""}`), reading: "what the ARIA mirror carries for the open editor and the band — the DOM an assistive technology reads" });
  const removes = await Promise.all([0, 1].map((index) => presentKey(`framework.history.editor.input.targets.chip.${index}`)));
  verdict("goal-the-editor-offers-the-selection-as-a-reference-list", Boolean(ed?.targets?.useSelection && ed.targets.chips.length === 2 && removes.every((key) => key !== null)), { chips: ed?.targets?.chips ?? null, useSelection: ed?.targets?.useSelection ?? null, removes, reading: "the drag's `targets` input is a reference list: Use selection takes the board selection, each referenced node is a chip named by its label with its own Remove" });
  const snapStep = renderer === "wgpu" ? (mirrorFind(await mirror(), "framework.history.editor.input.dx")?.step ?? null) : (ed?.numbers.dx?.step ?? null);
  verdict("goal-the-editor-offers-the-offset-as-steppers-with-the-grid-snap", Boolean(ed?.numbers.dx?.stepper && ed.numbers.dy?.stepper && near(Number(ed.numbers.dx.value), dx, 0.005) && near(Number(ed.numbers.dy.value), dy, 0.005) && (renderer === "wgpu" ? true : ed.numbers.dx.step === "1" && ed.numbers.dy.step === "1" && ed.numbers.dx.plus && ed.numbers.dx.minus)), { dx: ed?.numbers.dx, dy: ed?.numbers.dy, snapStep, reading: "dx / dy are number steppers reading the recorded offset; their step is the board's grid snap (`snapSource {config: gridFactor}`, default 1) — on wgpu the step itself is proven by the ArrowUp below" });
  const onGrid = Math.abs(dx - Math.round(dx)) < 1e-6 && Math.abs(dy - Math.round(dy)) < 1e-6;
  verdict("goal-the-recorded-drag-offset-lies-on-the-grid-snap", onGrid, { offset, editor: [ed?.numbers.dx?.value ?? null, ed?.numbers.dy?.value ?? null], row: drag.mutation.label, reading: "the board snaps a drag to its grid (snap 1 world unit): the recorded `dx` / `dy` are whole snaps, which is what the row label shows and what the steppers step from" });
  const dx1 = onGrid ? dx + 1 : Math.ceil(dx);
  const exact = onGrid ? 0.02 : 1e-5;
  await keyEditorNumber("dx", "ArrowUp");
  const stepped = await waitUntil(positions, (now) => placed(now, a, p0[a], dx1, dy, exact) && placed(now, b, p0[b], dx1, dy, exact), slow);
  verdict("goal-stepping-the-offset-previews-it-and-keeps-downstream-unapplied", stepped.ok && placed(stepped.value, c, beforeC[c], 0, 0), { a: stepped.value[a], expected: [p0[a][0] + dx1, p0[a][1] + dy], steppedTo: dx1, waitedMs: stepped.waitedMs, c: stepped.value[c], cBeforeItsDrag: beforeC[c], reading: "one ArrowUp = one grid snap (from an offset that is not on the grid: up to the next snap, as a number stepper steps); the preview is the document before the drag plus the draft, the later drag is not applied" });
  await revealWindowRow("framework.history.editor.inputs", "framework.history.editor.input.targets.chip.1.row");
  if (renderer === "wgpu") await mirrorAwait("framework.history.editor.input.targets.chip.1", slow);
  const removed = await pressAuthored("framework.history.editor.input.targets.chip.1", false);
  const oneChip = await waitUntil(readEditor, (value) => (value?.targets?.chips.length ?? 0) === 1, slow);
  const without = await waitUntil(positions, (now) => [a, b].filter((id) => placed(now, id, p0[id], dx1, dy, exact)).length === 1 && [a, b].filter((id) => placed(now, id, p0[id], 0, 0)).length === 1, slow);
  verdict("goal-removing-a-target-previews-the-drag-without-it", removed.present && oneChip.ok && without.ok, { removed, chips: oneChip.value?.targets?.chips ?? null, waitedMs: [oneChip.waitedMs, without.waitedMs], a: without.value[a], b: without.value[b], before: { a: p0[a], b: p0[b] }, reading: "Remove on one chip drops that node from the drag's selection: the preview moves only the remaining target" });
  const [dRow] = await rowsOf([d]);
  const selected = dRow ? await selectNodes([dRow]) : [];
  await openHistory();
  const used = await pressUseSelection("targets");
  const replaced = await waitUntil(readEditor, (value) => (value?.targets?.chips.length ?? 0) === 1, slow);
  const taken = await waitUntil(positions, (now) => placed(now, d, head[d], dx1, dy, 0.05) && placed(now, a, p0[a], 0, 0) && placed(now, b, p0[b], 0, 0), slow);
  verdict("goal-use-selection-replaces-the-targets", selected.includes(d) && Boolean(used) && replaced.ok && taken.ok, { selected, used, chips: replaced.value?.targets?.chips ?? null, waitedMs: [replaced.waitedMs, taken.waitedMs], d: taken.value[d], expected: [head[d][0] + dx1, head[d][1] + dy], a: taken.value[a], b: taken.value[b], reading: "the board selection replaces the drag's targets: the preview drags the newly selected node by the drafted offset and leaves the former targets where they were before the drag" });
  const via = await pressBand("accept");
  const review = await waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 60000, 100);
  const reapplied = await waitUntil(positions, (now) => placed(now, c, beforeC[c], cOffset[0], cOffset[1], 0.05) && placed(now, d, head[d], dx1, dy, 0.05), slow);
  verdict("goal-accept-re-applies-the-downstream-drag", review.value?.review === "ready" && reapplied.ok, { via, review: review.value?.review ?? null, text: review.value?.text?.slice(0, 160), c: reapplied.value[c], expectedC: [beforeC[c][0] + cOffset[0], beforeC[c][1] + cOffset[1]], d: reapplied.value[d], reading: "Accept replays every later mutation on the edited history: the downstream drag lands where it was recorded, the review is ready to finalize" });
  await shot("goal-reviewed");
  let exited = await exitSession();
  if (!exited.ok && renderer === "wgpu") exited = await waitUntil(band, (state) => state === null, slow);
  const back = await waitUntil(positions, (now) => movedIds(head, now, 1e-6).length === 0, renderer === "wgpu" ? slow : 20000, 500);
  const rowsSettled = await waitUntil(async () => documentEntries(await allHistoryRows()).map((row) => row.label), (rows) => rows.length === rowsPre.length, 8000, 800);
  verdict("goal-exit-leaves-zero-trace", exited.ok && back.ok && rowsSettled.ok, { band: exited.value?.stage ?? null, exitWaitedMs: exited.waitedMs, backWaitedMs: back.waitedMs, drift: movedIds(head, back.value, 1e-6).slice(0, 6), rows: [rowsPre.length, rowsSettled.value.length] });
  await closePanels();
};

type HandleRow = { id: string; at: [number, number]; node: string; kind: string; flag: boolean };

/** 🪝️ The handles the overview board publishes (React `data-board-handle-positions-json`: viewport-bounded and capped, rows
 * `[id, x, y, nodeId, kind, flag]`), or null where the renderer publishes none. */
const handleRows = async (): Promise<{ total: number; published: number; capped: boolean; rows: HandleRow[] } | null> => {
  if (renderer === "wgpu") return null;
  const raw = await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-handle-positions-json") ?? "", "", `window:${OVERVIEW}`);
  try {
    const parsed = JSON.parse(raw || "{}") as { total?: number; published?: number; capped?: boolean; rows?: [string, number, number, string, string, boolean][] };
    if (!parsed.rows) return null;
    return { total: parsed.total ?? 0, published: parsed.published ?? parsed.rows.length, capped: parsed.capped === true, rows: parsed.rows.map(([id, x, y, node, kind, flag]) => ({ id, at: [x, y] as [number, number], node, kind, flag })) };
  } catch {
    return null;
  }
};

/** 🪢️ Step 22 — design §22.13, `mutation.precondition-drifted`. The live example has no free handle, so the step makes a
 * compatible free pair: a one-handle node is duplicated (the clone's handle is free), the original is deleted (the handle it
 * was connected to is free again — read as the published handle row of another node whose flag flipped), and the clone is
 * dropped so its handle lies 8 world units beside that freed handle: the drop records the drag AND its proximity
 * `connect-handles` (with the tolerance it was recorded under) in ONE row. Edit the drag → dx far away → Accept: the connect
 * is kept and reads "Warning: Precondition drifted", new since this edit, review ready (a warning never blocks). Withdraw on
 * the connect's row → ready with two accepted changes and no warning → Finalize overwrite "2 mutations". A precondition the
 * probe cannot establish (no one-handle clone source, the freed handle not identifiable, no connect recorded) is a NOTE with
 * its evidence — the drifted outcome is then not judged. */
export const step22 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  if (renderer === "wgpu") {
    note("d13-not-probed-on-this-renderer", { reading: "the step aims at handles through React's `data-board-handle-positions-json`; `dumpBoard2d` publishes no handle positions" });
    return;
  }
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const tried: string[] = [];
  let spot: Awaited<ReturnType<typeof pickCloneSource>> = null;
  let published: Awaited<ReturnType<typeof handleRows>> = null;
  for (let attempt = 0; attempt < 8 && !spot; attempt++) {
    const candidate = await pickCloneSource([...exclude, ...tried]);
    if (!candidate) break;
    published = await handleRows();
    if ((published?.rows ?? []).filter((row) => row.node === candidate.row.id).length === 1) spot = candidate;
    else tried.push(candidate.row.id);
  }
  if (!spot || !published) {
    note("d13-not-provokable-no-one-handle-clone-source", { tried, published: published ? { total: published.total, published: published.published, capped: published.capped } : null });
    return;
  }
  let x = spot.row;
  let zoomedIn = 0;
  while (zoomedIn < 8 && (published.capped || !published.rows.some((row) => row.node === x.id))) {
    await page.mouse.move(x.at.x, x.at.y);
    await page.mouse.wheel(0, -300);
    await sleep(900);
    const [again] = await rowsOf([x.id]);
    if (again) x = again;
    published = (await waitUntil(handleRows, (rows) => rows !== null && rows.published > 0, 4000, 400)).value ?? published;
    zoomedIn += 1;
  }
  const ownBefore = published.rows.find((row) => row.node === x.id) ?? null;
  if (published.capped || !ownBefore) {
    note("d13-not-provokable-the-board-caps-its-published-handles", { source: x.id, zoomedIn, published: { total: published.total, published: published.published, capped: published.capped }, zoom: cameraOf(await vitals()).zoom, reading: "`data-board-handle-positions-json` is viewport-bounded and capped at 128 rows in id order; the step needs every handle around the source published, so it zooms in on it until the cap no longer cuts" });
    return;
  }
  const before = published;
  const cameras: Record<string, unknown> = { zoomedIn: cameraOf(await vitals()) };
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const edgesBefore = (await vitals())?.edges ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(x.at.x, x.at.y);
  await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === x.id, 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  if (!clone) {
    note("d13-not-provokable-no-clone", { nodes: [nodesBefore, (await vitals())?.nodes ?? null] });
    return;
  }
  await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  cameras.afterTheDuplicate = cameraOf(await vitals());
  const zoom = cameraOf(await vitals()).zoom;
  await page.mouse.click(x.at.x - 8 * zoom * Math.SQRT1_2, x.at.y - 8 * zoom * Math.SQRT1_2);
  const reselected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === x.id, 15000);
  cameras.afterTheReselect = cameraOf(await vitals());
  const deleted = reselected.ok ? await runPaletteCommand("delete", /./u, "deleteSelection") : null;
  const removed = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore && (v?.edges ?? -1) === edgesBefore - 1, 30000);
  cameras.afterTheDelete = cameraOf(await vitals());
  let republished = await waitUntil(handleRows, (rows) => (rows?.rows ?? []).some((row) => row.node === clone), 3000, 400);
  for (let attempt = 0; attempt < 8 && !republished.ok; attempt++) {
    const at = toScreen((await positions())[clone], cameraOf(await vitals()), await paneBox());
    await page.mouse.move(at.x, at.y);
    await page.mouse.wheel(0, -300);
    await sleep(900);
    republished = await waitUntil(handleRows, (rows) => rows !== null && !rows.capped && rows.rows.some((row) => row.node === clone), 2500, 400);
  }
  cameras.atTheRead = cameraOf(await vitals());
  const after = republished.value;
  const cloneHandle = (after?.rows ?? []).find((row) => row.node === clone) ?? null;
  const flipped = (after?.rows ?? []).filter((row) => row.node !== x.id && row.node !== clone && before.rows.some((old) => old.id === row.id && old.flag !== row.flag));
  const freed = flipped.length === 1 ? flipped[0] : null;
  const evidence = { source: x.id, sourceHandle: ownBefore, zoomedIn, cameras, clone, cloneHandle, flipped: flipped.slice(0, 6), reselected: reselected.ok, deleted, removed: removed.ok, counts: { nodes: [nodesBefore, removed.value?.nodes ?? null], edges: [edgesBefore, removed.value?.edges ?? null] }, published: after ? { total: after.total, published: after.published, capped: after.capped, waitedMs: republished.waitedMs } : null };
  if (!removed.ok || !cloneHandle || !freed) {
    note("d13-not-provokable-the-freed-handle-is-not-identifiable", { ...evidence, reading: "the handle the deleted node was connected to is read as the one published handle row of another node whose flag flipped with the delete" });
    return;
  }
  const vector: [number, number] = [freed.at[0] - cloneHandle.at[0], freed.at[1] - cloneHandle.at[1]];
  const length = Math.hypot(vector[0], vector[1]);
  const aim: [number, number] = length > 8 ? [Math.round(vector[0] - (8 * vector[0]) / length), Math.round(vector[1] - (8 * vector[1]) / length)] : [Math.round(vector[0]), Math.round(vector[1])];
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const box = await paneBox();
  const cloneAt = toScreen(beforeDrag[clone], camera, box);
  const grab = { x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 };
  const release = { x: grab.x + aim[0] * camera.zoom, y: grab.y + aim[1] * camera.zoom };
  if (!inside(grab, box, 20) || !inside(release, box, 20) || (aim[0] === 0 && aim[1] === 0)) {
    note("d13-not-provokable-the-drop-point-is-outside-the-pane", { ...evidence, freed, vector, aim, grab, release, zoom: camera.zoom });
    return;
  }
  await dragBy(grab, aim[0] * camera.zoom, aim[1] * camera.zoom);
  const dragged = await waitUntil(positions, (now) => offsetOf(beforeDrag, now, clone)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const offset = offsetOf(beforeDrag, dragged.value, clone) ?? [0, 0];
  const connected = await waitUntil(vitals, (v) => (v?.edges ?? -1) === edgesBefore, 10000);
  const dropped = (await waitUntil(handleRows, (rows) => (rows?.rows ?? []).some((row) => row.node === clone), 6000, 400)).value;
  const cloneHandleNow = (dropped?.rows ?? []).find((row) => row.node === clone) ?? null;
  const freedNow = (dropped?.rows ?? []).find((row) => row.id === freed.id) ?? null;
  const gap = cloneHandleNow && freedNow ? Math.hypot(cloneHandleNow.at[0] - freedNow.at[0], cloneHandleNow.at[1] - freedNow.at[1]) : null;
  const drag = dragged.ok ? await newMutation(newestBefore, (row) => row.label.startsWith(dragLabel(1, offset[0], offset[1]))) : { entry: null, mutation: null, waitedMs: 0 };
  const leaves = drag.entry ? (await expandEntry(drag.entry.id)).mutations : [];
  const connect = leaves.find((row) => row.id !== drag.mutation?.id) ?? null;
  const dropEvidence = { ...evidence, freed, vector, aim, offset, handleGapAfterTheDrop: gap, kinds: [cloneHandle.kind, freed.kind], edges: [edgesBefore - 1, connected.value?.edges ?? null], row: drag.entry?.label ?? null, leaves: leaves.map((row) => row.label) };
  if (!drag.mutation || !connect || !connected.ok) {
    note("d13-not-provokable-the-drop-recorded-no-connect", { ...dropEvidence, reading: "`puzzle2d_selection_yields`: a drop pairs every free handle of the moved node with the nearest compatible free handle within the proximity radius (12 world units) and records `connect-handles` beside the drag; with the gap above no connect was recorded — the freed handle may be misread, or the kinds are not compatible" });
    return;
  }
  verdict("d13-a-drop-beside-a-free-handle-records-the-drag-and-its-connect-in-one-row", leaves.length === 2, { ...dropEvidence, reading: "one gesture = one transaction = one History row: the `drag-selection` leaf and the proximity `connect-handles` it landed, which states the tolerance it was recorded under" });
  const head = await positions();
  await installBandTrace();
  const begun = await beginEditOf(drag.mutation.key);
  verdict("d13-edit-opens-the-drag", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const far = Math.round(offset[0]) + 200;
  const typed = await typeEditorNumber("dx", far);
  const previewed = await waitUntil(positions, (now) => placed(now, clone, beforeDrag[clone], far, offset[1], 0.05), 15000);
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("d13-a-drifted-connect-does-not-block-the-review", previewed.ok && review.value?.review === "ready", { typed, via, review: review.value?.review ?? null, text: review.value?.text?.slice(0, 200), clone: previewed.value[clone], expected: [beforeDrag[clone][0] + far, beforeDrag[clone][1] + offset[1]], reading: "design §16.1 / §22.13: the connect is kept and warns; a warning never blocks finalizing" });
  verdict("d13-the-band-names-the-warning", review.value?.outcome === "warning" || /Worst outcome: Warning|Schwerstes Ergebnis: Warnung/u.test(review.value?.text ?? ""), { outcome: review.value?.outcome ?? null, text: review.value?.text?.slice(0, 200) });
  const warned = (await waitUntil(() => findMutationRow(connect.key), (row) => Boolean(row?.text.includes(copy.drifted)), 15000)).value;
  verdict("d13-the-connect-row-reads-precondition-drifted", Boolean(warned?.text.includes(copy.drifted)), { row: warned?.text?.slice(0, 220) ?? null, expected: copy.drifted, reading: "`mutation.precondition-drifted`: the two handles now lie farther apart than the tolerance the connect was recorded under" });
  verdict("d13-the-drifted-warning-is-marked-new-since-this-edit", Boolean(warned?.text.includes(copy.introduced)), { row: warned?.text?.slice(0, 220) ?? null, expected: copy.introduced });
  const keptEdges = (await vitals())?.edges ?? null;
  const viaWithdraw = warned ? await pressRowAction(warned.id, copy.withdraw) : "absent";
  let ready = await waitUntil(band, (b) => b !== null && (b.stage === "editing" || (b.stage === "reviewing" && Boolean(b.review) && b.text.includes(copy.accepted(2)))), 30000, 100);
  if (ready.value?.stage === "editing") {
    await pressBand("accept");
    ready = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  }
  const apart = await waitUntil(vitals, (v) => (v?.edges ?? -1) === edgesBefore - 1, 10000);
  verdict("d13-withdrawing-the-drifted-connect-leaves-a-ready-review-without-a-warning", viaWithdraw === "action" && ready.value?.review === "ready" && Boolean(ready.value.text.includes(copy.accepted(2))) && ready.value.outcome !== "warning" && !/Worst outcome: Warning|Schwerstes Ergebnis: Warnung/u.test(ready.value.text) && apart.ok, { via: viaWithdraw, review: ready.value?.review ?? null, outcome: ready.value?.outcome ?? null, text: ready.value?.text?.slice(0, 220), edges: { kept: keptEdges, withdrawn: apart.value?.edges ?? null, expected: [edgesBefore, edgesBefore - 1] }, expected: copy.accepted(2) });
  await shot("d13-reviewed");
  const finalized = await finalizeOverwrite();
  const overwrite = await waitUntil(async () => documentEntries(await allHistoryRows()).find((row) => row.label.startsWith(copy.overwriteTwo)) ?? null, (row) => row !== null, 20000, 800);
  const settled = await waitUntil(positions, (now) => placed(now, clone, beforeDrag[clone], far, offset[1], 0.05) && movedIds(head, now, 1e-6).every((id) => id === clone), 15000);
  verdict("d13-finalize-overwrite-records-both-edits", finalized.closed && overwrite.ok && settled.ok && ((await vitals())?.edges ?? -1) === edgesBefore - 1, { finalized, row: overwrite.value?.label ?? null, expected: copy.overwriteTwo, clone: settled.value[clone], edges: (await vitals())?.edges ?? null });
  await closePanels();
};

/** 🟦️ The engine's live gesture state of the overview board (React `data-board-interaction-json`: `mode`, `utility`,
 * `hoveredId`), or null where the renderer publishes none. */
const boardInteraction = async (): Promise<{ mode: string; utility: string; hoveredId: string | null } | null> => {
  if (renderer === "wgpu") return null;
  const raw = await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-interaction-json") ?? "", "", `window:${OVERVIEW}`);
  try {
    const parsed = JSON.parse(raw || "null") as { mode?: string; utility?: string; hoveredId?: string | null } | null;
    return parsed ? { mode: parsed.mode ?? "", utility: parsed.utility ?? "", hoveredId: parsed.hoveredId ?? null } : null;
  } catch {
    return null;
  }
};

type TargetRegion = { id: string; x: number; y: number; width: number; height: number; locked: boolean };

/** 🔬️ The target regions the overview board publishes (React `data-board-target-regions-json`, world units). */
const targetRegions = async (): Promise<TargetRegion[]> => {
  if (renderer === "wgpu") return [];
  const raw = await evalSafe((surface) => document.querySelector(`[data-surface-id="${surface}"]`)?.getAttribute("data-board-target-regions-json") ?? "", "", `window:${OVERVIEW}`);
  try {
    return JSON.parse(raw || "[]") as TargetRegion[];
  } catch {
    return [];
  }
};

/** 🛠️ Arms the overview window's utility `id` through its utility bar (`ui.utilities.<window>` holds one button per utility,
 * its DOM id the utility id; a press on the armed one falls back to select, so an armed utility is left alone). */
const setUtility = async (id: "select" | "brush" | "areaBrush") => {
  if ((await vitals())?.utility === id) return true;
  const button = page.locator(`[id="ui.utilities.${OVERVIEW}"] button[id="${id}"]`).first();
  if (!(await button.count().catch(() => 0))) {
    await page.locator('[id$="2dOverview.utilityBar.unfold"]').first().click({ timeout: 4000 }).catch(() => {});
    await sleep(700);
  }
  await button.click({ timeout: 4000 }).catch((error) => log(`utility ${id} press failed ${String(error).split("\n")[0]}`));
  return (await waitUntil(vitals, (v) => v?.utility === id, 6000)).ok;
};

/** 🗜️ Zooms in on `world` until the board publishes every on-screen handle (its attribute is capped at 128 rows) and answers
 * the rows. A structural change resets the camera, so every handle aim starts here. */
const revealHandles = async (world: [number, number]) => {
  let read = (await waitUntil(handleRows, (rows) => rows !== null && rows.rows.length > 0, 3000, 400)).value;
  for (let attempt = 0; attempt < 8 && (!read || read.capped || read.rows.length === 0); attempt++) {
    const at = toScreen(world, cameraOf(await vitals()), await paneBox());
    await page.mouse.move(at.x, at.y);
    await page.mouse.wheel(0, -300);
    await sleep(900);
    read = await handleRows();
  }
  return read;
};

/** 🧯 The pixels of one board window's canvas (PNG, base64; the header strip and the footer left out) — equality of two
 * shots is the evidence that a window painted nothing new between them. */
const paneShot = async (windowId: string) => {
  const box = await page.locator(`[data-surface-id="window:${windowId}"] canvas`).first().boundingBox().catch(() => null);
  if (!box || box.width < 60 || box.height < 160) return null;
  const shot = await page.screenshot({ clip: { x: Math.round(box.x) + 4, y: Math.round(box.y) + 40, width: Math.round(box.width) - 8, height: Math.round(box.height) - 90 } }).catch(() => null);
  return shot ? shot.toString("base64") : null;
};

/** 🧰 Step 23 — design §22.32 (a), "tools are state machines that yield mutations within a transaction", on the board tools
 * a person reaches: one release = ONE History row with its mutations, a release that changes nothing or an aborted gesture =
 * no row. In order: a node drag's streamed preview paints in the other windows (pixel evidence) and Escape aborts it; a click
 * on empty board; a drag that becomes the upstream row; a history edit begun in the middle of a drag; Select All from the
 * Actions rail and Use selection in an open edit; Delete of a one-handle capsule (node + its edge) and its replay on Accept of
 * the upstream edit; a wire dragged from a clone's free handle onto the freed handle; a second delete and the brush (Alt +
 * sweep over the freed handle) placing a node that is selected at once, edited, withdrawn and restored; the area brush
 * painting a region and a grip resizing it. A precondition the probe cannot establish is a NOTE, never a FAIL. */
export const step23 = async (ctx: Ctx) => {
  const copy = COPY[currentLocale];
  if (renderer === "wgpu") {
    note("m32-not-probed-on-this-renderer", { reading: "the step aims at handles, regions and the utility bar through React's `data-board-*` attributes and DOM ids; the wgpu arm has no reader for them yet" });
    return;
  }
  const settledNewestSeq = async () => {
    let last = -2;
    const read = await waitUntil(async () => newestEntrySeq(await allHistoryRows()), (newest) => {
      const same = newest > 0 && newest === last;
      last = newest;
      return same;
    }, 12000, 600);
    return read.value;
  };
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { camera, picked } = await pickNodes(2, [[50, 40]], exclude, 60);
  if (picked.length < 2) {
    verdict("m32-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const [vNode, uNode] = picked;
  const zoom = camera.zoom;
  const windows = ["2d-detail", "2d-selection"];
  const shots = async () => {
    const out: Record<string, string | null> = {};
    for (const id of windows) out[id] = await paneShot(id);
    return out;
  };
  let seq = await settledNewestSeq();
  const p0 = await positions();
  await selectNodes([vNode]);
  await page.mouse.move(vNode.at.x, vNode.at.y);
  await sleep(1500);
  const still1 = await shots();
  const still2 = await shots();
  await page.mouse.down();
  await sleep(700);
  const pressed1 = await shots();
  const pressed2 = await shots();
  await page.mouse.move(vNode.at.x + 50 * zoom, vNode.at.y + 40 * zoom, { steps: 8 });
  await sleep(1000);
  const moved = await shots();
  const dragMode = (await boardInteraction())?.mode ?? null;
  const perWindow = windows.map((id) => ({ window: id, shot: moved[id] !== null, stableBeforeThePress: still1[id] !== null && still1[id] === still2[id], stableWhilePressed: pressed1[id] !== null && pressed1[id] === pressed2[id], paintedWhileDragged: moved[id] !== pressed2[id] }));
  const judged = perWindow.filter((row) => row.stableBeforeThePress && row.stableWhilePressed);
  verdict("m32-a-streamed-drag-preview-paints-in-the-other-windows", dragMode === "dragNodes" && judged.length > 0 && judged.every((row) => row.paintedWhileDragged), { perWindow, judgedOn: judged.map((row) => row.window), mode: dragMode, reading: "S5-PUZZLE B3: an open stream's preview paints in every window (framework overlay). Evidence is pixels: a window whose canvas is pixel-stable before the press and while the node is held must paint something new once the node is dragged; a window that is not pixel-stable on its own is listed, not judged" });
  await page.keyboard.press("Escape").catch(() => {});
  await sleep(600);
  await page.mouse.up();
  await sleep(2500);
  const afterEscape = await positions();
  const escapeRows = await documentRowsSince(seq, 0, 0);
  verdict("m32-an-aborted-drag-leaves-no-row", dragMode === "dragNodes" && movedIds(p0, afterEscape, 1e-6).length === 0 && escapeRows.documents.length === 0, { mode: dragMode, moved: movedIds(p0, afterEscape, 1e-6).slice(0, 4), documentRows: escapeRows.documents.map((row) => row.label), commandRows: escapeRows.commands, reading: "Escape in the middle of a node drag cancels the gesture: the node is back, the tool's transaction yields nothing, History lists no document row" });
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 4000);
  await sleep(1500);
  seq = Math.max(seq, await settledNewestSeq());
  await closePanels();
  const first = await layout();
  let empty: Point | null = null;
  for (let gy = 120; gy < first.box.height - 120 && !empty; gy += 40)
    for (let gx = 40; gx < first.box.width - 40 && !empty; gx += 40) {
      const candidate = { x: first.box.x + gx, y: first.box.y + gy };
      if (first.rows.every((row) => Math.hypot(row.at.x - candidate.x, row.at.y - candidate.y) > 90)) empty = candidate;
    }
  const beforeIdle = await positions();
  if (empty) await page.mouse.click(empty.x, empty.y);
  await sleep(3000);
  const idle = await documentRowsSince(seq, 0, 0);
  verdict("m32-a-press-and-release-that-changes-nothing-leaves-no-row", empty !== null && idle.documents.length === 0 && idle.commands.length === 0 && movedIds(beforeIdle, await positions(), 1e-6).length === 0 && selectionIds(await vitals()).length === 0, { at: empty, documentRows: idle.documents.map((row) => row.label), commandRows: idle.commands, reading: "a click on empty board with nothing selected changes neither the document nor the selection: zero trace, not even a command row" });
  seq = idle.newest;
  await selectNodes([uNode]);
  const pU = await positions();
  await dragBy(uNode.at, 40 * zoom, 30 * zoom);
  const uMoved = await waitUntil(positions, (now) => offsetOf(pU, now, uNode.id)?.some((value) => Math.abs(value) > 0.5) === true, 20000);
  const uOffset = offsetOf(pU, uMoved.value, uNode.id) ?? [0, 0];
  const uDrag = await newMutation(seq, (row) => row.label.startsWith(dragLabel(1, uOffset[0], uOffset[1])));
  if (!uDrag.mutation) {
    verdict("m32-the-upstream-drag-is-recorded", false, { offset: uOffset, expected: dragLabel(1, uOffset[0], uOffset[1]) });
    return;
  }
  const uKey = uDrag.mutation.key;
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 4000);
  await sleep(1200);
  const uRow = await findMutationRow(uKey);
  seq = newestEntrySeq(await readHistory());
  const pG = await positions();
  const [vLive] = await rowsOf([vNode.id]);
  const vAt = vLive?.at ?? vNode.at;
  await page.mouse.move(vAt.x, vAt.y);
  await page.mouse.down();
  await page.mouse.move(vAt.x + 50 * zoom, vAt.y + 40 * zoom, { steps: 8 });
  await sleep(700);
  const modeMid = (await boardInteraction())?.mode ?? null;
  const begunInPage = uRow
    ? await evalSafe(
        (arg) => {
          const row = document.getElementById(arg.rowId);
          const button = Array.from(row?.querySelectorAll<HTMLElement>('button, [role="button"]') ?? []).find((el) => new RegExp(arg.source, arg.flags).test((el.getAttribute("aria-label") ?? el.getAttribute("title") ?? el.innerText ?? "").trim()));
          button?.click();
          return Boolean(button);
        },
        false,
        { rowId: uRow.id, source: copy.edit.source, flags: copy.edit.flags },
      )
    : false;
  const frozen = await waitUntil(band, (state) => state?.stage === "editing", 10000, 100);
  const modeFrozen = (await boardInteraction())?.mode ?? null;
  const codesFrozen = await shownNoticeCodes();
  await page.mouse.up();
  await sleep(1500);
  if (await band()) await exitSession();
  await sleep(2000);
  const afterG = await positions();
  const gRows = await documentRowsSince(seq, 0, 0);
  verdict("m32-a-history-edit-begun-mid-drag-leaves-no-row", modeMid === "dragNodes" && begunInPage && frozen.ok && movedIds(pG, afterG, 1e-6).length === 0 && gRows.documents.length === 0, { modeWhileDragging: modeMid, editPressedInPage: begunInPage, bandAfterThePress: frozen.value?.stage ?? null, waitedMs: frozen.waitedMs, modeAfterTheBegin: modeFrozen, notices: codesFrozen.slice(-4), moved: movedIds(pG, afterG, 1e-6).slice(0, 4), documentRows: gRows.documents.map((row) => `${row.label} [${row.mutations.join(" | ")}]`), commandRows: gRows.commands, reading: "a history edit that begins while a board gesture is live freezes the document: the gesture is retired, its release commits nothing, the node is where it was once the session is left. Edit is pressed in page (a DOM click on the row action) because the pointer is held on the board" });
  seq = gRows.newest;
  await closePanels();
  const headU = await positions();
  const pane = page.locator(`[data-slot="window"][id="${OVERVIEW}"]`).first();
  const rail = pane.locator('[data-slot="window-action-pane"]').first();
  const railToggle = pane.locator('[id$=".engagement.toggle"]').first();
  const selectAllFromTheRail = async () => {
    if (!(await rail.isVisible().catch(() => false))) await railToggle.click({ timeout: 4000 }).catch(() => {});
    const item = pane.locator('[id="action.selectAll"]').first();
    const listed = await waitUntil(() => item.count().catch(() => 0), (count) => count > 0, 20000);
    const label = item.locator('[data-slot="tree-label"]').first();
    if (listed.ok) await ((await label.count().catch(() => 0)) ? label : item).click({ timeout: 4000 }).catch((error) => log(`select all press failed ${String(error).split("\n")[0]}`));
    const all = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0 && selectionIds(v).length === (v?.nodes ?? -1), 15000);
    if (await rail.isVisible().catch(() => false)) await railToggle.click({ timeout: 4000 }).catch(() => {});
    return { listed: listed.ok, ok: all.ok, selected: selectionIds(all.value).length, nodes: all.value?.nodes ?? null, waitedMs: all.waitedMs };
  };
  const everything = await selectAllFromTheRail();
  verdict("m32-select-all-from-the-actions-rail-selects-every-node", everything.listed && everything.ok, { ...everything, reading: "F16: the Actions rail's Select All selects every node of the board (the domain declares its topology)" });
  const begunAll = await beginEditOf(uKey);
  if (begunAll.band?.stage === "editing") {
    const kept = selectionIds(await vitals()).length;
    const again = kept === everything.nodes ? null : await selectAllFromTheRail();
    await openHistory();
    const used = await pressUseSelection("targets");
    const sample = Object.keys(headU).filter((id) => id !== uNode.id).slice(0, 5);
    const taken = await waitUntil(positions, (now) => placed(now, uNode.id, pU[uNode.id], uOffset[0], uOffset[1], 0.05) && sample.every((id) => placed(now, id, headU[id], uOffset[0], uOffset[1], 0.05)), 30000, 500);
    const editor = await readEditor();
    verdict("m32-use-selection-takes-the-select-all-selection", used !== null && taken.ok, { used, selectionWhenTheEditOpened: kept, selectedAgainInTheSession: again, chipsMaterialised: editor?.targets?.chips.length ?? null, sample: sample.slice(0, 3).map((id) => ({ id, at: taken.value[id] ?? null, expected: [headU[id][0] + uOffset[0], headU[id][1] + uOffset[1]] })), waitedMs: taken.waitedMs, reading: "Use selection in the open edit of the drag takes the Select All selection as its targets: the preview drags every node by the recorded offset" });
    await pressBand("discard");
    await sleep(700);
    await exitSession();
    await waitUntil(positions, (now) => movedIds(headU, now, 1e-6).length === 0, 20000, 500);
  } else verdict("m32-use-selection-takes-the-select-all-selection", false, { via: begunAll.via, band: begunAll.band?.stage ?? null });
  await closePanels();
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 6000);
  const cameraHome = cameraOf(await vitals());
  const focus: [number, number] = [cameraHome.x, cameraHome.y];
  const firstRows = await revealHandles(focus);
  const posNow = await positions();
  const camNow = cameraOf(await vitals());
  const boxNow = await paneBox();
  const perNode = new Map<string, number>();
  for (const row of firstRows?.rows ?? []) perNode.set(row.node, (perNode.get(row.node) ?? 0) + 1);
  const middle = { x: boxNow.x + boxNow.width / 2, y: boxNow.y + boxNow.height / 2 };
  const capsules = (firstRows?.rows ?? [])
    .filter((row) => /capsule/u.test(row.kind) && row.flag === false && perNode.get(row.node) === 1 && row.node !== uNode.id && row.node !== vNode.id && Boolean(posNow[row.node]) && inside(toScreen(posNow[row.node], camNow, boxNow), boxNow, 120))
    .map((row) => ({ id: row.node, kind: row.kind, far: Math.hypot(toScreen(posNow[row.node], camNow, boxNow).x - middle.x, toScreen(posNow[row.node], camNow, boxNow).y - middle.y) }))
    .sort((left, right) => left.far - right.far);
  const twin = capsules.find((row, index) => index > 0 && row.kind === capsules[0]?.kind) ?? null;
  const third = capsules.find((row, index) => index > 0 && row.id !== twin?.id) ?? null;
  if (!twin || !third) {
    note("m32-not-provokable-fewer-than-three-connected-one-handle-nodes-in-view", { capsules: capsules.slice(0, 8), published: firstRows ? { total: firstRows.total, published: firstRows.published, capped: firstRows.capped } : null, camera: camNow, reading: "the step deletes one such node, duplicates a second one of the SAME handle kind (its clone's free handle is then compatible with the handle the delete freed) and deletes a third" });
    return;
  }
  const [x1, c2, x2] = [capsules[0].id, twin.id, third.id];
  const cameras: { when: string; x: number; y: number; zoom: number }[] = [];
  const mark = async (when: string) => {
    const now = cameraOf(await vitals());
    cameras.push({ when, x: Number(now.x.toFixed(2)), y: Number(now.y.toFixed(2)), zoom: Number(now.zoom.toFixed(3)) });
  };
  const stableDocumentRows = async () => {
    let last = -1;
    const read = await waitUntil(async () => documentEntries(await allHistoryRows()).length, (count) => {
      const same = count > 0 && count === last;
      last = count;
      return same;
    }, 12000, 700);
    return read.value;
  };
  const deleteNode = async (id: string) => {
    await mark(`before the delete of ${id.slice(0, 8)}`);
    await prepareChord();
    await page.keyboard.press("Escape").catch(() => {});
    await waitUntil(vitals, (v) => selectionIds(v).length === 0, 5000);
    const before = await revealHandles(focus);
    const counts = await vitals();
    const at = toScreen((await positions())[id], cameraOf(await vitals()), await paneBox());
    await page.mouse.click(at.x - 6, at.y + 6);
    const selected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === id, 8000);
    const selection = selectionIds(selected.value).slice(0, 4);
    const cameraBefore = cameraOf(await vitals());
    await prepareChord();
    await page.keyboard.press("Delete").catch(() => {});
    const removed = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === (counts?.nodes ?? 0) - 1, 15000);
    await sleep(2500);
    const cameraAfter = cameraOf(await vitals());
    await mark(`2.5 s after the delete of ${id.slice(0, 8)}`);
    const after = await revealHandles(focus);
    const freed = (after?.rows ?? []).filter((row) => row.node !== id && (before?.rows ?? []).some((old) => old.id === row.id && old.flag !== row.flag));
    return { selected: selected.ok, selection, removed: removed.ok, nodes: [counts?.nodes ?? null, removed.value?.nodes ?? null], edges: [counts?.edges ?? null, removed.value?.edges ?? null], freed, cameraBefore, cameraAfter };
  };
  seq = Math.max(seq, await settledNewestSeq());
  await closePanels();
  const d1 = await deleteNode(x1);
  const d1Rows = await documentRowsSince(seq, 1, 20000);
  verdict("m32-deleting-a-node-with-its-edge-is-one-history-row", d1.selected && d1.removed && d1.edges[1] === (d1.edges[0] ?? 0) - 1 && d1Rows.documents.length === 1 && d1Rows.documents[0].mutations.some((label) => label.includes(x1)), { node: x1, nodes: d1.nodes, edges: d1.edges, rows: d1Rows.documents.map((row) => ({ row: row.label, mutationCount: row.count, mutations: row.mutations })), commandRows: d1Rows.commands, freedHandles: d1.freed.map((row) => row.id), reading: "`nodeDelete`: Delete on a selected node removes the node and its edge in ONE tool transaction — one History row" });
  note("m32-camera-after-a-node-delete", { before: d1.cameraBefore, after: d1.cameraAfter, reset: Math.abs(d1.cameraBefore.zoom - d1.cameraAfter.zoom) > 1e-6 || Math.abs(d1.cameraBefore.x - d1.cameraAfter.x) > 1e-6, reading: "O6: whether a structural change takes the person's camera back to its origin (read 2.5 s after the delete)" });
  seq = d1Rows.newest;
  const headD = await positions();
  const begunUp = await beginEditOf(uKey);
  if (begunUp.band?.stage === "editing") {
    const farther = Math.round(uOffset[0]) + 20;
    const typed = await typeEditorNumber("dx", farther);
    const preview = await waitUntil(positions, (now) => placed(now, uNode.id, pU[uNode.id], farther, uOffset[1], 0.05), 20000);
    verdict("m32-an-upstream-edit-previews-without-the-later-delete", preview.ok && Boolean(preview.value[x1]), { typed, u: preview.value[uNode.id] ?? null, expected: [pU[uNode.id][0] + farther, pU[uNode.id][1] + uOffset[1]], deletedNodeInThePreview: preview.value[x1] ?? null, reading: "the preview is the document as of the edited drag plus the draft: the later delete is not applied, the deleted node is shown" });
    const via = await pressBand("accept");
    const review = await waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 60000, 100);
    const replayed = await waitUntil(positions, (now) => !now[x1] && placed(now, uNode.id, pU[uNode.id], farther, uOffset[1], 0.05), 20000);
    verdict("m32-accept-of-the-upstream-edit-replays-the-delete", review.value?.review === "ready" && replayed.ok, { via, review: review.value?.review ?? null, text: review.value?.text?.slice(0, 160), deletedNodeAfterAccept: replayed.value[x1] ?? null, u: replayed.value[uNode.id] ?? null, reading: "Accept replays every later mutation: the delete row's mutations are applied again on the edited history, the review is ready" });
    await exitSession();
    await waitUntil(positions, (now) => movedIds(headD, now, 1e-6).length === 0 && !now[x1], 20000, 500);
  } else verdict("m32-an-upstream-edit-previews-without-the-later-delete", false, { via: begunUp.via, band: begunUp.band?.stage ?? null });
  seq = Math.max(seq, await settledNewestSeq());
  await closePanels();
  await mark("after the upstream edit session, before the duplicate");
  await revealHandles(focus);
  const idsBeforeClone = Object.keys(await positions());
  const c2At = toScreen((await positions())[c2], cameraOf(await vitals()), await paneBox());
  await page.mouse.click(c2At.x - 6, c2At.y + 6);
  await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === c2, 8000);
  const beforeClone = await vitals();
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  await waitUntil(vitals, (v) => (v?.nodes ?? -1) === (beforeClone?.nodes ?? 0) + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBeforeClone.includes(id)) ?? null;
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 6000);
  const cloneRows = await documentRowsSince(seq, 1, 15000);
  seq = cloneRows.newest;
  await closePanels();
  await mark("after the duplicate, before the wire");
  const wireRead = await revealHandles(focus);
  const cloneHandle = clone ? (wireRead?.rows.find((row) => row.node === clone) ?? null) : null;
  const wireTarget = d1.freed.length === 1 ? (wireRead?.rows.find((row) => row.id === d1.freed[0].id) ?? null) : null;
  if (!cloneHandle || !wireTarget) note("m32-not-provokable-no-free-handle-pair-for-a-wire", { clone, cloneHandle, freed: d1.freed.map((row) => row.id), wireTarget, cloneRow: cloneRows.documents.map((row) => row.label) });
  else {
    const cam = cameraOf(await vitals());
    const box = await paneBox();
    const from = toScreen(cloneHandle.at, cam, box);
    const to = toScreen(wireTarget.at, cam, box);
    const edgesBefore = (await vitals())?.edges ?? -1;
    await page.mouse.move(from.x, from.y);
    await sleep(600);
    await page.mouse.down();
    await page.mouse.move((from.x + to.x) / 2, (from.y + to.y) / 2, { steps: 6 });
    await sleep(500);
    const wireMode = (await boardInteraction())?.mode ?? null;
    await page.mouse.move(to.x, to.y, { steps: 6 });
    await sleep(600);
    await page.mouse.up();
    const wired = await waitUntil(vitals, (v) => (v?.edges ?? -1) === edgesBefore + 1, 15000);
    const wireRows = await documentRowsSince(seq, 1, 20000);
    verdict("m32-a-wire-dragged-handle-to-handle-is-one-history-row", wired.ok && wireRows.documents.length === 1 && wireRows.documents[0].count === 1 && wireRows.documents[0].mutations[0].includes(cloneHandle.id) && wireRows.documents[0].mutations[0].includes(wireTarget.id), { from: cloneHandle.id, to: wireTarget.id, kinds: [cloneHandle.kind, wireTarget.kind], mode: wireMode, edges: [edgesBefore, wired.value?.edges ?? null], rows: wireRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })), commandRows: wireRows.commands, reading: "`edgeCreate`: a wire dragged from a free handle and dropped on a compatible free handle is ONE tool transaction — one History row holding the connect" });
    seq = wireRows.newest;
  }
  await closePanels();
  const d2 = await deleteNode(x2);
  const d2Rows = await documentRowsSince(seq, 1, 20000);
  seq = d2Rows.newest;
  await closePanels();
  if (!d2.removed || d2.freed.length !== 1) note("m32-not-provokable-no-freed-handle-for-the-brush", { node: x2, selected: d2.selected, selection: d2.selection, removed: d2.removed, nodes: d2.nodes, edges: d2.edges, freed: d2.freed.map((row) => row.id), rows: d2Rows.documents.map((row) => ({ row: row.label, mutations: row.mutations.slice(0, 6) })), commandRows: d2Rows.commands });
  else {
    const armed = await setUtility("brush");
    await mark("before the brush");
    const brushRead = await revealHandles(focus);
    const slot = brushRead?.rows.find((row) => row.id === d2.freed[0].id) ?? d2.freed[0];
    const at = toScreen(slot.at, cameraOf(await vitals()), await paneBox());
    const idsBeforeBrush = Object.keys(await positions());
    const beforeBrush = await vitals();
    await page.mouse.move(at.x - 90, at.y - 60);
    await page.keyboard.down("Alt");
    await page.mouse.move(at.x, at.y, { steps: 8 });
    await sleep(1500);
    const hovered = (await boardInteraction())?.hoveredId ?? null;
    await page.mouse.move(at.x - 110, at.y - 80, { steps: 8 });
    await sleep(600);
    await page.keyboard.up("Alt");
    const stamped = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === (beforeBrush?.nodes ?? 0) + 1, 15000);
    const stampedAt = Date.now();
    const stampedId = Object.keys(await positions()).find((id) => !idsBeforeBrush.includes(id)) ?? null;
    await mark("right after the brush placed its node");
    const selectArmed = await setUtility("select");
    const stays = { clickedAfterMs: null as number | null, attempts: 0, selected: false, selection: [] as string[], stillSelectedAfterMs: null as number | null, still: false };
    const hovers: string[] = [];
    for (let attempt = 0; attempt < 2 && stampedId && !stays.selected; attempt++) {
      const where = (await positions())[stampedId];
      if (!where) break;
      const centre = toScreen(where, cameraOf(await vitals()), await paneBox());
      let aim: Point | null = null;
      for (const [dx, dy] of [[0, 0], [-6, 6], [6, 6], [8, 0], [-8, 0], [0, 9], [0, -9], [12, 12], [-12, 12], [14, 0], [-14, 0]] as const) {
        await page.mouse.move(centre.x + dx, centre.y + dy);
        await sleep(260);
        const over = (await boardInteraction())?.hoveredId ?? null;
        hovers.push(`${dx},${dy}=${over ?? "nothing"}`);
        if (over === stampedId) {
          aim = { x: centre.x + dx, y: centre.y + dy };
          break;
        }
      }
      if (!aim) continue;
      stays.attempts += 1;
      await page.mouse.click(aim.x, aim.y);
      stays.clickedAfterMs ??= Date.now() - stampedAt;
      const selected = await waitUntil(vitals, (v) => selectionIds(v).includes(stampedId), 5000);
      stays.selected = selected.ok;
      stays.selection = selectionIds(selected.value).slice(0, 4);
    }
    if (stays.selected && stampedId) {
      await sleep(3500);
      stays.stillSelectedAfterMs = 3500;
      stays.still = selectionIds(await vitals()).includes(stampedId);
    }
    const brushRows = await documentRowsSince(seq, 1, 20000);
    const brushRow = brushRows.documents[0] ?? null;
    verdict("m32-a-brush-placed-node-is-one-history-row", armed && stamped.ok && stampedId !== null && brushRows.documents.length === 1 && brushRow?.count === 2 && Boolean(brushRow.mutations.some((label) => label.includes(stampedId) && !label.includes(slot.id))) && (stamped.value?.edges ?? -1) === (beforeBrush?.edges ?? 0) + 1, { slot: slot.id, hoveredWhileArmed: hovered, placed: stampedId, nodes: [beforeBrush?.nodes ?? null, stamped.value?.nodes ?? null], edges: [beforeBrush?.edges ?? null, stamped.value?.edges ?? null], rows: brushRows.documents.map((row) => ({ row: row.label, mutationCount: row.count, materialised: row.mutations })), commandRows: brushRows.commands, reading: "`brushPlace`: with the brush armed, Alt held over an open handle previews a compatible node and leaving the slot stamps it — the node and its connect in ONE tool transaction, one History row" });
    if (stays.attempts === 0) note("m32-the-placed-node-could-not-be-aimed-at", { placed: stampedId, at: stampedId ? ((await positions())[stampedId] ?? null) : null, hovers: hovers.slice(0, 22), reading: "the board never reported the placed node under the pointer (`hoveredId`) at or around its published position, so the click that should select it was not made and the verdict is not judged" });
    else verdict("m32-a-placed-node-selected-at-once-stays-selected", selectArmed && stays.selected && stays.still, { ...stays, hovers: hovers.slice(0, 12), placed: stampedId, reading: "a node that was just placed is clicked as soon as the select utility is armed: it is selected and still selected 3.5 s later, after the guest's publication of the placement came back" });
    seq = brushRows.newest;
    const createIndex = brushRow && stampedId ? brushRow.mutations.findIndex((label) => label.includes(stampedId) && !label.includes(slot.id)) : -1;
    const createKey = brushRow && createIndex >= 0 ? brushRow.keys[createIndex] : null;
    if (!createKey || !stampedId) note("m32-the-create-mutation-of-the-placed-node-is-not-listed", { rows: brushRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })) });
    else {
      const createRow = await findMutationRow(createKey);
      const editOffer = createRow ? await rowActionState(createRow.id, copy.edit, false) : null;
      const begunC = editOffer?.present && editOffer.disabled !== true ? await beginEditOf(createKey) : null;
      const editorC = begunC?.band?.stage === "editing" ? (await waitUntil(readEditor, (value) => (value?.inputs.length ?? 0) > 0, 10000)).value : null;
      verdict("m32-edit-of-the-placed-node-offers-its-inputs", begunC?.band?.stage === "editing" && (editorC?.inputs.length ?? 0) > 0, { row: createRow?.label ?? null, edit: editOffer ? { present: editOffer.present, disabled: editOffer.disabled, reason: editOffer.reason } : null, band: begunC?.band?.text?.slice(0, 140) ?? null, inputs: editorC?.inputs.slice(0, 14) ?? null, reading: "Edit on the placed node's create mutation opens the editor with the leaf's inputs; a mutation whose inputs cannot be edited offers Edit disabled with its reason" });
      if (await band()) await exitSession();
      const headC = await positions();
      const nodesHeadC = (await vitals())?.nodes ?? null;
      const rowsHeadC = await stableDocumentRows();
      const forWithdraw = await findMutationRow(createKey);
      const withdrawOffer = forWithdraw ? await rowActionState(forWithdraw.id, copy.withdraw, false) : null;
      const bandBeforeWithdraw = (await band())?.stage ?? null;
      const viaWithdraw = forWithdraw ? await pressRowAction(forWithdraw.id, copy.withdraw) : "absent";
      let withdrawn = await waitUntil(band, (state) => state !== null && (state.stage === "editing" || (state.stage === "reviewing" && Boolean(state.review))), 20000, 100);
      if (withdrawn.value?.stage === "editing") {
        await pressBand("accept");
        withdrawn = await waitUntil(band, (state) => state?.stage === "reviewing" && Boolean(state.review), 60000, 100);
      }
      const gone = await waitUntil(positions, (now) => !now[stampedId], 15000);
      const forRestore = await findMutationRow(createKey);
      const restoreOffer = forRestore ? await rowActionState(forRestore.id, copy.restore, false) : null;
      const viaRestore = forRestore ? await pressRowAction(forRestore.id, copy.restore) : "absent";
      const back = await waitUntil(positions, (now) => Boolean(now[stampedId]), 20000);
      const afterRestore = await band();
      verdict("m32-withdraw-then-restore-of-the-placed-node-works", viaWithdraw === "action" && gone.ok && viaRestore === "action" && back.ok, { withdraw: { offered: withdrawOffer ? { present: withdrawOffer.present, disabled: withdrawOffer.disabled, reason: withdrawOffer.reason, busy: withdrawOffer.busy } : null, bandBeforeThePress: bandBeforeWithdraw, bandAfterThePress: withdrawn.value?.stage ?? null, waitedMs: withdrawn.waitedMs, notices: (await shownNoticeCodes()).slice(-4), via: viaWithdraw, review: withdrawn.value?.review ?? null, text: withdrawn.value?.text?.slice(0, 160), nodeGoneFromThePreview: gone.ok }, restore: { offered: restoreOffer ? { present: restoreOffer.present, disabled: restoreOffer.disabled } : null, via: viaRestore, nodeBackInThePreview: back.ok, band: afterRestore?.stage ?? null, review: afterRestore?.review ?? null }, reading: "Withdraw on the create mutation's row opens a session whose preview lacks the node; Restore on the same row drops that draft and the node is back" });
      if (await band()) await exitSession();
      const settledC = await waitUntil(positions, (now) => movedIds(headC, now, 1e-6).length === 0 && Boolean(now[stampedId]) && Object.keys(now).length === Object.keys(headC).length, 20000, 500);
      const rowsAfterC = await stableDocumentRows();
      verdict("m32-withdraw-then-restore-leaves-the-document-as-it-was", settledC.ok && rowsAfterC === rowsHeadC, { band: (await band())?.stage ?? null, placedNodeInTheDocument: Boolean(settledC.value[stampedId]), nodes: [nodesHeadC, (await vitals())?.nodes ?? null], positions: [Object.keys(headC).length, Object.keys(settledC.value).length], drift: movedIds(headC, settledC.value, 1e-6).slice(0, 4), rows: [rowsHeadC, rowsAfterC], waitedMs: settledC.waitedMs, reading: "Restore dropped the only draft, so the session is left with nothing to change: the document and its History rows are those from before the Withdraw" });
    }
  }
  await closePanels();
  await setUtility("select");
  await mark("before the regions");
  const home = cameras.filter((row, index) => index > 0 && row.zoom === 1 && row.x === 0 && row.y === 0 && cameras[index - 1].zoom !== 1);
  note("m32-camera-trace", { cameras, cameBackToItsOrigin: home.map((row) => row.when), reading: "O6: the probe zooms in to aim at handles; a camera that is back at (0, 0) zoom 1 at a later mark without the probe zooming out was reset by the shell after a structural change (delete, duplicate, wire, brush)" });
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  for (let attempt = 0; attempt < 8 && cameraOf(await vitals()).zoom > 1.01; attempt++) {
    const box = await paneBox();
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.wheel(0, 300);
    await sleep(900);
  }
  seq = Math.max(seq, await settledNewestSeq());
  const wide = await layout();
  let spot: Point | null = null;
  for (let gy = 140; gy < wide.box.height - 240 && !spot; gy += 40)
    for (let gx = 30; gx < wide.box.width - 200 && !spot; gx += 40) {
      const candidate = { x: wide.box.x + gx, y: wide.box.y + gy };
      const probes = [candidate, { x: candidate.x + 130, y: candidate.y + 100 }, { x: candidate.x + 65, y: candidate.y + 50 }, { x: candidate.x + 130, y: candidate.y }, { x: candidate.x, y: candidate.y + 100 }];
      if (wide.rows.every((row) => probes.every((point) => Math.hypot(row.at.x - point.x, row.at.y - point.y) > 80))) spot = candidate;
    }
  if (!spot) {
    note("m32-not-provokable-no-empty-board-area-for-a-region", { camera: wide.camera, nodes: wide.rows.length });
    return;
  }
  const areaArmed = await setUtility("areaBrush");
  await openHistory();
  await scrollHistory("start");
  await sleep(600);
  const regionsBefore = await targetRegions();
  await page.mouse.move(spot.x, spot.y);
  await page.mouse.down();
  await page.mouse.move(spot.x + 90, spot.y + 70, { steps: 8 });
  await sleep(400);
  const paintMode = (await boardInteraction())?.mode ?? null;
  await page.mouse.up();
  const painted = await waitUntil(targetRegions, (list) => list.length === regionsBefore.length + 1, 8000);
  const region = painted.value.find((row) => !regionsBefore.some((old) => old.id === row.id)) ?? null;
  const seqAtThePaint = seq;
  const shown = await waitUntil(readHistory, (rows) => rows.some((row) => row.kind === "entry" && Number(row.key) > seqAtThePaint && documentEntries([row]).length > 0), 8000, 400);
  const idleCreate = await documentRowsSince(seq, 1, 15000);
  let createRows = idleCreate;
  let flushedBy = idleCreate.documents.length ? "nothing — it arrived while the pointer rested" : "";
  if (!createRows.documents.length) {
    await page.mouse.move(spot.x + 200, spot.y + 150, { steps: 4 });
    createRows = await documentRowsSince(seq, 1, 6000);
    if (createRows.documents.length) flushedBy = "the next pointer move over the board";
  }
  if (!createRows.documents.length) {
    await setUtility("select");
    createRows = await documentRowsSince(seq, 1, 8000);
    if (createRows.documents.length) flushedBy = "arming another utility";
  }
  verdict("m32-creating-a-region-is-one-history-row", areaArmed && painted.ok && region !== null && shown.ok && idleCreate.documents.length === 1 && idleCreate.documents[0].count === 1 && Boolean(idleCreate.documents[0].mutations[0]?.includes(region.id)), { mode: paintMode, region, rowsCountedAfterSequence: seqAtThePaint, listedInTheOpenHistoryPanelUntouched: { ok: shown.ok, waitedMs: shown.waitedMs, reading: "the History panel is open and scrolled to its newest rows before the paint; for 8 s after the release nothing is pressed, moved or scrolled — the new row must show on its own, as it does for a drag" }, rowsWhileThePointerRested: idleCreate.documents.map((row) => ({ row: row.label, mutations: row.mutations })), restedMs: idleCreate.waitedMs, rowsInTheEnd: createRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })), broughtBy: flushedBy || "nothing brought it within 29 s", commandRows: createRows.commands, reading: "`regionCreate`: the area brush's release paints a target region — ONE tool transaction, one History row, committed by the release itself. A region that is on the board while History lists no row until the next input is a release that was not committed" });
  seq = createRows.newest;
  if (!region) return;
  await setUtility("select");
  const createArrived = createRows.documents.length > 0;
  const camR = cameraOf(await vitals());
  const boxR = await paneBox();
  const grip = toScreen([region.x + region.width, region.y + region.height], camR, boxR);
  await page.mouse.move(grip.x - 1, grip.y - 1);
  await sleep(600);
  await page.mouse.down();
  await page.mouse.move(grip.x + 40 * camR.zoom, grip.y + 30 * camR.zoom, { steps: 8 });
  await sleep(400);
  const gripMode = (await boardInteraction())?.mode ?? null;
  await page.mouse.up();
  const resized = await waitUntil(targetRegions, (list) => Math.abs((list.find((row) => row.id === region.id)?.width ?? region.width) - region.width) > 1, 8000);
  const resizeRows = await documentRowsSince(seq, createArrived ? 1 : 2, 15000);
  const now = resized.value.find((row) => row.id === region.id) ?? null;
  verdict("m32-resizing-a-region-is-one-history-row", resized.ok && resizeRows.documents.length === (createArrived ? 1 : 2) && resizeRows.documents.every((row) => row.count === 1 && Boolean(row.mutations[0]?.includes(region.id))), { mode: gripMode, size: { before: [region.width, region.height], after: now ? [now.width, now.height] : null }, rows: resizeRows.documents.map((row) => ({ row: row.label, mutations: row.mutations })), createRowWasListedBefore: createArrived, commandRows: resizeRows.commands, reading: "`regionResize`: a grip drag commits one absolute pose on release — ONE tool transaction, one History row (when the create row had not been listed yet, it arrives with it and both are counted)" });
  await closePanels();
};
//#endregion 🔖️Steps
