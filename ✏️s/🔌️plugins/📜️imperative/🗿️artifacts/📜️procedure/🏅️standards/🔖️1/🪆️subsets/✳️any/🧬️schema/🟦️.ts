/** 🧬️ Imperative artifact schema — every field with its state class. */

export interface ProcedureArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  path: ProcedurePath;
  /** @state artifact */
  seed: Record<string, unknown>;
  /** @state presence */
  selectedStepIds: string[];
  /** @state config */
  locale: string;
  /** @state config */
  contributionsJson: string;
  /** @state transient */
  runOutputJson: string;
}

export interface ProcedurePath {
  steps: ProcedureStep[];
}

export interface ProcedureStep {
  id: string;
  kind: string;
  params?: Record<string, unknown>;
  bodies?: Record<string, ProcedurePath>;
}
