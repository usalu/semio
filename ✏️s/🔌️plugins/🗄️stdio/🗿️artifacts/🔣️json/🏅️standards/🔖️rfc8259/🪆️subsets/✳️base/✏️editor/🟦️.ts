/** ✏️ Json editor — subset-level typed twin. Re-exports the window's typed view-model binding. */

export const JSON_EDITOR_DIALECT = { artifactKind: "s.stdio.json", standard: "rfc8259", subset: "*" } as const;

export const JSON_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
