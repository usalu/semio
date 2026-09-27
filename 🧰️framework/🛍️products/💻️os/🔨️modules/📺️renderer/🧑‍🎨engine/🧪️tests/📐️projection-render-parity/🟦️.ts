import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "vitest";
import { worldCurvilinearUnproject, worldProjectionGoalMatrix, worldProjectionOrientationLook, type WorldProjectionSpec } from "@semio-tech/infinite-world-r3f";
import { testViewport3dProjectionValues } from "../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧪️tests/📐️projection/🟦️.ts";

type MatrixRow = { readonly name: string; readonly spec: WorldProjectionSpec; readonly expectedElements: Readonly<Record<string, number>> };
type OrientationRow = { readonly name: string; readonly spec: WorldProjectionSpec; readonly expectedDirection: readonly number[]; readonly expectedUp: readonly number[] };
type CurvilinearRow = { readonly name: string; readonly spec: WorldProjectionSpec; readonly aspect: number; readonly visibleNdc: readonly [number, number]; readonly expectedCaptureNdc: readonly number[] };
type Fixture = { readonly renderParity: { readonly viewport: { readonly width: number; readonly height: number; readonly near: number; readonly far: number; readonly zoom: number }; readonly matrixCases: readonly MatrixRow[]; readonly orientationCases: readonly OrientationRow[]; readonly curvilinearCases: readonly CurvilinearRow[] } };

const fixture = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../🔨️modules/🖱️ui/🪟️viewport/🧫️fixtures/📐️projection/🔣️.json"), "utf8")) as Fixture;
const close = (actual: number, expected: number, label: string) => assert.ok(Math.abs(actual - expected) <= 1e-12, `${label}: ${actual} != ${expected}`);

describe("projection render parity", () => {
  it("admits and derives the shared viewport projection schema through the neutral oracle", () => {
    testViewport3dProjectionValues();
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
    for (const row of fixture.renderParity.curvilinearCases) {
      if (row.spec.mode.kind !== "curvilinear") throw new TypeError(row.name);
      const capture = worldCurvilinearUnproject(row.visibleNdc, row.spec.mode, row.aspect);
      capture.forEach((value, index) => close(value, row.expectedCaptureNdc[index]!, `${row.name}[${index}]`));
    }
  });
});
