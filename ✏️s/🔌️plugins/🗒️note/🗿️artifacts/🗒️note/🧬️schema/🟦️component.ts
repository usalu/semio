/** 🧬️ Note artifact schema — every field with its state class. */

export interface NoteArtifact {
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
  /** @state shared-ui */
  selectedBlockIds: string[];
  /** @state shared-ui */
  activeUtilityId: string;
  /** @state local-ui */
  engagementInput: string;
  /** @state local-ui */
  cameraX: number;
  /** @state local-ui */
  cameraY: number;
  /** @state local-ui */
  cameraZoom: number;
  /** @state local-ui */
  locale: string;
  /** @state preview */
  hoveredBlockId?: string;
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
