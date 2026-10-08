/** 🔺️ Remodeling diff schema — TypeScript twin of `🔺️diff/🦀️.rs`: sparse keyed rows per id-keyed collection, ordered content
 *  rows for the durable chunk store, per-facet params and per-slot results. Every row names exactly the entity and fields it
 *  changes; `applyRemodelingDiff` is the twin of the Rust `MutationDiff::apply` and the only writer of a snapshot.
 *
 *  `RemodelingDiff` carries `#[value(rename_all = "camelCase", default)]` and none of its optional fields is skipped on the
 *  wire, so an untouched lane is written as an explicit `null` — the encoder here does the same.
 */

import {ARTIFACT_CHILD_SPEC, CAMERA_CALIBRATION_SPEC, CAMERA_TRAJECTORY_SPEC, DENSE_CLOUD_SPEC, DENSE_PARAMS_SPEC, FEATURE_PARAMS_SPEC, FRAME_REF_SPEC, GCP_OBSERVATION_SPEC, GEO_PARAMS_SPEC, GEO_PRODUCTS_SPEC, GROUND_CONTROL_POINT_SPEC, INGEST_PARAMS_SPEC, MATCH_PARAMS_SPEC, MEDIA_STREAM_SPEC, MESH_PARAMS_SPEC, MOTION_PARAMS_SPEC, MOTION_TRACK_SUMMARY_SPEC, QC_REPORT_SPEC, REMODELING_MESH_SPEC, RIG_EXTRINSIC_SPEC, SFM_PARAMS_SPEC, SPARSE_CLOUD_SPEC, VIDEO_SOURCE_SPEC, defaultsOf, type ByteBuffer, type CameraCalibration, type CameraTrajectory, type DenseCloud, type DenseParams, type FeatureParams, type FrameRef, type GcpObservation, type GeoParams, type GeoProducts, type GroundControlPoint, type IngestParams, type MatchParams, type MediaStream, type MeshParams, type MotionParams, type MotionTrackSummary, type QcReportSnapshot, type RecordSpec, type RemodelingAssetChild, type RemodelingDurableArtifact, type RemodelingMesh, type RemodelingSnapshot, type RigExtrinsic, type SfmParams, type SparseCloud, type ValueSpec, type VideoSource} from "../📸️snapshot/🟦️.ts";
import {binary32Value, binary64Value, type Binary32, type Binary64} from "../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

//#region 🔖️Types
/** 🎯️ An explicitly assigned optional value, so assigning `null` stays distinct from leaving the slot untouched. */
export interface RemodelingAssigned<T> {
  value: T;
}

export type RemodelingRow<E, P> = { row: "insert"; entity: E } | { row: "replace"; entity: E } | { row: "remove"; key: string } | { row: "patch"; key: string; patch: P };

export interface RemodelingRows<E, P> {
  rows: RemodelingRow<E, P>[];
}

export type RemodelingNoPatch = Record<string, never>;

export interface RemodelingMembers<T> {
  removed: T[];
  added: T[];
}

export interface MediaStreamPatch {
  syncOffsetMs: Binary64 | null;
  source: RemodelingAssigned<VideoSource | null> | null;
  frames: RemodelingMembers<FrameRef> | null;
}

export interface GroundControlPointPatch {
  observations: RemodelingMembers<GcpObservation> | null;
}

export interface RemodelingAssetEntry {
  key: string;
  child: RemodelingAssetChild;
}

export type RemodelingStreamsDelta = RemodelingRows<MediaStream, MediaStreamPatch>;
export type RemodelingGcpsDelta = RemodelingRows<GroundControlPoint, GroundControlPointPatch>;
export type RemodelingCamerasDelta = RemodelingRows<CameraCalibration, RemodelingNoPatch>;
export type RemodelingRigDelta = RemodelingRows<RigExtrinsic, RemodelingNoPatch>;
export type RemodelingAssetsDelta = RemodelingRows<RemodelingAssetEntry, RemodelingNoPatch>;

export interface RemodelingCalibrationDelta {
  cameras: RemodelingCamerasDelta;
  rig: RemodelingRigDelta;
}

export interface RemodelingContentHeader {
  kind: string;
  mime: string | null;
  width: number;
  height: number;
}

export type RemodelingContentRow = { row: "append"; id: string; header: RemodelingContentHeader | null; chunks: ByteBuffer[] } | { row: "truncate"; id: string; from: bigint };

