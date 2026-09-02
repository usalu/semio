/** ✏️ `mp3` editor (any) — subset-level typed twin. Mirrors the editor
 * manifest's mode/window vocabulary; no mutation-payload types beyond the window twin re-exported
 * below (this surface's whole command set is the single frozen action its window kit declares). */

export const MP3_EDITOR_DIALECT = { artifactKind: "s.stdio.mp3", standard: "mpeg1-layer3", subset: "*" } as const;

export const MP3_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🪟️main/🟦️";
