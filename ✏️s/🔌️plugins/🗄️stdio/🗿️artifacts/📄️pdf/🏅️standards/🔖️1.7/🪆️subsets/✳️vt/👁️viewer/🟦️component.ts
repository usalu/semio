/** 👁️ PDF/VT Document (1.7) viewer -- subset-level typed twin. Read-only counterpart of the
 * mutation-capable surface's own typed twin: mirrors the viewer manifest's mode/window vocabulary,
 * no mutation-shaped exports. */

export const PDF17VT_VIEWER_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.7", subset: "vt" } as const;

export const PDF17VT_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
