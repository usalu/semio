import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { BoxGeometry, Vector3 } from "three";

test("original Session cold box law agrees with independent geometry", () => {
  const base = join(import.meta.dir, "../../🧫️fixtures/🧊️validation");
  const fixture = JSON.parse(readFileSync(join(base, "🔣️.json"), "utf8"));
  const geometry = new BoxGeometry(fixture.box.width, fixture.box.height, fixture.box.depth);
  const positions = geometry.getAttribute("position"), indices = geometry.getIndex()!;
  let area = 0, volume = 0;
  for (let i = 0; i < indices.count; i += 3) {
    const [a, b, c] = [0, 1, 2].map(offset => new Vector3().fromBufferAttribute(positions, indices.getX(i + offset)));
    area += b.clone().sub(a).cross(c.clone().sub(a)).length()/2;
    volume += a.dot(b.clone().cross(c))/6;
  }
  expect(indices.count/3).toBe(fixture.expected.triangles);
  expect(area).toBeCloseTo(fixture.expected.area, 10);
  expect(volume).toBeCloseTo(fixture.expected.volume, 10);
  geometry.dispose();
  console.log(`[DEBUG] independent Session box triangles=${indices.count/3} area=${area} volume=${volume}`);
});