export interface RemodelingContentDelta {
  rows: RemodelingContentRow[];
}

export interface RemodelingParamsDiff {
  ingest: IngestParams | null;
  feature: FeatureParams | null;
  matching: MatchParams | null;
  sfm: SfmParams | null;
  dense: DenseParams | null;
  mesh: MeshParams | null;
  motion: MotionParams | null;
  geo: GeoParams | null;
}

export interface RemodelingResultsDiff {
  sparse: RemodelingAssigned<SparseCloud | null> | null;
  dense: RemodelingAssigned<DenseCloud | null> | null;
  mesh: RemodelingMesh | null;
  trajectory: RemodelingAssigned<CameraTrajectory | null> | null;
  tracks: MotionTrackSummary[] | null;
  geo: RemodelingAssigned<GeoProducts | null> | null;
  qc: RemodelingAssigned<QcReportSnapshot | null> | null;
}

export interface RemodelingDiff {
  schema: string | null;
  id: string | null;
  streams: RemodelingStreamsDelta | null;
  assets: RemodelingAssetsDelta | null;
  durableArtifacts: RemodelingContentDelta | null;
  calibration: RemodelingCalibrationDelta | null;
  params: RemodelingParamsDiff | null;
  gcps: RemodelingGcpsDelta | null;
  results: RemodelingResultsDiff | null;
}
//#endregion 🔖️Types

//#region 🔖️Spec
const f = (name: string, spec: ValueSpec, dflt: () => unknown): { name: string; spec: ValueSpec; dflt: () => unknown } => ({ name, spec, dflt });
const text = { k: "text" } as const;
const u64 = { k: "u64" } as const;
const uint = { k: "uint" } as const;
const f64 = { k: "f64" } as const;
const opt = (of: ValueSpec): ValueSpec => ({ k: "opt", of });
const list = (of: ValueSpec): ValueSpec => ({ k: "list", of });
const rec = (of: () => RecordSpec): ValueSpec => ({ k: "rec", of });
const nothing = () => null;
const required = (): never => {
  throw new Error("required key");
};
const record = (title: string, fields: RecordSpec["fields"], serdeDefault = false): RecordSpec => ({ title, serdeDefault, fields });

const assignedSpec = (title: string, of: () => RecordSpec): RecordSpec => record(title, [f("value", opt(rec(of)), required)]);
const membersSpec = (title: string, item: () => RecordSpec): RecordSpec => record(title, [f("removed", list(rec(item)), () => []), f("added", list(rec(item)), () => [])], true);

const rowSpec = (title: string, entity: () => RecordSpec, patch: () => RecordSpec): ValueSpec => ({
  k: "tagged",
  tag: "row",
  variants: [
    { name: "insert", spec: () => record(`${title}Insert`, [f("entity", rec(entity), required)]) },
    { name: "replace", spec: () => record(`${title}Replace`, [f("entity", rec(entity), required)]) },
    { name: "remove", spec: () => record(`${title}Remove`, [f("key", text, required)]) },
    { name: "patch", spec: () => record(`${title}Patch`, [f("key", text, required), f("patch", rec(patch), required)]) },
  ],
});
const rowsSpec = (title: string, entity: () => RecordSpec, patch: () => RecordSpec): RecordSpec => record(title, [f("rows", list(rowSpec(`${title}Row`, entity, patch)), () => [])], true);

