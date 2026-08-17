/** 👁️ `gif` viewer (any) — read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const GIF_87A_VIEWER_DIALECT = { artifactKind: "s.stdio.gif", standard: "87a", subset: "*" } as const;

export const GIF_87A_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
