// #region 🧲️Header
// 💻️ ✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination/🟦️.ts
// Specs: The `📸️remodel` scope's story coordination: the typed story model and labels and the scene, panel and report projections built on it — everything the `🎭️*` stories need that is not itself a story.
// Specs: Story-local, self-contained copies of the remodel plugin's own populated document fixture and of
// its editor config defaults + complete en/de label set — the data half of the `📸️remodel` scope's stories.
// Summary: `REMODEL_POPULATED_SCENE` is a verbatim transcription of the shared mutation-fixture document
// `🗿️artifacts/📸️remodeling/…/🧬️schema/🧬️mutations/*/🧪️tests/<toy case>/📸️snapshot/⬅️before/🔣️.json` (the shared
// toy scene every `toy`-role vector starts from — two streams, three assets with their durable leaves, two
// calibrated cameras, two GCPs in canonical id order, and finished sparse/dense/trajectory/tracks/geo/qc
// results; a reconstruction run is framework tool-run state, never document state). It is copied rather than
// imported because a `?raw`/`?json` import of a plugin-owned asset would couple this scope to that plugin
// directory's layout, which is actively being renamed. `REMODEL_EMPTY_SCENE` mirrors `default_remodeling_scene()`
// (`🗿️artifacts/📸️remodeling/🦀️.rs`) — everything empty, the placeholder mesh handle seeded — which is also what
// the shipped `📚️examples/🎬️demo` document contains (its `🗣️.dsl.semio` declares zero streams/gcps/cameras).
// `REMODEL_LABELS` transcribes every field of `app_labels! { RemodelingLabels }` (`✏️editor/🗣️terminology/🦀️.rs`)
// in both native locales, so a story renders the same strings the Rust panels would for `locale: "en-US"/"de-DE"`.
// ⚠️ The populated document now carries its `durableArtifacts` store, so every `assets[key]` handle in it
// resolves to real content — but no leaf exists for the composed `s.stdio.semio@v1/mesh` CHILD handle in
// `results.mesh.mesh`, which resolves to nothing, matching `resolve_bounded_remodeling_mesh`'s documented
// "unavailable content renders no mesh entity" behavior. Every mesh-shaped readout below is therefore honestly
// empty, exactly as the plugin's own render path would be for these documents.
//
// Specs: The projection + reducer half of the `📸️remodel` scope — turns the story-local document/config
// fixtures (`./fixture.ts`) into exactly the payloads remodel's own Rust render functions build, and folds the
// host-dispatched `ActionDescriptor`s back into config the way `RemodelingCommand::dispatch` does.
// Summary: One projection per window kind, mirroring its Rust source line for line —
//   `remodelMainWorldScene`   ⇔ `✏️editor/🎭️modes/🧊️model/🪟️windows/🧊️model/🦀️.rs` (`remodeling-main`, World3d)
//   `remodelFramesCanvasScene`⇔ `✏️editor/🎭️modes/📷️capture/🪟️windows/🖼️frames/🦀️.rs` (`remodeling-frames`, Canvas2d)
//   `remodelReportTableScene` ⇔ `✏️editor/🎭️modes/🔍️analyze/🪟️windows/📊️report/🦀️.rs` (`remodeling-report`, Table)
//   `remodelViewerWorldScene` ⇔ `👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️model/🦀️.rs` (`remodeling-view-model`, World3d)
// plus one `BuiltNode` projection per panel tab (`✏️editor/📌️panels/*/🦀️.rs`), rendered through the REAL
// `InterpretedUiNode`. The panels' Rust still emits the legacy `UiNode` (`ui_stack_vertical`/`ui_text`/
// `ui_import_drop_zone`); no TS adapter from that shape to the retained `BuiltNode` document exists, so these
// helpers do the Stack→`container(plain)` / Text→`text` translation themselves and note it — the strings
// themselves are formatted exactly as the Rust `format!` calls do, in whichever locale the labels came from.
// Reducers cover the config-only command subset the hosts can actually reach without a plugin runtime
// (`setCamera`/`setLayerVisibility`/`setFrameCursor`/`setReportTable`/`setActiveUtility`/`setLocale`);
// an unrecognized action is ignored, matching `command_from_action` returning `None`.
// 2026 Ueli Saluz <ueli@semio-tech.com>
// #endregion 🧲️Header

import type { ActionDescriptor, UiComponentSceneNode } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import type { Canvas2dScene, TableScene, World3dScene } from "../../../../../🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts";
import type { AccessibilitySpec, BuiltNode, LayoutSpec, StyleSpec } from "../../../../../🧰️framework/🔨️modules/🛂️manifest/🟦️.ts";

//#region 🔖️SceneModel
/** @emoji 🎞️ One sampled frame of a media stream — `RemodelingSnapshot::streams[].frames[]`. */
export type RemodelFrameRef = { readonly index: number; readonly timestampMs: number; readonly assetId: string };

/** @emoji 📼️ The decoded-container facts a video import records — `streams[].source`. */
export type RemodelStreamSource = { readonly name: string; readonly container: string; readonly codec: string; readonly durationMs: number; readonly frameCount: number; readonly width: number; readonly height: number };

/** @emoji 🎥️ One media stream — `RemodelingSnapshot::streams[]`. */
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

/** @emoji 🧩️ A composed child-artifact handle (`s.stdio.semio@v1/<subset>`) — opaque geometry/pixels live behind it. */
export type RemodelChildHandle = { readonly childId: string; readonly target: { readonly artifactId: string; readonly dialect: { readonly artifactKind: string; readonly standard: string; readonly subset: string } } };

/** @emoji 💧️ The watertightness audit a reconstructed mesh carries — `results.mesh.watertight` / `results.qc.watertight`. */
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

/** @emoji 🧱️ One document-owned durable leaf — `durableArtifacts[childId]`, the content an asset handle addresses. */
export type RemodelDurableArtifact = { readonly kind: string; readonly mime: string | null; readonly width: number; readonly height: number; readonly chunks: readonly string[] };

/** @emoji 📷️ One calibrated camera — `calibration.cameras[]`. */
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

/** @emoji 🎯️ One ground control point and its per-frame pixel observations — `gcps[]`. */
export type RemodelGcp = {
  readonly id: string;
  readonly name: string;
  readonly worldPosition: readonly [number, number, number];
  readonly observations: readonly { readonly streamId: string; readonly frameIndex: number; readonly pixel: readonly [number, number] }[];
};

/** @emoji 🧭️ A recovered camera pose — one `results.trajectory.poses[]` entry. */
export type RemodelPose = { readonly cameraId: string; readonly rotationWxyz: readonly number[]; readonly translation: readonly [number, number, number] };

/** @emoji 📸️ The remodeling document — a story-local mirror of `RemodelingSnapshot`'s camelCase wire shape. */
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

