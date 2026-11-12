/** 🧷️ The data of one evaluated instant that every 3D output of the volume shares: the local placement of every bone, the placements of the glyph nodes (force arrows, moment arcs, centre of mass, emitter cones) and the paint of every look and glyph.
 *
 * One source of truth: `modelOf` (the neutral model, whose poses and animation channels are built from these frames) and the axonometric depiction (a drawing of the same instant) both call {@link frameDataOf}; nothing about glyph placement, sizes or paints is computed twice.
 * Bone placements come from the evaluated bone frames, analytic values from the solved thermal field, structural result and emitters, colours from the shared looks.
 *
 * @see ../🟦️.ts — `modelOf`
 * @see ../../🎨️depiction/🧊️axonometric/🟦️.ts — the axonometric depiction
 */
import type { Asset, Slug } from "../../../🧬️schema/🟦️.ts";
import type { Node, Solid, TransformPatch } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { EmitterState, EvaluatedState, StructuralResult, ThermalField, View } from "../../🔗️contracts/🟦️.ts";
import { type Theme, THEMES } from "../../🎨️depiction/🖍️palette/🟦️.ts";
import { arrowLengthMm, forceList, momentList, MOMENT_MIN_SHOWN, momentRadiusMm } from "../../🎨️depiction/🏗️structural-view/🟦️.ts";
import { magnitude } from "../../🏗️structure/📽️projection/🟦️.ts";
import type { LookSpec, Paint } from "../🎨️looks/🟦️.ts";
import { arcPaint, arrowPaint, massPaint, type ArcKind, type ArrowKind } from "../🏗️structure/🟦️.ts";
import { toNeutral, pseudoToNeutral } from "../🧭️frame/🟦️.ts";
import { HIDDEN, arcGlyph, arcPatch, arrowGlyph, arrowPatch, emitterGlyph, emitterPatch, massGlyph, slugOf } from "../🪧️glyphs/🟦️.ts";
import { placementsOf, type Placement } from "../🦴️skeleton/🟦️.ts";
import { emitterPaint } from "../🌡️thermal/🟦️.ts";

/** 🧮️ The analyses of one instant the analytic looks read. */
export type Analytic = { thermal?: ThermalField; structure?: StructuralResult; emitters?: readonly EmitterState[] };

/** 🦄 Everything one instant contributes: the bone placements, the paint of every analytic look by material key, the glyph node patches and the glyph paints by base material. */
export type FrameData = {
  readonly locals: Record<Slug, Placement>;
  readonly paints: Map<View, ReadonlyMap<string, Paint>>;
  readonly glyphs: Map<Slug, TransformPatch>;
  readonly glyphPaints: Map<Slug, Paint>;
};

/** 🪧️ A glyph slot: the kind of glyph that lives at a root node, its base material and a short detail (the load kind, the emitter id). */
export type Slot = { readonly kind: "arrow" | "arc" | "mass" | "emitter"; readonly material: Slug; readonly detail: string };

/** 🐙 The glyph slots met so far, in order of first use; the glyph nodes of a model are created in this order and absent glyphs are hidden by it. */
export class Slots {
  readonly map = new Map<Slug, Slot>();
  readonly order: Slug[] = [];

  /** ➕️ Registers the slot of a root node the first time it is met. */
  add(root: Slug, slot: Slot): void {
    if (!this.map.has(root)) {
      this.map.set(root, slot);
      this.order.push(root);
    }
  }
}

/** 🎭️ The looks a frame is painted with: the analytic ones when the view is wanted, and the four static ones. */
export type FrameLooks = {
  readonly thermal: LookSpec<ThermalField> | undefined;
  readonly forces: LookSpec<StructuralResult> | undefined;
  readonly moments: LookSpec<StructuralResult> | undefined;
  readonly fixed: Readonly<Partial<Record<"dark-solid" | "light-solid" | "dark-line" | "light-line", LookSpec>>>;
};

/** ⚙️ What {@link frameDataOf} needs besides the instant: the asset, the wanted views, the theme of the analytic views, the metres per paper millimetre `mm` of the scale, the paper length of the longest force arrow, the looks and the slot registry. */
export type FrameSetup = { readonly asset: Asset; readonly views: readonly View[]; readonly theme: Theme; readonly mm: number; readonly longestMm: number; readonly looks: FrameLooks; readonly slots: Slots };

