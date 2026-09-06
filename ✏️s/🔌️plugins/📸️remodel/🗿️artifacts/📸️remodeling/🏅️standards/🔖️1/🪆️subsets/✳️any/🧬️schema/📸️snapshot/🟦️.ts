/** 🧬️ Remodeling snapshot schema — TypeScript twin of `📸️snapshot/🦀️.rs`.
 *
 *  Field names are declared once in Rust `snake_case`; the JSON key is serde's `camelCase`
 *  rename of it and the `.dsl.semio` key is its `kebab-case` rename, both derived here rather
 *  than transcribed, so a rename cannot silently desynchronise the two wire forms.
 *  The record specs below are the runtime half of these types: they drive decoding
 *  (unknown-key and missing-key rejection), encoding order and every default. */

//#region 🔖️Enums
export type MediaKind = "image-sequence" | "video";
export type VideoCodec = "avc" | "hevc" | "vp9" | "av1" | "mjpeg" | "unknown";
export type FeatureDetector = "orb" | "akaze" | "harris";
export type MatcherKind = "brute-force" | "kd-tree";
export type RobustLossKind = "l2" | "huber" | "cauchy";
export type DenseResolution = "low" | "medium" | "high";
export type MeshSource = "placeholder" | "reconstructed" | "imported";
export type TrackClass = "static" | "moving";
export type ReconstructionStage =
  | "idle"
  | "ingesting"
  | "calibrating"
  | "extracting-features"
  | "matching-features"
  | "estimating-poses"
  | "bundle-adjusting"
  | "georeferencing"
  | "dense-stereo"
  | "fusing-volume"
  | "extracting-surface"
  | "cleaning-mesh"
  | "texturing"
  | "tracking-motion"
  | "deriving-geo-products"
  | "reporting-qc"
  | "done"
  | "failed";

export const MEDIA_KINDS: readonly MediaKind[] = ["image-sequence", "video"];
export const VIDEO_CODECS: readonly VideoCodec[] = ["avc", "hevc", "vp9", "av1", "mjpeg", "unknown"];
export const FEATURE_DETECTORS: readonly FeatureDetector[] = ["orb", "akaze", "harris"];
export const MATCHER_KINDS: readonly MatcherKind[] = ["brute-force", "kd-tree"];
export const ROBUST_LOSS_KINDS: readonly RobustLossKind[] = ["l2", "huber", "cauchy"];
export const DENSE_RESOLUTIONS: readonly DenseResolution[] = ["low", "medium", "high"];
export const MESH_SOURCES: readonly MeshSource[] = ["placeholder", "reconstructed", "imported"];
export const TRACK_CLASSES: readonly TrackClass[] = ["static", "moving"];
export const RECONSTRUCTION_STAGES: readonly ReconstructionStage[] = [
  "idle",
  "ingesting",
  "calibrating",
  "extracting-features",
  "matching-features",
  "estimating-poses",
  "bundle-adjusting",
  "georeferencing",
  "dense-stereo",
  "fusing-volume",
  "extracting-surface",
  "cleaning-mesh",
  "texturing",
  "tracking-motion",
  "deriving-geo-products",
  "reporting-qc",
  "done",
  "failed",
];
//#endregion 🔖️Enums

//#region 🔖️Domain
export type Vec2 = [number, number];
export type Vec3 = [number, number, number];
export type Vec4 = [number, number, number, number];
export type Vec5 = [number, number, number, number, number];

/** 🧩️ Compact `store::os_io::ArtifactRef` dialect triple. */
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}

/** 🪆 `store::ArtifactChild<S>` handle — identity plus the composed child it addresses. */
export interface ArtifactChild {
  childId: string;
  target: ArtifactRef;
}

export type RemodelingAssetChild = ArtifactChild;
export type RemodelingMeshChild = ArtifactChild;

export interface ImageAsset {
  mime: string;
  data: string;
  width: number;
  height: number;
}

