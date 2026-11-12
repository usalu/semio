/** 🧱️ Welding mesh builder: vertices with equal position and normal share an index (so smooth surfaces are indexed watertight and creases stay hard), triangles of zero area are dropped.
 * @see ../../🧬️schema/🟦️.ts */
import { cross, sub, length, type V3 } from "../../🧮️math/🟦️.ts";
import { Pace } from "../../⏱️operation/🟦️.ts";
import * as portable from "../../🧮️math/🎯️portable/🟦️.ts";

const POSITION_QUANTUM = 1e9;
const NORMAL_QUANTUM = 1e5;

/** 📐️ Cosine of the crease angle (35 degrees): surfaces turning by more than that keep hard normals. */
export const CREASE_COS = portable.cos((35 * Math.PI) / 180);

export type Geometry = { positions: number[]; normals: number[]; indices: number[] };

/** 🧰️ Accumulates one solid's triangles in its local frame; every vertex and triangle counts as work of the optional `pace`, so tessellation of one huge solid stays cancellable. */
export class MeshBuilder {
  readonly positions: number[] = [];
  readonly normals: number[] = [];
  readonly indices: number[] = [];
  private readonly lookup = new Map<string, number>();

  constructor(private readonly pace: Pace = new Pace()) {}

  vertex(p: readonly number[], n: readonly number[]): number {
    this.pace.tick();
    const key = `${Math.round(p[0]! * POSITION_QUANTUM)},${Math.round(p[1]! * POSITION_QUANTUM)},${Math.round(p[2]! * POSITION_QUANTUM)}|${Math.round(n[0]! * NORMAL_QUANTUM)},${Math.round(n[1]! * NORMAL_QUANTUM)},${Math.round(n[2]! * NORMAL_QUANTUM)}`;
    const found = this.lookup.get(key);
    if (found !== undefined) return found;
    const index = this.positions.length / 3;
    this.positions.push(p[0]!, p[1]!, p[2]!);
    this.normals.push(n[0]!, n[1]!, n[2]!);
    this.lookup.set(key, index);
    return index;
  }

  triangle(a: number, b: number, c: number): void {
    this.pace.tick();
    if (a === b || b === c || a === c) return;
    const pa = this.at(a), pb = this.at(b), pc = this.at(c);
    if (length(cross(sub(pb, pa), sub(pc, pa))) < 1e-18) return;
    this.indices.push(a, b, c);
  }

  at(index: number): V3 {
    return [this.positions[index * 3]!, this.positions[index * 3 + 1]!, this.positions[index * 3 + 2]!];
  }

  build(): Geometry {
    return { positions: this.positions, normals: this.normals, indices: this.indices };
  }
}

/** 🕸️ A grid of rows by columns around a closed or open sweep; `connect[i]` says whether row i joins row i+1. Triangles are `(i,j) (i,j+1) (i+1,j)` which face outward for rows advancing along `a` and columns counter-clockwise about `a`. */
export function gridSurface(builder: MeshBuilder, rows: number, columns: number, closed: boolean, point: (row: number, column: number) => { p: V3; n: V3 }, connect: (row: number) => boolean = () => true): void {
  const stride = closed ? columns : columns + 1;
  const ids: number[] = [];
  for (let i = 0; i < rows; i++) for (let j = 0; j < stride; j++) {
    const { p, n } = point(i, j);
    ids.push(builder.vertex(p, n));
  }
  const id = (i: number, j: number) => ids[i * stride + (closed ? j % columns : j)]!;
  for (let i = 0; i + 1 < rows; i++) {
    if (!connect(i)) continue;
    for (let j = 0; j < columns; j++) {
      const a = id(i, j), b = id(i, j + 1), c = id(i + 1, j), d = id(i + 1, j + 1);
      builder.triangle(a, b, c);
      builder.triangle(b, d, c);
    }
  }
}
