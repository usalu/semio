/** 🌓️ The silhouette agreement of the 2D drawings and the 3D volume of one asset, shared by the catalog-wide guard and the ticket tools.
 *
 * One physical body: in a pose its side elevation, front elevation, plan and volume are projections of the same body. The authored side is what `evaluate` draws (filled `body`, `detail` and `glazing` shapes; open strokes and unfilled outlines at the true width of the member they draw), the other side the kernel tessellation of the solids carried by the bone frames of the pose and projected through the staffage cameras.
 * A view declared as a section (`representation.sections`) is compared with the volume cut by the plane: the cut faces of the bodies the plane crosses, which it must show; what remains visible beyond the plane, which it may show; and what lies in front of the plane, which it may only outline.
 * For a section the agreement is |(filled ∩ visible) ∪ (outlines ∩ full)| / |A ∪ cut|: the drawing may fill anything that remains visible beyond the plane and outline any part of the body (overhead and hidden lines), but must show every cut face and nothing that is not there. The area measure is an owned scanline rasteriser; the independent references live in `./oracles` (polygon-clipping exact areas, sharp's librsvg rasteriser).
 *
 * @see ../../🧊️model-silhouette/🥒️.feature
 * @see ../../🧊️catalog-silhouette/🥒️.feature
 */
import { tessellateSolid } from "../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import type { Asset, ViewSection, VolumeSolid } from "../../../🧬️schema/🟦️.ts";
import { CAMERAS } from "../../../🔨️modules/🎨️depiction/🔭️views/🟦️.ts";
import { evaluate } from "../../../🔨️modules/🧮️evaluation/🟦️.ts";
import { qualityOf } from "../../../🔨️modules/🧊️volume/⏱️job/🟦️.ts";

/** 👁️ The three planar viewpoints. */
export const VIEWPOINTS = ["elevation", "front", "plan"] as const;
export type Viewpoint = (typeof VIEWPOINTS)[number];

/** 🚦️ The gate of the guard: the bounding box within 8 % per axis, IoU of at least 0.80 per view, at the working scale 1:50. */
export const GATE = { bbox: 0.08, iou: 0.8, scale: 50, memberMaxPaperMm: 3, cells: 800, cellsMax: 4000, cellM: 0.02 } as const;

export type Point = { readonly x: number; readonly y: number };
export type Ring = readonly Point[];
/** 🧩️ Rings that fill together by the even-odd rule (a polygon with holes, a single triangle). */
export type Group = readonly Ring[];
export type World = { readonly id: string; readonly positions: Float64Array; readonly indices: Uint32Array };
type Vec = readonly [number, number, number];

/** 🧭️ The cut axis (world x, y, z) and the sign of the retained side of the section of each viewpoint: the elevation is seen from -z (what lies at larger z remains), the front from +x (smaller x), the plan from +y (smaller y). */
export const CUT = { elevation: { axis: 2, retained: 1 }, front: { axis: 0, retained: -1 }, plan: { axis: 1, retained: -1 } } as const;

/** 🧊️ Every solid shown at 1:`scale`, tessellated by the kernel and carried to the world by the bone frames of the evaluated pose. */
export function worldSolids(asset: Asset, bones: Record<string, { readonly frame: { readonly o: { x: number; y: number; z: number }; readonly u: { x: number; y: number; z: number }; readonly v: { x: number; y: number; z: number }; readonly w: { x: number; y: number; z: number } } }>, scale: number): World[] {
  const quality = qualityOf(asset);
  return asset.volume.solids.filter((s: VolumeSolid) => s.maxDenominator === undefined || scale <= s.maxDenominator).map((solid) => {
    const f = bones[solid.bone]!.frame;
    const g = tessellateSolid({ ...solid, node: solid.bone } as never, quality);
    const positions = new Float64Array(g.positions.length);
    for (let i = 0; i < g.positions.length; i += 3) {
      const [a, b, c] = [g.positions[i]!, g.positions[i + 1]!, g.positions[i + 2]!];
      positions[i] = f.o.x + a * f.u.x + b * f.v.x + c * f.w.x;
      positions[i + 1] = f.o.y + a * f.u.y + b * f.v.y + c * f.w.y;
      positions[i + 2] = f.o.z + a * f.u.z + b * f.v.z + c * f.w.z;
    }
    return { id: solid.id, positions, indices: Uint32Array.from(g.indices) };
  });
}

