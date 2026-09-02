/** ✏️ Trinity Jack editor — subset-level typed twin. Re-exports every window's typed view-model
 * binding so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️.rs`'s `create_trinity_jack_app()` stitching every window/mode module
 * together. */

export const TRINITY_JACK_EDITOR_DIALECT = { artifactKind: "s.trinity.jack", standard: "1", subset: "*" } as const;

export const TRINITY_JACK_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): keeps each window's own constants/interfaces addressable without
// name collisions as more windows gain same-named exports over time.
export * as editorWindow from "./🎭️modes/✏️edit/🪟️windows/📝️editor/🟦️component";
export * as resultsWindow from "./🎭️modes/✏️edit/🪟️windows/📊️results/🟦️component";
export * as graphWindow from "./🎭️modes/✏️edit/🪟️windows/🌐️graph/🟦️component";
