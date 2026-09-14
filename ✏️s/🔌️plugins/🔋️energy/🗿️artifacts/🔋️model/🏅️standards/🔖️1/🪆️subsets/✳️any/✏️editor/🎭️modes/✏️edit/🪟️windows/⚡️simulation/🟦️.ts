/** ⚡️ Schema-first Energy simulation window action and accessibility contract — the typed twin of
 * `🦀️.rs`'s `definition()`. Every id here is also a retained tool id of the editor surface; the run
 * itself is driven by the framework-reserved `toolRun*` actions and chords, never by a window verb. */
export const ENERGY_SIMULATION_WINDOW_KIND_ID = "energy.simulation" as const;
export const ENERGY_SIMULATION_ACTIONS = [
  { id: "set-simulation-settings", label: { en: "Set simulation settings", de: "Simulationseinstellungen setzen" } },
  { id: "set-run-period", label: { en: "Set run period", de: "Simulationszeitraum setzen" } },
] as const;

export const ENERGY_SIMULATION_ACCESSIBILITY = { role: "status", ariaLive: "polite", busyIsTextual: true, colorOnly: false } as const;
