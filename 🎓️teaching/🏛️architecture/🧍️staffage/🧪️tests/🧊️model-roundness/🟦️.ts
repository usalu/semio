/** ⭕️ Round solids are round in the viewer and in every 3D export: the tessellation quality of a scale follows the roundness rule (chord within 2 % of the smallest feature of the scale and 1/3200 of the asset, at least 24 segments on a circle larger than one feature, capped per scale), with smooth normals on round surfaces and hard normals only at rims.
 *
 * THE REFERENCE is trimesh with numpy in `🐍️.py`, fed with the very triangles and normals of the kernel: the segments recomputed from the rule, the meridians counted, the gap to the analytic circle or sphere and the normal error measured.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️model-roundness/🔣️.json — the language-agnostic rule and cases
 * @see ../../🔨️modules/🧊️volume/⏱️job/🟦️.ts — `qualityOf` */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { scenesOf, tessellateSolid, type Quality, type Solid } from "../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset } from "../../🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { modelOfAsset } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { qualityOf } from "../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type Rule = { paperFeatureMm: number; chordPerFeature: number; largestPerChord: number; roundSegments: number; minSegments: number; maxSegments: Record<string, number> };
type Kind = "cylinder" | "sphere" | "capsule" | "loft";
type Vectors = { rule: Rule; scales: number[]; assets: { name: string; largestM: number }[]; radiiM: number[]; kinds: Kind[]; catalog: string[] };
type Measured = { expectedSegments: number; meridians: number; gapM: number; normalErrorDeg: number; hardRim: number; triangles: number };
type Entry = { name: string; kind: Kind | "revolved"; radiusM: number; lengthM: number; chordM: number; featureM: number; minSegments: number; maxSegments: number; roundSegments: number; positions: number[]; normals: number[]; indices: number[] };

const vectors = fixture<Vectors>("🧊️model-roundness");
const { rule } = vectors;
const ORACLE = join(STAFFAGE_ROOT, "🧪️tests/🧊️model-roundness/🐍️.py");
const featureOf = (scale: number): number => (rule.paperFeatureMm * scale) / 1000;
const assetOf = (largestM: number): Asset => ({ dimensions: { heightM: largestM, widthM: largestM / 2, depthM: largestM / 3 } }) as Asset;
const base = { id: "s", node: "n", material: "m" };
const solidOf = (kind: Kind, r: number): Solid =>
  (kind === "cylinder" ? { ...base, kind: "revolve", profile: [[0, r], [4 * r, r]], axis: [0, 1, 0], origin: [0, 0, 0] }
  : kind === "sphere" ? { ...base, kind: "ellipsoid", radii: [r, r, r] }
  : kind === "capsule" ? { ...base, kind: "capsule", p0: [0, 0, 0], p1: [0, 4 * r, 0], r0: r, r1: r }
  : { ...base, kind: "loft", sections: [{ at: [0, 0, 0], rx: r, ry: r }, { at: [0, 4 * r, 0], rx: r, ry: r }], capStart: true, capEnd: true }) as unknown as Solid;
const entryOf = (name: string, kind: Entry["kind"], radiusM: number, quality: Quality, scale: number, geometry: { positions: number[]; normals: number[]; indices: number[] }): Entry => ({ name, kind, radiusM, lengthM: 4 * radiusM, chordM: quality.chordM, featureM: featureOf(scale), minSegments: quality.minSegments, maxSegments: quality.maxSegments, roundSegments: rule.roundSegments, ...geometry });
const measure = (entries: readonly Entry[]): Record<string, Measured> => {
  const directory = mkdtempSync(join(tmpdir(), "staffage-roundness-"));
  try {
    writeFileSync(join(directory, "input.json"), JSON.stringify({ meshes: entries }));
    return JSON.parse(python(ORACLE, "geometry", [join(directory, "input.json")])) as Record<string, Measured>;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
};

describe("⭕️ the quality of a scale follows the roundness rule", () => {
  it.each(vectors.assets)("$name: chord, bounds and round floor at every scale", ({ largestM }) => {
    const chords = vectors.scales.map((scale) => {
      const quality = qualityOf(assetOf(largestM), scale);
      expect(quality.chordM).toBeCloseTo(Math.min(largestM / rule.largestPerChord, rule.chordPerFeature * featureOf(scale)), 12);
      expect(quality.chordM).toBeLessThanOrEqual(rule.chordPerFeature * featureOf(scale) * (1 + 1e-12));
      expect(quality.minSegments).toBe(rule.minSegments);
      expect(quality.maxSegments).toBe(rule.maxSegments[String(scale)]);
      expect(quality.round!.radiusM).toBeCloseTo(featureOf(scale), 12);
      expect(quality.round!.segments).toBe(rule.roundSegments);
      return quality.chordM;
    });
    expect([...chords].sort((a, b) => a - b)).toEqual(chords);
  });

  it("the reference of the validators without a scale is never coarser than the finest scale", () => {
    for (const { largestM } of vectors.assets) {
      const [reference, finest] = [qualityOf(assetOf(largestM)), qualityOf(assetOf(largestM), 50)];
      expect(reference.minSegments).toBeGreaterThanOrEqual(rule.roundSegments);
      expect(reference.maxSegments).toBeGreaterThanOrEqual(finest.maxSegments);
    }
  });
});

describe("⭕️ cylinders, spheres, capsules and round lofts are round at the quality of every asset and scale", () => {
  const cases = vectors.assets.flatMap((asset) => vectors.scales.flatMap((scale) => vectors.radiiM.flatMap((radiusM) => vectors.kinds.map((kind) => ({ asset, scale, radiusM, kind, name: `${asset.name}/1:${scale}/${kind}/${radiusM}` })))));
  let measured: Record<string, Measured> | undefined;
  const all = (): Record<string, Measured> => (measured ??= (requireTool("uv"), measure(cases.map((c) => {
    const quality = qualityOf(assetOf(c.asset.largestM), c.scale);
    return entryOf(c.name, c.kind, c.radiusM, quality, c.scale, tessellateSolid(solidOf(c.kind, c.radiusM), quality));
  }))));

  it("trimesh counts the segments of the rule, at least 24 where the radius exceeds one feature", { timeout: 600_000 }, () => {
    for (const c of cases) {
      const m = all()[c.name]!;
      expect(m.meridians, c.name).toBe(m.expectedSegments);
      if (c.radiusM > featureOf(c.scale)) expect(m.meridians, c.name).toBeGreaterThanOrEqual(Math.min(rule.roundSegments, rule.maxSegments[String(c.scale)]!));
      expect(m.meridians, c.name).toBeGreaterThanOrEqual(rule.minSegments);
    }
  });

  it("the gap to the analytic circle is within the chord unless the cap of the scale binds (a sphere: twice, over the diagonal of a cell)", { timeout: 600_000 }, () => {
    for (const c of cases) {
      const m = all()[c.name]!;
      const quality = qualityOf(assetOf(c.asset.largestM), c.scale);
      const sagitta = c.radiusM * (1 - Math.cos(Math.PI / m.expectedSegments));
      const capped = m.expectedSegments === quality.maxSegments;
      const limit = (capped ? sagitta : Math.min(sagitta, quality.chordM)) * (c.kind === "sphere" ? 2.05 : 1);
      if (m.expectedSegments > quality.minSegments) expect(m.gapM, c.name).toBeLessThanOrEqual(limit * (1 + 1e-6) + 1e-12);
      expect(m.gapM, c.name).toBeGreaterThan(0);
      expect(m.gapM, c.name).toBeLessThanOrEqual(c.radiusM * (1 - Math.cos(Math.PI / rule.minSegments)) * 2.05);
    }
  });

  it("the normals are smooth on the round surface and hard only at the rims of cylinders and capped lofts", { timeout: 600_000 }, () => {
    for (const c of cases) {
      const m = all()[c.name]!;
      expect(m.normalErrorDeg, c.name).toBeLessThan(0.01);
      expect(m.hardRim, c.name).toBe(c.kind === "cylinder" || c.kind === "loft" ? 2 * m.expectedSegments : 0);
    }
  });

  it("a coarser scale never has more triangles", { timeout: 600_000 }, () => {
    for (const asset of vectors.assets) for (const radiusM of vectors.radiiM) for (const kind of vectors.kinds) {
      const counts = vectors.scales.map((scale) => all()[`${asset.name}/1:${scale}/${kind}/${radiusM}`]!.triangles);
      for (let i = 1; i < counts.length; i += 1) expect(counts[i]!, `${asset.name} ${kind} ${radiusM} 1:${vectors.scales[i]}`).toBeLessThanOrEqual(counts[i - 1]!);
    }
  });
});

const cross = (a: readonly number[], b: readonly number[]): number[] => [a[1]! * b[2]! - a[2]! * b[1]!, a[2]! * b[0]! - a[0]! * b[2]!, a[0]! * b[1]! - a[1]! * b[0]!];
const unit = (a: readonly number[]): number[] => { const l = Math.hypot(a[0]!, a[1]!, a[2]!); return [a[0]! / l, a[1]! / l, a[2]! / l]; };

/** 🧭️ A full round solid of the catalog about its own axis: its largest radius and its vertices carried into the frame whose Y axis is that axis; `null` for every other solid. */
function aboutAxis(solid: any, geometry: { positions: number[]; normals: number[]; indices: number[] }): { radiusM: number; positions: number[] } | null {
  const frame = (origin: readonly number[], axis: readonly number[]): number[] => {
    const a = unit(axis);
    const e1 = unit(cross(a, Math.abs(a[0]!) < 0.9 ? [1, 0, 0] : [0, 0, 1]));
    const e2 = cross(a, e1);
    const out: number[] = [];
    for (let i = 0; i < geometry.positions.length; i += 3) {
      const d = [geometry.positions[i]! - origin[0]!, geometry.positions[i + 1]! - origin[1]!, geometry.positions[i + 2]! - origin[2]!];
      out.push(d[0]! * e1[0]! + d[1]! * e1[1]! + d[2]! * e1[2]!, d[0]! * a[0]! + d[1]! * a[1]! + d[2]! * a[2]!, d[0]! * e2[0]! + d[1]! * e2[1]! + d[2]! * e2[2]!);
    }
    return out;
  };
  if ((solid.lateralScale ?? 1) !== 1) return null;
  if (solid.kind === "revolve" && (solid.sweepRad === undefined || solid.sweepRad >= 2 * Math.PI - 1e-9)) return { radiusM: Math.max(...solid.profile.map((p: number[]) => p[1]!)), positions: frame(solid.origin, solid.axis) };
  if (solid.kind === "ellipsoid") return { radiusM: Math.max(...solid.radii), positions: geometry.positions.map((v, i) => v / solid.radii[i % 3]) };
  if (solid.kind === "capsule" && Math.hypot(solid.p1[0] - solid.p0[0], solid.p1[1] - solid.p0[1], solid.p1[2] - solid.p0[2]) > Math.max(solid.r0, solid.r1)) return { radiusM: Math.max(solid.r0, solid.r1), positions: frame(solid.p0, [solid.p1[0] - solid.p0[0], solid.p1[1] - solid.p0[1], solid.p1[2] - solid.p0[2]]) };
  return null;
}

describe("⭕️ the catalog shows round candles, valves, heads, limbs and wheels", async () => {
  const entries = (await assetPathsFromFiles()).filter((e) => vectors.catalog.includes(e.folder));

  it("finds the four assets", () => expect(entries.map((e) => e.folder).sort()).toEqual([...vectors.catalog].sort()));

  it.each(entries.map((e) => [e.folder, e] as const))("%s: every full revolve, ellipsoid and capsule larger than one feature has at least 24 meridians in every scene", { timeout: 900_000 }, async (_, entry) => {
    requireTool("uv");
    const { asset } = await readStreamedAsset(entry);
    const document = await modelOfAsset(asset, { poses: [asset.presentation.defaultPose], clips: [] });
    const meshes: Entry[] = [];
    const large: string[] = [];
    for (const scene of scenesOf(document)) {
      const scale = Number(/^s(\d+)-/u.exec(scene.id)![1]);
      const quality = qualityOf(asset, scale);
      const members = new Set(scene.nodes);
      for (const solid of document.solids) {
        if (!members.has(solid.node) || solid.tags?.includes("glyph") === true) continue;
        const { transform: _transform, repeat: _repeat, ...own } = solid as any;
        const geometry = tessellateSolid(own as Solid, quality);
        const round = aboutAxis(own, geometry);
        if (round === null || !(round.radiusM > 0)) continue;
        const name = `${scene.id}/${solid.id}`;
        meshes.push(entryOf(name, "revolved", round.radiusM, quality, scale, { ...geometry, positions: round.positions }));
        if (round.radiusM > featureOf(scale)) large.push(name);
      }
    }
    expect(large.length).toBeGreaterThan(0);
    const measured = measure(meshes);
    for (const mesh of meshes) expect(measured[mesh.name]!.meridians, mesh.name).toBe(measured[mesh.name]!.expectedSegments);
    for (const name of large) expect(measured[name]!.meridians, name).toBeGreaterThanOrEqual(rule.roundSegments);
  });
});