//#region 🔖️Documents
/** 📸️ The shared populated fixture document, transcribed field-for-field from the plugin's own `⬅️before/🔣️.json`. */
export const REMODEL_POPULATED_SCENE: RemodelScene = {
  schema: "remodeling.scene",
  id: "remodeling-fixture",
  streams: [{
    id: "stream-a",
    name: "Front",
    kind: "video",
    cameraId: "cam-a",
    syncOffsetMs: 12.5,
    fpsHint: 30,
    frames: [{
      index: 0,
      timestampMs: 0,
      assetId: "asset-a"
    }, {
      index: 1,
      timestampMs: 40,
      assetId: "asset-a"
    }],
    source: {
      name: "front.mp4",
      container: "mp4",
      codec: "avc",
      durationMs: 6400,
      frameCount: 160,
      width: 1024,
      height: 768
    }
  }, {
    id: "stream-b",
    name: "Side",
    kind: "image-sequence",
    cameraId: null,
    syncOffsetMs: 0,
    fpsHint: 24,
    frames: [{
      index: 0,
      timestampMs: 0,
      assetId: "asset-a"
    }],
    source: null
  }],
  assets: {
    "asset-a": {
      childId: "remodeling-asset-45070beb0101de64",
      target: {
        artifactId: "asset-a-image",
        dialect: {
          artifactKind: "s.stdio.semio",
          standard: "v1",
          subset: "image"
        }
      }
    },
    "asset-dsm": {
      childId: "remodeling-asset-a462ec9257d3fccf",
      target: {
        artifactId: "asset-dsm-image",
        dialect: {
          artifactKind: "s.stdio.semio",
          standard: "v1",
          subset: "image"
        }
      }
    },
    "asset-spare": {
      childId: "remodeling-asset-84ec1bd76bb83a25",
      target: {
        artifactId: "asset-spare-image",
        dialect: {
          artifactKind: "s.stdio.semio",
          standard: "v1",
          subset: "image"
        }
      }
    }
  },
  durableArtifacts: {
    "remodeling-asset-45070beb0101de64": {
      kind: "image",
      mime: "image/jpeg",
      width: 640,
      height: 480,
      chunks: ["ZnJhbWUtYQ=="]
    },
    "remodeling-asset-84ec1bd76bb83a25": {
      kind: "image",
      mime: "image/png",
      width: 64,
      height: 64,
      chunks: ["c3BhcmUtcmFzdGVy"]
    },
    "remodeling-asset-a462ec9257d3fccf": {
      kind: "image",
      mime: "image/tiff",
      width: 128,
      height: 128,
      chunks: ["dG95LWRzbS10aWxl"]
    }
  },
  calibration: {
    cameras: [{
      id: "cam-a",
      label: "Front",
      model: "brownConrady",
      fx: 1000,
      fy: 1000,
      cx: 512,
      cy: 384,
      skew: 0,
      distortion: [0.0625, -0.03125, 0, 0, 0],
      rmsReprojectionPx: 0.5,
      locked: false
    }, {
      id: "cam-b",
      label: "Side",
      model: "pinhole",
      fx: 800,
      fy: 800,
      cx: 320,
      cy: 240,
      skew: 0,
      distortion: [0, 0, 0, 0, 0],
      rmsReprojectionPx: null,
      locked: true
    }],
    rig: [{
      cameraId: "cam-a",
      rotationWxyz: [1, 0, 0, 0],
      translationM: [0, 0, 0]
    }]
  },
  params: {
    ingest: {
      frameSampleStride: 5,
      maxFrames: 200,
      downscaleLongEdgePx: 1600,
      minSharpness: 0.25
    },
    feature: {
      detector: "orb",
      targetCount: 4000,
      octaves: 4,
      edgeThreshold: 10
    },
    matching: {
      matcher: "brute-force",
      ratioTest: 0.75,
      crossCheck: true,
      sequentialWindow: 8,
      maxPairsPerFrame: 16,
      loopClosure: true
    },
    sfm: {
      ransacIterations: 1000,
      ransacThresholdPx: 2,
      minTrackLength: 3,
      baMaxIterations: 50,
      robustLoss: "huber",
      huberDeltaPx: 1.5
    },
    dense: {
      resolution: "medium",
      windowRadiusPx: 3,
      minViewConsistency: 3,
      confidenceThreshold: 0.5,
      maxPoints: 500000
    },
    mesh: {
      tsdfVoxelSizeMm: 5,
      tsdfTruncationMm: 20,
      decimateTargetTriangles: 200000,
      smoothingIterations: 2,
      textureEnabled: true,
      textureSize: 2048,
      guaranteeWatertight: true,
      holeFillMaxBoundaryVerts: 512,
      selfIntersectionCheck: false
    },
    motion: {
      enabled: false,
      maxTracks: 64,
      trackWindowPx: 21,
      minTrackQuality: 0.25,
      minTrackLengthFrames: 5
    },
    geo: {
      enabled: false,
      originLon: null,
      originLat: null,
      originAlt: null,
      gsdM: 0.0625,
      dsmCellM: 0.125,
      dtmFilterRadiusM: 2,
      orthoMaxPx: 4096
    }
  },
  gcps: [{
    id: "gcp-corner",
    name: "Corner",
    worldPosition: [1, 2, 3],
    observations: [{
      streamId: "stream-b",
      frameIndex: 0,
      pixel: [10, 20]
    }]
  }, {
    id: "gcp-ridge",
    name: "Ridge",
    worldPosition: [4, 5, 6],
    observations: []
  }],
  results: {
    sparse: {
      points: "AAAAAAAAAAAAAAAAAACAPwAAgD8AAIA/",
      colors: "/wAAAP8A"
    },
    dense: {
      positions: "AAAAAAAAAAAAAAAA",
      colors: "AAD/",
      confidence: "AAAAPw==",
      classification: "Ag=="
    },
    mesh: {
      mesh: {
        childId: "remodeling-mesh-901ccade3f60f8f1",
        target: {
          artifactId: "remodeling-mesh",
          dialect: {
            artifactKind: "s.stdio.semio",
            standard: "v1",
            subset: "mesh"
          }
        }
      },
      source: "reconstructed",
      textureAssetId: "asset-a",
      watertight: {
        vertexCount: 512,
        triangleCount: 1020,
        boundaryEdgeCount: 0,
        boundaryLoopCount: 0,
        nonManifoldEdgeCount: 0,
        nonManifoldVertexCount: 0,
        connectedComponents: 1,
        consistentlyOriented: true,
        eulerCharacteristic: 2,
        genus: 0,
        signedVolume: 12.5,
        selfIntersectionPairs: 0,
        closedFallbackUsed: false,
        isClosed: true,
        isTwoManifold: true,
        isWatertight: true
      }
    },
    trajectory: {
      poses: [{
        cameraId: "cam-a",
        rotationWxyz: [1, 0, 0, 0],
        translation: [0, 0, 0]
      }, {
        cameraId: "cam-a",
        rotationWxyz: [0.75, 0.25, 0, 0],
        translation: [0.5, 0, 0]
      }]
    },
    tracks: [{
      id: "track-a",
      length: 42,
      class: "moving",
      meanSpeedMS: 1.5
    }],
    geo: {
      dsmAssetId: "asset-dsm",
      dtmAssetId: null,
      orthoAssetId: null
    },
    qc: {
      reprojectionRmsPx: 0.5,
      gcpCheckpointRmse: 0.25,
      watertight: null,
      meanTrackLength: 6,
      registeredFrameRatio: 1,
      denseCoverageRatio: 0.75,
      warnings: ["low overlap on frame 12"]
    }
  }
};

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

