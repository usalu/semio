/** 👁️ Xlsx viewer (ecma-376/✳️any) — subset-level typed twin. Read-only counterpart of the sibling
 * mutation-capable surface's own typed twin: mirrors the viewer manifest's mode/window vocabulary,
 * no mutation-shaped exports. */

export const XLSX_VIEWER_DIALECT = { artifactKind: "s.stdio.xlsx", standard: "ecma-376", subset: "*" } as const;

export const XLSX_VIEW_MODE_ID = "view" as const;

export * as mainWindow from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
