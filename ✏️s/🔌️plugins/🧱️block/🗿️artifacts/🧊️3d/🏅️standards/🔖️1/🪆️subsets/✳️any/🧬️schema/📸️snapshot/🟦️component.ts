/** 🧬️ Block3d snapshot schema — artifact-lane fields only. */

export interface Block3dSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objectKind: BlockKindIdentity;
  /** @state artifact */
  representations: BlockRepresentation[];
  /** @state artifact */
  vortexKinds: Block3dVortexKind[];
  /** @state artifact */
  vortices: Block3dVortexTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera3d: BlockCamera3d;
  /** @state artifact */
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
