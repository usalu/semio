import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { cameraId, frames, streamId } from "../🟦️.ts";
const here = dirname(fileURLToPath(import.meta.url));
const assets = join(here, "../🖼️assets");
describe("synthetic-orbit", () => {
  it("ships primary asset", () => {
    expect(readFileSync(join(assets, "🗣️.dsl.semio"), "utf8").length).toBeGreaterThan(8);
  });
  it("ships one committed PNG per declared frame", () => {
    for (const frame of frames) expect(readFileSync(join(assets, frame.file)).subarray(0, 8)).toEqual(Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]));
  });
  it("agrees with the ground truth on camera, stream and frame table", () => {
    const truth = JSON.parse(readFileSync(join(assets, "🔮️ground-truth.json"), "utf8"));
    expect(truth.streamId).toBe(streamId);
    expect(truth.cameraId).toBe(cameraId);
    expect(truth.frames.map((frame: { assetId: string }) => frame.assetId)).toEqual(frames.map((frame) => frame.assetId));
    expect(truth.intrinsics.fx).toBeCloseTo(0.85 * truth.image.width, 9);
    expect(truth.extrinsics).toHaveLength(frames.length);
    expect(truth.pointsWorldM.length).toBeGreaterThanOrEqual(100);
  });
});
