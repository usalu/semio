/** 👁️ Step CC5 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const STEP_CC5_VIEWER_DIALECT = { artifactKind: "s.stdio.step", standard: "ap214", subset: "cc5" } as const;

export const STEP_CC5_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
