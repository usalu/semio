/** 🖇️ Stitching of fragments into clean vector paths: the union of non-overlapping convex polygons by cancelling shared edges, and the chaining of segments into polylines.
 *
 * Points are matched on a grid of `quantum` (a point is the integer pair of its rounded coordinates), so fragments that were cut separately at the same place meet exactly. The union keeps every polygon orientation (counter-clockwise polygons give counter-clockwise outlines and clockwise holes), which is what a non-zero fill needs; vertices on a straight line are dropped.
 */
import * as portable from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🧮️math/🎯️portable/🟦️.ts";

/** 📍️ A ring or polyline as flat `x, y` pairs; `closed` rings do not repeat the first point. */
export type Path = { readonly points: readonly number[]; readonly closed: boolean };

const key = (x: number, y: number, quantum: number): string => `${Math.round(x / quantum)},${Math.round(y / quantum)}`;

function straighten(points: readonly number[], closed: boolean, tolerance: number): number[] {
  const n = points.length / 2;
  if (n < 3) return [...points];
  const keep: number[] = [];
  for (let i = 0; i < n; i++) {
    if (!closed && (i === 0 || i === n - 1)) {
      keep.push(points[2 * i]!, points[2 * i + 1]!);
      continue;
    }
    const p = (i + n - 1) % n, q = (i + 1) % n;
    const ax = points[2 * i]! - points[2 * p]!, ay = points[2 * i + 1]! - points[2 * p + 1]!;
    const bx = points[2 * q]! - points[2 * i]!, by = points[2 * q + 1]! - points[2 * i + 1]!;
    const cross = ax * by - ay * bx;
    const along = ax * bx + ay * by;
    if (Math.abs(cross) <= tolerance * portable.hypot(ax, ay, bx, by) && along > 0) continue;
    keep.push(points[2 * i]!, points[2 * i + 1]!);
  }
  return keep;
}

/** 🧩️ The union outline of non-overlapping polygons: shared edges cancel, the remaining directed edges are walked into closed rings. */
export function unionOf(polygons: readonly (readonly number[])[], quantum: number): Path[] {
  const points = new Map<string, [number, number]>();
  const edges = new Map<string, [string, string][]>();
  const cancel = (from: string, to: string): boolean => {
    const opposite = edges.get(`${to}>${from}`);
    if (opposite === undefined || opposite.length === 0) return false;
    opposite.pop();
    return true;
  };
  for (const polygon of polygons) {
    const n = polygon.length / 2;
    if (n < 3) continue;
    const ids: string[] = [];
    for (let i = 0; i < n; i++) {
      const id = key(polygon[2 * i]!, polygon[2 * i + 1]!, quantum);
      if (!points.has(id)) points.set(id, [polygon[2 * i]!, polygon[2 * i + 1]!]);
      ids.push(id);
    }
    for (let i = 0; i < n; i++) {
      const from = ids[i]!, to = ids[(i + 1) % n]!;
      if (from === to || cancel(from, to)) continue;
      const slot = `${from}>${to}`;
      const list = edges.get(slot);
      if (list === undefined) edges.set(slot, [[from, to]]);
      else list.push([from, to]);
    }
  }
  const outgoing = new Map<string, string[]>();
  for (const list of edges.values()) for (const [from, to] of list) (outgoing.get(from) ?? outgoing.set(from, []).get(from)!).push(to);
  const rings: Path[] = [];
  for (const [start, targets] of outgoing) {
    while (targets.length > 0) {
      const ring: string[] = [start];
      let at = targets.pop()!;
      let guard = 0;
      while (at !== start && guard++ < 1_000_000) {
        ring.push(at);
        const next = outgoing.get(at);
        if (next === undefined || next.length === 0) break;
        at = next.pop()!;
      }
      if (at !== start || ring.length < 3) continue;
      const flat = ring.flatMap((id) => points.get(id)!);
      const straight = straighten(flat, true, 1e-9);
      if (straight.length >= 6) rings.push({ points: straight, closed: true });
    }
  }
  return rings;
}

/** 🔗️ The polylines of a set of segments (flat `x0, y0, x1, y1`): chains through points of degree two, so a junction ends a polyline; a chain that returns to its start is closed. Segments of zero length are dropped. */
export function chainOf(segments: readonly (readonly [number, number, number, number])[], quantum: number): Path[] {
  const points = new Map<string, [number, number]>();
  const around = new Map<string, number[]>();
  const kept: { a: string; b: string }[] = [];
  const seen = new Set<string>();
  for (const [x0, y0, x1, y1] of segments) {
    const a = key(x0, y0, quantum), b = key(x1, y1, quantum);
    if (a === b) continue;
    const pair = a < b ? `${a}|${b}` : `${b}|${a}`;
    if (seen.has(pair)) continue;
    seen.add(pair);
    if (!points.has(a)) points.set(a, [x0, y0]);
    if (!points.has(b)) points.set(b, [x1, y1]);
    const id = kept.push({ a, b }) - 1;
    (around.get(a) ?? around.set(a, []).get(a)!).push(id);
    (around.get(b) ?? around.set(b, []).get(b)!).push(id);
  }
  const used = new Uint8Array(kept.length);
  const walk = (from: string, first: number): { ids: string[]; end: string } => {
    const ids = [from];
    let at = from;
    let edge = first;
    for (;;) {
      used[edge] = 1;
      const e = kept[edge]!;
      at = e.a === at ? e.b : e.a;
      ids.push(at);
      const list = around.get(at)!;
      if (list.length !== 2) break;
      const next = list[0] === edge ? list[1]! : list[0]!;
      if (used[next] === 1) break;
      edge = next;
    }
    return { ids, end: at };
  };
  const paths: Path[] = [];
  const emit = (ids: string[], closed: boolean): void => {
    const flat = ids.flatMap((id) => points.get(id)!);
    const straight = straighten(closed ? flat.slice(0, -2) : flat, closed, 1e-9);
    if (straight.length >= 4) paths.push({ points: straight, closed });
  };
  for (const [id, list] of around) {
    if (list.length === 2) continue;
    for (const edge of list) if (used[edge] === 0) emit(walk(id, edge).ids, false);
  }
  for (let e = 0; e < kept.length; e++) {
    if (used[e] === 1) continue;
    const { ids } = walk(kept[e]!.a, e);
    emit(ids, ids[0] === ids[ids.length - 1]);
  }
  return paths;
}
