import type { Block5dGripKind, Block5dGripTemplate, BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation } from "../🟦️.ts";

/** 🧬️ Block5d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry positional rows (`protocol::list_delta`): removed (id at base index), inserted (row at after index), moved (id from base index to after index) and id-keyed modified rows. */

export interface Block5dDiff {
  schema?: string | null;
  partKind?: BlockKindIdentityPatch | null;
  part2d?: Block5dPart2dPatch | null;
  part3d?: Block5dPart3dPatch | null;
  representations: BlockRepresentationsDelta;
  gripKinds: Block5dGripKindsDelta;
  grips: Block5dGripsDelta;
  compatibility: BlockCompatibilityDelta;
  attributes: BlockAttributesDelta;
  authors: BlockAuthorsDelta;
  camera2d?: BlockCamera2dPatch | null;
  camera3d?: BlockCamera3dPatch | null;
  meta?: BlockMetaPatch | null;
}

export interface BlockOptionalText {
  value: string | null;
}

export interface BlockOptionalNumber {
  value: number | null;
}

export interface BlockOptionalOrientation {
  value: readonly [number, number, number, number] | null;
}

export interface BlockOptionalScale {
  value: readonly [number, number, number] | null;
}

export interface BlockKindIdentityPatch {
  id?: string | null;
  name?: string | null;
  label?: string | null;
  description?: string | null;
  variant?: BlockOptionalText | null;
  icon?: BlockOptionalText | null;
  unit?: BlockOptionalText | null;
}

export interface Block5dPart2dPatch {
  shape?: BlockOptionalText | null;
  radius?: BlockOptionalNumber | null;
  width?: BlockOptionalNumber | null;
  height?: BlockOptionalNumber | null;
  color?: BlockOptionalText | null;
  iconKind?: BlockOptionalText | null;
}

export interface Block5dPart3dPatch {
  orientation?: BlockOptionalOrientation | null;
  scale?: BlockOptionalScale | null;
}

export interface BlockRepresentationPatch {
  name?: string | null;
  meshUrl?: BlockOptionalText | null;
  lod?: BlockOptionalText | null;
  description?: string | null;
  tagsRemoved?: string[] | null;
  tagsAdded?: string[] | null;
  attributesRemoved?: string[] | null;
  attributesAdded?: BlockAttribute[] | null;
}

export interface Block5dGripKindPatch {
  name?: string | null;
  label?: string | null;
  color?: string | null;
  defaultRopeKind?: string | null;
}

export interface Block5dGripTemplatePatch {
  gripKind?: string | null;
  angle?: number | null;
  radius2d?: number | null;
  position?: readonly [number, number, number] | null;
  direction?: readonly [number, number, number] | null;
  radius3d?: number | null;
}

export interface BlockCompatibilityRulePatch {
  source?: string | null;
  target?: string | null;
  bidirectional?: boolean | null;
}

export interface BlockAttributePatch {
  value?: string | null;
  definition?: BlockOptionalText | null;
}

export interface BlockAuthorPatch {
  name?: string | null;
  email?: BlockOptionalText | null;
}

export interface BlockCamera2dPatch {
  x?: number | null;
  y?: number | null;
  zoom?: number | null;
}

export interface BlockCamera3dPatch {
  position?: readonly [number, number, number] | null;
  target?: readonly [number, number, number] | null;
  zoom?: number | null;
}

export interface BlockMetaPatch {
  description?: string | null;
}

export interface BlockRepresentationsPatchEntry {
  id: string;
  patch: BlockRepresentationPatch;
}

export interface BlockRepresentationsRemoval {
  id: string;
  index: number;
}

export interface BlockRepresentationsInsertion {
  index: number;
  row: BlockRepresentation;
}

export interface BlockRepresentationsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface BlockRepresentationsDelta {
  removed: BlockRepresentationsRemoval[];
  inserted: BlockRepresentationsInsertion[];
  moved: BlockRepresentationsRelocation[];
  modified: BlockRepresentationsPatchEntry[];
}

export interface Block5dGripKindsPatchEntry {
  id: string;
  patch: Block5dGripKindPatch;
}

export interface Block5dGripKindsRemoval {
  id: string;
  index: number;
}

export interface Block5dGripKindsInsertion {
  index: number;
  row: Block5dGripKind;
}

export interface Block5dGripKindsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Block5dGripKindsDelta {
  removed: Block5dGripKindsRemoval[];
  inserted: Block5dGripKindsInsertion[];
  moved: Block5dGripKindsRelocation[];
  modified: Block5dGripKindsPatchEntry[];
}

export interface Block5dGripsPatchEntry {
  id: string;
  patch: Block5dGripTemplatePatch;
}

export interface Block5dGripsRemoval {
  id: string;
  index: number;
}

export interface Block5dGripsInsertion {
  index: number;
  row: Block5dGripTemplate;
}

export interface Block5dGripsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Block5dGripsDelta {
  removed: Block5dGripsRemoval[];
  inserted: Block5dGripsInsertion[];
  moved: Block5dGripsRelocation[];
  modified: Block5dGripsPatchEntry[];
}

export interface BlockCompatibilityPatchEntry {
  id: string;
  patch: BlockCompatibilityRulePatch;
}

export interface BlockCompatibilityRemoval {
  id: string;
  index: number;
}

export interface BlockCompatibilityInsertion {
  index: number;
  row: BlockCompatibilityRule;
}

export interface BlockCompatibilityRelocation {
  id: string;
  from: number;
  to: number;
}

export interface BlockCompatibilityDelta {
  removed: BlockCompatibilityRemoval[];
  inserted: BlockCompatibilityInsertion[];
  moved: BlockCompatibilityRelocation[];
  modified: BlockCompatibilityPatchEntry[];
}

export interface BlockAttributesPatchEntry {
  id: string;
  patch: BlockAttributePatch;
}

export interface BlockAttributesRemoval {
  id: string;
  index: number;
}

export interface BlockAttributesInsertion {
  index: number;
  row: BlockAttribute;
}

export interface BlockAttributesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface BlockAttributesDelta {
  removed: BlockAttributesRemoval[];
  inserted: BlockAttributesInsertion[];
  moved: BlockAttributesRelocation[];
  modified: BlockAttributesPatchEntry[];
}

export interface BlockAuthorsPatchEntry {
  id: string;
  patch: BlockAuthorPatch;
}

export interface BlockAuthorsRemoval {
  id: string;
  index: number;
}

export interface BlockAuthorsInsertion {
  index: number;
  row: BlockAuthor;
}

export interface BlockAuthorsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface BlockAuthorsDelta {
  removed: BlockAuthorsRemoval[];
  inserted: BlockAuthorsInsertion[];
  moved: BlockAuthorsRelocation[];
  modified: BlockAuthorsPatchEntry[];
}

