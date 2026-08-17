/** ✏️ DWG AC1024 editor — subset-level typed twin. Mirrors the editor manifest's mode/window
 * vocabulary; no mutation payload types beyond the shared window kit's own (this subset uses the
 * minimal command pattern — see `🦀️component.rs`'s own doc comment for why). */

export const DWG_AC1024_EDITOR_DIALECT = { artifactKind: "s.stdio.dwg", standard: "ac1024", subset: "*" } as const;

export const DWG_AC1024_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
