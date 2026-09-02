/** 🖼️ Note editor — Composite window: typed twin of `🦀️.rs`'s `render()` boundary (the
 * full infinite-canvas ink-canvas scene: the note document plus the live, session-only camera and
 * active drawing utility this window's scene composes over). */

/** 📝 One block on the canvas — mirrors `NoteBlockNode`'s tagged-union shape; kept as an opaque bag
 * beyond `kind`/`id` here (six variant shapes, each already handcrafted in the schema mirror this
 * window's document field carries) rather than re-declaring all six inline. */
export interface NoteCompositeBlockNode {
  kind: string;
  id: string;
  [key: string]: unknown;
}

export interface NoteCompositeImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

/** 📸️ `render(document: &NoteSnapshot, cfg: &NoteConfig)`'s document input. */
export interface NoteCompositeSnapshot {
  schema: string;
  id: string;
  title?: string;
  blocks: NoteCompositeBlockNode[];
  gridVisible?: boolean;
  gridSpacing?: number;
  gridSubdivisions?: number;
  gridOpacity?: number;
  snapEnabled?: boolean;
  snapGridSpacing?: number;
  pencilWidth?: number;
  eraserRadius?: number;
  assets: Record<string, NoteCompositeImageAsset>;
}

/** 📷️ Session-only camera — never part of `NoteSnapshot`, merged into the wire payload by
 * `render_canvas_scene` so the ink-canvas host still gets a `camera` key. */
export interface NoteCompositeCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🖼️ The Composite window's typed view-model — the TS mirror of the Rust `render()` boundary. Unlike
 * the read-only navigator, this view-model carries `engagementInput` (the live block-rename buffer
 * `WindowEngagement.input.value` reflects) because the composite window's own engagement is editable. */
export interface NoteCompositeViewModel {
  windowKindId: "note-composite";
  bodyKey: "note.play.composite";
  surfaceId: "note.play.composite";
  document: NoteCompositeSnapshot;
  camera: NoteCompositeCamera;
  activeUtilityId: string;
  engagementInput: string;
  /** @see InkCanvasScene.interactive — `view_mode == "composite"` evaluates to `true` here. */
  interactive: true;
}

export const NOTE_PLAY_WINDOW_COMPOSITE = "note-composite" as const;
export const NOTE_PLAY_BODY_COMPOSITE = "note.play.composite" as const;
export const NOTE_PLAY_SURFACE_COMPOSITE = "note.play.composite" as const;
