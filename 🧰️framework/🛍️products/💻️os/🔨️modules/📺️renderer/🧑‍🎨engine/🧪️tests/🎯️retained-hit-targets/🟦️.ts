/**
 * 🎯️ The TypeScript twin of `🖱️ui/🧪️tests/🎯️retained-hit-targets/🦀️.rs`. Both read the SAME neutral
 * fixture (`🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json`): the Rust law drives the live wgpu
 * pipeline — real layout, real retained paint, real `InputState` — and proves the registry it mints
 * answers each point; this one re-derives the whole registry from the fixture's own declared rules
 * with a second, independent implementation, on React's own row metric
 * (`domSizePx("treeRowUiSpacing")`, the `--size-workbench` its `Tree` rows carry).
 *
 * The defect both sides pin: the host's flat registry held the shell chrome and not one row of any
 * retained window body, so a pointer at (160.696, 138) — inside the published `[0, 72, 315.392, 24]`
 * rect of the `Add Generation` row — answered the WINDOW's `ScrollRegion` and dispatched nothing
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { domSizePx } from "@semio-tech/ui-styling";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const uiRoot = resolve(repoRoot, "🧰️framework/🔨️modules/🖱️ui");

type ActionBinding = { readonly controller: string; readonly action: string };
type FixtureItem = { readonly id: string; readonly label?: string; readonly action?: ActionBinding | null };
type FixtureSection = { readonly id: string; readonly label?: string | null; readonly items?: readonly FixtureItem[] };
type FixtureNode =
  | { readonly kind: "tree"; readonly sections: readonly FixtureSection[] }
  | { readonly kind: "surface"; readonly surfaceId: string; readonly surfaceKind: string }
  | { readonly kind: "stack"; readonly id?: string; readonly activate?: ActionBinding | null; readonly children?: readonly FixtureNode[] }
  | { readonly kind: "text"; readonly value?: string };
type FixtureEntry = { readonly controlId: string; readonly kind: string; readonly action: string | null; readonly rect: readonly [number, number, number, number] };
type FixtureProbe = { readonly x: number; readonly y: number; readonly controlId: string; readonly kind: string; readonly action: string | null; readonly wheelPropagatesToScene?: boolean };
type FixtureCase = {
  readonly name: string;
  readonly windowId: string;
  readonly body: { readonly x: number; readonly y: number; readonly w: number; readonly h: number };
  readonly tree: FixtureNode;
  readonly expected: readonly FixtureEntry[];
  readonly probes: readonly FixtureProbe[];
};

const law = JSON.parse(readFileSync(resolve(uiRoot, "🧫️fixtures/🎯️retained-hit-targets/🔣️.json"), "utf8")) as {
  readonly metrics: { readonly rowHeightPx: number };
  readonly cases: readonly FixtureCase[];
};

/** 📏️ The row metric React's own `Tree` presents rows on — the one number this registry's tree bands share with layout and paint. */
const rowHeightPx = domSizePx("treeRowUiSpacing");

type Registration = { controlId: string; kind: string; action: string | null; rect: [number, number, number, number] };

/** 🖱️ The kind and control id an engine-surface canvas registers under — the `.pane` / `.map` convention the shell's own scene-surface predicate reads. */
const sceneRegistration = (surfaceId: string, surfaceKind: string): { kind: string; controlId: string } => {
  if (surfaceKind === "world-3d") return { kind: "world3d", controlId: surfaceId };
  if (surfaceKind === "node-graph" || surfaceKind === "board-2d") return { kind: "scrollRegion", controlId: `${surfaceId}.pane` };
  if (surfaceKind === "tiled-map") return { kind: "scrollRegion", controlId: `${surfaceId}.map` };
  return { kind: "generic", controlId: surfaceId };
};

/** 🖱️ `ShellState::wheel_propagates_to_scene_surface`, re-derived — the predicate the wheel gate stands on. */
const wheelPropagates = (entry: { kind: string; controlId: string }): boolean => {
  if (entry.kind === "world3d" || entry.kind === "window") return true;
  if (entry.kind === "scrollRegion") return entry.controlId.endsWith(".pane") || entry.controlId.endsWith(".map");
  return false;
};

