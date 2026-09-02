/** ✏️ Xml editor — subset-level typed twin. Re-exports the window's typed view-model binding. */

export const XML_VALID_EDITOR_DIALECT = { artifactKind: "s.stdio.xml", standard: "1.0", subset: "valid" } as const;

export const XML_VALID_EDIT_MODE_ID = "edit" as const;

export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
