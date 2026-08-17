/** 🧭️ Note editor — Navigator window: typed twin of `🦀️component.rs`'s `render()` boundary (a
 * non-interactive scaled overview of the SAME document/camera the Composite window renders, via
 * `composite::render_canvas_scene` with `viewMode: "navigator"`). */

/** 📝 One block on the canvas — mirrors `NoteBlockNode`'s tagged-union shape (same shape the
 * Composite window's own mirror carries; independently declared here, not imported, since every
 * window twin in this codebase is a standalone mirror — see `🧬️schema/📸️snapshot/🟦️component.ts`
 * for the precedent). */
export interface NoteNavigatorBlockNode {
  kind: string;
  id: string;
  [key: string]: unknown;
}

export interface NoteNavigatorImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}

export interface NoteNavigatorSnapshot {
  schema: string;
  id: string;
  title?: string;
  blocks: NoteNavigatorBlockNode[];
  gridVisible?: boolean;
  gridSpacing?: number;
  gridSubdivisions?: number;
  gridOpacity?: number;
  snapEnabled?: boolean;
  snapGridSpacing?: number;
  pencilWidth?: number;
  eraserRadius?: number;
  assets: Record<string, NoteNavigatorImageAsset>;
}

export interface NoteNavigatorCamera {
  x: number;
  y: number;
  zoom: number;
}

/** 🧭️ The Navigator window's typed view-model. No `engagementInput` field (unlike the composite
 * window's own view-model): `engagement()`'s `WindowEngagementInput.value` is always `None` here —
 * the navigator's one engagement action is `selectAll`, not a text buffer — and no per-block
 * `activeUtilityId`-scoped drawing utilities apply to a non-interactive overview. */
export interface NoteNavigatorViewModel {
  windowKindId: "note-navigator";
  bodyKey: "note.play.navigator";
  surfaceId: "note.play.navigator";
  document: NoteNavigatorSnapshot;
  camera: NoteNavigatorCamera;
  activeUtilityId: string;
  /** @see InkCanvasScene.interactive — `view_mode == "composite"` evaluates to `false` here. */
  interactive: false;
}

export const NOTE_PLAY_WINDOW_NAVIGATOR = "note-navigator" as const;
export const NOTE_PLAY_BODY_NAVIGATOR = "note.play.navigator" as const;
export const NOTE_PLAY_SURFACE_NAVIGATOR = "note.play.navigator" as const;
