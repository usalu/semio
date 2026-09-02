/** 👁️ `mp4` viewer (any) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const MP4_VIEWER_DIALECT = { artifactKind: "s.stdio.mp4", standard: "isobmff", subset: "*" } as const;

export const MP4_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️";
