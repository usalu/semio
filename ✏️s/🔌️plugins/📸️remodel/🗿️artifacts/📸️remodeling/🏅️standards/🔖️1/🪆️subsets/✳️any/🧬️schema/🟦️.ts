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

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingArtifactGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingArtifactGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingArtifactGuardRefusal(at, why);
};

type remodelRemodelingArtifactGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingArtifactGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingArtifactGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingArtifactGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingArtifactGuardReject(at, "value is not an object");
export const remodelRemodelingArtifactGuardArray = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingArtifactGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingArtifactGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingArtifactGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingArtifactGuardString = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingArtifactGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingArtifactGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingArtifactGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingArtifactGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingArtifactGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingArtifactGuardReject(at, "value is not a boolean"));
export const remodelRemodelingArtifactGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingArtifactGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingArtifactGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingArtifactGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingArtifactGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingArtifactGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingArtifactGuardNumber(value, at, bounds) : remodelRemodelingArtifactGuardReject(at, "value is not an integer");
export const remodelRemodelingArtifactGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingArtifactGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingArtifactGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingArtifactGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseRemodelingArtifact(value: unknown, at = "$"): RemodelingArtifact {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    schema: remodelRemodelingArtifactGuardString(row["schema"], `${at}.schema`),
    id: remodelRemodelingArtifactGuardString(row["id"], `${at}.id`),
    streams: remodelRemodelingArtifactGuardArray(row["streams"], `${at}.streams`).map((item, index) => parseMediaStream(item, `${at}.streams[${index}]`)),
    assets: remodelRemodelingArtifactGuardObject(row["assets"], `${at}.assets`),
    durableArtifacts: remodelRemodelingArtifactGuardObject(row["durableArtifacts"], `${at}.durableArtifacts`),
    calibration: parseCalibrationState(row["calibration"], `${at}.calibration`),
    params: parseReconstructionParams(row["params"], `${at}.params`),
    gcps: remodelRemodelingArtifactGuardArray(row["gcps"], `${at}.gcps`).map((item, index) => parseGroundControlPoint(item, `${at}.gcps[${index}]`)),
    job: parseReconstructionJob(row["job"], `${at}.job`),
    results: parseReconstructionResults(row["results"], `${at}.results`),
    selection: parseRemodelingUiSelection(row["selection"], `${at}.selection`),
    activeUtilityId: remodelRemodelingArtifactGuardString(row["activeUtilityId"], `${at}.activeUtilityId`),
    reportTable: remodelRemodelingArtifactGuardString(row["reportTable"], `${at}.reportTable`),
    frameCursor: parseRemodelingUiFrameCursor(row["frameCursor"], `${at}.frameCursor`),
    camera: parseRemodelingUiCamera(row["camera"], `${at}.camera`),
    layers: parseRemodelingUiLayers(row["layers"], `${at}.layers`),
  };
}

export interface ArtifactDialect {
  readonly artifactKind: string;
  readonly standard: string;
  readonly subset: string;
}

export function parseArtifactDialect(value: unknown, at = "$"): ArtifactDialect {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    artifactKind: remodelRemodelingArtifactGuardString(row["artifactKind"], `${at}.artifactKind`),
    standard: remodelRemodelingArtifactGuardString(row["standard"], `${at}.standard`),
    subset: remodelRemodelingArtifactGuardString(row["subset"], `${at}.subset`),
  };
}

export interface ArtifactRef {
  readonly artifactId: string;
  readonly dialect: ArtifactDialect;
}

export function parseArtifactRef(value: unknown, at = "$"): ArtifactRef {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    artifactId: remodelRemodelingArtifactGuardString(row["artifactId"], `${at}.artifactId`),
    dialect: parseArtifactDialect(row["dialect"], `${at}.dialect`),
  };
}

export interface CalibrationState {
  readonly cameras: readonly CameraCalibration[];
  readonly rig: readonly RigExtrinsic[];
}

