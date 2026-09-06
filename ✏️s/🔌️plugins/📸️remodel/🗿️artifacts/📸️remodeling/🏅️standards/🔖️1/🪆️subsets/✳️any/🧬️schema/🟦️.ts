/** 🧬️ Remodeling artifact schema — TypeScript twin of `🧬️schema/🦀️.rs`.
 *
 *  The nine artifact-lane fields are the snapshot's own; the seven presence/config fields are
 *  declared here. Every domain type is imported from `📸️snapshot/🟦️.ts` rather than restated, so
 *  the artifact and the snapshot cannot drift apart. Field order below is Rust declaration order,
 *  which is the order `serde` emits.
 *
 *  Drift against the committed `🔣️.json` JSON Schema leaf is recorded in
 *  `📓️w3-ts-codec-oracle.md`: that leaf declares `MediaStream`/`ImageAsset`/`CalibrationState`/
 *  `ReconstructionParams`/`GroundControlPoint`/`ReconstructionJob`/`ReconstructionResults` as bare
 *  `{ "type": "object" }` with no properties and omits `durableArtifacts` entirely, so it is NOT
 *  authoritative for those seven types — Rust is, and Rust is what this file mirrors.
 */

import {
  ARTIFACT_CHILD_SPEC,
  CALIBRATION_STATE_SPEC,
  DURABLE_ARTIFACT_SPEC,
  GROUND_CONTROL_POINT_SPEC,
  MEDIA_STREAM_SPEC,
  RECONSTRUCTION_JOB_SPEC,
  RECONSTRUCTION_PARAMS_SPEC,
  RECONSTRUCTION_RESULTS_SPEC,
  decodeRecord,
  defaultsOf,
  type CalibrationState,
  type GroundControlPoint,
  type MediaStream,
  type ReconstructionJob,
  type ReconstructionParams,
  type ReconstructionResults,
  type RecordSpec,
  type RemodelingAssetChild,
  type RemodelingDurableArtifactStore,
  type RemodelingSnapshot,
  type ValueSpec,
  type Vec3,
} from "./📸️snapshot/🟦️.ts";

export * from "./📸️snapshot/🟦️.ts";

//#region 🔖️UiHelpers
/** 🎥️ Artifact-owned orbit camera (mirror of app config camera). */
export interface RemodelingUiCamera {
  position: Vec3;
  target: Vec3;
  fov: number;
}

/** 🖱️ Artifact-owned selection (mirror of app config selection). */
export interface RemodelingUiSelection {
  mode: string;
  ids: string[];
}

/** 👁️ Artifact-owned layer visibility (mirror of app config layers). */
export interface RemodelingUiLayers {
  mesh: boolean;
  dense: boolean;
  sparse: boolean;
  cameras: boolean;
  gcps: boolean;
}

/** 🎞️ Artifact-owned frame cursor (mirror of app config frame cursor). */
export interface RemodelingUiFrameCursor {
  streamId: string | null;
  frameIndex: number;
}
//#endregion 🔖️UiHelpers

//#region 🔖️Artifact
/** 🧬️ Full remodeling artifact state across the artifact, presence and config lanes. */
export interface RemodelingArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  streams: MediaStream[];
  /** @state artifact */
  assets: Record<string, RemodelingAssetChild>;
  /** @state artifact */
  durableArtifacts: RemodelingDurableArtifactStore;
  /** @state artifact */
  calibration: CalibrationState;
  /** @state artifact */
  params: ReconstructionParams;
  /** @state artifact */
  gcps: GroundControlPoint[];
  /** @state artifact */
  job: ReconstructionJob;
  /** @state artifact */
  results: ReconstructionResults;
  /** @state presence */
  selection: RemodelingUiSelection;
  /** @state presence */
  activeUtilityId: string;
  /** @state presence */
  reportTable: string;
  /** @state presence */
  frameCursor: RemodelingUiFrameCursor;
  /** @state config */
  camera: RemodelingUiCamera;
  /** @state config */
  layers: RemodelingUiLayers;
  /** @state config */
  locale: string;
}
//#endregion 🔖️Artifact

//#region 🔖️Spec
const f = (name: string, spec: ValueSpec, dflt: () => unknown): { name: string; spec: ValueSpec; dflt: () => unknown } => ({ name, spec, dflt });
const text = { k: "text" } as const;
const bool = { k: "bool" } as const;
const uint = { k: "uint" } as const;
const f64 = { k: "f64" } as const;
const opt = (of: ValueSpec): ValueSpec => ({ k: "opt", of });
const list = (of: ValueSpec): ValueSpec => ({ k: "list", of });
const map = (of: ValueSpec): ValueSpec => ({ k: "map", of });
const rec = (of: () => RecordSpec): ValueSpec => ({ k: "rec", of });
const tuple3 = { k: "tuple", len: 3, w: 64 } as const;

