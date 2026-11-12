/** 🪧️ The glyph solids of the analytic views: arrows for loads and reactions, arcs for moments, a sphere for the centre of mass and cones for emitters.
 *
 * Glyphs are procedural solids on their own nodes (no parent: they live in the world, not on a bone), tagged `glyph`, shown by the variant of their view and hidden by every other (their base material is fully transparent). Every size is a paper size of the 2D view multiplied by the metres per paper millimetre of the requested scale (`denominator / 1000`),
 * so a glyph is as large in the model at 1:50 as it is on paper at 1:50. An arrow lies along local `+Y` with its tip at the node origin (the application point, the head on the point as in the 2D view) and has three nodes: the root (position, direction, visibility), the head (uniform scale, shortening on short arrows) and the shaft (length);
 * an arc is a 270 degree torus sector about local `+Y` in the right-handed sense with a cone as arrow head, the node scale its radius; a cone lies along `+Y` from the origin. A glyph that is absent at an instant is scaled to a hair, never removed, so animated clips keep a fixed node set.
 *
 * @see ../../🎨️depiction/🏗️structural-view/🟦️.ts — the paper sizes (`longestArrowMm`, `arrowLengthMm`, `momentRadiusMm`)
 */
import type { Slug } from "../../../🧬️schema/🟦️.ts";
import type { ModelDocument, Node, Primitive, Solid, TransformPatch } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { type Triple, basisAlongY, quaternionOf } from "../🧭️frame/🟦️.ts";
import * as portable from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 🔬️ A glyph that is not there at an instant. */
export const HIDDEN: Triple = [1e-5, 1e-5, 1e-5];

/** 🏹️ The paper sizes of the arrow in millimetres (the 2D arrow outline: shaft width 0.3, head width 1.3, head length at most 1.8). */
export const ARROW = { shaftMm: 0.3, headWidthMm: 1.3, headLengthMm: 1.8 } as const;

/** ⭕️ The proportions of a moment arc of radius 1: tube radius and arrow head. */
export const ARC = { tube: 0.06, headLength: 0.35, headRadius: 0.16, sweepRad: 1.5 * Math.PI } as const;

const identity: TransformPatch = { translation: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1] };

/** 🏷️ The slug of an id such as `force:applied:load#1`. */
export const slugOf = (id: string): Slug => id.toLowerCase().replace(/[^a-z0-9]+/gu, "-").replace(/^-+|-+$/gu, "");

const glyphNode = (id: Slug, parent: Slug | null, patch: TransformPatch = identity): Node => ({ id, parent, translation: patch.translation ?? [0, 0, 0], rotation: patch.rotation ?? [0, 0, 0, 1], scale: patch.scale ?? [1, 1, 1], tags: ["glyph"] });

const cone = (id: Slug, node: Slug, material: Slug, length: number, radius: number, toward: 1 | -1): Solid => ({
  id,
  node,
  material,
  tags: ["glyph"],
  kind: "revolve",
  profile: toward > 0 ? [[0, 0], [0, radius], [length, 0]] : [[-length, 0], [-length, radius], [0, 0]],
  axis: [0, 1, 0],
  origin: [0, 0, 0],
});

/** 🔗️ The material of the feature edges of the glyph solids that have the base material `material`: every variant maps it exactly as it maps the glyph surface (the edge ink in the view the glyph belongs to, hidden in every other). */
export const glyphEdgeOf = (material: Slug): Slug => `${material}-edge`;

/** 🌟️ The edge material of the feature-edge pass for the primitives of a procedural document: the shared `edge` material, except for the primitives of glyph solids, whose edges follow their own glyph material ({@link glyphEdgeOf}). */
export function edgeMaterialOf(document: ModelDocument): (source: Primitive) => Slug {
  const glyphs = new Set(document.solids.filter((solid) => solid.tags?.includes("glyph") === true).map((solid) => solid.material));
  return (source) => (glyphs.has(source.material) ? glyphEdgeOf(source.material) : "edge");
}

/** 📏️ The nodes and solids of one arrow glyph `id` with the base material `material`; `mm` is the metres per paper millimetre. */
export function arrowGlyph(id: Slug, material: Slug, mm: number, at: TransformPatch = identity): { readonly nodes: Node[]; readonly solids: Solid[] } {
  const root = `arrow-${id}`;
  const headLength = ARROW.headLengthMm * mm;
  return {
    nodes: [glyphNode(root, null, at), glyphNode(`${root}-head`, root), glyphNode(`${root}-shaft`, root, { translation: [0, -headLength, 0] })],
    solids: [
      cone(`${root}-head`, `${root}-head`, material, headLength, (ARROW.headWidthMm / 2) * mm, -1),
      { id: `${root}-shaft`, node: `${root}-shaft`, material, tags: ["glyph"], kind: "revolve", profile: [[-1, 0], [-1, (ARROW.shaftMm / 2) * mm], [0, (ARROW.shaftMm / 2) * mm], [0, 0]], axis: [0, 1, 0], origin: [0, 0, 0] },
    ],
  };
}