export function parseCalibrationState(value: unknown, at = "$"): CalibrationState {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    cameras: remodelRemodelingArtifactGuardArray(row["cameras"], `${at}.cameras`).map((item, index) => parseCameraCalibration(item, `${at}.cameras[${index}]`)),
    rig: remodelRemodelingArtifactGuardArray(row["rig"], `${at}.rig`).map((item, index) => parseRigExtrinsic(item, `${at}.rig[${index}]`)),
  };
}

export interface CameraPosePreview {
  readonly cameraId: string;
  readonly rotationWxyz: readonly number[];
  readonly translation: readonly number[];
}

export function parseCameraPosePreview(value: unknown, at = "$"): CameraPosePreview {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    cameraId: remodelRemodelingArtifactGuardString(row["cameraId"], `${at}.cameraId`),
    rotationWxyz: remodelRemodelingArtifactGuardArray(row["rotationWxyz"], `${at}.rotationWxyz`, {"minItems": 4, "maxItems": 4}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.rotationWxyz[${index}]`)),
    translation: remodelRemodelingArtifactGuardArray(row["translation"], `${at}.translation`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.translation[${index}]`)),
  };
}

export interface CameraTrajectory {
  readonly poses: readonly CameraPosePreview[];
}

export function parseCameraTrajectory(value: unknown, at = "$"): CameraTrajectory {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    poses: remodelRemodelingArtifactGuardArray(row["poses"], `${at}.poses`).map((item, index) => parseCameraPosePreview(item, `${at}.poses[${index}]`)),
  };
}

export interface DenseParams {
  readonly resolution: DenseResolution;
  readonly windowRadiusPx: number;
  readonly minViewConsistency: number;
  readonly confidenceThreshold: number;
  readonly maxPoints: number;
}

export function parseDenseParams(value: unknown, at = "$"): DenseParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    resolution: parseDenseResolution(row["resolution"], `${at}.resolution`),
    windowRadiusPx: remodelRemodelingArtifactGuardInteger(row["windowRadiusPx"], `${at}.windowRadiusPx`, {"minimum": 0}),
    minViewConsistency: remodelRemodelingArtifactGuardInteger(row["minViewConsistency"], `${at}.minViewConsistency`, {"minimum": 0}),
    confidenceThreshold: remodelRemodelingArtifactGuardNumber(row["confidenceThreshold"], `${at}.confidenceThreshold`),
    maxPoints: remodelRemodelingArtifactGuardInteger(row["maxPoints"], `${at}.maxPoints`, {"minimum": 0}),
  };
}

export type DenseResolution = "low" | "medium" | "high";

export function parseDenseResolution(value: unknown, at = "$"): DenseResolution {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["low", "medium", "high"] as const);
}

export type FeatureDetector = "orb" | "akaze" | "harris";

export function parseFeatureDetector(value: unknown, at = "$"): FeatureDetector {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["orb", "akaze", "harris"] as const);
}

export interface FeatureParams {
  readonly detector: FeatureDetector;
  readonly targetCount: number;
  readonly octaves: number;
  readonly edgeThreshold: number;
}

export function parseFeatureParams(value: unknown, at = "$"): FeatureParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    detector: parseFeatureDetector(row["detector"], `${at}.detector`),
    targetCount: remodelRemodelingArtifactGuardInteger(row["targetCount"], `${at}.targetCount`, {"minimum": 0}),
    octaves: remodelRemodelingArtifactGuardInteger(row["octaves"], `${at}.octaves`, {"minimum": 0}),
    edgeThreshold: remodelRemodelingArtifactGuardNumber(row["edgeThreshold"], `${at}.edgeThreshold`),
  };
}

export interface FrameRef {
  readonly index: number;
  readonly timestampMs: number;
  readonly assetId: string;
}

