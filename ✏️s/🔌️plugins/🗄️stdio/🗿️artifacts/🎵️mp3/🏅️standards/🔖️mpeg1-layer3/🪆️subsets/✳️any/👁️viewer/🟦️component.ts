/** 👁️ `mp3` viewer (any) — read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const MP3_VIEWER_DIALECT = { artifactKind: "s.stdio.mp3", standard: "mpeg1-layer3", subset: "*" } as const;

export const MP3_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
