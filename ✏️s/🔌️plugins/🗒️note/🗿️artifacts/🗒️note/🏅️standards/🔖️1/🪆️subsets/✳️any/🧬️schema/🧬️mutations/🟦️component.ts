/** 🗒️ Note direct-mutation discriminated union. */
import type { NoteBlockNode, NoteImageAsset } from "../🟦️component.ts";

/** 📝️ One rich-text run inside a [`NoteTextParagraph`]. */
export interface NoteTextRun {
  text: string;
  bold?: boolean;
  italic?: boolean;
  underline?: boolean;
  link?: string;
}

/** 📝️ One rich-text paragraph — a text block's content unit. */
export interface NoteTextParagraph {
  runs: NoteTextRun[];
}

/** 🏷️ `rename-note` payload — the document's new (or cleared) title. */
export interface RenameNote {
  newTitle: string | null;
}

/** 👁️ `change-grid-visible` payload — the grid's new (or cleared) visibility. */
export interface ChangeGridVisible {
  newVisible: boolean | null;
}

/** 📏️ `change-grid-spacing` payload — the grid's new (or cleared) spacing. */
export interface ChangeGridSpacing {
  newSpacing: number | null;
}

/** 🔢️ `change-grid-subdivisions` payload — the grid's new (or cleared) subdivisions. */
export interface ChangeGridSubdivisions {
  newSubdivisions: number | null;
}

/** 🌫️ `change-grid-opacity` payload — the grid's new (or cleared) opacity. */
export interface ChangeGridOpacity {
  newOpacity: number | null;
}

/** 🧲️ `change-snap-enabled` payload — snapping's new (or cleared) enabled flag. */
export interface ChangeSnapEnabled {
  newEnabled: boolean | null;
}

/** 📐️ `change-snap-grid-spacing` payload — the snap grid's new (or cleared) spacing. */
export interface ChangeSnapGridSpacing {
  newSpacing: number | null;
}

/** ✏️ `change-pencil-width` payload — the pencil tool's new (or cleared) width. */
export interface ChangePencilWidth {
  newWidth: number | null;
}

/** 🧽️ `change-eraser-radius` payload — the eraser tool's new (or cleared) radius. */
export interface ChangeEraserRadius {
  newRadius: number | null;
}

/** 🆕️ `create-asset` payload — adds a new id-keyed image asset. */
export interface CreateAsset {
  key: string;
  asset: NoteImageAsset;
}

/** 🔁️ `replace-asset-payload` payload — whole-value swap of an id-keyed image asset's payload. */
export interface ReplaceAssetPayload {
  key: string;
  newAsset: NoteImageAsset;
}

/** 🗑️ `delete-asset` payload — removes an id-keyed image asset. */
export interface DeleteAsset {
  key: string;
}

/** ➕️ `create-block` payload — inserts `block` under `parentId` at `index` (both nullable/final-state). */
export interface CreateBlock {
  block: NoteBlockNode;
  parentId: string | null;
  index: number | null;
}

/** ❌️ `delete-block` payload — the block's id. */
export interface DeleteBlock {
  id: string;
}

/** 🧺️ `delete-blocks` payload — the ids of every block to remove. */
export interface DeleteBlocks {
  ids: string[];
}

/** 🎯️ `duplicate-block` payload — copies `sourceId` into the new `block`. */
export interface DuplicateBlock {
  sourceId: string;
  block: NoteBlockNode;
}

/** 👥️ `duplicate-blocks` payload — copies every `sourceIds` entry into the parallel `blocks` list. */
export interface DuplicateBlocks {
  sourceIds: string[];
  blocks: NoteBlockNode[];
}

/** 🚚️ `move-block-to-container` payload — reparents a block under `newParentId` at `index`. */
export interface MoveBlockToContainer {
  id: string;
  newParentId: string | null;
  index: number;
}

/** 🤏️ `drag-blocks` payload — nudges every addressed block (and its subtree) by `dx`/`dy`. */
export interface DragBlocks {
  ids: string[];
  dx: number;
  dy: number;
}

/** 🔖️ `rename-block` payload — the block's new `name`. */
export interface RenameBlock {
  id: string;
  newName: string;
}

/** 👀️ `change-block-visible` payload — the block's new `visible` flag. */
export interface ChangeBlockVisible {
  id: string;
  newVisible: boolean;
}

/** 🔒️ `change-block-locked` payload — the block's new `locked` flag. */
export interface ChangeBlockLocked {
  id: string;
  newLocked: boolean;
}

