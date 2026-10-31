/** 🧩️ The shared shape pipeline of all seven views: representation pens, z-ordering, fills, hatches and exact hidden-line outlines.
 *
 * Every view paints the same prepared rings (identical `fill` item commands per shape id, only the style differs). Which shapes exist, with which pens and hatches, is decided by the individually
 * authored representation the state was evaluated with: there is no level-of-detail filter, no global pen table and no size cull. A shape whose role pen is `null` and that has no `penMm`
 * is not drawn. Outlines become role-pen runs with everything behind a higher-z filled shape removed; hatches are clipped to their shape and hidden like outlines.
 *
 * @see ../../../🧬️schema/🟦️.ts — `AuthoredRepresentation`, `Hatch`, `PenSet`
 * @see ../../🔗️contracts/🟦️.ts — {@link EvaluatedShape}
 */
import type { Asset, Hatch, PenSet, Shape, ShapeRole, Slug, Viewpoint } from "../../../🧬️schema/🟦️.ts";
import type { EvaluatedState, Fill, ItemValue, LayerId, Legend } from "../../🔗️contracts/🟦️.ts";
import type { Canvas, Ctx } from "../🧱️canvas/🟦️.ts";
import { type Box, type Occluder, type Ring, type Run, type Vec, boxOf, circleCommands, clipSegmentToRings, commandsOf, pointInside, ringsCommands, simplify, visibleRuns } from "../📐️geometry/🟦️.ts";

const ROLE_LAYER: Readonly<Record<ShapeRole, LayerId>> = { body: "body", detail: "detail", texture: "texture", glazing: "glazing", ground: "ground" };
const OCCLUDING_ROLES: readonly ShapeRole[] = ["body", "detail", "texture"];
const CLIP_EPS_MM = 1e-4;
const SIMPLIFY_MM = 0.01;
const HATCH_LIMIT = 6000;
const ANALYTIC_VIEWS: readonly string[] = ["thermal", "forces", "moments"];

/** 🖨️ What a representation contributes to drawing: its pens, hatches and whether its geometry is in paper millimetres (symbols other than `true-extent`). */
export type Source = { readonly pens: PenSet; readonly hatches: readonly Hatch[]; readonly paperUnits: boolean };

/** 🧩️ One shape ready to draw: simplified paper rings in z order with its occluder, pen and hatch. */
export type Prepared = { readonly shape: Shape; readonly rings: readonly (readonly Vec[])[]; readonly closed: boolean; readonly box: Box; readonly order: number; readonly occluder: Occluder | null; readonly weightMm: number; readonly hatch: Hatch | null; readonly parent: Slug | null };

/** 🎨️ How a view paints one shape: a fill (or none), the value it carries, and the width of the thin outline on the fill item (or none). */
export type Style = { readonly fill: Fill | null; readonly value?: ItemValue; readonly outlineMm: number | null };

/** 🔬️ An analytic view: how it paints shapes, what it adds on top, and which legends explain it. */
export type AnalyticView = { readonly styleOf: (p: Prepared) => Style; readonly overlay: (canvas: Canvas) => void; readonly inset?: (canvas: Canvas) => number; readonly legends: readonly Legend[] };

/** 🖨️ The pens, hatches and units of the representation a state was evaluated with; throws for a state evaluated without one. */
export function sourceOf(state: EvaluatedState): Source {
  const representation = state.representation;
  if (representation.kind === "scale") {
    const rep = state.asset.representations[`${representation.denominator}`];
    if (rep?.status !== "authored") throw new Error(`asset "${state.asset.id}" has no authored representation 1:${representation.denominator}`);
    return { pens: rep.pens, hatches: rep.hatches, paperUnits: false };
  }
  if (representation.kind === "symbol") {
    const symbol = state.asset.symbols.find((candidate) => candidate.id === representation.symbol);
    if (symbol === undefined) throw new Error(`asset "${state.asset.id}" has no symbol "${representation.symbol}"`);
    return { pens: symbol.pens, hatches: symbol.hatches, paperUnits: symbol.units === "paper-mm" };
  }
  throw new Error("the state was evaluated without a representation selector; evaluate(asset, query, selector) first");
}

