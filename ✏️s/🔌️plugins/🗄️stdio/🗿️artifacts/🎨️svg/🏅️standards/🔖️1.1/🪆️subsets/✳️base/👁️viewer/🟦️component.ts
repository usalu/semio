/** 👁️ `svg` viewer (any) — read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const SVG_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.svg", standard: "1.1", subset: "*" } as const;

export const SVG_ANY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
