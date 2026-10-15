/** 🌀️ Generation3d's interchange surface is checked against the stdio format artifacts it names, so the law lives where plugins may meet. */
import { expect, test } from "bun:test";
import { existsSync } from "node:fs";
import { join } from "node:path";
import {
  testGeneration3dDocumentCapacity,
  testGeneration3dDocumentIoSurface,
  testGeneration3dGltfSceneImport,
  testGeneration3dIoInputContracts,
  testGeneration3dMeshSurface,
  testGeneration3dPolygonImport,
  verifyPreparedGltfExportOracle,
} from "../🗿️artifact-surface/🟦️.ts";

test("the document io surface agrees with every owning stdio format definition", () => {
  testGeneration3dDocumentIoSurface();
});
test("document capacity refusals are exact", () => {
  expect(testGeneration3dDocumentCapacity()).toBeGreaterThan(0);
});
test("io input contracts hold", () => {
  expect(testGeneration3dIoInputContracts()).toBeGreaterThan(0);
});
test("the mesh surface uses the neutral polygon contract", () => {
  expect(testGeneration3dMeshSurface()).toBeGreaterThan(0);
});
test("glTF scene import places nodes", () => {
  testGeneration3dGltfSceneImport();
});
test("polygon import keeps OBJ and PLY topology", () => {
  expect(testGeneration3dPolygonImport()).toBeGreaterThan(0);
});
test("the prepared glTF export written by the native laws passes the independent oracle", async () => {
  const directory = process.env.SEMIO_TEST_ARTIFACT_DIR;
  const path = directory === undefined ? undefined : join(directory, "prepared-rich.gltf");
  if (path === undefined || !existsSync(path)) return;
  await verifyPreparedGltfExportOracle(path);
});
