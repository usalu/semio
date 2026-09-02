/** ⚡️ Schema-first Energy simulation window action and accessibility contract. */
export const ENERGY_SIMULATION_WINDOW_KIND_ID = "energy.simulation" as const;
export const ENERGY_SIMULATION_ACTIONS = [
  { id: "start-energy-simulation", label: { en: "Start simulation", de: "Simulation starten" }, keyboard: true },
  { id: "cancel-energy-simulation", label: { en: "Cancel simulation", de: "Simulation abbrechen" }, keyboard: true },
  { id: "retry-energy-simulation", label: { en: "Retry simulation", de: "Simulation wiederholen" }, keyboard: true },
  { id: "discard-energy-simulation", label: { en: "Discard result", de: "Ergebnis verwerfen" }, keyboard: true },
  { id: "adopt-energy-simulation", label: { en: "Adopt final result", de: "Endergebnis übernehmen" }, keyboard: true },
] as const;
export const ENERGY_SIMULATION_ACCESSIBILITY = { role: "status", ariaLive: "polite", busyIsTextual: true, colorOnly: false } as const;
