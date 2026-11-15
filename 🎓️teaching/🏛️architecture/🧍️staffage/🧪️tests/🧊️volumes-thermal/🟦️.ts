/** 🧿️ The thermal area measures of the volumes (C21 refined, C29 seams, hidden regions, `sectionFaces` on `{ of }` references) against hand values and trimesh.
 *
 * THE REFERENCES are the hand-computed area of a ring around a disc and trimesh surface samples in `🐍️.py`, fed with the very triangles of the kernel.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../🧊️volumes-rules/🟦️.ts — the rules these measures belong to */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { exposedArea, measureVolumes } from "../../🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts";
import { worldSolids, type World } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { tessellateSolid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

const sample: Asset = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🧊️volumes-thermal/🐍️.py");
const quality = { chordM: 0.002, minSegments: 32, maxSegments: 64 };
const circle = (r: number, n: number): [number, number][] => Array.from({ length: n }, (_, i) => [r * Math.cos((2 * Math.PI * i) / n), r * Math.sin((2 * Math.PI * i) / n)]);
const kernel = (id: string, solid: Record<string, unknown>): World => {
  const g = tessellateSolid({ id, bone: "b", material: "m", role: "body", tone: 0.5, node: "b", ...solid } as never, quality);
  return { id, positions: Float64Array.from(g.positions), indices: Uint32Array.from(g.indices) };
};
const ringAndDisc = (ringSegments: number, discSegments: number, discRadius: number): World[] => [
  kernel("ring", { kind: "extrude", plane: "xy", profile: circle(0.31, ringSegments), holes: [circle(0.26, ringSegments).reverse()], from: -0.11, to: 0.11 }),
  kernel("disc", { kind: "extrude", plane: "xy", profile: circle(discRadius, discSegments), from: -0.05, to: 0.05 }),
];
const PROBE = 0.00015;
const SEAM = 0.0015;
const area = (worlds: readonly World[]): number => exposedArea(worlds, worlds, [], () => false, PROBE, SEAM);
const [R, r, H, h] = [0.31, 0.26, 0.11, 0.05];
const union = 2 * Math.PI * R * 2 * H + 2 * Math.PI * (R * R - r * r) + 2 * 2 * Math.PI * r * (H - h) + 2 * Math.PI * r * r;
const gapped = 2 * Math.PI * R * 2 * H + 2 * Math.PI * (R * R - r * r) + 2 * Math.PI * r * 2 * H + 2 * Math.PI * 0.255 * 2 * h + 2 * Math.PI * 0.255 * 0.255;

describe("🧊️ C29: solids of one region that meet along a seam leave no surface, whatever their tessellation", () => {
  for (const [a, b] of [[64, 64], [64, 48], [64, 32], [48, 64], [32, 64], [64, 40]] as const)
    it(`a disc in a ring with ${a} and ${b} segments measures the hand value of the union`, () => {
      expect(Math.abs(area(ringAndDisc(a, b, 0.26)) / union - 1)).toBeLessThan(0.015);
    });

  it("keeps both faces of a real 5 mm gap", () => {
    expect(Math.abs(area(ringAndDisc(64, 64, 0.255)) / gapped - 1)).toBeLessThan(0.015);
  });

  it("equals the surface samples of trimesh for the same tessellation", { timeout: 300_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-thermal-"));
    try {
      const solids = ringAndDisc(64, 64, 0.26).map((w) => ({ positions: [...w.positions], indices: [...w.indices], sections: [] }));
      writeFileSync(join(directory, "input.json"), JSON.stringify({ regions: [{ name: "union", hidden: false, solids }], eps: PROBE }));
      const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
      expect(Math.abs(area(ringAndDisc(64, 64, 0.26)) / reference.union!.area - 1)).toBeLessThan(0.015);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

type Patch = (asset: any) => void;
const patched = (...edits: Patch[]): Asset => {
  const copy = structuredClone(sample) as any;
  for (const edit of edits) edit(copy);
  return copy as Asset;
};
const region = (asset: any, id: string): any => asset.thermal.regions.find((x: any) => x.id === id);
const outer = (asset: Asset, scale: 50 | 200 | 500, id: string): number => measureVolumes(asset, { scale, rules: ["volumes-thermal-coverage"] }).find((m) => m.name === `info:outerAreaM2:${id}`)!.value;
const axes = (w: World, faces: readonly string[]): number[][] => faces.map((f) => {
  const a = w.frame![f.slice(1) as "u" | "v" | "w"];
  const s = f[0] === "-" ? -1 : 1;
  return [s * a.x, s * a.y, s * a.z];
});
const oracleOf = (asset: Asset, scale: 50, regions: readonly { id: string; hidden: boolean }[]): Record<string, number> => {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
  const worlds = new Map(worldSolids(asset, state.bones as never, scale, "thermal").map((w) => [w.id, w] as const));
  const thermal = (asset.volumes as any)[String(scale)].thermal as { id: string; region: string }[];
  const directory = mkdtempSync(join(tmpdir(), "staffage-thermal-"));
  try {
    const input = regions.map(({ id, hidden }) => ({ name: id, hidden, solids: thermal.filter((s) => s.region === id).map((s) => worlds.get(s.id)!).filter((w) => w.emitter !== true).map((w) => ({ positions: [...w.positions], indices: [...w.indices], sections: axes(w, w.sectionFaces ?? []) })) }));
    writeFileSync(join(directory, "input.json"), JSON.stringify({ regions: input, eps: 0.02 * 0.0075 }));
    const reference = JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, { area: number }>;
    return Object.fromEntries(Object.entries(reference).map(([k, v]) => [k, v.area]));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
};

describe("🧊️ the sample figure region by region (C29, hidden regions, sectionFaces on references)", () => {
  const section = (asset: any, id: string, faces: string[]): void => void (asset.volumes["50"].thermal.find((s: any) => s.id === id).sectionFaces = faces);
  const hide = (asset: any): void => void (region(asset, "legs").hidden = { en: "inside", de: "innen" });
  const plain = sample;
  const cut = patched((a) => section(a, "arm-l-arms", ["-v", "+v"]));
  const hidden = patched(hide);

  it("equals trimesh for visible regions, for a hidden region (its full mesh area) and for sectionFaces on a { of } reference", { timeout: 300_000 }, () => {
    requireTool("uv");
    const [visible, cutAreas, hiddenAreas] = [oracleOf(plain, 50, [{ id: "torso", hidden: false }, { id: "arms", hidden: false }]), oracleOf(cut, 50, [{ id: "arms", hidden: false }]), oracleOf(hidden, 50, [{ id: "legs", hidden: true }])];
    expect(Math.abs(outer(plain, 50, "torso") / visible.torso! - 1)).toBeLessThan(0.03);
    expect(Math.abs(outer(plain, 50, "arms") / visible.arms! - 1)).toBeLessThan(0.03);
    expect(Math.abs(outer(cut, 50, "arms") / cutAreas.arms! - 1)).toBeLessThan(0.03);
    expect(Math.abs(outer(hidden, 50, "legs") / hiddenAreas.legs! - 1)).toBeLessThan(1e-6);
  });

  it("drops the declared faces of a { of } reference and does not inherit those of the body solid", () => {
    expect(outer(cut, 50, "arms")).toBeLessThan(outer(plain, 50, "arms") - 1e-4);
    const body = patched((a) => void (a.volumes["50"].body.find((s: any) => s.id === "arm-l").sectionFaces = ["-v", "+v"]));
    expect(outer(body, 50, "arms")).toBeCloseTo(outer(plain, 50, "arms"), 9);
  });

  it("measures a hidden region on its full mesh, larger than its visible outer area where its solids bury each other", () => {
    expect(outer(hidden, 50, "legs")).toBeGreaterThanOrEqual(outer(plain, 50, "legs") - 1e-9);
  });

  it("leaves a region below six squared features out of the total at 1:500 on both sides (C21 refined)", () => {
    const small = patched((a) => void (region(a, "feet").areaM2 = 0.01));
    const measures = measureVolumes(small, { scale: 500, rules: ["volumes-thermal-coverage"] });
    const get = (name: string): number => measures.find((m) => m.name === name)!.value;
    const ids = sample.thermal.regions.map((x) => x.id).filter((id) => measures.some((m) => m.name === `info:outerAreaM2:${id}`));
    expect(ids).toContain("feet");
    const declared = (id: string): number => (id === "feet" ? 0.01 : region(small, id).areaM2);
    const [others, all] = [ids.filter((id) => id !== "feet"), ids];
    const sum = (list: string[], f: (id: string) => number): number => list.reduce((s, id) => s + f(id), 0);
    const kept = Math.abs(sum(others, (id) => get(`info:outerAreaM2:${id}`)) / sum(others, declared) - 1);
    const old = Math.abs(sum(all, (id) => get(`info:outerAreaM2:${id}`)) / sum(all, declared) - 1);
    expect(get("areaDeviationTotal")).toBeCloseTo(kept, 9);
    expect(Math.abs(kept - old)).toBeGreaterThan(1e-4);
  });
});
