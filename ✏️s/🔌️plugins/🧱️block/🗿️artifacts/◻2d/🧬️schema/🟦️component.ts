/** 🧬️ Block2d artifact schema — every field with its state class. */

export interface Block2dArtifact {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  nodeKind: BlockKindIdentity;
  /** @state persistent */
  presentation: Block2dPresentation;
  /** @state persistent */
  handleKinds: Block2dHandleKind[];
  /** @state persistent */
  handles: Block2dHandleTemplate[];
  /** @state persistent */
  compatibility: BlockCompatibilityRule[];
  /** @state persistent */
  attributes: BlockAttribute[];
  /** @state persistent */
  authors: BlockAuthor[];
  /** @state persistent */
  camera2d: BlockCamera2d;
  /** @state persistent */
  meta: BlockMeta;
  /** @state shared-ui */
  selectedIds: string[];
  /** @state local-ui */
  locale: string;
}

export interface BlockKindIdentity { [key: string]: unknown; }

export interface Block2dPresentation { [key: string]: unknown; }

export interface Block2dHandleKind { [key: string]: unknown; }

export interface Block2dHandleTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera2d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }

export interface BlockKindIdentity { [key: string]: unknown; }

export interface Block2dPresentation { [key: string]: unknown; }

export interface Block2dHandleKind { [key: string]: unknown; }

export interface Block2dHandleTemplate { [key: string]: unknown; }

export interface BlockCompatibilityRule { [key: string]: unknown; }

export interface BlockAttribute { [key: string]: unknown; }

export interface BlockAuthor { [key: string]: unknown; }

export interface BlockCamera2d { [key: string]: unknown; }

export interface BlockMeta { [key: string]: unknown; }