/** 📦️ The extent of the solids along world x, y and z. */
export function extentOf(solids: readonly World[]): { readonly min: Vec; readonly max: Vec } {
  const min: [number, number, number] = [Infinity, Infinity, Infinity];
  const max: [number, number, number] = [-Infinity, -Infinity, -Infinity];
  for (const s of solids) for (let i = 0; i < s.positions.length; i += 3) for (let k = 0; k < 3; k++) {
    const v = s.positions[i + k]!;
    if (v < min[k]!) min[k] = v;
    if (v > max[k]!) max[k] = v;
  }
  return { min, max };
}

/** 📐️ Orthographic projection of a world point through the camera of a viewpoint. */
export function project(view: Viewpoint, x: number, y: number, z: number): Point {
  const c = CAMERAS[view];
  const pick = (axis: "x" | "y" | "z"): number => (axis === "x" ? x : axis === "y" ? y : z);
  return { x: c.paperX.sign * pick(c.paperX.axis), y: c.paperUp.sign * pick(c.paperUp.axis) };
}

const KEY = 1e6;

/** ✂️ The cut face of a closed triangle mesh by the plane `coordinate[axis] = plane`: the loops of the intersection, assembled from the triangle segments by their end points (rounded to a micrometre). */
export function cutLoops(solid: World, axis: number, plane: number): readonly (readonly Vec[])[] {
  const { positions: p, indices } = solid;
  const segments: [Vec, Vec][] = [];
  const at = (i: number): Vec => [p[i * 3]!, p[i * 3 + 1]!, p[i * 3 + 2]!];
  const cross = (a: Vec, b: Vec): Vec => {
    const [da, db] = [a[axis]! - plane, b[axis]! - plane];
    const lexicographic = a[0]! < b[0]! || (a[0] === b[0] && (a[1]! < b[1]! || (a[1] === b[1] && a[2]! < b[2]!)));
    const [lo, hi] = lexicographic ? [a, b] : [b, a];
    const [dlo, dhi] = lexicographic ? [da, db] : [db, da];
    const t = dlo / (dlo - dhi);
    return [lo[0]! + (hi[0]! - lo[0]!) * t, lo[1]! + (hi[1]! - lo[1]!) * t, lo[2]! + (hi[2]! - lo[2]!) * t];
  };
  for (let t = 0; t < indices.length; t += 3) {
    const v = [at(indices[t]!), at(indices[t + 1]!), at(indices[t + 2]!)];
    const above = v.map((q) => q[axis]! >= plane);
    if (above[0] === above[1] && above[1] === above[2]) continue;
    const ends: Vec[] = [];
    for (let k = 0; k < 3; k++) if (above[k] !== above[(k + 1) % 3]) ends.push(cross(v[k]!, v[(k + 1) % 3]!));
    if (ends.length === 2) segments.push([ends[0]!, ends[1]!]);
  }
  const key = (q: Vec): string => `${Math.round(q[0]! * KEY)},${Math.round(q[1]! * KEY)},${Math.round(q[2]! * KEY)}`;
  const byEnd = new Map<string, number[]>();
  segments.forEach(([a, b], i) => {
    for (const q of [a, b]) {
      const list = byEnd.get(key(q));
      if (list === undefined) byEnd.set(key(q), [i]);
      else list.push(i);
    }
  });
  const used = new Uint8Array(segments.length);
  const loops: Vec[][] = [];
  for (let s = 0; s < segments.length; s++) {
    if (used[s]) continue;
    used[s] = 1;
    const loop: Vec[] = [segments[s]![0]];
    let tail = segments[s]![1];
    const first = key(loop[0]!);
    while (key(tail) !== first) {
      loop.push(tail);
      const next = (byEnd.get(key(tail)) ?? []).find((i) => !used[i]);
      if (next === undefined) break;
      used[next] = 1;
      const [a, b] = segments[next]!;
      tail = key(a) === key(tail) ? b : a;
    }
    if (key(tail) === first && loop.length >= 3) loops.push(loop);
  }
  return loops;
}

