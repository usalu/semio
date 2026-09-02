/** ✏️ Docx editor — subset-level typed twin. Re-exports the single window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️.rs`'s `create_docx_editor()` stitching the mode/window module together. */

export const DOCX_EDITOR_DIALECT = { artifactKind: "s.stdio.docx", standard: "ecma-376", subset: "*" } as const;

export const DOCX_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
