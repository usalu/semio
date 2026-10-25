import type { World3dScene } from "../🌐️world3d/🟦️.ts";
import type { Board2dScene, Canvas2dScene, Paint2dScene, TiledMapScene } from "../📄️documents/🟦️.ts";
import { type SceneLane, sceneFromLanes } from "../🚚️lane-contract/🟦️.ts";
export { type SceneLane, type SceneLaneRef, sceneFromLanes } from "../🚚️lane-contract/🟦️.ts";

//#region 🚚️World3dSceneLanes
/** 🚚️ One world-3d payload lane. */
export type World3dSceneLane = SceneLane<World3dScene>;

/** 🔑️ Reserved carrier-key namespace — dotted and `framework.`-prefixed so a lane root can never
 * collide with an app-authored node id. */
export const WORLD3D_SCENE_LANE_KEY_PREFIX = "framework.scene.world3d.";

/** 🛤️ The twenty-five world-3d payload fields that ride OUTSIDE the fixed-capacity surface doc, each as
 * its own retained, individually paged text carrier. `SurfaceDoc.bytes` is a hard 32 KiB
 * `UiFixedBytes` ceiling that cannot page, so a world whose payload scales with its document (a
 * measured 57 281-byte Nakagin Capsule Tower) can only publish with the payload split out; keeping the
 * lanes SEPARATE is what lets an unchanged lane cost nothing on a partial refresh.
 *
 * Mirrors the Rust `World3dSceneLane` / `WORLD3D_SCENE_LANE_*` in
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🌍️world3d/🦀️.rs`. Both sides are pinned against
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️world3d-scene-lanes/🔣️.json`. */
export const WORLD3D_SCENE_LANES: readonly World3dSceneLane[] = [
  { lane: "meshes", field: "meshesJson", bodyKey: "framework.scene.world3d.meshes", optional: false },
  { lane: "instances", field: "instancesJson", bodyKey: "framework.scene.world3d.instances", optional: false },
  { lane: "instancesDelta", field: "instancesDeltaJson", bodyKey: "framework.scene.world3d.instancesDelta", optional: true },
  { lane: "selection", field: "selectionJson", bodyKey: "framework.scene.world3d.selection", optional: false },
  { lane: "vortices", field: "vorticesJson", bodyKey: "framework.scene.world3d.vortices", optional: true },
  { lane: "attractions", field: "attractionsJson", bodyKey: "framework.scene.world3d.attractions", optional: true },
  { lane: "targetVolumes", field: "targetVolumesJson", bodyKey: "framework.scene.world3d.targetVolumes", optional: true },
  { lane: "references", field: "referencesJson", bodyKey: "framework.scene.world3d.references", optional: true },
  { lane: "brushPreview", field: "brushPreviewJson", bodyKey: "framework.scene.world3d.brushPreview", optional: true },
  { lane: "interaction", field: "interactionJson", bodyKey: "framework.scene.world3d.interaction", optional: true },
  { lane: "engagementPreview", field: "engagementPreviewJson", bodyKey: "framework.scene.world3d.engagementPreview", optional: true },
  { lane: "pickTargets", field: "pickTargetsJson", bodyKey: "framework.scene.world3d.pickTargets", optional: true },
  { lane: "lod", field: "lodJson", bodyKey: "framework.scene.world3d.lod", optional: true },
  { lane: "presentation", field: "presentationJson", bodyKey: "framework.scene.world3d.presentation", optional: true },
  { lane: "chunking", field: "chunkingJson", bodyKey: "framework.scene.world3d.chunking", optional: true },
  { lane: "environment", field: "environmentJson", bodyKey: "framework.scene.world3d.environment", optional: true },
  { lane: "frame", field: "frameJson", bodyKey: "framework.scene.world3d.frame", optional: true },
  { lane: "fit", field: "fitJson", bodyKey: "framework.scene.world3d.fit", optional: true },
  { lane: "terrain", field: "terrainJson", bodyKey: "framework.scene.world3d.terrain", optional: true },
  { lane: "points", field: "pointsJson", bodyKey: "framework.scene.world3d.points", optional: true },
  { lane: "status", field: "statusJson", bodyKey: "framework.scene.world3d.status", optional: true },
  { lane: "toolRunTrace", field: "toolRunTrace", bodyKey: "framework.scene.world3d.toolRunTrace", optional: true },
  { lane: "annotations", field: "annotations", bodyKey: "framework.scene.world3d.annotations", optional: true, encoding: "json" },
  { lane: "scalarField", field: "scalarField", bodyKey: "framework.scene.world3d.scalarField", optional: true, encoding: "json" },
  { lane: "modellingOptions", field: "modellingOptions", bodyKey: "framework.scene.world3d.modellingOptions", optional: true, encoding: "json" },
];

