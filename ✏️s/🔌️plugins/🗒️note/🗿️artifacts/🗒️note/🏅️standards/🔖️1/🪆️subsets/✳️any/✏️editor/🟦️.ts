/** ✏️ Note editor — subset-level typed twin. Re-exports every window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_note_app()` stitching every window/mode module together. */

export const NOTE_EDITOR_DIALECT = { artifactKind: "s.note.note", standard: "1", subset: "*" } as const;

export const NOTE_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): keeps each window's own constants/interfaces addressable without
// name collisions — both windows independently declare a same-named `ViewModel`-shaped export.
export * as compositeWindow from "./🎭️modes/✏️edit/🪟️windows/🖼️composite/🟦️component";
export * as navigatorWindow from "./🎭️modes/✏️edit/🪟️windows/🧭️navigator/🟦️component";