/** 🧱️ The connected pieces of a mesh: groups of triangle indices that share a vertex (positions rounded to a micrometre), so that two bodies of one solid (the two mullions of a bay) count apart. */
export function pieces(solid: World): readonly World[] {
  const { positions: p, indices } = solid;
  const parent = new Int32Array(indices.length / 3).map((_, i) => i);
  const find = (i: number): number => {
    while (parent[i] !== i) {
      parent[i] = parent[parent[i]!]!;
      i = parent[i]!;
    }
    return i;
  };
  const seen = new Map<string, number>();
  for (let t = 0; t < indices.length / 3; t++) for (let k = 0; k < 3; k++) {
    const v = indices[t * 3 + k]!;
    const key = `${Math.round(p[v * 3]! * KEY)},${Math.round(p[v * 3 + 1]! * KEY)},${Math.round(p[v * 3 + 2]! * KEY)}`;
    const other = seen.get(key);
    if (other === undefined) seen.set(key, t);
    else parent[find(t)] = find(other);
  }
  const groups = new Map<number, number[]>();
  for (let t = 0; t < indices.length / 3; t++) {
    const root = find(t);
    const list = groups.get(root);
    if (list === undefined) groups.set(root, [t]);
    else list.push(t);
  }
  return [...groups.values()].map((triangles) => ({ id: solid.id, positions: p, indices: Uint32Array.from(triangles.flatMap((t) => [indices[t * 3]!, indices[t * 3 + 1]!, indices[t * 3 + 2]!])) }));
}

/** 🔪️ The part of a triangle on one side of the plane `coordinate[axis] = plane` (Sutherland-Hodgman), as a polygon of 3D points. */
export function clipTriangle(t: readonly Vec[], axis: number, plane: number, keepHigh: boolean): readonly Vec[] {
  const inside = (q: Vec): boolean => (keepHigh ? q[axis]! >= plane : q[axis]! < plane);
  const out: Vec[] = [];
  for (let i = 0; i < t.length; i++) {
    const [a, b] = [t[i]!, t[(i + 1) % t.length]!];
    if (inside(a)) out.push(a);
    if (inside(a) !== inside(b)) {
      const f = (plane - a[axis]!) / (b[axis]! - a[axis]!);
      out.push([a[0]! + (b[0]! - a[0]!) * f, a[1]! + (b[1]! - a[1]!) * f, a[2]! + (b[2]! - a[2]!) * f]);
    }
  }
  return out;
}

