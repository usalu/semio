/** 🧬️ Remodeling mutation vocabulary — TypeScript twin of `🧬️mutations/🦀️.rs` and of all
 *  thirty-five `<slug>/🔺️diff/🦀️.rs` leaves.
 *
 *  `RemodelingMutation` carries `#[serde(tag = "mutation", rename_all = "camelCase")]`, so the wire
 *  tag is the camelCase form of the Rust variant name (`createStream`), never the kebab-case
 *  `#[dsl(keyword)]` slug used for the directory names. Each payload's own fields are the camelCase
 *  rename of its Rust struct.
 *
 *  `remodelingMutationOutcome` is a second implementation of every leaf's `diff`, written from the
 *  Rust source rather than derived from it: the guard ORDER differs per leaf (some check the no-op
 *  first, some the invariant first) and that order is reproduced exactly, because it decides which
 *  message a refusal carries.
 */

import { applyRemodelingDiff, emptyRemodelingDiff, type RemodelingDiff, type RemodelingGcpList, type RemodelingMediaStreamList } from "../🔺️diff/🟦️.ts";
import {
  CAMERA_CALIBRATION_SPEC,
  CAMERA_TRAJECTORY_SPEC,
  DENSE_CLOUD_SPEC,
  DENSE_PARAMS_SPEC,
  FEATURE_PARAMS_SPEC,
  FRAME_REF_SPEC,
  GCP_OBSERVATION_SPEC,
  GEO_PARAMS_SPEC,
  GEO_PRODUCTS_SPEC,
  GROUND_CONTROL_POINT_SPEC,
  IMAGE_ASSET_SPEC,
  INGEST_PARAMS_SPEC,
  MATCH_PARAMS_SPEC,
  MEDIA_KINDS,
  MEDIA_STREAM_SPEC,
  MESH_PARAMS_SPEC,
  MOTION_PARAMS_SPEC,
  MOTION_TRACK_SUMMARY_SPEC,
  QC_REPORT_SPEC,
  RECONSTRUCTION_JOB_SPEC,
  REMODELING_MESH_SPEC,
  RIG_EXTRINSIC_SPEC,
  SFM_PARAMS_SPEC,
  SPARSE_CLOUD_SPEC,
  VIDEO_SOURCE_SPEC,
  camelOf,
  decodeRecord,
  type CalibrationState,
  type CameraCalibration,
  type CameraTrajectory,
  type DenseCloud,
  type DenseParams,
  type FeatureParams,
  type FieldSpec,
  type FrameRef,
  type GcpObservation,
  type GeoParams,
  type GeoProducts,
  type GroundControlPoint,
  type ImageAsset,
  type IngestParams,
  type MatchParams,
  type MediaKind,
  type MediaStream,
  type MeshParams,
  type MotionParams,
  type MotionTrackSummary,
  type QcReportSnapshot,
  type ReconstructionJob,
  type ReconstructionResults,
  type RecordSpec,
  type RemodelingAssetChild,
  type RemodelingDurableArtifact,
  type RemodelingMesh,
  type RemodelingSnapshot,
  type RigExtrinsic,
  type SfmParams,
  type SparseCloud,
  type ValueSpec,
  type VideoSource,
} from "../📸️snapshot/🟦️.ts";

//#region 🔖️Payloads
export interface CreateStream {
  stream: MediaStream;
}
export interface DeleteStream {
  id: string;
}
export interface ChangeStreamSync {
  id: string;
  newSyncOffsetMs: number;
}
export interface AddStreamFrame {
  id: string;
  frame: FrameRef;
  kind: MediaKind;
}
export interface RemoveStreamFrame {
  id: string;
  frameIndex: number;
}
export interface ReplaceStreamSource {
  id: string;
  source: VideoSource | null;
}
export interface CreateAsset {
  key: string;
  asset: ImageAsset;
}
export interface DeleteAsset {
  key: string;
}
export interface CreateCameraCalibration {
  camera: CameraCalibration;
}
export interface UpdateCameraCalibration {
  camera: CameraCalibration;
}
export interface DeleteCameraCalibration {
  cameraId: string;
}
export interface CreateRigExtrinsic {
  extrinsic: RigExtrinsic;
}
export interface DeleteRigExtrinsic {
  cameraId: string;
}
export interface UpdateRigExtrinsic {
  extrinsic: RigExtrinsic;
}
export interface CreateGcp {
  gcp: GroundControlPoint;
}
export interface DeleteGcp {
  id: string;
}
export interface AddGcpObservation {
  id: string;
  observation: GcpObservation;
}
export interface RemoveGcpObservation {
  id: string;
  observationIndex: number;
}
export interface UpdateIngestParams {
  params: IngestParams;
}
export interface UpdateFeatureParams {
  params: FeatureParams;
}
export interface UpdateMatchParams {
  params: MatchParams;
}
export interface UpdateSfmParams {
  params: SfmParams;
}
export interface UpdateDenseParams {
  params: DenseParams;
}
export interface UpdateMeshParams {
  params: MeshParams;
}
export interface UpdateMotionParams {
  params: MotionParams;
}
export interface UpdateGeoParams {
  params: GeoParams;
}
export interface ReplaceJob {
  job: ReconstructionJob;
}
export interface ReplaceSparse {
  sparse: SparseCloud | null;
}
export interface ReplaceDense {
  dense: DenseCloud | null;
}
export interface ReplaceMeshResult {
  mesh: RemodelingMesh;
}
export interface ReplaceTrajectory {
  trajectory: CameraTrajectory | null;
}
export interface ReplaceTracks {
  tracks: MotionTrackSummary[];
}
export interface ReplaceGeoProducts {
  geo: GeoProducts | null;
}
export interface ReplaceQc {
  qc: QcReportSnapshot | null;
}
export interface ReconstructionAssetCommit {
  id: string;
  asset: ImageAsset;
}
export interface CommitReconstruction {
  job: ReconstructionJob;
  sparse: SparseCloud | null;
  trajectory: CameraTrajectory | null;
  mesh: RemodelingMesh | null;
  geo: GeoProducts | null;
  qc: QcReportSnapshot | null;
  assets: ReconstructionAssetCommit[];
}

