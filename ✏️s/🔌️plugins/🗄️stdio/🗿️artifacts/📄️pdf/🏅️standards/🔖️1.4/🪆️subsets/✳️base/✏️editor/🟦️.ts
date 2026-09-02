/** ✏️ PDF Document (1.4) editor -- subset-level typed twin. Re-exports the `main` window's typed
 * view-model binding, mirroring `component.rs`'s `create_pdf14_editor()` stitching. */

export const PDF14_EDITOR_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.4", subset: "*" } as const;

export const PDF14_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
