/** 👁️ Tsv viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`. */

export const TSV_VIEWER_DIALECT = { artifactKind: "s.stdio.tsv", standard: "iana", subset: "*" } as const;

export const TSV_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
