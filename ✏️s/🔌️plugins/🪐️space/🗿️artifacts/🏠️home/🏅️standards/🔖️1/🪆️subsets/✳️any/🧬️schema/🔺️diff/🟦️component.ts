/** 🧬️ S Home diff schema — sparse field delta over the artifact. */

export interface SHomeDiff {
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  catalogGeneration?: number;
  /** @state config */
  activePanelTab?: string;
  /** @state config */
  locale?: string;
}
