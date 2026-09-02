/** 👁️ `wav` viewer (any) — read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports. */

export const WAV_VIEWER_DIALECT = { artifactKind: "s.stdio.wav", standard: "riff-pcm", subset: "*" } as const;

export const WAV_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🪟️main/🟦️component";
