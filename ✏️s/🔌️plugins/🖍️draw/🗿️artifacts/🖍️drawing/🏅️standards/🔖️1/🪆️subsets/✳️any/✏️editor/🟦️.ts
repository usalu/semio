/** ✏️ Drawing editor — subset-level typed twin. Re-exports the window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_drawing_app()` stitching the window module together. */

export const DRAWING_EDITOR_DIALECT = { artifactKind: "s.draw.drawing", standard: "1", subset: "*" } as const;

export const DRAWING_PLAY_MODE_EDIT = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🖼️canvas/🟦️";
