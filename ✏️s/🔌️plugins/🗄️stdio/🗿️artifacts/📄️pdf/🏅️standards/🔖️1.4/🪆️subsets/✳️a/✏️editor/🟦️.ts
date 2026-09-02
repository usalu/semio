/** ✏️ PDF/A Document (1.4) editor -- subset-level typed twin. Re-exports the `main` window's typed
 * view-model binding, mirroring `component.rs`'s `create_pdf14_a_editor()` stitching. */

export const PDF14A_EDITOR_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.4", subset: "a" } as const;

export const PDF14A_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