export type RemodelingMutationTag =
  | "createStream"
  | "deleteStream"
  | "changeStreamSync"
  | "addStreamFrame"
  | "removeStreamFrame"
  | "replaceStreamSource"
  | "createAsset"
  | "deleteAsset"
  | "createCameraCalibration"
  | "updateCameraCalibration"
  | "deleteCameraCalibration"
  | "createRigExtrinsic"
  | "deleteRigExtrinsic"
  | "updateRigExtrinsic"
  | "createGcp"
  | "deleteGcp"
  | "addGcpObservation"
  | "removeGcpObservation"
  | "updateIngestParams"
  | "updateFeatureParams"
  | "updateMatchParams"
  | "updateSfmParams"
  | "updateDenseParams"
  | "updateMeshParams"
  | "updateMotionParams"
  | "updateGeoParams"
  | "replaceJob"
  | "replaceSparse"
  | "replaceDense"
  | "replaceMeshResult"
  | "replaceTrajectory"
  | "replaceTracks"
  | "replaceGeoProducts"
  | "replaceQc"
  | "commitReconstruction";

export type RemodelingMutation =
  | ({ mutation: "createStream" } & CreateStream)
  | ({ mutation: "deleteStream" } & DeleteStream)
  | ({ mutation: "changeStreamSync" } & ChangeStreamSync)
  | ({ mutation: "addStreamFrame" } & AddStreamFrame)
  | ({ mutation: "removeStreamFrame" } & RemoveStreamFrame)
  | ({ mutation: "replaceStreamSource" } & ReplaceStreamSource)
  | ({ mutation: "createAsset" } & CreateAsset)
  | ({ mutation: "deleteAsset" } & DeleteAsset)
  | ({ mutation: "createCameraCalibration" } & CreateCameraCalibration)
  | ({ mutation: "updateCameraCalibration" } & UpdateCameraCalibration)
  | ({ mutation: "deleteCameraCalibration" } & DeleteCameraCalibration)
  | ({ mutation: "createRigExtrinsic" } & CreateRigExtrinsic)
  | ({ mutation: "deleteRigExtrinsic" } & DeleteRigExtrinsic)
  | ({ mutation: "updateRigExtrinsic" } & UpdateRigExtrinsic)
  | ({ mutation: "createGcp" } & CreateGcp)
  | ({ mutation: "deleteGcp" } & DeleteGcp)
  | ({ mutation: "addGcpObservation" } & AddGcpObservation)
  | ({ mutation: "removeGcpObservation" } & RemoveGcpObservation)
  | ({ mutation: "updateIngestParams" } & UpdateIngestParams)
  | ({ mutation: "updateFeatureParams" } & UpdateFeatureParams)
  | ({ mutation: "updateMatchParams" } & UpdateMatchParams)
  | ({ mutation: "updateSfmParams" } & UpdateSfmParams)
  | ({ mutation: "updateDenseParams" } & UpdateDenseParams)
  | ({ mutation: "updateMeshParams" } & UpdateMeshParams)
  | ({ mutation: "updateMotionParams" } & UpdateMotionParams)
  | ({ mutation: "updateGeoParams" } & UpdateGeoParams)
  | ({ mutation: "replaceJob" } & ReplaceJob)
  | ({ mutation: "replaceSparse" } & ReplaceSparse)
  | ({ mutation: "replaceDense" } & ReplaceDense)
  | ({ mutation: "replaceMeshResult" } & ReplaceMeshResult)
  | ({ mutation: "replaceTrajectory" } & ReplaceTrajectory)
  | ({ mutation: "replaceTracks" } & ReplaceTracks)
  | ({ mutation: "replaceGeoProducts" } & ReplaceGeoProducts)
  | ({ mutation: "replaceQc" } & ReplaceQc);

/** 🧬️ Every tag including `commitReconstruction`, whose payload the wire union above omits. */
export type RemodelingAnyMutation = RemodelingMutation | ({ mutation: "commitReconstruction" } & CommitReconstruction);

export interface RemodelingMutationEnvelope {
  mutation: RemodelingMutationTag;
}
//#endregion 🔖️Payloads

//#region 🔖️Spec
const f = (name: string, spec: ValueSpec, dflt: () => unknown, extra: Partial<FieldSpec> = {}): FieldSpec => ({ name, spec, dflt, ...extra });
const text = { k: "text" } as const;
const uint = { k: "uint" } as const;
const f64 = { k: "f64" } as const;
const opt = (of: ValueSpec): ValueSpec => ({ k: "opt", of });
const list = (of: ValueSpec): ValueSpec => ({ k: "list", of });
const rec = (of: () => RecordSpec): ValueSpec => ({ k: "rec", of });
const nothing = () => null;
const required = () => {
  throw new Error("payload field has no Rust default");
};

const payload = (title: string, fields: FieldSpec[]): RecordSpec => ({ title, serdeDefault: false, fields });

export const RECONSTRUCTION_ASSET_COMMIT_SPEC: RecordSpec = payload("ReconstructionAssetCommit", [f("id", text, required), f("asset", rec(() => IMAGE_ASSET_SPEC), required)]);