export interface RemodelingDurableArtifact {
  kind: string;
  mime: string | null;
  width: number;
  height: number;
  chunks: string[];
}

export type RemodelingDurableArtifactStore = Record<string, RemodelingDurableArtifact>;

export interface VideoSource {
  name: string;
  container: string;
  codec: VideoCodec;
  durationMs: number;
  frameCount: number;
  width: number;
  height: number;
}

export interface FrameRef {
  index: number;
  timestampMs: number;
  assetId: string;
}

export interface MediaStream {
  id: string;
  name: string;
  kind: MediaKind;
  cameraId: string | null;
  syncOffsetMs: number;
  fpsHint: number;
  frames: FrameRef[];
  source: VideoSource | null;
}

export interface CameraCalibration {
  id: string;
  label: string;
  model: string;
  fx: number;
  fy: number;
  cx: number;
  cy: number;
  skew: number;
  distortion: Vec5;
  rmsReprojectionPx: number | null;
  locked: boolean;
}

export interface RigExtrinsic {
  cameraId: string;
  rotationWxyz: Vec4;
  translationM: Vec3;
}

export interface CalibrationState {
  cameras: CameraCalibration[];
  rig: RigExtrinsic[];
}

export interface GcpObservation {
  streamId: string;
  frameIndex: number;
  pixel: Vec2;
}

export interface GroundControlPoint {
  id: string;
  name: string;
  worldPosition: Vec3;
  observations: GcpObservation[];
}

export interface IngestParams {
  frameSampleStride: number;
  maxFrames: number;
  downscaleLongEdgePx: number;
  minSharpness: number;
}

export interface FeatureParams {
  detector: FeatureDetector;
  targetCount: number;
  octaves: number;
  edgeThreshold: number;
}

export interface MatchParams {
  matcher: MatcherKind;
  ratioTest: number;
  crossCheck: boolean;
  sequentialWindow: number;
  maxPairsPerFrame: number;
  loopClosure: boolean;
}

export interface SfmParams {
  ransacIterations: number;
  ransacThresholdPx: number;
  minTrackLength: number;
  baMaxIterations: number;
  robustLoss: RobustLossKind;
  huberDeltaPx: number;
}

export interface DenseParams {
  resolution: DenseResolution;
  windowRadiusPx: number;
  minViewConsistency: number;
  confidenceThreshold: number;
  maxPoints: number;
}

export interface MeshParams {
  tsdfVoxelSizeMm: number;
  tsdfTruncationMm: number;
  decimateTargetTriangles: number;
  smoothingIterations: number;
  textureEnabled: boolean;
  textureSize: number;
  guaranteeWatertight: boolean;
  holeFillMaxBoundaryVerts: number;
  selfIntersectionCheck: boolean;
}

export interface MotionParams {
  enabled: boolean;
  maxTracks: number;
  trackWindowPx: number;
  minTrackQuality: number;
  minTrackLengthFrames: number;
}

export interface GeoParams {
  enabled: boolean;
  originLon: number | null;
  originLat: number | null;
  originAlt: number | null;
  gsdM: number;
  dsmCellM: number;
  dtmFilterRadiusM: number;
  orthoMaxPx: number;
}

export interface ReconstructionParams {
  ingest: IngestParams;
  feature: FeatureParams;
  matching: MatchParams;
  sfm: SfmParams;
  dense: DenseParams;
  mesh: MeshParams;
  motion: MotionParams;
  geo: GeoParams;
}

export interface CameraPosePreview {
  cameraId: string;
  rotationWxyz: Vec4;
  translation: Vec3;
}

export interface ReconstructionJob {
  id: string;
  stage: ReconstructionStage;
  progress01: number;
  cancelRequested: boolean;
  stageCursor: number;
  startedAtMs: number | null;
  error: string | null;
  cameraPosesPreview: CameraPosePreview[];
  sparsePointCloudPreview: string;
}