export const REMODELING_NO_PATCH_SPEC: RecordSpec = record("RemodelingNoPatch", []);
export const FRAME_MEMBERS_SPEC: RecordSpec = membersSpec("RemodelingFrameMembers", () => FRAME_REF_SPEC);
export const OBSERVATION_MEMBERS_SPEC: RecordSpec = membersSpec("RemodelingObservationMembers", () => GCP_OBSERVATION_SPEC);
export const ASSIGNED_VIDEO_SOURCE_SPEC: RecordSpec = assignedSpec("RemodelingAssignedVideoSource", () => VIDEO_SOURCE_SPEC);
export const MEDIA_STREAM_PATCH_SPEC: RecordSpec = record("MediaStreamPatch", [f("sync_offset_ms", opt(f64), nothing), f("source", opt(rec(() => ASSIGNED_VIDEO_SOURCE_SPEC)), nothing), f("frames", opt(rec(() => FRAME_MEMBERS_SPEC)), nothing)], true);
export const GCP_PATCH_SPEC: RecordSpec = record("GroundControlPointPatch", [f("observations", opt(rec(() => OBSERVATION_MEMBERS_SPEC)), nothing)], true);
export const REMODELING_ASSET_ENTRY_SPEC: RecordSpec = record("RemodelingAssetEntry", [f("key", text, required), f("child", rec(() => ARTIFACT_CHILD_SPEC), required)]);
export const REMODELING_STREAMS_DELTA_SPEC: RecordSpec = rowsSpec("RemodelingStreamsDelta", () => MEDIA_STREAM_SPEC, () => MEDIA_STREAM_PATCH_SPEC);
export const REMODELING_GCPS_DELTA_SPEC: RecordSpec = rowsSpec("RemodelingGcpsDelta", () => GROUND_CONTROL_POINT_SPEC, () => GCP_PATCH_SPEC);
export const REMODELING_CAMERAS_DELTA_SPEC: RecordSpec = rowsSpec("RemodelingCamerasDelta", () => CAMERA_CALIBRATION_SPEC, () => REMODELING_NO_PATCH_SPEC);
export const REMODELING_RIG_DELTA_SPEC: RecordSpec = rowsSpec("RemodelingRigDelta", () => RIG_EXTRINSIC_SPEC, () => REMODELING_NO_PATCH_SPEC);
export const REMODELING_ASSETS_DELTA_SPEC: RecordSpec = rowsSpec("RemodelingAssetsDelta", () => REMODELING_ASSET_ENTRY_SPEC, () => REMODELING_NO_PATCH_SPEC);
export const REMODELING_CALIBRATION_DELTA_SPEC: RecordSpec = record("RemodelingCalibrationDelta", [f("cameras", rec(() => REMODELING_CAMERAS_DELTA_SPEC), () => ({ rows: [] })), f("rig", rec(() => REMODELING_RIG_DELTA_SPEC), () => ({ rows: [] }))], true);
export const REMODELING_CONTENT_HEADER_SPEC: RecordSpec = record("RemodelingContentHeader", [f("kind", text, required), f("mime", opt(text), nothing), f("width", uint, required), f("height", uint, required)]);
export const REMODELING_CONTENT_DELTA_SPEC: RecordSpec = record(
  "RemodelingContentDelta",
  [
    f(
      "rows",
      list({
        k: "tagged",
        tag: "row",
        variants: [
          { name: "append", spec: () => record("RemodelingContentAppend", [f("id", text, required), f("header", opt(rec(() => REMODELING_CONTENT_HEADER_SPEC)), nothing), f("chunks", list({ k: "bytes" }), required)]) },
          { name: "truncate", spec: () => record("RemodelingContentTruncate", [f("id", text, required), f("from", u64, required)]) },
        ],
      }),
      () => [],
    ),
  ],
  true,
);
export const REMODELING_PARAMS_DIFF_SPEC: RecordSpec = record(
  "RemodelingParamsDiff",
  [
    f("ingest", opt(rec(() => INGEST_PARAMS_SPEC)), nothing),
    f("feature", opt(rec(() => FEATURE_PARAMS_SPEC)), nothing),
    f("matching", opt(rec(() => MATCH_PARAMS_SPEC)), nothing),
    f("sfm", opt(rec(() => SFM_PARAMS_SPEC)), nothing),
    f("dense", opt(rec(() => DENSE_PARAMS_SPEC)), nothing),
    f("mesh", opt(rec(() => MESH_PARAMS_SPEC)), nothing),
    f("motion", opt(rec(() => MOTION_PARAMS_SPEC)), nothing),
    f("geo", opt(rec(() => GEO_PARAMS_SPEC)), nothing),
  ],
  true,
);
const assignedOf = (title: string, of: () => RecordSpec): ValueSpec => opt(rec(() => assignedSpec(title, of)));
export const REMODELING_RESULTS_DIFF_SPEC: RecordSpec = record(
  "RemodelingResultsDiff",
  [
    f("sparse", assignedOf("RemodelingAssignedSparseCloud", () => SPARSE_CLOUD_SPEC), nothing),
    f("dense", assignedOf("RemodelingAssignedDenseCloud", () => DENSE_CLOUD_SPEC), nothing),
    f("mesh", opt(rec(() => REMODELING_MESH_SPEC)), nothing),
    f("trajectory", assignedOf("RemodelingAssignedCameraTrajectory", () => CAMERA_TRAJECTORY_SPEC), nothing),
    f("tracks", opt(list(rec(() => MOTION_TRACK_SUMMARY_SPEC))), nothing),
    f("geo", assignedOf("RemodelingAssignedGeoProducts", () => GEO_PRODUCTS_SPEC), nothing),
    f("qc", assignedOf("RemodelingAssignedQcReport", () => QC_REPORT_SPEC), nothing),
  ],
  true,
);

