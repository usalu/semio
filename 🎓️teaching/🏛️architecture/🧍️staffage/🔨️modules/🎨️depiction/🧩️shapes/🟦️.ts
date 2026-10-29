/** 🧩️ The shared shape pipeline of all seven views: level-of-detail selection, simplification, z-ordering, fills and exact hidden-line outlines.
 *
 * Every view paints the same prepared rings: filled shapes become one `fill` item each (identical commands in every view, only the style
 * differs), outlines become role-weighted runs with everything behind a higher-z filled shape removed.
 *
 * @see ../../📐️units/🟦️.ts — line weights and level of detail
 * @see ../../🔗️contracts/🟦️.ts — {@link EvaluatedShape}
 */
import type { Shape, ShapeRole, Slug, Viewpoint } from "../../../🧬️schema/🟦️.ts";
import type { DrawingScale, EvaluatedState, Fill, ItemValue, LayerId, Legend } from "../../🔗️contracts/🟦️.ts";
import { lineWeightMm } from "../../📐️units/🟦️.ts";
import type { Canvas, Ctx } from "../🧱️canvas/🟦️.ts";
import { type Box, type Occluder, type Run, type Vec, boxOf, commandsOf, ringsCommands, simplify, visibleRuns } from "../📐️geometry/🟦️.ts";

const ROLE_LAYER: Readonly<Record<ShapeRole, LayerId>> = { body: "body", detail: "detail", texture: "texture", glazing: "glazing", ground: "ground" };
const OCCLUDING_ROLES: readonly ShapeRole[] = ["body", "detail", "texture"];
const CLIP_EPS_MM = 1e-4;
const SIMPLIFY_MM = 0.015;
const MIN_DETAIL_MM = 0.4;

/** 🧩️ One shape ready to draw: simplified paper rings in z order with its occluder. */
export type Prepared = { readonly shape: Shape; readonly rings: readonly (readonly Vec[])[]; readonly closed: boolean; readonly box: Box; readonly order: number; readonly occluder: Occluder | null; readonly weightMm: number; readonly parent: Slug | null };

/** 🎨️ How a view paints one shape: a fill (or none), the value it carries, and the width of the thin outline on the fill item (or none). */
export type Style = { readonly fill: Fill | null; readonly value?: ItemValue; readonly outlineMm: number | null };

/** 🔬️ An analytic view: how it paints shapes, what it adds on top, and which legends explain it. */
export type AnalyticView = { readonly styleOf: (p: Prepared) => Style; readonly overlay: (canvas: Canvas) => void; readonly inset?: (canvas: Canvas) => number; readonly legends: readonly Legend[] };

/** 🔭️ Selects the shapes drawn at the scale and level of detail, simplifies them in paper millimetres and sorts them by z. */
export function prepare(state: EvaluatedState, viewpoint: Viewpoint, scale: DrawingScale, lod: number, k: number): Prepared[] {
  const selected = state.shapes[viewpoint].flatMap((e, index) => {
    const weight = lineWeightMm(scale, e.shape.role);
    if (e.shape.lod > lod || weight === null) return [];
    const rings = e.rings.map((r) => simplify(r.map((p) => ({ x: p.x * k, y: -p.y * k })), e.closed, SIMPLIFY_MM));
    const box = boxOf(rings);
    if (box === null) return [];
    if (lod <= 1 && e.shape.role === "detail" && Math.max(box.maxX - box.minX, box.maxY - box.minY) < MIN_DETAIL_MM) return [];
    return [{ e, index, rings, box, weight }];
  });
  const parents = new Map(state.asset.rig.bones.map((b) => [b.id, b.parent]));
  selected.sort((a, b) => a.e.shape.z - b.e.shape.z || a.index - b.index);
  return selected.map(({ e, rings, box, weight }, order) => ({
    shape: e.shape,
    rings,
    closed: e.closed,
    box,
    order,
    weightMm: weight,
    parent: parents.get(e.shape.bone) ?? null,
    occluder: e.closed && e.shape.filled && OCCLUDING_ROLES.includes(e.shape.role) ? { rings, box } : null,
  }));
}

/** 🖌️ Paints fills (one item per filled closed shape) and, where requested, role-weighted outlines with hidden lines removed. */
export function drawShapes(canvas: Canvas, ctx: Ctx, shapes: readonly Prepared[], styleOf: (p: Prepared) => Style, outlineAll: boolean): void {
  const ink = ctx.colours.ink;
  for (const p of shapes) {
    const filled = p.closed && p.shape.filled;
    if (!filled) continue;
    const style = styleOf(p);
    if (style.fill === null) continue;
    canvas.path("fill", `shape:${p.shape.id}`, ringsCommands(p.rings, true), {
      fill: style.fill,
      ...(style.outlineMm === null ? {} : { stroke: { color: ink, widthMm: style.outlineMm, cap: "round", join: "round" } }),
      ...(style.value ? { value: style.value } : {}),
      bone: p.shape.bone,
      ...(p.shape.region ? { region: p.shape.region } : {}),
    });
  }
  for (const { p, run, n } of outlineRuns(shapes, outlineAll))
    canvas.path(ROLE_LAYER[p.shape.role], `outline:${p.shape.id}:${n}`, commandsOf(run.points, run.closed), {
      stroke: { color: ink, widthMm: p.weightMm, cap: "round", join: "round" },
      bone: p.shape.bone,
      ...(p.shape.region ? { region: p.shape.region } : {}),
    });
}

const joined = (a: Prepared, b: Prepared): boolean => a.shape.region !== undefined && a.shape.region === b.shape.region && (a.shape.bone === b.shape.bone || a.parent === b.shape.bone || b.parent === a.shape.bone);

/** ✂️ The visible outline runs of every shape (filled closed shapes only when `includeFilled`): hidden lines behind higher-z filled shapes are removed, and filled shapes of one region on the same or adjacent bones hide each other's inner outlines so they read as one silhouette. */
export function* outlineRuns(shapes: readonly Prepared[], includeFilled: boolean): Generator<{ readonly p: Prepared; readonly run: Run; readonly n: number }> {
  for (const p of shapes) {
    if (p.closed && p.shape.filled && !includeFilled) continue;
    const hiders = shapes.flatMap((q) => (q.occluder !== null && q !== p && (q.order > p.order || joined(p, q)) ? [q.occluder] : []));
    let n = 0;
    for (const ring of p.rings) for (const run of visibleRuns(ring, p.closed, hiders, CLIP_EPS_MM)) yield { p, run, n: n++ };
  }
}
