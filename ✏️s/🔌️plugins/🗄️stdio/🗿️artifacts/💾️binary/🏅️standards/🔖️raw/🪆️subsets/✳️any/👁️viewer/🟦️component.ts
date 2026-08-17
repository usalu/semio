/** 👁️ Binary viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`:
 * no mutation-shaped exports, no command payload types. */

export const BINARY_VIEWER_DIALECT = { artifactKind: "s.stdio.binary", standard: "raw", subset: "*" } as const;

export const BINARY_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
