/** 📐️ The typed remodel story model: scene, stream, camera and result shapes plus the editor config the stories fold actions into. */
//#region 🔖️SceneModel
/** 🎞️ One sampled frame of a media stream — `RemodelingSnapshot::streams[].frames[]`. */
export type RemodelFrameRef = { readonly index: number; readonly timestampMs: number; readonly assetId: string };

/** 📼️ The decoded-container facts a video import records — `streams[].source`. */
export type RemodelStreamSource = { readonly name: string; readonly container: string; readonly codec: string; readonly durationMs: number; readonly frameCount: number; readonly width: number; readonly height: number };

/** 🎥️ One media stream — `RemodelingSnapshot::streams[]`. */
export type RemodelStream = {
  readonly id: string;
  readonly name: string;
  readonly kind: "video" | "image-sequence";
  readonly cameraId: string | null;
  readonly syncOffsetMs: number;
  readonly fpsHint: number;
  readonly frames: readonly RemodelFrameRef[];
  readonly source: RemodelStreamSource | null;
};

/** 🧩️ A composed child-artifact handle (`s.stdio.semio@v1/<subset>`) — opaque geometry/pixels live behind it. */
export type RemodelChildHandle = { readonly childId: string; readonly target: { readonly artifactId: string; readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string } } };

/** 💧️ The watertightness audit a reconstructed mesh carries — `results.mesh.watertight` / `results.qc.watertight`. */
export type RemodelWatertightReport = {
  readonly vertexCount: number;
  readonly triangleCount: number;
  readonly boundaryEdgeCount: number;
  readonly boundaryLoopCount: number;
  readonly nonManifoldEdgeCount: number;
  readonly nonManifoldVertexCount: number;
  readonly connectedComponents: number;
  readonly consistentlyOriented: boolean;
  readonly eulerCharacteristic: number;
  readonly genus: number | null;
  readonly signedVolume: number;
  readonly selfIntersectionPairs: number;
  readonly closedFallbackUsed: boolean;
  readonly isClosed: boolean;
  readonly isTwoManifold: boolean;
  readonly isWatertight: boolean;
};

/** 🧱️ One document-owned durable leaf — `durableArtifacts[childId]`, the content an asset handle addresses. */
export type RemodelDurableArtifact = { readonly kind: string; readonly mime: string | null; readonly width: number; readonly height: number; readonly chunks: readonly string[] };

/** 📷️ One calibrated camera — `calibration.cameras[]`. */
export type RemodelCamera = {
  readonly id: string;
  readonly label: string;
  readonly model: string;
  readonly fx: number;
  readonly fy: number;
  readonly cx: number;
  readonly cy: number;
  readonly skew: number;
  readonly distortion: readonly number[];
  readonly rmsReprojectionPx: number | null;
  readonly locked: boolean;
};

/** 🎯️ One ground control point and its per-frame pixel observations — `gcps[]`. */
export type RemodelGcp = {
  readonly id: string;
  readonly name: string;
  readonly worldPosition: readonly [number, number, number];
  readonly observations: readonly { readonly streamId: string; readonly frameIndex: number; readonly pixel: readonly [number, number] }[];
};

/** 🧭️ A recovered camera pose — one `results.trajectory.poses[]` entry. */
export type RemodelPose = { readonly cameraId: string; readonly rotationWxyz: readonly number[]; readonly translation: readonly [number, number, number] };

