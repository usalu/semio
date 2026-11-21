/** ✂️ Verifies signed sphere splits against independent faceted solids from Manifold. */
import {expect,test} from "bun:test";
import loadManifold from "manifold-3d";
import fixture from "../🧫️fixtures/spherical-plane-split.json";

test("Manifold measures the two signed sphere pieces",async()=>{
  const wasm=await loadManifold();
  wasm.setup();
  const sphere=wasm.Manifold.sphere(fixture.radius,256);
  for (const row of fixture.cases) for (const sign of [-1,1]) {
    const normal=row.normal.map(axis=>axis*sign) as [number,number,number];
    const positive=row.span?sphere.trimByPlane([0,0,1],row.span[0]):null;
    const stock=positive&&row.span?positive.trimByPlane([0,0,-1],-row.span[1]):sphere;
    const halves=stock.splitByPlane(normal,row.offset*sign);
    for (const [index,half] of halves.entries()) {
      expect(half.status()).toBe("NoError");
      expect(half.genus()).toBe(0);
      expect(Math.abs(half.volume()-row.volumes[index^(sign<0?1:0)])).toBeLessThan(0.05);
      half.delete();
    }
    if(stock!==sphere)stock.delete();
    positive?.delete();
  }
  sphere.delete();
});
