/** 🧮️ Steps 10 to 15: input controls, further edits, the lock warning and the phone and tablet journeys. */
import { near, vitals, selectionIds, layout, OVERVIEW, frameBoard, pickNodes, positions, movedIds, placed, pickCloneSource, cameraOf, toScreen, paneBox, dragBy, offsetOf, highlightedIds } from "../🎲️board/🟦️.ts";
import type { Positions } from "../🎲️board/🟦️.ts";
import { page, renderer, currentLocale, mod } from "@semio-tech/framework/time-travel/testing/state";
import type { Point, Device } from "@semio-tech/framework/time-travel/testing/state";
import { closePanels, openTab, INSPECTION_TAB, presentKey, textOfKey, inside, resolveDomId, byId } from "@semio-tech/framework/time-travel/testing/chrome";
import { prepareChord, installBandTrace, pressBand, band, otherWay, dialogChoose, bandOverlapVerdict, finalizeDialog } from "@semio-tech/framework/time-travel/testing/band";
import { waitUntil, sleep, evalSafe, shot, diagnose, onFreshPages, dumpJson, PHONE, TABLET } from "@semio-tech/framework/time-travel/testing/page";
import { mirror, wgpuPress, mirrorActivate, chromeHits, WGPU_BAND_CONTROLS } from "@semio-tech/framework/time-travel/testing/wgpu-mirror";
import { log, note, verdict } from "@semio-tech/framework/time-travel/testing/report";
import { beginDragEdit } from "../🌄️drag-journey/🟦️.ts";
import type { Ctx } from "../🌄️drag-journey/🟦️.ts";
import { COPY, labelNumber, dragLabel } from "../🌍️copy/🟦️.ts";
import { newestEntrySeq, allHistoryRows, newMutation, findMutationRow, documentEntries, pressAuthored, openHistory, historyInventory, tapRevealsReason } from "@semio-tech/framework/time-travel/testing/history";
import { beginEditOf, exitSession, finalizeOverwrite } from "@semio-tech/framework/time-travel/testing/history-session";
import { readNumberControl, keyEditorNumber, typeSliderText, readEditor, typeEditorNumber, editor, pressUseSelection, focusRead } from "@semio-tech/framework/time-travel/testing/editor";
import { ariaVerdict } from "@semio-tech/framework/time-travel/testing/aria-oracle";

//#region 🔖️Steps
//#region 🔖️Gaps

type Box = { x: number; y: number; width: number; height: number };

/** 🪐️ The board's pivot math of `rotate-selection` / `scale-selection`: the centroid of `ids`, a point turned by `degrees`
 * about a pivot (counter-clockwise in the board's world frame, the corpus `rotate-and-scale-replay` convention) and a point
 * spread by `factor` from it. */
const centroidOf = (p: Positions, ids: string[]): [number, number] => [ids.reduce((sum, id) => sum + p[id][0], 0) / ids.length, ids.reduce((sum, id) => sum + p[id][1], 0) / ids.length];

const rotatedAbout = (point: [number, number], pivot: [number, number], degrees: number): [number, number] => {
  const radians = (degrees * Math.PI) / 180;
  const [x, y] = [point[0] - pivot[0], point[1] - pivot[1]];
  return [pivot[0] + x * Math.cos(radians) - y * Math.sin(radians), pivot[1] + x * Math.sin(radians) + y * Math.cos(radians)];
};

const scaledAbout = (point: [number, number], pivot: [number, number], factor: number): [number, number] => [pivot[0] + (point[0] - pivot[0]) * factor, pivot[1] + (point[1] - pivot[1]) * factor];

const sitsAt = (p: Positions, id: string, expected: [number, number], eps = 0.05) => Boolean(p[id] && near(p[id][0], expected[0], eps) && near(p[id][1], expected[1], eps));

/** 🖇️ Clears the selection (Escape) and selects `rows` on the board: a click, then Shift-clicks; answers the selection. */
export const selectNodes = async (rows: { id: string; at: Point }[]) => {
  await closePanels();
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => selectionIds(v).length === 0, 4000);
  for (const [index, row] of rows.entries()) {
    if (index > 0) await page.keyboard.down("Shift");
    await page.mouse.click(row.at.x, row.at.y);
    if (index > 0) await page.keyboard.up("Shift");
    await waitUntil(vitals, (v) => selectionIds(v).includes(row.id), 8000);
  }
  return selectionIds(await vitals());
};

/** 🗾️ The current screen rows of `ids` (camera and pane as published now). */
export const rowsOf = async (ids: string[]) => (await layout()).rows.filter((row) => ids.includes(row.id));

/** 🎛️ Runs one arg-carrying app action the way a person reaches it. React: the Overview window's Actions rail (its header toggle
 * `framework.window.<id>.engagement.toggle` → the row `action.<id>` → the staged form → Execute
 * `framework.window.<id>.action.<id>.execute` with the staged defaults); the palette (`mod+p`) lists shell, plugin, app and mode
 * COMMANDS, panels and windows only — a window action (`resolveWindowActions`, flag `inPalette`) lives in its window's rail on
 * both renderers. wgpu: the same rail through the mirror (`framework.window.2dOverview.engagement.toggle` → the treeitem
 * `<window>/framework.section.engagements/action.<id>` → the form's Execute); the rail's keys are calibrated on the first live
 * wgpu run, the form's Execute key is matched by suffix until a board with nodes exists to open one. */
