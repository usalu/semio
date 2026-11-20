/** 🌊️ The flood from outside of the thermal outer area (C31) against hand values and an independent voxel flood.
 *
 * THE REFERENCES are the hand-computed areas of `🧫️fixtures/🧊️volumes-flood` and trimesh with scipy in `🐍️.py`, fed with the very triangles of the kernel.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🔨️modules/🗂️catalog/🧊️volumes/🌊️flood/🟦️.ts — the flood
 * @see ../🧊️volumes-thermal/🟦️.ts — the other area measures of the rule */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { measureVolumes } from "../../🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { FLOOD_LIMITS, floodOutside } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌊️flood/🟦️.ts";
import { COVER_FEATURES, FLOOD_FEATURES, exposedArea, outsideOf } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧮️measures/🟦️.ts";
import type { World } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Solid = { region: string; kind: "tube"; outerRadiusM: number; innerRadiusM: number; segments: number; from: number; to: number } | { region: string; kind: "box"; min: number[]; max: number[] };
type Case = { name: string; hand: string; solids: Solid[]; target: string; areaM2: number; unfloodedM2?: number };
type Vectors = { featureM: number; cellM: number; probeM: number; seamM: number; coverM: number; toleranceFraction: number; toleranceM2: number; cases: Case[]; sample: { scale: 50; region: string; remove: string; rod: { id: string }; tube: { id: string }; caps: { id: string }[]; open: string; openGainM2: number; toleranceFraction: number } };

const vectors = fixture<Vectors>("🧊️volumes-flood");
const sample: Asset = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🧊️volumes-flood/🐍️.py");
const quality = { chordM: 0.002, minSegments: 32, maxSegments: 64 };
const circle = (r: number, n: number): [number, number][] => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)]);
const kernel = (id: string, solid: Record<string, unknown>): World => {
  const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, quality);
  return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
};
const worldOf = (solid: Solid, k: number): World =>
  solid.kind === "tube"
    ? kernel(`${solid.region}-${k}`, { kind: "extrude", plane: "xy", profile: circle(solid.outerRadiusM, solid.segments), ...(solid.innerRadiusM > 0 ? { holes: [circle(solid.innerRadiusM, solid.segments).reverse()] } : {}), from: solid.from, to: solid.to })
    : kernel(`${solid.region}-${k}`, { kind: "extrude", plane: "xy", profile: [[solid.min[0], solid.min[1]], [solid.max[0], solid.min[1]], [solid.max[0], solid.max[1]], [solid.min[0], solid.max[1]]], from: solid.min[2], to: solid.max[2] });
const built = (c: Case): { worlds: World[]; own: World[] } => {
  const worlds = c.solids.map(worldOf);
  return { worlds, own: worlds.filter((_, k) => c.solids[k]!.region === c.target) };
};
const flooded = (c: Case): number => {
  const { worlds, own } = built(c);
  const outside = outsideOf(worlds, vectors.featureM, vectors.probeM);
  expect(outside.cellM).toBeCloseTo(vectors.cellM, 12);
  return exposedArea(own, own, [], () => false, vectors.probeM, vectors.seamM, (base, normal) => outside.counts(base, normal, (solid) => own.includes(solid)));
};
const unflooded = (c: Case): number => {
  const { own } = built(c);
  return exposedArea(own, own, [], () => false, vectors.probeM, vectors.seamM);
};
const within = (value: number, hand: number): boolean => Math.abs(value - hand) <= Math.max(vectors.toleranceFraction * hand, vectors.toleranceM2);

