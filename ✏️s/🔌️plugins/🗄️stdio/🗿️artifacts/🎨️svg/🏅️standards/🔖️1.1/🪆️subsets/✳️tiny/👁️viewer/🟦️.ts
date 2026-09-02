/** 👁️ `svg` viewer (tiny) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const SVG_TINY_VIEWER_DIALECT = { artifactKind: "s.stdio.svg", standard: "1.1", subset: "tiny" } as const;

export const SVG_TINY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
