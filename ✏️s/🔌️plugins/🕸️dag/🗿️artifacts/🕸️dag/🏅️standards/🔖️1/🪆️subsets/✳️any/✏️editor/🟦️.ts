/** ✏️ DAG editor — subset-level typed twin. Re-exports every window's typed view-model binding so a
 * host-side TS consumer has one import surface for the whole editor manifest, mirroring
 * `🦀️.rs`'s `create_dag_app()` stitching every window/mode module together. */

export const DAG_EDITOR_DIALECT = { artifactKind: "s.dag.dag", standard: "1", subset: "*" } as const;

export const DAG_PLAY_MODE_EDIT = "edit" as const;

// 🪟️ Namespaced (not `export *`): keeps each window's own constants/interfaces addressable without
// name collisions as more windows gain same-named exports over time.
export * as mainWindow from "./🎭️modes/✏️edit/🪟️windows/🕸️main/🟦️";
export * as compiledWindow from "./🎭️modes/✏️edit/🪟️windows/🧬️compiled/🟦️";