export interface WatertightReportSnapshot {
  vertexCount: number;
  triangleCount: number;
  boundaryEdgeCount: number;
  boundaryLoopCount: number;
  nonManifoldEdgeCount: number;
  nonManifoldVertexCount: number;
  connectedComponents: number;
  consistentlyOriented: boolean;
  eulerCharacteristic: number;
  genus: number | null;
  signedVolume: number;
  selfIntersectionPairs: number | null;
  closedFallbackUsed: boolean;
  isClosed: boolean;
  isTwoManifold: boolean;
  isWatertight: boolean;
}

export interface RemodelingMesh {
  mesh: RemodelingMeshChild;
  source: MeshSource;
  textureAssetId: string | null;
  watertight: WatertightReportSnapshot | null;
}

export interface SparseCloud {
  points: string;
  colors: string | null;
}

export interface DenseCloud {
  positions: string;
  colors: string | null;
  confidence: string | null;
  classification: string | null;
}

export interface CameraTrajectory {
  poses: CameraPosePreview[];
}

export interface MotionTrackSummary {
  id: string;
  length: number;
  class: TrackClass;
  meanSpeedMS: number;
}

export interface GeoProducts {
  dsmAssetId: string | null;
  dtmAssetId: string | null;
  orthoAssetId: string | null;
}

export interface QcReportSnapshot {
  reprojectionRmsPx: number;
  gcpCheckpointRmse: number | null;
  watertight: WatertightReportSnapshot | null;
  meanTrackLength: number;
  registeredFrameRatio: number;
  denseCoverageRatio: number;
  warnings: string[];
}

export interface ReconstructionResults {
  sparse: SparseCloud | null;
  dense: DenseCloud | null;
  mesh: RemodelingMesh;
  trajectory: CameraTrajectory | null;
  tracks: MotionTrackSummary[];
  geo: GeoProducts | null;
  qc: QcReportSnapshot | null;
}

export interface RemodelingSnapshot {
  schema: string;
  id: string;
  streams: MediaStream[];
  assets: Record<string, RemodelingAssetChild>;
  durableArtifacts: RemodelingDurableArtifactStore;
  calibration: CalibrationState;
  params: ReconstructionParams;
  gcps: GroundControlPoint[];
  job: ReconstructionJob;
  results: ReconstructionResults;
}
//#endregion 🔖️Domain

//#region 🔖️Spec
export type ValueSpec =
  | { k: "text" }
  | { k: "bool" }
  | { k: "uint" }
  | { k: "int" }
  | { k: "f64" }
  | { k: "f32" }
  | { k: "enum"; of: readonly string[] }
  | { k: "tuple"; len: number }
  | { k: "list"; of: ValueSpec }
  | { k: "map"; of: ValueSpec }
  | { k: "rec"; of: () => RecordSpec }
  | { k: "opt"; of: ValueSpec };

export interface FieldSpec {
  /** 🐍 Rust `snake_case` name — the single source both wire renames derive from. */
  name: string;
  spec: ValueSpec;
  dflt: () => unknown;
  /** 🔣 `#[serde(default)]` on this field alone (the container itself is not `default`). */
  jsonOptional?: boolean;
  /** 🗣️ DSL key when the record's own `DslField` impl does not use the kebab rename. */
  dslKey?: string;
}

export interface RecordSpec {
  title: string;
  /** 🔣 `#[serde(rename_all = "camelCase", default)]` — every key may be absent. */
  serdeDefault: boolean;
  fields: FieldSpec[];
}

/** 🐫 serde `rename_all = "camelCase"` over a Rust `snake_case` identifier. */
export const camelOf = (snake: string): string => snake.split("_").map((part, index) => (index === 0 ? part : part.charAt(0).toUpperCase() + part.slice(1))).join("");

/** 🍢 `dsl` `kebab-case` rename over a Rust `snake_case` identifier. */
export const kebabOf = (snake: string): string => snake.replace(/_/g, "-");

