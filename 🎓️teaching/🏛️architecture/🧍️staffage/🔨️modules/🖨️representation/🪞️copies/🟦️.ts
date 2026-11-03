/** 🪞️ Detection of representations that are copies of one another: a drawing authored for one scale must be redrawn, not reused for another.
 *
 * Every shape is reduced to a signature (bone, role, closedness and the flattened local rings rounded to 0.1 mm). Two representations are copies when
 * (a) their shapes have identical signatures one to one, (b) every shape of the coarser one has an identical twin in the finer one (the finer drawing with shapes deleted), or
 * (c) one equals the other with all coordinates multiplied by one common factor (a scaled copy; the factors tried are the denominator ratio and the ratio of the drawn extents; the geometry parameters are scaled before flattening, paths with arcs are not scaled).
 *
 * @see ../../🗂️catalog/🟦️.ts — issue `representation-copied`
 * @see ../../🧮️evaluation/🟦️.ts — `flattenGeometry`
 */
import { VIEWPOINTS, type AuthoredRepresentation, type Shape, type Viewpoint } from "../../../🧬️schema/🟦️.ts";
import { flattenGeometry } from "../../🧮️evaluation/🟦️.ts";

/** 🔗️ How two representations relate: `identical` (same shapes), `subset` (all shapes of `coarse` occur identically in `fine`), `scaled` (equal up to a common factor) or `distinct`. */
export type Similarity = { readonly kind: "identical" | "subset" | "scaled" | "distinct"; readonly sharedShapes: number; readonly coarseShapes: number; readonly fineShapes: number; readonly factor?: number };

const ROUND_M = 1e-4;

const NUMBER = /-?\d*\.?\d+(?:e-?\d+)?/giu;

/** 📏️ A path with its lengths multiplied by `factor`; of an arc (`A`, `a`) only `rx`, `ry`, `x` and `y` scale, the rotation and the large-arc and sweep flags stay. */
function scaledPath(d: string, factor: number): string {
  return d.replace(/([a-df-zA-DF-Z])([^a-df-zA-DF-Z]*)/gu, (_, command: string, args: string) => {
    let index = 0;
    return command + args.replace(NUMBER, (n) => {
      const position = index++ % 7;
      const keep = command.toLowerCase() === "a" && (position === 2 || position === 3 || position === 4);
      return keep ? n : String(Number(n) * factor);
    });
  });
}

function scaled(shape: Shape, factor: number): Shape {
  if (factor === 1) return shape;
  const out: Record<string, unknown> = { ...shape };
  for (const key of ["x0", "y0", "x1", "y1", "r0", "r1", "cx", "cy", "rx", "ry", "x", "y", "w", "h", "r"]) if (typeof out[key] === "number") out[key] = (out[key] as number) * factor;
  if (shape.kind === "poly") out.points = shape.points.map(([x, y]) => [x * factor, y * factor]);
  if (shape.kind === "path") out.d = scaledPath(shape.d, factor);
  return out as Shape;
}

type Placed = { readonly shape: Shape; readonly viewpoint: Viewpoint };

function placed(rep: AuthoredRepresentation): Placed[] {
  return VIEWPOINTS.flatMap((viewpoint) => rep.geometry[viewpoint].map((shape) => ({ shape, viewpoint })));
}

function signature({ shape, viewpoint }: Placed, factor: number): string {
  const flat = flattenGeometry(scaled(shape, factor));
  const rounded = flat.rings.map((ring) => ring.map((p) => `${Math.round(p.x / ROUND_M)},${Math.round(p.y / ROUND_M)}`).join(";")).join("|");
  return `${viewpoint}/${shape.bone}/${shape.role}/${flat.closed ? "c" : "o"}/${rounded}`;
}

function signatures(shapes: readonly Placed[], factor = 1): string[] {
  return shapes.map((item) => signature(item, factor));
}

function extentOf(shapes: readonly Placed[]): number {
  let max = 0;
  for (const { shape } of shapes) for (const ring of flattenGeometry(shape).rings) for (const p of ring) max = Math.max(max, Math.abs(p.x), Math.abs(p.y));
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

/** 🔬️ Compares the elevation, front and plan geometry of two authored representations; `coarse` is the one with the larger denominator. */
export function compareRepresentations(coarse: AuthoredRepresentation, fine: AuthoredRepresentation): Similarity {
  const a = placed(coarse);
  const b = placed(fine);
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
