/** 👁️ Energy model viewer — subset-level typed twin. Read-only counterpart of
 * `✏️editor/🟦️component.ts`: mirrors the viewer manifest's mode/window vocabulary, no mutation-shaped
 * exports (no command payload types, no config schema beyond the framework's own empty config). */

export const ENERGY_MODEL_VIEWER_DIALECT = { artifactKind: "s.energy.model", standard: "1", subset: "*" } as const;

export const ENERGY_MODEL_VIEW_MODE_ID = "view" as const;
export const ENERGY_SIMULATION_VIEWER_WINDOW_KIND_ID = "energy.simulation.viewer" as const;

export * as structureWindow from "./🎭️modes/👁️view/🪟️windows/🌳️structure/🟦️component";
export * as zonesWindow from "./🎭️modes/👁️view/🪟️windows/📊️zones/🟦️component";
export * as simulationWindow from "./🎭️modes/👁️view/🪟️windows/⚡️simulation/🟦️component";
