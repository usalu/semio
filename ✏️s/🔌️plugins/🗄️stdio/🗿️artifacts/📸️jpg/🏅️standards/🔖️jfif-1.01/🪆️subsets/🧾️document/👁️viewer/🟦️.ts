/** 👁️ `jpg` viewer (any) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const JPG_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.jpg", standard: "jfif-1.01", subset: "*" } as const;

export const JPG_ANY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
