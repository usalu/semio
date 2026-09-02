/** 🧬️ Block5d diff schema — sparse field delta. */

export interface Block5dDiff {
  /** @state artifact */
  artifact?: Block5dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  partKind?: BlockKindIdentity;
  /** @state artifact */
  part2d?: Block5dPart2d;
  /** @state artifact */
  part3d?: Block5dPart3d;
  /** @state artifact */
  representations?: Block5dRepresentationsDelta;
  /** @state artifact */
  gripKinds?: Block5dGripKindsDelta;
  /** @state artifact */
  grips?: Block5dGripsDelta;
  /** @state artifact */
  compatibility?: Block5dCompatibilityDelta;
  /** @state artifact */
  attributes?: Block5dAttributesDelta;
  /** @state artifact */
  authors?: Block5dAuthorList;
  /** @state artifact */
  camera2d?: BlockCamera2d;
  /** @state artifact */
  camera3d?: BlockCamera3d;
  /** @state artifact */
  meta?: BlockMeta;
  /** @state presence */
  selectedIds?: Block5dStringList;
  /** @state config */
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