/** 📍️ `move-block` payload — the block's new absolute position. */
export interface MoveBlock {
  id: string;
  newX: number;
  newY: number;
}

/** ↔️ `resize-block` payload — the block's new extent. */
export interface ResizeBlock {
  id: string;
  newWidth: number;
  newHeight: number;
}

/** 🔤️ `change-block-font-size` payload — a text block's new font size. */
export interface ChangeBlockFontSize {
  id: string;
  newFontSize: number;
}

/** 📝️ `edit-block-text` payload — whole-value swap of a text block's paragraphs. */
export interface EditBlockText {
  id: string;
  newParagraphs: NoteTextParagraph[];
}

/** 🧮️ `edit-block-math` payload — a math block's new TeX source. */
export interface EditBlockMath {
  id: string;
  newTex: string;
}

/** 🖊️ `change-block-ink-width` payload — an ink block's new stroke width. */
export interface ChangeBlockInkWidth {
  id: string;
  newStrokeWidth: number;
}

/** 🎨️ `edit-block-ink-stroke` payload — whole-value swap of an ink block's polyline + bounds. */
export interface EditBlockInkStroke {
  id: string;
  newPoints: [number, number][];
  newX: number;
  newY: number;
  newWidth: number;
  newHeight: number;
}

/** ⬇️ `insert-table-row` payload — appends a blank row to the table block. */
export interface InsertTableRow {
  id: string;
}

/** ⬆️ `remove-table-row` payload — drops the trailing row from the table block. */
export interface RemoveTableRow {
  id: string;
}

/** ➡️ `insert-table-column` payload — appends a lettered column to the table block. */
export interface InsertTableColumn {
  id: string;
}

/** ⬅️ `remove-table-column` payload — drops the trailing column from the table block. */
export interface RemoveTableColumn {
  id: string;
}

export type NoteMutation =
  | ({ mutation: "renameNote" } & RenameNote)
  | ({ mutation: "changeGridVisible" } & ChangeGridVisible)
  | ({ mutation: "changeGridSpacing" } & ChangeGridSpacing)
  | ({ mutation: "changeGridSubdivisions" } & ChangeGridSubdivisions)
  | ({ mutation: "changeGridOpacity" } & ChangeGridOpacity)
  | ({ mutation: "changeSnapEnabled" } & ChangeSnapEnabled)
  | ({ mutation: "changeSnapGridSpacing" } & ChangeSnapGridSpacing)
  | ({ mutation: "changePencilWidth" } & ChangePencilWidth)
  | ({ mutation: "changeEraserRadius" } & ChangeEraserRadius)
  | ({ mutation: "createAsset" } & CreateAsset)
  | ({ mutation: "replaceAssetPayload" } & ReplaceAssetPayload)
  | ({ mutation: "deleteAsset" } & DeleteAsset)
  | ({ mutation: "createBlock" } & CreateBlock)
  | ({ mutation: "deleteBlock" } & DeleteBlock)
  | ({ mutation: "deleteBlocks" } & DeleteBlocks)
  | ({ mutation: "duplicateBlock" } & DuplicateBlock)
  | ({ mutation: "duplicateBlocks" } & DuplicateBlocks)
  | ({ mutation: "moveBlockToContainer" } & MoveBlockToContainer)
  | ({ mutation: "dragBlocks" } & DragBlocks)
  | ({ mutation: "renameBlock" } & RenameBlock)
  | ({ mutation: "changeBlockVisible" } & ChangeBlockVisible)
  | ({ mutation: "changeBlockLocked" } & ChangeBlockLocked)
  | ({ mutation: "moveBlock" } & MoveBlock)
  | ({ mutation: "resizeBlock" } & ResizeBlock)
  | ({ mutation: "changeBlockFontSize" } & ChangeBlockFontSize)
  | ({ mutation: "editBlockText" } & EditBlockText)
  | ({ mutation: "editBlockMath" } & EditBlockMath)
  | ({ mutation: "changeBlockInkWidth" } & ChangeBlockInkWidth)
  | ({ mutation: "editBlockInkStroke" } & EditBlockInkStroke)
  | ({ mutation: "insertTableRow" } & InsertTableRow)
  | ({ mutation: "removeTableRow" } & RemoveTableRow)
  | ({ mutation: "insertTableColumn" } & InsertTableColumn)
  | ({ mutation: "removeTableColumn" } & RemoveTableColumn);
