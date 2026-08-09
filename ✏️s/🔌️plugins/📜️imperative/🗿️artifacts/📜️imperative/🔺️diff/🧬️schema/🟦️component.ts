/** 🧬️ Imperative diff schema — sparse field delta over the artifact. */

export interface ImperativeDiff {
  /** @state persistent */ artifact?: ImperativeArtifact;
  /** @state persistent */ schema?: string;
  /** @state persistent */ path?: ImperativePathDelta;
  /** @state persistent */ seed?: Record<string, unknown>;
  /** @state shared-ui */ selectedStepIds?: ImperativeStringList;
  /** @state local-ui */ locale?: string;
  /** @state local-ui */ contributionsJson?: string;
}

export interface ImperativeStringList {
  values: string[];
}

export interface ImperativePathRef {
  owner?: string;
  slot?: string;
}

export interface ImperativePathDelta {
  pathRef: ImperativePathRef;
  steps: ImperativeStepsDelta;
}

export interface ImperativeStepsDelta {
  added: ImperativeStep[];
  removed: string[];
  patched: ImperativeStepPatchEntry[];
  reordered?: string[];
}

export interface ImperativeStepPatchEntry {
  id: string;
  patch: Record<string, unknown>;
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

export interface ImperativeArtifact {
  schema: string;
  path: ImperativePath;
  seed: Record<string, unknown>;
  selectedStepIds: string[];
  locale: string;
  contributionsJson: string;
  runOutputJson: string;
}
