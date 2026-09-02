/** ✏️ Block 3D editor — subset-level typed twin. Re-exports the world window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_block3d_app()` stitching the mode/window module together. */

export const BLOCK3D_EDITOR_DIALECT = { artifactKind: "s.block.block3d", standard: "1", subset: "*" } as const;

export const BLOCK3D_PLAY_MODE_EDIT = "edit" as const;

export * as worldWindow from "./🎭️modes/✏️edit/🪟️windows/🌐️world/🟦️component";
