/** ✏️ Puzzle 3d editor — subset-level typed twin. Mirrors the editor manifest's mode/window
 * vocabulary; the mutation-shaped exports (per-command payload types) live in their own
 * `🎮️commands/<group>` leaves, not re-exported here to avoid a name explosion at this root. */

export const PUZZLE3D_EDITOR_DIALECT = { artifactKind: "s.puzzle.puzzle3d", standard: "1", subset: "*" } as const;

export const PUZZLE3D_EDIT_MODE_ID = "edit" as const;

export * from "./🎭️modes/✏️edit/🪟️windows/🧊️main/🟦️";
