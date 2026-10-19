/** 🗣️ The complete English and German remodel label sets and the locale lookup. */
import labels from "./🔣️.json";

//#region 🔖️Labels
/** 🗣️ One locale's resolution of `RemodelingLabels` — one field per `app_labels!` row, native variant. */
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
const REMODEL_LABELS_EN: RemodelLabels = labels.en;

/** 🇩🇪️ `native_de` column of `app_labels! { RemodelingLabels }`, transcribed verbatim (umlauts included, as authored). */
const REMODEL_LABELS_DE: RemodelLabels = labels.de;

/** 🌐️ Story-local mirror of `resolve_labels_for_locale::<RemodelingLabels>` — any non-`de` BCP-47 tag falls back to English, matching the framework resolver's language-prefix match. */
export function remodelLabelsFor(locale: string): RemodelLabels {
  return locale.toLowerCase().startsWith("de") ? REMODEL_LABELS_DE : REMODEL_LABELS_EN;
}
//#endregion 🔖️Labels
