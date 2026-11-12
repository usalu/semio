/** 🧊️ The declarative 3D volume: conformance of the sample figure's `volume` and of broken variants to `🧬️schema/🔣️.json` (ajv) and the semantic rules of the owned validator, one invalid vector per rule.
 *
 * THE REFERENCES are ajv (JSON Schema), shapely (profile validity, 🐍️.py) and the exact predicates of `robust-predicates` (segment intersection), plus the hand-computed bounding box of the sample figure from its rig.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️volume-schema/🔣️.json — the vectors
 */
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import Ajv from "ajv";
import { orient2d } from "robust-predicates";
import { beforeAll, describe, expect, it } from "vitest";
import * as twin from "../../🧬️schema/🟦️.ts";
import type { Asset, Vector2, VolumeSolid } from "../../🧬️schema/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { defaultDepictionOptions, depict, loadAxonometric } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { parseArguments } from "../../🔨️modules/📤️export/🎬️cli/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { holeInside, holesOverlap, profileVerdict, segmentsMeet, volumeIssues } from "../../🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
import { restFrames } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

beforeAll(loadAxonometric);

type Operation = { op: "replace" | "add" | "remove"; path: string; value?: unknown };
type Polygon = { name: string; points: Vector2[]; holes?: Vector2[][]; verdict: string; holesVerdict?: string; oracle: boolean };
type Polyline = { name: string; points: Vector2[]; verdict: string; oracle: boolean };
type Vectors = {
  defs: string[];
  rules: string[];
  solidKinds: string[];
  bounds: { toleranceRelative: number; toleranceM: number };
  valid: { name: string; patch: Operation[] }[];
  invalid: { name: string; rule: string | null; schema: "accepts" | "rejects"; absent?: string[]; patch: Operation[] }[];
  polygons: Polygon[];
  polylines: Polyline[];
};

const read = (path: string) => JSON.parse(readFileSync(new URL(path, import.meta.url), "utf8"));
const schema = read("../../🧬️schema/🔣️.json");
const vectors = fixture<Vectors>("🧊️volume-schema");
const sample: Asset = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const validator = (name: string) => ajv.compile({ $ref: `${schema.$id}#/$defs/${name}` });
const assetSchema = validator("Asset");

function patch(document: any, operations: readonly Operation[]): any {
  const root = structuredClone(document);
  for (const { op, path, value } of operations) {
    const segments = path.split("/").slice(1);
    let node = root;
    const at = (n: any, s: string) => (Array.isArray(n) ? (s.startsWith("id=") ? n.findIndex((i: any) => i.id === s.slice(3)) : Number(s)) : s);
    for (const segment of segments.slice(0, -1)) node = node[at(node, segment)];
    const key = at(node, segments[segments.length - 1]!);
    if (op === "remove") Array.isArray(node) ? node.splice(key as number, 1) : delete node[key];
    else if (op === "add" && Array.isArray(node)) node.push(structuredClone(value));
    else node[key] = structuredClone(value);
  }
  return root;
}

const codes = (asset: Asset): string[] => volumeIssues(asset).map((issue) => issue.code);
const solidOf = (id: string): VolumeSolid => sample.volume!.solids.find((solid) => solid.id === id)!;

type Box = { min: [number, number, number]; max: [number, number, number] };

