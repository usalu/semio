/** ✏️ IFC 4 editor — subset-level typed twin. Mirrors the editor manifest's mode/window
 * vocabulary; no mutation payload types beyond the shared window kit's own (this subset uses the
 * minimal command pattern — see `🦀️.rs`'s own doc comment for why). */

export const IFC4_ANY_EDITOR_DIALECT = { artifactKind: "s.stdio.ifc", standard: "4", subset: "*" } as const;

export const IFC4_ANY_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
