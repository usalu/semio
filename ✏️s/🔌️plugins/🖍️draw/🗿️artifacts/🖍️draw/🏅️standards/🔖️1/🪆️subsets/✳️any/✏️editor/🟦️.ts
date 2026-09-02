/** ✏️ Draw editor — subset-level typed twin. Re-exports the window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_draw_app()` stitching the window module together. */

export const DRAW_EDITOR_DIALECT = { artifactKind: "s.draw.draw", standard: "1", subset: "*" } as const;

export const DRAW_PLAY_MODE_EDIT = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🖼️canvas/🟦️";