const f = (name: string, spec: ValueSpec, dflt: () => unknown, extra: Partial<FieldSpec> = {}): FieldSpec => ({ name, spec, dflt, ...extra });
const text = { k: "text" } as const;
const bool = { k: "bool" } as const;
const uint = { k: "uint" } as const;
const int = { k: "int" } as const;
const f64 = { k: "f64" } as const;
const f32 = { k: "f32" } as const;
const opt = (of: ValueSpec): ValueSpec => ({ k: "opt", of });
const list = (of: ValueSpec): ValueSpec => ({ k: "list", of });
const map = (of: ValueSpec): ValueSpec => ({ k: "map", of });
const rec = (of: () => RecordSpec): ValueSpec => ({ k: "rec", of });
const tuple = (len: number): ValueSpec => ({ k: "tuple", len });
const zeros = (len: number): number[] => Array.from({ length: len }, () => 0);
const identityQuat = (): number[] => [1, 0, 0, 0];

export const ARTIFACT_DIALECT_SPEC: RecordSpec = {
  title: "ArtifactDialect",
  serdeDefault: false,
  fields: [f("artifact_kind", text, () => ""), f("standard", text, () => ""), f("subset", text, () => "")],
};

export const ARTIFACT_REF_SPEC: RecordSpec = {
  title: "ArtifactRef",
  serdeDefault: false,
  fields: [f("artifact_id", text, () => ""), f("dialect", rec(() => ARTIFACT_DIALECT_SPEC), () => defaultsOf(ARTIFACT_DIALECT_SPEC))],
};

export const ARTIFACT_CHILD_SPEC: RecordSpec = {
  title: "ArtifactChild",
  serdeDefault: false,
  fields: [f("child_id", text, () => "", { dslKey: "child_id" }), f("target", rec(() => ARTIFACT_REF_SPEC), () => defaultsOf(ARTIFACT_REF_SPEC), { dslKey: "target" })],
};

export const IMAGE_ASSET_SPEC: RecordSpec = {
  title: "ImageAsset",
  serdeDefault: false,
  fields: [f("mime", text, () => ""), f("data", text, () => ""), f("width", uint, () => 0), f("height", uint, () => 0)],
};

export const DURABLE_ARTIFACT_SPEC: RecordSpec = {
  title: "RemodelingDurableArtifact",
  serdeDefault: true,
  fields: [f("kind", text, () => ""), f("mime", opt(text), () => null), f("width", uint, () => 0), f("height", uint, () => 0), f("chunks", list(text), () => [])],
};

export const VIDEO_SOURCE_SPEC: RecordSpec = {
  title: "VideoSource",
  serdeDefault: true,
  fields: [
    f("name", text, () => ""),
    f("container", text, () => ""),
    f("codec", { k: "enum", of: VIDEO_CODECS }, () => "unknown"),
    f("duration_ms", f64, () => 0),
    f("frame_count", uint, () => 0),
    f("width", uint, () => 0),
    f("height", uint, () => 0),
  ],
};

export const FRAME_REF_SPEC: RecordSpec = {
  title: "FrameRef",
  serdeDefault: false,
  fields: [f("index", uint, () => 0), f("timestamp_ms", f64, () => 0), f("asset_id", text, () => "")],
};

export const MEDIA_STREAM_SPEC: RecordSpec = {
  title: "MediaStream",
  serdeDefault: true,
  fields: [
    f("id", text, () => ""),
    f("name", text, () => ""),
    f("kind", { k: "enum", of: MEDIA_KINDS }, () => "image-sequence"),
    f("camera_id", opt(text), () => null),
    f("sync_offset_ms", f64, () => 0),
    f("fps_hint", f64, () => 0),
    f("frames", list(rec(() => FRAME_REF_SPEC)), () => []),
    f("source", opt(rec(() => VIDEO_SOURCE_SPEC)), () => null),
  ],
};

