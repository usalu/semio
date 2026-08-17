/** 🖼️ Note viewer — Composite window: typed twin of `🦀️component.rs`'s view-model. Read-only mirror
 * of the ink-canvas scene payload `render()` produces — no camera field (hardcoded default, never
 * threaded through), no active-utility field, no engagement-shaped fields, matching the viewer's
 * `ViewEmit`-only contract. */

export interface NoteViewBlockNode {
  kind: string;
  id: string;
  [key: string]: unknown;
}

export interface NoteViewImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

/** 👁️ `render(document: &NoteSnapshot)`'s sole input — a bare snapshot, no runtime/config/camera
 * state: a viewer has none of those. */
export interface NoteViewSnapshot {
  schema: string;
  id: string;
  title?: string;
  blocks: NoteViewBlockNode[];
  gridVisible?: boolean;
  gridSpacing?: number;
  gridSubdivisions?: number;
  gridOpacity?: number;
  snapEnabled?: boolean;
  snapGridSpacing?: number;
  pencilWidth?: number;
  eraserRadius?: number;
  assets: Record<string, NoteViewImageAsset>;
}

/** 👁️ The Composite window's typed view-model — the TS mirror of the Rust `render()` boundary. */
export interface NoteViewCompositeViewModel {
  windowKindId: "note-view-composite";
  bodyKey: "note.view.composite";
  surfaceId: "note.view.composite";
  document: NoteViewSnapshot;
  /** @see InkCanvasScene.interactive — always `false`: the viewer never passes `true`. */
  interactive: false;
}

export const NOTE_VIEW_COMPOSITE_WINDOW_KIND_ID = "note-view-composite" as const;
export const NOTE_VIEW_COMPOSITE_BODY_KEY = "note.view.composite" as const;
export const NOTE_VIEW_COMPOSITE_SURFACE_ID = "note.view.composite" as const;
