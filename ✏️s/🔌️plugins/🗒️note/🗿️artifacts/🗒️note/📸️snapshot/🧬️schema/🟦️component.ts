/** 🧬️ Note snapshot schema — persistent fields only. */

export interface NoteSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  id: string;
  /** @state persistent */
  title?: string;
  /** @state persistent */
  blocks: NoteBlockNode[];
  /** @state persistent */
  gridVisible?: boolean;
  /** @state persistent */
  gridSpacing?: number;
  /** @state persistent */
  gridSubdivisions?: number;
  /** @state persistent */
  gridOpacity?: number;
  /** @state persistent */
  snapEnabled?: boolean;
  /** @state persistent */
  snapGridSpacing?: number;
  /** @state persistent */
  pencilWidth?: number;
  /** @state persistent */
  eraserRadius?: number;
  /** @state persistent */
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