export const CAMERA_CALIBRATION_SPEC: RecordSpec = {
  title: "CameraCalibration",
  serdeDefault: true,
  fields: [
    f("id", text, () => ""),
    f("label", text, () => ""),
    f("model", text, () => ""),
    f("fx", f64, () => 0),
    f("fy", f64, () => 0),
    f("cx", f64, () => 0),
    f("cy", f64, () => 0),
    f("skew", f64, () => 0),
    f("distortion", tuple(5), () => zeros(5)),
    f("rms_reprojection_px", opt(f32), () => null),
    f("locked", bool, () => false),
  ],
};

export const RIG_EXTRINSIC_SPEC: RecordSpec = {
  title: "RigExtrinsic",
  serdeDefault: true,
  fields: [f("camera_id", text, () => ""), f("rotation_wxyz", tuple(4), identityQuat), f("translation_m", tuple(3), () => zeros(3))],
};

export const CALIBRATION_STATE_SPEC: RecordSpec = {
  title: "CalibrationState",
  serdeDefault: true,
  fields: [f("cameras", list(rec(() => CAMERA_CALIBRATION_SPEC)), () => []), f("rig", list(rec(() => RIG_EXTRINSIC_SPEC)), () => [])],
};

export const GCP_OBSERVATION_SPEC: RecordSpec = {
  title: "GcpObservation",
  serdeDefault: false,
  fields: [f("stream_id", text, () => ""), f("frame_index", uint, () => 0), f("pixel", tuple(2), () => zeros(2))],
};

export const GROUND_CONTROL_POINT_SPEC: RecordSpec = {
  title: "GroundControlPoint",
  serdeDefault: true,
  fields: [f("id", text, () => ""), f("name", text, () => ""), f("world_position", tuple(3), () => zeros(3)), f("observations", list(rec(() => GCP_OBSERVATION_SPEC)), () => [])],
};

export const INGEST_PARAMS_SPEC: RecordSpec = {
  title: "IngestParams",
  serdeDefault: true,
  fields: [f("frame_sample_stride", uint, () => 5), f("max_frames", uint, () => 200), f("downscale_long_edge_px", uint, () => 1600), f("min_sharpness", f32, () => Math.fround(0.3))],
};

export const FEATURE_PARAMS_SPEC: RecordSpec = {
  title: "FeatureParams",
  serdeDefault: true,
  fields: [f("detector", { k: "enum", of: FEATURE_DETECTORS }, () => "orb"), f("target_count", uint, () => 4000), f("octaves", uint, () => 4), f("edge_threshold", f32, () => 10)],
};

export const MATCH_PARAMS_SPEC: RecordSpec = {
  title: "MatchParams",
  serdeDefault: true,
  fields: [
    f("matcher", { k: "enum", of: MATCHER_KINDS }, () => "brute-force"),
    f("ratio_test", f32, () => Math.fround(0.8)),
    f("cross_check", bool, () => true),
    f("sequential_window", uint, () => 8),
    f("max_pairs_per_frame", uint, () => 16),
    f("loop_closure", bool, () => true),
  ],
};

export const SFM_PARAMS_SPEC: RecordSpec = {
  title: "SfmParams",
  serdeDefault: true,
  fields: [
    f("ransac_iterations", uint, () => 1000),
    f("ransac_threshold_px", f32, () => 2),
    f("min_track_length", uint, () => 3),
    f("ba_max_iterations", uint, () => 50),
    f("robust_loss", { k: "enum", of: ROBUST_LOSS_KINDS }, () => "huber"),
    f("huber_delta_px", f32, () => 1.5),
  ],
};

export const DENSE_PARAMS_SPEC: RecordSpec = {
  title: "DenseParams",
  serdeDefault: true,
  fields: [
    f("resolution", { k: "enum", of: DENSE_RESOLUTIONS }, () => "medium"),
    f("window_radius_px", uint, () => 3),
    f("min_view_consistency", uint, () => 3),
    f("confidence_threshold", f32, () => 0.5),
    f("max_points", uint, () => 500000),
  ],
};