/** 🧬️ One `RecordSpec` per wire tag; the `mutation` tag itself is handled by the decoder. */
export const REMODELING_MUTATION_SPECS: Record<RemodelingMutationTag, RecordSpec> = {
  createStream: payload("CreateStream", [f("stream", rec(() => MEDIA_STREAM_SPEC), required)]),
  deleteStream: payload("DeleteStream", [f("id", text, required)]),
  changeStreamSync: payload("ChangeStreamSync", [f("id", text, required), f("new_sync_offset_ms", f64, required)]),
  addStreamFrame: payload("AddStreamFrame", [f("id", text, required), f("frame", rec(() => FRAME_REF_SPEC), required), f("kind", { k: "enum", of: MEDIA_KINDS }, required)]),
  removeStreamFrame: payload("RemoveStreamFrame", [f("id", text, required), f("frame_index", uint, required)]),
  replaceStreamSource: payload("ReplaceStreamSource", [f("id", text, required), f("source", opt(rec(() => VIDEO_SOURCE_SPEC)), nothing, { jsonOptional: true })]),
  createAsset: payload("CreateAsset", [f("key", text, required), f("asset", rec(() => IMAGE_ASSET_SPEC), required)]),
  deleteAsset: payload("DeleteAsset", [f("key", text, required)]),
  createCameraCalibration: payload("CreateCameraCalibration", [f("camera", rec(() => CAMERA_CALIBRATION_SPEC), required)]),
  updateCameraCalibration: payload("UpdateCameraCalibration", [f("camera", rec(() => CAMERA_CALIBRATION_SPEC), required)]),
  deleteCameraCalibration: payload("DeleteCameraCalibration", [f("camera_id", text, required)]),
  createRigExtrinsic: payload("CreateRigExtrinsic", [f("extrinsic", rec(() => RIG_EXTRINSIC_SPEC), required)]),
  deleteRigExtrinsic: payload("DeleteRigExtrinsic", [f("camera_id", text, required)]),
  updateRigExtrinsic: payload("UpdateRigExtrinsic", [f("extrinsic", rec(() => RIG_EXTRINSIC_SPEC), required)]),
  createGcp: payload("CreateGcp", [f("gcp", rec(() => GROUND_CONTROL_POINT_SPEC), required)]),
  deleteGcp: payload("DeleteGcp", [f("id", text, required)]),
  addGcpObservation: payload("AddGcpObservation", [f("id", text, required), f("observation", rec(() => GCP_OBSERVATION_SPEC), required)]),
  removeGcpObservation: payload("RemoveGcpObservation", [f("id", text, required), f("observation_index", uint, required)]),
  updateIngestParams: payload("UpdateIngestParams", [f("params", rec(() => INGEST_PARAMS_SPEC), required)]),
  updateFeatureParams: payload("UpdateFeatureParams", [f("params", rec(() => FEATURE_PARAMS_SPEC), required)]),
  updateMatchParams: payload("UpdateMatchParams", [f("params", rec(() => MATCH_PARAMS_SPEC), required)]),
  updateSfmParams: payload("UpdateSfmParams", [f("params", rec(() => SFM_PARAMS_SPEC), required)]),
  updateDenseParams: payload("UpdateDenseParams", [f("params", rec(() => DENSE_PARAMS_SPEC), required)]),
  updateMeshParams: payload("UpdateMeshParams", [f("params", rec(() => MESH_PARAMS_SPEC), required)]),
  updateMotionParams: payload("UpdateMotionParams", [f("params", rec(() => MOTION_PARAMS_SPEC), required)]),
  updateGeoParams: payload("UpdateGeoParams", [f("params", rec(() => GEO_PARAMS_SPEC), required)]),
  replaceJob: payload("ReplaceJob", [f("job", rec(() => RECONSTRUCTION_JOB_SPEC), required)]),
  replaceSparse: payload("ReplaceSparse", [f("sparse", opt(rec(() => SPARSE_CLOUD_SPEC)), nothing, { jsonOptional: true })]),
  replaceDense: payload("ReplaceDense", [f("dense", opt(rec(() => DENSE_CLOUD_SPEC)), nothing, { jsonOptional: true })]),
  replaceMeshResult: payload("ReplaceMeshResult", [f("mesh", rec(() => REMODELING_MESH_SPEC), required)]),
  replaceTrajectory: payload("ReplaceTrajectory", [f("trajectory", opt(rec(() => CAMERA_TRAJECTORY_SPEC)), nothing, { jsonOptional: true })]),
  replaceTracks: payload("ReplaceTracks", [f("tracks", list(rec(() => MOTION_TRACK_SUMMARY_SPEC)), required)]),
  replaceGeoProducts: payload("ReplaceGeoProducts", [f("geo", opt(rec(() => GEO_PRODUCTS_SPEC)), nothing, { jsonOptional: true })]),
  replaceQc: payload("ReplaceQc", [f("qc", opt(rec(() => QC_REPORT_SPEC)), nothing, { jsonOptional: true })]),
  commitReconstruction: payload("CommitReconstruction", [
    f("job", rec(() => RECONSTRUCTION_JOB_SPEC), required),
    f("sparse", opt(rec(() => SPARSE_CLOUD_SPEC)), nothing),
    f("trajectory", opt(rec(() => CAMERA_TRAJECTORY_SPEC)), nothing),
    f("mesh", opt(rec(() => REMODELING_MESH_SPEC)), nothing),
    f("geo", opt(rec(() => GEO_PRODUCTS_SPEC)), nothing),
    f("qc", opt(rec(() => QC_REPORT_SPEC)), nothing),
    f("assets", list(rec(() => RECONSTRUCTION_ASSET_COMMIT_SPEC)), required),
  ]),
};

export const REMODELING_MUTATION_TAGS = Object.keys(REMODELING_MUTATION_SPECS) as RemodelingMutationTag[];

/** 🦠️ Decodes a parsed RFC 8259 value into a validated tagged mutation. */
export function decodeRemodelingMutation(json: unknown): RemodelingAnyMutation {
  if (typeof json !== "object" || json === null || Array.isArray(json)) throw new Error(`mutation: expected an object, got ${JSON.stringify(json)}`);
  const source = json as Record<string, unknown>;
  const tag = source.mutation;
  if (typeof tag !== "string" || !(tag in REMODELING_MUTATION_SPECS)) throw new Error(`mutation: unknown tag ${JSON.stringify(tag)}`);
  const { mutation: _tag, ...rest } = source;
  const body = decodeRecord(rest, REMODELING_MUTATION_SPECS[tag as RemodelingMutationTag], `${tag}`);
  return { mutation: tag, ...body } as unknown as RemodelingAnyMutation;
}
//#endregion 🔖️Spec

//#region 🔖️AssetHandles
const SIP_MASK = (1n << 64n) - 1n;
const rotl = (value: bigint, bits: bigint): bigint => ((value << bits) | (value >> (64n - bits))) & SIP_MASK;

/** #️⃣ Rust `std::collections::hash_map::DefaultHasher` — SipHash-1-3 keyed `(0, 0)`. */
class DefaultHasher {
  private v0 = 0x736f6d6570736575n;
  private v1 = 0x646f72616e646f6dn;
  private v2 = 0x6c7967656e657261n;
  private v3 = 0x7465646279746573n;
  private tail: number[] = [];
  private length = 0;

  private round(): void {
    this.v0 = (this.v0 + this.v1) & SIP_MASK;
    this.v1 = rotl(this.v1, 13n) ^ this.v0;
    this.v0 = rotl(this.v0, 32n);
    this.v2 = (this.v2 + this.v3) & SIP_MASK;
    this.v3 = rotl(this.v3, 16n) ^ this.v2;
    this.v0 = (this.v0 + this.v3) & SIP_MASK;
    this.v3 = rotl(this.v3, 21n) ^ this.v0;
    this.v2 = (this.v2 + this.v1) & SIP_MASK;
    this.v1 = rotl(this.v1, 17n) ^ this.v2;
    this.v2 = rotl(this.v2, 32n);
  }

  private block(word: bigint): void {
    this.v3 ^= word;
    this.round();
    this.v0 ^= word;
  }

