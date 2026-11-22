/** 🧊️ Puzzle 3D browser lifecycle using the real React host and local archive. */
import { join } from "node:path";
import { mkdirSync, readFileSync } from "node:fs";
import type { TimeTravelScenarioV1 } from "@semio-tech/framework/time-travel/testing/scenario";
import { OUT, stamp, currentLocale, mod, page, renderer, expectedReloads, setExpectedReloads, setProbeReloadAt } from "@semio-tech/framework/time-travel/testing/state";
import { attachFolder } from "@semio-tech/framework/time-travel/testing/history-session";
import { closePanels, waitForBoot } from "@semio-tech/framework/time-travel/testing/chrome";
import { prepareChord } from "@semio-tech/framework/time-travel/testing/band";
import { DESKTOP, onFreshPages, waitUntil } from "@semio-tech/framework/time-travel/testing/page";
import { verdict } from "@semio-tech/framework/time-travel/testing/report";

const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { schema: string; variant: string; world: string; canvas: string; selection: string; palette: Record<"en" | "de", string>; grid: number[]; addedObjects: number; clipboardOffset: [number, number, number]; dragPixels: number; minMovement: number; pickTimeoutMs: number; clearTimeoutMs: number; clearExample: Record<"en" | "de", string>; clearPoints: [number, number][] };
type Instance = { id: string; position?: number[]; rotation?: number[]; scale?: number[] };
const instances = async (): Promise<Instance[]> => page.evaluate((selector) => {
  const value = document.querySelector(selector)?.getAttribute("data-instances-json");
  if (!value) return [];
  const rows = JSON.parse(value) as Instance[];
  return rows.map(({ id, position, rotation, scale }) => ({ id, position, rotation, scale })).sort((a, b) => a.id.localeCompare(b.id));
}, fixture.world);
const selection = async (): Promise<string[]> => page.evaluate((selector) => {
  const value = document.querySelector(selector)?.getAttribute("data-selection-json");
  if (!value) return [];
  const parsed = JSON.parse(value) as { ids?: string[] };
  return parsed.ids ?? [];
}, fixture.selection);
const same = (left: Instance[], right: Instance[]) => JSON.stringify(left) === JSON.stringify(right);

