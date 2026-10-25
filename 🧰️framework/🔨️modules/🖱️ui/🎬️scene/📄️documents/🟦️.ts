import type { IconName } from "@semio-tech/assets";
import { type SceneLane, type SceneLaneRef, sceneFromLanes } from "../🚚️lane-contract/🟦️.ts";

/** 🖼️ A 2D canvas surface scene payload — mirrors the wasm `componentScene` node's `canvas2d` field. */
export type Canvas2dScene = {
  readonly cameraX: number;
  readonly cameraY: number;
  readonly zoom: number;
  readonly layersJson: string;
  readonly framing?: import("../📷️framing/🟦️.ts").Canvas2dFraming | null;
  /** ⏯️ The base64url `ToolRunTraceDelta` paged to this window — see `World3dScene.toolRunTrace`. */
  readonly toolRunTrace?: string | null;
  /** 🚚️ The spine's lane manifest — see `CANVAS2D_SCENE_LANES`. */
  readonly lanes?: readonly SceneLaneRef[];
};

/** 📋️ A table surface scene payload — mirrors the wasm `componentScene` node's `table` field. */
export type TableScene = {
  readonly lanes?: readonly SceneLaneRef[];
  readonly columnsJson: string;
  readonly rowsJson: string;
  readonly selectionJson?: string;
  readonly rowDragMime?: string;
  readonly dropActionJson?: string;
  readonly sortJson?: string;
  readonly domainId?: string;
  readonly domainGranularityId?: string;
};

export const TABLE_SCENE_LANES: readonly SceneLane<TableScene>[] = [
  { lane: "columns", field: "columnsJson", bodyKey: "framework.scene.table.columns", optional: false },
  { lane: "rows", field: "rowsJson", bodyKey: "framework.scene.table.rows", optional: false },
];

/** 📊️ Restores every table row and column from its paged scene carriers. */
export function tableSceneFromLanes(spine: TableScene, laneTexts: ReadonlyMap<string, string>): TableScene {
  return sceneFromLanes(spine, laneTexts, TABLE_SCENE_LANES);
}

/** 🖌️ A 2D paint surface scene payload — mirrors the wasm `componentScene` node's `paint2d` field. */
export type Paint2dScene = {
  readonly documentSyncJson: string;
  readonly assetsJson: string;
  readonly cameraJson: string;
  readonly selectionJson: string;
  readonly pixelSelectionJson?:string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly brushSize: number;
  readonly brushOpacity: number;
  readonly brushColor: string;
  readonly brushHardness: number;
  readonly paintTarget:"pixels"|"mask";
  readonly maskValue:number;
  /** 🪣️ The session colour tolerance (0..255) every host's bucket and wand floods with. */
  readonly fillTolerance:number;
  readonly viewMode: string;
  readonly compositeViewportJson?: string;
  /** 🚚️ The spine's lane manifest — see {@link PAINT2D_SCENE_LANES}. */
  readonly lanes?: readonly SceneLaneRef[];
};

/** 🎨️ An icon-render preview surface scene payload — mirrors the wasm `componentScene` node's `iconRender` field. */
export type IconRenderScene = {
  readonly requestJson: string;
  readonly footer?: string;
  readonly frameJson?: string;
};

/** 🗂️ A virtual-file-system browser surface scene payload — mirrors the wasm `componentScene` node's `virtualFileSystem` field. */
export type VirtualFileSystemScene = {
  readonly schemaJson: string;
  readonly rowsJson: string;
  readonly selectedRowIdsJson?: string;
  readonly hoveredRowId?: string;
  readonly emptyMessage?: string;
  readonly dragDropEnabled?: boolean;
};

/** 🗺️ A tiled map surface scene payload — mirrors the wasm `componentScene` node's `tiledMap` field. */
export type TiledMapScene = {
  readonly mapDescriptorJson: string;
  readonly cameraJson: string;
  readonly renderMode: string;
  readonly vectorStyle: string;
  readonly lodMode: string;
  readonly tileUrlTemplate: string;
  readonly vectorTileUrlTemplate: string;
  readonly layerVisibilityJson: string;
  readonly layerStrokeScaleJson: string;
  readonly selectionJson: string;
  readonly hoverJson: string;
  readonly selectionMethod: string;
  readonly selectionMode: string;
  /** 🛤️ The spine's lane manifest — see {@link TILEDMAP_SCENE_LANES}. */
  readonly lanes?: readonly SceneLaneRef[];
};

/** 💡️ One placement candidate row of a board's handle-suggestions popup. */
export type Board2dSuggestionCandidate = {
  readonly index: number;
  readonly nodeLabel: string;
  readonly handleLabel: string;
  readonly icon?: string;
  readonly color?: string;
};

/** 🪟️ The one-shot handle-suggestions popup a board window has open — the board twin of the world's suggestion menu. `pending` means the slot is still resolving; an empty `candidates` with `pending: false` is the polite refusal ("no placement available"). */
export type Board2dSuggestionMenu = {
  readonly open: boolean;
  readonly x: number;
  readonly y: number;
  readonly windowId?: string;
  readonly handleId?: string;
  readonly hoveredIndex: number;
  readonly pending: boolean;
  readonly candidates: readonly Board2dSuggestionCandidate[];
};

