/** ✏️ Energy model editor — subset-level typed twin. Re-exports both windows' typed view-model
 * bindings so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️component.rs`'s `create_energy_model_editor()` stitching every window/mode module
 * together. Namespaced (not `export *`): both windows independently export a same-named
 * `EnergyModelZonesViewModel`/`EnergyModelStructureViewModel`-shaped module surface, and a blanket
 * `export *` from more than one would risk an ambiguous re-export as this surface grows. */

export const ENERGY_MODEL_EDITOR_DIALECT = { artifactKind: "s.energy.model", standard: "1", subset: "*" } as const;

export const ENERGY_MODEL_EDIT_MODE_ID = "edit" as const;

export * as structureWindow from "./🎭️modes/✏️edit/🪟️windows/🌳️structure/🟦️component";
export * as zonesWindow from "./🎭️modes/✏️edit/🪟️windows/📊️zones/🟦️component";
