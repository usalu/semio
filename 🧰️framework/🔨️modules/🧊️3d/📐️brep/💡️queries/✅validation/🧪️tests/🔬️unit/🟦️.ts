import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BufferGeometry, Float32BufferAttribute, Triangle, Vector3 } from "three";

test("cold validation neutral tetrahedron agrees with independent topology and volume", () => {
  const base = join(import.meta.dir, "../🧫️fixtures/🧊️cold-planning");
  const fixture = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8"));
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new Float32BufferAttribute(fixture.vertices.flat(), 3));
  geometry.setIndex(fixture.faces.flat());
  const points = geometry.getAttribute("position");
  const indices = geometry.getIndex()!;
  const uses = new Map<string, { count: number; direction: number }>();
  let volume = 0;
  let samples = 0, maximumDeviation = 0;
  for (let i = 0; i < indices.count; i += 3) {
    const ids = [indices.getX(i), indices.getX(i + 1), indices.getX(i + 2)];
    const [a, b, c] = ids.map(id => new Vector3().fromBufferAttribute(points, id));
    volume += a.dot(b.clone().cross(c)) / 6;
    for (let j = 0; j < 3; j++) {
      const u = ids[j], v = ids[(j + 1) % 3];
      const key = `${Math.min(u, v)}:${Math.max(u, v)}`;
      const use = uses.get(key) ?? { count: 0, direction: 0 };
      use.count++;
      use.direction += u < v ? 1 : -1;
      uses.set(key, use);
      const corners = [a, b, c];
      const triangle = new Triangle(a, b, c);
      for (let sample = 0; sample <= fixture.budget.sameParameterBaseSamples; sample++) {
        const point = corners[j].clone().lerp(corners[(j + 1) % 3], sample / fixture.budget.sameParameterBaseSamples);
        maximumDeviation = Math.max(maximumDeviation, point.distanceTo(triangle.closestPointToPoint(point, new Vector3())));
        samples++;
      }
    }
  }
  expect(uses.size).toBe(fixture.expected.edges);
  expect(indices.count / 3).toBe(fixture.expected.faces);
  expect([...uses.values()].every(use => use.count === 2 && use.direction === 0)).toBe(true);
  expect(volume).toBeCloseTo(fixture.expected.volume, 10);
  expect(samples).toBe(fixture.expected.faces * 3 * (fixture.budget.sameParameterBaseSamples + 1));
  expect(maximumDeviation).toBeLessThan(1e-12);
  const probe = fixture.sameParameterProbe;
  const start = new Vector3(...fixture.vertices[probe.edge[0]]);
  const end = new Vector3(...fixture.vertices[probe.edge[1]]);
  const deviations = probe.samples.map((s: number) => ({ s, deviation: start.distanceTo(start.clone().lerp(end, s)) }));
  const worst = deviations.reduce((a: { s: number; deviation: number }, b: { s: number; deviation: number }) => b.deviation >= a.deviation ? b : a);
  expect(worst.deviation).toBeCloseTo(probe.maximumDeviation, 12);
  expect(worst.s).toBe(probe.worstParameter);
  expect(samples - (fixture.budget.sameParameterBaseSamples + 1) + deviations.length).toBe(probe.totalBodySamples);
  geometry.dispose();
  console.log(`[DEBUG] independent cold validation edges=${uses.size} faces=${indices.count / 3} volume=${volume} samples=${samples} maximumDeviation=${maximumDeviation} adaptiveSamples=${deviations.length} adaptiveDeviation=${worst.deviation} adaptiveWorst=${worst.s}`);
});
