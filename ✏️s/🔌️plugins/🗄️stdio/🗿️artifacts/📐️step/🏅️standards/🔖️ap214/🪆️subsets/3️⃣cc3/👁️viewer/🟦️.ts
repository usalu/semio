/** 👁️ Step CC3 viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️.ts`: mirrors the viewer manifest's mode/window vocabulary, no
 * mutation-shaped exports. */

export const STEP_CC3_VIEWER_DIALECT = { artifactKind: "s.stdio.step", standard: "ap214", subset: "cc3" } as const;

export const STEP_CC3_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
