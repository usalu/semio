/** ✏️ Xlsx editor (ecma-376/🧱️base) — subset-level typed twin. Re-exports the window's typed
 * view-model binding so a host-side TS consumer has one import surface for the whole editor
 * manifest, mirroring `🦀️.rs`'s `create_xlsx_editor()` stitching every window/mode module
 * together. */

export const XLSX_EDITOR_DIALECT = { artifactKind: "s.stdio.xlsx", standard: "ecma-376", subset: "*" } as const;

export const XLSX_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
