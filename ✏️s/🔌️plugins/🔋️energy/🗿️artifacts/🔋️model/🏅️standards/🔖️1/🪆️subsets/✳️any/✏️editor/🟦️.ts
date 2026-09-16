/** ✏️ Energy model editor — subset-level typed twin. Re-exports every window's typed view-model
 * bindings so a host-side TS consumer has one import surface for the whole editor manifest,
 * mirroring `🦀️.rs`'s `create_energy_model_editor()` stitching every window/mode module
 * together. Namespaced (not `export *`): authored windows can independently export a same-named
 * `EnergyModelZonesViewModel`/`EnergyModelStructureViewModel`-shaped module surface, and a blanket
 * `export *` from more than one would risk an ambiguous re-export as this surface grows. */

export const ENERGY_MODEL_EDITOR_DIALECT = { artifactKind: "s.energy.model", standard: "1", subset: "*" } as const;

export const ENERGY_MODEL_EDIT_MODE_ID = "edit" as const;

export const ENERGY_SIMULATION_RUN_SCHEMA = "energy.simulation.run.v1" as const;

/** ⏯️ The energy simulation tool whose read-only framework `ToolRun` the reserved `toolRun*` actions drive. */
export const ENERGY_SIMULATION_TOOL_ID = "energySimulation" as const;

/** 🎚️ The editor config record (`🎚️config/🧬️schema/🔣️.json`): the run settings a simulation run uses. */
export interface EnergyModelConfig {
  readonly zoneTimestepMinutes: number;
  readonly systemTimestepMinutes: number;
  readonly warmupDays: number;
}

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
  "set-simulation-settings",
  "set-surface-property",
  "set-fenestration-property",
  "set-zone-property",
  "set-glazing-material-property",
  "set-gas-material-property",
] as const;

/** 📬️ The verbs that publish a semantic mutation into the document store. `setActiveExample`
 * is NOT one of them — a whole-document swap has no mutation representative in this artifact's
 * vocabulary, so it emits a `LoadDocument` effect (the host's `ArtifactStore::reset` route, outside
 * undo history) and declares the `HostOnly` publication lane; `set-simulation-settings` publishes to the config lane. */
export const ENERGY_MODEL_DOCUMENT_TOOL_IDS = ENERGY_MODEL_RETAINED_TOOL_IDS.filter(
  (id) => id !== "setActiveExample" && id !== "set-simulation-settings",
);

/** 📌️ The two dock panels the editor contributes: the artifact tree and the inspector. Their body
 * keys are what `ArtifactEditor::render_with_request_context` routes on. */
export const ENERGY_MODEL_ARTIFACT_PANEL_BODY_KEY = "energy.model.artifact" as const;
export const ENERGY_MODEL_INSPECTION_PANEL_BODY_KEY = "energy.model.inspection" as const;

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

export * as structureWindow from "./🎭️modes/✏️edit/🪟️windows/🌳️structure/🟦️";
export * as zonesWindow from "./🎭️modes/✏️edit/🪟️windows/📊️zones/🟦️";
export * as simulationWindow from "./🎭️modes/✏️edit/🪟️windows/⚡️simulation/🟦️";