  write(bytes: ArrayLike<number>): void {
    for (let index = 0; index < bytes.length; index += 1) {
      this.tail.push(bytes[index]);
      this.length += 1;
      if (this.tail.length === 8) {
        let word = 0n;
        for (let byte = 7; byte >= 0; byte -= 1) word = (word << 8n) | BigInt(this.tail[byte]);
        this.block(word);
        this.tail = [];
      }
    }
  }

  /** 🔤 `impl Hash for str` — the bytes, then a `0xff` terminator. */
  writeStr(value: string): void {
    this.write(new TextEncoder().encode(value));
    this.write([0xff]);
  }

  finish(): bigint {
    let last = BigInt(this.length & 0xff) << 56n;
    for (let byte = this.tail.length - 1; byte >= 0; byte -= 1) last |= BigInt(this.tail[byte]) << BigInt(8 * byte);
    this.block(last);
    this.v2 ^= 0xffn;
    this.round();
    this.round();
    this.round();
    return (this.v0 ^ this.v1 ^ this.v2 ^ this.v3) & SIP_MASK;
  }
}

const REMODELING_DURABLE_CHUNK_RAW_BYTES = 4096;
const REMODELING_RASTER_CONTENT_BYTES = 1_114_112;
const ASSET_STAGE_PREFIX = "__remodeling_asset_stage__:";
const MESH_STAGE_PREFIX = "__remodeling_mesh_stage__:";
const CONTENT_HANDLE_PREFIX = "remodeling-content:";
const MESH_STAGE_HANDLE_PREFIX = "mesh-stage:";

/** 🚧 A path this twin deliberately does not model — the process-global staging registries. */
export class RemodelingUnsupportedError extends Error {
  constructor(readonly reason: string) {
    super(reason);
    this.name = "RemodelingUnsupportedError";
  }
}

/** 🕸️ `image_asset_child_handle` — content-addressed CHILD handle for one bounded durable asset. */
export function imageAssetChildHandle(assetId: string, asset: ImageAsset): RemodelingAssetChild {
  const hasher = new DefaultHasher();
  hasher.writeStr(asset.mime);
  hasher.writeStr(asset.data);
  const childId = `remodeling-asset-${hasher.finish().toString(16).padStart(16, "0")}`;
  return { childId, target: { artifactId: `${assetId}-image`, dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "image" } } };
}

const base64Bytes = (value: string): Uint8Array | null => {
  try {
    const binary = atob(value);
    const bytes = new Uint8Array(binary.length);
    for (let index = 0; index < binary.length; index += 1) bytes[index] = binary.charCodeAt(index);
    return bytes;
  } catch {
    return null;
  }
};

const base64Text = (bytes: Uint8Array): string => {
  let binary = "";
  for (const byte of bytes) binary += String.fromCharCode(byte);
  return btoa(binary);
};

/** 🧩️ `durable_remodeling_asset` — bounded 4 KiB leaves, or `null` when the payload is too large. */
export function durableRemodelingAsset(asset: ImageAsset): RemodelingDurableArtifact | null {
  const bytes = base64Bytes(asset.data);
  if (bytes === null || bytes.length > REMODELING_RASTER_CONTENT_BYTES) return null;
  const chunks: string[] = [];
  for (let offset = 0; offset < bytes.length; offset += REMODELING_DURABLE_CHUNK_RAW_BYTES) chunks.push(base64Text(bytes.subarray(offset, offset + REMODELING_DURABLE_CHUNK_RAW_BYTES)));
  return { kind: "image", mime: asset.mime, width: asset.width, height: asset.height, chunks };
}
//#endregion 🔖️AssetHandles

//#region 🔖️Outcome
export type RemodelingMessageSeverity = "info" | "warn" | "error" | "fatal";

export interface RemodelingMutationMessage {
  severity: RemodelingMessageSeverity;
  code: string;
  message: string;
  target: string[];
}

export interface RemodelingMutationOutcome {
  diff: RemodelingDiff;
  messages: RemodelingMutationMessage[];
}

const ok = (patch: Partial<RemodelingDiff>): RemodelingMutationOutcome => ({ diff: { ...emptyRemodelingDiff(), ...patch }, messages: [] });
const empty = (): RemodelingMutationOutcome => ({ diff: emptyRemodelingDiff(), messages: [] });
const noted = (outcome: RemodelingMutationOutcome, severity: RemodelingMessageSeverity, code: string, message: string, target: string[] = []): RemodelingMutationOutcome => ({
  diff: outcome.diff,
  messages: [...outcome.messages, { severity, code, message, target }],
});
const refuse = (severity: "error" | "fatal", code: string, message: string, target: string[] = []): RemodelingMutationOutcome => noted(empty(), severity, code, message, target);

const same = (left: unknown, right: unknown): boolean => {
  if (left === right) return true;
  if (left === null || right === null || typeof left !== "object" || typeof right !== "object") return false;
  if (Array.isArray(left) !== Array.isArray(right)) return false;
  const leftKeys = Object.keys(left as object);
  const rightKeys = Object.keys(right as object);
  if (leftKeys.length !== rightKeys.length) return false;
  return leftKeys.every((key) => key in (right as object) && same((left as Record<string, unknown>)[key], (right as Record<string, unknown>)[key]));
};

const clone = <T,>(value: T): T => structuredClone(value);
const streamList = (values: MediaStream[]): RemodelingMediaStreamList => ({ values });
const gcpList = (values: GroundControlPoint[]): RemodelingGcpList => ({ values });
const finite = (value: number): boolean => Number.isFinite(value);
/** 🔢️ Where a member belongs in a collection held in ascending key order: the count of members that
 * sort BEFORE the new one. Every keyed collection in this document is ordered — ids for streams,
 * cameras, rig entries and ground control points, `(index, assetId)` for a stream's frames,
 * `(streamId, frameIndex)` for a GCP's observations — which is what makes every delete/remove exactly
 * invertible by the create/add that pairs with it. */
const orderedIndex = <T,>(items: readonly T[], before: (item: T) => boolean): number => {
  let position = 0;
  while (position < items.length && before(items[position]!)) position += 1;
  return position;
};
const inserted = <T,>(items: readonly T[], at: number, value: T): T[] => [...items.slice(0, at), value, ...items.slice(at)];
const withoutKey = (store: Record<string, RemodelingDurableArtifact>, key: string | undefined): Record<string, RemodelingDurableArtifact> =>
  Object.fromEntries(Object.entries(store).filter(([name]) => name !== key));
