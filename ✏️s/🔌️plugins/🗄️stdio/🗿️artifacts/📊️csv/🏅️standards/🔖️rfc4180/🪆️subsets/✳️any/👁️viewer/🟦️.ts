/** 👁️ Csv viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const CSV_VIEWER_DIALECT = { artifactKind: "s.stdio.csv", standard: "rfc4180", subset: "*" } as const;

export const CSV_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
