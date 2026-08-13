/** 🧬️ Note snapshot schema — persistent fields only. */

export interface NoteSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  id: string;
  /** @state artifact */
  title?: string;
  /** @state artifact */
  blocks: NoteBlockNode[];
  /** @state artifact */
  gridVisible?: boolean;
  /** @state artifact */
  gridSpacing?: number;
  /** @state artifact */
  gridSubdivisions?: number;
  /** @state artifact */
  gridOpacity?: number;
  /** @state artifact */
  snapEnabled?: boolean;
  /** @state artifact */
  snapGridSpacing?: number;
  /** @state artifact */
  pencilWidth?: number;
  /** @state artifact */
  eraserRadius?: number;
  /** @state artifact */
  assets: Record<string, NoteImageAsset>;
}

export interface NoteBlockNode {
  kind: string;
  [key: string]: unknown;
}

export interface NoteImageAsset {
  mime: string;
  data: string;
  width?: number;
  height?: number;
}