function localBox(solid: VolumeSolid): Box {
  const lo: [number, number, number] = [Infinity, Infinity, Infinity];
  const hi: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  const cover = (point: readonly number[], radius: readonly [number, number, number] = [0, 0, 0]): void =>
    point.forEach((value, axis) => {
      lo[axis] = Math.min(lo[axis]!, value - radius[axis]!);
      hi[axis] = Math.max(hi[axis]!, value + radius[axis]!);
    });
  switch (solid.kind) {
    case "box":
      cover([0, 0, 0], [solid.size[0] / 2, solid.size[1] / 2, solid.size[2] / 2]);
      break;
    case "ellipsoid":
      cover([0, 0, 0], solid.radii);
      break;
    case "capsule": {
      const lateral = solid.lateralScale ?? 1;
      for (const [p, r] of [[solid.p0, solid.r0], [solid.p1, solid.r1]] as const) cover(p, [r, r, r * lateral]);
      break;
    }
    case "revolve": {
      const reach = Math.max(...solid.profile.map((q) => q[1]));
      const [min, max] = [Math.min(...solid.profile.map((q) => q[0])), Math.max(...solid.profile.map((q) => q[0]))];
      cover([solid.origin[0] + min, solid.origin[1], solid.origin[2]], [0, reach, reach * (solid.lateralScale ?? 1)]);
      cover([solid.origin[0] + max, solid.origin[1], solid.origin[2]], [0, reach, reach * (solid.lateralScale ?? 1)]);
      break;
    }
    case "extrude": {
      const a = solid.profile.map((q) => q[0]);
      const b = solid.profile.map((q) => q[1]);
      const axes = { xy: [0, 1, 2], yz: [1, 2, 0], zx: [2, 0, 1] }[solid.plane];
      const range = (axis: number, from: number, to: number): void => {
        lo[axis] = Math.min(lo[axis]!, from);
        hi[axis] = Math.max(hi[axis]!, to);
      };
      range(axes[0]!, Math.min(...a), Math.max(...a));
      range(axes[1]!, Math.min(...b), Math.max(...b));
      range(axes[2]!, solid.from, solid.to);
      break;
    }
    case "loft":
      for (const section of solid.sections) cover(section.at, [0, section.ry, section.rx]);
      break;
  }
  const t = solid.transform?.translation ?? [0, 0, 0];
  const step = solid.repeat === undefined ? [0, 0, 0] : solid.repeat.translation?.map((value) => value * (solid.repeat!.count - 1)) ?? [0, 0, 0];
  return { min: lo.map((v, i) => v + t[i]! + Math.min(0, step[i]!)) as Box["min"], max: hi.map((v, i) => v + t[i]! + Math.max(0, step[i]!)) as Box["max"] };
}

function worldBox(asset: Asset): Box {
  const frames = restFrames(asset.rig);
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (const solid of asset.volume!.solids) {
    const { frame } = frames[solid.bone]!;
    const box = localBox(solid);
    for (const corner of [0, 1, 2, 3, 4, 5, 6, 7]) {
      const local = [corner & 1 ? box.max[0] : box.min[0], corner & 2 ? box.max[1] : box.min[1], corner & 4 ? box.max[2] : box.min[2]];
      for (let axis = 0; axis < 3; axis++) {
        const value = frame.o[axis]! + local[0]! * frame.u[axis]! + local[1]! * frame.v[axis]! + local[2]! * frame.w[axis]!;
        min[axis] = Math.min(min[axis]!, value);
        max[axis] = Math.max(max[axis]!, value);
      }
    }
  }
  return { min, max };
}

const exactSide = (a: Vector2, b: Vector2, c: Vector2): number => Math.sign(orient2d(a[0], a[1], b[0], b[1], c[0], c[1]));
const inBox = (a: Vector2, b: Vector2, c: Vector2): boolean => Math.min(a[0], b[0]) <= c[0] && c[0] <= Math.max(a[0], b[0]) && Math.min(a[1], b[1]) <= c[1] && c[1] <= Math.max(a[1], b[1]);
function exactMeet(a: Vector2, b: Vector2, c: Vector2, d: Vector2): boolean {
  const [d1, d2, d3, d4] = [exactSide(a, b, c), exactSide(a, b, d), exactSide(c, d, a), exactSide(c, d, b)];
  if (d1 * d2 < 0 && d3 * d4 < 0) return true;
  return (d1 === 0 && inBox(a, b, c)) || (d2 === 0 && inBox(a, b, d)) || (d3 === 0 && inBox(c, d, a)) || (d4 === 0 && inBox(c, d, b));
}

