/** 👁️ Read-only adopted Energy result window contract. */
export const ENERGY_SIMULATION_VIEWER_WINDOW_KIND_ID = "energy.simulation.viewer" as const;
export const ENERGY_SIMULATION_VIEWER_LABEL = { en: "Energy results", de: "Energieergebnisse" } as const;
export const ENERGY_SIMULATION_VIEWER_ACCESSIBILITY = { role: "status", ariaLive: "polite", readOnly: true } as const;
