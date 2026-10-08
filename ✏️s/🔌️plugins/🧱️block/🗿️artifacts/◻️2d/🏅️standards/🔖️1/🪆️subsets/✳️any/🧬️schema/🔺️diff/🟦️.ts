import type { Block2dHandleKind, Block2dHandleTemplate, BlockAttribute, BlockAuthor, BlockCompatibilityRule } from "../🟦️.ts";

/** 🧬️ Block2d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry positional rows (`protocol::list_delta`): removed (id at base index), inserted (row at after index), moved (id from base index to after index) and id-keyed modified rows. */

export interface Block2dDiff {
  schema?: string | null;
  nodeKind?: BlockKindIdentityPatch | null;
  presentation?: Block2dPresentationPatch | null;
  handleKinds: Block2dHandleKindsDelta;
  handles: Block2dHandlesDelta;
  compatibility: BlockCompatibilityDelta;
  attributes: BlockAttributesDelta;
  authors: BlockAuthorsDelta;
  camera2d?: BlockCamera2dPatch | null;
  meta?: BlockMetaPatch | null;
}

export interface BlockOptionalText {
  value: string | null;
}

export interface BlockOptionalNumber {
  value: number | null;
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

export interface Block2dPresentationPatch {
  shape?: BlockOptionalText | null;
  radius?: BlockOptionalNumber | null;
  width?: BlockOptionalNumber | null;
  height?: BlockOptionalNumber | null;
  color?: BlockOptionalText | null;
  iconKind?: BlockOptionalText | null;
}

export interface Block2dHandleKindPatch {
  name?: string | null;
  label?: string | null;
  color?: string | null;
  defaultWireKind?: string | null;
}

export interface Block2dHandleTemplatePatch {
  handleKind?: string | null;
  angle?: number | null;
  radius?: number | null;
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

export interface BlockMetaPatch {
  description?: string | null;
}

export interface Block2dHandleKindsPatchEntry {
  id: string;
  patch: Block2dHandleKindPatch;
}

export interface Block2dHandleKindsRemoval {
  id: string;
  index: number;
}

export interface Block2dHandleKindsInsertion {
  index: number;
  row: Block2dHandleKind;
}

export interface Block2dHandleKindsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Block2dHandleKindsDelta {
  removed: Block2dHandleKindsRemoval[];
  inserted: Block2dHandleKindsInsertion[];
  moved: Block2dHandleKindsRelocation[];
  modified: Block2dHandleKindsPatchEntry[];
}

export interface Block2dHandlesPatchEntry {
  id: string;
  patch: Block2dHandleTemplatePatch;
}

export interface Block2dHandlesRemoval {
  id: string;
  index: number;
}

export interface Block2dHandlesInsertion {
  index: number;
  row: Block2dHandleTemplate;
}

export interface Block2dHandlesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Block2dHandlesDelta {
  removed: Block2dHandlesRemoval[];
  inserted: Block2dHandlesInsertion[];
  moved: Block2dHandlesRelocation[];
  modified: Block2dHandlesPatchEntry[];
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

