/** 🧊️ The `sweep` of an extruded solid is required: `w` and `v` place the profile in different planes, and a missing or unknown sweep is refused by name (`SolidSweepError` from `evaluate` and the multiview resolver, `solid-sweep` from the semantic validator, a `required` error from ajv) instead of being drawn transposed.
 *
 * THE REFERENCE is gl-matrix, an independent linear-algebra library: it carries the corners of the swept profile through the hand-written bone basis of the vectors, and ajv rejects the same documents by the schema; the numbers of the vectors are hand-computed.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/🧊️solid-sweep/🔣️.json — the vectors
 */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { mat4, vec3 } from "gl-matrix";
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, Shape } from "../../🧬️schema/🟦️.ts";
import { validateAsset } from "../../🔨️modules/🗂️catalog/🟦️.ts";
import { SolidSweepError, evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { resolveViews, type DrawOptions } from "../../🔨️modules/🔭️multiview/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { drawOptions } from "../🔭️multiview-coverage/🧰️.ts";

type Extent = { minX: number; maxX: number; minY: number; maxY: number };
type Vectors = {
  scale: number;
  pose: string;
  toleranceM: number;
  bone: string;
  profile: { x: number; y: number; w: number; h: number };
  span: { z0: number; z1: number };
  basis: { u: number[]; v: number[]; w: number[] };
  cases: { name: string; sweep: "v" | "w"; front: Extent; plan: Extent }[];
  refused: { name: string; solid: Record<string, unknown> }[];
};

const vectors = fixture<Vectors>("🧊️solid-sweep");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const key = String(vectors.scale) as keyof Asset["representations"];
const authored = sample.representations[key] as AuthoredRepresentation;

const slat = (solid: Record<string, unknown>): Shape => ({ id: "slat", kind: "rect", bone: vectors.bone, role: "body", z: 1, filled: true, tone: 0.5, ...vectors.profile, solid }) as unknown as Shape;
const withSlat = (solid: Record<string, unknown>): Asset => ({ ...sample, representations: { ...sample.representations, [key]: { ...authored, geometry: { ...authored.geometry, front: [slat(solid)], plan: [slat(solid)] } } } as Asset["representations"] });
const sweptAsset = (sweep: "v" | "w"): Asset => withSlat({ kind: "extruded", sweep, ...vectors.span });

function measured(asset: Asset): Record<"front" | "plan", Extent> {
  const state = evaluate(asset, { kind: "pose", pose: vectors.pose }, { kind: "scale", scale: { denominator: vectors.scale } });
  const origin = state.bones.pelvis!.origin;
  const extent = (view: "front" | "plan"): Extent => {
    const points = state.shapes[view].find((e) => e.shape.id === "slat")!.rings.flat();
    const xs = points.map((p) => p.x - (view === "plan" ? origin.x : 0));
    const ys = points.map((p) => p.y - (view === "front" ? origin.y : 0));
    return { minX: Math.min(...xs), maxX: Math.max(...xs), minY: Math.min(...ys), maxY: Math.max(...ys) };
  };
  return { front: extent("front"), plan: extent("plan") };
}

function glMatrixExtent(sweep: "v" | "w"): Record<"front" | "plan", Extent> {
  const { u, v, w } = vectors.basis;
  const frame = mat4.fromValues(u[0]!, u[1]!, u[2]!, 0, v[0]!, v[1]!, v[2]!, 0, w[0]!, w[1]!, w[2]!, 0, 0, 0, 0, 1);
  const { x, y, w: width, h: height } = vectors.profile;
  const corners = [[x, y], [x + width, y], [x + width, y + height], [x, y + height]] as const;
  const world = (a: number, b: number, c: number) => vec3.transformMat4(vec3.create(), sweep === "w" ? [a, b, c] : [a, c, b], frame);
  const points = [vectors.span.z0, vectors.span.z1].flatMap((c) => corners.map(([a, b]) => world(a, b, c)));
  const box = (xOf: (p: vec3) => number, yOf: (p: vec3) => number): Extent => ({ minX: Math.min(...points.map(xOf)), maxX: Math.max(...points.map(xOf)), minY: Math.min(...points.map(yOf)), maxY: Math.max(...points.map(yOf)) });
  return { front: box((p) => p[2]!, (p) => p[1]!), plan: box((p) => p[0]!, (p) => p[2]!) };
}

const expectExtent = (label: string, actual: Extent, expected: Extent): void => {
  for (const edge of ["minX", "maxX", "minY", "maxY"] as const) expect(Math.abs(actual[edge] - expected[edge]), `${label} ${edge}`).toBeLessThanOrEqual(vectors.toleranceM);
};

describe("🧊️ the sweep axis of an extruded solid", () => {
  for (const c of vectors.cases)
    it(`places the profile as the vectors say: ${c.name}`, () => {
      const found = measured(sweptAsset(c.sweep));
      expectExtent("front", found.front, c.front);
      expectExtent("plan", found.plan, c.plan);
    });

  for (const c of vectors.cases)
    it(`equals the gl-matrix recomputation: ${c.name}`, () => {
      const reference = glMatrixExtent(c.sweep);
      const found = measured(sweptAsset(c.sweep));
      expectExtent("front", found.front, reference.front);
      expectExtent("plan", found.plan, reference.plan);
    });

  it("draws w and v differently, so a part is never transposed silently", () => {
    const [w, v] = [measured(sweptAsset("w")), measured(sweptAsset("v"))];
    expect(Math.abs(w.front.maxX - v.front.maxX)).toBeGreaterThan(0.1);
    expect(Math.abs(w.plan.maxX - v.plan.maxX)).toBeGreaterThan(0.1);
  });
});

describe("🧊️ a missing or unknown sweep is refused by name", () => {
  for (const c of vectors.refused) {
    const asset = withSlat(c.solid);

    it(`${c.name}: evaluate throws SolidSweepError naming the asset, the shape and the viewpoint in both languages`, () => {
      const run = (): unknown => evaluate(asset, { kind: "pose", pose: vectors.pose }, { kind: "scale", scale: { denominator: vectors.scale } });
      expect(run).toThrow(SolidSweepError);
      try {
        run();
      } catch (error) {
        const refusal = error as SolidSweepError;
        expect([refusal.asset, refusal.shape, refusal.viewpoint]).toEqual([sample.id, "slat", "front"]);
        expect(refusal.reason.en).toContain("slat");
        expect(refusal.reason.de).toContain("slat");
      }
    });

    it(`${c.name}: the multiview resolver does not turn it into an unavailable view`, () => {
      expect(() => resolveViews(asset, { kind: "pose", pose: vectors.pose }, { kind: "scale", denominator: vectors.scale }, "light-line", drawOptions("light-line", vectors.scale) as DrawOptions)).toThrow(SolidSweepError);
    });

    it(`${c.name}: the semantic validator reports solid-sweep at the shape`, () => {
      const issues = validateAsset(asset).filter((issue) => issue.code === "solid-sweep");
      expect(issues.length).toBeGreaterThan(0);
      for (const issue of issues) {
        expect(issue.path).toContain(".solid.sweep");
        expect(issue.message.en.length).toBeGreaterThan(0);
        expect(issue.message.de.length).toBeGreaterThan(0);
      }
    });

    it(`${c.name}: ajv rejects the document`, () => {
      const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/🔣️.json", import.meta.url), "utf8"));
      const ajv = new Ajv({ strict: true, allErrors: true });
      ajv.addSchema(schema);
      expect(ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` })(asset)).toBe(false);
    });
  }

  it("accepts a sweep of w and of v in the validator", () => {
    for (const sweep of ["w", "v"] as const) expect(validateAsset(sweptAsset(sweep)).filter((issue) => issue.code === "solid-sweep")).toEqual([]);
  });
});
