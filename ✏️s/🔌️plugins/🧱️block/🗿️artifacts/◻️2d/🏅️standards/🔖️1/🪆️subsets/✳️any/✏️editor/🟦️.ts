/** ✏️ Block 2D editor — subset-level typed twin. Re-exports the board window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️.rs`'s `create_block2d_app()` stitching the window/mode together. */

export const BLOCK2D_EDITOR_DIALECT = { artifactKind: "s.block.block2d", standard: "1", subset: "*" } as const;

export const BLOCK2D_PLAY_MODE_EDIT = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/📋️board/🟦️";
