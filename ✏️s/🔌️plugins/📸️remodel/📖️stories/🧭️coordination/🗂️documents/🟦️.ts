/** 🗂️ The story-local remodel documents: the populated toy scene and the empty default scene. */
import type { RemodelScene } from "../📐️scene-model/🟦️.ts";
import populatedScene from "./🔣️.json";

//#region 🔖️Documents
/** 📸️ The shared populated fixture document, transcribed field-for-field from the plugin's own `⬅️before/🔣️.json`. */
export const REMODEL_POPULATED_SCENE: RemodelScene = populatedScene as unknown as RemodelScene;

/** 🌱️ The boot document — `default_remodeling_scene()` / the shipped `📚️examples/🎬️demo` DSL: everything empty, the placeholder mesh handle seeded. */
export const REMODEL_EMPTY_SCENE: RemodelScene = {
  ...REMODEL_POPULATED_SCENE,
  id: "remodeling",
  streams: [],
  assets: {},
  durableArtifacts: {},
  calibration: { cameras: [], rig: [] },
  gcps: [],
  results: { sparse: null, dense: null, mesh: { mesh: REMODEL_POPULATED_SCENE.results.mesh.mesh, source: "placeholder", textureAssetId: null, watertight: null }, trajectory: null, tracks: [], geo: null, qc: null },
};
//#endregion 🔖️Documents