/** 📚️ What a view is compared with. `cut` is what the drawing has to show: every projected triangle of the solids, or for a declared section the cut faces of the bodies the plane crosses. `visible` is all the drawing may fill: every projected triangle, or for a section the projection of everything that remains beyond the plane (the far faces of a cut body included, clipped at the plane). A body in front of the plane is removed from `cut` and `visible`; `full` is the projection of the whole volume, where a section may still show a removed or hidden body as a thin outline (the overhead and hidden-line convention). */
export function referenceOf(view: Viewpoint, solids: readonly World[], section: ViewSection | undefined): { readonly cut: Group[]; readonly visible: Group[]; readonly full: Group[] } {
  const cut: Group[] = [];
  const visible: Group[] = [];
  const full: Group[] = [];
  const side = CUT[view];
  const at = (s: World, i: number): Vec => [s.positions[i * 3]!, s.positions[i * 3 + 1]!, s.positions[i * 3 + 2]!];
  const flat = (polygon: readonly Vec[]): Group => [polygon.map((q) => project(view, q[0]!, q[1]!, q[2]!))];
  for (const whole of solids) for (const s of section === undefined ? [whole] : pieces(whole)) {
    if (section !== undefined) for (let t = 0; t < whole.indices.length; t += 3) full.push(flat([at(whole, whole.indices[t]!), at(whole, whole.indices[t + 1]!), at(whole, whole.indices[t + 2]!)]));
    let low = false;
    let high = false;
    if (section !== undefined) for (let t = 0; t < s.indices.length; t++) (s.positions[s.indices[t]! * 3 + side.axis]! < section.plane ? (low = true) : (high = true));
    const crossing = low && high;
    if (section !== undefined && !crossing && (side.retained > 0 ? !high : !low)) continue;
    for (let t = 0; t < s.indices.length; t += 3) {
      const triangle = [at(s, s.indices[t]!), at(s, s.indices[t + 1]!), at(s, s.indices[t + 2]!)];
      const kept = section === undefined || !crossing ? triangle : clipTriangle(triangle, side.axis, section.plane, side.retained > 0);
      if (kept.length >= 3) visible.push(flat(kept));
      if (section === undefined) cut.push(flat(triangle));
    }
    if (crossing && section !== undefined) cut.push(cutLoops(s, side.axis, section.plane).map((loop) => loop.map((q) => project(view, q[0]!, q[1]!, q[2]!))));
  }
  return { cut, visible, full: section === undefined ? visible : full };
}

export type Box = { readonly minX: number; readonly minY: number; readonly maxX: number; readonly maxY: number };

/** 🖼️ A binary coverage grid sampled at cell centres, filled by the even-odd scanline rule. */
export class Mask {
  readonly width: number;
  readonly height: number;
  readonly cells: Uint8Array;
  constructor(readonly box: Box, readonly cell: number) {
    this.width = Math.max(1, Math.ceil((box.maxX - box.minX) / cell));
    this.height = Math.max(1, Math.ceil((box.maxY - box.minY) / cell));
    this.cells = new Uint8Array(this.width * this.height);
  }

  fill(rings: Group): void {
    const { box, cell, width, height, cells } = this;
    let lo = Infinity;
    let hi = -Infinity;
    for (const ring of rings) for (const p of ring) {
      if (p.y < lo) lo = p.y;
      if (p.y > hi) hi = p.y;
    }
    if (!Number.isFinite(lo)) return;
    const xs: number[] = [];
    for (let j = Math.max(0, Math.ceil((lo - box.minY) / cell - 0.5)); j <= Math.min(height - 1, Math.floor((hi - box.minY) / cell - 0.5)); j++) {
      const y = box.minY + (j + 0.5) * cell;
      xs.length = 0;
      for (const ring of rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
        const [a, b] = [ring[k]!, ring[i]!];
        if (a.y <= y !== b.y <= y) xs.push(a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x));
      }
      xs.sort((a, b) => a - b);
      for (let n = 0; n + 1 < xs.length; n += 2) for (let i = Math.max(0, Math.ceil((xs[n]! - box.minX) / cell - 0.5)); i <= Math.min(width - 1, Math.ceil((xs[n + 1]! - box.minX) / cell - 0.5) - 1); i++) cells[j * width + i] = 1;
    }
  }

  has(x: number, y: number): boolean {
    const i = Math.floor((x - this.box.minX) / this.cell);
    const j = Math.floor((y - this.box.minY) / this.cell);
    return i >= 0 && j >= 0 && i < this.width && j < this.height && this.cells[j * this.width + i] === 1;
  }
}

