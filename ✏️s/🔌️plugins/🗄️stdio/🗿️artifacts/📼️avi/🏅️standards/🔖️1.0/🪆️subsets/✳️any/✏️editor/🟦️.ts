/** ✏️ `avi` editor (any) — subset-level typed twin. Mirrors the editor
 * manifest's mode/window vocabulary; no mutation-payload types beyond the window twin re-exported
 * below (this surface's whole command set is the single frozen action its window kit declares). */

export const AVI_EDITOR_DIALECT = { artifactKind: "s.stdio.avi", standard: "1.0", subset: "*" } as const;

export const AVI_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