export const MESH_PARAMS_SPEC: RecordSpec = {
  title: "MeshParams",
  serdeDefault: true,
  fields: [
    f("tsdf_voxel_size_mm", f32, () => 5),
    f("tsdf_truncation_mm", f32, () => 20),
    f("decimate_target_triangles", uint, () => 200000),
    f("smoothing_iterations", uint, () => 2),
    f("texture_enabled", bool, () => true),
    f("texture_size", uint, () => 2048),
    f("guarantee_watertight", bool, () => true),
    f("hole_fill_max_boundary_verts", uint, () => 512),
    f("self_intersection_check", bool, () => false),
  ],
};

export const MOTION_PARAMS_SPEC: RecordSpec = {
  title: "MotionParams",
  serdeDefault: true,
  fields: [f("enabled", bool, () => false), f("max_tracks", uint, () => 64), f("track_window_px", uint, () => 21), f("min_track_quality", f32, () => Math.fround(0.3)), f("min_track_length_frames", uint, () => 5)],
};

export const GEO_PARAMS_SPEC: RecordSpec = {
  title: "GeoParams",
  serdeDefault: true,
  fields: [
    f("enabled", bool, () => false),
    f("origin_lon", opt(f64), () => null),
    f("origin_lat", opt(f64), () => null),
    f("origin_alt", opt(f64), () => null),
    f("gsd_m", f32, () => Math.fround(0.05)),
    f("dsm_cell_m", f32, () => Math.fround(0.1)),
    f("dtm_filter_radius_m", f32, () => 2),
    f("ortho_max_px", uint, () => 4096),
  ],
};

export const RECONSTRUCTION_PARAMS_SPEC: RecordSpec = {
  title: "ReconstructionParams",
  serdeDefault: true,
  fields: [
    f("ingest", rec(() => INGEST_PARAMS_SPEC), () => defaultsOf(INGEST_PARAMS_SPEC)),
    f("feature", rec(() => FEATURE_PARAMS_SPEC), () => defaultsOf(FEATURE_PARAMS_SPEC)),
    f("matching", rec(() => MATCH_PARAMS_SPEC), () => defaultsOf(MATCH_PARAMS_SPEC)),
    f("sfm", rec(() => SFM_PARAMS_SPEC), () => defaultsOf(SFM_PARAMS_SPEC)),
    f("dense", rec(() => DENSE_PARAMS_SPEC), () => defaultsOf(DENSE_PARAMS_SPEC)),
    f("mesh", rec(() => MESH_PARAMS_SPEC), () => defaultsOf(MESH_PARAMS_SPEC)),
    f("motion", rec(() => MOTION_PARAMS_SPEC), () => defaultsOf(MOTION_PARAMS_SPEC)),
    f("geo", rec(() => GEO_PARAMS_SPEC), () => defaultsOf(GEO_PARAMS_SPEC)),
  ],
};

export const CAMERA_POSE_PREVIEW_SPEC: RecordSpec = {
  title: "CameraPosePreview",
  serdeDefault: false,
  fields: [f("camera_id", text, () => ""), f("rotation_wxyz", tuple(4), identityQuat), f("translation", tuple(3), () => zeros(3))],
};

export const RECONSTRUCTION_JOB_SPEC: RecordSpec = {
  title: "ReconstructionJob",
  serdeDefault: true,
  fields: [
    f("id", text, () => ""),
    f("stage", { k: "enum", of: RECONSTRUCTION_STAGES }, () => "idle"),
    f("progress_0_1", f32, () => 0),
    f("cancel_requested", bool, () => false),
    f("stage_cursor", uint, () => 0),
    f("started_at_ms", opt(f64), () => null),
    f("error", opt(text), () => null),
    f("camera_poses_preview", list(rec(() => CAMERA_POSE_PREVIEW_SPEC)), () => []),
    f("sparse_point_cloud_preview", text, () => ""),
  ],
};

