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

const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as { schema: string; variant: string; world: string; canvas: string; selection: string; palette: Record<"en" | "de", string>; grid: number[]; addedObjects: number; dragPixels: number; minMovement: number; pickTimeoutMs: number; clearTimeoutMs: number; clearExample: Record<"en" | "de", string>; clearPoints: [number, number][] };
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
  const parsed = JSON.parse(value) as { selectedIds?: string[]; activeObjectId?: string };
  return [...(parsed.selectedIds ?? []), ...(parsed.activeObjectId ? [parsed.activeObjectId] : [])];
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
  const createdId = added.length === 1 ? added[0].id : undefined;
  verdict("puzzle3d-created", created.ok && !!createdId, { before: blank.value.length, after: created.value.length, createdId });
  if (!created.ok || !createdId) return;
  await closePanels();
  const canvas = page.locator(fixture.canvas).first(), box = await canvas.boundingBox();
  let picked = false;
  if (box) for (const y of fixture.grid) { for (const x of fixture.grid) {
    let cleared = false;
    for (const [cx, cy] of fixture.clearPoints) {
      await page.mouse.click(box.x + box.width * cx, box.y + box.height * cy);
      if ((await waitUntil(selection, (ids) => ids.length === 0, fixture.clearTimeoutMs, 200)).ok) { cleared = true; break; }
    }
    if (!cleared) { verdict("puzzle3d-clear-selection", false, { selected: await selection() }); return; }
    await page.mouse.click(box.x + box.width * x, box.y + box.height * y);
    const result = await waitUntil(selection, (ids) => ids.includes(createdId), fixture.pickTimeoutMs, 200);
    if (result.ok) { picked = true; break; }
  } if (picked) break; }
  verdict("puzzle3d-spatial-pick", picked, { createdId, selected: await selection() });
  if (!picked || !box) return;
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
  await page.mouse.move(box.x + origin.sx + dx * 0.6 + dx / length * fixture.dragPixels, box.y + origin.sy + dy * 0.6 + dy / length * fixture.dragPixels, { steps: 16 });
  await page.mouse.up();
  const prior = created.value.find((row) => row.id === createdId);
  const edited = await waitUntil(instances, (rows) => {
    const after = rows.find((row) => row.id === createdId);
    return !!prior?.position && !!after?.position && after.position.some((value, index) => Math.abs(value - prior.position![index]) > fixture.minMovement);
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
