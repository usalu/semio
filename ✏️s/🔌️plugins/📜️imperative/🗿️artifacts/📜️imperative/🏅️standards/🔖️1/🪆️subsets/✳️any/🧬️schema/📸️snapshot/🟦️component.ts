/** 🧬️ Imperative snapshot schema — persistent fields only. */

export interface ImperativeSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  path: ImperativePath;
  /** @state persistent */
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
