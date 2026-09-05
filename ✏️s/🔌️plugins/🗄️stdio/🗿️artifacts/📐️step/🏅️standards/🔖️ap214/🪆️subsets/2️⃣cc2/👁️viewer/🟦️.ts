/** 👁️ Step CC2 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const STEP_CC2_VIEWER_DIALECT = { artifactKind: "s.stdio.step", standard: "ap214", subset: "cc2" } as const;

export const STEP_CC2_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
