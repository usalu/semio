/** 🌿️ Steps 6 to 8: undo, redo, alternatives and the fatal path. */
import { beginDragEdit } from "../🌄️drag-journey/🟦️.ts";
import type { Ctx } from "../🌄️drag-journey/🟦️.ts";
import { verdict, note, log, emit } from "@semio-tech/framework/time-travel/testing/report";
import { placed, positions, movedIds, near, frameBoard, pickCloneSource, vitals, selectionIds, cameraOf, toScreen, paneBox, dragBy, offsetOf } from "../🎲️board/🟦️.ts";
import type { Positions } from "../🎲️board/🟦️.ts";
import { mod, page, currentLocale, stamp, renderer } from "@semio-tech/framework/time-travel/testing/state";
import { prepareChord, installBandTrace, band, pressBand, finalizeDialog, dialogFillName, dialogChoose, mutationRowRevealed, bandOverlapVerdict } from "@semio-tech/framework/time-travel/testing/band";
import { waitUntil, shot, dumpJson, diagnose, sleep, evalSafe } from "@semio-tech/framework/time-travel/testing/page";
import { openHistory, pressAuthored, allHistoryRows, documentEntries, historyInventory, readHistory, newestEntrySeq, expandEntry, pressRowAction } from "@semio-tech/framework/time-travel/testing/history";
import type { HistoryRow } from "@semio-tech/framework/time-travel/testing/history";
import { openPanelTabIds, HISTORY_TAB, closePanels } from "@semio-tech/framework/time-travel/testing/chrome";
import { COPY, dragLabel } from "../🌍️copy/🟦️.ts";
import { readAlternatives, switchAlternative } from "@semio-tech/framework/time-travel/testing/history-session";
import { readEditor, typeEditorNumber, readNumberControl, editor, keyEditorNumber } from "@semio-tech/framework/time-travel/testing/editor";
import { revealWindowRow } from "../🛣️lanes/🟦️.ts";

//#region 🔖️Steps

/** ↩️ Step 6 — Undo takes the finalize back (+80), Redo re-authors it (+120); chord first, then the panel button. */
export const step6 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.afterOverwrite) {
    verdict("precondition-step-5", false, {});
    return;
  }
  const isAt = (dx: number) => async () => placed(await positions(), ctx.a!, ctx.p0![ctx.a!], dx, ctx.dy!);
  const attempt = async (verb: "undo" | "redo", target: number) => {
    const chord = verb === "undo" ? `${mod}+z` : `${mod}+Shift+z`;
    await prepareChord();
    await page.keyboard.press(chord).catch(() => {});
    const byChord = await waitUntil(isAt(target), (ok) => ok, 12000, 400);
    verdict(`${verb}-chord-${verb === "undo" ? "takes-back" : "re-applies"}-the-finalize`, byChord.ok, { chord, a: (await positions())[ctx.a!], expected: [ctx.p0![ctx.a!][0] + target, ctx.p0![ctx.a!][1] + ctx.dy!], waitedMs: byChord.waitedMs });
    if (byChord.ok) return 0;
    const presses: Record<string, unknown>[] = [];
    await openHistory();
    for (let press = 1; press <= 4; press++) {
      const before = await positions();
      const panelsBefore = await openPanelTabIds();
      if (!panelsBefore.includes(HISTORY_TAB)) await openHistory();
      const pressed = await pressAuthored(`framework.history.${verb}`);
      const settled = await waitUntil(isAt(target), (ok) => ok, 8000, 400);
      presses.push({ press, pressed, reached: settled.ok, moved: movedIds(before, await positions()).length, panels: await openPanelTabIds() });
      if (settled.ok) {
        verdict(`${verb}-button-reaches-the-finalize`, press === 1, { presses });
        return press;
      }
    }
    verdict(`${verb}-button-reaches-the-finalize`, false, { presses });
    return -1;
  };
  await closePanels();
  await attempt("undo", ctx.dx!);
  const undone = await positions();
  verdict("undo-restores-plus-80", placed(undone, ctx.a, ctx.p0[ctx.a], ctx.dx!, ctx.dy!) && placed(undone, ctx.b!, ctx.p0[ctx.b!], ctx.dx!, ctx.dy!), { a: undone[ctx.a], b: undone[ctx.b!] });
  await closePanels();
  await attempt("redo", 120);
  const redone = await positions();
  verdict("redo-restores-plus-120", placed(redone, ctx.a, ctx.p0[ctx.a], 120, ctx.dy!) && placed(redone, ctx.b!, ctx.p0[ctx.b!], 120, ctx.dy!), { a: redone[ctx.a] });
  const copy = COPY[currentLocale];
  const rows = await allHistoryRows();
  const labels = documentEntries(rows).map((row) => row.label);
  verdict("undone-and-redone-rows-appear", labels.some((label) => label.startsWith(copy.row("undo", null, 1))) && labels.some((label) => label.startsWith(copy.row("redo", null, 1))), { expected: [copy.row("undo", null, 1), copy.row("redo", null, 1)], labels: labels.slice(-8) });
  await shot("undo-redo");
  await closePanels();
};

