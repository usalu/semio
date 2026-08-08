/** 🧬️ Block3d snapshot schema — persistent fields only. */

export interface Block3dSnapshot {
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
