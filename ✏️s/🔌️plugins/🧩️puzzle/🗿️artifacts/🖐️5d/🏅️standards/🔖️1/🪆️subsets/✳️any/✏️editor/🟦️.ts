/** ✏️ Puzzle 5D editor — subset-level typed twin. Re-exports every window's typed view-model binding
 * so a host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_puzzle5d_app()` stitching every window/mode module together. */

export const PUZZLE5D_EDITOR_DIALECT = { artifactKind: "s.puzzle.puzzle5d", standard: "1", subset: "*" } as const;

export const PUZZLE5D_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): each window is free to grow its own same-named view-model shape
// over time, and a blanket re-export from more than one would risk an ambiguous name collision.
export * as board2dWindow from "./🎭️modes/✏️edit/🪟️windows/◻️2d/🟦️";
export * as world3dWindow from "./🎭️modes/✏️edit/🪟️windows/🧊️3d/🟦️";
