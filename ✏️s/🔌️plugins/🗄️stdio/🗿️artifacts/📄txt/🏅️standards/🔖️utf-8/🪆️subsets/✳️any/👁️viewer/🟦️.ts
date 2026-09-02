/** 👁️ Txt viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`. */

export const TXT_VIEWER_DIALECT = { artifactKind: "s.stdio.txt", standard: "utf-8", subset: "*" } as const;

export const TXT_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