export const runPaletteCommand = async (query: string, label: RegExp, commandId: string) => {
  if (renderer === "wgpu") {
    await closePanels();
    const railRow = async () => (await mirror()).find((node) => node.key.endsWith(`/action.${commandId}`) || node.key === `action.${commandId}`) ?? null;
    if (!(await railRow())) await wgpuPress("framework.window.2dOverview.engagement.toggle");
    const row = await waitUntil(railRow, (node) => node !== null, 10000, 300);
    if (row.value) await mirrorActivate(row.value.key, row.value.window);
    const execute = await waitUntil(async () => (await mirror()).find((node) => node.role === "button" && new RegExp(`(^|[./])action\\.${commandId}\\.execute$|${commandId}[-.]execute$`, "u").test(node.key)) ?? null, (node) => node !== null, 10000, 300);
    const staged = (await mirror()).filter((node) => node.key.includes(`action.${commandId}.arg.`) || node.key.includes(`${commandId}.arg.`)).map((node) => `${node.key.replace(/^.*\.arg\./, "")}=${node.value ?? node.valueNow ?? node.label}`);
    if (execute.value && !execute.value.disabled) await mirrorActivate(execute.value.key, execute.value.window);
    await sleep(600);
    if (await railRow()) await wgpuPress("framework.window.2dOverview.engagement.toggle");
    return { palette: row.ok, item: row.value?.label ?? null, execute: execute.value?.key ?? null, disabled: execute.value?.disabled ?? null, staged, inventory: execute.ok ? null : (await mirror()).filter((node) => /engagement|action\./u.test(node.key)).map((node) => `${node.key}|${node.role}`).slice(0, 40) };
  }
  await closePanels();
  const pane = page.locator(`[data-slot="window"][id="${OVERVIEW}"]`).first();
  const rail = pane.locator('[data-slot="window-action-pane"]').first();
  const toggle = pane.locator('[id$=".engagement.toggle"]').first();
  if (!(await rail.isVisible().catch(() => false))) await toggle.click({ timeout: 4000 }).catch((error) => log(`actions rail toggle failed ${String(error).split("\n")[0]}`));
  const row = pane.locator(`[id="action.${commandId}"]`).first();
  const opened = await waitUntil(() => row.count().catch(() => 0), (count) => count > 0, 30000);
  if (opened.waitedMs > 3000) note("actions-rail-opened-late", { commandId, waitedMs: opened.waitedMs, opened: opened.ok, reading: "the window's Actions rail showed its rows this long after its header toggle was pressed" });
  const itemText = opened.ok ? ((await row.innerText({ timeout: 2000 }).catch(() => null))?.replace(/\s+/g, " ").trim() ?? null) : null;
  if (opened.ok) {
    await row.scrollIntoViewIfNeeded({ timeout: 3000 }).catch(() => {});
    const rowLabel = row.locator('[data-slot="tree-label"]').first();
    await ((await rowLabel.count().catch(() => 0)) ? rowLabel : row).click({ timeout: 4000 }).catch((error) => log(`actions rail item click failed ${String(error).split("\n")[0]}`));
  }
  const execute = pane.locator(`[id$=".action.${commandId}.execute"]`).first();
  const shown = await waitUntil(() => execute.count().catch(() => 0), (count) => count > 0, 10000);
  const executeId = shown.ok ? await execute.getAttribute("id").catch(() => null) : null;
  const staged = await evalSafe((id) => Array.from(document.querySelectorAll<HTMLElement>(`[id^="action.${id}.arg."]`)).filter((el) => !el.id.endsWith(".disclosureLabel")).map((el) => `${el.id.replace(/^.*\.arg\./, "")}=${(el.querySelector("input") as HTMLInputElement | null)?.value ?? el.querySelector('[role="slider"]')?.getAttribute("aria-valuenow") ?? el.querySelector('[role="combobox"]')?.textContent?.trim() ?? el.innerText.replace(/\s+/g, " ").trim().slice(0, 40)}`), [] as string[], commandId);
  const disabled = shown.ok ? await execute.isDisabled({ timeout: 2000 }).catch(() => null) : null;
  if (shown.ok) {
    await execute.scrollIntoViewIfNeeded({ timeout: 3000 }).catch(() => {});
    await execute.click({ timeout: 4000 }).catch((error) => log(`execute click failed ${String(error).split("\n")[0]}`));
    await sleep(400);
  }
  const inventory = shown.ok ? null : await evalSafe(() => Array.from(document.querySelectorAll<HTMLElement>('[data-slot="window-action-pane"] [id]')).map((el) => el.id).slice(0, 60), [] as string[]);
  if (await rail.isVisible().catch(() => false)) await toggle.click({ timeout: 4000 }).catch(() => {});
  return { palette: opened.ok, item: itemText && label.test(itemText) ? itemText : itemText === null ? null : `${itemText} (label mismatch)`, execute: executeId, disabled, staged, inventory };
};

/** 🕛️ The angle detents a dial's ticks name in degrees: radians when they fit one turn (the stored unit), else as given. */
const tickDegrees = (ticks: { snap: number }[]) => ticks.map((tick) => Math.round(Math.abs(tick.snap) <= Math.PI + 1e-6 ? (tick.snap * 180) / Math.PI : tick.snap));

/** 🎡️ Step 10 — G6 input controls and their edits: a rotate command → Edit → the angle dial (ticks 0, ±90, 180°, degrees,
 * Arrow = 1°, PageUp = next detent) → Exit with zero trace; Edit again → PageUp to 180° → Accept → Finalize overwrite → the head
 * and the row read 180°. A scale command → Edit → the factor slider (ticks at its snaps on a log axis; a typed factor beyond the
 * hard bound is refused naming it and the draft is kept) → a typed factor 2 → Accept → Finalize overwrite → head and row read 2. */
