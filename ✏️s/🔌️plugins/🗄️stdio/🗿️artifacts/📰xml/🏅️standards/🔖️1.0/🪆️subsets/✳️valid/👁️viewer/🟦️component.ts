/** 👁️ Xml viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`. */

export const XML_VALID_VIEWER_DIALECT = { artifactKind: "s.stdio.xml", standard: "1.0", subset: "valid" } as const;

export const XML_VALID_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
