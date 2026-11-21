import { strict as assert } from "node:assert";
import { PerspectiveCamera, Plane, Raycaster, Vector2, Vector3 } from "three";
import fixture from "./🖱️engagement-pointer.json";

const [left, top, width, height] = fixture.viewport;
const [clipLeft, clipTop, clipWidth, clipHeight] = fixture.pickBounds;
const plane = new Plane(new Vector3(0, 0, 1), 0);
const ray = new Raycaster();
for (const sample of fixture.cases) {
  const [x, y] = sample.point;
  const inside = x >= clipLeft && x <= clipLeft + clipWidth && y >= clipTop && y <= clipTop + clipHeight;
  const camera = new PerspectiveCamera(45, width / height, 0.1, 1000);
  camera.up.set(0, 0, 1);
  camera.position.fromArray(sample.camera);
  camera.lookAt(new Vector3().fromArray(sample.target));
  camera.updateMatrixWorld(true);
  ray.setFromCamera(new Vector2(2 * (x - left) / width - 1, 1 - 2 * (y - top) / height), camera);
  const hit = inside && sample.active && sample.declared && Math.abs(ray.ray.direction.z) > 1e-8
    ? ray.ray.intersectPlane(plane, new Vector3())
    : null;
  if (sample.position === null) {
    assert.equal(hit, null, sample.id);
    continue;
  }
  assert.ok(hit && hit.distanceTo(new Vector3().fromArray(sample.position)) < 1e-6, sample.id);
}
console.log("Verified engagement pointer fixture with Three.js");
