/** 🧊️ The extent of the 3D model: at rest against the physical `dimensions` of the asset, and for every named pose against the elevation, front and plan extents of the same evaluated state (decision D18: the larger of 2 % and 15 mm).
 *
 * THE REFERENCES are three.js (`Box3.expandByObject(object, true)`, the precise skinned extent of the GLB loaded by `GLTFLoader`), the 2D shapes of `evaluate`, and the hand-computed extent of the sample figure's rest frames in `🧫️fixtures/🧊️volume-schema`.
 *
 * The ahead and up extents are compared with the elevation (the view that shows both), the lateral extent with the front and the plan; the sample figure's authored front shapes stay on the ground line in a stride, so their vertical extent is not a reference.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { describe, expect, it } from "vitest";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";
import { writeGlb } from "../../🔨️modules/📤️export/🧊️glb/🟦️.ts";
import { modelOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { extentTolerance, loadGlb, restExtent, sampleFigure, sceneExtent, shapeExtent } from "../🧰️support/🧊️.ts";

const SCALE = 50;
const within = (got: number, want: number, what: string): void => void expect(Math.abs(got - want), `${what}: ${got} vs ${want}`).toBeLessThanOrEqual(extentTolerance(want));

describe("🧊️ the extent of the 3D model", () => {
  const asset = sampleFigure();

  it("has at rest the extent of the physical dimensions: ahead the width, up the height, lateral the depth", async () => {
    const e = await restExtent(asset, await modelOf(asset, { scale: SCALE, views: ["light-solid"], poses: [], clips: [] }));
    within(e.max[0] - e.min[0], asset.dimensions.widthM, "widthM");
    within(e.max[1] - e.min[1], asset.dimensions.heightM, "heightM");
    within(e.max[2] - e.min[2], asset.dimensions.depthM, "depthM");
  });

  it.each(asset.poses.filter((p) => p.key !== true).map((p) => p.id))("has in pose %s the extent of the 2D elevation, front and plan shapes of the same state", async (pose) => {
    const { scene } = await loadGlb(await writeGlb(asset, { pose, scale: SCALE }));
    const e = sceneExtent(scene);
    const state = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: SCALE } });
    const elevation = shapeExtent(state, "elevation");
    const front = shapeExtent(state, "front");
    const plan = shapeExtent(state, "plan");
    within(e.max[0] - e.min[0], elevation.horizontal[1] - elevation.horizontal[0], "elevation x extent");
    within(e.min[0], elevation.horizontal[0], "elevation x min");
    within(e.max[0], elevation.horizontal[1], "elevation x max");
    within(e.max[1], elevation.vertical[1], "elevation y max");
    within(e.max[1] - e.min[1], elevation.vertical[1] - elevation.vertical[0], "elevation y extent");
    within(e.max[2] - e.min[2], front.horizontal[1] - front.horizontal[0], "front z extent");
    within(e.max[0] - e.min[0], plan.horizontal[1] - plan.horizontal[0], "plan x extent");
    within(e.max[2] - e.min[2], plan.vertical[1] - plan.vertical[0], "plan z extent");
  });

  it("keeps the hands and feet of a pose on the ground and the solids above it: the lowest point of a standing pose is the ground", async () => {
    const { scene } = await loadGlb(await writeGlb(asset, { pose: "stand", scale: SCALE }));
    const e = sceneExtent(scene);
    expect(Math.abs(e.min[1])).toBeLessThanOrEqual(0.015);
  });
});
