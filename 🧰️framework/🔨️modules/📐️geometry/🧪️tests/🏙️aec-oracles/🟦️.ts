/** 🧫️ Third-party (`three`) reproduction of the language-agnostic geometry fixtures in `🧫️fixtures/*` — arcs, loops, triangulation and extrusions. */
import { expect, test } from "vitest";
import { BufferAttribute, EllipseCurve, ExtrudeGeometry, Path, Shape, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import bulge from "../../🧫️fixtures/🌙️bulge/🔣️.json";
import loops from "../../🧫️fixtures/➰️loops/🔣️.json";
import triangulation from "../../🧫️fixtures/🔺️triangulation/🔣️.json";
import mesh from "../../🧫️fixtures/🕸️mesh/🔣️.json";

type Seg = { start: number[]; end: number[]; bulge: number };

const quarterTurns = (b: number) => 4 * Math.atan(b);

function curveOf(seg: Seg, center: number[], radius: number): EllipseCurve {
  const sweep = quarterTurns(seg.bulge);
  const a0 = Math.atan2(seg.start[1]! - center[1]!, seg.start[0]! - center[0]!);
  return new EllipseCurve(center[0]!, center[1]!, radius, radius, a0, a0 + sweep, sweep < 0, 0);
}

for (const arc of bulge.arcs.filter((a) => a.expected.center !== null)) {
  test(`three EllipseCurve reproduces arc: ${arc.name}`, () => {
    const e = arc.expected;
    const curve = curveOf(arc as Seg, e.center as number[], e.radius as number);
    expect(Math.abs(curve.getLength() - e.length)).toBeLessThan(5e-4 * e.length);
    const start = curve.getPoint(0);
    const end = curve.getPoint(1);
    expect(start.distanceTo(new Vector2(arc.start[0], arc.start[1]))).toBeLessThan(1e-9);
    expect(end.distanceTo(new Vector2(arc.end[0], arc.end[1]))).toBeLessThan(1e-9);
    expect(curve.getPoint(0.5).distanceTo(new Vector2(e.mid[0], e.mid[1]))).toBeLessThan(1e-9);
    const points = curve.getPoints(4000);
    const area = Math.abs(ShapeUtils.area(points));
    expect(Math.abs(area - Math.abs(e.segment_area))).toBeLessThan(1e-5 * Math.max(1, Math.abs(e.segment_area)) + 1e-5);
    if ("bounds" in e && e.bounds) {
      const xs = points.map((p) => p.x);
      const ys = points.map((p) => p.y);
      const [x0, y0, x1, y1] = e.bounds as number[];
      expect(Math.min(...xs)).toBeCloseTo(x0!, 5);
      expect(Math.min(...ys)).toBeCloseTo(y0!, 5);
      expect(Math.max(...xs)).toBeCloseTo(x1!, 5);
      expect(Math.max(...ys)).toBeCloseTo(y1!, 5);
    }
  });
}

function pathOf(vertices: number[][]): Path {
  const path = new Path();
  path.moveTo(vertices[0]![0]!, vertices[0]![1]!);
  vertices.forEach((v, i) => {
    const next = vertices[(i + 1) % vertices.length]!;
    if (v[2] === 0) {
      path.lineTo(next[0]!, next[1]!);
      return;
    }
    const seg: Seg = { start: [v[0]!, v[1]!], end: [next[0]!, next[1]!], bulge: v[2]! };
    const sweep = quarterTurns(seg.bulge);
    const chord = Math.hypot(seg.end[0]! - seg.start[0]!, seg.end[1]! - seg.start[1]!);
    const radius = chord / (2 * Math.sin(Math.abs(sweep) / 2));
    const dir = new Vector2(seg.end[0]! - seg.start[0]!, seg.end[1]! - seg.start[1]!).divideScalar(chord);
    const offset = (chord / 2) * (1 - seg.bulge ** 2) / (2 * seg.bulge);
    const mid = new Vector2((seg.start[0]! + seg.end[0]!) / 2, (seg.start[1]! + seg.end[1]!) / 2);
    const center = mid.add(new Vector2(-dir.y, dir.x).multiplyScalar(offset));
    const a0 = Math.atan2(seg.start[1]! - center.y, seg.start[0]! - center.x);
    path.absarc(center.x, center.y, radius, a0, a0 + sweep, sweep < 0);
  });
  return path;
}

for (const loop of loops.loops) {
  test(`three Path reproduces loop area and perimeter: ${loop.name}`, () => {
    const path = pathOf(loop.vertices);
    const points = path.getPoints(3000);
    expect(Math.abs(Math.abs(ShapeUtils.area(points)) - loop.expected.area)).toBeLessThan(2e-3 * Math.max(1, loop.expected.area));
    expect(Math.abs(path.getLength() - loop.expected.perimeter)).toBeLessThan(2e-3 * loop.expected.perimeter);
  });
}

const ring = (points: number[][]) => points.map((p) => new Vector2(p[0], p[1]));

for (const [index, c] of triangulation.entries()) {
  test(`three earcut reproduces triangulation area and count: ${c.name}`, () => {
    const contour = ring(c.outer);
    const holes = c.holes.map(ring);
    const faces = ShapeUtils.triangulateShape(contour, holes);
    const all = [...contour, ...holes.flat()];
    const area = faces.reduce((sum, f) => sum + Math.abs(ShapeUtils.area([all[f[0]!]!, all[f[1]!]!, all[f[2]!]!])), 0);
    expect(area).toBeCloseTo(c.area, 9);
    expect(faces.length).toBe(c.triangles);
    expect(index).toBeGreaterThanOrEqual(0);
  });
}

function meshStats(geometry: ExtrudeGeometry): { volume: number; area: number } {
  const g = geometry.index ? geometry.toNonIndexed() : geometry;
  const position = g.getAttribute("position") as BufferAttribute;
  let volume = 0;
  let area = 0;
  for (let i = 0; i < position.count; i += 3) {
    const a = new Vector3().fromBufferAttribute(position, i);
    const b = new Vector3().fromBufferAttribute(position, i + 1);
    const c = new Vector3().fromBufferAttribute(position, i + 2);
    volume += a.dot(new Vector3().crossVectors(b, c)) / 6;
    area += new Triangle(a, b, c).getArea();
  }
  return { volume: Math.abs(volume), area };
}

for (const c of mesh.extrusions.filter((e) => e.bottom[0] === 0 && e.bottom[1] === 0 && e.top[0] === 0 && e.top[1] === 0)) {
  test(`three ExtrudeGeometry reproduces extrusion volume and area: ${c.name}`, () => {
    const shape = new Shape(ring(c.outer));
    for (const hole of c.holes) shape.holes.push(new Path(ring(hole)));
    const geometry = new ExtrudeGeometry(shape, { depth: c.top[2]! - c.bottom[2]!, bevelEnabled: false, steps: 1 });
    const stats = meshStats(geometry);
    expect(Math.abs(stats.volume - c.expected.volume)).toBeLessThan(1e-6 * Math.max(1, c.expected.volume));
    expect(Math.abs(stats.area - c.expected.area)).toBeLessThan(1e-6 * Math.max(1, c.expected.area));
  });
}
