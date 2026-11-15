/** 🧊️ The catalog-wide guard of the 3D models: every asset exports a complete, valid model set at every authored scale and in every mode, checked by independent oracles.
 *
 * Per authored asset: a volume without findings, the 3D formats offered in `exports`, the procedural multi-scene model valid, every scene tessellated at the quality of its scale without non-finite or degenerate triangles and watertight where its solid kind is closed, triangle budgets that fall with the scale in every mode, then the files of the batch export read back: a Khronos-valid GLB per pose with one glTF scene per scale and mode that three and glTF-Transform both read, an FBX per clip, authored scale and checked view that three's `FBXLoader` and assimp both load with every rig bone and whose triangles equal those of its GLB scene, every model JSON valid against `semio.model/v1` with Ajv and holding the same scenes, the extent of every scale's body scene against `dimensions`, and the ZIP with unique members equal to the files.
 * The assets are re-authored by others and become valid gradually: an asset with no authored scale must be named in the `pending` list of `🧫️fixtures/🧊️model-catalog/🔣️.json`, and a listed asset that has been authored fails the guard until it is removed (a ratchet: the list only shrinks, an authored asset can never silently lose its volumes, no test is skipped). The report of the run (`SEMIO_STAFFAGE_MODEL_REPORT` names a file) lists the authored and the pending assets.
 * `SEMIO_STAFFAGE_MODELS_DIR` names an existing batch output (`📤️dist/🧊️models` after `export-models`) whose files are checked instead of exporting each asset again; `SEMIO_STAFFAGE_MODEL_ASSETS` (comma-separated ids) narrows the run; `SEMIO_STAFFAGE_MODEL_VIEWS` (comma-separated views or `all`) names the FBX views of the in-process export, default one view per mode (`light-solid`, `thermal`, `forces`).
 * A limitation of an asset is stated in {@link LIMITATIONS} with its reason; nothing else is excused.
 *
 * @see ./🥒️.feature — the scenarios
 * @see ./🔍️oracles/🟦️.ts — the checks
 */
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { VIEWS, type View } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { type BatchManifest, exportModels, plannedNames } from "../../🔨️modules/📤️export/📦️batch/🟦️.ts";
import { exportIssues, modelFormats, volumeIssues } from "../../🔨️modules/🗂️catalog/🧊️volume/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "../../🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { authoredScales } from "../../🔨️modules/🧊️volume/🟦️.ts";
import { fixture } from "../🧰️support/🧰️.ts";
import { type Check, type Finding, auditFiles, auditGeometry } from "./🔍️oracles/🟦️.ts";

const DIRECTORY = process.env.SEMIO_STAFFAGE_MODELS_DIR;
const ONLY = process.env.SEMIO_STAFFAGE_MODEL_ASSETS?.split(",").filter((id) => id.length > 0);
const ASKED = process.env.SEMIO_STAFFAGE_MODEL_VIEWS;
const FBX_VIEWS: readonly View[] = ASKED === "all" ? VIEWS : ASKED === undefined ? ["light-solid", "thermal", "forces"] : (ASKED.split(",") as View[]);
const REPORT = process.env.SEMIO_STAFFAGE_MODEL_REPORT;
const PENDING = new Set(fixture<{ readonly pending: readonly string[] }>("🧊️model-catalog").pending);

/** 🚧️ The stated limitations: asset id → the check that is excused for an item, with the reason. Empty: every authored asset passes every check. */
export const LIMITATIONS: Readonly<Record<string, readonly { readonly check: Check; readonly item?: string; readonly reason: string }[]>> = {};

const excused = (asset: string, finding: Finding): boolean => (LIMITATIONS[asset] ?? []).some((l) => l.check === finding.check && (l.item === undefined || l.item === finding.item));
const entries = await assetPathsFromFiles();
const selected = entries.filter((e) => ONLY === undefined || ONLY.includes(e.folder));
const loaded = await Promise.all(selected.map(async (entry) => ({ entry, asset: (await readStreamedAsset(entry)).asset })));
const authored = loaded.filter(({ asset }) => authoredScales(asset.volumes).length > 0);
const pending = loaded.filter(({ asset }) => authoredScales(asset.volumes).length === 0);

