/** ✏️ Tsv editor — subset-level typed twin. Re-exports the window's typed view-model binding. */

export const TSV_EDITOR_DIALECT = { artifactKind: "s.stdio.tsv", standard: "iana", subset: "*" } as const;

export const TSV_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