export function parseFrameRef(value: unknown, at = "$"): FrameRef {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    index: remodelRemodelingArtifactGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    timestampMs: remodelRemodelingArtifactGuardNumber(row["timestampMs"], `${at}.timestampMs`),
    assetId: remodelRemodelingArtifactGuardString(row["assetId"], `${at}.assetId`),
  };
}

export interface GcpObservation {
  readonly streamId: string;
  readonly frameIndex: number;
  readonly pixel: readonly number[];
}

export function parseGcpObservation(value: unknown, at = "$"): GcpObservation {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    streamId: remodelRemodelingArtifactGuardString(row["streamId"], `${at}.streamId`),
    frameIndex: remodelRemodelingArtifactGuardInteger(row["frameIndex"], `${at}.frameIndex`, {"minimum": 0}),
    pixel: remodelRemodelingArtifactGuardArray(row["pixel"], `${at}.pixel`, {"minItems": 2, "maxItems": 2}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.pixel[${index}]`)),
  };
}

export interface GroundControlPoint {
  readonly id: string;
  readonly name: string;
  readonly worldPosition: readonly number[];
  readonly observations: readonly GcpObservation[];
}

export function parseGroundControlPoint(value: unknown, at = "$"): GroundControlPoint {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    id: remodelRemodelingArtifactGuardString(row["id"], `${at}.id`),
    name: remodelRemodelingArtifactGuardString(row["name"], `${at}.name`),
    worldPosition: remodelRemodelingArtifactGuardArray(row["worldPosition"], `${at}.worldPosition`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.worldPosition[${index}]`)),
    observations: remodelRemodelingArtifactGuardArray(row["observations"], `${at}.observations`).map((item, index) => parseGcpObservation(item, `${at}.observations[${index}]`)),
  };
}

export interface IngestParams {
  readonly frameSampleStride: number;
  readonly maxFrames: number;
  readonly downscaleLongEdgePx: number;
  readonly minSharpness: number;
}

export function parseIngestParams(value: unknown, at = "$"): IngestParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    frameSampleStride: remodelRemodelingArtifactGuardInteger(row["frameSampleStride"], `${at}.frameSampleStride`, {"minimum": 0}),
    maxFrames: remodelRemodelingArtifactGuardInteger(row["maxFrames"], `${at}.maxFrames`, {"minimum": 0}),
    downscaleLongEdgePx: remodelRemodelingArtifactGuardInteger(row["downscaleLongEdgePx"], `${at}.downscaleLongEdgePx`, {"minimum": 0}),
    minSharpness: remodelRemodelingArtifactGuardNumber(row["minSharpness"], `${at}.minSharpness`),
  };
}

export interface MatchParams {
  readonly matcher: MatcherKind;
  readonly ratioTest: number;
  readonly crossCheck: boolean;
  readonly sequentialWindow: number;
  readonly maxPairsPerFrame: number;
  readonly loopClosure: boolean;
}

export function parseMatchParams(value: unknown, at = "$"): MatchParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    matcher: parseMatcherKind(row["matcher"], `${at}.matcher`),
    ratioTest: remodelRemodelingArtifactGuardNumber(row["ratioTest"], `${at}.ratioTest`),
    crossCheck: remodelRemodelingArtifactGuardBoolean(row["crossCheck"], `${at}.crossCheck`),
    sequentialWindow: remodelRemodelingArtifactGuardInteger(row["sequentialWindow"], `${at}.sequentialWindow`, {"minimum": 0}),
    maxPairsPerFrame: remodelRemodelingArtifactGuardInteger(row["maxPairsPerFrame"], `${at}.maxPairsPerFrame`, {"minimum": 0}),
    loopClosure: remodelRemodelingArtifactGuardBoolean(row["loopClosure"], `${at}.loopClosure`),
  };
}

export type MatcherKind = "brute-force" | "kd-tree";

export function parseMatcherKind(value: unknown, at = "$"): MatcherKind {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["brute-force", "kd-tree"] as const);
}

export type MediaKind = "image-sequence" | "video";

export function parseMediaKind(value: unknown, at = "$"): MediaKind {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["image-sequence", "video"] as const);
}