//#region 🔖️Config
/** @emoji 🎚️ Story-local mirror of `RemodelingConfig` (`✏️editor/🎚️config/🦀️.rs`). */
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

//#region 🔖️Labels
/** @emoji 🗣️ One locale's resolution of `RemodelingLabels` — one field per `app_labels!` row, native variant. */
export type RemodelLabels = Readonly<Record<RemodelLabelKey, string>>;

export type RemodelLabelKey =
  | "model"
  | "capture"
  | "analyze"
  | "reconstruction"
  | "error"
  | "status"
  | "running"
  | "idle"
  | "utility"
  | "mesh"
  | "vertices"
  | "triangles"
  | "streams"
  | "assets"
  | "noStreams"
  | "streamKindVideo"
  | "streamKindImageSequence"
  | "frames"
  | "syncOffset"
  | "sparseCloud"
  | "denseCloud"
  | "resultsNone"
  | "trajectory"
  | "poses"
  | "geoProducts"
  | "available"
  | "paramsIngest"
  | "paramsFeature"
  | "paramsMatching"
  | "paramsSfm"
  | "paramsDense"
  | "paramsMesh"
  | "paramsMotion"
  | "paramsGeo"
  | "strideShort"
  | "maxShort"
  | "downscaleShort"
  | "targetShort"
  | "octavesShort"
  | "ratioShort"
  | "windowShort"
  | "ransacShort"
  | "minTrackShort"
  | "baShort"
  | "voxelShort"
  | "enabled"
  | "disabled"
  | "camerasCalibrated"
  | "rigExtrinsics"
  | "gcps"
  | "tracks"
  | "tracksNone"
  | "motionNotImplemented"
  | "qcNone"
  | "qcReprojection"
  | "qcTrackLength"
  | "qcRegisteredRatio"
  | "qcDenseCoverage"
  | "qcGcpRmse"
  | "qcWatertight"
  | "qcBoundaryEdges"
  | "qcComponents"
  | "qcEuler"
  | "qcGenus"
  | "qcClosedFallback"
  | "panelMedia"
  | "panelPipeline"
  | "panelResults"
  | "panelParameters"
  | "panelCalibration"
  | "panelTracks"
  | "panelQc"
  | "windowFrames"
  | "windowReport"
  | "layers"
  | "layerMesh"
  | "layerDense"
  | "layerSparse"
  | "layerCameras"
  | "layerGcps";

/** 🇬🇧️ `native_en` column of `app_labels! { RemodelingLabels }`, transcribed verbatim. */
const REMODEL_LABELS_EN: RemodelLabels = {
  model: "Model",
  capture: "Capture",
  analyze: "Analyze",
  reconstruction: "Reconstruction",
  error: "error",
  status: "Status",
  running: "Running",
  idle: "Idle",
  utility: "Utility",
  mesh: "Mesh",
  vertices: "vertices",
  triangles: "triangles",
  streams: "Streams",
  assets: "Assets",
  noStreams: "No media streams imported yet",
  streamKindVideo: "video",
  streamKindImageSequence: "image sequence",
  frames: "frames",
  syncOffset: "sync offset",
  sparseCloud: "Sparse point cloud",
  denseCloud: "Dense point cloud",
  resultsNone: "none",
  trajectory: "Trajectory",
  poses: "poses",
  geoProducts: "Geo products",
  available: "available",
  paramsIngest: "Ingest",
  paramsFeature: "Feature",
  paramsMatching: "Matching",
  paramsSfm: "SfM",
  paramsDense: "Dense",
  paramsMesh: "Mesh",
  paramsMotion: "Motion",
  paramsGeo: "Geo",
  strideShort: "stride",
  maxShort: "max",
  downscaleShort: "downscale",
  targetShort: "target",
  octavesShort: "octaves",
  ratioShort: "ratio",
  windowShort: "window",
  ransacShort: "ransac",
  minTrackShort: "min track",
  baShort: "ba",
  voxelShort: "voxel",
  enabled: "enabled",
  disabled: "disabled",
  camerasCalibrated: "Calibrated cameras",
  rigExtrinsics: "Rig extrinsics",
  gcps: "Ground control points",
  tracks: "Motion tracks",
  tracksNone: "No motion tracks",
  motionNotImplemented: "Motion tracking is not yet driven by the reconstruction engine",
  qcNone: "No quality report yet",
  qcReprojection: "Mean reprojection error",
  qcTrackLength: "Mean track length",
  qcRegisteredRatio: "Registered frame ratio",
  qcDenseCoverage: "Dense coverage ratio",
  qcGcpRmse: "GCP checkpoint RMSE",
  qcWatertight: "Watertight",
  qcBoundaryEdges: "Boundary edges",
  qcComponents: "Connected components",
  qcEuler: "Euler characteristic",
  qcGenus: "Genus",
  qcClosedFallback: "Closed via fallback",
  panelMedia: "Media",
  panelPipeline: "Pipeline",
  panelResults: "Results",
  panelParameters: "Parameters",
  panelCalibration: "Calibration",
  panelTracks: "Tracks",
  panelQc: "Quality",
  windowFrames: "Frames",
  windowReport: "Report",
  layers: "Layers",
  layerMesh: "Mesh",
  layerDense: "Dense cloud",
  layerSparse: "Sparse cloud",
  layerCameras: "Cameras",
  layerGcps: "GCPs",
};

