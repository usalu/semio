/** ✏️ Txt editor — subset-level typed twin. Re-exports the window's typed view-model binding. */

export const TXT_EDITOR_DIALECT = { artifactKind: "s.stdio.txt", standard: "utf-8", subset: "*" } as const;

export const TXT_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