const frameBefore = (frame: FrameRef, next: FrameRef): boolean => frame.index < next.index || (frame.index === next.index && frame.assetId < next.assetId);
const observationBefore = (observation: GcpObservation, next: GcpObservation): boolean =>
  observation.streamId < next.streamId || (observation.streamId === next.streamId && observation.frameIndex < next.frameIndex);
//#endregion 🔖️Outcome

//#region 🔖️Diffs
/** 🔺️ The second implementation of every leaf's `diff`, guard order included. */
export function remodelingMutationOutcome(base: RemodelingSnapshot, mutation: RemodelingMutation): RemodelingMutationOutcome {
  switch (mutation.mutation) {
    case "createStream": {
      const payloadStream = mutation.stream;
      if (base.streams.some((stream) => stream.id === payloadStream.id)) return refuse("fatal", "mutation.duplicate-id", `A stream with id "${payloadStream.id}" already exists.`, [payloadStream.id]);
      if (payloadStream.cameraId !== null && !base.calibration.cameras.some((camera) => camera.id === payloadStream.cameraId))
        return refuse("fatal", "mutation.invariant", `Stream "${payloadStream.id}" references unknown camera "${payloadStream.cameraId}".`, [payloadStream.id]);
      const streams = clone(base.streams);
      return ok({ streams: streamList(inserted(streams, orderedIndex(streams, (stream) => stream.id < payloadStream.id), clone(payloadStream))) });
    }
    case "deleteStream": {
      if (!base.streams.some((stream) => stream.id === mutation.id)) return refuse("error", "mutation.target-missing", `Stream "${mutation.id}" does not exist.`, [mutation.id]);
      const referencing = base.gcps.filter((gcp) => gcp.observations.some((observation) => observation.streamId === mutation.id)).map((gcp) => gcp.id);
      if (referencing.length > 0)
        return refuse("error", "mutation.referenced", `Stream "${mutation.id}" is still observed by ${referencing.length} ground control point(s); remove those observations first.`, referencing);
      return ok({ streams: streamList(clone(base.streams).filter((stream) => stream.id !== mutation.id)) });
    }
    case "changeStreamSync": {
      const existing = base.streams.find((stream) => stream.id === mutation.id);
      if (existing === undefined) return refuse("error", "mutation.target-missing", `Stream "${mutation.id}" does not exist.`, [mutation.id]);
      if (!finite(mutation.newSyncOffsetMs)) return refuse("fatal", "mutation.invariant", `Stream "${mutation.id}" sync offset must be finite, got ${mutation.newSyncOffsetMs}.`, [mutation.id]);
      if (existing.syncOffsetMs === mutation.newSyncOffsetMs) return noted(empty(), "warn", "mutation.no-op", `Stream "${mutation.id}" sync offset is already ${mutation.newSyncOffsetMs}ms.`);
      const streams = clone(base.streams).map((stream) => (stream.id === mutation.id ? { ...stream, syncOffsetMs: mutation.newSyncOffsetMs } : stream));
      return ok({ streams: streamList(streams) });
    }
    case "addStreamFrame": {
      const stream = base.streams.find((candidate) => candidate.id === mutation.id);
      if (stream === undefined) return refuse("error", "mutation.target-missing", `Stream "${mutation.id}" does not exist.`, [mutation.id]);
      if (mutation.kind !== stream.kind) return refuse("fatal", "mutation.invariant", `Stream "${mutation.id}" is not of the media kind this frame declares.`, [mutation.id]);
      if (stream.frames.some((frame) => same(frame, mutation.frame))) return noted(empty(), "warn", "mutation.no-op", `Stream "${mutation.id}" already has frame ${mutation.frame.index}.`);
      const streams = clone(base.streams).map((candidate) =>
        candidate.id === mutation.id ? { ...candidate, frames: inserted(candidate.frames, orderedIndex(candidate.frames, (frame) => frameBefore(frame, mutation.frame)), clone(mutation.frame)) } : candidate,
      );
      return ok({ streams: streamList(streams) });
    }
    case "removeStreamFrame": {
      const stream = base.streams.find((candidate) => candidate.id === mutation.id);
      if (stream === undefined) return refuse("error", "mutation.target-missing", `Stream "${mutation.id}" does not exist.`, [mutation.id]);
      if (mutation.frameIndex >= stream.frames.length) return refuse("error", "mutation.target-missing", `Stream "${mutation.id}" has no frame at index ${mutation.frameIndex}.`, [mutation.id]);
      const streams = clone(base.streams).map((candidate) => (candidate.id === mutation.id ? { ...candidate, frames: candidate.frames.filter((_, index) => index !== mutation.frameIndex) } : candidate));
      return ok({ streams: streamList(streams) });
    }
    case "replaceStreamSource": {
      if (!base.streams.some((stream) => stream.id === mutation.id)) return refuse("error", "mutation.target-missing", `Stream "${mutation.id}" does not exist.`, [mutation.id]);
      const streams = clone(base.streams).map((stream) => (stream.id === mutation.id ? { ...stream, source: clone(mutation.source) } : stream));
      return ok({ streams: streamList(streams) });
    }
    case "createAsset": {
      if (mutation.key.startsWith(ASSET_STAGE_PREFIX) || mutation.key.startsWith(MESH_STAGE_PREFIX))
        throw new RemodelingUnsupportedError(`createAsset staging keys drive the plugin's process-global chunk registry, which this twin does not model (key "${mutation.key}")`);
      if (mutation.asset.data.startsWith(CONTENT_HANDLE_PREFIX))
        return refuse("error", "mutation.invalid-asset-payload", "Private reconstruction staging handles are accepted only by CommitReconstruction.", [mutation.key]);
      const artifact = durableRemodelingAsset(mutation.asset);
      if (artifact === null) return refuse("error", "mutation.invalid-asset-payload", "The asset payload is malformed or exceeds its exact bounded envelope.", [mutation.key]);
      const handle = imageAssetChildHandle(mutation.key, mutation.asset);
      const durableArtifacts = { ...withoutKey(clone(base.durableArtifacts), base.assets[mutation.key]?.childId), [handle.childId]: artifact };
      return ok({ assets: { ...clone(base.assets), [mutation.key]: handle }, durableArtifacts });
    }
    case "deleteAsset": {
      if (mutation.key.startsWith(ASSET_STAGE_PREFIX) || mutation.key.startsWith(MESH_STAGE_PREFIX))
        throw new RemodelingUnsupportedError(`deleteAsset staging keys discard from the plugin's process-global chunk registry, which this twin does not model (key "${mutation.key}")`);
      if (!(mutation.key in base.assets)) return refuse("error", "mutation.target-missing", `Asset "${mutation.key}" does not exist.`, [mutation.key]);
      const referencing = base.streams.filter((stream) => stream.frames.some((frame) => frame.assetId === mutation.key)).map((stream) => stream.id);
      if (base.results.mesh.textureAssetId === mutation.key) referencing.push("results.mesh.textureAssetId");
      if (base.results.geo !== null)
        for (const [lane, assetId] of [["dsm", base.results.geo.dsmAssetId], ["dtm", base.results.geo.dtmAssetId], ["ortho", base.results.geo.orthoAssetId]] as const)
          if (assetId === mutation.key) referencing.push(`results.geo.${lane}AssetId`);
      if (referencing.length > 0) return refuse("error", "mutation.referenced", `Asset "${mutation.key}" is still referenced by ${referencing.length} place(s) in the document.`, referencing);
      const assets = clone(base.assets);
      const childId = assets[mutation.key]?.childId;
      delete assets[mutation.key];
      return ok({ assets, durableArtifacts: withoutKey(clone(base.durableArtifacts), childId) });
    }
    case "createCameraCalibration": {
      if (base.calibration.cameras.some((camera) => camera.id === mutation.camera.id)) return refuse("fatal", "mutation.duplicate-id", `A camera calibration with id "${mutation.camera.id}" already exists.`, [mutation.camera.id]);
      const cameras = clone(base.calibration.cameras);
      const calibration: CalibrationState = { ...clone(base.calibration), cameras: inserted(cameras, orderedIndex(cameras, (camera) => camera.id < mutation.camera.id), clone(mutation.camera)) };
      return ok({ calibration });
    }
    case "updateCameraCalibration": {
      const existing = base.calibration.cameras.find((camera) => camera.id === mutation.camera.id);
      if (existing === undefined) return refuse("error", "mutation.target-missing", `Camera calibration "${mutation.camera.id}" does not exist.`, [mutation.camera.id]);
      const camera = mutation.camera;
      const nonFinite = ![camera.fx, camera.fy, camera.cx, camera.cy, camera.skew].every(finite) || !camera.distortion.every(finite) || (camera.rmsReprojectionPx !== null && !finite(camera.rmsReprojectionPx));
      if (nonFinite) return refuse("fatal", "mutation.invariant", `Camera calibration "${camera.id}" has non-finite intrinsics or distortion.`, [camera.id]);
      if (same(existing, camera)) return noted(empty(), "warn", "mutation.no-op", `Camera calibration "${mutation.camera.id}" is already up to date.`);
      const calibration: CalibrationState = { ...clone(base.calibration), cameras: clone(base.calibration.cameras).map((candidate) => (candidate.id === camera.id ? clone(camera) : candidate)) };
      return ok({ calibration });
    }
    case "deleteCameraCalibration": {
      if (!base.calibration.cameras.some((camera) => camera.id === mutation.cameraId)) return refuse("error", "mutation.target-missing", `Camera calibration "${mutation.cameraId}" does not exist.`, [mutation.cameraId]);
      const referencing = base.streams.filter((stream) => stream.cameraId === mutation.cameraId).map((stream) => stream.id);
      if (base.calibration.rig.some((extrinsic) => extrinsic.cameraId === mutation.cameraId)) referencing.push(`calibration.rig.${mutation.cameraId}`);
      if (referencing.length > 0)
        return refuse("error", "mutation.referenced", `Camera calibration "${mutation.cameraId}" is still referenced by ${referencing.length} record(s); detach them first.`, referencing);
      const calibration: CalibrationState = { ...clone(base.calibration), cameras: clone(base.calibration.cameras).filter((camera) => camera.id !== mutation.cameraId) };
      return ok({ calibration });
    }
    case "createRigExtrinsic": {
      const cameraId = mutation.extrinsic.cameraId;
      if (base.calibration.rig.some((extrinsic) => extrinsic.cameraId === cameraId)) return refuse("fatal", "mutation.duplicate-id", `A rig extrinsic for camera "${cameraId}" already exists.`, [cameraId]);
      if (!base.calibration.cameras.some((camera) => camera.id === cameraId)) return refuse("fatal", "mutation.invariant", `Rig extrinsic references unknown camera "${cameraId}".`, [cameraId]);
      const rig = clone(base.calibration.rig);
      const calibration: CalibrationState = { ...clone(base.calibration), rig: inserted(rig, orderedIndex(rig, (entry) => entry.cameraId < cameraId), clone(mutation.extrinsic)) };
      return ok({ calibration });
    }
    case "deleteRigExtrinsic": {
      if (!base.calibration.rig.some((extrinsic) => extrinsic.cameraId === mutation.cameraId)) return refuse("error", "mutation.target-missing", `Rig extrinsic for camera "${mutation.cameraId}" does not exist.`, [mutation.cameraId]);
      const calibration: CalibrationState = { ...clone(base.calibration), rig: clone(base.calibration.rig).filter((extrinsic) => extrinsic.cameraId !== mutation.cameraId) };
      return ok({ calibration });
    }
    case "updateRigExtrinsic": {
      const extrinsic = mutation.extrinsic;
      const existing = base.calibration.rig.find((candidate) => candidate.cameraId === extrinsic.cameraId);
      if (existing === undefined) return refuse("error", "mutation.target-missing", `Rig extrinsic "${extrinsic.cameraId}" does not exist.`, [extrinsic.cameraId]);
      if (!extrinsic.rotationWxyz.every(finite) || !extrinsic.translationM.every(finite))
        return refuse("fatal", "mutation.invariant", `Rig extrinsic "${extrinsic.cameraId}" has a non-finite rotation or translation.`, [extrinsic.cameraId]);
      if (same(existing, extrinsic)) return noted(empty(), "warn", "mutation.no-op", `Rig extrinsic "${extrinsic.cameraId}" is unchanged.`);
      const calibration: CalibrationState = { ...clone(base.calibration), rig: clone(base.calibration.rig).map((candidate) => (candidate.cameraId === extrinsic.cameraId ? clone(extrinsic) : candidate)) };
      return ok({ calibration });
    }
    case "createGcp": {
      if (base.gcps.some((gcp) => gcp.id === mutation.gcp.id)) return refuse("fatal", "mutation.duplicate-id", `A GCP with id "${mutation.gcp.id}" already exists.`, [mutation.gcp.id]);
      const gcps = clone(base.gcps);
      return ok({ gcps: gcpList(inserted(gcps, orderedIndex(gcps, (gcp) => gcp.id < mutation.gcp.id), clone(mutation.gcp))) });
    }
    case "deleteGcp": {
      const gcp = base.gcps.find((candidate) => candidate.id === mutation.id);
      if (gcp === undefined) return refuse("error", "mutation.target-missing", `GCP "${mutation.id}" does not exist.`, [mutation.id]);
      const cascaded = gcp.observations.length;
      const outcome = ok({ gcps: gcpList(clone(base.gcps).filter((candidate) => candidate.id !== mutation.id)) });
      return cascaded === 0 ? outcome : noted(outcome, "info", "mutation.cascade", `Deleting GCP "${mutation.id}" also removed ${cascaded} observation(s).`);
    }
    case "addGcpObservation": {
      const gcp = base.gcps.find((candidate) => candidate.id === mutation.id);
      if (gcp === undefined) return refuse("error", "mutation.target-missing", `GCP "${mutation.id}" does not exist.`, [mutation.id]);
      if (!base.streams.some((stream) => stream.id === mutation.observation.streamId))
        return refuse("fatal", "mutation.invariant", `GCP "${mutation.id}" cannot be observed in unknown stream "${mutation.observation.streamId}".`, [mutation.observation.streamId]);
      if (gcp.observations.some((observation) => same(observation, mutation.observation))) return noted(empty(), "warn", "mutation.no-op", `GCP "${mutation.id}" already has this observation.`);
      const gcps = clone(base.gcps).map((candidate) =>
        candidate.id === mutation.id
          ? { ...candidate, observations: inserted(candidate.observations, orderedIndex(candidate.observations, (observation) => observationBefore(observation, mutation.observation)), clone(mutation.observation)) }
          : candidate,
      );
      return ok({ gcps: gcpList(gcps) });
    }
    case "removeGcpObservation": {
      const gcp = base.gcps.find((candidate) => candidate.id === mutation.id);
      if (gcp === undefined) return refuse("error", "mutation.target-missing", `GCP "${mutation.id}" does not exist.`, [mutation.id]);
      if (mutation.observationIndex >= gcp.observations.length) return refuse("error", "mutation.target-missing", `GCP "${mutation.id}" has no observation at index ${mutation.observationIndex}.`, [mutation.id]);
      const gcps = clone(base.gcps).map((candidate) => (candidate.id === mutation.id ? { ...candidate, observations: candidate.observations.filter((_, index) => index !== mutation.observationIndex) } : candidate));
      return ok({ gcps: gcpList(gcps) });
    }
    case "updateIngestParams": {
      const params = mutation.params;
      if (!finite(params.minSharpness) || params.minSharpness < 0 || params.maxFrames === 0 || params.frameSampleStride === 0)
        return refuse(
          "fatal",
          "mutation.invariant",
          `Ingest params need a finite non-negative min sharpness and positive max frames/frame sample stride (got min_sharpness=${params.minSharpness}, max_frames=${params.maxFrames}, frame_sample_stride=${params.frameSampleStride}).`,
        );
      if (same(params, base.params.ingest)) return noted(empty(), "warn", "mutation.no-op", "Ingest params are unchanged.");
      return ok({ params: { ...clone(base.params), ingest: clone(params) } });
    }
    case "updateFeatureParams": {
      const params = mutation.params;
      if (params.targetCount === 0 || !finite(params.edgeThreshold) || params.edgeThreshold < 0)
        return refuse("fatal", "mutation.invariant", `Feature params need a positive target count and a finite non-negative edge threshold (got target_count=${params.targetCount}, edge_threshold=${params.edgeThreshold}).`);
      if (same(params, base.params.feature)) return noted(empty(), "warn", "mutation.no-op", "Feature params are unchanged.");
      return ok({ params: { ...clone(base.params), feature: clone(params) } });
    }
    case "updateMatchParams": {
      const params = mutation.params;
      if (!finite(params.ratioTest) || params.ratioTest <= 0 || params.ratioTest > 1) return refuse("fatal", "mutation.invariant", `Match ratio test ${params.ratioTest} must be finite and within (0, 1].`);
      if (same(params, base.params.matching)) return noted(empty(), "warn", "mutation.no-op", "Matching params are unchanged.");
      return ok({ params: { ...clone(base.params), matching: clone(params) } });
    }
    case "updateSfmParams": {
      const params = mutation.params;
      if (!finite(params.ransacThresholdPx) || !finite(params.huberDeltaPx)) return refuse("fatal", "mutation.invariant", "SfM params have non-finite thresholds.", [base.id]);
      if (same(params, base.params.sfm)) return noted(empty(), "warn", "mutation.no-op", "SfM params are already up to date.");
      return ok({ params: { ...clone(base.params), sfm: clone(params) } });
    }
    case "updateDenseParams": {
      const params = mutation.params;
      if (!finite(params.confidenceThreshold)) return refuse("fatal", "mutation.invariant", "Dense params have a non-finite confidence threshold.", [base.id]);
      if (same(params, base.params.dense)) return noted(empty(), "warn", "mutation.no-op", "Dense params are already up to date.");
      return ok({ params: { ...clone(base.params), dense: clone(params) } });
    }
    case "updateMeshParams": {
      const params = mutation.params;
      if (!finite(params.tsdfVoxelSizeMm) || !finite(params.tsdfTruncationMm)) return refuse("fatal", "mutation.invariant", "Mesh params have a non-finite TSDF voxel size or truncation.", [base.id]);
      if (same(params, base.params.mesh)) return noted(empty(), "warn", "mutation.no-op", "Mesh params are already up to date.");
      return ok({ params: { ...clone(base.params), mesh: clone(params) } });
    }
    case "updateMotionParams": {
      const params = mutation.params;
      if (!finite(params.minTrackQuality)) return refuse("fatal", "mutation.invariant", "Motion params have a non-finite minimum track quality.", [base.id]);
      if (same(params, base.params.motion)) return noted(empty(), "warn", "mutation.no-op", "Motion params are already up to date.");
      return ok({ params: { ...clone(base.params), motion: clone(params) } });
    }
    case "updateGeoParams": {
      const params = mutation.params;
      const distancesOk = [params.gsdM, params.dsmCellM, params.dtmFilterRadiusM].every((value) => finite(value) && value > 0);
      const latOk = params.originLat === null || (finite(params.originLat) && params.originLat >= -90 && params.originLat <= 90);
      const lonOk = params.originLon === null || (finite(params.originLon) && params.originLon >= -180 && params.originLon <= 180);
      if (!distancesOk || params.orthoMaxPx === 0 || !latOk || !lonOk)
        return refuse("fatal", "mutation.invariant", "Geo params need finite positive distances, a positive ortho resolution, and an in-range origin.");
      if (same(params, base.params.geo)) return noted(empty(), "warn", "mutation.no-op", "Geo params are unchanged.");
      return ok({ params: { ...clone(base.params), geo: clone(params) } });
    }
    case "replaceJob": {
      if (same(mutation.job, base.job)) return noted(empty(), "warn", "mutation.no-op", "Reconstruction job already has this value.");
      return ok({ job: clone(mutation.job) });
    }
    case "replaceSparse": {
      if (same(mutation.sparse, base.results.sparse)) return noted(empty(), "warn", "mutation.no-op", "Sparse results already have this value.");
      return ok({ results: { ...clone(base.results), sparse: clone(mutation.sparse) } as ReconstructionResults });
    }
    case "replaceDense": {
      if (same(mutation.dense, base.results.dense)) return noted(empty(), "warn", "mutation.no-op", "Dense results already have this value.");
      return ok({ results: { ...clone(base.results), dense: clone(mutation.dense) } as ReconstructionResults });
    }
    case "replaceMeshResult": {
      if (mutation.mesh.mesh.target.artifactId.startsWith(MESH_STAGE_HANDLE_PREFIX))
        return refuse("error", "mutation.incomplete-mesh", "Private reconstruction staging handles are accepted only by CommitReconstruction.", [mutation.mesh.mesh.childId]);
      if (same(mutation.mesh, base.results.mesh)) return noted(empty(), "warn", "mutation.no-op", "Mesh result is already up to date.");
      return ok({ results: { ...clone(base.results), mesh: clone(mutation.mesh) } });
    }
    case "replaceTrajectory": {
      if (mutation.trajectory === null && base.results.trajectory === null) return refuse("error", "mutation.target-missing", "There is no trajectory to clear.", [base.id]);
      if (same(mutation.trajectory, base.results.trajectory)) return noted(empty(), "warn", "mutation.no-op", "Trajectory is already up to date.");
      return ok({ results: { ...clone(base.results), trajectory: clone(mutation.trajectory) } as ReconstructionResults });
    }
    case "replaceTracks": {
      if (same(mutation.tracks, base.results.tracks)) return noted(empty(), "warn", "mutation.no-op", "Tracks already have this value.");
      return ok({ results: { ...clone(base.results), tracks: clone(mutation.tracks) } });
    }
    case "replaceGeoProducts": {
      if (mutation.geo === null && base.results.geo === null) return refuse("error", "mutation.target-missing", "There are no geo products to clear.", [base.id]);
      if (same(mutation.geo, base.results.geo)) return noted(empty(), "warn", "mutation.no-op", "Geo products are already up to date.");
      return ok({ results: { ...clone(base.results), geo: clone(mutation.geo) } as ReconstructionResults });
    }
    case "replaceQc": {
      if (mutation.qc === null && base.results.qc === null) return refuse("error", "mutation.target-missing", "There is no QC report to clear.", [base.id]);
      if (same(mutation.qc, base.results.qc)) return noted(empty(), "warn", "mutation.no-op", "QC report is already up to date.");
      return ok({ results: { ...clone(base.results), qc: clone(mutation.qc) } as ReconstructionResults });
    }
  }
}

