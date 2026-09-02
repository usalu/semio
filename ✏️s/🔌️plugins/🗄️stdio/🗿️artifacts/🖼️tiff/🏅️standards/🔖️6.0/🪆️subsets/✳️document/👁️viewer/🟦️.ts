/** 👁️ `tiff` viewer (any) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const TIFF_ANY_VIEWER_DIALECT = { artifactKind: "s.stdio.tiff", standard: "6.0", subset: "*" } as const;

export const TIFF_ANY_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
