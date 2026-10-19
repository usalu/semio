/** 🧩️ The remodel panel tab documents, built as retained nodes from the scene and labels. */
import type { AccessibilitySpec, BuiltNode, LayoutSpec, StyleSpec } from "@semio-tech/framework/manifest";
import type { RemodelConfig, RemodelScene } from "../📐️scene-model/🟦️.ts";
import { type RemodelLabels, remodelLabelsFor } from "../🗣️labels/🟦️.ts";
import { debugEnum, fixed, REMODEL_EDITOR_CONTROLLER_ID, REMODEL_MEDIA_ACCEPT, type RemodelPanelId } from "../🪟️window-scenes/🟦️.ts";

//#region 🔖️PanelDocuments
const PANEL_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const PANEL_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };
const PANEL_STACK_LAYOUT: LayoutSpec = { kind: "stack", axis: "vertical", gap: "sm", padding: { all: "sm" }, align: "stretch", justify: "start", grow: false, wrap: false };
const PANEL_LEAF_LAYOUT: LayoutSpec = { kind: "leaf", width: "fill", height: "hug" };

/** 🔤️ `ui_text(Label::data(…))` translated to the retained document's `Component::Text`. */
function panelText(key: string, value: string): BuiltNode {
  return { key, component: { type: "text", value, emphasize: null, dataAttributes: null }, layout: PANEL_LEAF_LAYOUT, style: PANEL_STYLE, activity: "idle", disabled: false, accessibility: PANEL_ACCESSIBILITY, bindings: [], menu: null, children: [] };
}

/** 📚️ `ui_stack_vertical(children)` translated to the retained document's `Component::Container` in its `plain` role. */
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

/** 📥️ `ui_import_drop_zone` translated: a `plain` container carrying its `dropOverlay` title/hint/accept plus the two text children the Rust builder nests inside it, with `importFramePayload` bound to `Trigger::Drop`. */
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
  const meshLabel = `${labels.mesh}: ${debugEnum(results.mesh.source)}, 0 ${labels.vertices}, 0 ${labels.triangles}`;
  const sparse = results.sparse === null ? `${labels.sparseCloud}: ${labels.resultsNone}` : `${labels.sparseCloud}: ${Math.floor(base64ByteLength(results.sparse.points) / 4 / 3)}`;
  const dense = results.dense === null ? `${labels.denseCloud}: ${labels.resultsNone}` : `${labels.denseCloud}: ${Math.floor(base64ByteLength(results.dense.positions) / 4 / 3)}`;
  const trajectory = results.trajectory === null ? `${labels.trajectory}: ${labels.resultsNone}` : `${labels.trajectory}: ${results.trajectory.poses.length} ${labels.poses}`;
  const geo = results.geo === null ? `${labels.geoProducts}: ${labels.resultsNone}` : `${labels.geoProducts}: ${labels.available}`;
  return panelStack("results", [panelText("results-mesh", meshLabel), panelText("results-sparse", sparse), panelText("results-dense", dense), panelText("results-trajectory", trajectory), panelText("results-geo", geo)]);
}

/** 🧮️ Byte length behind a base64 payload — the story's stand-in for `PackedF32::to_f32_vec().len()`. */
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
