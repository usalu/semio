/** 🧬️ S Home diff schema — sparse field delta over the artifact. */

export interface SHomeDiff {
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  catalogGeneration?: number;
  /** @state local-ui */
  activePanelTab?: string;
  /** @state local-ui */
  locale?: string;
}