/** 💾️ Creates and selects a spatial object, then checks fresh-peer, reload and history. */
const lifecycle = async () => {
  verdict("puzzle3d-fixture", fixture.schema === "s.puzzle3d.lifecycle.v1" && (currentLocale === "en" || currentLocale === "de"), { schema: fixture.schema, locale: currentLocale });
  if (renderer !== "react") { verdict("puzzle3d-react-host", false, { renderer }); return; }
  const folder = join(OUT, "puzzle3d-" + stamp + "-" + currentLocale);
  mkdirSync(folder, { recursive: true });
  await closePanels();
  const attached = await attachFolder(folder);
  verdict("puzzle3d-archive-attached", attached.typed === folder && attached.attachButton, { attached });
  if (attached.typed !== folder || !attached.attachButton) return;
  await closePanels();
  const boot = await waitUntil(instances, (rows) => rows.length > 0, 120000, 500);
  verdict("puzzle3d-world-booted", boot.ok, { count: boot.value.length });
  if (!boot.ok) return;
  await prepareChord();
  await page.keyboard.press(mod + "+p");
  const clearInput = page.locator("[role=dialog] [data-slot=command-input]").first();
  await clearInput.waitFor({ state: "visible", timeout: 15000 });
  await clearInput.fill(fixture.clearExample[currentLocale]);
  await page.locator("[data-slot=command-item]").filter({ hasText: fixture.clearExample[currentLocale] }).first().click({ timeout: 15000 });
  const confirmation = page.locator("[role=alertdialog] button, [data-slot=dialog-content] button").filter({ hasText: /^(confirm|replace|bestätigen|ersetzen)$/i }).first();
  await confirmation.waitFor({ state: "visible", timeout: 3000 }).catch(() => undefined);
  if (await confirmation.isVisible()) await confirmation.click();
  const blank = await waitUntil(instances, (rows) => rows.length === 0, 90000, 500);
  verdict("puzzle3d-blank-document", blank.ok, { count: blank.value.length });
  if (!blank.ok) return;
  await prepareChord();
  await page.keyboard.press(mod + "+p");
  const input = page.locator("[role=dialog] [data-slot=command-input]").first();
  await input.waitFor({ state: "visible", timeout: 15000 });
  await input.fill(fixture.palette[currentLocale]);
  const command = page.locator("[data-slot=command-item]").filter({ hasText: fixture.palette[currentLocale] }).first();
  await command.click({ timeout: 15000 });
  const dialog = page.locator("[data-slot=dialog-content]").filter({ has: page.locator("#objectKind") }).first();
  await dialog.waitFor({ state: "visible", timeout: 15000 });
  verdict("puzzle3d-add-dialog", await dialog.isVisible(), { locale: currentLocale });
  const select = dialog.locator("#objectKind, [data-slot=select-trigger]").first();
  await select.click();
  await page.locator("[role=option]").first().click();
  await dialog.locator("button").filter({ hasText: /^(add|hinzufügen)/i }).first().click();
  const created = await waitUntil(instances, (rows) => rows.length === blank.value.length + fixture.addedObjects, 90000, 500);
  const priorIds = new Set(blank.value.map((row) => row.id));
  const added = created.value.filter((row) => !priorIds.has(row.id));
  const createdId = added.length === 1 ? added[0]?.id : undefined;
  verdict("puzzle3d-created", created.ok && !!createdId, { before: blank.value.length, after: created.value.length, createdId });
  if (!created.ok || !createdId) return;
  await closePanels();
  const canvas = page.locator(fixture.canvas).first(), box = await canvas.boundingBox();
  let pickedAt: { x: number; y: number } | undefined;
  if (box) for (const y of fixture.grid) { for (const x of fixture.grid) {
    let cleared = false;
    for (const [cx, cy] of fixture.clearPoints) {
      await page.mouse.click(box.x + box.width * cx, box.y + box.height * cy);
      if ((await waitUntil(selection, (ids) => ids.length === 0, fixture.clearTimeoutMs, 200)).ok) { cleared = true; break; }
    }
    if (!cleared) { verdict("puzzle3d-clear-selection", false, { selected: await selection() }); return; }
    const point = { x: box.x + box.width * x, y: box.y + box.height * y };
    await page.mouse.click(point.x, point.y);
    const result = await waitUntil(selection, (ids) => ids.includes(createdId), fixture.pickTimeoutMs, 200);
    if (result.ok) { pickedAt = point; break; }
  } if (pickedAt) break; }
  verdict("puzzle3d-spatial-pick", pickedAt !== undefined, { createdId, selected: await selection() });
  if (!pickedAt || !box) return;
  await prepareChord();
  await page.keyboard.press(mod + "+c");
  await page.keyboard.press(mod + "+v");
  const pasted = await waitUntil(instances, (rows) => rows.length === created.value.length + 1, 15000, 200);
  const original = created.value.find((row) => row.id === createdId);
  const clone = pasted.value.find((row) => row.id !== createdId);
  const expectedPosition = original?.position?.map((value, index) => value + fixture.clipboardOffset[index]!);
  verdict("puzzle3d-copy-paste-relative-position", pasted.ok && !!expectedPosition && clone?.position?.every((value, index) => Math.abs(value - expectedPosition[index]!) < fixture.minMovement) === true, { original, clone, expectedPosition });
  if (!pasted.ok) return;
  const selectedClone = await waitUntil(selection, (ids) => clone !== undefined && ids.includes(clone.id), 10000, 200);
  verdict("puzzle3d-pasted-object-selected", selectedClone.ok, { selected: selectedClone.value, clone: clone?.id });
  await prepareChord();
  await page.keyboard.press(mod + "+z");
  const restoredClipboard = await waitUntil(instances, (rows) => same(rows, created.value), 15000, 200);
  verdict("puzzle3d-paste-undo", restoredClipboard.ok, { count: restoredClipboard.value.length });
  if (!restoredClipboard.ok) return;
  await page.mouse.click(pickedAt.x, pickedAt.y);
  const selectedOriginal = await waitUntil(selection, (ids) => ids.includes(createdId), fixture.pickTimeoutMs, 200);
  if (!selectedOriginal.ok) { verdict("puzzle3d-reselect-after-paste-undo", false, { selected: selectedOriginal.value }); return; }
  await prepareChord();
  await page.keyboard.press(mod + "+x");
  const cut = await waitUntil(instances, (rows) => rows.length === blank.value.length && rows.every((row) => row.id !== createdId), 15000, 200);
  verdict("puzzle3d-cut-removes-object", cut.ok, { createdId, count: cut.value.length });
  if (!cut.ok) return;
  await prepareChord();
  await page.keyboard.press(mod + "+z");
  const restoredCut = await waitUntil(instances, (rows) => same(rows, created.value), 15000, 200);
  verdict("puzzle3d-cut-undo", restoredCut.ok, { count: restoredCut.value.length });
  if (!restoredCut.ok) return;
  await page.mouse.click(pickedAt.x, pickedAt.y);
  const selectedAfterCut = await waitUntil(selection, (ids) => ids.includes(createdId), fixture.pickTimeoutMs, 200);
  if (!selectedAfterCut.ok) { verdict("puzzle3d-reselect-after-cut-undo", false, { selected: selectedAfterCut.value }); return; }
  const unfold = page.locator('[id="framework.window.puzzle3dMainPerspective.utilityBar.unfold"]').first();
  if (await unfold.isVisible()) await unfold.click();
  const transform = page.locator('[id="puzzle3d-main-perspective"] #transform').first();
  await transform.waitFor({ state: "visible", timeout: 30000 });
  await transform.click();
  const handles = await waitUntil(async () => page.evaluate((selector) => {
    const value = document.querySelector(selector)?.getAttribute("data-gumball-hits");
    return value ? JSON.parse(value) as { kind: string; sx: number; sy: number }[] : [];
  }, fixture.world), (hits) => hits.some((hit) => hit.kind === "moveX") && hits.some((hit) => hit.kind === "origin"), 30000, 500);
  const handle = handles.value.find((hit) => hit.kind === "moveX"), origin = handles.value.find((hit) => hit.kind === "origin");
  verdict("puzzle3d-transform-handle", handles.ok && !!handle && !!origin, { handles: handles.value });
  if (!handle || !origin) return;
  const dx = handle.sx - origin.sx, dy = handle.sy - origin.sy, length = Math.hypot(dx, dy);
  if (length < 1) { verdict("puzzle3d-edit", false, { reason: "degenerate projected axis" }); return; }
  await page.mouse.move(box.x + origin.sx + dx * 0.6, box.y + origin.sy + dy * 0.6);
  await page.mouse.down();
  const beforePreview = await canvas.screenshot();
  await page.mouse.move(box.x + origin.sx + dx * 0.6 + dx / length * fixture.dragPixels, box.y + origin.sy + dy * 0.6 + dy / length * fixture.dragPixels, { steps: 16 });
  await page.evaluate(() => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
  const duringPreview = await canvas.screenshot();
  const committedDuringPreview = await instances();
  await page.mouse.up();
  const changedPixels = !beforePreview.equals(duringPreview);
  const documentStable = same(committedDuringPreview, created.value);
  verdict("puzzle3d-transform-preview-before-release", changedPixels && documentStable, { changedPixels, documentStable });
  const prior = created.value.find((row) => row.id === createdId);
  const edited = await waitUntil(instances, (rows) => {
    const from = prior?.position;
    const to = rows.find((row) => row.id === createdId)?.position;
    return !!from && !!to && to[0]! - from[0]! > fixture.minMovement
      && Math.abs(to[1]! - from[1]!) < fixture.minMovement && Math.abs(to[2]! - from[2]!) < fixture.minMovement;
  }, 90000, 500);
  verdict("puzzle3d-edit", edited.ok, { createdId, before: prior?.position, after: edited.value.find((row) => row.id === createdId)?.position });
  if (!edited.ok) return;
  let saved = false;
  await onFreshPages(async (open) => {
    const peer = await open(DESKTOP, "puzzle3d-archive-readback");
    if (!peer) { verdict("puzzle3d-fresh-peer", false, { booted: false }); return; }
    await closePanels();
    const peerAttachment = await attachFolder(folder);
    const readback = await waitUntil(instances, (rows) => same(rows, edited.value), 90000, 1000);
    saved = peerAttachment.typed === folder && peerAttachment.attachButton && readback.ok;
    verdict("puzzle3d-fresh-peer", saved, { attached: peerAttachment, count: readback.value.length });
  });
  if (!saved) return;
  setExpectedReloads(expectedReloads + 1);
  setProbeReloadAt(Date.now());
  await page.reload({ waitUntil: "domcontentloaded", timeout: 60000 });
  const booted = await waitForBoot("puzzle3d-reload", 60);
  const reattached = await attachFolder(folder);
  const restored = await waitUntil(instances, (rows) => same(rows, edited.value), 90000, 1000);
  verdict("puzzle3d-reloaded", booted && reattached.typed === folder && reattached.attachButton && restored.ok, { reattached, count: restored.value.length });
  if (!restored.ok) return;
  await closePanels();
  await prepareChord();
  await page.keyboard.press(mod + "+z");
  const undone = await waitUntil(instances, (rows) => same(rows, created.value), 60000, 500);
  verdict("puzzle3d-undo-after-reload", undone.ok, { count: undone.value.length });
  await prepareChord();
  await page.keyboard.press(mod + "+Shift+z");
  const redone = await waitUntil(instances, (rows) => same(rows, edited.value), 60000, 500);
  verdict("puzzle3d-redo-after-reload", redone.ok, { count: redone.value.length });
};

/** 🎬️ One explicitly localized lifecycle for the real puzzle 3D app. */
export const timeTravelScenarioV1: TimeTravelScenarioV1 = {
  variant: fixture.variant,
  defaultServes: { react: "http://127.0.0.1:6013/", wgpu: "http://127.0.0.1:6113/" },
  bootWindowPrefix: "puzzle3d-",
  editorNumbers: [],
  paletteQuery: fixture.palette,
  stepNumbers: [1], order: [1], steps: { 1: lifecycle },
  context: () => ({}), prepare: async () => null, beforeStep: async () => {}, afterSteps: async () => {},
  diagnostics: async () => ({ instances: await instances(), selection: await selection() }),
  exploreInventory: async () => ({ instances: await instances(), selection: await selection() }),
};