/** 🧵️ The quadrilateral of a stroke segment of the given width, its ends extended by half the width. */
export function strokeQuad(a: Point, b: Point, width: number): Ring {
  const h = width / 2;
  const l = Math.hypot(b.x - a.x, b.y - a.y);
  const [ex, ey] = l < 1e-12 ? [h, 0] : [((b.x - a.x) / l) * h, ((b.y - a.y) / l) * h];
  const [nx, ny] = [-ey, ex];
  return [{ x: a.x - ex + nx, y: a.y - ey + ny }, { x: b.x + ex + nx, y: b.y + ey + ny }, { x: b.x + ex - nx, y: b.y + ey - ny }, { x: a.x - ex - nx, y: a.y - ey - ny }];
}

const RING: readonly (readonly [number, number])[] = [[0, 0], [1, 0], [-1, 0], [0, 1], [0, -1], [0.7, 0.7], [-0.7, 0.7], [0.7, -0.7], [-0.7, -0.7]];
const beside = (mask: Mask, x: number, y: number, radius: number): boolean => RING.some(([dx, dy]) => mask.has(x + dx! * radius, y + dy! * radius));

/** 🪡️ The member a stroke segment draws: at samples along the segment, the largest stripe centred on the stroke that the body fills (twice the shorter of the two distances to its boundary along the normal, measured on the first of `bodies` that holds the sample, the cut faces before the whole projection), the median of the samples, capped at {@link GATE.memberMaxPaperMm} of paper. A stroke on the boundary of a body is the edge of that body, not a member: it counts two cells wide, shifted into the body. One that lies within a pen width of a closed shape of the drawing (a seam or detail line on a body) is a line of the drawing and counts two cells wide; one that lies on no body keeps its pen width, so a line drawn where nothing is costs its ink. Returns the width and the signed shift along the normal of the segment. */
export function memberOf(a: Point, b: Point, bodies: readonly Mask[], drawn: Mask, penM: number, capM: number): { readonly width: number; readonly shift: number } {
  const l = Math.hypot(b.x - a.x, b.y - a.y);
  if (l < 1e-12) return { width: penM, shift: 0 };
  const [nx, ny] = [-(b.y - a.y) / l, (b.x - a.x) / l];
  const samples = Math.max(1, Math.min(9, Math.round(l / Math.max(bodies[0]!.cell * 2, penM))));
  const found: { width: number; shift: number }[] = [];
  for (let s = 0; s < samples; s++) {
    const t = (s + 0.5) / samples;
    const [x, y] = [a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t];
    const body = bodies.find((m) => beside(m, x, y, m.cell));
    if (body === undefined) {
      found.push({ width: penM, shift: 0 });
      continue;
    }
    if (beside(drawn, x, y, penM)) {
      found.push({ width: 2 * body.cell, shift: 0 });
      continue;
    }
    const steps = Math.ceil(capM / 2 / body.cell);
    let up = 0;
    while (up < steps && body.has(x + nx * (up + 1) * body.cell, y + ny * (up + 1) * body.cell)) up++;
    let down = 0;
    while (down < steps && body.has(x - nx * (down + 1) * body.cell, y - ny * (down + 1) * body.cell)) down++;
    if (Math.min(up, down) <= 1 && Math.max(up, down) > 2) found.push({ width: 2 * body.cell, shift: (up > down ? 1 : -1) * body.cell });
    else found.push({ width: Math.min((2 * Math.min(up, down) + 1) * body.cell, capM), shift: 0 });
  }
  found.sort((p, q) => p.width - q.width);
  return found[found.length >> 1]!;
}

/** 🫙️ Whether the volume fills at least half of the area an unfilled closed outline encloses (the outline of a solid part, drawn as its boundary) rather than less (a hoop or a frame around a void). */
export function encloses(rings: Group, volume: Mask): boolean {
  let lo = Infinity, hi = -Infinity, left = Infinity, right = -Infinity;
  for (const ring of rings) for (const p of ring) {
    lo = Math.min(lo, p.y);
    hi = Math.max(hi, p.y);
    left = Math.min(left, p.x);
    right = Math.max(right, p.x);
  }
  const probe = new Mask({ minX: left, minY: lo, maxX: right, maxY: hi }, Math.max(volume.cell, Math.sqrt(((right - left) * (hi - lo)) / 200000)));
  probe.fill(rings);
  let inside = 0;
  let filled = 0;
  for (let j = 0; j < probe.height; j++) for (let i = 0; i < probe.width; i++) {
    if (!probe.cells[j * probe.width + i]) continue;
    inside++;
    if (volume.has(left + (i + 0.5) * probe.cell, lo + (j + 0.5) * probe.cell)) filled++;
  }
  return inside > 0 && filled >= inside / 2;
}

