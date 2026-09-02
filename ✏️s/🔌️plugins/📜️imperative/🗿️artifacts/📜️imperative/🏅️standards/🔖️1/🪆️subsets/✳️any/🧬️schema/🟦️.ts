/** 🧬️ Imperative artifact schema — every field with its state class. */

export interface ImperativeArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  path: ImperativePath;
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

export interface ImperativePath {
  steps: ImperativeStep[];
}

export interface ImperativeStep {
  id: string;
  kind: string;
  params?: Record<string, unknown>;
  bodies?: Record<string, ImperativePath>;
}
