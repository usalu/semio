/** ✏️ Energy model editor — subset-level typed twin. Re-exports every window's typed view-model
 * bindings so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️component.rs`'s `create_energy_model_editor()` stitching every window/mode module
 * together. Namespaced (not `export *`): authored windows can independently export a same-named
 * `EnergyModelZonesViewModel`/`EnergyModelStructureViewModel`-shaped module surface, and a blanket
 * `export *` from more than one would risk an ambiguous re-export as this surface grows. */

export const ENERGY_MODEL_EDITOR_DIALECT = { artifactKind: "s.energy.model", standard: "1", subset: "*" } as const;

export const ENERGY_MODEL_EDIT_MODE_ID = "edit" as const;

export const ENERGY_SIMULATION_EVENT_SCHEMA = "semio.energy.simulation-event.v1" as const;

export type EnergySimulationLocale = "en" | "de";

export type EnergySimulationEvent =
  | { kind: "start"; request: bigint; locale: EnergySimulationLocale; checkpointToken: bigint; zoneTimestepMinutes: number; systemTimestepMinutes: number; warmupDays: number; runPeriodStartMonth: number; runPeriodStartDay: number; runPeriodEndMonth: number; runPeriodEndDay: number }
  | { kind: "cancel" | "retry" | "discard" | "adopt"; request: bigint; operation: bigint; generation: bigint; configDigest: bigint };

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

export * as structureWindow from "./🎭️modes/✏️edit/🪟️windows/🌳️structure/🟦️component";
export * as zonesWindow from "./🎭️modes/✏️edit/🪟️windows/📊️zones/🟦️component";
export * as simulationWindow from "./🎭️modes/✏️edit/🪟️windows/⚡️simulation/🟦️component";