export interface MeshParams {
  readonly tsdfVoxelSizeMm: number;
  readonly tsdfTruncationMm: number;
  readonly decimateTargetTriangles: number;
  readonly smoothingIterations: number;
  readonly textureEnabled: boolean;
  readonly textureSize: number;
  readonly guaranteeWatertight: boolean;
  readonly holeFillMaxBoundaryVerts: number;
  readonly selfIntersectionCheck: boolean;
}

export function parseMeshParams(value: unknown, at = "$"): MeshParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    tsdfVoxelSizeMm: remodelRemodelingArtifactGuardNumber(row["tsdfVoxelSizeMm"], `${at}.tsdfVoxelSizeMm`),
    tsdfTruncationMm: remodelRemodelingArtifactGuardNumber(row["tsdfTruncationMm"], `${at}.tsdfTruncationMm`),
    decimateTargetTriangles: remodelRemodelingArtifactGuardInteger(row["decimateTargetTriangles"], `${at}.decimateTargetTriangles`, {"minimum": 0}),
    smoothingIterations: remodelRemodelingArtifactGuardInteger(row["smoothingIterations"], `${at}.smoothingIterations`, {"minimum": 0}),
    textureEnabled: remodelRemodelingArtifactGuardBoolean(row["textureEnabled"], `${at}.textureEnabled`),
    textureSize: remodelRemodelingArtifactGuardInteger(row["textureSize"], `${at}.textureSize`, {"minimum": 0}),
    guaranteeWatertight: remodelRemodelingArtifactGuardBoolean(row["guaranteeWatertight"], `${at}.guaranteeWatertight`),
    holeFillMaxBoundaryVerts: remodelRemodelingArtifactGuardInteger(row["holeFillMaxBoundaryVerts"], `${at}.holeFillMaxBoundaryVerts`, {"minimum": 0}),
    selfIntersectionCheck: remodelRemodelingArtifactGuardBoolean(row["selfIntersectionCheck"], `${at}.selfIntersectionCheck`),
  };
}

export type MeshSource = "placeholder" | "reconstructed" | "imported";

export function parseMeshSource(value: unknown, at = "$"): MeshSource {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["placeholder", "reconstructed", "imported"] as const);
}

export interface MotionParams {
  readonly enabled: boolean;
  readonly maxTracks: number;
  readonly trackWindowPx: number;
  readonly minTrackQuality: number;
  readonly minTrackLengthFrames: number;
}

export function parseMotionParams(value: unknown, at = "$"): MotionParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    enabled: remodelRemodelingArtifactGuardBoolean(row["enabled"], `${at}.enabled`),
    maxTracks: remodelRemodelingArtifactGuardInteger(row["maxTracks"], `${at}.maxTracks`, {"minimum": 0}),
    trackWindowPx: remodelRemodelingArtifactGuardInteger(row["trackWindowPx"], `${at}.trackWindowPx`, {"minimum": 0}),
    minTrackQuality: remodelRemodelingArtifactGuardNumber(row["minTrackQuality"], `${at}.minTrackQuality`),
    minTrackLengthFrames: remodelRemodelingArtifactGuardInteger(row["minTrackLengthFrames"], `${at}.minTrackLengthFrames`, {"minimum": 0}),
  };
}

export interface MotionTrackSummary {
  readonly id: string;
  readonly length: number;
  readonly class: TrackClass;
  readonly meanSpeedMS: number;
}

export function parseMotionTrackSummary(value: unknown, at = "$"): MotionTrackSummary {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    id: remodelRemodelingArtifactGuardString(row["id"], `${at}.id`),
    length: remodelRemodelingArtifactGuardInteger(row["length"], `${at}.length`, {"minimum": 0}),
    class: parseTrackClass(row["class"], `${at}.class`),
    meanSpeedMS: remodelRemodelingArtifactGuardNumber(row["meanSpeedMS"], `${at}.meanSpeedMS`),
  };
}

