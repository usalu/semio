/** ✏️ `html` editor (any) — subset-level typed twin. Mirrors the editor
 * manifest's mode/window vocabulary; no mutation-payload types beyond the window twin re-exported
 * below (this surface's whole command set is the single frozen action its window kit declares). */

export const HTML_EDITOR_DIALECT = { artifactKind: "s.stdio.html", standard: "5", subset: "*" } as const;

export const HTML_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
