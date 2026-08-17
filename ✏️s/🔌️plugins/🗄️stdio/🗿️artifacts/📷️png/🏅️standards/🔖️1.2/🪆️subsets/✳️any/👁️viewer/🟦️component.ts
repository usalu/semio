/** 👁️ `png` viewer (any) — read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const PNG_VIEWER_DIALECT = { artifactKind: "s.stdio.png", standard: "1.2", subset: "*" } as const;

export const PNG_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
