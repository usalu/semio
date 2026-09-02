/** 👁️ Deflate viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`:
 * no mutation-shaped exports, no command payload types. */

export const DEFLATE_VIEWER_DIALECT = { artifactKind: "s.stdio.deflate", standard: "rfc1950", subset: "*" } as const;

export const DEFLATE_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
