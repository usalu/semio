/** 🧬️ Block2d artifact schema — every field with its state class. */

export interface Block2dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  nodeKind: BlockKindIdentity;
  /** @state artifact */
  presentation: Block2dPresentation;
  /** @state artifact */
  handleKinds: Block2dHandleKind[];
  /** @state artifact */
  handles: Block2dHandleTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera2d: BlockCamera2d;
  /** @state artifact */
  meta: BlockMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state config */
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
