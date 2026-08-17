/** ✏️ `mp4` editor (any) — subset-level typed twin. Mirrors the editor
 * manifest's mode/window vocabulary; no mutation-payload types beyond the window twin re-exported
 * below (this surface's whole command set is the single frozen action its window kit declares). */

export const MP4_EDITOR_DIALECT = { artifactKind: "s.stdio.mp4", standard: "isobmff", subset: "*" } as const;

export const MP4_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️component";
