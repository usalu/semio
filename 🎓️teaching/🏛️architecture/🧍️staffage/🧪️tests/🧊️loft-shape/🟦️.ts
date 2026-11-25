/** 🧵️ The `loft` shape: superelliptic sections in the frame of a bone, carried by the full 3D frame of the bone and drawn as one body in the side elevation, the front elevation and the plan.
 *
 * THE REFERENCES are gl-matrix (section axes turned by the section quaternion and carried through the hand-written basis), the kernel's tessellation of the same sections as a volume loft, shapely with numpy and scipy (🐍️.py) and ajv; the numbers of the vectors are hand-computed.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️loft-shape/🔣️.json — the vectors
 */
import Ajv from "ajv";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { quat, vec3 } from "gl-matrix";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, LoftSection, ScaleDenominator, Shape, VolumeSolid } from "../../🧬️schema/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { smallestDimension, thinnestDrawn } from "../../🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { copyMesh } from "../../🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { LoftInPaperUnitsError, evaluate, loftRings, shapeDepth } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { STAFFAGE_ROOT, fixture, requireTool } from "../🧰️support/🧰️.ts";
import { python } from "../🧰️support/🐍️.ts";

type View = "elevation" | "front" | "plan";
type Extent = { minX: number; maxX: number; minY: number; maxY: number };
type Vectors = {
  scale: number;
  bone: string;
  segments: number;
  toleranceM: number;
  kernelToleranceM: number;
  areaToleranceM2: number;
  lofts: Record<string, { sections: LoftSection[] }>;
  cases: { name: string; pose: string; basis: Record<"u" | "v" | "w", number[]>; extents: Record<string, Record<View, Extent>>; depth: Record<string, { front: number; plan: number }> }[];
  refused: { name: string; code: string; schema: boolean; geometry: Record<string, unknown> }[];
};

const VIEWS = ["elevation", "front", "plan"] as const;
const vectors = fixture<Vectors>("🧊️loft-shape");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const key = String(vectors.scale) as keyof Asset["representations"];
const authored = sample.representations[key] as AuthoredRepresentation;
const selector = { kind: "scale", scale: { denominator: vectors.scale } } as const;

const loft = (id: string, geometry: Record<string, unknown>): Shape => ({ id, kind: "loft", bone: vectors.bone, role: "body", z: 1, filled: true, tone: 0.5, ...geometry }) as unknown as Shape;
const withShapes = (shapes: readonly Shape[]): Asset => ({ ...sample, representations: { ...sample.representations, [key]: { ...authored, geometry: { ...authored.geometry, elevation: [...shapes], front: [...shapes], plan: [...shapes] } } } as Asset["representations"] });
const lofts = Object.entries(vectors.lofts).map(([id, geometry]) => loft(id, geometry));
const patched = withShapes(lofts);

const extentOf = (points: readonly { x: number; y: number }[], dx: number, dy: number): Extent => ({ minX: Math.min(...points.map((p) => p.x)) - dx, maxX: Math.max(...points.map((p) => p.x)) - dx, minY: Math.min(...points.map((p) => p.y)) - dy, maxY: Math.max(...points.map((p) => p.y)) - dy });
const areaOf = (rings: readonly (readonly { x: number; y: number }[])[]): number => rings.reduce((sum, ring) => sum + Math.abs(ring.reduce((a, p, i) => a + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0)) / 2, 0);

function measured(asset: Asset, pose: string): Record<string, Record<View, Extent & { area: number; rings: number }>> {
  const state = evaluate(asset, { kind: "pose", pose }, selector);
  const origin = state.bones.pelvis!.origin;
  const out: Record<string, Record<View, Extent & { area: number; rings: number }>> = {};
  for (const view of VIEWS)
    for (const evaluated of state.shapes[view]) {
      const rings = evaluated.rings;
      (out[evaluated.shape.id] ??= {} as Record<View, Extent & { area: number; rings: number }>)[view] = { ...extentOf(rings.flat(), view === "front" ? 0 : origin.x, view === "plan" ? 0 : origin.y), area: areaOf(rings), rings: rings.length };
    }
  return out;
}