/** 🧮️ The patches of the three nodes of arrow `id` for an arrow with its tip at `at` (neutral frame) pointing along `direction` with paper length `lengthMm`; `null` hides it. */
export function arrowPatch(id: Slug, at: Triple | null, direction: Triple, lengthMm: number, mm: number): Record<Slug, TransformPatch> {
  const root = `arrow-${id}`;
  if (at === null || !(lengthMm > 0)) return { [root]: { scale: HIDDEN }, [`${root}-head`]: { scale: [1, 1, 1] }, [`${root}-shaft`]: { translation: [0, -ARROW.headLengthMm * mm, 0], scale: [1, 1e-5, 1] } };
  const length = lengthMm * mm;
  const head = Math.min(ARROW.headLengthMm * mm, 0.6 * length);
  const n = portable.hypot(...direction) || 1;
  return {
    [root]: { translation: at, rotation: quaternionOf(basisAlongY([direction[0] / n, direction[1] / n, direction[2] / n])), scale: [1, 1, 1] },
    [`${root}-head`]: { scale: [head / (ARROW.headLengthMm * mm), head / (ARROW.headLengthMm * mm), head / (ARROW.headLengthMm * mm)] },
    [`${root}-shaft`]: { translation: [0, -head, 0], scale: [1, Math.max(length - head, 1e-5), 1] },
  };
}

/** 🧊️ The node and solids of one moment arc glyph `id` with the base material `material`. */
export function arcGlyph(id: Slug, material: Slug, at: TransformPatch = identity): { readonly nodes: Node[]; readonly solids: Solid[] } {
  const root = `arc-${id}`;
  const ring: [number, number][] = Array.from({ length: 13 }, (_, i) => [ARC.tube * portable.cos((2 * Math.PI * i) / 12), 1 + ARC.tube * portable.sin((2 * Math.PI * i) / 12)]);
  const end: Triple = [portable.cos(ARC.sweepRad), 0, -portable.sin(ARC.sweepRad)];
  const rotation = quaternionOf([[0, -1, 0], [1, 0, 0], [0, 0, 1]]);
  return {
    nodes: [glyphNode(root, null, at)],
    solids: [
      { id: `${root}-arc`, node: root, material, tags: ["glyph"], kind: "revolve", profile: ring, axis: [0, 1, 0], origin: [0, 0, 0], sweepRad: ARC.sweepRad },
      { ...cone(`${root}-head`, root, material, ARC.headLength, ARC.headRadius, 1), transform: { translation: end, rotation } },
    ],
  };
}

/** 🧪️ The patch of arc `id` centred at `at` with radius `radiusM`, its axis along `axis` (the neutral pseudo-vector of the moment); `null` hides it. */
export function arcPatch(id: Slug, at: Triple | null, axis: Triple, radiusM: number): Record<Slug, TransformPatch> {
  const root = `arc-${id}`;
  if (at === null || !(radiusM > 0)) return { [root]: { scale: HIDDEN } };
  const n = portable.hypot(...axis) || 1;
  return { [root]: { translation: at, rotation: quaternionOf(basisAlongY([axis[0] / n, axis[1] / n, axis[2] / n])), scale: [radiusM, radiusM, radiusM] } };
}

/** ⚖️ The node and solid of the centre-of-mass sphere of paper radius 1.3 mm. */
export function massGlyph(material: Slug, mm: number, at: TransformPatch = identity): { readonly nodes: Node[]; readonly solids: Solid[] } {
  return { nodes: [glyphNode("centre-of-mass", null, at)], solids: [{ id: "centre-of-mass", node: "centre-of-mass", material, tags: ["glyph"], kind: "ellipsoid", radii: [1.3 * mm, 1.3 * mm, 1.3 * mm] }] };
}

/** 🔥️ The node and cone of one emitter marker glyph `id`: paper length 5 mm along the outflow, base radius 0.2 of the length. */
export function emitterGlyph(id: Slug, material: Slug, mm: number, at: TransformPatch = identity): { readonly nodes: Node[]; readonly solids: Solid[] } {
  const root = `emitter-${id}`;
  return { nodes: [glyphNode(root, null, at)], solids: [cone(root, root, material, 5 * mm, 1 * mm, 1)] };
}

/** 📤️ The patch of emitter marker `id` at `at` along `direction`; `null` (not flowing) hides it. */
export function emitterPatch(id: Slug, at: Triple | null, direction: Triple): Record<Slug, TransformPatch> {
  const root = `emitter-${id}`;
  if (at === null) return { [root]: { scale: HIDDEN } };
  const n = portable.hypot(...direction) || 1;
  return { [root]: { translation: at, rotation: quaternionOf(basisAlongY([direction[0] / n, direction[1] / n, direction[2] / n])), scale: [1, 1, 1] } };
}
