/** 🧬️ Note diff schema — sparse field delta. */

export interface NoteDiff {
  /** @state artifact */
  artifact?: NoteArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  blocks?: NoteBlocksDelta;
  /** @state artifact */
  gridVisible?: boolean | null;
  /** @state artifact */
  gridSpacing?: number | null;
  /** @state artifact */
  gridSubdivisions?: number | null;
  /** @state artifact */
  gridOpacity?: number | null;
  /** @state artifact */
  snapEnabled?: boolean | null;
  /** @state artifact */
  snapGridSpacing?: number | null;
  /** @state artifact */
  pencilWidth?: number | null;
  /** @state artifact */
  eraserRadius?: number | null;
  /** @state artifact */
  assets?: NoteAssetsDelta;
  /** @state presence */
  selectedBlockIds?: NoteStringList;
  /** @state presence */
  activeUtilityId?: string;
  /** @state config */
  engagementInput?: string;
  /** @state config */
  cameraX?: number;
  /** @state config */
  cameraY?: number;
  /** @state config */
  cameraZoom?: number;
  /** @state config */
  locale?: string;
  /** @state artifact */
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