describe("🌊️ C31: the outer area is the surface the air from outside reaches", () => {
  it("uses a quarter feature as cell and half a feature as cover, as the fixture states", () => {
    expect(FLOOD_FEATURES * vectors.featureM).toBeCloseTo(vectors.cellM, 12);
    expect(COVER_FEATURES * vectors.featureM).toBeCloseTo(vectors.coverM, 12);
  });

  it.each(vectors.cases)("$name: the flooded outer area equals the hand value", (c) => {
    const area = flooded(c);
    expect(within(area, c.areaM2), `${c.name}: ${area} against ${c.areaM2} (${c.hand})`).toBe(true);
    if (c.unfloodedM2 !== undefined) {
      expect(within(unflooded(c), c.unfloodedM2), `${c.name} without the flood: ${unflooded(c)} against ${c.unfloodedM2}`).toBe(true);
      expect(within(c.unfloodedM2, c.areaM2)).toBe(false);
    }
  });

  it("equals the voxel flood of trimesh and scipy on the same triangles", { timeout: 600_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-flood-"));
    try {
      const cases = vectors.cases.map((c) => ({ name: c.name, target: c.target, solids: built(c).worlds.map((w, k) => ({ region: c.solids[k]!.region, positions: [...w.positions], indices: [...w.indices] })) }));
      writeFileSync(join(directory, "input.json"), JSON.stringify({ pitch: vectors.cellM, eps: vectors.probeM, cover: vectors.coverM, cases }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
      for (const c of vectors.cases) {
        expect(within(reference[c.name]!.area, c.areaM2), `${c.name} (oracle): ${reference[c.name]!.area} against ${c.areaM2}`).toBe(true);
        expect(within(flooded(c), reference[c.name]!.area), `${c.name} (owned against oracle)`).toBe(true);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("raises the cell when the grid would exceed its budget and still closes the cavity", () => {
    const c = vectors.cases.find((x) => x.name === "a casing of six panels")!;
    const { worlds } = built(c);
    const coarse = floodOutside(worlds, vectors.cellM, () => {}, { fineBlocks: 2000, coarseBlocks: FLOOD_LIMITS.coarseBlocks });
    expect(coarse.cellM).toBeGreaterThan(vectors.cellM);
    expect(coarse.reaches([0, 0, 0.12], [0, 0, 1])).toBe(true);
    expect(coarse.reaches([0, 0, 0.0899], [0, 0, -1])).toBe(false);
    expect(coarse.reaches([0, 0, 0], [0, 0, 1])).toBe(false);
    expect(coarse.firstHit([0, 0, 0], [0, 0, 0.2])).toBe(1);
    expect(coarse.firstHit([0, 0, 0], [0, 0, 0.05])).toBe(-1);
  });

  it("stops when its tick throws", () => {
    const { worlds } = built(vectors.cases[0]!);
    expect(() => floodOutside(worlds, vectors.cellM, () => { throw new Error("cancelled"); })).toThrow("cancelled");
  });
});

describe("🌊️ the thermal rule on the sample figure with a hollow arm", () => {
  const s = vectors.sample;
  const variant = (solids: readonly unknown[]): Asset => {
    const copy = structuredClone(sample) as any;
    const list = copy.volumes[String(s.scale)].thermal as { id: string }[];
    list.splice(list.findIndex((x) => x.id === s.remove), 1, ...(solids as { id: string }[]));
    return copy as Asset;
  };
  const measured = (asset: Asset, operation = {}) => measureVolumes(asset, { scale: s.scale, rules: ["volumes-thermal-coverage"] }, operation);
  const outer = (asset: Asset): number => measured(asset).find((m) => m.name === `info:outerAreaM2:${s.region}`)!.value;
  const rod = variant([s.rod]);
  const closed = variant([s.tube, ...s.caps]);
  const open = variant([s.tube, ...s.caps.filter((cap) => cap.id !== s.open)]);

  it("measures the arm closed by two discs like the solid rod and counts the bore once a disc is removed", () => {
    expect(Math.abs(outer(closed) / outer(rod) - 1)).toBeLessThan(s.toleranceFraction);
    expect(Math.abs((outer(open) - outer(closed)) / s.openGainM2 - 1)).toBeLessThan(3 * s.toleranceFraction);
  });

  it("reports the cell of the flood and stops when the signal is aborted", () => {
    expect(measured(closed).find((m) => m.name === "info:floodCellM")!.value).toBeCloseTo(vectors.cellM, 12);
    const controller = new AbortController();
    controller.abort();
    expect(() => measured(closed, { signal: controller.signal })).toThrow();
  });
});
