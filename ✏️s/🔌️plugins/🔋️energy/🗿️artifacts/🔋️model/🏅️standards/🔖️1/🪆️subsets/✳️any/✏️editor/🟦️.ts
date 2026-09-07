/** ✏️ Energy model editor — subset-level typed twin. Re-exports every window's typed view-model
 * bindings so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️.rs`'s `create_energy_model_editor()` stitching every window/mode module
 * together. Namespaced (not `export *`): authored windows can independently export a same-named
 * `EnergyModelZonesViewModel`/`EnergyModelStructureViewModel`-shaped module surface, and a blanket
 * `export *` from more than one would risk an ambiguous re-export as this surface grows. */

export const ENERGY_MODEL_EDITOR_DIALECT = { artifactKind: "s.energy.model", standard: "1", subset: "*" } as const;

export const ENERGY_MODEL_EDIT_MODE_ID = "edit" as const;

export const ENERGY_SIMULATION_EVENT_SCHEMA = "semio.energy.simulation-event.v1" as const;

export type EnergySimulationLocale = "en" | "de";

export type EnergySimulationEvent =
  | { kind: "start"; request: bigint }
  | { kind: "configure"; locale: EnergySimulationLocale; zoneTimestepMinutes: number; systemTimestepMinutes: number; warmupDays: number }
  | { kind: "cancel" | "retry" | "discard" | "adopt"; request: bigint; operation: bigint; generation: bigint; configDigest: bigint };

/** 🧵️ Every retained tool id of this editor, in `ENERGY_MODEL_RETAINED_TOOL_IDS` order. The Rust
 * surface asserts set equality between this roster, the typed command schema's `TOOL_JOB_IDS`, the
 * `Migrated` classification list, the factory's publication contracts and its proof rows. */
export const ENERGY_MODEL_RETAINED_TOOL_IDS = [
  "set-node",
  "set-cell",
  "create-zone",
  "rename-zone",
  "delete-zone",
  "create-surface",
  "delete-surface",
  "assign-surface-construction",
  "set-material-property",
  "set-thermostat-setpoints",
  "set-site",
  "set-run-period",
  "setActiveExample",
  "start-energy-simulation",
  "cancel-energy-simulation",
  "retry-energy-simulation",
  "discard-energy-simulation",
  "adopt-energy-simulation",
  "configure-energy-simulation",
] as const;

/** 📬️ The twelve verbs that publish a semantic mutation into the document store. `setActiveExample`
 * is NOT one of them — a whole-document swap has no mutation representative in this artifact's
 * vocabulary, so it emits a `LoadDocument` effect (the host's `ArtifactStore::reset` route, outside
 * undo history) and declares the `HostOnly` publication lane like the six session verbs. */
export const ENERGY_MODEL_DOCUMENT_TOOL_IDS = ENERGY_MODEL_RETAINED_TOOL_IDS.slice(0, 12);

/** 📚️ The bundled examples the plugin root registers through `editor_with_examples`. */
export const ENERGY_MODEL_EXAMPLE_IDS = [
  "demo",
  "bestest-600",
  "bestest-600ff",
  "bestest-610",
  "bestest-620",
  "bestest-630",
  "bestest-640",
  "bestest-650",
  "bestest-900",
  "bestest-900ff",
  "bestest-910",
  "bestest-920",
  "bestest-930",
  "bestest-940",
  "bestest-950",
] as const;

export interface EnergySimulationTierProjection {
  readonly operation: bigint;
  readonly generation: bigint;
  readonly configDigest: bigint;
  readonly sequence: bigint;
  readonly tier: "steadyStateEstimate" | "designDay" | "coarseTimestep" | "final";
  readonly stage: string;
  readonly timestep: number;
  readonly totalTimesteps: number;
  readonly facilityElectricityKwh: number;
}

export * as structureWindow from "./🎭️modes/✏️edit/🪟️windows/🌳️structure/🟦️";
export * as zonesWindow from "./🎭️modes/✏️edit/🪟️windows/📊️zones/🟦️";
export * as simulationWindow from "./🎭️modes/✏️edit/🪟️windows/⚡️simulation/🟦️";
