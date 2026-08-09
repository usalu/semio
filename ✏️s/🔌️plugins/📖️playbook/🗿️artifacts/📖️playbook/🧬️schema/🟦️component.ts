/** 🧬️ Playbook artifact schema — every field with its state class. */

export interface PlaybookArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  version: string;
  /** @state persistent */
  title?: string;
  /** @state persistent */
  steps: PlaybookStep[];
  /** @state shared-ui */
  selectedIds: string[];
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
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
  description?: string;
  required?: boolean;
  placeholder?: string;
  text?: string;
}