/** 🇩🇪️ `native_de` column of `app_labels! { RemodelingLabels }`, transcribed verbatim (umlauts included, as authored). */
const REMODEL_LABELS_DE: RemodelLabels = {
  model: "Modell",
  capture: "Aufnahme",
  analyze: "Analyse",
  reconstruction: "Rekonstruktion",
  error: "Fehler",
  status: "Status",
  running: "Läuft",
  idle: "Leerlauf",
  utility: "Werkzeug",
  mesh: "Mesh",
  vertices: "Vertices",
  triangles: "Dreiecke",
  streams: "Streams",
  assets: "Assets",
  noStreams: "Noch keine Medien-Streams importiert",
  streamKindVideo: "Video",
  streamKindImageSequence: "Bildsequenz",
  frames: "Frames",
  syncOffset: "Sync-Versatz",
  sparseCloud: "Dünne Punktwolke",
  denseCloud: "Dichte Punktwolke",
  resultsNone: "keine",
  trajectory: "Trajektorie",
  poses: "Posen",
  geoProducts: "Geo-Produkte",
  available: "verfügbar",
  paramsIngest: "Ingest",
  paramsFeature: "Feature",
  paramsMatching: "Matching",
  paramsSfm: "SfM",
  paramsDense: "Dense",
  paramsMesh: "Mesh",
  paramsMotion: "Bewegung",
  paramsGeo: "Geo",
  strideShort: "Schrittweite",
  maxShort: "max",
  downscaleShort: "Verkleinerung",
  targetShort: "Ziel",
  octavesShort: "Oktaven",
  ratioShort: "Verhältnis",
  windowShort: "Fenster",
  ransacShort: "Ransac",
  minTrackShort: "min. Spur",
  baShort: "BA",
  voxelShort: "Voxel",
  enabled: "aktiviert",
  disabled: "deaktiviert",
  camerasCalibrated: "Kalibrierte Kameras",
  rigExtrinsics: "Rig-Extrinsik",
  gcps: "Passpunkte",
  tracks: "Bewegungsspuren",
  tracksNone: "Keine Bewegungsspuren",
  motionNotImplemented: "Bewegungsverfolgung wird von der Rekonstruktions-Engine noch nicht ausgeführt",
  qcNone: "Noch kein Qualitätsbericht",
  qcReprojection: "Mittlerer Reprojektionsfehler",
  qcTrackLength: "Mittlere Spurlänge",
  qcRegisteredRatio: "Anteil registrierter Frames",
  qcDenseCoverage: "Dense-Abdeckungsanteil",
  qcGcpRmse: "Passpunkt-Kontroll-RMSE",
  qcWatertight: "Wasserdicht",
  qcBoundaryEdges: "Ränder",
  qcComponents: "Zusammenhangskomponenten",
  qcEuler: "Euler-Charakteristik",
  qcGenus: "Genus",
  qcClosedFallback: "Über Fallback geschlossen",
  panelMedia: "Medien",
  panelPipeline: "Pipeline",
  panelResults: "Ergebnisse",
  panelParameters: "Parameter",
  panelCalibration: "Kalibrierung",
  panelTracks: "Spuren",
  panelQc: "Qualität",
  windowFrames: "Frames",
  windowReport: "Bericht",
  layers: "Ebenen",
  layerMesh: "Mesh",
  layerDense: "Dichte Punktwolke",
  layerSparse: "Dünne Punktwolke",
  layerCameras: "Kameras",
  layerGcps: "Passpunkte",
};

/** 🌐️ Story-local mirror of `resolve_labels_for_locale::<RemodelingLabels>` — any non-`de` BCP-47 tag falls back to English, matching the framework resolver's language-prefix match. */
export function remodelLabelsFor(locale: string): RemodelLabels {
  return locale.toLowerCase().startsWith("de") ? REMODEL_LABELS_DE : REMODEL_LABELS_EN;
}
//#endregion 🔖️Labels

//#region 🔖️Constants
/** 🆔️ `REMODELING_PLAY_APP_ID` — the editor's world/canvas/table controller id. */
export const REMODEL_EDITOR_CONTROLLER_ID = "remodeling-play";
/** 🆔️ `REMODELING_VIEW_CONTROLLER_ID` — kept distinct so a viewer surface is never mistaken for an editor one. */
export const REMODEL_VIEWER_CONTROLLER_ID = "remodeling-view";
const REMODEL_MESH_ID = "remodeling-result";
const REMODEL_SURFACE_MAIN = "remodeling.play";
const REMODEL_SURFACE_FRAMES = "remodeling.play.frames";
const REMODEL_SURFACE_REPORT = "remodeling.play.report";
const REMODEL_SURFACE_VIEW = "remodeling.view.scene3d/model";
/** 📥️ `REMODELING_MEDIA_ACCEPT` (`✏️editor/🎮️commands/🎞️import-frames/🦀️.rs`) — the media panel drop zone's accept list, verbatim. */
const REMODEL_MEDIA_ACCEPT = "image/png,image/jpeg,video/mp4,video/quicktime,video/webm,video/x-msvideo,.png,.jpg,.jpeg,.mp4,.mov,.webm,.avi";

/** 🕒️ The three modes `create_remodeling_app` declares, each with the window kind its layout puts in the main slot. */
export const REMODEL_MODES = [
  { id: "capture", windowKindId: "remodeling-frames" },
  { id: "model", windowKindId: "remodeling-main" },
  { id: "analyze", windowKindId: "remodeling-report" },
] as const;

/** 🪟️ Every panel tab `create_remodeling_app` registers, in registration order, with its body key. */
export const REMODEL_PANELS = [
  { id: "pipeline", bodyKey: "remodeling.play.pipeline", labelKey: "panelPipeline" },
  { id: "media", bodyKey: "remodeling.play.media", labelKey: "panelMedia" },
  { id: "results", bodyKey: "remodeling.play.results", labelKey: "panelResults" },
  { id: "parameters", bodyKey: "remodeling.play.parameters", labelKey: "panelParameters" },
  { id: "calibration", bodyKey: "remodeling.play.calibration", labelKey: "panelCalibration" },
  { id: "tracks", bodyKey: "remodeling.play.tracks", labelKey: "panelTracks" },
  { id: "quality", bodyKey: "remodeling.play.qc", labelKey: "panelQc" },
] as const;

export type RemodelPanelId = (typeof REMODEL_PANELS)[number]["id"];
//#endregion 🔖️Constants

//#region 🔖️Formatting
/** @emoji 🔢️ Rust's `{:.N}` — fixed decimals, never JS's shortest round-trip. */
function fixed(value: number, decimals: number): string {
  return value.toFixed(decimals);
}

