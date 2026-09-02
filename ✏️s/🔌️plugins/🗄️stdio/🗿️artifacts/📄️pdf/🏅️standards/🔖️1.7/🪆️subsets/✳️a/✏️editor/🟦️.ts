/** ✏️ PDF/A Document (1.7) editor -- subset-level typed twin. Re-exports the `main` window's typed
 * view-model binding, mirroring `component.rs`'s `create_pdf17_a_editor()` stitching. */

export const PDF17A_EDITOR_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.7", subset: "a" } as const;

export const PDF17A_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
