/** 👁️ `tiff` viewer (baseline) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const TIFF_BASELINE_VIEWER_DIALECT = { artifactKind: "s.stdio.tiff", standard: "6.0", subset: "baseline" } as const;

export const TIFF_BASELINE_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
