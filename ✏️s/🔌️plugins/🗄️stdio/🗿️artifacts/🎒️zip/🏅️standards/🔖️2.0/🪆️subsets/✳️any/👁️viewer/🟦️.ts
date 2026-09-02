/** 👁️ Zip viewer (2.0/✳️any) — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: no mutation-shaped exports, no command payload types. */

export const ZIP_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.zip", standard: "2.0", subset: "*" } as const;

export const ZIP_ANY_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
