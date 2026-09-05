/** 👁️ Docx viewer — subset-level typed twin. Read-only counterpart re-exporting the single
 * window's typed view-model binding, no mutation-shaped exports (no command payload types, no
 * config schema beyond the framework's own empty config). */

export const DOCX_VIEWER_DIALECT = { artifactKind: "s.stdio.docx", standard: "ecma-376", subset: "*" } as const;

export const DOCX_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
