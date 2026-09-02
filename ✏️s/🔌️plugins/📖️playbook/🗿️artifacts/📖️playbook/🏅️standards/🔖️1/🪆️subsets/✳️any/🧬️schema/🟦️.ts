/** 🧬️ Playbook artifact schema — every field with its state class. */

export interface PlaybookArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  version: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  steps: PlaybookStep[];
  /** @state presence */
  selectedIds: string[];
  /** @state config */
  locale: string;
  /** @state config */
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