/** 🔗️ The thermal regions a shape paints: its single `region`, or the several of a merged `regions` form. */
export function regionsOf(shape: Shape): readonly Slug[] {
  return shape.region !== undefined ? [shape.region] : (shape.regions ?? []);
}

/** ⚖️ The area-weighted value of the regions a shape paints (`undefined` when none has a value); a single-region shape returns that region's value. Merged forms are labelled merged by the views. */
export function areaWeighted(asset: Asset, shape: Shape, valueOf: (region: Slug) => number | undefined): number | undefined {
  let weight = 0;
  let sum = 0;
  for (const id of regionsOf(shape)) {
    const value = valueOf(id);
    const area = asset.thermal.regions.find((region) => region.id === id)?.areaM2 ?? 0;
    if (value === undefined || !(area > 0)) continue;
    weight += area;
    sum += area * value;
  }
  return weight > 0 ? sum / weight : undefined;
}

/** 🔭️ Selects the shapes the representation draws, maps them to paper millimetres (`k` paper mm per metre; paper-unit symbols sit at the model origin), simplifies them and sorts them by z. */
export function prepare(state: EvaluatedState, viewpoint: Viewpoint, source: Source, k: number): Prepared[] {
  const hatches = new Map(source.hatches.map((hatch) => [hatch.id, hatch]));
  const selected = state.shapes[viewpoint].flatMap((e, index) => {
    const weight = e.shape.penMm ?? source.pens[e.shape.role];
    if (weight === null) return [];
    const map = (p: { readonly x: number; readonly y: number }): Vec => (source.paperUnits ? { x: p.x, y: -p.y } : { x: p.x * k, y: -p.y * k });
    const rings = e.rings.map((r) => simplify(r.map(map), e.closed, SIMPLIFY_MM));
    const box = boxOf(rings);
    return box === null ? [] : [{ e, index, rings, box, weight }];
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
    hatch: e.shape.hatch === undefined ? null : (hatches.get(e.shape.hatch) ?? null),
    parent: parents.get(e.shape.bone) ?? null,
    occluder: e.closed && e.shape.filled && OCCLUDING_ROLES.includes(e.shape.role) ? { rings, box } : null,
  }));
}

function hash(text: string): number {
  let h = 2166136261;
  for (let i = 0; i < text.length; i++) h = Math.imul(h ^ text.charCodeAt(i), 16777619);
  return h >>> 0;
}

function unitRandom(seed: number): number {
  let x = seed || 1;
  x ^= x << 13;
  x ^= x >>> 17;
  x ^= x << 5;
  return (x >>> 0) / 4294967296;
}