/** @emoji 🐍️ Rust's `{:?}` over a serde-kebab enum tag: the fixtures store `"brute-force"`, `Debug` prints `BruteForce`. */
function debugEnum(tag: string): string {
  return tag
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

/** @emoji 🔡️ `descriptor.args` narrowed to a plain record — the shape every host dispatches. */
export function remodelActionArgs(args: ActionDescriptor["args"]): Record<string, unknown> {
  return (args ?? {}) as Record<string, unknown>;
}
//#endregion 🔖️Formatting

//#region 🔖️WindowScenes
/** ☁️ `world_points_json` — the sparse/dense clouds, the stored trajectory's camera poses and the GCP world positions, each gated on its own `config.layers` toggle. `positionsB64`/`colorsB64` are already base64 LE-f32 / u8-rgb buffers on the document, so no re-encode happens here either. */
function remodelWorldPointsJson(scene: RemodelScene, layers: RemodelConfig["layers"], gateOnConfig: boolean): string | undefined {
  const out: Record<string, unknown>[] = [];
  if ((!gateOnConfig || layers.sparse) && scene.results.sparse && scene.results.sparse.points.length > 0) {
    out.push({ id: "remodeling-sparse", positionsB64: scene.results.sparse.points, colorsB64: scene.results.sparse.colors, size: 3, sizeAttenuation: false });
  }
  if ((!gateOnConfig || layers.dense) && scene.results.dense && scene.results.dense.positions.length > 0) {
    out.push({ id: "remodeling-dense", positionsB64: scene.results.dense.positions, colorsB64: scene.results.dense.colors, size: 2, sizeAttenuation: false });
  }
  if ((!gateOnConfig || layers.cameras) && scene.results.trajectory && scene.results.trajectory.poses.length > 0) {
    out.push({ id: "remodeling-camera-poses", positionsB64: packF32(scene.results.trajectory.poses.flatMap((pose) => [...pose.translation])), colorsB64: null, size: 9, sizeAttenuation: false });
  }
  if ((!gateOnConfig || layers.gcps) && scene.gcps.length > 0) {
    out.push({ id: "remodeling-gcps", positionsB64: packF32(scene.gcps.flatMap((gcp) => [...gcp.worldPosition])), colorsB64: null, size: 10, sizeAttenuation: false });
  }
  return out.length === 0 ? undefined : JSON.stringify(out);
}

/** @emoji 🧮️ `PackedF32::from_f32_slice` — little-endian f32 triples, base64'd, byte-for-byte the wire shape `WorldPointCloudLayer` decodes. */
function packF32(values: readonly number[]): string {
  const buffer = new ArrayBuffer(values.length * 4);
  const view = new DataView(buffer);
  values.forEach((value, index) => view.setFloat32(index * 4, value, true));
  let binary = "";
  for (const byte of new Uint8Array(buffer)) binary += String.fromCharCode(byte);
  return btoa(binary);
}

/**
 * 🧊️ `remodeling-main`'s `World3dScene`. `meshesJson` mirrors `world_meshes_json`: the composed
 * `s.stdio.semio@v1/mesh` CHILD resolves through `resolve_bounded_remodeling_mesh` against
 * `scene.durable_artifacts`, and NEITHER story document carries that map — so it is `"[]"`, exactly as the
 * plugin renders these documents. `instancesJson` still emits its instance (the Rust gates it only on
 * `config.layers.mesh`, never on whether the mesh resolved), so the host shows its own missing-mesh
 * placeholder for it — a faithful reproduction of the runtime, not a story shortcut.
 */
export function remodelMainWorldScene(scene: RemodelScene, config: RemodelConfig): World3dScene {
  return {
    cameraJson: JSON.stringify({ position: config.camera.position, target: config.camera.target, fov: config.camera.fov }),
    meshesJson: "[]",
    instancesJson: config.layers.mesh ? JSON.stringify([{ id: REMODEL_MESH_ID, meshId: REMODEL_MESH_ID, position: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], selected: false, hovered: false }]) : "[]",
    selectionJson: JSON.stringify({ mode: "rectangle", ids: [], primaryId: null }),
    pointsJson: remodelWorldPointsJson(scene, config.layers, true),
  };
}

/** 👁️ `remodeling-view-model`'s `World3dScene` — same projection with a hardcoded camera and every layer unconditionally on (a viewer keeps no per-session layer state). */
export function remodelViewerWorldScene(scene: RemodelScene): World3dScene {
  return {
    cameraJson: JSON.stringify({ position: [4, -4, 3], target: [0, 0, 0], fov: 45 }),
    meshesJson: "[]",
    instancesJson: JSON.stringify([{ id: REMODEL_MESH_ID, meshId: REMODEL_MESH_ID, position: [0, 0, 0], rotation: [0, 0, 0, 1], scale: [1, 1, 1], selected: false, hovered: false }]),
    selectionJson: JSON.stringify({ mode: "rectangle", ids: [], primaryId: null }),
    pointsJson: remodelWorldPointsJson(scene, { mesh: true, dense: true, sparse: true, cameras: true, gcps: true }, false),
  };
}

/**
 * 🖼️ `frames_layers_json` VERBATIM — the cursored frame's image layer plus every GCP observation planted on it.
 * ⚠️ Both layer objects are keyed `type` (`"image"`/`"points"`), and the point layer carries
 * `points: [{x, y, label}]`. `Canvas2dHost`'s `JsonLayersCanvasSession` keys on `kind` and expects
 * `points: [[x, y], …]` — so this payload draws nothing recognizable. See `remodelFramesCanvasSceneHostShaped`.
 */
export function remodelFramesLayersJson(scene: RemodelScene, cursor: RemodelConfig["frameCursor"]): string {
  if (cursor.streamId === null) return "[]";
  const stream = scene.streams.find((candidate) => candidate.id === cursor.streamId);
  if (!stream) return "[]";
  const layers: Record<string, unknown>[] = [];
  const frame = stream.frames.find((candidate) => candidate.index === cursor.frameIndex);
  // 🖼️ `remodeling_asset` resolves the asset CHILD handle through `durable_artifacts`, which neither story
  // document carries — so, as in the plugin, no image layer is emitted for these documents.
  if (frame !== undefined && scene.assets[frame.assetId] === undefined) void frame;
  const points = scene.gcps.flatMap((gcp) => gcp.observations.filter((observation) => observation.streamId === cursor.streamId && observation.frameIndex === cursor.frameIndex).map((observation) => ({ x: observation.pixel[0], y: observation.pixel[1], label: gcp.name })));
  if (points.length > 0) layers.push({ type: "points", id: "remodeling-gcp-observations", points });
  return JSON.stringify(layers);
}

/** 🖼️ `remodeling-frames`' `Canvas2dScene` — the plugin's own payload, unchanged. */
export function remodelFramesCanvasScene(scene: RemodelScene, config: RemodelConfig): Canvas2dScene {
  return { cameraX: 0, cameraY: 0, zoom: 1, layersJson: remodelFramesLayersJson(scene, config.frameCursor) };
}

/** 🩹️ The same content re-keyed into the schema `JsonLayersCanvasSession` actually speaks (`kind: "circle"` markers with bounds) — the story's own adapter, showing what the window WOULD paint once the plugin emits a host-shaped layer list. Not a plugin behavior. */
export function remodelFramesCanvasSceneHostShaped(scene: RemodelScene, config: RemodelConfig): Canvas2dScene {
  const cursor = config.frameCursor;
  const stream = cursor.streamId === null ? undefined : scene.streams.find((candidate) => candidate.id === cursor.streamId);
  const layers = !stream
    ? []
    : scene.gcps.flatMap((gcp) =>
        gcp.observations
          .filter((observation) => observation.streamId === cursor.streamId && observation.frameIndex === cursor.frameIndex)
          .map((observation) => ({ id: `${gcp.id}@${observation.frameIndex}`, kind: "circle", role: "handle", name: gcp.name, x: observation.pixel[0] - 8, y: observation.pixel[1] - 8, width: 16, height: 16 })),
      );
  return { cameraX: 0, cameraY: 0, zoom: 4, layersJson: JSON.stringify(layers) };
}