/** 🗝️ Resolves a retained node key back to the lane it carries, `undefined` for every other key. */
export function world3dSceneLaneForBodyKey(bodyKey: string): World3dSceneLane | undefined {
  return WORLD3D_SCENE_LANES.find((lane) => lane.bodyKey === bodyKey);
}

/** 🌐️ {@link sceneFromLanes} over {@link WORLD3D_SCENE_LANES}. */
export function world3dSceneFromLanes(spine: World3dScene, laneTexts: ReadonlyMap<string, string>): World3dScene {
  return sceneFromLanes(spine, laneTexts, WORLD3D_SCENE_LANES);
}
//#endregion 🚚️World3dSceneLanes

//#region 🚚️Canvas2dSceneLanes
/** 🔐️ Reserved carrier-key namespace of the canvas-2d lanes. */
export const CANVAS2D_SCENE_LANE_KEY_PREFIX = "framework.scene.canvas2d.";

/** 🖼️ The canvas-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — mirrors the Rust
 * `Canvas2dSceneLane` / `CANVAS2D_SCENE_LANE_*`; both pinned against
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️canvas2d-scene-lanes/🔣️.json`. */
export const CANVAS2D_SCENE_LANES: readonly SceneLane<Canvas2dScene>[] = [
  { lane: "toolRunTrace", field: "toolRunTrace", bodyKey: "framework.scene.canvas2d.toolRunTrace", optional: true },
];

/** 🧷️ Resolves a retained node key back to the canvas-2d lane it carries. */
export function canvas2dSceneLaneForBodyKey(bodyKey: string): SceneLane<Canvas2dScene> | undefined {
  return CANVAS2D_SCENE_LANES.find((lane) => lane.bodyKey === bodyKey);
}

/** 🔲️ {@link sceneFromLanes} over {@link CANVAS2D_SCENE_LANES}. */
export function canvas2dSceneFromLanes(spine: Canvas2dScene, laneTexts: ReadonlyMap<string, string>): Canvas2dScene {
  return sceneFromLanes(spine, laneTexts, CANVAS2D_SCENE_LANES);
}
//#endregion 🚚️Canvas2dSceneLanes

//#region 🚚️Board2dSceneLanes
/** 🏷️ Reserved carrier-key namespace of the board-2d lanes. */
export const BOARD2D_SCENE_LANE_KEY_PREFIX = "framework.scene.board2d.";

/** 🎲️ The board-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — mirrors the Rust
 * `Board2dSceneLane` / `BOARD2D_SCENE_LANE_*`; both pinned against
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️board2d-scene-lanes/🔣️.json`. */
export const BOARD2D_SCENE_LANES: readonly SceneLane<Board2dScene>[] = [
  { lane: "snapshot", field: "snapshotJson", bodyKey: "framework.scene.board2d.snapshot", optional: false },
  { lane: "toolRunTrace", field: "toolRunTrace", bodyKey: "framework.scene.board2d.toolRunTrace", optional: true },
];

/** 🪪️ Resolves a retained node key back to the board-2d lane it carries. */
export function board2dSceneLaneForBodyKey(bodyKey: string): SceneLane<Board2dScene> | undefined {
  return BOARD2D_SCENE_LANES.find((lane) => lane.bodyKey === bodyKey);
}