export const WATERTIGHT_REPORT_SPEC: RecordSpec = {
  title: "WatertightReportSnapshot",
  serdeDefault: true,
  fields: [
    f("vertex_count", uint, () => 0),
    f("triangle_count", uint, () => 0),
    f("boundary_edge_count", uint, () => 0),
    f("boundary_loop_count", uint, () => 0),
    f("non_manifold_edge_count", uint, () => 0),
    f("non_manifold_vertex_count", uint, () => 0),
    f("connected_components", uint, () => 0),
    f("consistently_oriented", bool, () => false),
    f("euler_characteristic", int, () => 0),
    f("genus", opt(int), () => null),
    f("signed_volume", f64, () => 0),
    f("self_intersection_pairs", opt(uint), () => null),
    f("closed_fallback_used", bool, () => false),
    f("is_closed", bool, () => false),
    f("is_two_manifold", bool, () => false),
    f("is_watertight", bool, () => false),
  ],
};

/** 🧊️ `empty_remodeling_mesh_handle()` — the stable payload-free child handle. */
export const emptyRemodelingMeshHandle = (): RemodelingMeshChild => remodelingMeshHandle("remodeling-mesh-constant-empty", "remodeling-mesh-constant:empty");

/** 📦️ `placeholder_remodeling_mesh_handle()` — the stable bounded placeholder child handle. */
export const placeholderRemodelingMeshHandle = (): RemodelingMeshChild => remodelingMeshHandle("remodeling-mesh-constant-box", "remodeling-mesh-constant:box");

/** 🧵 `mesh_child_handle` — every mesh child addresses the `s.stdio.semio@v1/mesh` dialect. */
export const remodelingMeshHandle = (childId: string, artifactId: string): RemodelingMeshChild => ({
  childId,
  target: { artifactId, dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "mesh" } },
});

export const REMODELING_MESH_SPEC: RecordSpec = {
  title: "RemodelingMesh",
  serdeDefault: true,
  fields: [
    f("mesh", rec(() => ARTIFACT_CHILD_SPEC), emptyRemodelingMeshHandle),
    f("source", { k: "enum", of: MESH_SOURCES }, () => "placeholder"),
    f("texture_asset_id", opt(text), () => null),
    f("watertight", opt(rec(() => WATERTIGHT_REPORT_SPEC)), () => null),
  ],
};

export const SPARSE_CLOUD_SPEC: RecordSpec = {
  title: "SparseCloud",
  serdeDefault: true,
  fields: [f("points", text, () => ""), f("colors", opt(text), () => null)],
};

export const DENSE_CLOUD_SPEC: RecordSpec = {
  title: "DenseCloud",
  serdeDefault: true,
  fields: [f("positions", text, () => ""), f("colors", opt(text), () => null), f("confidence", opt(text), () => null), f("classification", opt(text), () => null)],
};

export const CAMERA_TRAJECTORY_SPEC: RecordSpec = {
  title: "CameraTrajectory",
  serdeDefault: true,
  fields: [f("poses", list(rec(() => CAMERA_POSE_PREVIEW_SPEC)), () => [])],
};

export const MOTION_TRACK_SUMMARY_SPEC: RecordSpec = {
  title: "MotionTrackSummary",
  serdeDefault: true,
  fields: [f("id", text, () => ""), f("length", uint, () => 0), f("class", { k: "enum", of: TRACK_CLASSES }, () => "static"), f("mean_speed_m_s", f32, () => 0)],
};

export const GEO_PRODUCTS_SPEC: RecordSpec = {
  title: "GeoProducts",
  serdeDefault: true,
  fields: [f("dsm_asset_id", opt(text), () => null), f("dtm_asset_id", opt(text), () => null), f("ortho_asset_id", opt(text), () => null)],
};

