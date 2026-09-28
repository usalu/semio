import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "vitest";
import { frameWorldProjectionPose, worldCurvilinearUnproject, worldProjectionGoalMatrix, worldProjectionOrientationLook, type WorldProjectionSpec } from "@semio-tech/infinite-world-r3f";
import { testViewport3dProjectionValues } from "../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧪️tests/📐️projection/🟦️.ts";
import { parseCameraState, world3dFitProjectionContent } from "../../🧱️elements/🌐️World3dHost/🟦️.tsx";

type MatrixRow = { readonly name: string; readonly spec: WorldProjectionSpec; readonly expectedElements: Readonly<Record<string, number>> };
type OrientationRow = { readonly name: string; readonly spec: WorldProjectionSpec; readonly expectedDirection: readonly number[]; readonly expectedUp: readonly number[] };
type CurvilinearRow = { readonly name: string; readonly spec: WorldProjectionSpec; readonly aspect: number; readonly visibleNdc: readonly [number, number]; readonly expectedCaptureNdc: readonly number[] };
type FrameRow = { readonly name: string; readonly spec: WorldProjectionSpec; readonly bounds: { readonly center: readonly [number, number, number]; readonly halfExtent: readonly [number, number, number] }; readonly viewport: { readonly width: number; readonly height: number }; readonly expected: { readonly position: readonly number[]; readonly target: readonly number[]; readonly up: readonly number[]; readonly zoom: number; readonly projection: string } };
type FrameLifecycleRow = { readonly name: string; readonly viewportOwned: boolean; readonly cameraNavigating: boolean; readonly hasProjectionSeed: boolean; readonly lockContentFrame: boolean; readonly projectionFrame: "content" | "preserveCamera"; readonly expectedMounted: boolean };
type Fixture = { readonly renderParity: { readonly viewport: { readonly width: number; readonly height: number; readonly near: number; readonly far: number; readonly zoom: number }; readonly matrixCases: readonly MatrixRow[]; readonly orientationCases: readonly OrientationRow[]; readonly frameCases: readonly FrameRow[]; readonly frameLifecycleCases: readonly FrameLifecycleRow[]; readonly curvilinearCases: readonly CurvilinearRow[] } };

const fixture = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧫️fixtures/📐️projection/🔣️.json"), "utf8")) as Fixture;
const close = (actual: number, expected: number, label: string) => assert.ok(Math.abs(actual - expected) <= 1e-12, `${label}: ${actual} != ${expected}`);

describe("projection render parity", () => {
  it("admits and derives the shared viewport projection schema through the neutral oracle", () => {
    testViewport3dProjectionValues();

    assert.equal(parseCameraState('{"position":[4,-4,3],"target":[0,0,0],"zoom":1}').projectionFrame, "content");
    assert.equal(parseCameraState('{"position":[4,-4,3],"target":[0,0,0],"zoom":1,"projectionFrame":"preserveCamera"}').projectionFrame, "preserveCamera");
  });

  it("matches the shared matrix, orientation, and curvilinear fixture through React's production authority", () => {
    const viewport = fixture.renderParity.viewport;
    for (const row of fixture.renderParity.matrixCases) {
      const matrix = worldProjectionGoalMatrix(row.spec, { zoom: viewport.zoom, viewport, near: viewport.near, far: viewport.far });
      for (const [index, expected] of Object.entries(row.expectedElements)) close(matrix.elements[Number(index)]!, expected, `${row.name}[${index}]`);
    }
    for (const row of fixture.renderParity.orientationCases) {
      const look = worldProjectionOrientationLook(row.spec);
      look.dir.forEach((value, index) => close(value, row.expectedDirection[index]!, `${row.name}.dir[${index}]`));
      look.up.forEach((value, index) => close(value, row.expectedUp[index]!, `${row.name}.up[${index}]`));
    }
    for (const row of fixture.renderParity.frameCases) {
      const framed = frameWorldProjectionPose(row.spec, row.bounds, { viewportWidth: row.viewport.width, viewportHeight: row.viewport.height });
      framed.position.forEach((value, index) => close(value, row.expected.position[index]!, `${row.name}.position[${index}]`));
      framed.target.forEach((value, index) => close(value, row.expected.target[index]!, `${row.name}.target[${index}]`));
      framed.up?.forEach((value, index) => close(value, row.expected.up[index]!, `${row.name}.up[${index}]`));
      close(framed.zoom, row.expected.zoom, `${row.name}.zoom`);
      assert.equal(framed.projection, row.expected.projection, `${row.name}.projection`);
    }
    for (const row of fixture.renderParity.frameLifecycleCases) {
      assert.equal(world3dFitProjectionContent(row.viewportOwned, row.cameraNavigating, row.hasProjectionSeed, row.lockContentFrame, row.projectionFrame), row.expectedMounted, row.name);
    }
    for (const row of fixture.renderParity.curvilinearCases) {
      if (row.spec.mode.kind !== "curvilinear") throw new TypeError(row.name);
      const capture = worldCurvilinearUnproject(row.visibleNdc, row.spec.mode, row.aspect);
      capture.forEach((value, index) => close(value, row.expectedCaptureNdc[index]!, `${row.name}[${index}]`));
    }
  });
});
