/** 👁️ Note viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️component.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command
 * payload types, no config schema beyond the framework's own empty config). */

export const NOTE_VIEWER_DIALECT = { artifactKind: "s.note.note", standard: "1", subset: "*" } as const;

export const NOTE_VIEW_MODE_VIEW = "view" as const;

export * as compositeWindow from "./🎭️modes/👁️view/🪟️windows/🖼️composite/🟦️component";