export const QC_REPORT_SPEC: RecordSpec = {
  title: "QcReportSnapshot",
  serdeDefault: true,
  fields: [
    f("reprojection_rms_px", f64, () => 0),
    f("gcp_checkpoint_rmse", opt(f64), () => null),
    f("watertight", opt(rec(() => WATERTIGHT_REPORT_SPEC)), () => null),
    f("mean_track_length", f32, () => 0),
    f("registered_frame_ratio", f32, () => 0),
    f("dense_coverage_ratio", f32, () => 0),
    f("warnings", list(text), () => []),
  ],
};

export const RECONSTRUCTION_RESULTS_SPEC: RecordSpec = {
  title: "ReconstructionResults",
  serdeDefault: true,
  fields: [
    f("sparse", opt(rec(() => SPARSE_CLOUD_SPEC)), () => null),
    f("dense", opt(rec(() => DENSE_CLOUD_SPEC)), () => null),
    f("mesh", rec(() => REMODELING_MESH_SPEC), () => defaultsOf(REMODELING_MESH_SPEC)),
    f("trajectory", opt(rec(() => CAMERA_TRAJECTORY_SPEC)), () => null),
    f("tracks", list(rec(() => MOTION_TRACK_SUMMARY_SPEC)), () => []),
    f("geo", opt(rec(() => GEO_PRODUCTS_SPEC)), () => null),
    f("qc", opt(rec(() => QC_REPORT_SPEC)), () => null),
  ],
};

export const REMODELING_DOCUMENT_SCHEMA = "remodeling.scene";

export const REMODELING_SNAPSHOT_SPEC: RecordSpec = {
  title: "RemodelingSnapshot",
  serdeDefault: false,
  fields: [
    f("schema", text, () => REMODELING_DOCUMENT_SCHEMA),
    f("id", text, () => "remodeling"),
    f("streams", list(rec(() => MEDIA_STREAM_SPEC)), () => [], { jsonOptional: true }),
    f("assets", map(rec(() => ARTIFACT_CHILD_SPEC)), () => ({}), { jsonOptional: true }),
    f("durable_artifacts", map(rec(() => DURABLE_ARTIFACT_SPEC)), () => ({}), { jsonOptional: true }),
    f("calibration", rec(() => CALIBRATION_STATE_SPEC), () => defaultsOf(CALIBRATION_STATE_SPEC), { jsonOptional: true }),
    f("params", rec(() => RECONSTRUCTION_PARAMS_SPEC), () => defaultsOf(RECONSTRUCTION_PARAMS_SPEC), { jsonOptional: true }),
    f("gcps", list(rec(() => GROUND_CONTROL_POINT_SPEC)), () => [], { jsonOptional: true }),
    f("job", rec(() => RECONSTRUCTION_JOB_SPEC), () => defaultsOf(RECONSTRUCTION_JOB_SPEC), { jsonOptional: true }),
    f("results", rec(() => RECONSTRUCTION_RESULTS_SPEC), () => defaultsOf(RECONSTRUCTION_RESULTS_SPEC), { jsonOptional: true }),
  ],
};
//#endregion 🔖️Spec

//#region 🔖️Defaults
/** 🧱 `Default::default()` for one record, keyed the way its JSON is keyed. */
export function defaultsOf(spec: RecordSpec): Record<string, unknown> {
  const out: Record<string, unknown> = {};
  for (const field of spec.fields) out[camelOf(field.name)] = field.dflt();
  return out;
}

/** 📸️ `RemodelingSnapshot::default()`. */
export const defaultRemodelingSnapshot = (): RemodelingSnapshot => defaultsOf(REMODELING_SNAPSHOT_SPEC) as unknown as RemodelingSnapshot;

/** 🎬️ `default_remodeling_scene()` — the default snapshot with its placeholder mesh handle. */
export function defaultRemodelingScene(): RemodelingSnapshot {
  const scene = defaultRemodelingSnapshot();
  scene.results.mesh = { mesh: placeholderRemodelingMeshHandle(), source: "placeholder", textureAssetId: null, watertight: null };
  return scene;
}
//#endregion 🔖️Defaults
