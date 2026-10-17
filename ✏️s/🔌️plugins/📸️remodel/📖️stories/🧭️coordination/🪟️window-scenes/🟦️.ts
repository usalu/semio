/** 🪟️ The remodel story constants and the per-window scene projections: main and viewer world, frames canvas, report table. */
import type { ActionDescriptor, UiComponentSceneNode } from "../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import type { Canvas2dScene, TableScene, World3dScene } from "@semio-tech/framework/ui/scene";
import type { RemodelConfig, RemodelScene } from "../📐️scene-model/🟦️.ts";

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
export const REMODEL_MEDIA_ACCEPT = "image/png,image/jpeg,video/mp4,video/quicktime,video/webm,video/x-msvideo,.png,.jpg,.jpeg,.mp4,.mov,.webm,.avi";

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
/** 🔢️ Rust's `{:.N}` — fixed decimals, never JS's shortest round-trip. */
export function fixed(value: number, decimals: number): string {
  return value.toFixed(decimals);
}

/** 🐍️ Rust's `{:?}` over a serde-kebab enum tag: the fixtures store `"brute-force"`, `Debug` prints `BruteForce`. */
export function debugEnum(tag: string): string {
  return tag
    .split("-")
    .map((part) => part.charAt(0).toUpperCase() + part.slice(1))
    .join("");
}

/** 🔡️ `descriptor.args` narrowed to a plain record — the shape every host dispatches. */
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

/** 🧮️ `PackedF32::from_f32_slice` — little-endian f32 triples, base64'd, byte-for-byte the wire shape `WorldPointCloudLayer` decodes. */
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
