/** ✏️ PDF/X Document (1.4) editor -- subset-level typed twin. Re-exports the `main` window's typed
 * view-model binding, mirroring `component.rs`'s `create_pdf14_x_editor()` stitching. */

export const PDF14X_EDITOR_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.4", subset: "x" } as const;

export const PDF14X_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
