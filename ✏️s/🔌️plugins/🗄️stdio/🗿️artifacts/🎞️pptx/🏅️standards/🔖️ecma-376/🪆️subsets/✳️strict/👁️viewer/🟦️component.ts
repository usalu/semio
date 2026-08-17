/** 👁️ Pptx strict viewer — subset-level typed twin. Read-only counterpart re-exporting the
 * single window's typed view-model binding, no mutation-shaped exports (no command payload types,
 * no config schema beyond the framework's own empty config). */

export const PPTX_STRICT_VIEWER_DIALECT = { artifactKind: "s.stdio.pptx", standard: "ecma-376", subset: "strict" } as const;

export const PPTX_STRICT_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
