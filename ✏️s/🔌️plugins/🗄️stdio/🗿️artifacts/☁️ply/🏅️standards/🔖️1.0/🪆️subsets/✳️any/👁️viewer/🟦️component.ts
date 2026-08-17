/** 👁️ PLY viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const PLY_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.ply", standard: "1.0", subset: "*" } as const;

export const PLY_ANY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
