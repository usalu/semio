/** ✏️ Block 5D editor — subset-level typed twin. Re-exports every window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️component.rs`'s `create_block5d_app()` stitching every window/mode module together. */

export const BLOCK5D_EDITOR_DIALECT = { artifactKind: "s.block.block5d", standard: "1", subset: "*" } as const;

export const BLOCK5D_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): both windows are free to grow same-named exports later, and this
// mirrors the cad pilot's precedent for a multi-window editor surface root.
export * as boardWindow from "./🎭️modes/✏️edit/🪟️windows/📋️board/🟦️component";
export * as worldWindow from "./🎭️modes/✏️edit/🪟️windows/🌐️world/🟦️component";
