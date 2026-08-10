/** 🧬️ Imperative artifact schema — every field with its state class. */

export interface ImperativeArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  path: ImperativePath;
  /** @state persistent */
  seed: Record<string, unknown>;
  /** @state shared-ui */
  selectedStepIds: string[];
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  contributionsJson: string;
  /** @state effect */
  runOutputJson: string;
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