/** 📸️ The remodeling document — a story-local mirror of `RemodelingSnapshot`'s camelCase wire shape. */
export type RemodelScene = {
  readonly schema: string;
  readonly id: string;
  readonly streams: readonly RemodelStream[];
  readonly assets: Readonly<Record<string, RemodelChildHandle>>;
  readonly durableArtifacts: Readonly<Record<string, RemodelDurableArtifact>>;
  readonly calibration: { readonly cameras: readonly RemodelCamera[]; readonly rig: readonly { readonly cameraId: string; readonly rotationWxyz: readonly number[]; readonly translationM: readonly number[] }[] };
  readonly params: {
    readonly ingest: { readonly frameSampleStride: number; readonly maxFrames: number; readonly downscaleLongEdgePx: number; readonly minSharpness: number };
    readonly feature: { readonly detector: string; readonly targetCount: number; readonly octaves: number; readonly edgeThreshold: number };
    readonly matching: { readonly matcher: string; readonly ratioTest: number; readonly crossCheck: boolean; readonly sequentialWindow: number; readonly maxPairsPerFrame: number; readonly loopClosure: boolean };
    readonly sfm: { readonly ransacIterations: number; readonly ransacThresholdPx: number; readonly minTrackLength: number; readonly baMaxIterations: number; readonly robustLoss: string; readonly huberDeltaPx: number };
    readonly dense: { readonly resolution: string; readonly windowRadiusPx: number; readonly minViewConsistency: number; readonly confidenceThreshold: number; readonly maxPoints: number };
    readonly mesh: { readonly tsdfVoxelSizeMm: number; readonly tsdfTruncationMm: number; readonly decimateTargetTriangles: number; readonly smoothingIterations: number; readonly textureEnabled: boolean; readonly textureSize: number; readonly guaranteeWatertight: boolean; readonly holeFillMaxBoundaryVerts: number; readonly selfIntersectionCheck: boolean };
    readonly motion: { readonly enabled: boolean; readonly maxTracks: number; readonly trackWindowPx: number; readonly minTrackQuality: number; readonly minTrackLengthFrames: number };
    readonly geo: { readonly enabled: boolean; readonly originLon: number | null; readonly originLat: number | null; readonly originAlt: number | null; readonly gsdM: number; readonly dsmCellM: number; readonly dtmFilterRadiusM: number; readonly orthoMaxPx: number };
  };
  readonly gcps: readonly RemodelGcp[];
  readonly results: {
    readonly sparse: { readonly points: string; readonly colors: string | null } | null;
    readonly dense: { readonly positions: string; readonly colors: string | null; readonly confidence: string | null; readonly classification: string | null } | null;
    readonly mesh: { readonly mesh: RemodelChildHandle; readonly source: string; readonly textureAssetId: string | null; readonly watertight: RemodelWatertightReport | null };
    readonly trajectory: { readonly poses: readonly RemodelPose[] } | null;
    readonly tracks: readonly { readonly id: string; readonly length: number; readonly class: string; readonly meanSpeedMS: number }[];
    readonly geo: { readonly dsmAssetId: string | null; readonly dtmAssetId: string | null; readonly orthoAssetId: string | null } | null;
    readonly qc: {
      readonly reprojectionRmsPx: number;
      readonly gcpCheckpointRmse: number | null;
      readonly watertight: RemodelWatertightReport | null;
      readonly meanTrackLength: number;
      readonly registeredFrameRatio: number;
      readonly denseCoverageRatio: number;
      readonly warnings: readonly string[];
    } | null;
  };
};

//#endregion 🔖️SceneModel
//#region 🔖️Config
/** 🎚️ Story-local mirror of `RemodelingConfig` (`✏️editor/🎚️config/🦀️.rs`). */
export type RemodelConfig = {
  readonly camera: { readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly fov: number };
  readonly layers: { readonly mesh: boolean; readonly dense: boolean; readonly sparse: boolean; readonly cameras: boolean; readonly gcps: boolean };
  readonly frameCursor: { readonly streamId: string | null; readonly frameIndex: number };
  readonly reportTable: string;
  readonly activeUtilityId: string;
  readonly locale: string;
};

/** 🎚️ `RemodelingConfig::default()` verbatim. */
export const REMODEL_DEFAULT_CONFIG: RemodelConfig = {
  camera: { position: [4, -4, 3], target: [0, 0, 0], fov: 45 },
  layers: { mesh: true, dense: true, sparse: true, cameras: true, gcps: true },
  frameCursor: { streamId: null, frameIndex: 0 },
  reportTable: "frames",
  activeUtilityId: "select",
  locale: "en-US",
};
//#endregion 🔖️Config
