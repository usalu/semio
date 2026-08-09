/** 🧬️ Playbook snapshot schema — persistent fields only. */

export interface PlaybookSnapshot {
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
