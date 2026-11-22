/** 🧊️ Exercises the Puzzle 3D WGPU clipboard through a real world pick and keyboard chords. */
import { readFileSync } from "node:fs";
import type { TimeTravelScenarioV1 } from "@semio-tech/framework/time-travel/testing/scenario";
import { currentLocale, mod, page, renderer } from "@semio-tech/framework/time-travel/testing/state";
import { prepareChord } from "@semio-tech/framework/time-travel/testing/band";
import { closePanels } from "@semio-tech/framework/time-travel/testing/chrome";
import { evalSafe, waitUntil } from "@semio-tech/framework/time-travel/testing/page";
import { verdict } from "@semio-tech/framework/time-travel/testing/report";

type Vec3 = [number, number, number];
type Instance = {
  id: string;
  interactionId: string;
  meshId: string;
  position?: Vec3;
  rotation?: [number, number, number, number];
  scale?: Vec3;
};
type Surface = {
  surfaceId: string;
  rect: [number, number, number, number];
  instances: Instance[];
  selected: string[];
};
const fixture = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")) as {
  schema: string;
  variant: string;
  windowId: string;
  surfaceId: string;
  palette: Record<"en" | "de", string>;
  grid: number[];
  clipboardOffset: Vec3;
  tolerance: number;
  pickTimeoutMs: number;
};

const world = (): Promise<Surface | null> => evalSafe(async ({ windowId, surfaceId }) => {
  const hook = (window as Window & { semioWgpuIntrospection?: { dumpMeshStats(id: string): Promise<string> } }).semioWgpuIntrospection;
  if (!hook) return null;
  const dump = JSON.parse(await hook.dumpMeshStats(windowId)) as { surfaces: Surface[] };
  return dump.surfaces.find((surface) => surface.surfaceId === surfaceId) ?? null;
}, null as Surface | null, { windowId: fixture.windowId, surfaceId: fixture.surfaceId });

const sameInstances = (left: Instance[], right: Instance[]) => {
  const sorted = (rows: Instance[]) => [...rows].sort((a, b) => a.id.localeCompare(b.id));
  return JSON.stringify(sorted(left)) === JSON.stringify(sorted(right));
};
const near = (actual: readonly number[] | undefined, expected: readonly number[] | undefined) =>
  !!actual && !!expected && actual.length === expected.length
    && actual.every((value, axis) => Math.abs(value - expected[axis]!) < fixture.tolerance);

const pickedInstance = (surface: Surface | null): Instance | undefined => {
  if (!surface || surface.selected.length !== 1) return undefined;
  return surface.instances.find((instance) => instance.interactionId === surface.selected[0] && instance.position
    && instance.scale?.some((component) => component !== 0));
};

const pick = async (surface: Surface) => {
  let previousSelection = surface.selected;
  for (const row of fixture.grid) {
    for (const column of fixture.grid) {
      const [x, y, width, height] = surface.rect;
      await page.mouse.click(x + width * column, y + height * row);
      const result = await waitUntil(world, (latest) => {
        const instance = pickedInstance(latest);
        return instance !== undefined && !previousSelection.includes(instance.interactionId);
      }, fixture.pickTimeoutMs, 150);
      if (result.ok) return result.value;
      previousSelection = result.value?.selected ?? previousSelection;
    }
  }
  return null;
};

const clipboard = async () => {
  const ready = renderer === "wgpu" && fixture.schema === "s.puzzle3d.wgpu-clipboard.v1";
  verdict("puzzle3d-wgpu-clipboard-fixture", ready, { renderer, locale: currentLocale, schema: fixture.schema });
  if (!ready) return;
  await closePanels();
  const boot = await waitUntil(world, (surface) => surface !== null && surface.rect[2] > 0 && surface.rect[3] > 0
    && surface.instances.some((instance) => instance.position && instance.scale?.some((component) => component !== 0)), 120000, 500);
  verdict("puzzle3d-wgpu-visible-world", boot.ok, { windowId: fixture.windowId, count: boot.value?.instances.length });
  if (!boot.ok || !boot.value) return;
  const selected = await pick(boot.value);
  const source = pickedInstance(selected);
  verdict("puzzle3d-wgpu-pick", source !== undefined, { selected: selected?.selected });
  if (!selected || !source?.position) return;
  const focus = await prepareChord();
  verdict("puzzle3d-wgpu-keyboard-focus", focus === "canvas", { focus });
  if (focus !== "canvas") return;
  const baseline = selected.instances;
  const ids = new Set(baseline.map((instance) => instance.id));
  await page.keyboard.press(mod + "+c");
  await page.keyboard.press(mod + "+v");
  const pasted = await waitUntil(world, (surface) => surface !== null
    && surface.instances.length === baseline.length + 1
    && surface.instances.filter((instance) => !ids.has(instance.id)).length === 1, 15000, 200);
  const clone = pasted.value?.instances.find((instance) => !ids.has(instance.id));
  const expectedPosition = source.position.map((value, axis) => value + fixture.clipboardOffset[axis]!);
  const originalsUnchanged = !!pasted.value && sameInstances(pasted.value.instances.filter((instance) => ids.has(instance.id)), baseline);
  verdict("puzzle3d-wgpu-paste-transform", pasted.ok && !!clone && originalsUnchanged && clone.meshId === source.meshId
    && near(clone.position, expectedPosition) && near(clone.rotation, source.rotation) && near(clone.scale, source.scale),
  { source, clone, expectedPosition, originalsUnchanged, count: pasted.value?.instances.length });
  if (!pasted.ok || !clone) return;
  const pickedClone = await waitUntil(world, (surface) => surface?.selected.length === 1
    && surface.selected[0] === clone.interactionId, 10000, 200);
  verdict("puzzle3d-wgpu-pasted-object-selected", pickedClone.ok, { selected: pickedClone.value?.selected, clone: clone.interactionId });
  await prepareChord();
  await page.keyboard.press(mod + "+z");
  const undone = await waitUntil(world, (surface) => surface !== null && sameInstances(surface.instances, baseline), 15000, 200);
  verdict("puzzle3d-wgpu-paste-undo", undone.ok, { count: undone.value?.instances.length });
};

/** 🎬️ Registers the WGPU clipboard journey on the existing localized browser harness. */
export const timeTravelScenarioV1: TimeTravelScenarioV1 = {
  variant: fixture.variant,
  defaultServes: { react: "http://127.0.0.1:6013/", wgpu: "http://127.0.0.1:6113/" },
  bootWindowPrefix: "puzzle3d-",
  editorNumbers: [], paletteQuery: fixture.palette,
  stepNumbers: [1], order: [1], steps: { 1: clipboard },
  context: () => ({}), prepare: async () => null, beforeStep: async () => {}, afterSteps: async () => {},
  diagnostics: async () => ({ surface: await world() }),
  exploreInventory: async () => ({ surface: await world() }),
};
