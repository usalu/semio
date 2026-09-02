/** ✏️ Pptx strict editor — subset-level typed twin. Re-exports the single window's typed
 * view-model binding so a host-side TS consumer has one import surface for the whole editor
 * manifest, mirroring `🦀️.rs`'s `create_pptx_strict_editor()` stitching the mode/window
 * module together. */

export const PPTX_STRICT_EDITOR_DIALECT = { artifactKind: "s.stdio.pptx", standard: "ecma-376", subset: "strict" } as const;

export const PPTX_STRICT_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
