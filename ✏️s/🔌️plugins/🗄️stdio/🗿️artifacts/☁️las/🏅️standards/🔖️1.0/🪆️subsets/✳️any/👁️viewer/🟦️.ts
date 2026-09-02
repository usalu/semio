/** 👁️ LAS viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const LAS_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.las", standard: "1.0", subset: "*" } as const;

export const LAS_ANY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
