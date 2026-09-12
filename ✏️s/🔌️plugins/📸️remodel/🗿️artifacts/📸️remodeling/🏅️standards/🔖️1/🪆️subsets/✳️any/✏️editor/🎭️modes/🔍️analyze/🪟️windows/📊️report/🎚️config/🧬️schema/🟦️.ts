/** 🧬️ Exact Remodeling Report window configuration. */
export interface RemodelingReportWindowConfig {
  reportTable: string;
}

/** 🧬️ Atomic replacement of one exact Report window configuration. */
export type RemodelingReportWindowConfigMutation = { kind: "snapshot"; config: RemodelingReportWindowConfig };

/** 🔁️ Applies one Report window configuration mutation. */
export const applyRemodelingReportWindowConfigMutation = (_base: RemodelingReportWindowConfig, mutation: RemodelingReportWindowConfigMutation): RemodelingReportWindowConfig => structuredClone(mutation.config);