/** 🕹️ {@link sceneFromLanes} over {@link BOARD2D_SCENE_LANES}. */
export function board2dSceneFromLanes(spine: Board2dScene, laneTexts: ReadonlyMap<string, string>): Board2dScene {
  return sceneFromLanes(spine, laneTexts, BOARD2D_SCENE_LANES);
}
//#endregion 🚚️Board2dSceneLanes

//#region 🚚️TiledMapSceneLanes
/** 📛️ Reserved carrier-key namespace of the tiled-map lanes. */
export const TILEDMAP_SCENE_LANE_KEY_PREFIX = "framework.scene.tiledmap.";

/** 🀄️ The tiled-map payload field that rides OUTSIDE the fixed-capacity surface doc — mirrors the Rust
 * `TiledMapSceneLane` / `TILEDMAP_SCENE_LANE_*`; both pinned against
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️tiledmap-scene-lanes/🔣️.json`. The map descriptor
 * scales with the document (one opaque payload per position/route/region), so it outgrows the 32 KiB
 * surface doc on a real map — the gis `demo` map is 59 667 bytes. */
export const TILEDMAP_SCENE_LANES: readonly SceneLane<TiledMapScene>[] = [
  { lane: "mapDescriptor", field: "mapDescriptorJson", bodyKey: "framework.scene.tiledmap.mapDescriptor", optional: false },
];

/** 📍️ Resolves a retained node key back to the tiled-map lane it carries. */
export function tiledMapSceneLaneForBodyKey(bodyKey: string): SceneLane<TiledMapScene> | undefined {
  return TILEDMAP_SCENE_LANES.find((lane) => lane.bodyKey === bodyKey);
}

/** 🗺️ {@link sceneFromLanes} over {@link TILEDMAP_SCENE_LANES}. */
export function tiledMapSceneFromLanes(spine: TiledMapScene, laneTexts: ReadonlyMap<string, string>): TiledMapScene {
  return sceneFromLanes(spine, laneTexts, TILEDMAP_SCENE_LANES);
}
//#endregion 🚚️TiledMapSceneLanes

/** 🔓️ Reserved carrier-key namespace of the paint-2d lanes. */
export const PAINT2D_SCENE_LANE_KEY_PREFIX = "framework.scene.paint2d.";

/** 🖌️ The paint-2d payload fields that ride OUTSIDE the fixed-capacity surface doc — mirrors the Rust
 * `Paint2dSceneLane` / `PAINT2D_SCENE_LANE_*`; both pinned against
 * `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🚚️paint2d-scene-lanes/🔣️.json`. Document, assets and completed
 * pixel coverage travel separately so the 32 KiB scene header stays bounded. */
export const PAINT2D_SCENE_LANES: readonly SceneLane<Paint2dScene>[] = [
  { lane: "documentSync", field: "documentSyncJson", bodyKey: "framework.scene.paint2d.documentSync", optional: false },
  { lane: "assets", field: "assetsJson", bodyKey: "framework.scene.paint2d.assets", optional: false },
  { lane: "pixelSelection", field: "pixelSelectionJson", bodyKey: "framework.scene.paint2d.pixelSelection", optional: true },
];

/** 🎫️ Resolves a retained node key back to the paint-2d lane it carries. */
export function paint2dSceneLaneForBodyKey(bodyKey: string): SceneLane<Paint2dScene> | undefined {
  return PAINT2D_SCENE_LANES.find((lane) => lane.bodyKey === bodyKey);
}

/** 🎨️ {@link sceneFromLanes} over {@link PAINT2D_SCENE_LANES}. */
export function paint2dSceneFromLanes(spine: Paint2dScene, laneTexts: ReadonlyMap<string, string>): Paint2dScene {
  return sceneFromLanes(spine, laneTexts, PAINT2D_SCENE_LANES);
}
