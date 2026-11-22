/** 🕳️ Independently measures sections through closed planar bore stock. */
import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixtures from "../🧫️fixtures/through-hole-section.json";

test("through-hole sections cross faces with multiple boundary loops", async () => {
  const wasm = await loadManifold();
  wasm.setup();
  for (const fixture of fixtures) {
    const outside = new wasm.CrossSection(fixture.outside as [number, number][]);
    const bores = fixture.bores.map(points => new wasm.CrossSection(points as [number, number][]));
    const opening = wasm.CrossSection.union(bores);
    const profile = outside.subtract(opening);
    const stock = profile.extrude(fixture.height);
    try {
      for (const cut of fixture.cuts) {
        for (const sign of [-1, 1]) {
          const rotation: [number, number, number] = cut.normal[0] ? [0, -sign * 90, 0]
            : cut.normal[1] ? [sign * 90, 0, 0] : [sign === 1 ? 0 : 180, 0, 0];
          const turn = stock.rotate(rotation);
          const height = cut.origin.reduce((offset, value, axis) => offset + value * cut.normal[axis], 0);
          const section = turn.slice(height * sign);
          const components = section.decompose();
          try {
            const rings = components.map(face => face.toPolygons().map(ring => ring.reduce((sum, [x, y], i) => {
              const [nextX, nextY] = ring[(i + 1) % ring.length];
              return sum + x * nextY - y * nextX;
            }, 0) / 2).sort((a, b) => b - a)).sort((a, b) => b[0] - a[0]);
            expect(rings).toEqual(cut.rings);
          } finally {
            for (const shape of [turn, section, ...components]) shape.delete();
          }
        }
      }
    } finally {
      for (const shape of [stock, profile, outside, opening, ...bores]) shape.delete();
    }
  }
});
