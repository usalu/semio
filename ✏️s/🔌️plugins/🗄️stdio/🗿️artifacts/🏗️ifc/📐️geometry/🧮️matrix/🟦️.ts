import type {Point} from "../🟦️.ts";
export type Matrix=number[][];
export const add = (a: Point, b: Point): Point => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
export const subtract = (a: Point, b: Point): Point => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
export const scale = (a: Point, s: number): Point => [a[0] * s, a[1] * s, a[2] * s];
export const dot = (a: Point, b: Point): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
export const cross = (a: Point, b: Point): Point => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
export const normalize = (a: Point): Point => {
  const largest = Math.max(...a.map(Math.abs));
  if (!Number.isFinite(largest) || largest === 0) throw new Error("invalid-direction");
  const unit = a.map(value => value / largest) as Point, length = Math.hypot(...unit);
  return unit.map(value => value / length) as Point;
};
export const identity = (): Matrix => [[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]];
export const matrix = (o: Point, x: Point, y: Point, z: Point): Matrix => [[x[0], y[0], z[0], o[0]], [x[1], y[1], z[1], o[1]], [x[2], y[2], z[2], o[2]], [0, 0, 0, 1]];
export const multiply = (a: Matrix, b: Matrix): Matrix => a.map(row => b[0].map((_, j) => row.reduce((value, x, k) => value + x * b[k][j], 0)));
export const transformPoint = (m: Matrix, p: Point): Point => [0, 1, 2].map(i => m[i][0] * p[0] + m[i][1] * p[1] + m[i][2] * p[2] + m[i][3]) as Point;
export const determinant = (m: Matrix): number => dot([m[0][0], m[1][0], m[2][0]], cross([m[0][1], m[1][1], m[2][1]], [m[0][2], m[1][2], m[2][2]]));
