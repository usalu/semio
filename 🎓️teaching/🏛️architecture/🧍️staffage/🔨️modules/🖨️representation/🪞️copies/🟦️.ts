/** 🪞️ Detection of representations that are copies of one another: a drawing authored for one scale must be redrawn, not reused for another.
 *
 * Every shape is reduced to a signature (bone, role, closedness and the flattened local rings rounded to 0.1 mm). Two representations are copies when
 * (a) their shapes have identical signatures one to one, (b) every shape of the coarser one has an identical twin in the finer one (the finer drawing with shapes deleted), or
 * (c) one equals the other with all coordinates multiplied by one common factor (a scaled copy; the factors tried are the denominator ratio and the ratio of the drawn extents; the geometry parameters are scaled before flattening, paths with arcs are not scaled).
 *
 * @see ../../🗂️catalog/🟦️.ts — issue `representation-copied`
 * @see ../../🧮️evaluation/🟦️.ts — `flattenGeometry`
 */
import type { AuthoredRepresentation, Shape } from "../../../🧬️schema/🟦️.ts";
import { flattenGeometry } from "../../🧮️evaluation/🟦️.ts";

/** 🪞️ How two representations relate: `identical` (same shapes), `subset` (all shapes of `coarse` occur identically in `fine`), `scaled` (equal up to a common factor) or `distinct`. */
export type Similarity = { readonly kind: "identical" | "subset" | "scaled" | "distinct"; readonly sharedShapes: number; readonly coarseShapes: number; readonly fineShapes: number; readonly factor?: number };

const ROUND_M = 1e-4;

function scaled(shape: Shape, factor: number): Shape {
  if (factor === 1) return shape;
  const out: Record<string, unknown> = { ...shape };
  for (const key of ["x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r"]) if (typeof out[key] === "number") out[key] = (out[key] as number) * factor;
  if (shape.kind === "poly") out.points = shape.points.map(([x, y]) => [x * factor, y * factor]);
  if (shape.kind === "path") out.d = shape.d.replace(/-?\d*\.?\d+(?:e-?\d+)?/giu, (n) => String(Number(n) * factor));
  return out as Shape;
}

function signature(shape: Shape, factor: number): string {
  const flat = flattenGeometry(scaled(shape, factor));
  const rounded = flat.rings.map((ring) => ring.map((p) => `${Math.round(p.x / ROUND_M)},${Math.round(p.y / ROUND_M)}`).join(";")).join("|");
  return `${shape.bone}/${shape.role}/${flat.closed ? "c" : "o"}/${rounded}`;
}

function signatures(shapes: readonly Shape[], factor = 1): string[] {
  return shapes.map((shape) => signature(shape, factor));
}

function extentOf(shapes: readonly Shape[]): number {
  let max = 0;
  for (const shape of shapes) for (const ring of flattenGeometry(shape).rings) for (const p of ring) max = Math.max(max, Math.abs(p.x), Math.abs(p.y));
  return max;
}

function multisetEqual(a: readonly string[], b: readonly string[]): boolean {
  if (a.length !== b.length) return false;
  const counts = new Map<string, number>();
  for (const item of a) counts.set(item, (counts.get(item) ?? 0) + 1);
  for (const item of b) {
    const left = counts.get(item) ?? 0;
    if (left === 0) return false;
    counts.set(item, left - 1);
  }
  return true;
}

/** 🪞️ Compares the elevation (and plan) geometry of two authored representations; `coarse` is the one with the larger denominator. */
export function compareRepresentations(coarse: AuthoredRepresentation, fine: AuthoredRepresentation): Similarity {
  const a = [...coarse.geometry.elevation, ...(coarse.geometry.plan ?? [])];
  const b = [...fine.geometry.elevation, ...(fine.geometry.plan ?? [])];
  const sa = signatures(a);
  const sb = signatures(b);
  const pool = new Map<string, number>();
  for (const item of sb) pool.set(item, (pool.get(item) ?? 0) + 1);
  let shared = 0;
  for (const item of sa) {
    const left = pool.get(item) ?? 0;
    if (left > 0) {
      shared++;
      pool.set(item, left - 1);
    }
  }
  const base = { sharedShapes: shared, coarseShapes: a.length, fineShapes: b.length };
  if (multisetEqual(sa, sb)) return { kind: "identical", ...base };
  if (a.length > 0 && shared === a.length) return { kind: "subset", ...base };
  const ea = extentOf(a);
  const eb = extentOf(b);
  const factors = new Set<number>([coarse.denominator / fine.denominator, fine.denominator / coarse.denominator]);
  if (ea > 0 && eb > 0) {
    factors.add(eb / ea);
    factors.add(ea / eb);
  }
  for (const factor of factors) {
    if (Math.abs(factor - 1) < 1e-9) continue;
    if (multisetEqual(signatures(a, factor), sb)) return { kind: "scaled", factor, ...base };
  }
  return { kind: "distinct", ...base };
}