describe("🧊️ volume schema and rules", () => {
  it("restates the volume enums and constants of the twin and defines every new type", () => {
    for (const name of vectors.defs) expect(schema.$defs[name], name).toBeDefined();
    expect(schema.$defs.VolumeSolid.oneOf.map((branch: any) => branch.properties.kind.const)).toEqual([...twin.VOLUME_SOLID_KINDS]);
    expect(vectors.solidKinds).toEqual([...twin.VOLUME_SOLID_KINDS]);
    expect(schema.$defs.VolumeSolid.oneOf.find((branch: any) => branch.properties.kind.const === "extrude").properties.plane.enum).toEqual([...twin.VOLUME_PLANES]);
    expect(schema.$defs.VolumeRepeat.properties.count.maximum).toBe(twin.VOLUME_REPEAT_MAX);
    expect(schema.$defs.VolumeSolid.oneOf[0].properties.weights.maxItems).toBe(twin.VOLUME_WEIGHTS_MAX);
    expect(schema.$defs.Viewpoint.enum).toEqual([...twin.VIEWPOINTS]);
    expect(schema.$defs.ExportFormat.enum).toEqual([...twin.EXPORT_FORMATS]);
    expect(schema.$defs.ExportFormat.enum).toEqual(expect.arrayContaining(["glb", "fbx", "model"]));
    expect(twin.PLANAR_VIEWPOINTS).toEqual(["elevation", "front", "plan"]);
    expect(schema.$defs.Asset.properties.volume).toEqual(expect.objectContaining({ $ref: "#/$defs/Volume" }));
    expect(schema.$defs.Volume.properties.status.const).toBe("authored");
    expect(schema.$defs.Volume.required).toEqual(["status", "solids", "decisions"]);
  });

  it("gives every solid kind the common fields as required", () => {
    for (const branch of schema.$defs.VolumeSolid.oneOf) {
      expect(branch.additionalProperties).toBe(false);
      for (const field of ["id", "kind", "bone", "material", "role", "tone"]) expect(branch.required, branch.properties.kind.const).toContain(field);
      for (const field of ["region", "regions", "maxDenominator", "weights", "transform", "repeat"]) expect(branch.properties[field], field).toBeDefined();
    }
  });

  it("validates the hand-authored volume of the sample figure with ajv and the owned rules", () => {
    expect(assetSchema(sample), JSON.stringify(assetSchema.errors?.slice(0, 3))).toBe(true);
    expect(volumeIssues(sample)).toEqual([]);
    expect(validateAsset(sample).filter((issue) => issue.code.startsWith("volume-"))).toEqual([]);
    expect(new Set(sample.volume!.solids.map((solid) => solid.kind))).toEqual(new Set(twin.VOLUME_SOLID_KINDS));
    expect(sample.volume!.solids.some((solid) => solid.weights !== undefined && solid.repeat === undefined)).toBe(true);
    expect(sample.volume!.solids.some((solid) => solid.repeat !== undefined && solid.maxDenominator === 50)).toBe(true);
  });

  it("agrees with the dimensions of the sample figure within the extent tolerance", () => {
    const { min, max } = worldBox(sample);
    const { heightM, widthM, depthM } = sample.dimensions;
    const measured = { heightM: max[1] - min[1], widthM: max[0] - min[0], depthM: max[2] - min[2] };
    for (const [name, declared] of Object.entries({ heightM, widthM, depthM }) as [keyof typeof measured, number][])
      expect(Math.abs(measured[name] - declared), `${name} ${measured[name]} against ${declared}`).toBeLessThanOrEqual(Math.max(vectors.bounds.toleranceRelative * declared, vectors.bounds.toleranceM));
    expect(min[1]).toBeCloseTo(-0.96, 2);
  });

  it("binds every solid of the sample figure to its bone, material and thermal region", () => {
    const bones = new Set(sample.rig.bones.map((bone) => bone.id));
    for (const solid of sample.volume!.solids) {
      expect(bones.has(solid.bone), solid.id).toBe(true);
      expect(sample.materials.some((material) => material.id === solid.material), solid.id).toBe(true);
      expect(sample.thermal.regions.some((region) => region.id === solid.region), solid.id).toBe(true);
    }
    expect(solidOf("arm-l").kind).toBe("capsule");
  });

  for (const c of vectors.valid)
    it(`accepts: ${c.name}`, () => {
      const asset = patch(sample, c.patch) as Asset;
      expect(assetSchema(asset), JSON.stringify(assetSchema.errors?.slice(0, 3))).toBe(true);
      expect(volumeIssues(asset)).toEqual([]);
    });

  for (const c of vectors.invalid)
    it(`rejects: ${c.name}`, () => {
      const asset = patch(sample, c.patch) as Asset;
      expect(assetSchema(asset), `schema should ${c.schema.slice(0, -1)}`).toBe(c.schema === "accepts");
      if (c.rule === null) return;
      const found = codes(asset);
      expect(found, JSON.stringify(volumeIssues(asset).map((issue) => issue.path))).toContain(c.rule);
      for (const absent of c.absent ?? []) expect(found).not.toContain(absent);
      for (const issue of volumeIssues(asset)) expect(issue.message.en.length * issue.message.de.length).toBeGreaterThan(0);
    });

  it("has an invalid vector for every rule id, and each vector breaks only the rule it names", () => {
    const named = new Set(vectors.invalid.flatMap((c) => (c.rule === null ? [] : [c.rule])));
    for (const rule of vectors.rules.filter((rule) => rule !== "volume-non-finite")) expect(named.has(rule), rule).toBe(true);
    for (const c of vectors.invalid.filter((candidate) => candidate.rule !== null)) expect([...new Set(codes(patch(sample, c.patch) as Asset))], c.name).toEqual([c.rule]);
  });

  it("depicts the axonometric viewpoint (🧊️axonometric-depiction) and parses it on the command line", () => {
    const pose = sample.poses[0]!.id;
    const state = evaluate(sample, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 100 } });
    expect(depict(state, defaultDepictionOptions("dark-line", "en", 100, "axonometric")).metadata.viewpoint).toBe("axonometric");
    expect(parseArguments(["--asset", sample.id, "--pose", pose, "--view", "dark-line", "--scale", "100", "--format", "svg", "--viewpoint", "axonometric", "--lang", "en", "--out", "out"]).viewpoint).toBe("axonometric");
  });

  it("names a coordinate that is not finite", () => {
    const broken = patch(sample, [{ op: "replace", path: "/volume/solids/id=arm-l/p1", value: [Infinity, 0, 0.225] }]) as Asset;
    expect(codes(broken)).toContain("volume-non-finite");
  });

  it("raises the volume rules through validateAsset with their path", () => {
    const broken = patch(sample, [{ op: "replace", path: "/volume/solids/id=head/bone", value: "no-such-bone" }]) as Asset;
    const issue = validateAsset(broken).find((found) => found.code === "volume-bone");
    expect(issue?.path).toBe(`volume.solids[${sample.volume!.solids.findIndex((solid) => solid.id === "head")}].bone`);
  });

  for (const c of vectors.polygons)
    it(`judges the polygon: ${c.name}`, () => {
      expect(profileVerdict(c.points, true)).toBe(c.verdict);
      if (c.holes === undefined) return;
      const holeVerdicts = c.holes.map((hole) => profileVerdict(hole, true));
      expect(holeVerdicts.every((verdict) => verdict === "simple")).toBe(true);
      const outside = c.holes.some((hole) => !holeInside(c.points, hole));
      const overlap = c.holes.some((hole, k) => c.holes!.slice(k + 1).some((other) => holesOverlap(hole, other)));
      expect(outside ? "hole-outside" : overlap ? "holes-overlap" : "ok").toBe(c.holesVerdict);
    });

  for (const c of vectors.polylines)
    it(`judges the open profile: ${c.name}`, () => {
      expect(profileVerdict(c.points, false)).toBe(c.verdict);
    });

  it("meets the segments exactly as the exact predicates of robust-predicates do", () => {
    let state = 20260611;
    const next = (): number => {
      state = (Math.imul(state, 1103515245) + 12345) >>> 0;
      return (state >>> 8) % 9;
    };
    for (let i = 0; i < 4000; i++) {
      const [a, b, c, d]: Vector2[] = [[next(), next()], [next(), next()], [next(), next()], [next(), next()]];
      if ((a[0] === b[0] && a[1] === b[1]) || (c[0] === d[0] && c[1] === d[1])) continue;
      expect(segmentsMeet(a, b, c, d), `${a} ${b} ${c} ${d}`).toBe(exactMeet(a, b, c, d));
    }
  });

  it("equals the verdicts of shapely on every polygon and polyline", { timeout: 300_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-volume-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ polygons: vectors.polygons, polylines: vectors.polylines }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️volume-schema/🐍️.py"), "geometry", [input])) as { polygons: Record<string, { valid: boolean }>; polylines: Record<string, { simple: boolean }> };
      for (const c of vectors.polygons.filter((candidate) => candidate.oracle)) {
        const owned = profileVerdict(c.points, true) === "simple" && (c.holes ?? []).every((hole) => profileVerdict(hole, true) === "simple" && holeInside(c.points, hole)) && !(c.holes ?? []).some((hole, k) => c.holes!.slice(k + 1).some((other) => holesOverlap(hole, other)));
        expect(owned, `${c.name} (shapely)`).toBe(reference.polygons[c.name]!.valid);
      }
      for (const c of vectors.polylines.filter((candidate) => candidate.oracle)) expect(profileVerdict(c.points, false) === "simple", `${c.name} (shapely)`).toBe(reference.polylines[c.name]!.simple);
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
});
