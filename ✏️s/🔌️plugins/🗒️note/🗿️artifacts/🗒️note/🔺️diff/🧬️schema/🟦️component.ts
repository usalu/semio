/** 🧬️ Note diff schema — sparse field delta. */

export interface NoteDiff {
  /** @state persistent */
  artifact?: NoteArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  id?: string;
  /** @state persistent */
  title?: string | null;
  /** @state persistent */
  blocks?: NoteBlocksDelta;
  /** @state persistent */
  gridVisible?: boolean | null;
  /** @state persistent */
  gridSpacing?: number | null;
  /** @state persistent */
  gridSubdivisions?: number | null;
  /** @state persistent */
  gridOpacity?: number | null;
  /** @state persistent */
  snapEnabled?: boolean | null;
  /** @state persistent */
  snapGridSpacing?: number | null;
  /** @state persistent */
  pencilWidth?: number | null;
  /** @state persistent */
  eraserRadius?: number | null;
  /** @state persistent */
  assets?: NoteAssetsDelta;
  /** @state shared-ui */
  selectedBlockIds?: NoteStringList;
  /** @state shared-ui */
  activeUtilityId?: string;
  /** @state local-ui */
  engagementInput?: string;
  /** @state local-ui */
  cameraX?: number;
  /** @state local-ui */
  cameraY?: number;
  /** @state local-ui */
  cameraZoom?: number;
  /** @state local-ui */
  locale?: string;
  /** @state preview */
  hoveredBlockId?: string | null;
}

export interface NoteArtifact {
  schema: string;
  id: string;
  title?: string;
  blocks: NoteBlockNode[];
  gridVisible?: boolean;
  gridSpacing?: number;
  gridSubdivisions?: number;
  gridOpacity?: number;
  snapEnabled?: boolean;
  snapGridSpacing?: number;
  pencilWidth?: number;
  eraserRadius?: number;
  assets: Record<string, NoteImageAsset>;
  selectedBlockIds: string[];
  activeUtilityId: string;
  engagementInput: string;
  cameraX: number;
  cameraY: number;
  cameraZoom: number;
  locale: string;
  hoveredBlockId?: string;
}

export interface NoteAssetsDelta {
  entries: Record<string, NoteImageAsset | null>;
}

export interface NoteStringList {
  values: string[];
}

export interface NoteBlocksDelta {
  added: NoteBlockNode[];
  removed: string[];
  patched: NoteBlockPatchEntry[];
  reordered?: string[];
}

export interface NoteBlockPatchEntry {
  id: string;
  patch: NoteBlockPatch;
}

export interface NoteBlockPatch {
  blockJson?: string;
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
