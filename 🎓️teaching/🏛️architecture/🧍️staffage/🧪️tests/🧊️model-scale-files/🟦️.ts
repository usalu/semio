/** 🧊️ The file set of the handcrafted four-scale sample figure through every oracle of the catalog guard: a GLB per pose with one glTF scene per scale and mode, an FBX per clip, scale and view, the multi-scene model JSON and the ZIP, read back by gltf-validator, three, glTF-Transform, assimp, Ajv and fflate.
 *
 * THE REFERENCES are the Khronos `gltf-validator`, three's `GLTFLoader` and `FBXLoader`, glTF-Transform's `NodeIO`, assimp (`assimpjs`), Ajv with the `semio.model/v1` JSON Schema and fflate; the figure is `🧫️fixtures/🧊️volume-scales`, so the test does not wait for the re-authoring of the catalog.
 *
 * @see ./🥒️.feature — the scenarios
 */
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { afterAll, describe, expect, it } from "vitest";
import { VIEWS } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { exportModels, plannedNames } from "../../🔨️modules/📤️export/📦️batch/🟦️.ts";
import { glbName } from "../../🔨️modules/📤️export/🗿️model/🟦️.ts";
import { MODES, authoredScales, sceneIdOf } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { auditFiles, auditGeometry, expectedScenes, glbFacts } from "../🧊️model-catalog/🔍️oracles/🟦️.ts";
import { scaledFigure } from "../🧰️support/🧊️volumes/🟦️.ts";

const base = scaledFigure();
const asset = { ...base, id: "scaled-figure", clips: base.clips.filter((c) => c.id === "walk") };
const made: string[] = [];
afterAll(() => made.forEach((d) => rmSync(d, { recursive: true, force: true })));

const catalogOf = (): string => {
  const root = mkdtempSync(join(tmpdir(), "staffage-scale-files-"));
  made.push(root);
  writeFileSync(join(root, "🔣️.json"), JSON.stringify({ version: "semio.staffage/v2", categories: [{ id: "test", emoji: "🧪", title: { en: "Test", de: "Test" }, description: { en: "Test", de: "Test" }, folder: "test" }], collections: [] }));
  mkdirSync(join(root, "test", asset.id), { recursive: true });
  writeFileSync(join(root, "test", asset.id, "🔣️.json"), JSON.stringify({ version: "semio.staffage/v2", asset }));
  return root;
};

describe("🧊️ the file set of the four-scale sample figure passes every oracle", () => {
  it("has valid, watertight, finite scenes whose triangles never grow towards coarser scales", { timeout: 300_000 }, async () => {
    const audit = await auditGeometry(asset);
    expect(audit.findings.map((f) => `${f.check} ${f.item}: ${f.message}`)).toEqual([]);
    expect(audit.facts.scenes.map((s) => s.scene)).toEqual(expectedScenes(asset));
    expect(audit.facts.scenes).toHaveLength(authoredScales(asset.volumes).length * MODES.length);
  });

  it("exports a GLB with a glTF scene per scale and mode, an FBX per clip, scale and view, valid model JSON with the same scenes, and a ZIP equal to the files", { timeout: 600_000 }, async () => {
    const catalog = catalogOf();
    const out = mkdtempSync(join(tmpdir(), "staffage-scale-out-"));
    made.push(out);
    const { manifest } = await exportModels({ out, catalog, producer: "e".repeat(64), views: VIEWS });
    const files = await auditFiles(asset, manifest.files, (file) => new Uint8Array(readFileSync(join(out, file))), plannedNames(asset));
    expect(files.findings.map((f) => `${f.check} ${f.item}: ${f.message}`)).toEqual([]);
    expect(files.bounds.ok).toBe(true);
    expect(files.poses).toHaveLength(asset.poses.filter((p) => p.key !== true).length);
    expect(files.clips).toHaveLength(authoredScales(asset.volumes).length * VIEWS.length);
    const glb = glbFacts(new Uint8Array(readFileSync(join(out, asset.id, glbName(asset.id, asset.presentation.defaultPose)))));
    expect(Object.keys(glb.scenes)).toEqual(expectedScenes(asset));
    expect(glb.scenes[sceneIdOf(50, "body")]!.triangles).toBeGreaterThan(glb.scenes[sceneIdOf(500, "body")]!.triangles);
  });
});
