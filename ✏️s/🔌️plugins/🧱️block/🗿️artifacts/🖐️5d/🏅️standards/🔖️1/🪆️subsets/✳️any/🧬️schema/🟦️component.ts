/** 🧬️ Block5d artifact schema — every field with its state class. */

export interface Block5dArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  partKind: BlockKindIdentity;
  /** @state persistent */
  part2d: Block5dPart2d;
  /** @state persistent */
  part3d: Block5dPart3d;
  /** @state persistent */
  representations: BlockRepresentation[];
  /** @state persistent */
  gripKinds: Block5dGripKind[];
  /** @state persistent */
  grips: Block5dGripTemplate[];
  /** @state persistent */
  compatibility: BlockCompatibilityRule[];
  /** @state persistent */
  attributes: BlockAttribute[];
  /** @state persistent */
  authors: BlockAuthor[];
  /** @state persistent */
  camera2d: BlockCamera2d;
  /** @state persistent */
  camera3d: BlockCamera3d;
  /** @state persistent */
  meta: BlockMeta;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state local-ui */
  locale: string;
}

export interface BlockKindIdentity { [key: string]: unknown; }

export interface Block5dPart2d { [key: string]: unknown; }

export interface Block5dPart3d { [key: string]: unknown; }

export interface BlockRepresentation { [key: string]: unknown; }

export interface Block5dGripKind { [key: string]: unknown; }

export interface Block5dGripTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera2d { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }

export interface BlockKindIdentity { [key: string]: unknown; }

export interface Block5dPart2d { [key: string]: unknown; }

export interface Block5dPart3d { [key: string]: unknown; }

export interface BlockRepresentation { [key: string]: unknown; }

export interface Block5dGripKind { [key: string]: unknown; }

export interface Block5dGripTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera2d { [key: string]: unknown; }

export interface BlockCamera3d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }
