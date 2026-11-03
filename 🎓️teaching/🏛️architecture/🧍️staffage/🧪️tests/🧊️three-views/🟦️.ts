/** 🧊️ Three views: every authored representation of every catalog asset draws the side (elevation), the front and the top (plan) view for every pose and clip instant, and the three views of the default pose are one object that matches the dimensions.
 *
 * THE REFERENCE is shapely (🐍️.py): it recomputes the extents of the hand-computed vector shapes and of the exported evaluated rings of the complete reference assets.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ./🧰️.ts — the sweep and the consistency rules
 * @see ../../🧫️fixtures/🧊️three-views/🔣️.json — the vectors
 */
import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, Shape, Viewpoint } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool, python } from "../🧰️support/🧰️.ts";
import { VIEWS, authoredScales, catalog, consistency, extentOf, sweep, toleranceM, type Extent } from "./🧰️.ts";

type Part = Partial<Extent>;
type Travel = Record<string, { dx: number | string; dy: number | string }>;
type Pose = { pose: string; originTravel: Travel; tipTravel: Travel; liftM?: number };
type Vectors = {
  tolerance: { absoluteM: number; relative: number; cases: { heightM: number; widthM: number; depthM: number; toleranceM: number }[] };
  asset: string;
  scale: number;
  dimensions: { heightM: number; widthM: number; depthM: number };
  bones: Record<string, unknown>;
  poses: Record<string, Pose>;
  shapes: Record<Viewpoint, Shape[]>;
  cases: { name: string; pose: string; extents: Record<string, Part>; views?: Record<string, Part>; heightsM?: Record<string, number>; offsetsM?: { name: string; of: string; edge: keyof Extent; from: string; fromEdge: keyof Extent; valueM: number }[] }[];
  consistency: { pose: string; cases: { name: string; patch: { view: Viewpoint; shape: string; set: Record<string, number> }[]; failing: string[] }[] };
  catalog: { complete: string[]; scales: number[] };
  extentToleranceM: number;
};

const vectors = fixture<Vectors>("🧊️three-views");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const select = (denominator: number) => ({ kind: "scale", scale: { denominator } }) as const;

const synthetic = (shapes: Record<Viewpoint, Shape[]>, dimensions = vectors.dimensions): Asset => {
  const authored = sample.representations[String(vectors.scale) as keyof Asset["representations"]] as AuthoredRepresentation;
  const representation: AuthoredRepresentation = { ...authored, geometry: shapes };
  return { ...sample, dimensions: { ...sample.dimensions, ...dimensions }, representations: { ...sample.representations, [String(vectors.scale)]: representation } };
};

const stateOf = (asset: Asset, pose: string): EvaluatedState => evaluate(asset, { kind: "pose", pose }, select(vectors.scale));

function extentsOf(state: EvaluatedState): Record<string, Extent> {
  const out: Record<string, Extent> = {};
  for (const view of VIEWS) for (const evaluated of state.shapes[view]) out[`${view}/${evaluated.shape.id}`] = extentOf([evaluated], true)!;
  return out;
}

const expectPart = (label: string, actual: Extent, expected: Part): void => {
  for (const [edge, value] of Object.entries(expected)) expect(Math.abs(actual[edge as keyof Extent] - value), `${label} ${edge}: ${actual[edge as keyof Extent]} vs ${value}`).toBeLessThanOrEqual(vectors.extentToleranceM);
};

describe("🧊️ the vectors of the three views", () => {
  const asset = synthetic(vectors.shapes);

  for (const c of vectors.cases)
    it(c.name, () => {
      const state = stateOf(asset, vectors.poses[c.pose]!.pose);
      const extents = extentsOf(state);
      for (const [key, part] of Object.entries(c.extents)) expectPart(key, extents[key]!, part);
      for (const [view, part] of Object.entries(c.views ?? {})) expectPart(view, extentOf(state.shapes[view as Viewpoint])!, part);
      for (const [key, height] of Object.entries(c.heightsM ?? {})) expect(Math.abs(extents[key]!.maxY - extents[key]!.minY - height), `${key} height`).toBeLessThanOrEqual(vectors.extentToleranceM);
      for (const offset of c.offsetsM ?? []) expect(Math.abs(extents[offset.of]![offset.edge] - extents[offset.from]![offset.fromEdge] - offset.valueM), offset.name).toBeLessThanOrEqual(vectors.extentToleranceM);
    });

  it("lifts the pelvis to the hip height of the rest pose", () => {
    expect(stateOf(asset, "stand").bones.pelvis!.origin.y).toBeCloseTo(vectors.poses.stand!.liftM!, 5);
  });

  it("excludes ground shapes from every extent", () => {
    const state = stateOf(asset, "stand");
    for (const view of VIEWS) {
      const ground = state.shapes[view].find((evaluated) => evaluated.shape.role === "ground")!;
      const all = extentOf([ground, ...state.shapes[view]].filter((evaluated) => evaluated.shape.role !== "ground"))!;
      expect(extentOf(state.shapes[view]), view).toEqual(all);
      expect(extentOf([ground]), view).toBeUndefined();
    }
  });
});

describe("🧊️ the consistency rules", () => {
  it.each(vectors.tolerance.cases)("uses a tolerance of $toleranceM m for $heightM x $widthM x $depthM m", ({ toleranceM: expected, ...dimensions }) => {
    expect(toleranceM({ ...sample, dimensions: { ...dimensions, massKg: 1 } })).toBeCloseTo(expected, 9);
  });

  for (const c of vectors.consistency.cases)
    it(c.name, () => {
      const shapes = Object.fromEntries(VIEWS.map((view) => [view, vectors.shapes[view].map((shape) => ({ ...shape, ...Object.assign({}, ...c.patch.filter((p) => p.view === view && p.shape === shape.id).map((p) => p.set)) }))])) as Record<Viewpoint, Shape[]>;
      const trimmed = Object.fromEntries(VIEWS.map((view) => [view, shapes[view].filter((shape) => shape.id === "block" || shape.role === "ground")])) as Record<Viewpoint, Shape[]>;
      const findings = consistency(synthetic(trimmed), vectors.scale);
      expect(findings.map((finding) => finding.check).sort(), findings.map((finding) => finding.message).join("\n")).toEqual(c.failing);
    });

  it("reports a missing view once and leaves the other views alone", () => {
    const asset = synthetic({ elevation: vectors.shapes.elevation, front: [], plan: vectors.shapes.plan });
    const findings = sweep(asset, vectors.scale);
    expect(findings.filter((finding) => finding.check === "front-missing")).toHaveLength(1);
    expect(findings.filter((finding) => finding.check.endsWith("-empty") || finding.check.endsWith("-bounds"))).toEqual([]);
  });

  it("reports a view that evaluates to ground only as empty", () => {
    const asset = synthetic({ ...vectors.shapes, plan: vectors.shapes.plan.filter((shape) => shape.role === "ground") });
    expect(sweep(asset, vectors.scale).map((finding) => finding.check)).toContain("plan-missing");
  });
});

describe("🧊️ a third-party library recomputes the extents", () => {
  const ready = requireTool("uv");

  it("measures the hand-computed shapes and the evaluated rings of the complete references with shapely", { timeout: 300_000 }, () => {
    const asset = synthetic(vectors.shapes);
    const fixtureInput: Record<string, unknown> = {};
    const ours: Record<string, Record<string, Extent>> = {};
    for (const [key, pose] of Object.entries(vectors.poses)) {
      const state = stateOf(asset, pose.pose);
      fixtureInput[key] = { shapes: vectors.shapes, bones: vectors.bones, originTravel: pose.originTravel, tipTravel: pose.tipTravel, liftM: pose.liftM ?? state.bones.pelvis!.origin.y };
      ours[key] = { ...extentsOf(state), ...Object.fromEntries(VIEWS.map((view) => [view, extentOf(state.shapes[view])!])) };
    }
    const rings: Record<string, number[][][]> = {};
    const catalogOurs: Record<string, Extent | undefined> = {};
    for (const id of vectors.catalog.complete) {
      const complete = catalog().find((candidate) => candidate.id === id)!;
      for (const scale of authoredScales(complete)) {
        const state = evaluate(complete, { kind: "pose", pose: complete.presentation.defaultPose }, select(scale));
        for (const view of VIEWS) {
          const key = `${id}/${scale}/${view}`;
          const drawn = state.shapes[view].filter((evaluated) => evaluated.shape.role !== "ground");
          rings[key] = drawn.flatMap((evaluated) => evaluated.rings.map((ring) => ring.map((p) => [p.x, p.y])));
          catalogOurs[key] = extentOf(drawn);
        }
      }
    }
    const directory = mkdtempSync(join(tmpdir(), "staffage-three-views-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ fixture: fixtureInput, rings }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️three-views/🐍️.py"), ["shapely"], [input])) as { fixture: Record<string, Record<string, Extent>>; rings: Record<string, Extent | null> };
      for (const [key, extents] of Object.entries(ours))
        for (const [name, extent] of Object.entries(extents)) for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(reference.fixture[key]![name]![edge] - extent[edge]), `${key} ${name} ${edge}`).toBeLessThanOrEqual(vectors.extentToleranceM);
      for (const c of vectors.cases) for (const [name, part] of Object.entries(c.extents)) expectPart(`${c.name} ${name} (shapely)`, reference.fixture[c.pose]![name]!, part);
      for (const [key, extent] of Object.entries(catalogOurs)) {
        expect(reference.rings[key], key).not.toBeNull();
        for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(reference.rings[key]![edge] - extent![edge]), `${key} ${edge}`).toBeLessThanOrEqual(vectors.extentToleranceM);
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});

describe("🧊️ the dimensions of the complete references", () => {
  const assets = catalog().filter((asset) => vectors.catalog.complete.includes(asset.id));

  it("has the complete references", () => expect(assets.map((asset) => asset.id).sort()).toEqual([...vectors.catalog.complete].sort()));

  for (const asset of assets)
    for (const scale of authoredScales(asset))
      it(`reads widthM as the elevation x extent, depthM as the lateral extent and heightM as the height of ${asset.id} at 1:${scale}`, () => {
        const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, select(scale));
        const [e, f, p] = [state.shapes.elevation, state.shapes.front, state.shapes.plan].map((shapes) => extentOf(shapes)!);
        const tolerance = toleranceM(asset);
        expect(e!.maxX - e!.minX, "elevation x extent = widthM").toBeCloseTo(asset.dimensions.widthM, 1);
        expect(Math.abs(e!.maxX - e!.minX - asset.dimensions.widthM)).toBeLessThanOrEqual(tolerance);
        expect(Math.abs(p!.maxY - p!.minY - asset.dimensions.depthM), "plan y extent = depthM").toBeLessThanOrEqual(tolerance);
        expect(Math.abs(f!.maxX - f!.minX - asset.dimensions.depthM), "front x extent = depthM").toBeLessThanOrEqual(tolerance);
        expect(Math.abs(e!.maxY - e!.minY - asset.dimensions.heightM), "elevation height = heightM").toBeLessThanOrEqual(tolerance);
        expect(Math.abs(f!.maxY - f!.minY - asset.dimensions.heightM), "front height = heightM").toBeLessThanOrEqual(tolerance);
      });
});

describe("🧊️ every asset of the catalog draws the side, the front and the top view", { timeout: 600_000 }, () => {
  const assets = catalog();

  it("has the authored representations to guard", () => {
    expect(assets.length).toBeGreaterThanOrEqual(100);
    expect(assets.flatMap(authoredScales).length).toBeGreaterThanOrEqual(500);
  });

  it.each(assets.map((asset) => [asset.id, asset] as const))("%s draws non-empty finite geometry in every view for every pose and clip instant at every authored scale", (_, asset) => {
    const findings = authoredScales(asset).flatMap((scale) => sweep(asset, scale));
    expect(findings.map((finding) => `[${finding.check}] ${finding.message}`).join("\n")).toBe("");
  });

  it.each(assets.map((asset) => [asset.id, asset] as const))("%s keeps its three views one object at the default pose at every authored scale", (_, asset) => {
    const findings = authoredScales(asset).flatMap((scale) => consistency(asset, scale));
    expect(findings.map((finding) => `[${finding.check}] ${finding.message}`).join("\n")).toBe("");
  });
});
