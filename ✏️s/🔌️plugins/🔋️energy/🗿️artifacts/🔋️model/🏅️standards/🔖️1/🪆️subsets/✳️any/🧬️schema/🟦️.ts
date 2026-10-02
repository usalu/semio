/** 🧬️ The complete typed Energy artifact with its independently recomputed preview. */
import type {EnergyModelSnapshot} from "./📸️snapshot/🟦️.ts";

export interface EnergyModelArtifact extends EnergyModelSnapshot {
  resultsJson: string;
}
