import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import ManifoldModule from "manifold-3d";

test("coincident boundaries agree with independent Manifold occupancy and volumes", async () => {
  const root = new URL("../../🧫️fixtures/coincident-boundary/", import.meta.url), fixture = JSON.parse(readFileSync(new URL("🔣️.json", root), "utf8"));
  expect(new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("📐️schema.json", root), "utf8")))(fixture)).toBe(true);
  const module = await ManifoldModule();
  module.setup();
  for (const row of fixture.cases) {
    const a = module.Manifold.cube([1, 1, 1]), seed = module.Manifold.cube([1, 1, 1]), b = seed.translate([0, 0, row.aligned ? 0 : 1]);
    const result = row.operation === "union" ? a.add(b) : row.operation === "intersection" ? a.intersect(b) : a.subtract(b);
    try {
      const occupancy = row.leftInside.map((left: boolean, index: number) => row.operation === "union" ? left || row.rightInside[index] : row.operation === "intersection" ? left && row.rightInside[index] : left && !row.rightInside[index]);
      expect(occupancy[0] !== occupancy[1]).toBe(row.retainsBoundary);
      expect(result.volume()).toBe(row.volume);
      const mesh = result.getMesh();
      let boundary = false;
      for (let index = 0; index < mesh.triVerts.length; index += 3) if ([0, 1, 2].every(offset => mesh.vertProperties[mesh.triVerts[index + offset]! * mesh.numProp + 2] === 1)) boundary = true;
      expect(boundary).toBe(row.retainsBoundary);
      console.log(`[DEBUG] Independent coincident boundary ${row.operation} aligned=${row.aligned} retained=${boundary} volume=${result.volume()}`);
    } finally { result.delete(); b.delete(); seed.delete(); a.delete(); }
  }
}, 30000);


test("coincident bore refill has a closed shell and independent Manifold volume", async () => {
  const root = new URL("../../🧫️fixtures/coincident-seam/", import.meta.url), row = JSON.parse(readFileSync(new URL("🔣️.json", root), "utf8"));
  expect(new Ajv({ strict: true }).compile(JSON.parse(readFileSync(new URL("📐️schema.json", root), "utf8")))(row)).toBe(true);
  const module = await ManifoldModule();
  module.setup();
  const stock = module.Manifold.cube(row.stock), seed = module.Manifold.cylinder(row.height, row.radius, row.radius, row.circularSegments), bit = seed.translate(row.translation), bored = stock.subtract(bit), filled = bored.add(bit);
  try {
    expect(filled.status() === "NoError").toBe(row.closedShell);
    expect(Math.abs(filled.volume() - row.expectedVolume)).toBeLessThan(1e-5);
    expect(stock.volume()).toBe(1);
    console.log(`[DEBUG] Independent Manifold bore refill closed=${row.closedShell} volume=${filled.volume()} analytic=${row.expectedVolume} segments=${row.circularSegments}`);
  } finally { filled.delete(); bored.delete(); bit.delete(); seed.delete(); stock.delete(); }
}, 30000);