const expectExtent = (label: string, actual: Extent, expected: Extent, tolerance: number): void => {
  for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(actual[edge] - expected[edge]), `${label} ${edge}`).toBeLessThanOrEqual(tolerance);
};

function glMatrixExtents(sections: readonly LoftSection[], basis: Record<"u" | "v" | "w", number[]>): Record<View, Extent> {
  const world = (p: vec3): vec3 => vec3.fromValues(p[0]! * basis.u[0]! + p[1]! * basis.v[0]! + p[2]! * basis.w[0]!, p[0]! * basis.u[1]! + p[1]! * basis.v[1]! + p[2]! * basis.w[1]!, p[0]! * basis.u[2]! + p[1]! * basis.v[2]! + p[2]! * basis.w[2]!);
  const centres = sections.map((section) => vec3.fromValues(section.at[0], section.at[1], section.at[2]));
  const reach = [0, 1, 2].map(() => [Infinity, -Infinity]);
  sections.forEach((section, i) => {
    let x = vec3.create();
    let y = vec3.create();
    if (section.rotation !== undefined) {
      const turn = quat.fromValues(section.rotation[0], section.rotation[1], section.rotation[2], section.rotation[3]);
      vec3.transformQuat(x, [1, 0, 0], turn);
      vec3.transformQuat(y, [0, 1, 0], turn);
    } else {
      const tangent = vec3.normalize(vec3.create(), vec3.subtract(vec3.create(), centres[Math.min(sections.length - 1, i + 1)]!, centres[Math.max(0, i - 1)]!));
      x = vec3.scaleAndAdd(vec3.create(), [1, 0, 0], tangent, -tangent[0]!);
      if (vec3.length(x) < 1e-6) x = vec3.scaleAndAdd(vec3.create(), [0, 0, 1], tangent, -tangent[2]!);
      vec3.normalize(x, x);
      y = vec3.cross(vec3.create(), tangent, x);
    }
    const [centre, ax, ay] = [world(centres[i]!), world(x), world(y)];
    for (let k = 0; k < 3; k++) {
      const half = Math.hypot(section.rx * ax[k]!, section.ry * ay[k]!);
      reach[k] = [Math.min(reach[k]![0]!, centre[k]! - half), Math.max(reach[k]![1]!, centre[k]! + half)];
    }
  });
  const box = (a: number, b: number): Extent => ({ minX: reach[a]![0]!, maxX: reach[a]![1]!, minY: reach[b]![0]!, maxY: reach[b]![1]! });
  return { elevation: box(0, 1), front: box(2, 1), plan: box(0, 2) };
}

