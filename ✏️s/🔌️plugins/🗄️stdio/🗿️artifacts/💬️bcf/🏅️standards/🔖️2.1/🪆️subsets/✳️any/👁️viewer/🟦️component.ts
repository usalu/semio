/** 👁️ BCF viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const BCF_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.bcf", standard: "2.1", subset: "*" } as const;

export const BCF_ANY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
