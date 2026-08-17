/** ✏️ Semio Document editor — subset-level typed twin. Mirrors the editor manifest's mode/window
 * vocabulary; no mutation payload types beyond the shared window kit's own (this subset uses the
 * minimal command pattern — see `🦀️component.rs`'s own doc comment for why). */

export const SEMIO_DOCUMENT_EDITOR_DIALECT = { artifactKind: "s.stdio.semio", standard: "v1", subset: "document" } as const;

export const SEMIO_DOCUMENT_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
