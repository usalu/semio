/** 🧬️ Imperative diff schema — sparse field delta over the artifact. */

export interface ProcedureDiff {
  /** @state artifact */ artifact?: ProcedureArtifact;
  /** @state artifact */ schema?: string;
  /** @state artifact */ path?: ProcedurePathDelta;
  /** @state artifact */ seed?: Record<string, unknown>;
  /** @state presence */ selectedStepIds?: ProcedureStringList;
  /** @state config */ locale?: string;
  /** @state config */ contributionsJson?: string;
}

export interface ProcedureStringList {
  values: string[];
}

export interface ProcedurePathRef {
  owner?: string;
  slot?: string;
}

export interface ProcedurePathDelta {
  pathRef: ProcedurePathRef;
  steps: ProcedureStepsDelta;
}

export interface ProcedureStepsDelta {
  added: ProcedureStep[];
  removed: string[];
  patched: ProcedureStepPatchEntry[];
  reordered?: string[];
}

export interface ProcedureStepPatchEntry {
  id: string;
  patch: Record<string, unknown>;
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

export interface ProcedureArtifact {
  schema: string;
  path: ProcedurePath;
  seed: Record<string, unknown>;
  selectedStepIds: string[];
  locale: string;
  contributionsJson: string;
  runOutputJson: string;
}
