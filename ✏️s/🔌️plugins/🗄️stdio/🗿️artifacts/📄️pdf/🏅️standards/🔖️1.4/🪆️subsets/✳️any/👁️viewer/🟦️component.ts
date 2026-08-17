/** 👁️ PDF Document (1.4) viewer -- subset-level typed twin. Read-only counterpart of the
 * mutation-capable surface's own typed twin: mirrors the viewer manifest's mode/window vocabulary,
 * no mutation-shaped exports. */

export const PDF14_VIEWER_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.4", subset: "*" } as const;

export const PDF14_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
