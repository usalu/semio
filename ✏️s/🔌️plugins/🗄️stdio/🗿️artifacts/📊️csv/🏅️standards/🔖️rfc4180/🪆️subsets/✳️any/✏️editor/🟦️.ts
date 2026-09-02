/** ✏️ Csv editor — subset-level typed twin. Re-exports the window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_csv_editor()` stitching the mode/window module together. */

export const CSV_EDITOR_DIALECT = { artifactKind: "s.stdio.csv", standard: "rfc4180", subset: "*" } as const;

export const CSV_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