/** 📊️ `report_table_json` — the `(columns, rows)` pair for one dataset name; any unknown name falls back to the frame list. */
export function remodelReportTableJson(scene: RemodelScene, table: string): { readonly columnsJson: string; readonly rowsJson: string } {
  switch (table) {
    case "cameras":
      return {
        columnsJson: JSON.stringify([
          { id: "id", label: "Id" },
          { id: "model", label: "Model" },
          { id: "fx", label: "fx" },
          { id: "fy", label: "fy" },
          { id: "rms", label: "RMS (px)" },
        ]),
        rowsJson: JSON.stringify(scene.calibration.cameras.map((camera) => ({ id: camera.id, model: camera.model, fx: camera.fx, fy: camera.fy, rms: camera.rmsReprojectionPx }))),
      };
    case "tracks":
      return {
        columnsJson: JSON.stringify([
          { id: "id", label: "Id" },
          { id: "length", label: "Length" },
          { id: "class", label: "Class" },
          { id: "speed", label: "Mean Speed (m/s)" },
        ]),
        rowsJson: JSON.stringify(scene.results.tracks.map((track) => ({ id: track.id, length: track.length, class: debugEnum(track.class), speed: track.meanSpeedMS }))),
      };
    case "gcps":
      return {
        columnsJson: JSON.stringify([
          { id: "id", label: "Id" },
          { id: "name", label: "Name" },
          { id: "x", label: "X" },
          { id: "y", label: "Y" },
          { id: "z", label: "Z" },
          { id: "observations", label: "Observations" },
        ]),
        rowsJson: JSON.stringify(scene.gcps.map((gcp) => ({ id: gcp.id, name: gcp.name, x: gcp.worldPosition[0], y: gcp.worldPosition[1], z: gcp.worldPosition[2], observations: gcp.observations.length }))),
      };
    case "qcStages": {
      const qc = scene.results.qc;
      return {
        columnsJson: JSON.stringify([
          { id: "check", label: "Check" },
          { id: "value", label: "Value" },
        ]),
        rowsJson: JSON.stringify(
          qc === null
            ? []
            : [{ check: "reprojectionRmsPx", value: qc.reprojectionRmsPx }, { check: "registeredFrameRatio", value: qc.registeredFrameRatio }, ...qc.warnings.map((warning) => ({ check: "warning", value: warning }))],
        ),
      };
    }
    case "matches":
      return { columnsJson: JSON.stringify([{ id: "note", label: "Note" }]), rowsJson: JSON.stringify([{ note: "Pairwise match data is reconstruction-runtime scratch, never distilled into durable document state." }]) };
    default:
      return {
        columnsJson: JSON.stringify([
          { id: "streamId", label: "Stream" },
          { id: "index", label: "Index" },
          { id: "timestampMs", label: "Timestamp (ms)" },
          { id: "assetId", label: "Asset" },
        ]),
        rowsJson: JSON.stringify(scene.streams.flatMap((stream) => stream.frames.map((frame) => ({ id: `${stream.id}#${frame.index}`, streamId: stream.id, index: frame.index, timestampMs: frame.timestampMs, assetId: frame.assetId })))),
      };
  }
}

/** 📊️ `remodeling-report`'s `TableScene`, with the plugin's own header row made sortable so `TableHost`'s sort round-trips visibly. */
export function remodelReportTableScene(scene: RemodelScene, config: RemodelConfig): TableScene {
  const { columnsJson, rowsJson } = remodelReportTableJson(scene, config.reportTable);
  return { columnsJson: JSON.stringify((JSON.parse(columnsJson) as Record<string, unknown>[]).map((column) => ({ ...column, sortable: true }))), rowsJson };
}

/** 🪟️ The `UiComponentSceneNode` each window kind mounts against. */
export function remodelWindowNode(windowKindId: string, scene: RemodelScene, config: RemodelConfig, hostShapedFrames = false): UiComponentSceneNode {
  switch (windowKindId) {
    case "remodeling-frames":
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_FRAMES, controllerId: REMODEL_EDITOR_CONTROLLER_ID, componentKind: "canvas-2d", canvas2d: hostShapedFrames ? remodelFramesCanvasSceneHostShaped(scene, config) : remodelFramesCanvasScene(scene, config) };
    case "remodeling-report":
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_REPORT, controllerId: REMODEL_EDITOR_CONTROLLER_ID, componentKind: "table", table: remodelReportTableScene(scene, config) };
    case "remodeling-view-model":
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_VIEW, controllerId: REMODEL_VIEWER_CONTROLLER_ID, componentKind: "world-3d", world3d: remodelViewerWorldScene(scene) };
    default:
      return { type: "componentScene", surfaceId: REMODEL_SURFACE_MAIN, controllerId: REMODEL_EDITOR_CONTROLLER_ID, componentKind: "world-3d", world3d: remodelMainWorldScene(scene, config) };
  }
}
//#endregion 🔖️WindowScenes

//#region 🔖️PanelDocuments
const PANEL_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const PANEL_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };
const PANEL_STACK_LAYOUT: LayoutSpec = { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "sm" }, align: "stretch", justify: "start", grow: false, wrap: false };
const PANEL_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "fill", height: "hug" };

/** @emoji 🔤️ `ui_text(Label::data(…))` translated to the retained document's `Component::Text`. */
function panelText(key: string, value: string): BuiltNode {
  return { key, component: { type: "text", value, emphasize: null, dataAttributes: null }, layout: PANEL_LEAF_LAYOUT, style: PANEL_STYLE, activity: "idle", disabled: false, accessibility: PANEL_ACCESSIBILITY, bindings: [], menu: null, children: [] };
}

/** @emoji 📚️ `ui_stack_vertical(children)` translated to the retained document's `Component::Container` in its `plain` role. */
function panelStack(key: string, children: readonly BuiltNode[]): BuiltNode {
  return {
    key,
    component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: null },
    layout: PANEL_STACK_LAYOUT,
    style: PANEL_STYLE,
    activity: "idle",
    disabled: false,
    accessibility: PANEL_ACCESSIBILITY,
    bindings: [],
    menu: null,
    children: [...children],
  };
}

/** @emoji 📥️ `ui_import_drop_zone` translated: a `plain` container carrying its `dropOverlay` title/hint/accept plus the two text children the Rust builder nests inside it, with `importFramePayload` bound to `Trigger::Drop`. */
function panelDropZone(key: string, title: string, hint: string, accept: string): BuiltNode {
  return {
    key,
    component: { type: "container", role: "plain", label: null, description: null, required: null, error: null, defaultOpen: null, dropOverlay: { title, hint, accept } },
    layout: { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "md" }, align: "stretch", justify: "start", grow: false, wrap: false },
    style: PANEL_STYLE,
    activity: "idle",
    disabled: false,
    accessibility: PANEL_ACCESSIBILITY,
    bindings: [{ trigger: "drop", action: { scope: REMODEL_EDITOR_CONTROLLER_ID, name: "importFramePayload", version: 1 }, args: null, capability: null }],
    menu: null,
    children: [panelText(`${key}-title`, title), panelText(`${key}-hint`, hint)],
  };
}

