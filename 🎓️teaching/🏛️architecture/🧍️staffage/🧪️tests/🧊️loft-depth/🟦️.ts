/** 🌿️ The depth of the flat front and plan shapes of a bone whose side elevation is a `loft`: the loft counts as the depth twin of its bone, as an authored flat outline of the same silhouette does.
 *
 * THE REFERENCE is the engine's own path for a flat elevation outline (the twin of every flat front or plan shape that has none of its own id), fed with the rest outline of the very loft in the bone frame; the vectors of the lofts are those of `../../🧫️fixtures/🧊️loft-shape`, whose three views are held against gl-matrix, shapely and the kernel by `../🧊️loft-shape`.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { describe, expect, it } from "vitest";
import type { Asset, AuthoredRepresentation, LoftSection, Shape } from "../../🧬️schema/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { apply, invert, solveRig } from "../../🔨️modules/🦴️rig/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";

type Vectors = { scale: number; bone: string; lofts: Record<string, { sections: LoftSection[] }> };
type Rings = readonly (readonly { readonly x: number; readonly y: number }[])[];

const vectors = fixture<Vectors>("🧊️loft-shape");
const sample = fixture<{ asset: Asset }>("👤️sample-figure").asset;
const key = String(vectors.scale) as keyof Asset["representations"];
const authored = sample.representations[key] as AuthoredRepresentation;
const selector = { kind: "scale", scale: { denominator: vectors.scale } } as const;
/** 🦴️ The bone that carries the stroke of each view: a flat plan stroke on a bone along the viewing axis grows from the origin to the tip instead of taking a depth, so the plan reads the foot. */
const BONES = { front: vectors.bone, plan: "foot-l" } as const;
const VIEWS = ["front", "plan"] as const;

const loftOn = (bone: string): Shape => ({ id: `limb-${bone}`, kind: "loft", bone, role: "body", z: 1, filled: true, tone: 0.5, sections: vectors.lofts.limb!.sections.map((section) => ({ ...section, at: [section.at[0], section.at[1] + 0.2, section.at[2]] })) }) as unknown as Shape;
const markOn = (bone: string): Shape => ({ id: `mark-${bone}`, kind: "poly", bone, role: "body", z: 2, filled: true, closed: true, tone: 0.3, points: [[-0.04, 0.02], [0.04, 0.02], [0.04, 0.4], [-0.04, 0.4]] }) as unknown as Shape;
const lofts = VIEWS.map((view) => loftOn(BONES[view]));
const marks = VIEWS.map((view) => markOn(BONES[view]));

/** 🧬️ The asset with these shapes: every asset gets its own copies, because the engine caches the depth of a flat shape by the identity of the shape. */
const withViews = (elevation: readonly Shape[], flat: readonly Shape[]): Asset => ({ ...sample, poses: [...sample.poses, { ...sample.poses[0]!, id: "rest-zero", ground: "authored", contacts: [] }], representations: { ...sample.representations, [key]: { ...authored, geometry: { ...authored.geometry, elevation: structuredClone([...elevation]), front: structuredClone([...flat]), plan: structuredClone([...flat]) } } } as Asset["representations"] });

const stroke = (asset: Asset, pose: string, view: (typeof VIEWS)[number]): Rings => evaluate(asset, { kind: "pose", pose }, selector).shapes[view].find((s) => s.shape.id === `mark-${BONES[view]}`)!.rings;
const worst = (a: Rings, b: Rings): number => Math.max(...a.flat().map((p, i) => Math.hypot(p.x - b.flat()[i]!.x, p.y - b.flat()[i]!.y)));

const asLoft = withViews(lofts, [...lofts, ...marks]);
const restState = evaluate(asLoft, { kind: "pose", pose: "rest-zero" }, selector);
const rests = solveRig(asLoft.rig, {}, { x: 0, y: 0, angleDeg: 0 });
const flatOutlines = VIEWS.map((view) => {
  const bone = BONES[view];
  const outline = restState.shapes.elevation.find((s) => s.shape.id === `limb-${bone}`)!.rings;
  expect(outline, `${bone} rest outline`).toHaveLength(1);
  return { id: `outline-${bone}`, kind: "poly", bone, role: "body", z: 1, filled: true, closed: true, tone: 0.5, points: outline[0]!.map((p) => { const local = apply(invert(rests[bone]!), p.x, p.y); return [local.x, local.y]; }) } as unknown as Shape;
});
const asFlat = withViews(flatOutlines, [...lofts, ...marks]);
const without = withViews([], [...lofts, ...marks]);

describe("🌿️ the depth twin of a bone whose elevation is a loft", () => {
  it("measures the rest outline of the loft in the frame that the rest pose has", () => {
    for (const bone of Object.values(BONES)) for (let i = 0; i < 6; i++) expect(Math.abs(restState.bones[bone]!.world[i]! - rests[bone]![i]!), `${bone} matrix entry ${i}`).toBeLessThanOrEqual(1e-12);
  });

  it.each(VIEWS)("carries a flat %s stroke without a twin of its own as the flat outline of the same silhouette does, in every pose", (view) => {
    for (const pose of asLoft.poses.map((p) => p.id)) expect(worst(stroke(asLoft, pose, view), stroke(asFlat, pose, view)), `${view} ${pose}`).toBeLessThanOrEqual(1e-9);
  });

  it.each(VIEWS)("moves the %s stroke away from where the bone origin carries it when the bone turns, so the twin is read and not skipped", (view) => {
    const moved = Math.max(...asLoft.poses.map((p) => worst(stroke(asLoft, p.id, view), stroke(without, p.id, view))));
    expect(moved).toBeGreaterThan(1e-3);
  });
});