export const step10 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const copy = COPY[currentLocale];
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { picked } = await pickNodes(2, [[0, 0]], exclude, 60);
  if (picked.length < 2) {
    verdict("g6-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const ids = picked.map((row) => row.id);
  const selected = await selectNodes(picked);
  verdict("g6-two-nodes-selected", selected.length === 2 && ids.every((id) => selected.includes(id)), { selected, ids });
  const before = await positions();
  const pivot = centroidOf(before, ids);
  const newestRotate = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const rotateRun = await runPaletteCommand(currentLocale === "de" ? "Drehen" : "Rotate", copy.rotateCommand, "rotateSelection");
  const rotated = await waitUntil(positions, (p) => ids.every((id) => sitsAt(p, id, rotatedAbout(before[id], pivot, 90))), 20000);
  verdict("g6-rotate-command-turns-the-selection-by-90-degrees", rotated.ok, { run: rotateRun, a: rotated.value[ids[0]], expected: rotatedAbout(before[ids[0]], pivot, 90), pivot, reading: "palette `mod+p` → Rotate… → command panel form (angle default 90) → Execute; one `rotateSelection` ToolTransaction" });
  await closePanels();
  const rotateRow = await newMutation(newestRotate, (row) => row.label.startsWith(copy.rotate(2, labelNumber(90, currentLocale))));
  verdict("g6-rotate-is-one-row-labelled-from-its-leaf", Boolean(rotateRow.mutation), { entry: rotateRow.entry?.label, mutation: rotateRow.mutation?.label, expected: copy.rotate(2, labelNumber(90, currentLocale)) });
  await shot("rotated");
  if (rotateRow.mutation) {
    await installBandTrace();
    const begun = await beginEditOf(rotateRow.mutation.key);
    verdict("g6-edit-opens-the-rotate-session", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
    if (begun.band?.stage === "editing") {
      const dial = (await waitUntil(() => readNumberControl("angle"), (control) => control !== null, 15000)).value;
      const degrees = tickDegrees(dial?.ticks ?? []);
      verdict("g6-angle-is-a-dial", Boolean(dial?.dial), { dial, reading: "x-semio-ui `widget: dial` → `SliderAppearance::Dial` (the editor builder's Dial arm, S2-W1E S2.4)" });
      if (renderer === "wgpu") note("g6-angle-ticks-are-painted-only", { dial, screenshot: await shot("angle-dial"), reading: "the wgpu mirror projects a slider as `<input type=range>` with `aria-value*`; detent ticks are painted only, so the tick verdict is React's and the detent law is proven by PageUp below" });
      else verdict("g6-angle-dial-shows-ticks-at-0-90-180-degrees", [-180, -90, 0, 90, 180].every((value) => degrees.includes(value)), { ticks: dial?.ticks, degrees, expected: [-180, -90, 0, 90, 180] });
      verdict("g6-angle-reads-degrees", Boolean(dial && dial.valueNow !== null && near(dial.valueNow, 90, 0.01) && /°|deg/.test(dial.valueText ?? "")), { valueNow: dial?.valueNow, valueText: dial?.valueText, min: dial?.min, max: dial?.max, expected: "aria-valuenow 90, aria-valuetext \"90 °\" (displayFactor 180/π, displayUnit deg)" });
      const turnedBy = (value: number) => async () => {
        const p = await positions();
        return ids.every((id) => sitsAt(p, id, rotatedAbout(before[id], pivot, value)));
      };
      await keyEditorNumber("angle", "ArrowUp");
      const up = await waitUntil(turnedBy(91), (ok) => ok, 10000, 300);
      const afterUp = await readNumberControl("angle");
      await keyEditorNumber("angle", "ArrowDown");
      const down = await waitUntil(turnedBy(90), (ok) => ok, 10000, 300);
      verdict("g6-arrow-keys-step-the-angle-by-one-degree", up.ok && down.ok, { up: up.ok, down: down.ok, valueAfterUp: afterUp?.valueNow, a: (await positions())[ids[0]], expectedAtUp: rotatedAbout(before[ids[0]], pivot, 91) });
      await keyEditorNumber("angle", "PageUp");
      const detent = await waitUntil(turnedBy(180), (ok) => ok, 10000, 300);
      const afterPage = await readNumberControl("angle");
      verdict("g6-page-up-goes-to-the-next-detent", detent.ok, { valueAfterPageUp: afterPage?.valueNow, expected: 180, a: (await positions())[ids[0]], expectedA: rotatedAbout(before[ids[0]], pivot, 180), reading: "design §18: PageUp = next detent when snaps exist (90° → 180°), else ±10 steps" });
      await shot("angle-dial");
      const exited = await exitSession();
      const kept = await waitUntil(turnedBy(90), (ok) => ok, 15000, 400);
      verdict("g6-exit-leaves-the-rotation-as-it-was", exited.ok && kept.ok, { band: exited.value?.stage ?? null, a: (await positions())[ids[0]] });
      const again = await beginEditOf(rotateRow.mutation.key);
      if (again.band?.stage === "editing") {
        const dialAgain = await waitUntil(() => readNumberControl("angle"), (control) => control !== null && control.valueNow !== null, 15000, 300);
        if (!dialAgain.ok) await diagnose("rotate-second-session-no-editor");
        const keyed = await keyEditorNumber("angle", "PageUp");
        const half = await waitUntil(turnedBy(180), (ok) => ok, 10000, 300);
        if (!half.ok) await diagnose("rotate-second-session-no-preview");
        const acceptVia = await pressBand("accept");
        const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
        if (!review.ok) await diagnose("rotate-second-session-no-review");
        const finalized = review.value?.review === "ready" ? await finalizeOverwrite() : null;
        const head = await waitUntil(turnedBy(180), (ok) => ok, 15000, 400);
        verdict("g6-rotate-edit-finalizes-the-new-angle", half.ok && review.value?.review === "ready" && Boolean(finalized?.closed) && head.ok, { secondSession: { via: again.via, waitedMs: again.waitedMs, dial: dialAgain.value?.valueNow ?? null, dialWaitedMs: dialAgain.waitedMs, keyed, acceptVia, stageAfterAccept: review.value?.stage ?? null }, preview180: half.ok, review: review.value?.review, finalized, a: head.value ? (await positions())[ids[0]] : null, expected: rotatedAbout(before[ids[0]], pivot, 180), reading: "PageUp to the next detent (180°) → Accept → Finalize → Overwrite: the rotation is replayed about its recorded pivot" });
        const relabelledRow = await waitUntil(() => findMutationRow(rotateRow.mutation!.key), (row) => Boolean(row?.label.startsWith(copy.rotate(2, labelNumber(180, currentLocale)))), 12000, 500);
        const relabelled = relabelledRow.value;
        verdict("g6-rotate-row-reads-the-edited-angle", Boolean(relabelled?.label.startsWith(copy.rotate(2, labelNumber(180, currentLocale)))), { label: relabelled?.label, text: relabelled?.text?.slice(0, 120), waitedMs: relabelledRow.waitedMs, expected: copy.rotate(2, labelNumber(180, currentLocale)) });
        if (!finalized?.closed) await exitSession();
      } else verdict("g6-rotate-edit-finalizes-the-new-angle", false, { reason: "the second rotate session did not open", via: again.via, band: again.band?.text?.slice(0, 160) });
    }
  }
  const beforeScale = await positions();
  const pivotScale = centroidOf(beforeScale, ids);
  const newestScale = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const stillSelected = selectionIds(await vitals());
  if (!ids.every((id) => stillSelected.includes(id))) await selectNodes(await rowsOf(ids));
  const scaleRun = await runPaletteCommand(currentLocale === "de" ? "Skalieren" : "Scale", copy.scaleCommand, "scaleSelection");
  const scaled = await waitUntil(positions, (p) => ids.every((id) => sitsAt(p, id, scaledAbout(beforeScale[id], pivotScale, 1.5))), 20000);
  verdict("g6-scale-command-spreads-the-selection-by-1.5", scaled.ok, { run: scaleRun, a: scaled.value[ids[0]], expected: scaledAbout(beforeScale[ids[0]], pivotScale, 1.5), pivot: pivotScale });
  await closePanels();
  const scaleRow = await newMutation(newestScale, (row) => row.label.startsWith(copy.scale(2, labelNumber(1.5, currentLocale))));
  verdict("g6-scale-is-one-row-labelled-from-its-leaf", Boolean(scaleRow.mutation), { entry: scaleRow.entry?.label, mutation: scaleRow.mutation?.label, expected: copy.scale(2, labelNumber(1.5, currentLocale)) });
  if (!scaleRow.mutation) return;
  await installBandTrace();
  const begun = await beginEditOf(scaleRow.mutation.key);
  verdict("g6-edit-opens-the-scale-session", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const factor = (await waitUntil(() => readNumberControl("factor"), (control) => control !== null, 15000)).value;
  const snaps = (factor?.ticks ?? []).map((tick) => tick.snap);
  const travel = { min: factor?.min ?? 0.1, max: factor?.max ?? 10 };
  const logAt = (value: number) => (100 * Math.log(value / travel.min)) / Math.log(travel.max / travel.min);
  if (renderer === "wgpu") note("g6-factor-ticks-are-painted-only", { factor, screenshot: await shot("factor-slider"), reading: "ticks and their axis positions are painted only on wgpu" });
  else {
    verdict("g6-factor-slider-has-ticks-at-its-snaps", [0.25, 0.5, 1, 2, 4].every((value) => snaps.some((snap) => near(snap, value, 1e-6))), { ticks: factor?.ticks, expected: [0.25, 0.5, 1, 2, 4] });
    verdict("g6-factor-ticks-sit-on-a-log-axis", (factor?.ticks.length ?? 0) >= 5 && (factor?.ticks ?? []).every((tick) => tick.left !== null && near(tick.left, logAt(tick.snap), 1.5)), { ticks: factor?.ticks, expectedLeft: snaps.map((snap) => Number(logAt(snap).toFixed(1))), travel, reading: "`scale: log`: 1 sits at the travel's geometric middle (50 % for 0.1 … 10), never at 9 % as on a linear axis" });
  }
  const draftBefore = await positions();
  const typed = await typeSliderText("factor", "-5");
  await sleep(1200);
  const refused = await readNumberControl("factor");
  const refusal = refused?.refusal ?? (copy.greaterThanZero.test(refused?.rowText ?? "") ? refused?.rowText ?? null : null);
  verdict("g6-out-of-bounds-factor-is-refused-naming-the-bound", Boolean(refusal && copy.greaterThanZero.test(refusal)), { typed, refusal, invalid: refused?.invalid, rowText: refused?.rowText, expected: String(copy.greaterThanZero), reading: "schema `exclusiveMinimum: 0` → `UiNumberBound` refusal in display units" });
  const drift = movedIds(draftBefore, await positions(), 1e-6);
  verdict("g6-refusal-keeps-the-draft", drift.length === 0 && Boolean(refused && factor && refused.valueNow !== null && factor.valueNow !== null && near(refused.valueNow, factor.valueNow, 1e-6)), { valueBefore: factor?.valueNow, valueAfter: refused?.valueNow, drift: drift.slice(0, 6) });
  await shot("factor-refused");
  if (renderer !== "wgpu") await page.keyboard.press("Escape").catch(() => {});
  const doubled = await typeSliderText("factor", "2");
  const spreadBy = (factor: number) => async () => {
    const p = await positions();
    return ids.every((id) => sitsAt(p, id, scaledAbout(beforeScale[id], pivotScale, factor)));
  };
  const spread = await waitUntil(spreadBy(2), (ok) => ok, 15000, 300);
  verdict("g6-typed-factor-previews-the-new-spread", spread.ok, { typed: doubled, a: (await positions())[ids[0]], expected: scaledAbout(beforeScale[ids[0]], pivotScale, 2) });
  await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  const finalized = review.value?.review === "ready" ? await finalizeOverwrite() : null;
  const head = await waitUntil(spreadBy(2), (ok) => ok, 15000, 400);
  verdict("g6-scale-edit-finalizes-the-new-factor", review.value?.review === "ready" && Boolean(finalized?.closed) && head.ok, { review: review.value?.review, finalized, a: (await positions())[ids[0]], expected: scaledAbout(beforeScale[ids[0]], pivotScale, 2) });
  const relabelledRow = await waitUntil(() => findMutationRow(scaleRow.mutation!.key), (row) => Boolean(row?.label.startsWith(copy.scale(2, labelNumber(2, currentLocale)))), 12000, 500);
  const relabelled = relabelledRow.value;
  verdict("g6-scale-row-reads-the-edited-factor", Boolean(relabelled?.label.startsWith(copy.scale(2, labelNumber(2, currentLocale)))), { label: relabelled?.label, text: relabelled?.text?.slice(0, 120), waitedMs: relabelledRow.waitedMs, expected: copy.scale(2, labelNumber(2, currentLocale)) });
  if (!finalized?.closed) await exitSession();
  await closePanels();
};

/** ♻️ Step 11 — keep editing after a clean review: the drag's dy → Accept → `ready` → Begin the downstream drag's dx (from
 * the review) → Discard (back to the review, one accepted draft) → Begin it again → Accept the other way (chord ↔ button) →
 * `ready` with two accepted drafts → Finalize overwrite of both → one row, head carries both. */
export const step11 = async (ctx: Ctx) => {
  if (!ctx.p0 || !ctx.a || !ctx.b || !ctx.c || !ctx.cMutation || !ctx.dragMutation) {
    verdict("precondition-step-2", false, { ctx: Object.keys(ctx) });
    return;
  }
  const copy = COPY[currentLocale];
  const head = await positions();
  await installBandTrace();
  const begun = await beginDragEdit(ctx);
  verdict("keep-editing-first-session-opens", begun.band?.stage === "editing", { via: begun.via, stage: begun.band?.stage });
  if (begun.band?.stage !== "editing") return;
  const ed = (await waitUntil(readEditor, (value) => Boolean(value?.numbers.dx && value?.numbers.dy), 15000)).value;
  const [dx, dy] = [Number(ed?.numbers.dx?.value ?? Number.NaN), Number(ed?.numbers.dy?.value ?? Number.NaN)];
  const dyNew = Math.round(dy + 10);
  const typedDy = await typeEditorNumber("dy", dyNew);
  await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], dx, dyNew), 15000);
  await pressBand("accept");
  const first = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("keep-editing-first-review-is-ready", first.value?.review === "ready", { typedDy, dx, dy, dyNew, review: first.value?.review, text: first.value?.text?.slice(0, 160) });
  if (first.value?.review !== "ready") return;
  const second = await beginEditOf(ctx.cMutation);
  verdict("keep-editing-begins-another-mutation-from-a-ready-review", second.band?.stage === "editing", { via: second.via, row: second.row?.label, band: second.band?.text?.slice(0, 160), reading: "design §4: Begin is legal from Reviewing; the session keeps its accepted draft" });
  if (second.band?.stage !== "editing") return;
  const cEd = (await waitUntil(readEditor, (value) => Boolean(value?.numbers.dx), 15000)).value;
  const cdx = Number(cEd?.numbers.dx?.value ?? Number.NaN);
  const cdxNew = Math.round(cdx + 15);
  const discardedDraft = await typeEditorNumber("dx", cdxNew);
  const reviewedHead = await positions();
  const discardVia = await pressBand("discard");
  const discarded = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 20000, 100);
  const cBack = await waitUntil(positions, (p) => placed(p, ctx.c!, head[ctx.c!], 0, 0), 10000);
  verdict("keep-editing-discard-returns-to-the-review-keeping-the-accepted-draft", discarded.value?.review === "ready" && discarded.value.text.includes(copy.accepted(1)) && cBack.ok, { via: discardVia, typed: discardedDraft, review: discarded.value?.review, text: discarded.value?.text?.slice(0, 200), c: cBack.value[ctx.c], cHead: head[ctx.c], a: reviewedHead[ctx.a], reading: "design §4: Discard returns to the stage the draft began from (Reviewing), the accepted draft stays, the discarded one leaves no trace" });
  const third = await beginEditOf(ctx.cMutation);
  if (third.band?.stage !== "editing") {
    verdict("keep-editing-second-review-is-ready-with-two-accepted-drafts", false, { reason: "the session did not reopen on the downstream drag after Discard", via: third.via, band: third.band?.text?.slice(0, 160) });
    return;
  }
  const typedDx = await typeEditorNumber("dx", cdxNew);
  const acceptVia = await pressBand("accept", otherWay());
  const both = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("keep-editing-second-review-is-ready-with-two-accepted-drafts", both.value?.review === "ready" && both.value.text.includes(copy.accepted(2)), { typedDx, cdx, cdxNew, via: acceptVia, review: both.value?.review, text: both.value?.text?.slice(0, 200), expected: copy.accepted(2), reading: "the second Accept takes the way this locale does not take by default (chord ↔ button), so both are driven" });
  const reviewed = await waitUntil(positions, (p) => placed(p, ctx.a!, ctx.p0![ctx.a!], dx, dyNew) && placed(p, ctx.c!, head[ctx.c!], cdxNew - cdx, 0), 15000);
  verdict("keep-editing-review-shows-both-drafts", reviewed.ok && placed(reviewed.value, ctx.b, ctx.p0[ctx.b], dx, dyNew), { a: reviewed.value[ctx.a], c: reviewed.value[ctx.c], expectedC: head[ctx.c] ? [head[ctx.c][0] + cdxNew - cdx, head[ctx.c][1]] : null });
  await shot("two-drafts");
  const finalized = await finalizeOverwrite();
  const expected = copy.row("edit", null, 2);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expected)), 20000, 1000);
  verdict("keep-editing-finalize-overwrites-both-in-one-row", finalized.closed && rows.ok, { finalized, expected, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  const after = await positions();
  const others = movedIds(head, after).filter((id) => id !== ctx.a && id !== ctx.b && id !== ctx.c);
  verdict("keep-editing-head-carries-both-edits", placed(after, ctx.a, ctx.p0[ctx.a], dx, dyNew) && placed(after, ctx.b, ctx.p0[ctx.b], dx, dyNew) && placed(after, ctx.c, head[ctx.c], cdxNew - cdx, 0) && others.length === 0, { a: after[ctx.a], c: after[ctx.c], others: others.slice(0, 6) });
  await closePanels();
};

/** 🩹️ Step 12 — G3, the fatal loop resolved by EDITING targets: duplicate a node and drag the clone; withdraw the upstream
 * `create-node` → the drag is `mutation.target-missing`, review `blocked` → Next problem opens the drag → select two remaining
 * nodes → Use selection → chips read labels, the board highlights them, the preview moves them → Accept → `ready` → Finalize
 * overwrite → head = the pre-session head without the clone, the two nodes moved by the drag's offset. */
export const step12 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const copy = COPY[currentLocale];
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const spot = await pickCloneSource(exclude);
  if (!spot) {
    verdict("g3-clone-source-with-free-space", false, { reason: "no node whose clone (+24,+24) and drag destination are ≥ 55 world units from every other node" });
    return;
  }
  const d = spot.row;
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const idsBefore = Object.keys(await positions());
  const nodesBefore = (await vitals())?.nodes ?? -1;
  const [source] = await rowsOf([d.id]);
  await selectNodes(source ? [source] : [d]);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`).catch(() => {});
  const grew = await waitUntil(vitals, (v) => (v?.nodes ?? -1) === nodesBefore + 1, 30000);
  const clone = Object.keys(await positions()).find((id) => !idsBefore.includes(id));
  verdict("g3-duplicate-adds-one-clone", grew.ok && Boolean(clone), { d: d.id, clone, nodes: [nodesBefore, grew.value?.nodes] });
  if (!clone) return;
  await prepareChord();
  await page.keyboard.press("Escape").catch(() => {});
  await waitUntil(vitals, (v) => !selectionIds(v).includes(clone), 8000);
  const beforeDrag = await positions();
  const camera = cameraOf(await vitals());
  const cloneAt = toScreen(beforeDrag[clone], camera, await paneBox());
  await dragBy({ x: cloneAt.x + 8 * camera.zoom * Math.SQRT1_2, y: cloneAt.y + 8 * camera.zoom * Math.SQRT1_2 }, spot.move[0] * camera.zoom, spot.move[1] * camera.zoom);
  const dragged = await waitUntil(positions, (p) => offsetOf(beforeDrag, p, clone)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
  const move = offsetOf(beforeDrag, dragged.value, clone);
  verdict("g3-clone-drag-moves-only-the-clone", dragged.ok && movedIds(beforeDrag, dragged.value).every((id) => id === clone), { move, moved: movedIds(beforeDrag, dragged.value).slice(0, 4) });
  if (!move) return;
  const createRow = await newMutation(newestBefore, (row) => row.label.startsWith(copy.createNode(clone)));
  const dragRow = await newMutation(newestBefore, (row) => row.label.startsWith(dragLabel(1, move[0], move[1])));
  verdict("g3-create-and-drag-rows-carry-their-mutations", Boolean(createRow.mutation && dragRow.mutation), { create: createRow.mutation?.label, drag: dragRow.mutation?.label, expected: [copy.createNode(clone), dragLabel(1, move[0], move[1])] });
  if (!createRow.mutation || !dragRow.mutation) return;
  const dragKey = dragRow.mutation.key;
  const head = await positions();
  await closePanels();
  const { picked } = await pickNodes(2, [[move[0], move[1]]], [...exclude, d.id, clone], 60);
  if (picked.length < 2) {
    verdict("g3-two-remaining-nodes-to-pick", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const targets = picked.map((row) => row.id);
  await installBandTrace();
  const begun = await beginEditOf(createRow.mutation.key);
  verdict("g3-edit-opens-the-upstream-create-node", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  const deleted = await waitUntil(positions, (p) => !p[clone], 15000);
  verdict("g3-withdrawing-the-create-node-deletes-the-upstream-node", withdraw.present && deleted.ok, { withdraw });
  await pressBand("accept");
  const blocked = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("g3-the-review-is-blocked", blocked.value?.review === "blocked", { review: blocked.value?.review, text: blocked.value?.text?.slice(0, 200) });
  const failing = (await waitUntil(() => findMutationRow(dragKey), (row) => Boolean(row?.text.includes(copy.targetMissing)), 15000)).value;
  verdict("g3-the-downstream-drag-reads-error-target-missing", Boolean(failing?.text.includes(copy.targetMissing)), { row: failing?.text?.slice(0, 200), expected: copy.targetMissing });
  await shot("g3-blocked");
  const next = await pressAuthored("framework.history.timeTravel.nextProblem");
  const reopened = await waitUntil(band, (b) => b?.stage === "editing", 15000);
  await sleep(800);
  const heading = (await editor())?.heading ?? reopened.value?.target ?? null;
  verdict("g3-next-problem-opens-the-failing-drag", next.present && reopened.ok && Boolean((heading ?? "").includes(dragLabel(1, move[0], move[1]))), { next, heading, band: reopened.value?.text?.slice(0, 160) });
  if (!reopened.ok) return;
  const selected = await selectNodes(await rowsOf(targets));
  verdict("g3-board-selection-works-while-editing", selected.length === 2 && targets.every((id) => selected.includes(id)), { selected, targets, reading: "design §7: interaction (selection) verbs keep working while the document is frozen" });
  await openHistory();
  const used = await pressUseSelection("targets");
  const chips = await waitUntil(readEditor, (value) => (value?.targets?.chips.length ?? 0) === 2, 10000);
  const chipTexts = chips.value?.targets?.chips ?? [];
  verdict("g3-use-selection-replaces-the-targets", Boolean(used) && chips.ok, { used, chips: chipTexts, targets });
  verdict("g3-chips-show-labels-not-raw-ids", chipTexts.length === 2 && chipTexts.every((chip) => chip.trim().length > 0 && !targets.some((id) => chip.includes(id))), { chips: chipTexts, ids: targets, reading: "design §16.4: a chip reads the entity's label (`ArtifactApp::entity_label`), not its raw id" });
  const lit = await waitUntil(vitals, (v) => {
    const shown = highlightedIds(v);
    return shown !== null && shown.length === 2 && targets.every((id) => shown.includes(id));
  }, 10000);
  verdict("g3-board-highlights-the-referenced-nodes", lit.ok, { highlighted: highlightedIds(lit.value), raw: lit.value?.highlighted ?? null, targets, reading: renderer === "wgpu" ? "wgpu: `dumpBoard2d` must publish `highlighted` (the board scene's `highlighted_ids_json`); absent = not observable" : "React `data-board-highlighted-ids-json` (`Board2dScene.highlighted_ids_json` ← `InteractionView::draft_references`)" });
  const preview = await waitUntil(positions, (p) => targets.every((id) => placed(p, id, head[id], move[0], move[1], 0.05)), 15000);
  verdict("g3-preview-moves-the-new-targets", preview.ok, { targets: targets.map((id) => ({ id, at: preview.value[id], expected: head[id] ? [head[id][0] + move[0], head[id][1] + move[1]] : null })) });
  await shot("g3-use-selection");
  await pressBand("accept");
  const ready = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("g3-editing-the-targets-makes-the-review-ready", ready.value?.review === "ready", { review: ready.value?.review, text: ready.value?.text?.slice(0, 200) });
  if (ready.value?.review !== "ready") {
    await exitSession();
    await closePanels();
    return;
  }
  const finalized = await finalizeOverwrite();
  const expectedRow = copy.row("edit", null, 2);
  const rows = await waitUntil(allHistoryRows, (all) => all.some((row) => row.kind === "entry" && row.label.startsWith(expectedRow)), 20000, 1000);
  verdict("g3-finalize-overwrite-closes-the-session", finalized.closed && rows.ok, { finalized, expectedRow, entries: documentEntries(rows.value).map((row) => row.label).slice(-6) });
  const final = await positions();
  const expected: Positions = Object.fromEntries(Object.entries(head).filter(([id]) => id !== clone).map(([id, xy]): [string, [number, number]] => [id, targets.includes(id) ? [xy[0] + move[0], xy[1] + move[1]] : xy]));
  const drift = movedIds(expected, final, 0.05);
  verdict("g3-head-equals-the-expectation", drift.length === 0, { drift: drift.slice(0, 8), clonePresent: Boolean(final[clone]), targets: targets.map((id) => ({ id, at: final[id], expected: expected[id] })) });
  await shot("g3-finalized");
  await closePanels();
};

/** ⚠️ Step 13 — G4, a warning introduced by an upstream edit (S2-W2D corpus `warning-from-an-upstream-edit`, on the live
 * example): lock a node in the Inspection panel, unlock it, drag it together with a second node; withdraw the unlock → the
 * drag skips the locked member, Warning "Partially applied" (new since this edit), review `ready` → Finalize overwrite → the
 * warning stays on the row; the reload check re-reads it after the folder reload. */
export const step13 = async (ctx: Ctx) => {
  await closePanels();
  await frameBoard(6);
  const copy = COPY[currentLocale];
  const exclude = [ctx.a, ctx.b, ctx.c].filter((id): id is string => Boolean(id));
  const { picked } = await pickNodes(2, [[40, 40]], exclude, 60);
  if (picked.length < 2) {
    verdict("g4-two-clickable-nodes", false, { picked: picked.map((row) => row.id) });
    return;
  }
  const [p, q] = picked;
  const lockKey = "puzzle2d-play-inspector.node.locked";
  const newestBefore = newestEntrySeq(await allHistoryRows());
  await selectNodes([p]);
  await openTab(INSPECTION_TAB);
  const shown = await waitUntil(() => presentKey(lockKey), (id) => id !== null, 10000);
  const lockText = await textOfKey(lockKey);
  const lockPress = await pressAuthored(lockKey);
  const locked = await waitUntil(() => textOfKey(lockKey), (text) => /\btrue\b/.test(text), 10000);
  const lockMutation = await newMutation(newestBefore, (row) => row.label.startsWith(copy.changeLocked(p.id)));
  verdict("g4-lock-in-the-inspector-is-a-history-row", shown.ok && locked.ok && Boolean(lockMutation.mutation), { node: p.id, before: lockText, after: locked.value, press: lockPress, mutation: lockMutation.mutation?.label, expected: copy.changeLocked(p.id), via: `${INSPECTION_TAB} → ${lockKey}` });
  const newestAfterLock = newestEntrySeq(await allHistoryRows());
  await openTab(INSPECTION_TAB);
  await pressAuthored(lockKey);
  const unlocked = await waitUntil(() => textOfKey(lockKey), (text) => /\bfalse\b/.test(text), 10000);
  const unlockMutation = await newMutation(newestAfterLock, (row) => row.label.startsWith(copy.changeLocked(p.id)) && row.key !== lockMutation.mutation?.key);
  verdict("g4-unlock-in-the-inspector-is-a-history-row", unlocked.ok && Boolean(unlockMutation.mutation), { after: unlocked.value, mutation: unlockMutation.mutation?.key });
  if (!unlockMutation.mutation) return;
  const selected = await selectNodes(await rowsOf([p.id, q.id]));
  const beforeDrag = await positions();
  const newestBeforeDrag = newestEntrySeq(await allHistoryRows());
  await closePanels();
  const zoom = cameraOf(await vitals()).zoom;
  await dragBy(toScreen(beforeDrag[p.id], cameraOf(await vitals()), await paneBox()), 40 * zoom, 40 * zoom);
  const moved = await waitUntil(positions, (now) => [p.id, q.id].every((id) => offsetOf(beforeDrag, now, id)?.some((v) => Math.abs(v) > 0.5) === true), 20000);
  const offset = offsetOf(beforeDrag, moved.value, p.id);
  verdict("g4-both-nodes-dragged-together", moved.ok && Boolean(offset), { selected, offset, q: offsetOf(beforeDrag, moved.value, q.id) });
  if (!offset) return;
  const dragRow = await newMutation(newestBeforeDrag, (row) => row.label.startsWith(dragLabel(2, offset[0], offset[1])));
  verdict("g4-the-drag-is-one-row", Boolean(dragRow.mutation), { mutation: dragRow.mutation?.label, expected: dragLabel(2, offset[0], offset[1]) });
  if (!dragRow.mutation) return;
  const dragKey = dragRow.mutation.key;
  await installBandTrace();
  const begun = await beginEditOf(unlockMutation.mutation.key);
  verdict("g4-edit-opens-the-upstream-unlock", begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
  if (begun.band?.stage !== "editing") return;
  const withdraw = await pressAuthored("framework.history.editor.withdraw");
  await sleep(800);
  await pressBand("accept");
  const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
  verdict("g4-the-review-is-ready-not-blocked", review.value?.review === "ready", { withdraw, review: review.value?.review, text: review.value?.text?.slice(0, 200), reading: "design §16.1: warnings never block finalizing" });
  verdict("g4-the-band-names-the-warning", review.value?.outcome === "warning" || /Worst outcome: Warning|Schwerstes Ergebnis: Warnung/.test(review.value?.text ?? ""), { outcome: review.value?.outcome, text: review.value?.text?.slice(0, 200) });
  const warned = (await waitUntil(() => findMutationRow(dragKey), (row) => Boolean(row?.text.includes(copy.warningPartial)), 15000)).value;
  verdict("g4-the-drag-row-reads-warning-partially-applied", Boolean(warned?.text.includes(copy.warningPartial)), { row: warned?.text?.slice(0, 200), expected: copy.warningPartial });
  verdict("g4-the-warning-is-marked-new-since-this-edit", Boolean(warned?.text.includes(copy.introduced)), { row: warned?.text?.slice(0, 200), expected: copy.introduced, reading: "design §16.5: `HistoryMutationEntry.introduced`" });
  const replayedHead = await waitUntil(positions, (now) => placed(now, p.id, beforeDrag[p.id], 0, 0, 0.05) && placed(now, q.id, beforeDrag[q.id], offset[0], offset[1], 0.05), 10000, 200);
  const replayed = replayedHead.value;
  verdict("g4-the-replay-skips-the-locked-member", replayedHead.ok, { p: replayed[p.id], pBefore: beforeDrag[p.id], q: replayed[q.id], qExpected: [beforeDrag[q.id][0] + offset[0], beforeDrag[q.id][1] + offset[1]], waitedMs: replayedHead.waitedMs });
  await shot("g4-ready-with-warning");
  const finalized = await finalizeOverwrite();
  verdict("g4-finalize-overwrite-closes-the-session", finalized.closed, finalized);
  const after = (await waitUntil(() => findMutationRow(dragKey), (row) => Boolean(row?.text.includes(copy.warningPartial)), 15000)).value;
  verdict("g4-the-warning-stays-visible-after-finalize", Boolean(after?.text.includes(copy.warningPartial)), { row: after?.text?.slice(0, 200) });
  const final = await positions();
  verdict("g4-the-head-keeps-the-locked-member-in-place", placed(final, p.id, beforeDrag[p.id], 0, 0, 0.05) && placed(final, q.id, beforeDrag[q.id], offset[0], offset[1], 0.05), { p: final[p.id], q: final[q.id] });
  ctx.g4 = { drag: dragKey, p: p.id, q: q.id };
  await shot("g4-finalized");
  await closePanels();
};

/** 🔲️ The page box of the band (`band`) or the open prompt (`dialog`): React's element, wgpu's union of the chrome hit rects
 * that carry its control ids. */
const chromeBox = async (kind: "band" | "dialog"): Promise<Box | null> => {
  if (renderer === "wgpu") {
    const hits = ((await chromeHits()).hits ?? []).filter((hit) => (kind === "band" ? /^(ui\.timeTravel|shell\.time-travel)\./ : /^shell\.dialog\./).test(hit.controlId));
    if (!hits.length) return null;
    const [x0, y0] = [Math.min(...hits.map((hit) => hit.rect[0])), Math.min(...hits.map((hit) => hit.rect[1]))];
    const [x1, y1] = [Math.max(...hits.map((hit) => hit.rect[0] + hit.rect[2])), Math.max(...hits.map((hit) => hit.rect[1] + hit.rect[3]))];
    return { x: x0, y: y0, width: x1 - x0, height: y1 - y0 };
  }
  return page.locator(kind === "band" ? "[data-semio-time-travel]" : '[role="dialog"]').first().boundingBox({ timeout: 3000 }).catch(() => null);
};

/** 📏️ Every visible band control with its page box (React `[data-semio-time-travel-control]`, wgpu the band's hit rects). */
const bandControlBoxes = async (): Promise<{ control: string; box: Box | null }[]> => {
  if (renderer === "wgpu") {
    const hits = (await chromeHits()).hits ?? [];
    return WGPU_BAND_CONTROLS.map(([control, key]) => ({ control, hit: hits.find((hit) => hit.controlId === key) })).filter((entry) => entry.hit).map((entry) => ({ control: entry.control, box: { x: entry.hit!.rect[0], y: entry.hit!.rect[1], width: entry.hit!.rect[2], height: entry.hit!.rect[3] } }));
  }
  const controls = page.locator("[data-semio-time-travel-control]");
  const rows: { control: string; box: Box | null }[] = [];
  for (let index = 0; index < (await controls.count().catch(() => 0)); index++) {
    const control = controls.nth(index);
    rows.push({ control: (await control.getAttribute("data-semio-time-travel-control").catch(() => null)) ?? "", box: await control.boundingBox({ timeout: 2000 }).catch(() => null) });
  }
  return rows;
};

/** 🖼️ Whether `box` lies inside the page viewport. */
const insideViewport = (box: Box | null) => {
  const size = page.viewportSize() ?? { width: 0, height: 0 };
  return Boolean(box && box.width > 0 && box.height > 0 && box.x >= -0.5 && box.y >= -0.5 && box.x + box.width <= size.width + 0.5 && box.y + box.height <= size.height + 0.5);
};

/** 🤏️ Taps a band control with a touch (React: the control element; wgpu: the touchscreen at its chrome hit rect). */
const tapBand = async (control: string) => {
  if (renderer === "wgpu") {
    const key = WGPU_BAND_CONTROLS.find(([name]) => name === control)?.[1];
    const hit = ((await chromeHits()).hits ?? []).find((row) => row.controlId === key);
    if (!hit) return (await wgpuPress(key ?? control)) ?? "absent";
    await page.touchscreen.tap(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
    return `tap:${hit.controlId}`;
  }
  return page.locator(`[data-semio-time-travel-control="${control}"]`).first().tap({ timeout: 4000 }).then(() => "tap").catch((error) => `tap failed ${String(error).split("\n")[0]}`);
};

/** 🫵️ Taps the open prompt's destructive Overwrite with a touch. */
const tapOverwrite = async () => {
  if (renderer === "wgpu") {
    const hit = ((await chromeHits()).hits ?? []).find((row) => /^shell\.dialog\.[^.]+\.choice\.overwrite$/.test(row.controlId));
    if (!hit) return (await dialogChoose("overwrite")) ? "mirror" : "absent";
    await page.touchscreen.tap(hit.rect[0] + hit.rect[2] / 2, hit.rect[1] + hit.rect[3] / 2);
    return `tap:${hit.controlId}`;
  }
  return page.locator('[id="ui.dialog.choice.overwrite"]').first().tap({ timeout: 4000 }).then(() => "tap").catch((error) => `tap failed ${String(error).split("\n")[0]}`);
};

/** 🌀️ Tab and Shift+Tab inside the open prompt: focus must stay in it (a modal traps focus) and reach at least two stops. */
const tabsStayInThePrompt = async () => {
  const trail: string[] = [];
  let inside = true;
  for (const key of ["Tab", "Tab", "Tab", "Shift+Tab", "Shift+Tab"]) {
    await page.keyboard.press(key).catch(() => {});
    await sleep(250);
    const focus = await focusRead();
    trail.push(`${key}→${focus.key ?? focus.tag}`);
    inside &&= focus.inDialog;
  }
  return { ok: inside && new Set(trail.map((row) => row.split("→")[1])).size >= 2, trail };
};

/** 🤳️ Steps 14 and 15 — G13 phone and tablet: a fresh touch context (its own document) at the device's viewport; one tapped and
 * dragged node → Edit → the band inside the viewport with touch-sized controls and no structural ARIA finding, the editor's dx
 * typed by keyboard, Accept by touch, the prompt inside the viewport, keyboard-trapped and ARIA-clean, Overwrite by touch; no
 * horizontal page scroll. Verdicts carry the device key (`mobile-…`, `tablet-…`). */
const deviceJourney = (device: Device) => async (_ctx: Ctx) =>
  onFreshPages(async (open) => {
    const copy = COPY[currentLocale];
    const key = device.key;
    const opened = await open(device, key);
    verdict(`${key}-boots-at-${device.viewport.width}-px`, opened !== null, { viewport: page.viewportSize() });
    if (!opened) {
      dumpJson(`${key}-boot-failure`, { inventory: await historyInventory(), body: await evalSafe(() => document.body?.innerText.slice(0, 400) ?? "", "") });
      await shot(`${key}-boot-failure`);
      return;
    }
    const width = await evalSafe(() => ({ scroll: document.documentElement.scrollWidth, inner: innerWidth }), { scroll: -1, inner: -1 });
    verdict(`${key}-no-horizontal-page-scroll`, width.scroll >= 0 && width.scroll <= width.inner + 1, width);
    const board = await waitUntil(vitals, (v) => (v?.nodes ?? 0) > 0, 30000);
    await closePanels();
    await frameBoard(2);
    const { camera, picked } = await pickNodes(1, [[40, 0]], [], 40);
    if (!board.ok || !picked.length) {
      verdict(`${key}-one-clickable-node`, false, { vitals: board.value, picked: picked.length });
      return;
    }
    const m = picked[0];
    const newestBefore = newestEntrySeq(await allHistoryRows());
    await closePanels();
    const under = renderer === "wgpu" ? null : await evalSafe((at) => {
      const el = document.elementFromPoint(at.x, at.y);
      return el ? `${el.tagName.toLowerCase()} in ${el.closest("[data-slot]")?.getAttribute("data-slot") ?? "no slot"} / ${el.closest('[data-slot="window"], [data-slot="panel"]')?.id ?? "no window"}` : null;
    }, null as string | null, m.at);
    await page.touchscreen.tap(m.at.x, m.at.y);
    const tapped = await waitUntil(vitals, (v) => selectionIds(v).includes(m.id), 10000);
    const p0 = await positions();
    await dragBy(m.at, 40 * camera.zoom, 0);
    const moved = await waitUntil(positions, (p) => offsetOf(p0, p, m.id)?.some((v) => Math.abs(v) > 0.5) === true, 20000);
    const offset = offsetOf(p0, moved.value, m.id);
    verdict(`${key}-tap-selects-and-a-drag-moves-the-node`, tapped.ok && moved.ok, { node: m.id, offset, selection: selectionIds(tapped.value), tapWaitedMs: tapped.waitedMs, underTheTap: under, reading: "`underTheTap` is the element at the tap point just before the tap: a tap that selects nothing while a panel still covers the node is the probe's timing, one on the board canvas a dropped tap" });
    if (!offset) return;
    const row = await newMutation(newestBefore, (entry) => entry.label.startsWith(dragLabel(1, offset[0], offset[1])));
    if (!row.mutation) {
      verdict(`${key}-the-drag-is-a-row`, false, { expected: dragLabel(1, offset[0], offset[1]) });
      return;
    }
    const begun = await beginEditOf(row.mutation.key);
    verdict(`${key}-edit-opens-the-band`, begun.band?.stage === "editing", { via: begun.via, band: begun.band?.text?.slice(0, 160) });
    if (begun.band?.stage !== "editing") return;
    await shot(`${key}-editing`);
    const bandBox = await chromeBox("band");
    verdict(`${key}-band-inside-the-viewport`, insideViewport(bandBox), { bandBox, viewport: page.viewportSize() });
    const controls = await bandControlBoxes();
    verdict(`${key}-band-controls-are-touch-sized`, controls.length > 0 && controls.every((entry) => entry.box !== null && entry.box.width >= 24 && entry.box.height >= 24 && insideViewport(entry.box)), { controls, reading: "WCAG 2.5.8: a target is at least 24 × 24 CSS px" });
    await bandOverlapVerdict(`${key}-band-overlaps-no-other-text`);
    await ariaVerdict(`${key}-aria-band-and-history-have-no-structural-findings`, "session");
    const dxNew = Math.round(offset[0] + 20);
    const typed = await typeEditorNumber("dx", dxNew);
    const previewed = await waitUntil(positions, (p) => placed(p, m.id, p0[m.id], dxNew, offset[1], 0.05), 15000);
    const dxId = renderer === "wgpu" ? null : await resolveDomId("framework.history.editor.input.dx");
    const dxBox = dxId ? await byId(dxId).boundingBox({ timeout: 2000 }).catch(() => null) : null;
    verdict(`${key}-editor-input-reachable-by-keyboard`, typed.present && previewed.ok && (renderer === "wgpu" || insideViewport(dxBox)), { typed, dxBox, a: previewed.value[m.id] });
    if (renderer === "wgpu") note(`${key}-a-tap-on-a-refused-edit-tells-its-reason`, { reading: "wgpu paints its tooltip; the mirror description carries the reason (step 4)" });
    else {
      const edited = typed.present ? await findMutationRow(row.mutation.key) : null;
      const tapped = edited ? await tapRevealsReason(edited.id, copy.edit) : null;
      verdict(`${key}-a-tap-on-a-refused-edit-tells-its-reason`, Boolean(tapped?.tapped && tapped.disabled && tapped.shown?.includes(copy.refusalBlocked)), { row: edited?.label ?? null, tapped, expected: copy.refusalBlocked, reading: "W1E-1 on touch: with a changed draft open every Edit is refused (Blocked); a tap on one reveals its reason (`[data-slot=row-action-reason][data-revealed]`)" });
    }
    const accept = await tapBand("accept");
    const review = await waitUntil(band, (b) => b?.stage === "reviewing" && Boolean(b.review), 60000, 100);
    verdict(`${key}-accept-by-touch-reviews-ready`, review.value?.review === "ready", { accept, review: review.value?.review, text: review.value?.text?.slice(0, 160) });
    const finalize = await tapBand("finalize");
    const dialog = await waitUntil(finalizeDialog, (d) => d !== null, 15000);
    const dialogBox = await chromeBox("dialog");
    verdict(`${key}-finalize-prompt-inside-the-viewport`, dialog.ok && insideViewport(dialogBox) && Boolean(dialog.value?.title.includes(copy.dialogTitle)), { finalize, dialogBox, title: dialog.value?.title });
    await shot(`${key}-prompt`);
    await ariaVerdict(`${key}-aria-finalize-prompt-has-no-structural-findings`, "prompt");
    const tabs = await tabsStayInThePrompt();
    verdict(`${key}-finalize-prompt-is-keyboard-reachable`, tabs.ok, tabs);
    const overwrite = await tapOverwrite();
    const gone = await waitUntil(band, (b) => b === null, 30000);
    const head = await positions();
    verdict(`${key}-overwrite-by-touch-closes-the-session`, gone.ok && placed(head, m.id, p0[m.id], dxNew, offset[1], 0.05), { overwrite, band: gone.value?.stage ?? null, a: head[m.id] });
    await shot(`${key}-done`);
  });

export const step14 = deviceJourney(PHONE);

export const step15 = deviceJourney(TABLET);
//#endregion 🔖️Gaps
//#endregion 🔖️Steps
