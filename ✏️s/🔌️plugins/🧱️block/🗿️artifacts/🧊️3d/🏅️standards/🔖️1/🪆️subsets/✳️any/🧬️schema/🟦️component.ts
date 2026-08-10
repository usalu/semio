/** 🧬️ Block3d artifact schema — every field with its state class. */

export interface Block3dArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  objectKind: BlockKindIdentity;
  /** @state persistent */
  representations: BlockRepresentation[];
  /** @state persistent */
  vortexKinds: Block3dVortexKind[];
  /** @state persistent */
  vortices: Block3dVortexTemplate[];
  /** @state persistent */
  compatibility: BlockCompatibilityRule[];
  /** @state persistent */
  attributes: BlockAttribute[];
  /** @state persistent */
  authors: BlockAuthor[];
  /** @state persistent */
  camera3d: BlockCamera3d;
  /** @state persistent */
  meta: BlockMeta;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state shared-ui */
  activeRepresentationId?: string;
  /** @state shared-ui */
  wantedTags: string[];
  /** @state local-ui */
  locale: string;
  /** @state local-ui */
  windows: Block3dWindowView[];
  /** @state local-ui */
  brushVortexKindId?: string;
  /** @state local-ui */
  brushRadius: number;
  /** @state local-ui */
  brushFlip: boolean;
  /** @state preview */
  brushPreview?: Block3dBrushPreview;
  /** @state local-ui */
  camera?: BlockCamera3d;
  /** @state preview */
  hoveredVortexFullId?: string;
}

export interface BlockKindIdentity { [key: string]: unknown; }

export interface BlockRepresentation { [key: string]: unknown; }

export interface Block3dVortexKind { [key: string]: unknown; }

export interface Block3dVortexTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }

export interface Block3dWindowView { [key: string]: unknown; }

export interface Block3dBrushPreview { [key: string]: unknown; }

export interface BlockKindIdentity { [key: string]: unknown; }

export interface BlockRepresentation { [key: string]: unknown; }

export interface Block3dVortexKind { [key: string]: unknown; }

export interface Block3dVortexTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }

export interface Block3dWindowView { [key: string]: unknown; }

export interface Block3dBrushPreview { [key: string]: unknown; }