export interface ReconstructionParams {
  readonly ingest: IngestParams;
  readonly feature: FeatureParams;
  readonly matching: MatchParams;
  readonly sfm: SfmParams;
  readonly dense: DenseParams;
  readonly mesh: MeshParams;
  readonly motion: MotionParams;
  readonly geo: GeoParams;
}

export function parseReconstructionParams(value: unknown, at = "$"): ReconstructionParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    ingest: parseIngestParams(row["ingest"], `${at}.ingest`),
    feature: parseFeatureParams(row["feature"], `${at}.feature`),
    matching: parseMatchParams(row["matching"], `${at}.matching`),
    sfm: parseSfmParams(row["sfm"], `${at}.sfm`),
    dense: parseDenseParams(row["dense"], `${at}.dense`),
    mesh: parseMeshParams(row["mesh"], `${at}.mesh`),
    motion: parseMotionParams(row["motion"], `${at}.motion`),
    geo: parseGeoParams(row["geo"], `${at}.geo`),
  };
}

export type ReconstructionStage = "idle" | "ingesting" | "calibrating" | "extracting-features" | "matching-features" | "estimating-poses" | "bundle-adjusting" | "georeferencing" | "dense-stereo" | "fusing-volume" | "extracting-surface" | "cleaning-mesh" | "texturing" | "tracking-motion" | "deriving-geo-products" | "reporting-qc" | "done" | "failed";

export function parseReconstructionStage(value: unknown, at = "$"): ReconstructionStage {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["idle", "ingesting", "calibrating", "extracting-features", "matching-features", "estimating-poses", "bundle-adjusting", "georeferencing", "dense-stereo", "fusing-volume", "extracting-surface", "cleaning-mesh", "texturing", "tracking-motion", "deriving-geo-products", "reporting-qc", "done", "failed"] as const);
}

export interface RemodelingAssetChild {
  readonly childId: string;
  readonly target: ArtifactRef;
}

export function parseRemodelingAssetChild(value: unknown, at = "$"): RemodelingAssetChild {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    childId: remodelRemodelingArtifactGuardString(row["childId"], `${at}.childId`),
    target: parseArtifactRef(row["target"], `${at}.target`),
  };
}

export interface RemodelingMeshChild {
  readonly childId: string;
  readonly target: ArtifactRef;
}

export function parseRemodelingMeshChild(value: unknown, at = "$"): RemodelingMeshChild {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    childId: remodelRemodelingArtifactGuardString(row["childId"], `${at}.childId`),
    target: parseArtifactRef(row["target"], `${at}.target`),
  };
}

export function parseRemodelingUiCamera(value: unknown, at = "$"): RemodelingUiCamera {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    position: remodelRemodelingArtifactGuardArray(row["position"], `${at}.position`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.position[${index}]`)),
    target: remodelRemodelingArtifactGuardArray(row["target"], `${at}.target`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.target[${index}]`)),
    fov: remodelRemodelingArtifactGuardNumber(row["fov"], `${at}.fov`),
  };
}

export function parseRemodelingUiLayers(value: unknown, at = "$"): RemodelingUiLayers {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    mesh: remodelRemodelingArtifactGuardBoolean(row["mesh"], `${at}.mesh`),
    dense: remodelRemodelingArtifactGuardBoolean(row["dense"], `${at}.dense`),
    sparse: remodelRemodelingArtifactGuardBoolean(row["sparse"], `${at}.sparse`),
    cameras: remodelRemodelingArtifactGuardBoolean(row["cameras"], `${at}.cameras`),
    gcps: remodelRemodelingArtifactGuardBoolean(row["gcps"], `${at}.gcps`),
  };
}

export function parseRemodelingUiSelection(value: unknown, at = "$"): RemodelingUiSelection {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    mode: remodelRemodelingArtifactGuardString(row["mode"], `${at}.mode`),
    ids: remodelRemodelingArtifactGuardArray(row["ids"], `${at}.ids`).map((item, index) => remodelRemodelingArtifactGuardString(item, `${at}.ids[${index}]`)),
  };
}

