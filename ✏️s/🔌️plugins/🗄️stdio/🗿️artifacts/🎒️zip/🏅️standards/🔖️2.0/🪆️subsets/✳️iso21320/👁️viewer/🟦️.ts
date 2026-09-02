/** 👁️ Zip viewer (2.0/✳️iso21320) — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: no mutation-shaped exports, no command payload types. */

export const ZIP_ISO21320_VIEWER_DIALECT = { artifactKind: "s.stdio.zip", standard: "2.0", subset: "iso21320" } as const;

export const ZIP_ISO21320_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