export const REMODELING_DIFF_SPEC: RecordSpec = {
  title: "RemodelingDiff",
  serdeDefault: true,
  fields: [
    f("schema", opt(text), nothing),
    f("id", opt(text), nothing),
    f("streams", opt(rec(() => REMODELING_STREAMS_DELTA_SPEC)), nothing),
    f("assets", opt(rec(() => REMODELING_ASSETS_DELTA_SPEC)), nothing),
    f("durable_artifacts", opt(rec(() => REMODELING_CONTENT_DELTA_SPEC)), nothing),
    f("calibration", opt(rec(() => REMODELING_CALIBRATION_DELTA_SPEC)), nothing),
    f("params", opt(rec(() => REMODELING_PARAMS_DIFF_SPEC)), nothing),
    f("gcps", opt(rec(() => REMODELING_GCPS_DELTA_SPEC)), nothing),
    f("results", opt(rec(() => REMODELING_RESULTS_DIFF_SPEC)), nothing),
  ],
};
//#endregion 🔖️Spec

//#region 🔖️Builders
/** 🫙 `RemodelingDiff::default()` — every lane untouched. */
export const emptyRemodelingDiff = (): RemodelingDiff => defaultsOf(REMODELING_DIFF_SPEC) as unknown as RemodelingDiff;

/** 🖊️ The lanes a diff actually writes — the load-bearing shape assertion for a fixture. */
export const remodelingDiffLanes = (diff: RemodelingDiff): string[] =>
  REMODELING_DIFF_SPEC.fields.map((field) => field.name).filter((name) => {
    const key = name.split("_").map((part, index) => (index === 0 ? part : part.charAt(0).toUpperCase() + part.slice(1))).join("");
    return (diff as unknown as Record<string, unknown>)[key] !== null;
  });

/** 🏗️ A single-lane diff. */
export const remodelingLane = (lane: Partial<RemodelingDiff>): RemodelingDiff => ({ ...emptyRemodelingDiff(), ...lane });
export const calibrationRows = (delta: Partial<RemodelingCalibrationDelta>): RemodelingCalibrationDelta => ({ cameras: { rows: [] }, rig: { rows: [] }, ...delta });
export const noPatch = (): RemodelingNoPatch => ({});
export const streamPatch = (patch: Partial<MediaStreamPatch>): MediaStreamPatch => ({ syncOffsetMs: null, source: null, frames: null, ...patch });
export const assigned = <T,>(value: T): RemodelingAssigned<T> => ({ value });
export const resultsDiff = (patch: Partial<RemodelingResultsDiff>): RemodelingResultsDiff => ({ sparse: null, dense: null, mesh: null, trajectory: null, tracks: null, geo: null, qc: null, ...patch });
export const paramsDiff = (patch: Partial<RemodelingParamsDiff>): RemodelingParamsDiff => ({ ingest: null, feature: null, matching: null, sfm: null, dense: null, mesh: null, motion: null, geo: null, ...patch });
//#endregion 🔖️Builders

//#region 🔖️Apply
/** 🚫️ A diff the base cannot take: malformed rows are refused, never skipped. */
export class RemodelingDiffApplyError extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const refuseApply = (at: string, why: string): never => {
  throw new RemodelingDiffApplyError(at, why);
};

const numberOf = (value: Binary32 | Binary64): number => (typeof value.bits === "bigint" ? binary64Value(value as Binary64) : binary32Value(value as Binary32));

const same = (left: unknown, right: unknown): boolean => {
  if (left === right) return true;
  if (left === null || right === null || typeof left !== "object" || typeof right !== "object") return false;
  if (Array.isArray(left) !== Array.isArray(right)) return false;
  const leftKeys = Object.keys(left as object);
  const rightKeys = Object.keys(right as object);
  if (leftKeys.length !== rightKeys.length) return false;
  return leftKeys.every((key) => key in (right as object) && same((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key]));
};

const compareText = (left: string, right: string): number => (left < right ? -1 : left > right ? 1 : 0);
const compareNumber = (left: number, right: number): number => (left < right ? -1 : left > right ? 1 : 0);

