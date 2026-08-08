/** 🧬️ Block5d diff schema — sparse field delta. */

export interface Block5dDiff {
  /** @state persistent */
  artifact?: Block5dArtifact;
  /** @state persistent */
  schema?: string;
  /** @state persistent */
  partKind?: BlockKindIdentity;
  /** @state persistent */
  part2d?: Block5dPart2d;
  /** @state persistent */
  part3d?: Block5dPart3d;
  /** @state persistent */
  representations?: Block5dRepresentationsDelta;
  /** @state persistent */
  gripKinds?: Block5dGripKindsDelta;
  /** @state persistent */
  grips?: Block5dGripsDelta;
  /** @state persistent */
  compatibility?: Block5dCompatibilityDelta;
  /** @state persistent */
  attributes?: Block5dAttributesDelta;
  /** @state persistent */
  authors?: Block5dAuthorList;
  /** @state persistent */
  camera2d?: BlockCamera2d;
  /** @state persistent */
  camera3d?: BlockCamera3d;
  /** @state persistent */
  meta?: BlockMeta;
  /** @state shared-ui */
  selectedIds?: Block5dStringList;
  /** @state local-ui */
  locale?: string;
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

export interface Block5dStringList {
  values: string[];
}

export interface Block5dAuthorList {
  values: BlockAuthor[];
}

export interface Block5dRepresentationsDelta {
  added: BlockRepresentation[];
  removed: string[];
  patched: Block5dRepresentationsPatchEntry[];
  reordered?: string[];
}

export interface Block5dRepresentationsPatchEntry {
  id: string;
  patch: Block5dRepresentationsPatch;
}

export interface Block5dRepresentationsPatch {
  replacement?: BlockRepresentation;
}

export interface Block5dGripKindsDelta {
  added: Block5dGripKind[];
  removed: string[];
  patched: Block5dGripKindsPatchEntry[];
  reordered?: string[];
}

export interface Block5dGripKindsPatchEntry {
  id: string;
  patch: Block5dGripKindsPatch;
}

export interface Block5dGripKindsPatch {
  replacement?: Block5dGripKind;
}

export interface Block5dGripsDelta {
  added: Block5dGripTemplate[];
  removed: string[];
  patched: Block5dGripsPatchEntry[];
  reordered?: string[];
}

export interface Block5dGripsPatchEntry {
  id: string;
  patch: Block5dGripsPatch;
}

export interface Block5dGripsPatch {
  replacement?: Block5dGripTemplate;
}

export interface Block5dCompatibilityDelta {
  added: BlockCompatibilityRule[];
  removed: string[];
  patched: Block5dCompatibilityPatchEntry[];
  reordered?: string[];
}

export interface Block5dCompatibilityPatchEntry {
  id: string;
  patch: Block5dCompatibilityPatch;
}

export interface Block5dCompatibilityPatch {
  replacement?: BlockCompatibilityRule;
}

export interface Block5dAttributesDelta {
  added: BlockAttribute[];
  removed: string[];
  patched: Block5dAttributesPatchEntry[];
  reordered?: string[];
}

export interface Block5dAttributesPatchEntry {
  id: string;
  patch: Block5dAttributesPatch;
}

export interface Block5dAttributesPatch {
  replacement?: BlockAttribute;
}

export interface Block5dArtifact { [key: string]: unknown; }
