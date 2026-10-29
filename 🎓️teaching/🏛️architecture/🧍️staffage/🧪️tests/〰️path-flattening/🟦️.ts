/** 〰️ SVG path flattening against analytic curves from three.js.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ../../🧫️fixtures/〰️path-flattening/🔣️.json — the vectors
 */
import { readFileSync } from "node:fs";
import { CubicBezierCurve, EllipseCurve, QuadraticBezierCurve, Vector2 } from "three";
import type { Curve } from "three";
import { describe, expect, it } from "vitest";
import { FLATTEN_TOLERANCE_M, flattenPath } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

type Analytic = { type: string; points?: number[][]; cx?: number; cy?: number; rx?: number; ry?: number; rotationDeg?: number; fromDeg?: number; toDeg?: number };
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/〰️path-flattening/🔣️.json", import.meta.url), "utf8"));

function curveOf(a: Analytic): Curve<Vector2> {
  const v = (p: number[]) => new Vector2(p[0], p[1]);
  if (a.type === "cubic") return new CubicBezierCurve(v(a.points![0]!), v(a.points![1]!), v(a.points![2]!), v(a.points![3]!));
  if (a.type === "quadratic") return new QuadraticBezierCurve(v(a.points![0]!), v(a.points![1]!), v(a.points![2]!));
  const rad = (d: number) => (d * Math.PI) / 180;
  return new EllipseCurve(a.cx!, a.cy!, a.rx!, a.ry!, rad(a.fromDeg!), rad(a.toDeg!), a.toDeg! < a.fromDeg!, rad(a.rotationDeg!));
}

function distanceToPolyline(p: { x: number; y: number }, line: readonly { x: number; y: number }[]): number {
  let best = Infinity;
  for (let i = 1; i < line.length; i++) {
    const a = line[i - 1]!, b = line[i]!;
    const dx = b.x - a.x, dy = b.y - a.y;
    const len2 = dx * dx + dy * dy;
    const t = len2 === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / len2));
    best = Math.min(best, Math.hypot(p.x - (a.x + t * dx), p.y - (a.y + t * dy)));
  }
  return best;
}

describe("〰️ path flattening", () => {
  for (const c of fixture.curves) {
    it(`flattens "${c.name}" within the tolerance, both ways`, () => {
      const flat = flattenPath(c.d);
      const ring = flat.rings[0]!;
      const references = [curveOf(c.analytic), ...(c.second ? [curveOf(c.second)] : [])];
      for (const curve of references) {
        for (let i = 0; i <= 2000; i++) {
          const q = curve.getPoint(i / 2000);
          expect(distanceToPolyline({ x: q.x, y: q.y }, ring)).toBeLessThanOrEqual(fixture.toleranceM);
        }
      }
      const dense = references.map((curve) => Array.from({ length: 8001 }, (_, i) => curve.getPoint(i / 8000)));
      for (const p of ring) expect(Math.min(...dense.map((line) => distanceToPolyline(p, line)))).toBeLessThanOrEqual(1e-6);
      expect(ring.length).toBeGreaterThan(3);
    });
  }

  it("keeps the 0.5 mm constant in sync with the fixture", () => {
    expect(FLATTEN_TOLERANCE_M).toBe(fixture.toleranceM);
  });

  for (const c of fixture.polylines) {
    it(`keeps exact vertices of "${c.name}"`, () => {
      const flat = flattenPath(c.d);
      expect(flat.rings.map((ring) => ring.map((p) => [p.x, p.y]))).toEqual(c.rings);
      expect(flat.closed).toBe(c.closed);
    });
  }

  for (const d of fixture.invalid) it(`rejects "${d}"`, () => expect(() => flattenPath(d)).toThrow(/invalid path data/));
});
