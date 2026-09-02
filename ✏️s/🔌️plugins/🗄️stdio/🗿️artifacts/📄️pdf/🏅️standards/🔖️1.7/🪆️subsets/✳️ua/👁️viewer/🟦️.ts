/** 👁️ PDF/UA Document (1.7) viewer -- subset-level typed twin. Read-only counterpart of the
 * mutation-capable surface's own typed twin: mirrors the viewer manifest's mode/window vocabulary,
 * no mutation-shaped exports. */

export const PDF17UA_VIEWER_DIALECT = { artifactKind: "s.stdio.pdf", standard: "1.7", subset: "ua" } as const;

export const PDF17UA_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
