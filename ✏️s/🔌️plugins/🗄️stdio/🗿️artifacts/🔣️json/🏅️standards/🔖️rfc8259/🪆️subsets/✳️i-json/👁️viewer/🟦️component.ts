/** 👁️ Json viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`. */

export const JSON_I_JSON_VIEWER_DIALECT = { artifactKind: "s.stdio.json", standard: "rfc8259", subset: "i-json" } as const;

export const JSON_I_JSON_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
