/** 👁️ `avi` viewer (any) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const AVI_VIEWER_DIALECT = { artifactKind: "s.stdio.avi", standard: "1.0", subset: "*" } as const;

export const AVI_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