/** ⏯️ `ToolRunAction::{chord, label}` for the five actions the Reconstruction section lists, in both native locales. */
const REMODEL_TOOL_RUN_ACTIONS = [
  { id: "start", chord: "mod+enter", en: "Start", de: "Starten" },
  { id: "pause", chord: "mod+alt+enter", en: "Pause", de: "Pausieren" },
  { id: "step", chord: "mod+alt+arrowright", en: "Step", de: "Einzelschritt" },
  { id: "abort", chord: "mod+.", en: "Abort", de: "Abbrechen" },
  { id: "finalize", chord: "mod+shift+enter", en: "Finalize", de: "Abschließen" },
] as const;

/** 🗿️ `📌️panels/🗿️artifact` — the Reconstruction section as it renders without a live run: "No reconstruction run", the stored trajectory's camera count, and the tool-run chords. A live run is framework tool-run state no story owns. */
function pipelinePanel(scene: RemodelScene, config: RemodelConfig): BuiltNode {
  const german = config.locale.toLowerCase().startsWith("de");
  const say = (en: string, de: string): string => (german ? de : en);
  return panelStack("remodeling-pipeline", [
    panelText("remodeling-pipeline.reconstruction", say("Reconstruction", "Rekonstruktion")),
    panelText("remodeling-pipeline.run", say("No reconstruction run", "Kein Rekonstruktionslauf")),
    panelText("remodeling-pipeline.result", `${say("Stored cameras", "Gespeicherte Kameras")}: ${scene.results.trajectory?.poses.length ?? 0}`),
    ...REMODEL_TOOL_RUN_ACTIONS.map((action) => panelText(`remodeling-pipeline.keys.${action.id}`, `${action.chord} — ${german ? action.de : action.en}`)),
  ]);
}

/** 🗂️ `📌️panels/🗂️media` — the import drop zone, the stream/asset counts, and one line per stream (plus its decoded container facts). */
function mediaPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const lines: BuiltNode[] = [panelDropZone("remodeling-media-drop", labels.panelMedia, labels.noStreams, REMODEL_MEDIA_ACCEPT), panelText("media-counts", `${labels.streams}: ${scene.streams.length} - ${labels.assets}: ${Object.keys(scene.assets).length}`)];
  for (const stream of scene.streams) {
    const kindLabel = stream.kind === "video" ? labels.streamKindVideo : labels.streamKindImageSequence;
    lines.push(panelText(`media-${stream.id}`, `${stream.name} (${kindLabel}, ${stream.frames.length} ${labels.frames}, ${labels.syncOffset}: ${fixed(stream.syncOffsetMs, 1)}ms)`));
    if (stream.source) lines.push(panelText(`media-${stream.id}-source`, `  ${debugEnum(stream.source.codec)} ${stream.source.width}x${stream.source.height} ${fixed(stream.source.durationMs, 0)}ms`));
  }
  return panelStack("media", lines);
}

/** 🧵️ `📌️panels/🧵️results` — mesh source + vertex/triangle counts, then sparse/dense/trajectory/geo. */
function resultsPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const results = scene.results;
  // 🧩️ The composed mesh CHILD is unresolvable without a `durableArtifacts` map, so `unwrap_or_default()`
  // reports 0/0 rather than fabricating a count — exactly what the Rust panel does here.
  const meshLabel = `${labels.mesh}: ${debugEnum(results.mesh.source)}, 0 ${labels.vertices}, 0 ${labels.triangles}`;
  const sparse = results.sparse === null ? `${labels.sparseCloud}: ${labels.resultsNone}` : `${labels.sparseCloud}: ${Math.floor(base64ByteLength(results.sparse.points) / 4 / 3)}`;
  const dense = results.dense === null ? `${labels.denseCloud}: ${labels.resultsNone}` : `${labels.denseCloud}: ${Math.floor(base64ByteLength(results.dense.positions) / 4 / 3)}`;
  const trajectory = results.trajectory === null ? `${labels.trajectory}: ${labels.resultsNone}` : `${labels.trajectory}: ${results.trajectory.poses.length} ${labels.poses}`;
  const geo = results.geo === null ? `${labels.geoProducts}: ${labels.resultsNone}` : `${labels.geoProducts}: ${labels.available}`;
  return panelStack("results", [panelText("results-mesh", meshLabel), panelText("results-sparse", sparse), panelText("results-dense", dense), panelText("results-trajectory", trajectory), panelText("results-geo", geo)]);
}

/** @emoji 🧮️ Byte length behind a base64 payload — the story's stand-in for `PackedF32::to_f32_vec().len()`. */
function base64ByteLength(value: string): number {
  const padding = value.endsWith("==") ? 2 : value.endsWith("=") ? 1 : 0;
  return (value.length / 4) * 3 - padding;
}

/** ⚙️ `📌️panels/⚙️parameters` — one line per parameter group, formatted exactly as the Rust `format!` calls. */
function parametersPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const p = scene.params;
  return panelStack("parameters", [
    panelText("params-ingest", `${labels.paramsIngest}: ${labels.strideShort} ${p.ingest.frameSampleStride}, ${labels.maxShort} ${p.ingest.maxFrames}, ${labels.downscaleShort} ${p.ingest.downscaleLongEdgePx}px, min sharpness ${fixed(p.ingest.minSharpness, 2)}`),
    panelText("params-feature", `${labels.paramsFeature}: ${debugEnum(p.feature.detector)}, ${labels.targetShort} ${p.feature.targetCount}, ${labels.octavesShort} ${p.feature.octaves}`),
    panelText("params-matching", `${labels.paramsMatching}: ${debugEnum(p.matching.matcher)}, ${labels.ratioShort} ${fixed(p.matching.ratioTest, 2)}, ${labels.windowShort} ${p.matching.sequentialWindow}`),
    panelText("params-sfm", `${labels.paramsSfm}: ${labels.ransacShort} ${p.sfm.ransacIterations}, ${labels.minTrackShort} ${p.sfm.minTrackLength}, ${labels.baShort} ${p.sfm.baMaxIterations}`),
    panelText("params-dense", `${labels.paramsDense}: ${debugEnum(p.dense.resolution)}, ${labels.windowShort} ${p.dense.windowRadiusPx}px`),
    panelText("params-mesh", `${labels.paramsMesh}: ${labels.voxelShort} ${fixed(p.mesh.tsdfVoxelSizeMm, 1)}mm, ${labels.targetShort} ${p.mesh.decimateTargetTriangles}, watertight ${p.mesh.guaranteeWatertight}`),
    panelText("params-motion", `${labels.paramsMotion}: ${p.motion.enabled ? labels.enabled : labels.disabled}`),
    panelText("params-geo", `${labels.paramsGeo}: ${p.geo.enabled ? labels.enabled : labels.disabled}`),
  ]);
}