/** 🏁 `commit-reconstruction` — only its documented refusal path is modelled here.
 *
 *  A successful commit publishes staged content out of the plugin's process-global blob registries
 *  (`commit_staged_remodeling_reconstruction`, `durable_staged_remodeling_asset`), which no
 *  document-level twin can observe. The feature file names exactly one committed vector for this
 *  kind and it is the refusal one: a `sparse` payload that is a plain point buffer rather than a
 *  `remodeling-content:` replayable handle earns `mutation.invalid-reconstruction-sparse` and
 *  leaves the scene untouched. That path — and the sibling asset/mesh handle refusals — is real
 *  here; any input that would actually commit raises `RemodelingUnsupportedError`.
 */
export function commitReconstructionOutcome(base: RemodelingSnapshot, payload: CommitReconstruction): RemodelingMutationOutcome {
  if (payload.sparse !== null && payload.sparse.points !== "" && !payload.sparse.points.startsWith(CONTENT_HANDLE_PREFIX))
    return refuse("error", "mutation.invalid-reconstruction-sparse", "The terminal sparse cloud is not a replayable content handle.", ["sparse"]);
  for (const committed of payload.assets)
    if (!committed.asset.data.startsWith(CONTENT_HANDLE_PREFIX)) return refuse("error", "mutation.invalid-reconstruction-asset", "A terminal asset is not a replayable content handle.", [committed.id]);
  if (payload.mesh !== null && !payload.mesh.mesh.target.artifactId.startsWith(MESH_STAGE_HANDLE_PREFIX))
    return refuse("error", "mutation.invalid-reconstruction-mesh", "The terminal mesh is not a staged replayable handle.", [payload.mesh.mesh.childId]);
  throw new RemodelingUnsupportedError("commitReconstruction publishes from the plugin's process-global staging registries, which this twin does not model; only its refusal paths are implemented");
}

/** ▶️ `apply_remodeling_mutation` — the outcome's diff applied; a refusal leaves `base` untouched. */
export function applyRemodelingMutation(snapshot: RemodelingSnapshot, mutation: RemodelingAnyMutation): RemodelingSnapshot {
  const outcome = mutation.mutation === "commitReconstruction" ? commitReconstructionOutcome(snapshot, mutation) : remodelingMutationOutcome(snapshot, mutation);
  return applyRemodelingDiff(outcome.diff, snapshot);
}

/** 🔺️ The outcome of any tag, `commitReconstruction` included. */
export const remodelingMutationDiff = (snapshot: RemodelingSnapshot, mutation: RemodelingAnyMutation): RemodelingMutationOutcome =>
  mutation.mutation === "commitReconstruction" ? commitReconstructionOutcome(snapshot, mutation) : remodelingMutationOutcome(snapshot, mutation);

/** 🏷️ The wire tag for one kebab-case leaf slug (`create-stream` → `createStream`). */
export const mutationTagOfSlug = (slug: string): RemodelingMutationTag => camelOf(slug.replace(/-/g, "_")) as RemodelingMutationTag;
//#endregion 🔖️Diffs