export const REMODELING_UI_CAMERA_SPEC: RecordSpec = {
  title: "RemodelingUiCamera",
  serdeDefault: true,
  fields: [f("position", tuple3, () => [4, -4, 3]), f("target", tuple3, () => [0, 0, 0]), f("fov", f64, () => 45)],
};

export const REMODELING_UI_SELECTION_SPEC: RecordSpec = {
  title: "RemodelingUiSelection",
  serdeDefault: true,
  fields: [f("mode", text, () => ""), f("ids", list(text), () => [])],
};

export const REMODELING_UI_LAYERS_SPEC: RecordSpec = {
  title: "RemodelingUiLayers",
  serdeDefault: true,
  fields: [f("mesh", bool, () => true), f("dense", bool, () => true), f("sparse", bool, () => true), f("cameras", bool, () => true), f("gcps", bool, () => true)],
};

export const REMODELING_UI_FRAME_CURSOR_SPEC: RecordSpec = {
  title: "RemodelingUiFrameCursor",
  serdeDefault: true,
  fields: [f("stream_id", opt(text), () => null), f("frame_index", uint, () => 0)],
};

export const REMODELING_ARTIFACT_SPEC: RecordSpec = {
  title: "RemodelingArtifact",
  serdeDefault: false,
  fields: [
    f("schema", text, () => "remodeling.scene"),
    f("id", text, () => "remodeling"),
    f("streams", list(rec(() => MEDIA_STREAM_SPEC)), () => []),
    f("assets", map(rec(() => ARTIFACT_CHILD_SPEC)), () => ({})),
    f("durable_artifacts", map(rec(() => DURABLE_ARTIFACT_SPEC)), () => ({})),
    f("calibration", rec(() => CALIBRATION_STATE_SPEC), () => defaultsOf(CALIBRATION_STATE_SPEC)),
    f("params", rec(() => RECONSTRUCTION_PARAMS_SPEC), () => defaultsOf(RECONSTRUCTION_PARAMS_SPEC)),
    f("gcps", list(rec(() => GROUND_CONTROL_POINT_SPEC)), () => []),
    f("job", rec(() => RECONSTRUCTION_JOB_SPEC), () => defaultsOf(RECONSTRUCTION_JOB_SPEC)),
    f("results", rec(() => RECONSTRUCTION_RESULTS_SPEC), () => defaultsOf(RECONSTRUCTION_RESULTS_SPEC)),
    f("selection", rec(() => REMODELING_UI_SELECTION_SPEC), () => defaultsOf(REMODELING_UI_SELECTION_SPEC)),
    f("active_utility_id", text, () => "select"),
    f("report_table", text, () => "frames"),
    f("frame_cursor", rec(() => REMODELING_UI_FRAME_CURSOR_SPEC), () => defaultsOf(REMODELING_UI_FRAME_CURSOR_SPEC)),
    f("camera", rec(() => REMODELING_UI_CAMERA_SPEC), () => defaultsOf(REMODELING_UI_CAMERA_SPEC)),
    f("layers", rec(() => REMODELING_UI_LAYERS_SPEC), () => defaultsOf(REMODELING_UI_LAYERS_SPEC)),
    f("locale", text, () => "en-US"),
  ],
};

/** 🔑 The nine artifact-lane field names shared by the artifact, the snapshot and the diff. */
export const REMODELING_SNAPSHOT_FIELDS: readonly string[] = ["schema", "id", "streams", "assets", "durableArtifacts", "calibration", "params", "gcps", "job", "results"];
//#endregion 🔖️Spec

//#region 🔖️Conversions
/** 🧬️ `RemodelingArtifact::from_snapshot` — UI fields land on their declared defaults. */
export function remodelingArtifactFromSnapshot(snapshot: RemodelingSnapshot): RemodelingArtifact {
  const artifact = defaultsOf(REMODELING_ARTIFACT_SPEC) as unknown as RemodelingArtifact;
  for (const field of REMODELING_SNAPSHOT_FIELDS) (artifact as unknown as Record<string, unknown>)[field] = (snapshot as unknown as Record<string, unknown>)[field];
  return artifact;
}

/** 📸️ `RemodelingArtifact::to_snapshot` — the persisted subset. */
export function remodelingArtifactToSnapshot(artifact: RemodelingArtifact): RemodelingSnapshot {
  const snapshot: Record<string, unknown> = {};
  for (const field of REMODELING_SNAPSHOT_FIELDS) snapshot[field] = (artifact as unknown as Record<string, unknown>)[field];
  return snapshot as unknown as RemodelingSnapshot;
}

/** 🧬️ Decodes a parsed RFC 8259 value into a validated `RemodelingArtifact`. */
export const decodeRemodelingArtifact = (json: unknown): RemodelingArtifact => decodeRecord(json, REMODELING_ARTIFACT_SPEC, "") as unknown as RemodelingArtifact;

//#endregion 🔖️Conversions