/** ▤️ The hatch of a shape in paper millimetres: line segments (`lines`, `cross`) clipped to the rings, and dot centres (`dots` regular, `stipple` jittered deterministically by shape id) inside them. The pattern is anchored at the paper origin, so neighbouring shapes continue one another. */
export function hatchOf(rings: readonly Ring[], hatch: Hatch, id: string): { readonly lines: readonly (readonly [Vec, Vec])[]; readonly dots: readonly Vec[] } {
  const box = boxOf(rings);
  if (box === null || !(hatch.spacingMm > 0)) return { lines: [], dots: [] };
  const s = hatch.spacingMm;
  const theta = (hatch.angleDeg * Math.PI) / 180;
  const family = (angle: number): [Vec, Vec][] => {
    const u = { x: Math.cos(angle), y: -Math.sin(angle) };
    const n = { x: Math.sin(angle), y: Math.cos(angle) };
    const corners = [{ x: box.minX, y: box.minY }, { x: box.maxX, y: box.minY }, { x: box.maxX, y: box.maxY }, { x: box.minX, y: box.maxY }];
    const ts = corners.map((c) => c.x * n.x + c.y * n.y);
    const as = corners.map((c) => c.x * u.x + c.y * u.y);
    const a0 = Math.min(...as) - 1;
    const a1 = Math.max(...as) + 1;
    const out: [Vec, Vec][] = [];
    for (let i = Math.ceil(Math.min(...ts) / s), last = Math.floor(Math.max(...ts) / s); i <= last && out.length < HATCH_LIMIT; i++) {
      const t = i * s;
      const p = { x: n.x * t + u.x * a0, y: n.y * t + u.y * a0 };
      const q = { x: n.x * t + u.x * a1, y: n.y * t + u.y * a1 };
      out.push(...clipSegmentToRings(p, q, rings));
    }
    return out;
  };
  if (hatch.pattern === "lines") return { lines: family(theta), dots: [] };
  if (hatch.pattern === "cross") return { lines: [...family(theta), ...family(theta + Math.PI / 2)], dots: [] };
  const u = { x: Math.cos(theta), y: -Math.sin(theta) };
  const n = { x: Math.sin(theta), y: Math.cos(theta) };
  const corners = [{ x: box.minX, y: box.minY }, { x: box.maxX, y: box.minY }, { x: box.maxX, y: box.maxY }, { x: box.minX, y: box.maxY }];
  const is = corners.map((c) => c.x * u.x + c.y * u.y);
  const js = corners.map((c) => c.x * n.x + c.y * n.y);
  const dots: Vec[] = [];
  const seed = hash(id);
  for (let i = Math.floor(Math.min(...is) / s); i <= Math.ceil(Math.max(...is) / s) && dots.length < HATCH_LIMIT; i++)
    for (let j = Math.floor(Math.min(...js) / s); j <= Math.ceil(Math.max(...js) / s) && dots.length < HATCH_LIMIT; j++) {
      const jitter = hatch.pattern === "stipple" ? 0.35 : 0;
      const a = (i + (unitRandom(seed ^ Math.imul(i, 73856093) ^ Math.imul(j, 19349663)) - 0.5) * 2 * jitter) * s;
      const b = (j + (unitRandom(seed ^ Math.imul(j, 83492791) ^ Math.imul(i, 2654435761)) - 0.5) * 2 * jitter) * s;
      const p = { x: u.x * a + n.x * b, y: u.y * a + n.y * b };
      if (pointInside(p, rings)) dots.push(p);
    }
  return { lines: [], dots };
}

/** 🖌️ Paints fills (one item per filled closed shape), hatches (graphic views only, clipped to their shape and hidden behind higher-z filled shapes) and, where requested, role-pen outlines with hidden lines removed. */
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
  if (!ANALYTIC_VIEWS.includes(ctx.options.view)) {
    const solid = ctx.options.view.endsWith("-solid");
    for (const p of shapes) {
      if (p.hatch === null || !p.closed || !p.shape.filled) continue;
      const hiders = shapes.flatMap((q) => (q.occluder !== null && q.order > p.order ? [q.occluder] : []));
      const { lines, dots } = hatchOf(p.rings, p.hatch, p.shape.id);
      const commands = lines.flatMap(([a, b]) => visibleRuns([a, b], false, hiders, CLIP_EPS_MM).flatMap((run) => commandsOf(run.points, false)));
      const identity = { bone: p.shape.bone, ...(p.shape.region ? { region: p.shape.region } : {}) };
      if (commands.length > 0) canvas.path("texture", `hatch:${p.shape.id}`, commands, { stroke: { color: ink, widthMm: p.hatch.penMm, cap: "butt", ...(solid ? { opacity: 0.55 } : {}) }, ...identity });
      const radius = p.hatch.penMm / 2;
      const visible = dots.filter((d) => !hiders.some((h) => d.x >= h.box.minX && d.x <= h.box.maxX && d.y >= h.box.minY && d.y <= h.box.maxY && pointInside(d, h.rings)));
      if (visible.length > 0) canvas.path("texture", `hatch-dots:${p.shape.id}`, visible.flatMap((d) => circleCommands(d, radius)), { fill: { color: ink, ...(solid ? { opacity: 0.55 } : {}) }, ...identity });
    }
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
