/** 👁️ Json viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`. */

export const JSON_VIEWER_DIALECT = { artifactKind: "s.stdio.json", standard: "rfc8259", subset: "*" } as const;

export const JSON_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
