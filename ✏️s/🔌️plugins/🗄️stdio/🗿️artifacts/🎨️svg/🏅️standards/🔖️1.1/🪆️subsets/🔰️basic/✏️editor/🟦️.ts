/** ✏️ `svg` editor (basic) — subset-level typed twin. Mirrors the editor
 * manifest's mode/window vocabulary; no mutation-payload types beyond the window twin re-exported
 * below (this surface's whole command set is the single frozen action its window kit declares). */

export const SVG_BASIC_EDITOR_DIALECT = { artifactKind: "s.stdio.svg", standard: "1.1", subset: "basic" } as const;

export const SVG_BASIC_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
