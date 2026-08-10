/** 🧬️ Block2d diff schema — sparse field delta. */

export interface Block2dDiff {
  /** @state persistent */
  artifact?: Block2dArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  nodeKind?: BlockKindIdentity;
  /** @state persistent */
  presentation?: Block2dPresentation;
  /** @state persistent */
  handleKinds?: Block2dHandleKindsDelta;
  /** @state persistent */
  handles?: Block2dHandlesDelta;
  /** @state persistent */
  compatibility?: Block2dCompatibilityDelta;
  /** @state persistent */
  attributes?: Block2dAttributesDelta;
  /** @state persistent */
  authors?: Block2dAuthorList;
  /** @state persistent */
  camera2d?: BlockCamera2d;
  /** @state persistent */
  meta?: BlockMeta;
  /** @state shared-ui */
  selectedIds?: Block2dStringList;
  /** @state local-ui */
  locale?: string;
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

export interface Block2dStringList {
  values: string[];
}

export interface Block2dAuthorList {
  values: BlockAuthor[];
}

export interface Block2dHandleKindsDelta {
  added: Block2dHandleKind[];
  removed: string[];
  patched: Block2dHandleKindsPatchEntry[];
  reordered?: string[];
}

export interface Block2dHandleKindsPatchEntry {
  id: string;
  patch: Block2dHandleKindsPatch;
}

export interface Block2dHandleKindsPatch {
  replacement?: Block2dHandleKind;
}

export interface Block2dHandlesDelta {
  added: Block2dHandleTemplate[];
  removed: string[];
  patched: Block2dHandlesPatchEntry[];
  reordered?: string[];
}

export interface Block2dHandlesPatchEntry {
  id: string;
  patch: Block2dHandlesPatch;
}

export interface Block2dHandlesPatch {
  replacement?: Block2dHandleTemplate;
}

export interface Block2dCompatibilityDelta {
  added: BlockCompatibilityRule[];
  removed: string[];
  patched: Block2dCompatibilityPatchEntry[];
  reordered?: string[];
}

export interface Block2dCompatibilityPatchEntry {
  id: string;
  patch: Block2dCompatibilityPatch;
}

export interface Block2dCompatibilityPatch {
  replacement?: BlockCompatibilityRule;
}

export interface Block2dAttributesDelta {
  added: BlockAttribute[];
  removed: string[];
  patched: Block2dAttributesPatchEntry[];
  reordered?: string[];
}

export interface Block2dAttributesPatchEntry {
  id: string;
  patch: Block2dAttributesPatch;
}

export interface Block2dAttributesPatch {
  replacement?: BlockAttribute;
}

export interface Block2dArtifact { [key: string]: unknown; }
