/** 👁️ PDF/X Document (1.7) viewer -- subset-level typed twin. Read-only counterpart of the
 * mutation-capable surface's own typed twin: mirrors the viewer manifest's mode/window vocabulary,
 * no mutation-shaped exports. */

export const PDF17X_VIEWER_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.7", subset: "x" } as const;

export const PDF17X_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
