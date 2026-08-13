/** 🧬️ Block2d diff schema — sparse field delta. */

export interface Block2dDiff {
  /** @state artifact */
  artifact?: Block2dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  nodeKind?: BlockKindIdentity;
  /** @state artifact */
  presentation?: Block2dPresentation;
  /** @state artifact */
  handleKinds?: Block2dHandleKindsDelta;
  /** @state artifact */
  handles?: Block2dHandlesDelta;
  /** @state artifact */
  compatibility?: Block2dCompatibilityDelta;
  /** @state artifact */
  attributes?: Block2dAttributesDelta;
  /** @state artifact */
  authors?: Block2dAuthorList;
  /** @state artifact */
  camera2d?: BlockCamera2d;
  /** @state artifact */
  meta?: BlockMeta;
  /** @state presence */
  selectedIds?: Block2dStringList;
  /** @state config */
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