/** 🧩️ A 2D board surface scene payload — mirrors the wasm `componentScene` node's `board2d` field. */
export type Board2dScene = {
  readonly snapshotJson: string;
  readonly cameraJson: string;
  readonly glyphCatalogsJson: string;
  readonly selectionJson: string;
  /** 🔗️ The ids the board paints highlighted, as a JSON id array — what the open time-travel draft references; absent highlights nothing. */
  readonly highlightedIdsJson?: string;
  readonly interactive: boolean;
  readonly hoveredId?: string;
  readonly activeUtility?: string;
  readonly selectionMethod: string;
  /** 👁️ Whether the board host strokes its world grid at all — independent of snapping. */
  readonly gridVisible?: boolean;
  readonly gridSnapEnabled: boolean;
  readonly gridFactor: number;
  /** 🎯️ Which granularity a pick may reach — the owning app's selectable-kind filter; absent reads as on. */
  readonly selectableNodes?: boolean;
  readonly selectableEdges?: boolean;
  readonly selectableHandles?: boolean;
  readonly suggestionOffset: number;
  readonly brushWeightsJson: string;
  readonly placementCompatibilityJson: string;
  readonly lodMode: string;
  /** 🕹️ Which selection-gumball handles this board's select utility composes, as `{"move":boolean,"rotate":boolean}`; absent leaves the engine default (both on). */
  readonly transformFlags?: string;
  /** 🖍️ World extent one area-brush CLICK paints, as `{"width":number,"height":number}`; absent leaves the engine default. A click-drag states its own rectangle. */
  readonly areaBrushSize?: string;
  /** 🧰️ The framework interaction domain this board's picks and hovers belong to; absent means the app declares none and the host publishes no `interactionHover`. */
  readonly domainId?: string;
  /** 🏷️ The owning domain's brush placement id prefixes in the board's wire form, `{"brushNodePrefix":string,"brushEdgePrefix":string}`, so every host mints the domain's ids; absent leaves the host's installed naming. */
  readonly placementNaming?: string;
  /** 📂️ The one-shot handle-suggestions popup this window has open, as a {@link Board2dSuggestionMenu} JSON string; absent is the closed state. */
  readonly suggestionMenuJson?: string;
  /** ⏯️ The base64url `ToolRunTraceDelta` paged to this board — carried outside the doc, see {@link BOARD2D_SCENE_LANES}. */
  readonly toolRunTrace?: string;
  /** 🧾️ The spine's lane manifest — see {@link BOARD2D_SCENE_LANES}. */
  readonly lanes?: readonly SceneLaneRef[];
};

/** 🏁️ A complete interaction publication address for an ink canvas. */
export type InkCanvasInteractionDomain = {
  readonly id: string;
  readonly granularityId: string;
};

/** 🖊️ An ink-canvas surface scene payload — mirrors the wasm `componentScene` node's `inkCanvas` field. `documentJson` is opaque to the framework: the owning program defines its shape, conventionally an array of items (e.g. stroke | shape | text | image) each carrying its own transform; `selectionJson` is a `string[]` of selected item ids. */
export type InkCanvasScene = {
  readonly documentJson: string;
  readonly selectionJson: string;
  readonly hoveredId?: string;
  readonly activeUtility: string;
  readonly viewMode: string;
  readonly interactive: boolean;
  /** ✅️ The complete interaction publication address; absent on read-only/non-domain surfaces. */
  readonly interactionDomain?: InkCanvasInteractionDomain;
};

/** ✒️ Renderer-to-plugin action names for ink-canvas surfaces (modeled after {@link nodeGraphActions}/{@link textEditorActions}). */
export const inkCanvasActions = {
  applyEvents: "inkApplyEvents",
  setCamera: "setCamera",
} as const;

/** 🗄️ A checkpoint ancestor-graph history view. `columnsJson` is a `HistoryColumn[]` array, newest checkpoint first. */
export type GraphTimelineScene = {
  readonly columnsJson: string;
};

/** 🧱️ A palette entry for a block kind insertable into a {@link BlockListScene}, contributed either by the host app's own built-ins or by a `playbookBlockKind` module contribution. */
export type BlockPaletteEntry = {
  readonly blockKind: string;
  readonly label: string;
  readonly iconId: IconName;
};

/** 🆚️ A before/after text diff surface scene payload — mirrors the wasm `componentScene` node's `diffView` field. */
export type DiffViewScene = {
  readonly before: string;
  readonly after: string;
  readonly language?: string;
  readonly mode?: "unified" | "split";
};

/** 📰️ One entry of an {@link EventFeedScene}'s `entriesJson` array. */
export type EventFeedEntry = {
  readonly id: string;
  readonly timestampMs: number;
  readonly iconId: IconName;
  readonly title: string;
  readonly detail?: string;
  readonly tone?: string;
};

/** 📜️ A scrolling event/log feed surface scene payload — mirrors the wasm `componentScene` node's `eventFeed` field. `entriesJson` is an {@link EventFeedEntry}`[]` array. */
export type EventFeedScene = {
  readonly entriesJson: string;
  readonly follow?: boolean;
  readonly activateAction?: string;
};
