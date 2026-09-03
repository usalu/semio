/** 🧬️ Imperative snapshot schema — artifact-lane fields only. */

export interface ProcedureSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  path: ProcedurePath;
  /** @state artifact */
  seed: Record<string, unknown>;
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