describe("🧊️ every asset of the catalog has a valid 3D model set at every authored scale", () => {
  it("finds every asset of the catalog, each once", () => {
    expect(new Set(entries.map((e) => e.folder)).size).toBe(entries.length);
    expect(entries.length).toBeGreaterThanOrEqual(152);
  });

  it("names exactly the assets without an authored scale as pending, and reports authored and pending", () => {
    const stale = authored.filter(({ asset }) => PENDING.has(asset.id)).map(({ asset }) => asset.id);
    const unlisted = pending.filter(({ asset }) => !PENDING.has(asset.id)).map(({ asset }) => asset.id);
    const unknown = [...PENDING].filter((id) => !entries.some((e) => e.folder === id));
    if (REPORT !== undefined) writeFileSync(REPORT, `${JSON.stringify({ authored: authored.map(({ asset }) => ({ id: asset.id, scales: authoredScales(asset.volumes) })), pending: pending.map(({ asset }) => asset.id) }, null, 1)}\n`);
    expect({ authoredButListedPending: stale, notAuthoredAndNotListed: ONLY === undefined ? unlisted : [], unknownInList: unknown }).toEqual({ authoredButListedPending: [], notAuthoredAndNotListed: [], unknownInList: [] });
  });

  describe.each(selected.map((e) => [e.folder, e] as const))("%s", (id, entry) => {
    it("offers the 3D formats, has a valid volume at every authored scale, valid watertight finite scenes with falling budgets, and a valid file set in every pose, clip, scale and mode (or is pending its re-authoring)", { timeout: 900_000 }, async () => {
      const { asset } = await readStreamedAsset(entry);
      if (authoredScales(asset.volumes).length === 0) {
        expect(PENDING.has(id), `${id} has no authored scale and is not listed pending`).toBe(true);
        return;
      }
      expect(volumeIssues(asset).map((i) => `${i.code} ${i.path}`)).toEqual([]);
      expect(exportIssues(asset).map((i) => `${i.code} ${i.path}`)).toEqual([]);
      for (const format of modelFormats(asset)) expect(asset.exports, `exports names ${format}`).toContain(format);
      expect(asset.clips.length).toBeGreaterThanOrEqual(1);

      const geometry = await auditGeometry(asset);
      const directory = DIRECTORY ?? mkdtempSync(join(tmpdir(), "staffage-models-"));
      try {
        if (DIRECTORY === undefined) await exportModels({ out: directory, assets: [asset.id], views: FBX_VIEWS });
        const manifest = JSON.parse(readFileSync(join(directory, "manifest.json"), "utf8")) as BatchManifest;
        const have = new Set(manifest.files.filter((f) => f.asset === asset.id && f.kind === "fbx").map((f) => f.view));
        const files = await auditFiles(asset, manifest.files, (file) => new Uint8Array(readFileSync(join(directory, file))), plannedNames(asset, { views: DIRECTORY === undefined ? FBX_VIEWS : VIEWS.filter((v) => have.has(v)) }));
        const findings = [...geometry.findings, ...files.findings].filter((f) => !excused(asset.id, f));
        expect(findings.map((f) => `${f.check} ${f.item}: ${f.message}`)).toEqual([]);
        expect(files.bounds.ok, `${asset.id} presentation pose ${files.bounds.pose}`).toBe(true);
        expect(files.poses.length).toBe(asset.poses.filter((p) => p.key !== true).length);
        expect(new Set(files.clips.map((c) => c.id)).size).toBe(asset.clips.length);
      } finally {
        if (DIRECTORY === undefined) rmSync(directory, { recursive: true, force: true });
      }
    });
  });
});