describe("🧵️ a loft in the three views", () => {
  for (const c of vectors.cases)
    it(`places the sections as the vectors say: ${c.name}`, () => {
      const found = measured(patched, c.pose);
      for (const [id, views] of Object.entries(c.extents)) for (const view of VIEWS) expectExtent(`${id} ${view}`, found[id]![view], views[view], vectors.toleranceM);
    });

  it("draws every loft as one outline per view", () => {
    for (const c of vectors.cases) for (const views of Object.values(measured(patched, c.pose))) for (const view of VIEWS) expect(views[view].rings).toBe(1);
  });

  for (const c of vectors.cases)
    it(`equals the gl-matrix recomputation of an elliptic loft: ${c.name}`, () => {
      const found = measured(patched, c.pose);
      const reference = glMatrixExtents(vectors.lofts.limb!.sections, c.basis);
      for (const view of VIEWS) expectExtent(`limb ${view}`, found.limb![view], reference[view], vectors.kernelToleranceM);
    });

  it("draws a section as the stated number of points, the first on its x axis", () => {
    const rings = loftRings(lofts[0] as Extract<Shape, { kind: "loft" }>);
    expect(rings.map((ring) => ring.length)).toEqual(vectors.lofts.limb!.sections.map(() => vectors.segments));
    const first = vectors.lofts.limb!.sections[0]!;
    expect(rings[0]![0]!.map((value) => Number(value.toFixed(12)) + 0)).toEqual([first.at[0], first.at[1] + first.rx, first.at[2]]);
  });

  it("has the section frames of the volume loft: the rings reach as far as the kernel's tessellation in every direction", () => {
    const directions: number[][] = [];
    for (let i = 0; i < 7; i++) for (let j = 0; j < 12; j++) directions.push([Math.cos((Math.PI * i) / 6), Math.sin((Math.PI * i) / 6) * Math.cos((Math.PI * j) / 6), Math.sin((Math.PI * i) / 6) * Math.sin((Math.PI * j) / 6)]);
    for (const [id, geometry] of Object.entries(vectors.lofts)) {
      const solid = { id, kind: "loft", bone: vectors.bone, material: sample.materials[0]!.id, role: "body", tone: 0.5, sections: geometry.sections, capStart: true, capEnd: true } as unknown as VolumeSolid;
      const { positions } = copyMesh(solid, { chordM: 0.0001, minSegments: 64, maxSegments: 128 });
      const rings = loftRings(loft(id, geometry) as Extract<Shape, { kind: "loft" }>).flat();
      for (const d of directions) {
        let kernel = -Infinity;
        for (let k = 0; k < positions.length; k += 3) kernel = Math.max(kernel, positions[k]! * d[0]! + positions[k + 1]! * d[1]! + positions[k + 2]! * d[2]!);
        const drawn = Math.max(...rings.map((p) => p[0] * d[0]! + p[1] * d[1]! + p[2] * d[2]!));
        expect(Math.abs(kernel - drawn), `${id} along ${d.map((value) => value.toFixed(2)).join(" ")}`).toBeLessThanOrEqual(vectors.kernelToleranceM);
      }
    }
  });

  it("equals the outlines that shapely builds from the sections numpy and scipy place", { timeout: 300_000 }, () => {
    requireTool("uv");
    const directory = mkdtempSync(join(tmpdir(), "staffage-loft-"));
    try {
      const input = join(directory, "input.json");
      writeFileSync(input, JSON.stringify({ lofts: vectors.lofts, cases: vectors.cases, segments: vectors.segments }));
      const reference = JSON.parse(python(join(STAFFAGE_ROOT, "🧪️tests/🧊️loft-shape/🐍️.py"), "geometry", [input])) as Record<string, Record<string, Record<View, Extent & { area: number }>>>;
      for (const c of vectors.cases) {
        const found = measured(patched, c.pose);
        for (const id of Object.keys(vectors.lofts))
          for (const view of VIEWS) {
            expectExtent(`${c.pose} ${id} ${view} (shapely)`, found[id]![view], reference[c.pose]![id]![view], vectors.toleranceM);
            expect(Math.abs(found[id]![view].area - reference[c.pose]![id]![view].area), `${c.pose} ${id} ${view} area`).toBeLessThanOrEqual(vectors.areaToleranceM2);
          }
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });

  it("is one body in the three views at every pose", () => {
    for (const pose of sample.poses) {
      const state = evaluate(patched, { kind: "pose", pose: pose.id }, selector);
      for (const id of Object.keys(vectors.lofts)) {
        const [elevation, front, plan] = VIEWS.map((view) => extentOf(state.shapes[view].find((evaluated) => evaluated.shape.id === id)!.rings.flat(), 0, 0));
        expect(Math.abs(elevation!.minY - front!.minY) + Math.abs(elevation!.maxY - front!.maxY), `${pose.id} ${id} height`).toBeLessThanOrEqual(1e-9);
        expect(Math.abs(elevation!.minX - plan!.minX) + Math.abs(elevation!.maxX - plan!.maxX), `${pose.id} ${id} forward`).toBeLessThanOrEqual(1e-9);
        expect(Math.abs(front!.minX - plan!.minY) + Math.abs(front!.maxX - plan!.maxY), `${pose.id} ${id} lateral`).toBeLessThanOrEqual(1e-9);
      }
    }
  });

  it("is the rigid loft when it is skinned wholly to its own bone", () => {
    const skinned = withShapes([loft("limb", { ...vectors.lofts.limb, weights: [{ bone: vectors.bone, weight: 1 }] })]);
    for (const c of vectors.cases) {
      const [rigid, bound] = [measured(patched, c.pose), measured(skinned, c.pose)];
      for (const view of VIEWS) expectExtent(`${c.pose} ${view}`, bound.limb![view], rigid.limb![view], 1e-9);
    }
  });

  it("states as the thinnest drawing of its bone the width of its body (the smallest dimension of the volume solid), not its tapered end section", () => {
    const sections = vectors.lofts.limb!.sections;
    const tapered = [...sections, { ...sections.at(-1)!, at: [sections.at(-1)!.at[0] + 0.002, sections.at(-1)!.at[1], sections.at(-1)!.at[2]] as const, rx: 0.0001, ry: 0.0001 }] as readonly LoftSection[];
    const solid = { id: "limb", kind: "loft", bone: vectors.bone, material: sample.materials[0]!.id, role: "body", tone: 0.5, sections: tapered, capStart: true, capEnd: true } as unknown as VolumeSolid;
    const drawn = thinnestDrawn(withShapes([loft("limb", { sections: tapered })]), vectors.scale as ScaleDenominator).get(vectors.bone)!;
    expect(drawn).toBeLessThanOrEqual(smallestDimension(solid) + 1e-9);
    expect(drawn).toBeGreaterThanOrEqual(0.9 * smallestDimension(solid));
  });

  for (const c of vectors.cases)
    it(`ranks the loft by its nearest point: ${c.name}`, () => {
      const state = evaluate(patched, { kind: "pose", pose: c.pose }, selector);
      const origin = state.bones.pelvis!.origin;
      for (const [id, depth] of Object.entries(c.depth)) {
        expect(Math.abs(shapeDepth(state, state.shapes.front.find((evaluated) => evaluated.shape.id === id)!.shape, "front") - origin.x - depth.front), `${id} front`).toBeLessThanOrEqual(vectors.toleranceM);
        expect(Math.abs(shapeDepth(state, state.shapes.plan.find((evaluated) => evaluated.shape.id === id)!.shape, "plan") - origin.y - depth.plan), `${id} plan`).toBeLessThanOrEqual(vectors.toleranceM);
      }
    });
});

describe("🧵️ a loft that cannot be a body is refused", () => {
  const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(schema);
  const conforms = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });

  it("accepts the lofts of the vectors in the schema and in the validator", () => {
    expect(conforms(patched), JSON.stringify(conforms.errors?.slice(0, 3))).toBe(true);
    expect(validateAsset(patched).filter((issue) => issue.code.startsWith("loft-") || issue.code === "shape-field")).toEqual([]);
  });

  for (const c of vectors.refused) {
    const asset = withShapes([loft("limb", c.geometry)]);

    it(`${c.name}: the validator reports ${c.code} at the shape in both languages`, () => {
      const issues = validateAsset(asset).filter((issue) => issue.code === c.code);
      expect(issues.length).toBeGreaterThan(0);
      for (const issue of issues) {
        expect(issue.path).toContain(".geometry.");
        expect(issue.message.en.length).toBeGreaterThan(0);
        expect(issue.message.de.length).toBeGreaterThan(0);
      }
    });

    it(`${c.name}: ajv ${c.schema ? "accepts what only the validator can judge" : "rejects the document"}`, () => {
      expect(conforms(asset)).toBe(c.schema);
    });
  }

  it("has no paper-unit drawing: evaluate throws a named error and the validator reports loft-in-paper-symbol", () => {
    const shape = { ...loft("limb", vectors.lofts.limb!), bone: sample.rig.root } as Shape;
    const asset = { ...sample, symbols: [{ ...sample.symbols[0]!, id: "loft-symbol", units: "paper-mm", geometry: { elevation: [shape], front: [shape], plan: [shape] } }] } as unknown as Asset;
    const run = (): unknown => evaluate(asset, { kind: "pose", pose: "stand" }, { kind: "symbol", symbol: "loft-symbol" });
    expect(run).toThrow(LoftInPaperUnitsError);
    try {
      run();
    } catch (error) {
      const refusal = error as LoftInPaperUnitsError;
      expect([refusal.asset, refusal.shape]).toEqual([sample.id, "limb"]);
      expect(refusal.reason.en).toContain("limb");
      expect(refusal.reason.de).toContain("limb");
    }
    expect(validateAsset(asset).some((issue) => issue.code === "loft-in-paper-symbol")).toBe(true);
  });
});
