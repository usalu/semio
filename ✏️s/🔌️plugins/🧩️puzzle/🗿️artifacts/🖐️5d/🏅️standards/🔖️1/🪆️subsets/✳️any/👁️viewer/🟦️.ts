/** 👁️ Puzzle 5D viewer — subset-level typed twin. Read-only counterpart of `✏️editor/🟦️.ts`:
 * mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped exports (no command
 * payload types, no config schema beyond the framework's own empty config). */

export const PUZZLE5D_VIEWER_DIALECT = { artifactKind: "s.puzzle.puzzle5d", standard: "1", subset: "*" } as const;

export const PUZZLE5D_VIEW_MODE_ID = "view" as const;

export * from "./🎭️modes/👁️view/🪟️windows/🧊️3d/🟦️component";