/** 🌳️ A tree's registry entries, in the pre-order the retained paint walk visits them: one section header band, then that section's item bands. */
const treeRegistrations = (sections: readonly FixtureSection[], originX: number, originY: number, width: number): Registration[] => {
  const entries: Registration[] = [];
  let y = originY;
  for (const section of sections) {
    const headerHeight = section.label === undefined || section.label === null ? 0 : rowHeightPx;
    if (headerHeight > 0) entries.push({ controlId: `section.chevron.${section.id}`, kind: "treeItem", action: null, rect: [originX, y, width, headerHeight] });
    let rowY = y + headerHeight;
    for (const item of section.items ?? []) {
      entries.push({ controlId: `tree.label.${item.id}`, kind: "treeItem", action: item.action?.action ?? null, rect: [originX, rowY, width, rowHeightPx] });
      rowY += rowHeightPx;
    }
    y = rowY;
  }
  return entries;
};

/** 🎯️ The whole registry one case's body publishes, derived from the fixture's rules alone. */
const registrations = (entry: FixtureCase): Registration[] => {
  const { x, y, w, h } = entry.body;
  const node = entry.tree;
  if (node.kind === "tree") return treeRegistrations(node.sections, x, y, w);
  if (node.kind === "surface") {
    const scene = sceneRegistration(node.surfaceId, node.surfaceKind);
    return [{ controlId: scene.controlId, kind: scene.kind, action: null, rect: [x, y, w, h] }];
  }
  if (node.kind === "stack" && node.activate !== undefined && node.activate !== null) {
    return [{ controlId: node.id ?? node.activate.action, kind: "button", action: node.activate.action, rect: [x, y, w, h] }];
  }
  return [];
};

/** 🔢️ The registry a host scans: the window's own `ScrollRegion` first, the body's entries after, resolved in reverse. */
const resolve_at = (entry: FixtureCase, x: number, y: number): Registration | undefined => {
  const scan: Registration[] = [{ controlId: entry.windowId, kind: "scrollRegion", action: null, rect: [entry.body.x, entry.body.y, entry.body.w, entry.body.h] }, ...registrations(entry)];
  for (let index = scan.length - 1; index >= 0; index -= 1) {
    const [rx, ry, rw, rh] = scan[index]!.rect;
    if (x >= rx && x < rx + rw && y >= ry && y < ry + rh) return scan[index];
  }
  return undefined;
};

describe("🎯️ retained hit targets", () => {
  it("pins the row metric the fixture was authored against", () => {
    expect(rowHeightPx).toBe(law.metrics.rowHeightPx);
  });

  for (const entry of law.cases) {
    it(`re-derives every registry entry of ${entry.name}`, () => {
      const derived = registrations(entry);
      expect(derived.map((row) => row.controlId)).toEqual(entry.expected.map((row) => row.controlId));
      expect(derived.map((row) => row.kind)).toEqual(entry.expected.map((row) => row.kind));
      expect(derived.map((row) => row.action)).toEqual(entry.expected.map((row) => row.action));
      derived.forEach((row, index) => {
        const want = entry.expected[index]!.rect;
        row.rect.forEach((value, axis) => expect(value).toBeCloseTo(want[axis]!, 3));
      });
    });

    it(`resolves every pointer point of ${entry.name}`, () => {
      for (const probe of entry.probes) {
        const hit = resolve_at(entry, probe.x, probe.y);
        expect(hit, `(${probe.x}, ${probe.y}) resolved nothing`).toBeDefined();
        expect(hit!.controlId).toBe(probe.controlId);
        expect(hit!.kind).toBe(probe.kind);
        expect(hit!.action).toBe(probe.action);
        if (probe.wheelPropagatesToScene !== undefined) expect(wheelPropagates(hit!)).toBe(probe.wheelPropagatesToScene);
      }
    });
  }

  it("orders every body entry above the window that hosts it", () => {
    for (const entry of law.cases) {
      for (const row of registrations(entry)) {
        const [rx, ry, rw, rh] = row.rect;
        const hit = resolve_at(entry, rx + rw / 2, ry + rh / 2);
        expect(hit!.controlId, `${entry.name}: ${row.controlId} was outranked at its own centre`).toBe(row.controlId);
        expect(hit!.kind).toBe(row.kind);
      }
    }
  });

  it("refuses the defect: the derived row point never answers the window", () => {
    const entry = law.cases.find((candidate) => candidate.name === "generate-mode-generations-window")!;
    const hit = resolve_at(entry, 160.696, 138);
    expect(hit!.controlId).not.toBe(entry.windowId);
    expect(hit!.kind).toBe("treeItem");
    expect(hit!.action).toBe("addGeneration");
  });
});
