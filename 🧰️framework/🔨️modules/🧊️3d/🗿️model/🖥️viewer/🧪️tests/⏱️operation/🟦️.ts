/** ⏱️ The cancellation law of the viewer: `sceneOf`, `SoftwareBackend.render` and `pick` poll inside their hot loops and stop at the first positive answer with a `CancelledError`; a cancelled render leaves the previous picture untouched. `sceneOfAsync` yields inside one primitive. */
import { describe, expect, it } from "vitest";
import { CancelledError } from "../../../⏱️operation/🟦️.ts";
import { caseOf, denseResolved, expectCancelledAt, expectLaw, vector } from "../../../🧪️tests/🧰️support/⏱️operation/🟦️.ts";
import { cameraOf, fitCamera } from "../../📷️camera/🟦️.ts";
import { boundsOf, createFrame, frameOf } from "../../🎞️frame/🟦️.ts";
import { pickAt } from "../../🎯️picking/🟦️.ts";
import { sceneOf, sceneOfAsync } from "../../🧱️scene/🟦️.ts";
import { SoftwareBackend } from "../../🖼️software/🟦️.ts";

const prepared = async () => {
  const scene = sceneOf(await denseResolved());
  const { raster } = vector();
  const probeFrame = frameOf(scene, { camera: cameraOf(raster.preset), aspect: 1 }, createFrame(scene));
  const camera = fitCamera(cameraOf(raster.preset), boundsOf(scene, probeFrame), 1);
  const frame = frameOf(scene, { camera, aspect: 1 }, createFrame(scene));
  return { scene, camera, frame, raster };
};

describe("viewer cancellation", () => {
  it("sceneOf cancels inside the vertex loops and sceneOfAsync rejects", async () => {
    const resolved = await denseResolved();
    const scene = await expectLaw((operation) => sceneOf(resolved, operation), caseOf("sceneOf"));
    expect(scene.vertices.length).toBeGreaterThan(0);
    await expect(sceneOfAsync(resolved, { report: () => undefined, cancelled: () => true })).rejects.toBeInstanceOf(CancelledError);
  });
  it("SoftwareBackend.render cancels inside the pixel and triangle loops and keeps the previous picture", async () => {
    const { scene, frame, raster } = await prepared();
    const backend = new SoftwareBackend(raster.width, raster.height);
    backend.load(scene);
    const style = { background: [0.1, 0.2, 0.3, 1] as const };
    backend.render(frame, style);
    const picture = backend.pixels.slice();
    const depth = backend.depth.slice();
    await expectLaw((operation) => backend.render(frame, { background: [1, 1, 1, 1] }, operation), caseOf("SoftwareBackend.render"));
    backend.render(frame, style);
    expect(backend.pixels).toEqual(picture);
    await expectCancelledAt((operation) => backend.render(frame, { background: [0, 0, 0, 1] }, operation), 5);
    expect(backend.pixels).toEqual(picture);
    expect(backend.depth).toEqual(depth);
  });
  it("pick cancels inside the triangle loop and returns no hit", async () => {
    const { scene, camera, frame } = await prepared();
    const hit = await expectLaw((operation) => pickAt(scene, frame, camera, 1, 0, 0, operation), caseOf("pick"));
    expect(hit).not.toBeNull();
  });
});
