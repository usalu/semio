/** 🖐️ A 5D part's board and world edits survive a folder archive and history traversal. */
import { mkdirSync, readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import type { TimeTravelScenarioV1 } from "@semio-tech/framework/time-travel/testing/scenario";
import { currentLocale, mod, OUT, page, renderer, stamp, expectedReloads, setExpectedReloads, setProbeReloadAt } from "@semio-tech/framework/time-travel/testing/state";
import { closePanels, waitForBoot } from "@semio-tech/framework/time-travel/testing/chrome";
import { attachFolder } from "@semio-tech/framework/time-travel/testing/history-session";
import { prepareChord } from "@semio-tech/framework/time-travel/testing/band";
import { DESKTOP, onFreshPages, use, waitUntil } from "@semio-tech/framework/time-travel/testing/page";
import { verdict } from "@semio-tech/framework/time-travel/testing/report";

const plan = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { schema: string; boardWindow: string; worldWindow: string; boardDrag: [number, number]; worldDrag: [number, number]; grid: number[]; background: [number, number][]; minimumMovement: number };
type Positions = Record<string, [number, number]>;
type WorldPositions = Record<string, [number, number, number]>;
const board = () => page.locator(`[data-slot="window"][id="${plan.boardWindow}"] [data-board-positions-json]`).first();
const world = () => page.locator(`[data-slot="window"][id="${plan.worldWindow}"] [data-instances-json]`).first();
const positions = async (): Promise<Positions> => JSON.parse((await board().getAttribute("data-board-positions-json").catch(() => null)) ?? "{}");
const worldPositions = async (): Promise<WorldPositions> => {
  const instances = JSON.parse((await world().getAttribute("data-instances-json").catch(() => null)) ?? "[]") as { id: string; position: [number, number, number] }[];
  return Object.fromEntries(instances.map((instance) => [instance.id, instance.position]));
};
const worldSelection = async () => {
  const value = await world().getAttribute("data-selection-json").catch(() => null);
  return value === null ? null : JSON.parse(value) as { selectedIds?: string[]; activeObjectId?: string | null };
};
const changed = (a: Positions, b: Positions, id: string) => Boolean(a[id] && b[id] && Math.hypot(a[id][0] - b[id][0], a[id][1] - b[id][1]) > plan.minimumMovement);
const same = (a: Positions, b: Positions) => Object.keys(a).length === Object.keys(b).length && Object.keys(a).every((id) => b[id] && Math.hypot(a[id][0] - b[id][0], a[id][1] - b[id][1]) < 1e-6);
const sameWorld = (a: WorldPositions, b: WorldPositions) => Object.keys(a).length === Object.keys(b).length && Object.keys(a).every((id) => b[id] && Math.hypot(...a[id].map((value, axis) => value - b[id][axis])) < 1e-6);

/** 🧩️ Drives real surfaces and refuses to count a missing projection or gesture as a pass. */
const lifecycle = async () => onFreshPages(async (open) => {
  verdict("5d-plan", plan.schema === "s.puzzle5d.lifecycle.v1" && renderer === "react", { renderer, schema: plan.schema });
  if (renderer !== "react") return;
  const folder = join(OUT, `puzzle5d-lifecycle-${stamp}-${currentLocale}`);
  mkdirSync(folder, { recursive: true });
  const a = await open(DESKTOP, "5d-author");
  if (!a) { verdict("5d-author-boots", false, {}); return; }
  await closePanels();
  const attached = await attachFolder(folder);
  verdict("5d-folder-attached", attached.typed === folder && attached.attachButton, { attached });
  if (attached.typed !== folder || !attached.attachButton) return;
  await closePanels();
  const initial = await waitUntil(positions, (p) => Object.keys(p).length > 0, 30000);
  const box = await board().locator("canvas").first().boundingBox().catch(() => null);
  const camera = JSON.parse((await board().getAttribute("data-board-camera-json").catch(() => null)) ?? "{}");
  const candidate = box && Object.entries(initial.value).map(([id, p]) => ({ id, x: box.x + box.width / 2 + (p[0] - (camera.x ?? 0)) * (camera.zoom ?? 1), y: box.y + box.height / 2 + (p[1] - (camera.y ?? 0)) * (camera.zoom ?? 1) })).find((p) => p.x > box.x + 30 && p.x < box.x + box.width - 30 && p.y > box.y + 30 && p.y < box.y + box.height - 30);
  verdict("5d-board-source-visible", initial.ok && Boolean(candidate), { candidate });
  if (!candidate) return;
  await page.mouse.click(candidate.x, candidate.y);
  await prepareChord();
  await page.keyboard.press(`${mod}+d`);
  const created = await waitUntil(positions, (p) => Object.keys(p).length === Object.keys(initial.value).length + 1, 30000);
  const id = Object.keys(created.value).find((key) => !(key in initial.value));
  verdict("5d-created-part", created.ok && Boolean(id), { id });
  if (!id || !box) return;
  const point = created.value[id];
  const x = box.x + box.width / 2 + (point[0] - (camera.x ?? 0)) * (camera.zoom ?? 1);
  const y = box.y + box.height / 2 + (point[1] - (camera.y ?? 0)) * (camera.zoom ?? 1);
  await page.mouse.move(x, y); await page.mouse.down(); await page.mouse.move(x + plan.boardDrag[0], y + plan.boardDrag[1], { steps: 12 }); await page.mouse.up();
  const boardEdited = await waitUntil(positions, (p) => changed(created.value, p, id), 20000);
  verdict("5d-board-edit", boardEdited.ok, { id, before: created.value[id], after: boardEdited.value[id] });
  if (!boardEdited.ok) return;
  const beforeWorld = await worldPositions();
  const worldBox = await world().locator("canvas").first().boundingBox().catch(() => null);
  verdict("5d-world-and-origin-visible", Boolean(worldBox && beforeWorld[id]), { origin: beforeWorld[id], worldBox });
  if (!worldBox || !beforeWorld[id]) return;
  let hit: { x: number; y: number } | null = null;
  for (const fy of plan.grid) {
    for (const fx of plan.grid) {
      let cleared = false;
      for (const [bx, by] of plan.background) {
        await page.mouse.click(worldBox.x + worldBox.width * bx, worldBox.y + worldBox.height * by);
        const observed = await waitUntil(worldSelection, (selection) => selection !== null && (selection.selectedIds?.length ?? 0) === 0 && !selection.activeObjectId, 1500, 200);
        if (observed.ok) { cleared = true; break; }
      }
      if (!cleared) { verdict("5d-world-selection-cleared", false, { selection: await worldSelection() }); return; }
      const point = { x: worldBox.x + worldBox.width * fx, y: worldBox.y + worldBox.height * fy };
      await page.mouse.click(point.x, point.y);
      const selected = await waitUntil(worldSelection, (selection) => selection?.selectedIds?.includes(id) === true || selection?.activeObjectId === id, 1500, 200);
      if (selected.ok) { hit = point; break; }
    }
    if (hit) break;
  }
  verdict("5d-world-selects-created-part", Boolean(hit), { id, hit });
  if (!hit) return;
  const pane = page.locator(`[data-slot="window"][id="${plan.worldWindow}"]`).first();
  const unfold = pane.locator('[id$=".utilityBar.unfold"]').first();
  if (await unfold.isVisible().catch(() => false)) await unfold.click();
  const transform = pane.locator('[id="transform"]').first();
  const transformVisible = await transform.isVisible().catch(() => false);
  verdict("5d-world-transform-available", transformVisible, {});
  if (!transformVisible) return;
  await transform.click();
  const handles = await waitUntil(async () => JSON.parse((await world().getAttribute("data-gumball-hits")) ?? "[]") as { kind: string; sx: number; sy: number }[], (items) => items.some((item) => item.kind === "moveX") && items.some((item) => item.kind === "origin"), 30000, 500);
  const axis = handles.value.find((item) => item.kind === "moveX"), origin = handles.value.find((item) => item.kind === "origin");
  verdict("5d-world-transform-handle", handles.ok && Boolean(axis && origin), { handles: handles.value });
  if (!axis || !origin) return;
  const dx = axis.sx - origin.sx, dy = axis.sy - origin.sy, length = Math.hypot(dx, dy);
  if (length < 1) { verdict("5d-world-edit", false, { reason: "degenerate projected axis" }); return; }
  const wx = worldBox.x + origin.sx + dx * 0.6, wy = worldBox.y + origin.sy + dy * 0.6;
  await page.mouse.move(wx, wy); await page.mouse.down(); await page.mouse.move(wx + dx / length * plan.worldDrag[0], wy + dy / length * plan.worldDrag[0], { steps: 16 }); await page.mouse.up();
  const worldEdited = await waitUntil(async () => ({ world: await worldPositions(), positions: await positions() }), (p) => Boolean(p.world[id] && Math.hypot(...p.world[id].map((value, axis) => value - beforeWorld[id][axis])) > 1e-6 && changed(boardEdited.value, p.positions, id)), 20000);
  verdict("5d-world-edit", worldEdited.ok, { beforeOrigin: beforeWorld[id], afterOrigin: worldEdited.value.world[id], before: boardEdited.value[id], after: worldEdited.value.positions[id] });
  if (!worldEdited.ok) return;
  const written = await waitUntil(async () => readdirSync(folder, { recursive: true, withFileTypes: true }).some((entry) => entry.isFile()), Boolean, 30000, 1000);
  verdict("5d-archive-written", written.ok, {});
  if (!written.ok) return;
  const b = await open(DESKTOP, "5d-reader");
  if (!b) { verdict("5d-reader-boots", false, {}); return; }
  await closePanels();
  const attachedB = await attachFolder(folder);
  const shared = await waitUntil(async () => ({ board: await positions(), world: await worldPositions() }), (p) => same(worldEdited.value.positions, p.board) && sameWorld(worldEdited.value.world, p.world), 60000, 1000);
  const peerReadback = attachedB.typed === folder && attachedB.attachButton && shared.ok;
  verdict("5d-fresh-peer-readback", peerReadback, { attachedB });
  if (!peerReadback) return;
  use(a);
  setExpectedReloads(expectedReloads + 1); setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 });
  const booted = await waitForBoot("puzzle5d-lifecycle", 60);
  const reattached = await attachFolder(folder);
  const restored = await waitUntil(async () => ({ board: await positions(), world: await worldPositions() }), (p) => same(worldEdited.value.positions, p.board) && sameWorld(worldEdited.value.world, p.world), 60000, 1000);
  verdict("5d-reopened", booted && reattached.typed === folder && reattached.attachButton && restored.ok, { reattached });
  if (!booted || reattached.typed !== folder || !reattached.attachButton || !restored.ok) return;
  await closePanels(); await prepareChord(); await page.keyboard.press(`${mod}+z`);
  const undone = await waitUntil(async () => ({ board: await positions(), world: await worldPositions() }), (p) => same(boardEdited.value, p.board) && sameWorld(beforeWorld, p.world), 30000);
  verdict("5d-reopened-undo", undone.ok, {});
  await prepareChord(); await page.keyboard.press(`${mod}+Shift+z`);
  const redone = await waitUntil(async () => ({ board: await positions(), world: await worldPositions() }), (p) => same(worldEdited.value.positions, p.board) && sameWorld(worldEdited.value.world, p.world), 30000);
  verdict("5d-reopened-redo", redone.ok, {});
});

/** 🎬️ Real React-hosted puzzle 5D lifecycle scenario. */
export const timeTravelScenarioV1: TimeTravelScenarioV1<Record<string, never>> = {
  variant: "puzzle5d", defaultServes: { react: "http://127.0.0.1:6014/", wgpu: "http://127.0.0.1:6114/" }, bootWindowPrefix: "puzzle5d-",
  editorNumbers: [], paletteQuery: { en: "Add Part", de: "Teil hinzufügen" }, stepNumbers: [1], order: [1], steps: { 1: lifecycle }, context: () => ({}),
  prepare: async () => null, beforeStep: async () => {}, afterSteps: async () => {},
  diagnostics: async () => ({ board: await positions(), world: await worldPositions() }),
  exploreInventory: async () => ({ board: await positions(), world: await worldPositions() }),
};