/** 🎯️ `📌️panels/🎯️calibration` — camera/rig counts, one line per calibrated camera, then the GCP list. */
function calibrationPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const lines: BuiltNode[] = [panelText("calibration-counts", `${labels.camerasCalibrated}: ${scene.calibration.cameras.length} - ${labels.rigExtrinsics}: ${scene.calibration.rig.length}`)];
  for (const camera of scene.calibration.cameras) lines.push(panelText(`calibration-${camera.id}`, `${camera.label} (${camera.model}): fx ${fixed(camera.fx, 1)} fy ${fixed(camera.fy, 1)}`));
  lines.push(panelText("calibration-gcps", `${labels.gcps}: ${scene.gcps.length}`));
  for (const gcp of scene.gcps) lines.push(panelText(`calibration-${gcp.id}`, `${gcp.name} [${fixed(gcp.worldPosition[0], 2)}, ${fixed(gcp.worldPosition[1], 2)}, ${fixed(gcp.worldPosition[2], 2)}] (${gcp.observations.length} obs)`));
  return panelStack("calibration", lines);
}

/** 🏃️ `📌️panels/🏃️tracks` — the track list, or the documented "not yet driven by the engine" empty state. */
function tracksPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  if (scene.results.tracks.length === 0) return panelStack("tracks", [panelText("tracks-none", labels.tracksNone), panelText("tracks-gap", labels.motionNotImplemented)]);
  const lines: BuiltNode[] = [panelText("tracks-count", `${labels.tracks}: ${scene.results.tracks.length}`)];
  for (const track of scene.results.tracks) lines.push(panelText(`tracks-${track.id}`, `${track.id} (${debugEnum(track.class)}): ${track.length} frames, ${fixed(track.meanSpeedMS, 2)} m/s`));
  return panelStack("tracks", lines);
}

/** ✅️ `📌️panels/✅️quality` — the QC report's metrics, its optional watertight block, and every warning. */
function qualityPanel(scene: RemodelScene, labels: RemodelLabels): BuiltNode {
  const qc = scene.results.qc;
  if (qc === null) return panelStack("quality", [panelText("quality-none", labels.qcNone)]);
  const lines: BuiltNode[] = [
    panelText("quality-reprojection", `${labels.qcReprojection}: ${fixed(qc.reprojectionRmsPx, 2)}px`),
    panelText("quality-track-length", `${labels.qcTrackLength}: ${fixed(qc.meanTrackLength, 1)}`),
    panelText("quality-registered", `${labels.qcRegisteredRatio}: ${fixed(qc.registeredFrameRatio * 100, 0)}%`),
    panelText("quality-dense", `${labels.qcDenseCoverage}: ${fixed(qc.denseCoverageRatio * 100, 0)}%`),
  ];
  if (qc.gcpCheckpointRmse !== null) lines.push(panelText("quality-gcp-rmse", `${labels.qcGcpRmse}: ${fixed(qc.gcpCheckpointRmse, 3)}m`));
  if (qc.watertight) {
    lines.push(panelText("quality-watertight", `${labels.qcWatertight}: ${qc.watertight.isWatertight}`));
    lines.push(panelText("quality-boundary", `${labels.qcBoundaryEdges}: ${qc.watertight.boundaryEdgeCount}`));
    lines.push(panelText("quality-components", `${labels.qcComponents}: ${qc.watertight.connectedComponents}`));
    lines.push(panelText("quality-euler", `${labels.qcEuler}: ${qc.watertight.eulerCharacteristic}`));
    if (qc.watertight.genus !== null) lines.push(panelText("quality-genus", `${labels.qcGenus}: ${qc.watertight.genus}`));
    lines.push(panelText("quality-fallback", `${labels.qcClosedFallback}: ${qc.watertight.closedFallbackUsed}`));
  }
  qc.warnings.forEach((warning, index) => lines.push(panelText(`quality-warning-${index}`, `⚠️ ${warning}`)));
  return panelStack("quality", lines);
}

/** 🪟️ `ArtifactEditor::render`'s panel half — one `BuiltNode` document per registered panel tab, keyed by tab id. */
export function remodelPanelDocument(panelId: RemodelPanelId, scene: RemodelScene, config: RemodelConfig): BuiltNode {
  const labels = remodelLabelsFor(config.locale);
  switch (panelId) {
    case "media":
      return mediaPanel(scene, labels);
    case "results":
      return resultsPanel(scene, labels);
    case "parameters":
      return parametersPanel(scene, labels);
    case "calibration":
      return calibrationPanel(scene, labels);
    case "tracks":
      return tracksPanel(scene, labels);
    case "quality":
      return qualityPanel(scene, labels);
    default:
      return pipelinePanel(scene, config);
  }
}
//#endregion 🔖️PanelDocuments

//#region 🔖️Reducer
/**
 * 🎮️ Story-local mirror of `command_from_action` → `RemodelingCommand::dispatch` for the config-only subset a
 * host surface can reach without a plugin runtime. Document-mutating commands (`addGcp`, `importFramePayload`,
 * the reconstruction tool run, …) are deliberately NOT emulated: they emit artifact mutations through the event-sourced
 * store, which no story owns. An unrecognized action is ignored, exactly as `command_from_action` returning
 * `None` leaves the dispatch a no-op.
 */
export function reduceRemodelStoryAction(config: RemodelConfig, descriptor: ActionDescriptor): RemodelConfig {
  const args = remodelActionArgs(descriptor.args);
  switch (descriptor.action) {
    case "setCamera": {
      const camera = args.camera as { readonly position?: readonly [number, number, number]; readonly target?: readonly [number, number, number]; readonly fov?: number } | undefined;
      if (!camera) return config;
      return { ...config, camera: { position: camera.position ?? config.camera.position, target: camera.target ?? config.camera.target, fov: camera.fov ?? config.camera.fov } };
    }
    case "setLayerVisibility": {
      const layer = String(args.layer ?? "");
      if (!(layer in config.layers)) return config;
      return { ...config, layers: { ...config.layers, [layer]: args.visible === undefined ? !config.layers[layer as keyof RemodelConfig["layers"]] : Boolean(args.visible) } };
    }
    case "setFrameCursor":
      return { ...config, frameCursor: { streamId: typeof args.streamId === "string" ? args.streamId : null, frameIndex: Number(args.frameIndex ?? 0) } };
    case "setReportTable":
      return { ...config, reportTable: String(args.table ?? config.reportTable) };
    case "setActiveUtility":
      return { ...config, activeUtilityId: String(args.utilityId ?? config.activeUtilityId) };
    case "setLocale":
      return { ...config, locale: String(args.locale ?? config.locale) };
    default:
      return config;
  }
}
//#endregion 🔖️Reducer