const frameRank = (left: FrameRef, right: FrameRef): number => compareNumber(left.index, right.index) || compareText(left.assetId, right.assetId) || compareNumber(numberOf(left.timestampMs), numberOf(right.timestampMs));
const observationRank = (left: GcpObservation, right: GcpObservation): number =>
  compareText(left.streamId, right.streamId) || compareNumber(left.frameIndex, right.frameIndex) || compareNumber(numberOf(left.pixel[0]), numberOf(right.pixel[0])) || compareNumber(numberOf(left.pixel[1]), numberOf(right.pixel[1]));

function writeMembers<T>(list: readonly T[], members: RemodelingMembers<T>, rank: (left: T, right: T) => number, at: string): T[] {
  const next = [...list];
  for (const member of members.removed) {
    const position = next.findIndex((item) => same(item, member));
    if (position < 0) refuseApply(`${at}.removed`, "removed member does not exist");
    next.splice(position, 1);
  }
  for (const member of members.added) {
    let position = 0;
    while (position < next.length && rank(next[position]!, member) < 0) position += 1;
    next.splice(position, 0, structuredClone(member));
  }
  return next;
}

function writeStreamPatch(stream: MediaStream, patch: MediaStreamPatch, at: string): MediaStream {
  const next = structuredClone(stream);
  if (patch.syncOffsetMs !== null) next.syncOffsetMs = patch.syncOffsetMs;
  if (patch.source !== null) next.source = structuredClone(patch.source.value);
  if (patch.frames !== null) next.frames = writeMembers(next.frames, patch.frames, frameRank, `${at}.frames`);
  return next;
}

function writeGcpPatch(gcp: GroundControlPoint, patch: GroundControlPointPatch, at: string): GroundControlPoint {
  const next = structuredClone(gcp);
  if (patch.observations !== null) next.observations = writeMembers(next.observations, patch.observations, observationRank, `${at}.observations`);
  return next;
}

function writeRows<E, P>(items: readonly E[], delta: RemodelingRows<E, P>, key: (entity: E) => string, patched: (entity: E, patch: P, at: string) => E, at: string): E[] {
  const next = [...items];
  delta.rows.forEach((entry, row) => {
    const where = `${at}.rows.${row}`;
    switch (entry.row) {
      case "insert": {
        const id = key(entry.entity);
        if (next.some((item) => key(item) === id)) refuseApply(`${where}.entity`, "inserted member key already exists");
        let position = 0;
        while (position < next.length && key(next[position]!) < id) position += 1;
        next.splice(position, 0, structuredClone(entry.entity));
        break;
      }
      case "replace": {
        const id = key(entry.entity);
        const position = next.findIndex((item) => key(item) === id);
        if (position < 0) refuseApply(`${where}.entity`, "replaced member does not exist");
        next[position] = structuredClone(entry.entity);
        break;
      }
      case "remove": {
        const position = next.findIndex((item) => key(item) === entry.key);
        if (position < 0) refuseApply(`${where}.key`, "removed member does not exist");
        next.splice(position, 1);
        break;
      }
      case "patch": {
        const position = next.findIndex((item) => key(item) === entry.key);
        if (position < 0) refuseApply(`${where}.key`, "patched member does not exist");
        next[position] = patched(next[position]!, entry.patch, `${where}.patch`);
        break;
      }
    }
  });
  return next;
}

const keepWhole = <E,>(entity: E): E => entity;

function writeContent(store: Record<string, RemodelingDurableArtifact>, delta: RemodelingContentDelta, at: string): Record<string, RemodelingDurableArtifact> {
  const next = structuredClone(store);
  delta.rows.forEach((entry, row) => {
    const where = `${at}.rows.${row}`;
    if (entry.row === "append") {
      if (entry.header !== null) {
        if (Object.hasOwn(next, entry.id)) refuseApply(`${where}.id`, "created content already exists");
        next[entry.id] = { kind: entry.header.kind, mime: entry.header.mime, width: entry.header.width, height: entry.header.height, chunks: structuredClone(entry.chunks) };
        return;
      }
      const artifact = Object.hasOwn(next, entry.id) ? next[entry.id] : undefined;
      if (artifact === undefined) refuseApply(`${where}.id`, "extended content does not exist");
      artifact!.chunks.push(...structuredClone(entry.chunks));
      return;
    }
    const artifact = Object.hasOwn(next, entry.id) ? next[entry.id] : undefined;
    if (artifact === undefined) refuseApply(`${where}.id`, "truncated content does not exist");
    if (entry.from >= BigInt(artifact!.chunks.length)) refuseApply(`${where}.from`, "truncation point is not inside the stored leaves");
    if (entry.from === 0n) delete next[entry.id];
    else artifact!.chunks = artifact!.chunks.slice(0, Number(entry.from));
  });
  return next;
}

