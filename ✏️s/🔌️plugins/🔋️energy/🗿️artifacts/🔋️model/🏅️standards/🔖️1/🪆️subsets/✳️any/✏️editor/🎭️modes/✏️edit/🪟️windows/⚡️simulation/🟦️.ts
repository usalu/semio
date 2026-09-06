/** ⚡️ Schema-first Energy simulation window action and accessibility contract — the typed twin of
 * `🦀️.rs`'s `definition()`. Every id here is also a retained tool id of the editor surface. */
export const ENERGY_SIMULATION_WINDOW_KIND_ID = "energy.simulation" as const;
export const ENERGY_SIMULATION_ACTIONS = [
  { id: "start-energy-simulation", label: { en: "Start simulation", de: "Simulation starten" }, keyboard: "mod+enter" },
  { id: "cancel-energy-simulation", label: { en: "Cancel simulation", de: "Simulation abbrechen" }, keyboard: "mod+period" },
  { id: "retry-energy-simulation", label: { en: "Retry simulation", de: "Simulation wiederholen" }, keyboard: null },
  { id: "discard-energy-simulation", label: { en: "Discard result", de: "Ergebnis verwerfen" }, keyboard: null },
  { id: "adopt-energy-simulation", label: { en: "Adopt final result", de: "Endergebnis übernehmen" }, keyboard: "mod+shift+enter" },
  { id: "configure-energy-simulation", label: { en: "Configure run", de: "Lauf konfigurieren" }, keyboard: null },
] as const;

/** 🎛️ The editable, ephemeral local-only run settings `configure-energy-simulation` carries. The run
 * period is NOT here — it is persisted model data, edited through `set-run-period`. */
export interface EnergySimulationRunSettings {
  locale: "en" | "de";
  zoneTimestepMinutes: number;
  systemTimestepMinutes: number;
  warmupDays: number;
}

export const ENERGY_SIMULATION_ACCESSIBILITY = { role: "status", ariaLive: "polite", busyIsTextual: true, colorOnly: false } as const;
