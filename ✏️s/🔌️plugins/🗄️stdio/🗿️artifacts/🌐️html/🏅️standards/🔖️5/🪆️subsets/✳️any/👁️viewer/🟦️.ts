/** 👁️ `html` viewer (any) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const HTML_VIEWER_DIALECT = { artifactKind: "s.stdio.html", standard: "5", subset: "*" } as const;

export const HTML_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
