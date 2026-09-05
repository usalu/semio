/** 👁️ `svg` viewer (basic) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const SVG_BASIC_VIEWER_DIALECT = { artifactKind: "s.stdio.svg", standard: "1.1", subset: "basic" } as const;

export const SVG_BASIC_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