export type Measured = { readonly iou: number; readonly recall: number; readonly precision: number; readonly filled: readonly Group[]; readonly outlines: readonly Group[]; readonly reference: readonly Group[]; readonly visible: readonly Group[]; readonly full: readonly Group[]; readonly section: boolean; readonly box: Box; readonly cell: number; readonly openStrokes: number; readonly shapes: number };

type Evaluated = ReturnType<typeof evaluate>;

/** 🔬️ Compares one view of an evaluated pose with the reference of the volume on one raster of `cells` cells across the long side (by default 800, more where a cell would be coarser than 20 mm, at most 4000). */
export function measureView(asset: Asset, state: Evaluated, solids: readonly World[], view: Viewpoint, scale: number, cells?: number): Measured {
  const rep = asset.representations[String(scale) as unknown as keyof Asset["representations"]] as unknown as { readonly pens: Record<string, number | null>; readonly sections?: Partial<Record<Viewpoint, ViewSection>> };
  const section = rep.sections?.[view];
  const { cut: reference, visible, full } = referenceOf(view, solids, section);
  const evaluated = (state.shapes[view] as readonly { readonly shape: { readonly id: string; readonly role: string; readonly penMm?: number; readonly filled?: boolean }; readonly rings: readonly Ring[]; readonly closed: boolean }[]).filter((s) => ["body", "detail", "glazing"].includes(s.shape.role) && s.rings.length > 0);
  const bounds = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  const grow = (p: Point): void => {
    if (p.x < bounds.minX) bounds.minX = p.x;
    if (p.x > bounds.maxX) bounds.maxX = p.x;
    if (p.y < bounds.minY) bounds.minY = p.y;
    if (p.y > bounds.maxY) bounds.maxY = p.y;
  };
  for (const g of full) for (const r of g) for (const p of r) grow(p);
  for (const g of reference) for (const r of g) for (const p of r) grow(p);
  for (const s of evaluated) for (const r of s.rings) for (const p of r) grow(p);
  const long = Math.max(bounds.maxX - bounds.minX, bounds.maxY - bounds.minY);
  const cell = Math.max(2e-4, long / (cells ?? Math.min(GATE.cellsMax, Math.max(GATE.cells, Math.round(long / GATE.cellM)))));
  const box: Box = { minX: bounds.minX - 2 * cell, minY: bounds.minY - 2 * cell, maxX: bounds.maxX + 2 * cell, maxY: bounds.maxY + 2 * cell };
  const volume = new Mask(box, cell);
  for (const g of reference) volume.fill(g);
  const upper = section === undefined ? volume : new Mask(box, cell);
  const whole = section === undefined ? volume : new Mask(box, cell);
  if (section !== undefined) {
    for (const g of visible) upper.fill(g);
    for (const g of full) whole.fill(g);
  }
  let fine = volume;
  let fineFull = whole;
  const pen = (s: (typeof evaluated)[number]): number => ((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000;
  const candidates = evaluated.filter((s) => !s.closed || s.shape.filled === false);
  if (candidates.length > 0) {
    const smallest = Math.min(...candidates.map((s) => Math.max(pen(s), 1e-4)));
    const finer = Math.min(cell, Math.max(smallest / 4, long / 6000, 5e-4));
    if (finer < cell * 0.99) {
      fine = new Mask(box, finer);
      for (const g of reference) fine.fill(g);
      if (section !== undefined) {
        fineFull = new Mask(box, finer);
        for (const g of full) fineFull.fill(g);
      }
    }
  }
  const solid = new Map<(typeof evaluated)[number], boolean>();
  const fills = (s: (typeof evaluated)[number]): boolean => {
    if (!s.closed) return false;
    if (s.shape.filled !== false) return true;
    let known = solid.get(s);
    if (known === undefined) {
      known = encloses(s.rings, fine);
      solid.set(s, known);
    }
    return known;
  };
  const strokes = evaluated.filter((s) => !fills(s));
  const filled: Group[] = [];
  const outlines: Group[] = [];
  const drawn = new Mask(box, fine.cell);
  for (const s of evaluated) if (fills(s)) drawn.fill(s.rings);
  const capM = (GATE.memberMaxPaperMm * scale) / 1000;
  for (const s of evaluated) {
    if (fills(s)) filled.push(s.rings);
    else for (const ring of s.rings) {
      const r = s.closed ? [...ring, ring[0]!] : ring;
      for (let i = 0; i + 1 < r.length; i++) {
        const [p, q] = [r[i]!, r[i + 1]!];
        const { width, shift } = memberOf(p, q, section === undefined ? [fine] : [fine, fineFull], drawn, Math.max(pen(s), 1e-4), capM);
        const l = Math.hypot(q.x - p.x, q.y - p.y) || 1;
        const [dx, dy] = [(-(q.y - p.y) / l) * shift, ((q.x - p.x) / l) * shift];
        outlines.push([strokeQuad({ x: p.x + dx, y: p.y + dy }, { x: q.x + dx, y: q.y + dy }, width)]);
      }
    }
  }
  const ink = new Mask(box, cell);
  const area = new Mask(box, cell);
  for (const g of filled) area.fill(g);
  for (const g of outlines) ink.fill(g);
  let both = 0;
  let either = 0;
  let a = 0;
  let v = 0;
  let drawnCut = 0;
  for (let i = 0; i < area.cells.length; i++) {
    const [f, o, y, u, w] = [area.cells[i]!, ink.cells[i]!, volume.cells[i]!, upper.cells[i]!, whole.cells[i]!];
    const x = f | o;
    a += x;
    v += y;
    drawnCut += x & y;
    both += (f & u) | (o & w);
    either += x | y;
  }
  return { iou: either === 0 ? 1 : both / either, recall: a === 0 ? 1 : both / a, precision: v === 0 ? 1 : drawnCut / v, filled, outlines, reference, visible, full, section: section !== undefined, box, cell, openStrokes: strokes.length, shapes: evaluated.length };
}

export type AssetMeasure = { readonly asset: string; readonly pose: string; readonly bbox: readonly [number, number, number]; readonly bboxError: readonly [number, number, number]; readonly views: Record<Viewpoint, Measured>; readonly sections: Partial<Record<Viewpoint, ViewSection>> };

/** 🧪️ The bounding box against `dimensions` and the three views of an asset in its presentation pose at 1:`scale`. */
export function measureAsset(asset: Asset, scale: number = GATE.scale, pose: string = asset.presentation.defaultPose, cells?: number): AssetMeasure {
  const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: scale } });
  const solids = worldSolids(asset, state.bones as never, scale);
  const e = extentOf(solids);
  const d = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  const bbox = [0, 1, 2].map((k) => e.max[k]! - e.min[k]!) as [number, number, number];
  const rep = asset.representations[String(scale) as unknown as keyof Asset["representations"]] as unknown as { readonly sections?: Partial<Record<Viewpoint, ViewSection>> };
  const views = Object.fromEntries(VIEWPOINTS.map((v) => [v, measureView(asset, state, solids, v, scale, cells)])) as Record<Viewpoint, Measured>;
  return { asset: asset.id, pose, bbox, bboxError: bbox.map((b, k) => (b - d[k]!) / d[k]!) as [number, number, number], views, sections: rep.sections ?? {} };
}