export interface RigExtrinsic {
  readonly cameraId: string;
  readonly rotationWxyz: readonly number[];
  readonly translationM: readonly number[];
}

export function parseRigExtrinsic(value: unknown, at = "$"): RigExtrinsic {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    cameraId: remodelRemodelingArtifactGuardString(row["cameraId"], `${at}.cameraId`),
    rotationWxyz: remodelRemodelingArtifactGuardArray(row["rotationWxyz"], `${at}.rotationWxyz`, {"minItems": 4, "maxItems": 4}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.rotationWxyz[${index}]`)),
    translationM: remodelRemodelingArtifactGuardArray(row["translationM"], `${at}.translationM`, {"minItems": 3, "maxItems": 3}).map((item, index) => remodelRemodelingArtifactGuardNumber(item, `${at}.translationM[${index}]`)),
  };
}

export type RobustLossKind = "l2" | "huber" | "cauchy";

export function parseRobustLossKind(value: unknown, at = "$"): RobustLossKind {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["l2", "huber", "cauchy"] as const);
}

export interface SfmParams {
  readonly ransacIterations: number;
  readonly ransacThresholdPx: number;
  readonly minTrackLength: number;
  readonly baMaxIterations: number;
  readonly robustLoss: RobustLossKind;
  readonly huberDeltaPx: number;
}

export function parseSfmParams(value: unknown, at = "$"): SfmParams {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    ransacIterations: remodelRemodelingArtifactGuardInteger(row["ransacIterations"], `${at}.ransacIterations`, {"minimum": 0}),
    ransacThresholdPx: remodelRemodelingArtifactGuardNumber(row["ransacThresholdPx"], `${at}.ransacThresholdPx`),
    minTrackLength: remodelRemodelingArtifactGuardInteger(row["minTrackLength"], `${at}.minTrackLength`, {"minimum": 0}),
    baMaxIterations: remodelRemodelingArtifactGuardInteger(row["baMaxIterations"], `${at}.baMaxIterations`, {"minimum": 0}),
    robustLoss: parseRobustLossKind(row["robustLoss"], `${at}.robustLoss`),
    huberDeltaPx: remodelRemodelingArtifactGuardNumber(row["huberDeltaPx"], `${at}.huberDeltaPx`),
  };
}

export type TrackClass = "static" | "moving";

export function parseTrackClass(value: unknown, at = "$"): TrackClass {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["static", "moving"] as const);
}

export type VideoCodec = "avc" | "hevc" | "vp9" | "av1" | "mjpeg" | "unknown";

export function parseVideoCodec(value: unknown, at = "$"): VideoCodec {
  return remodelRemodelingArtifactGuardMember(value, `${at}`, ["avc", "hevc", "vp9", "av1", "mjpeg", "unknown"] as const);
}

export interface VideoSource {
  readonly name: string;
  readonly container: string;
  readonly codec: VideoCodec;
  readonly durationMs: number;
  readonly frameCount: number;
  readonly width: number;
  readonly height: number;
}

export function parseVideoSource(value: unknown, at = "$"): VideoSource {
  const row = remodelRemodelingArtifactGuardObject(value, at);
  return {
    name: remodelRemodelingArtifactGuardString(row["name"], `${at}.name`),
    container: remodelRemodelingArtifactGuardString(row["container"], `${at}.container`),
    codec: parseVideoCodec(row["codec"], `${at}.codec`),
    durationMs: remodelRemodelingArtifactGuardNumber(row["durationMs"], `${at}.durationMs`),
    frameCount: remodelRemodelingArtifactGuardInteger(row["frameCount"], `${at}.frameCount`, {"minimum": 0}),
    width: remodelRemodelingArtifactGuardInteger(row["width"], `${at}.width`, {"minimum": 0}),
    height: remodelRemodelingArtifactGuardInteger(row["height"], `${at}.height`, {"minimum": 0}),
  };
}