/** 🦋 The {@link FrameData} of an evaluated instant and its analyses. */
export function frameDataOf(setup: FrameSetup, state: EvaluatedState, analytic: Analytic): FrameData {
  const { asset, views, theme, mm, longestMm, looks, slots } = setup;
  const wantsForces = views.includes("forces");
  const wantsMoments = views.includes("moments");
  const locals = placementsOf(asset, Object.fromEntries(Object.entries(state.bones).map(([id, b]) => [id, b.frame])));
  const paints = new Map<View, ReadonlyMap<string, Paint>>();
  const glyphs = new Map<Slug, TransformPatch>();
  const glyphPaints = new Map<Slug, Paint>();
  const put = (patch: Record<Slug, TransformPatch>): void => void Object.entries(patch).forEach(([id, p]) => glyphs.set(id, p));
  if (looks.thermal && analytic.thermal) {
    paints.set("thermal", looks.thermal.paint(analytic.thermal));
    for (const emitter of asset.thermal.emitters) {
      const root = `emitter-${slugOf(emitter.id)}`;
      slots.add(root, { kind: "emitter", material: `glyph-emitter-${slugOf(emitter.id)}`, detail: emitter.id });
      const e = analytic.emitters?.find((s) => s.emitter === emitter.id);
      put(emitterPatch(slugOf(emitter.id), e !== undefined && e.active ? toNeutral([e.at.x, e.at.y, e.at.z]) : null, e === undefined ? [0, 1, 0] : toNeutral([e.direction.x, e.direction.y, e.direction.z])));
      if (e !== undefined) glyphPaints.set(`glyph-emitter-${slugOf(emitter.id)}`, emitterPaint(e, analytic.thermal.environment.airTempC, theme));
    }
  }
  const result = analytic.structure;
  if (result !== undefined) {
    if (looks.forces) paints.set("forces", looks.forces.paint(result));
    if (looks.moments) paints.set("moments", looks.moments.paint(result));
    if (wantsForces) {
      const items = forceList(state, result).filter((i) => i.drawn);
      const all = forceList(state, result);
      const max = Math.max(0, ...all.map((i) => magnitude(i.force)));
      for (const item of items) {
        const id = slugOf(item.id);
        slots.add(`arrow-${id}`, { kind: "arrow", material: `glyph-arrow-${item.kind}`, detail: item.kind });
        const f = toNeutral([item.force.x, item.force.y, item.force.z]);
        const mag = magnitude(item.force);
        put(arrowPatch(id, mag < 1e-9 ? null : toNeutral([item.at.x, item.at.y, item.at.z]), f, mag < 1e-9 ? 0 : arrowLengthMm(mag, max, longestMm), mm));
      }
      slots.add("centre-of-mass", { kind: "mass", material: "glyph-mass", detail: "mass" });
      put({ "centre-of-mass": { translation: toNeutral([result.centreOfMass.x, result.centreOfMass.y, result.centreOfMass.z]), scale: [1, 1, 1] } });
    }
    if (wantsMoments) {
      const arcs = momentList(state, result);
      const max = Math.max(0, ...arcs.map((a) => magnitude(a.m)));
      for (const arc of arcs) {
        const id = slugOf(arc.id);
        slots.add(`arc-${id}`, { kind: "arc", material: `glyph-arc-${arc.kind}`, detail: arc.kind });
        const total = magnitude(arc.m);
        const shown = total >= Math.max(1e-9, max * MOMENT_MIN_SHOWN);
        put(arcPatch(id, shown ? toNeutral([arc.at.x, arc.at.y, arc.at.z]) : null, pseudoToNeutral([arc.m.x, arc.m.y, arc.m.z]), shown ? momentRadiusMm(total, max) * mm : 0));
      }
    }
  }
  for (const view of ["dark-solid", "light-solid", "dark-line", "light-line"] as const) if (views.includes(view)) paints.set(view, looks.fixed[view]!.paint(undefined));
  for (const root of slots.order) {
    if (glyphs.has(root)) continue;
    const s = slots.map.get(root)!;
    const id = root.slice(root.indexOf("-") + 1);
    if (s.kind === "arrow") put(arrowPatch(id, null, [0, 1, 0], 0, mm));
    else if (s.kind === "arc") put(arcPatch(id, null, [0, 1, 0], 0));
    else if (s.kind === "emitter") put(emitterPatch(id, null, [0, 1, 0]));
  }
  return { locals, paints, glyphs, glyphPaints };
}

/** 🐝 The nodes and solids of the glyph of slot `root` placed by the patches of one instant (`glyphs` of its {@link FrameData}; a glyph without a patch is hidden), with the sizes of the scale `mm` (metres per paper millimetre). */
export function glyphOf(root: Slug, slot: Slot, mm: number, glyphs: ReadonlyMap<Slug, TransformPatch>): { readonly nodes: Node[]; readonly solids: Solid[] } {
  const id = root.slice(root.indexOf("-") + 1);
  const at: TransformPatch = glyphs.get(root) ?? { scale: HIDDEN };
  const made = slot.kind === "arrow" ? arrowGlyph(id, slot.material, mm, at) : slot.kind === "arc" ? arcGlyph(id, slot.material, at) : slot.kind === "mass" ? massGlyph(slot.material, mm, at) : emitterGlyph(id.replace(/^emitter-/u, ""), slot.material, mm, at);
  const nodes = made.nodes.map((node) => {
    const sub = glyphs.get(node.id);
    return sub === undefined || node.id === root ? node : { ...node, translation: sub.translation ?? node.translation, rotation: sub.rotation ?? node.rotation, scale: sub.scale ?? node.scale };
  });
  return { nodes, solids: made.solids };
}

/** 🎨️ The paint of the glyph of a slot: the arrow paint of its load kind, the arc paint of its moment kind, the ink of the centre of mass, or the temperature paint of an emitter at this instant (`glyphPaints` of its {@link FrameData}, else a 55 % opaque ink). */
export function glyphPaintOf(slot: Slot, theme: Theme, glyphPaints: ReadonlyMap<Slug, Paint>): Paint {
  return slot.kind === "arrow" ? arrowPaint(theme, slot.detail as ArrowKind) : slot.kind === "arc" ? arcPaint(theme, slot.detail as ArcKind) : slot.kind === "mass" ? massPaint(theme) : (glyphPaints.get(slot.material) ?? { hex: THEMES[theme].ink, alpha: 0.55 });
}
