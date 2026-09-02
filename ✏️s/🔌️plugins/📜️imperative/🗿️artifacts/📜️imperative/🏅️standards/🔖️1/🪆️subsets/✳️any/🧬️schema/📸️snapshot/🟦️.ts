/** 🧬️ Imperative snapshot schema — artifact-lane fields only. */

export interface ImperativeSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  path: ImperativePath;
  /** @state artifact */
  seed: Record<string, unknown>;
}

export interface ImperativePath {
  steps: ImperativeStep[];
}

export interface ImperativeStep {
  id: string;
  kind: string;
  params?: Record<string, unknown>;
  bodies?: Record<string, ImperativePath>;
}