/** 🌿️ Step 7 — a second session on dy → New alternative with a name → its row and head; then switch alternatives. */
export const step7 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.dragEntry) {
    verdict("precondition-step-2", false, {});
    return;
  }
  const dy0 = ctx.dy!;
  ctx.dyEdited = Math.round(dy0 + 30);
  ctx.alternative = `probe ${currentLocale} ${stamp.slice(11, 19)}`;
  const first = await editDragAsAlternative(ctx, ctx.dyEdited, ctx.alternative, "");
  if (!first) return;
  await shot("alternative");
  const listed = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(ctx.alternative!)), 15000, 1000);
  verdict("alternatives-section-lists-the-new-alternative", listed.ok, { rows: listed.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${row.active ? " [active]" : ""}`), expectedSelector: "framework.history.alternatives / framework.history.alternative.<id>" });
  if (!listed.value.length) {
    verdict("alternative-switcher-present", false, { inventory: (await historyInventory()).filter((row) => /alternative/i.test(String(row.id))).slice(0, 12), note: "no `framework.history.alternative.<id>` rows in this build" });
    await closePanels();
    return;
  }
  const trunk = listed.value.find((row) => !row.text.includes(ctx.alternative!));
  const expectAt = (dy: number) => async () => placed(await positions(), ctx.a!, ctx.p0![ctx.a!], 120, dy) && placed(await positions(), ctx.b!, ctx.p0![ctx.b!], 120, dy);
  if (trunk) {
    const via = await switchAlternative(trunk);
    const back = await waitUntil(expectAt(dy0), (ok) => ok, 20000, 500);
    verdict("switching-to-the-trunk-shows-original-positions", back.ok, { via, trunk: trunk.text.slice(0, 60), a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + dy0] });
    const mine = (await readAlternatives()).find((row) => row.text.includes(ctx.alternative!));
    const forth = mine ? await switchAlternative(mine) : "absent";
    const edited = await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    verdict("switching-back-shows-the-edited-positions", edited.ok, { via: forth, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
  } else {
    verdict("trunk-listed-after-new-alternative", false, { rows: listed.value.map((row) => row.text.slice(0, 60)), pending: "W1-G: after finalize-as-new-alternative the original trunk is not yet listed — expected-pending" });
    const second = `${ctx.alternative} b`;
    const dy2 = Math.round(dy0 + 60);
    const made = await editDragAsAlternative(ctx, dy2, second, "-2");
    if (!made) return;
    const both = await waitUntil(readAlternatives, (rows) => rows.some((row) => row.text.includes(second)) && rows.some((row) => row.text.includes(ctx.alternative!) && !row.text.includes(second)), 15000, 1000);
    verdict("alternatives-section-lists-both-alternatives", both.ok, { rows: both.value.map((row) => `${row.key}=${row.text.slice(0, 60)}${row.active ? " [active]" : ""}`) });
    const firstRow = both.value.find((row) => row.text.includes(ctx.alternative!) && !row.text.includes(second));
    const viaFirst = firstRow ? await switchAlternative(firstRow) : "absent";
    const atFirst = await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    verdict("switching-to-the-first-alternative-shows-its-positions", atFirst.ok, { via: viaFirst, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + ctx.dyEdited] });
    const secondRow = (await readAlternatives()).find((row) => row.text.includes(second));
    const viaSecond = secondRow ? await switchAlternative(secondRow) : "absent";
    const atSecond = await waitUntil(expectAt(dy2), (ok) => ok, 20000, 500);
    verdict("switching-to-the-second-alternative-shows-its-positions", atSecond.ok, { via: viaSecond, a: (await positions())[ctx.a], expected: [ctx.p0[ctx.a][0] + 120, ctx.p0[ctx.a][1] + dy2] });
    const back = firstRow ? await switchAlternative((await readAlternatives()).find((row) => row.key === firstRow.key) ?? firstRow) : "absent";
    await waitUntil(expectAt(ctx.dyEdited), (ok) => ok, 20000, 500);
    note("left-on-the-first-alternative", { via: back });
  }
  await shot("switched");
  await closePanels();
};

/** 🍀️ One history-edit session on the drag's dy through the real stepper → Accept → Finalize → New alternative `name`;
 * verdicts carry `suffix` so a second alternative reads apart from the first. Answers whether the session committed. */
const editDragAsAlternative = async (ctx: Ctx, dyValue: number, name: string, suffix: string) => {
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  verdict(`session-opens${suffix}`, begun.band?.stage === "editing", { via: begun.via, stage: begun.band?.stage });
  if (begun.band?.stage !== "editing") return false;
  const ed = (await waitUntil(readEditor, (value) => Boolean(value?.numbers.dx && value?.numbers.dy), 15000)).value;
  verdict(`editor-reads-the-effective-input${suffix}`, Boolean(ed?.numbers.dx && near(Number(ed.numbers.dx.value), 120, 0.005)), { dx: ed?.numbers.dx?.value, dy: ed?.numbers.dy?.value, note: "the drag's effective input in the current alternative carries the overwritten dx=120" });
  if (!ed?.numbers.dx) {
    dumpJson(`no-editor${suffix}`, { band: await band(), panels: await openPanelTabIds(), inventory: (await historyInventory()).filter((row) => !/entry\.|mutation\./.test(String(row.id))), rows: (await readHistory()).slice(0, 40) });
    await shot(`no-editor${suffix}`);
  }
  const typed = await typeEditorNumber("dy", dyValue);
  const reachable = verdict(`dy-stepper-control-reachable${suffix}`, typed.present && typed.typed === String(dyValue), { typed, authored: "framework.history.editor.input.dy" });
  if (!reachable) await diagnose(`dy-unreachable${suffix}`);
  const preview = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, dyValue), 20000);
  verdict(`dy-edit-updates-the-preview${suffix}`, preview.ok, { a: preview.value[ctx.a!], expected: [ctx.p0![ctx.a!][0] + 120, ctx.p0![ctx.a!][1] + dyValue] });
  const via = await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict(`review-ready${suffix}`, review.value?.review === "ready", { via, review: review.value?.review, text: review.value?.text?.slice(0, 160) });
  await pressBand("finalize");
  const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
  const nameValue = await dialogFillName(name);
  verdict(`name-field-takes-the-alternative-name${suffix}`, dialog.ok && nameValue === name, { nameValue, submit: dialog.value?.submit });
  await dialogChoose("submit");
  const gone = await waitUntil(band, (b) => b === null, 30000);
  verdict(`new-alternative-closes-the-session${suffix}`, gone.ok, { band: gone.value?.stage ?? null, fault: gone.value?.fault ?? null });
  const expected = COPY[currentLocale].row("edit", name, 1);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict(`alternative-row-appears${suffix}`, rows.ok, { expected, entries: documentEntries(rows.value).map((row) => row.label).slice(0, 6) });
  const head = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], 120, dyValue), 15000);
  verdict(`head-is-on-the-edited-alternative${suffix}`, head.ok, { a: head.value[ctx.a!], expected: [ctx.p0![ctx.a!][0] + 120, ctx.p0![ctx.a!][1] + dyValue] });
  return gone.ok;
};

/** 🚧️ The hard-minimum law of a stepper (design §18, `time_travel_number_limits`) in the open `create-node` session: the
 * `/index` stepper (schema `minimum: 0`, an inclusive hard bound) refuses a typed `-1` visibly, naming the bound ("Must be at
 * least 0"), dispatches nothing (the preview keeps the clone where it was), and Home reaches the bound itself. */
const hardMinimumRefusal = async (clone: string, beforeDrag: Positions) => {
  const copy = COPY[currentLocale];
  await revealWindowRow("framework.history.editor.inputs", "framework.history.editor.input.index.row");
  const before = await readNumberControl("index");
  if (!before) {
    verdict("stepper-below-its-hard-minimum-is-refused-naming-the-bound", false, { reason: "no `framework.history.editor.input.index` stepper in the create-node editor", inputs: (await editor())?.inputs });
    return;
  }
  const typed = await typeEditorNumber("index", -1);
  await sleep(900);
  const refused = await readNumberControl("index");
  const refusal = refused?.refusal ?? (copy.atLeastZero.test(refused?.rowText ?? "") ? (refused?.rowText ?? null) : null);
  verdict("stepper-below-its-hard-minimum-is-refused-naming-the-bound", Boolean(refusal && copy.atLeastZero.test(refusal)), { typed, before: { valueNow: before.valueNow, min: before.min }, refusal, invalid: refused?.invalid, rowText: refused?.rowText, expected: String(copy.atLeastZero), reading: "create-node `/index` (x-semio-ui stepper, schema `minimum: 0`): an inclusive hard bound → `UiNumberLimits` refusal \"Must be at least 0\" / \"Muss mindestens 0 sein\", nothing dispatched" });
  const kept = await positions();
  verdict("stepper-refusal-keeps-the-draft", placed(kept, clone, beforeDrag[clone], 0, 0) && (renderer === "wgpu" || refused?.invalid === true), { clone: kept[clone], beforeDrag: beforeDrag[clone], invalid: refused?.invalid, valueAfter: refused?.valueNow });
  await keyEditorNumber("index", "Home");
  const home = await waitUntil(() => readNumberControl("index"), (control) => control?.valueNow === 0 && !control.invalid, 8000, 300);
  verdict("stepper-home-reaches-the-hard-minimum", home.ok, { valueNow: home.value?.valueNow, invalid: home.value?.invalid, refusal: home.value?.refusal, reading: "design §18: Home = the hard minimum" });
};

/** 💥️ Step 8 — duplicate a node, drag the clone, withdraw the clone's `create-node` → blocked review with an Error row,
 * Finalize disabled; Next problem → withdraw the drag → ready; Exit → zero trace. */
export const step8 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(4);
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const spot = await pickCloneSource(exclude);
  const d = spot?.row;
  if (!spot || !d) {
    verdict("clone-source-with-free-space", false, { reason: "no node whose clone (+24,+24) and drag destination are ≥ 55 world units from every other node" });
    return;
  }
  log(`clone source ${d.id} at ${JSON.stringify(d.world)} drag ${JSON.stringify(spot.move)} zoom=${spot.zoom}`);
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const idsBefore = Object.keys(await positions());
  await page.mouse.click(d.at.x, d.at.y);
  const selected = await waitUntil(vitals, (v) => selectionIds(v).length === 1 && selectionIds(v)[0] === d.id, 15000);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  const grew = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  verdict("duplicate-adds-one-clone", selected.ok && grew.ok && Boolean(clone), { d: d.id, clone, nodes: [nodesBefore, grew.value?.nodes] });
  if (!clone) return;
  const reselected = await waitUntil(vitals, (v) => selectionIds(v).includes(clone), 15000);
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  const cleared = await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  note("clone-deselected-before-its-drag", { cleared: cleared.ok, selection: selectionIds(cleared.value), reading: "board `sole_indirect_handle_hit_idle_selected_node`: while a node with a sole free handle is the only selection, a press inside it resolves to that handle and starts a link drag" });
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  const grab = { x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 };
  await dragBy(grab, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const cloneOffset = offsetOf(beforeDrag, dragged.value, clone);
  verdict("clone-drag-moves-only-the-clone", dragged.ok && movedIds(beforeDrag, dragged.value).every((id) => id === clone), { reselected: reselected.ok, cloneOffset, moved: movedIds(beforeDrag, dragged.value).slice(0, 4), selection: selectionIds(await vitals()), grab, cloneAt });
  const head = dragged.value;
  const added = (await waitUntil(async () => documentEntries(await allHistoryRows()).filter((row) => Number(row.key) > newestBefore), (rows) => rows.length >= 2, 30000, 1000)).value;
  const copy = COPY[currentLocale];
  let createRow: HistoryRow | undefined;
  let dragRow: HistoryRow | undefined;
  for (const entry of added) {
    const expanded = await expandEntry(entry.id);
    createRow ??= expanded.mutations.find((row) => row.label.startsWith(copy.createNode(clone)));
    if (cloneOffset) dragRow ??= expanded.mutations.find((row) => row.label.startsWith(dragLabel(1, cloneOffset[0], cloneOffset[1])));
  }
  verdict("duplicate-and-drag-rows-carry-their-mutations", Boolean(createRow && dragRow), { added: added.map((row) => row.label), create: createRow?.label, drag: dragRow?.label, expectedCreate: copy.createNode(clone) });
  if (!createRow || !dragRow) return;
  const rowsPre = documentEntries(await allHistoryRows()).map((row) => `${row.id}=${row.label}`);
  await installBandTrace();
  const via = await pressRowAction(createRow.id, copy.edit);
  const editing = await waitUntil(band, (b) => b?.stage === "editing", 20000);
  const preview = await waitUntil(positions, (p) => placed(p, clone, beforeDrag[clone], 0, 0), 15000);
  verdict("editing-the-create-node-previews-the-clone-before-its-drag", editing.ok && preview.ok, { via, clone: preview.value[clone], beforeDrag: beforeDrag[clone], head: head[clone], waitedMs: preview.waitedMs });
  await hardMinimumRefusal(clone, beforeDrag);
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  verdict("withdraw-control-reachable", withdraw.present && withdraw.disabled !== true, { withdraw, authored: "framework.history.editor.withdraw(.row)" });
  const withdrawn = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("withdraw-previews-the-document-without-the-clone", withdrawn.ok, { withdraw, hasClone: Boolean(withdrawn.value[clone]) });
  await pressBand("accept");
  const blocked = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("replay-review-is-blocked", blocked.value?.review === "blocked", { review: blocked.value?.review, text: blocked.value?.text?.slice(0, 200), outcome: blocked.value?.outcome });
  verdict("review-line-names-the-blocking-errors", Boolean(blocked.value?.text.includes(copy.reviewBlocked)), { text: blocked.value?.text?.slice(0, 200) });
  const finalize = blocked.value?.controls.find((c) => c.control === "finalize");
  verdict("finalize-disabled-while-blocked", finalize?.disabled === true, { finalize });
  const blockedControls = blocked.value?.controls ?? [];
  const refusedControls = blockedControls.filter((c) => c.disabled);
  verdict("band-offers-next-problem-first-while-blocked", blockedControls[0]?.control === "nextProblem" && blockedControls[0].disabled === false, { controls: blockedControls.map((c) => `${c.control}${c.disabled ? " (refused)" : ""}`), reading: "design §22.2: while the review blocks, the band offers Next problem before every other control (React `data-semio-time-travel-control=nextProblem`, wgpu `shell.time-travel.next-problem`)" });
  verdict("refused-band-controls-stay-reachable-and-name-why", refusedControls.length > 0 && refusedControls.every((c) => c.native !== true && Boolean(c.title)), { refused: refusedControls, reading: "a refused band control is `aria-disabled` with its reason as `aria-describedby` → `[data-slot=row-action-reason]` (wgpu: the mirror node's description), never the native `disabled` attribute, so keyboard and assistive technology reach the refusal" });
  const blockingRead = await waitUntil(() => mutationRowRevealed(dragRow!.key), (read) => read.revealed, 4000, 200);
  const blockingRow = blockingRead.value;
  const blockingWindow = renderer === "wgpu" ? null : await evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>("[data-tree-window-key]")).filter((el) => /framework\.history\.(commands|entry\.\d+)$/.test(el.getAttribute("data-tree-window-key") ?? "")).map((el) => `${el.getAttribute("data-tree-window-key")}: total ${el.getAttribute("data-tree-window-total")}, offset ${el.getAttribute("data-tree-window-offset")}, length ${el.getAttribute("data-tree-window-length")}`).slice(0, 8), [] as string[]);
  verdict("the-first-blocking-row-is-revealed-on-a-blocked-review", blockingRow.revealed, { ...blockingRow, waitedMs: blockingRead.waitedMs, windows: blockingWindow, expected: `framework.history.mutation.${dragRow.key}`, reading: "on the edge into a blocked review the host opens the History panel and scrolls to the first blocking row — read before the probe scrolls anything" });
  await bandOverlapVerdict("band-overlaps-no-other-text-while-blocked");
  await openHistory();
  const failing = (await waitUntil(readHistory, (rows) => rows.some((row) => row.key === dragRow!.key && row.text.includes(copy.targetMissing)), 15000)).value.find((row) => row.key === dragRow!.key);
  verdict("failing-row-reads-error-target-missing", Boolean(failing?.text.includes(copy.targetMissing) && copy.severityError.test(failing.text)), { row: failing?.text?.slice(0, 200), expected: copy.targetMissing });
  await shot("blocked");
  emit({ kind: "inventory", tag: "blocked", rows: await historyInventory(), band: blocked.value });
  const rounds: Record<string, unknown>[] = [];
  for (let round = 0; round < 4 && (await band())?.review === "blocked"; round++) {
    const next: Record<string, unknown> = await pressAuthored("framework.history.timeTravel.nextProblem");
    if (!next.present) {
      await openHistory();
      const problem = (await readHistory()).find((row) => row.kind === "mutation" && copy.severityError.test(row.text));
      next.fallback = problem ? `edit on failing row ${problem.key} via ${await pressRowAction(problem.id, copy.edit)}` : "no failing row";
    }
    const generationBefore = (await band())?.generation;
    const reopened = await waitUntil(band, (b) => b?.stage === "editing", 15000);
    await sleep(800);
    const generationEditing = (await band())?.generation;
    const heading = (await editor())?.heading ?? (await band())?.target;
    const pulled: Record<string, unknown> = await pressAuthored("framework.history.editor.withdraw");
    const marked = await waitUntil(readHistory, (rows) => rows.some((row) => row.key === dragRow!.key && /Withdrawn|Zurückgezogen/.test(row.text)), 8000);
    const dragText = marked.value.find((row) => row.key === dragRow!.key)?.text?.slice(0, 160);
    await pressBand("accept");
    const settled = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
    rounds.push({ round, next, reopened: reopened.ok, generationBefore, generationEditing, heading, pulled, dragMarkedWithdrawn: marked.ok, dragText, review: settled.value?.review, accepted: settled.value?.text?.match(/Accepted changes: \d+|Übernommene Änderungen: \d+/)?.[0] });
  }
  verdict("next-problem-control-reachable", rounds.length > 0 && (rounds[0].next as { present?: boolean } | undefined)?.present === true, { first: rounds[0]?.next, authored: "framework.history.timeTravel.nextProblem(.row)" });
  const ready = await band();
  verdict("withdrawing-the-failing-drag-makes-the-review-ready", ready?.review === "ready", { rounds, review: ready?.review, text: ready?.text?.slice(0, 160) });
  await shot("resolved");
  await pressBand("exit");
  const gone = await waitUntil(band, (b) => b === null, 20000);
  verdict("exit-closes-the-session", gone.ok, { band: gone.value?.stage ?? null });
  const after = await waitUntil(positions, (p) => movedIds(head, p).length === 0, 15000);
  verdict("exit-leaves-head-positions-unchanged", after.ok, { drift: movedIds(head, after.value).slice(0, 8) });
  const seqOf = (row: string) => Number(row.match(/framework\.history\.entry\.(\d+)=/)?.[1] ?? -1);
  const newestPre = Math.max(...rowsPre.map(seqOf));
  const rowsPost = documentEntries(await allHistoryRows()).map((row) => `${row.id}=${row.label}`);
  const addedRows = rowsPost.filter((row) => seqOf(row) > newestPre);
  const relabelled = rowsPost.filter((row) => seqOf(row) <= newestPre && rowsPre.some((pre) => seqOf(pre) === seqOf(row)) && !rowsPre.includes(row));
  verdict("exit-leaves-no-new-rows", addedRows.length === 0 && relabelled.length === 0, { newestPre, added: addedRows.map((row) => row.slice(0, 120)), relabelled: relabelled.map((row) => row.slice(0, 120)) });
  await shot("exited");
  await closePanels();
};
//#endregion 🔖️Steps
