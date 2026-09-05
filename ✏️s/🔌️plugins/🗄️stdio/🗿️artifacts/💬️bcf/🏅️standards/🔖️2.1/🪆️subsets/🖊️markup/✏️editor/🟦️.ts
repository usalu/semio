/** ✏️ BCF editor — subset-level typed twin. Mirrors the editor manifest's mode/window
 * vocabulary; no mutation payload types beyond the shared window kit's own (this subset uses the
 * minimal command pattern — see `🦀️.rs`'s own doc comment for why). */

export const BCF_ANY_EDITOR_DIALECT = { artifactKind: "s.stdio.bcf", standard: "2.1", subset: "*" } as const;

export const BCF_ANY_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