/** 🩹 `MutationDiff::apply` — rows and slots written lane by lane over a copy of `snapshot`; a row the base cannot take throws. */
export function applyRemodelingDiff(diff: RemodelingDiff, snapshot: RemodelingSnapshot): RemodelingSnapshot {
  const next = structuredClone(snapshot);
  if (diff.schema !== null) next.schema = diff.schema;
  if (diff.id !== null) next.id = diff.id;
  if (diff.streams !== null) next.streams = writeRows(snapshot.streams, diff.streams, (stream) => stream.id, writeStreamPatch, "streams");
  if (diff.assets !== null) {
    const entries = Object.keys(snapshot.assets).sort(compareText).map((key): RemodelingAssetEntry => ({ key, child: snapshot.assets[key]! }));
    next.assets = Object.fromEntries(writeRows(entries, diff.assets, (entry) => entry.key, keepWhole, "assets").map((entry) => [entry.key, entry.child]));
  }
  if (diff.durableArtifacts !== null) next.durableArtifacts = writeContent(snapshot.durableArtifacts, diff.durableArtifacts, "durableArtifacts");
  if (diff.calibration !== null) {
    next.calibration.cameras = writeRows(snapshot.calibration.cameras, diff.calibration.cameras, (camera) => camera.id, keepWhole, "calibration.cameras");
    next.calibration.rig = writeRows(snapshot.calibration.rig, diff.calibration.rig, (extrinsic) => extrinsic.cameraId, keepWhole, "calibration.rig");
  }
  if (diff.params !== null) for (const facet of ["ingest", "feature", "matching", "sfm", "dense", "mesh", "motion", "geo"] as const) if (diff.params[facet] !== null) (next.params as unknown as Record<string, unknown>)[facet] = structuredClone(diff.params[facet]);
  if (diff.gcps !== null) next.gcps = writeRows(snapshot.gcps, diff.gcps, (gcp) => gcp.id, writeGcpPatch, "gcps");
  if (diff.results !== null) {
    const results = diff.results;
    if (results.sparse !== null) next.results.sparse = structuredClone(results.sparse.value);
    if (results.dense !== null) next.results.dense = structuredClone(results.dense.value);
    if (results.mesh !== null) next.results.mesh = structuredClone(results.mesh);
    if (results.trajectory !== null) next.results.trajectory = structuredClone(results.trajectory.value);
    if (results.tracks !== null) next.results.tracks = structuredClone(results.tracks);
    if (results.geo !== null) next.results.geo = structuredClone(results.geo.value);
    if (results.qc !== null) next.results.qc = structuredClone(results.qc.value);
  }
  return next;
}
//#endregion 🔖️Apply

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class remodelRemodelingDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const remodelRemodelingDiffGuardReject = (at: string, why: string): never => {
  throw new remodelRemodelingDiffGuardRefusal(at, why);
};

type remodelRemodelingDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type remodelRemodelingDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type remodelRemodelingDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const remodelRemodelingDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : remodelRemodelingDiffGuardReject(at, "value is not an object");
export const remodelRemodelingDiffGuardArray = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return remodelRemodelingDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) remodelRemodelingDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) remodelRemodelingDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const remodelRemodelingDiffGuardString = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return remodelRemodelingDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) remodelRemodelingDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) remodelRemodelingDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) remodelRemodelingDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const remodelRemodelingDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : remodelRemodelingDiffGuardReject(at, "value is not a boolean"));
export const remodelRemodelingDiffGuardNumber = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return remodelRemodelingDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) remodelRemodelingDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) remodelRemodelingDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const remodelRemodelingDiffGuardInteger = (value: unknown, at: string, bounds: remodelRemodelingDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? remodelRemodelingDiffGuardNumber(value, at, bounds) : remodelRemodelingDiffGuardReject(at, "value is not an integer");
export const remodelRemodelingDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : remodelRemodelingDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const remodelRemodelingDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : remodelRemodelingDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

