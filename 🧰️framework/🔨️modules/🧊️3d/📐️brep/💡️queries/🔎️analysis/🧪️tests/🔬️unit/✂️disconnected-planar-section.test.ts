/** ✂️ Compares disconnected planar BRep sections with Manifold cross-sections. */
import { expect, test } from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "../🧫️fixtures/disconnected-planar-section.json";

const wasm = await loadManifold();
wasm.setup();

type Point = [number, number];
function signedArea(ring: Point[]) {
  return ring.reduce((sum, [x, y], index) => {
    const [nextX, nextY] = ring[(index + 1) % ring.length];
    return sum + x * nextY - y * nextX;
  }, 0) / 2;
}

for (const row of fixture.cases) {
  test(row.name, () => {
    const profile = new wasm.CrossSection([fixture.profile as Point[]]);
    const stock = profile.extrude(fixture.height);
    const cavities = row.voids.map(({ origin, size }) => {
      const box = wasm.Manifold.cube(size as [number, number, number]);
      const located = box.translate(origin as [number, number, number]);
      box.delete();
      return located;
    });
    const material = wasm.Manifold.difference([stock, ...cavities]);
    const turned = material.rotate([90, 0, 0]);
    const slice = turned.slice(fixture.cut);
    const faces = slice.decompose();
    try {
      const measured = faces.map(face => ({
        area: face.area(),
        rings: face.toPolygons().map(signedArea).sort((a, b) => b - a),
      })).sort((a, b) => a.area - b.area);
      expect(measured).toEqual(row.faces);
    } finally {
      for (const shape of [profile, stock, ...cavities, material, turned, slice, ...faces]) shape.delete();
    }
  });
}
