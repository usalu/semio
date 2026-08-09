/** 🧬️ Playbook diff schema — sparse field delta. */

export interface PlaybookDiff {
  /** @state persistent */
  artifact?: PlaybookArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  id?: string;
  /** @state persistent */
  version?: string;
  /** @state persistent */
  title?: string | null;
  /** @state persistent */
  steps?: PlaybookStepsDelta;
  /** @state shared-ui */
  selectedIds?: PlaybookStringList;
  /** @state local-ui */
  locale?: string;
  /** @state local-ui */
  contributionsJson?: string;
}

export interface PlaybookStringList {
  values: string[];
}

export interface PlaybookStepsDelta {
  added: PlaybookStep[];
  removed: string[];
  patched: PlaybookStepPatchEntry[];
  reordered?: string[];
}

export interface PlaybookBlocksDelta {
  added: PlaybookBlock[];
  removed: string[];
  patched: PlaybookBlockPatchEntry[];
  reordered?: string[];
}

export interface PlaybookStepPatchEntry {
  id: string;
  patch: PlaybookStepPatch;
}

export interface PlaybookBlockPatchEntry {
  id: string;
  patch: PlaybookBlockPatch;
}

export interface PlaybookStepPatch {
  title?: string;
  description?: string | null;
  blocks?: PlaybookBlocksDelta;
}

export interface PlaybookBlockPatch {
  block?: PlaybookBlock;
}

export interface PlaybookArtifact {
  schema: string;
  id: string;
  version: string;
  title?: string;
  steps: PlaybookStep[];
  selectedIds: string[];
  locale: string;
  contributionsJson: string;
}

export interface PlaybookStep {
  id: string;
  title: string;
  description?: string;
  blocks: PlaybookBlock[];
}

export interface PlaybookBlock {
  id: string;
  label: string;
  kind: string;
}
