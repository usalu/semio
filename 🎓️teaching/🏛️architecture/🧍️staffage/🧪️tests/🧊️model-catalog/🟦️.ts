/** 🧊️ The catalog-wide guard of the 3D models: every one of the 152 assets exports a complete, valid model set, checked by independent oracles.
 *
 * Per asset: an authored volume without findings, the 3D formats offered in `exports`, the procedural model valid and tessellated without non-finite or degenerate triangles and watertight where its solid kind is closed, the extent in `presentation.defaultPose` within the authoring gate of `dimensions` (the best-matching pose is reported, the rest pose never compared), triangle budgets that fall with the scale, then the files of the batch export read back: a Khronos-valid GLB with all seven variants per pose, an FBX per clip that three's `FBXLoader` and assimp both load with every rig bone, a non-empty finite animation of the clip's duration, every model JSON valid against `semio.model/v1` with Ajv, and the ZIP with unique members equal to the files.
 * `SEMIO_STAFFAGE_MODELS_DIR` names an existing batch output (`📤️dist/🧊️models` after `export-models`) whose files are checked instead of exporting each asset again; `SEMIO_STAFFAGE_MODEL_ASSETS` (comma-separated ids) narrows the run.
 * A limitation of an asset is stated in {@link LIMITATIONS} with its reason; nothing else is excused.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ./🔍️oracles/🟦️.ts — the checks
 */
import { mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { type BatchManifest, exportModels, plannedNames } from "../../🔨️modules/📤️export/📦️batch/🟦️.ts";
import { exportIssues, modelFormats, volumeIssues } from "../../🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { type Check, type Finding, auditFiles, auditGeometry } from "./🔍️oracles/🟦️.ts";

const SCALE = 50;
const DIRECTORY = process.env.SEMIO_STAFFAGE_MODELS_DIR;
const ONLY = process.env.SEMIO_STAFFAGE_MODEL_ASSETS?.split(",").filter((id) => id.length > 0);

/** 🚧️ The stated limitations: asset id → the check that is excused for an item, with the reason. Empty: every asset of the catalog passes every check. */
export const LIMITATIONS: Readonly<Record<string, readonly { readonly check: Check; readonly item?: string; readonly reason: string }[]>> = {};

const excused = (asset: string, finding: Finding): boolean => (LIMITATIONS[asset] ?? []).some((l) => l.check === finding.check && (l.item === undefined || l.item === finding.item));
const entries = await assetPathsFromFiles();
const selected = entries.filter((e) => ONLY === undefined || ONLY.includes(e.folder));

describe("🧊️ every asset of the catalog has a valid 3D model set", () => {
  it("finds every asset of the catalog, each once, each with an authored volume", () => {
    expect(new Set(entries.map((e) => e.folder)).size).toBe(entries.length);
    expect(entries.length).toBeGreaterThanOrEqual(152);
  });

  describe.each(selected.map((e) => [e.folder, e] as const))("%s", (_id, entry) => {
    it("offers the 3D formats, has a valid volume, a valid watertight finite model and falling budgets, and a valid file set in every pose and clip", { timeout: 600_000 }, async () => {
      const { asset } = await readStreamedAsset(entry);
      expect(volumeIssues(asset).map((i) => `${i.code} ${i.path}`)).toEqual([]);
      expect(exportIssues(asset).map((i) => `${i.code} ${i.path}`)).toEqual([]);
      for (const format of modelFormats(asset)) expect(asset.exports, `exports names ${format}`).toContain(format);
      expect(asset.clips.length).toBeGreaterThanOrEqual(1);

      const geometry = await auditGeometry(asset);
      const directory = DIRECTORY ?? mkdtempSync(join(tmpdir(), "staffage-models-"));
      try {
        if (DIRECTORY === undefined) await exportModels({ out: directory, scale: SCALE, assets: [asset.id] });
        const manifest = JSON.parse(readFileSync(join(directory, "manifest.json"), "utf8")) as BatchManifest;
        expect(manifest.settings.scale).toBe(SCALE);
        const files = await auditFiles(asset, manifest.files, (file) => new Uint8Array(readFileSync(join(directory, file))), plannedNames(asset, SCALE, manifest.settings.view));
        const findings = [...geometry.findings, ...files.findings].filter((f) => !excused(asset.id, f));
        expect(findings.map((f) => `${f.check} ${f.item}: ${f.message}`)).toEqual([]);
        expect(files.bounds.ok, `${asset.id} presentation pose ${files.bounds.pose}`).toBe(true);
        expect(files.poses.length).toBe(asset.poses.filter((p) => p.key !== true).length);
        expect(files.clips.length).toBe(asset.clips.length);
      } finally {
        if (DIRECTORY === undefined) rmSync(directory, { recursive: true, force: true });
      }
    });
  });
});
