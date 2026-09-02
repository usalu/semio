/** ✏️ Trinity Rewrite editor — subset-level typed twin. Re-exports every window's typed
 * view-model binding so a host-side TS consumer has one import surface for the whole editor
 * manifest, mirroring `🦀️.rs`'s `create_rewrite_app()` stitching every window/mode module
 * together. */

export const TRINITY_REWRITE_EDITOR_DIALECT = { artifactKind: "s.trinity.rewrite", standard: "1", subset: "*" } as const;

export const TRINITY_REWRITE_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): keeps each window's own constants/interfaces addressable without
// name collisions as more windows gain same-named exports over time.
export * as parametersWindow from "./🎭️modes/✏️edit/🪟️windows/🎛️parameters/🟦️";
export * as beforeWindow from "./🎭️modes/✏️edit/🪟️windows/⬅️before/🟦️";
export * as lhsWindow from "./🎭️modes/✏️edit/🪟️windows/👈️lhs/🟦️";
export * as afterWindow from "./🎭️modes/✏️edit/🪟️windows/⏭️after/🟦️";
export * as rhsWindow from "./🎭️modes/✏️edit/🪟️windows/➡️rhs/🟦️";
export * as jackWindow from "./🎭️modes/✏️edit/🪟️windows/🔎️jack/🟦️";
