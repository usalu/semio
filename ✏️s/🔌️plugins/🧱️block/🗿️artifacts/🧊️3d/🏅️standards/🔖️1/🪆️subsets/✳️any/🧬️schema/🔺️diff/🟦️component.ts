/** 🧬️ Block3d diff schema — sparse field delta. */

export interface Block3dDiff {
  /** @state artifact */
  artifact?: Block3dArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  objectKind?: BlockKindIdentity;
  /** @state artifact */
  representations?: Block3dRepresentationsDelta;
  /** @state artifact */
  vortexKinds?: Block3dVortexKindsDelta;
  /** @state artifact */
  vortices?: Block3dVorticesDelta;
  /** @state artifact */
  compatibility?: Block3dCompatibilityDelta;
  /** @state artifact */
  attributes?: Block3dAttributesDelta;
  /** @state artifact */
  authors?: Block3dAuthorList;
  /** @state artifact */
  camera3d?: BlockCamera3d;
  /** @state artifact */
  meta?: BlockMeta;
  /** @state presence */
  selectedIds?: Block3dStringList;
  /** @state presence */
  activeRepresentationId?: string | null;
  /** @state presence */
  wantedTags?: Block3dStringList;
  /** @state config */
  locale?: string;
  /** @state config */
  windows?: Block3dWindowsList;
  /** @state config */
  brushVortexKindId?: string | null;
  /** @state config */
  brushRadius?: number;
  /** @state config */
  brushFlip?: boolean;
  /** @state artifact */
  brushPreview?: Block3dBrushPreview | null;
  /** @state config */
  camera?: BlockCamera3d | null;
  /** @state artifact */
  hoveredVortexFullId?: string | null;
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

export interface Block3dStringList {
  values: string[];
}

export interface Block3dAuthorList {
  values: BlockAuthor[];
}

export interface Block3dRepresentationsDelta {
  added: BlockRepresentation[];
  removed: string[];
  patched: Block3dRepresentationsPatchEntry[];
  reordered?: string[];
}

export interface Block3dRepresentationsPatchEntry {
  id: string;
  patch: Block3dRepresentationsPatch;
}

export interface Block3dRepresentationsPatch {
  replacement?: BlockRepresentation;
}

export interface Block3dVortexKindsDelta {
  added: Block3dVortexKind[];
  removed: string[];
  patched: Block3dVortexKindsPatchEntry[];
  reordered?: string[];
}

export interface Block3dVortexKindsPatchEntry {
  id: string;
  patch: Block3dVortexKindsPatch;
}

export interface Block3dVortexKindsPatch {
  replacement?: Block3dVortexKind;
}

export interface Block3dVorticesDelta {
  added: Block3dVortexTemplate[];
  removed: string[];
  patched: Block3dVorticesPatchEntry[];
  reordered?: string[];
}

export interface Block3dVorticesPatchEntry {
  id: string;
  patch: Block3dVorticesPatch;
}

export interface Block3dVorticesPatch {
  replacement?: Block3dVortexTemplate;
}

export interface Block3dCompatibilityDelta {
  added: BlockCompatibilityRule[];
  removed: string[];
  patched: Block3dCompatibilityPatchEntry[];
  reordered?: string[];
}

export interface Block3dCompatibilityPatchEntry {
  id: string;
  patch: Block3dCompatibilityPatch;
}

export interface Block3dCompatibilityPatch {
  replacement?: BlockCompatibilityRule;
}

export interface Block3dAttributesDelta {
  added: BlockAttribute[];
  removed: string[];
  patched: Block3dAttributesPatchEntry[];
  reordered?: string[];
}

export interface Block3dAttributesPatchEntry {
  id: string;
  patch: Block3dAttributesPatch;
}

export interface Block3dAttributesPatch {
  replacement?: BlockAttribute;
}

export interface Block3dWindowsList {
  values: Block3dWindowView[];
}

export interface Block3dArtifact { [key: string]: unknown; }
