/** 🧾️ InkCanvasHost model: the ink document, item, event and gesture types and the scene and selection id parsers. */
//#region 🔖️InkCanvasHost
//#region Types
export type Vec2 = readonly [number, number];

export type InkCamera = { readonly x: number; readonly y: number; readonly zoom: number };

export interface InkTextRun {
  readonly text: string;
  readonly bold?: boolean;
  readonly italic?: boolean;
  readonly underline?: boolean;
  readonly link?: string;
}

export interface InkTextParagraph {
  readonly runs: readonly InkTextRun[];
}

export interface InkItemBase {
  readonly id: string;
  readonly interactionId?: string;
  readonly name: string;
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
  readonly rotation?: number;
  readonly visible: boolean;
  readonly locked: boolean;
}

export interface InkTextItem extends InkItemBase {
  readonly kind: "text";
  readonly paragraphs: readonly InkTextParagraph[];
  readonly fontSize: number;
  readonly fontWeight: "normal" | "bold";
  readonly align: "left" | "center" | "right";
}

export interface InkImageAsset {
  readonly mime: string;
  readonly data: string;
  readonly width?: number;
  readonly height?: number;
}

export interface InkImageItem extends InkItemBase {
  readonly kind: "image";
  readonly imageKey: string;
}

export interface InkTableCell {
  readonly content: string;
}

export interface InkTableItem extends InkItemBase {
  readonly kind: "table";
  readonly columns: readonly string[];
  readonly rows: readonly (readonly InkTableCell[])[];
}

export interface InkMathItem extends InkItemBase {
  readonly kind: "math";
  readonly tex: string;
  readonly displayMode: boolean;
}

export interface InkStrokeItem extends InkItemBase {
  readonly kind: "stroke";
  readonly points: readonly Vec2[];
  readonly strokeWidth: number;
  readonly color: readonly [number, number, number, number];
}

export interface InkGroupItem extends InkItemBase {
  readonly kind: "group";
  readonly children: readonly InkItem[];
}

export type InkItem = InkTextItem | InkImageItem | InkTableItem | InkMathItem | InkStrokeItem | InkGroupItem;
export type InkItemKind = InkItem["kind"];

export interface InkDocument {
  readonly schema: "ink.document";
  readonly id: string;
  readonly title?: string;
  readonly camera: InkCamera;
  readonly blocks: readonly InkItem[];
  readonly assets?: Readonly<Record<string, InkImageAsset>>;
  readonly gridVisible?: boolean;
  readonly gridSpacing?: number;
  readonly gridSubdivisions?: number;
  readonly gridOpacity?: number;
  readonly snapEnabled?: boolean;
  readonly snapGridSpacing?: number;
  readonly pencilWidth?: number;
  readonly eraserRadius?: number;
}

export type InkResizeHandle = "nw" | "n" | "ne" | "e" | "se" | "s" | "sw" | "w";

export interface InkBounds {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
}

export type InkCanvasEvent =
  | { readonly operation: "addBlock"; readonly block: InkItem; readonly parentId?: string | null; readonly index?: number | null }
  | { readonly operation: "updateBlock"; readonly blockId: string; readonly block: InkItem }
  | { readonly operation: "removeBlock"; readonly blockId: string }
  | { readonly operation: "putAsset"; readonly key: string; readonly asset: InkImageAsset }
  | { readonly operation: "setCamera"; readonly camera: InkCamera };

/** 🤏️ The host's record of a block drag — the offset from the gesture's start — which the plugin yields as ONE relative
 * `drag-blocks` leaf; the per-block events of a drag only feed the local draft. */
export interface InkDragGesture {
  readonly kind: "drag";
  readonly ids: readonly string[];
  readonly dx: number;
  readonly dy: number;
}

/** 🌊️ What the next live frame publishes: every event since the last frame (an eraser's removals must all arrive) and
 * the latest drag record. */
export interface InkPendingLive {
  readonly events: readonly InkCanvasEvent[];
  readonly gesture?: InkDragGesture;
}

export function parseInkScene(documentJson: string | undefined): InkDocument | null {
  if (!documentJson) return null;
  try {
    const parsed = JSON.parse(documentJson) as Partial<InkDocument>;
    if (parsed.schema !== "ink.document" || !Array.isArray(parsed.blocks)) return null;
    return parsed as InkDocument;
  } catch {
    return null;
  }
}

export function parseSelectionIds(json: string | undefined): readonly string[] {
  if (!json) return [];
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((id): id is string => typeof id === "string") : [];
  } catch {
    return [];
  }
}
//#endregion Types
//#endregion 🔖️InkCanvasHost
