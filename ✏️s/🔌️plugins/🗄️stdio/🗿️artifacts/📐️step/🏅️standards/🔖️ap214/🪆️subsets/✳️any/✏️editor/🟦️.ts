/** ✏️ Step Any editor — subset-level typed twin. Mirrors the editor manifest's mode/window
 * vocabulary; no mutation payload types beyond the shared window kit's own (this subset uses the
 * minimal command pattern — see `🦀️.rs`'s own doc comment for why). */

export const STEP_ANY_EDITOR_DIALECT = { artifactKind: "s.stdio.step", standard: "ap214", subset: "*" } as const;

export const STEP_ANY_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
