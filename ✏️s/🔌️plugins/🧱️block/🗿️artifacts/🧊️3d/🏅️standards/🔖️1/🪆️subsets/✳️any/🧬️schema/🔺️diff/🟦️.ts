import type { Block3dVortexTemplate, BlockAttribute, BlockAuthor, BlockCompatibilityRule, BlockRepresentation } from "../🟦️.ts";
import type { Block3dVortexKind } from "../../../../../../🟦️.ts";

/** 🧬️ Block3d diff schema — a field-sparse, id-keyed delta over the artifact: sub-documents carry field patches, id-keyed lists carry positional rows (`protocol::list_delta`): removed (id at base index), inserted (row at after index), moved (id from base index to after index) and id-keyed modified rows. */

export interface Block3dDiff {
  schema?: string | null;
  objectKind?: BlockKindIdentityPatch | null;
  representations: BlockRepresentationsDelta;
  vortexKinds: Block3dVortexKindsDelta;
  vortices: Block3dVorticesDelta;
  compatibility: BlockCompatibilityDelta;
  attributes: BlockAttributesDelta;
  authors: BlockAuthorsDelta;
  camera3d?: BlockCamera3dPatch | null;
  meta?: BlockMetaPatch | null;
}

export interface BlockOptionalText {
  value: string | null;
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

export interface Block3dVortexKindPatch {
  name?: string | null;
  label?: string | null;
  color?: string | null;
  defaultCableKind?: string | null;
}

export interface Block3dVortexTemplatePatch {
  vortexKind?: string | null;
  position?: readonly [number, number, number] | null;
  direction?: readonly [number, number, number] | null;
  radius?: number | null;
  label?: BlockOptionalText | null;
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

export interface Block3dVortexKindsPatchEntry {
  id: string;
  patch: Block3dVortexKindPatch;
}

export interface Block3dVortexKindsRemoval {
  id: string;
  index: number;
}

export interface Block3dVortexKindsInsertion {
  index: number;
  row: Block3dVortexKind;
}

export interface Block3dVortexKindsRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Block3dVortexKindsDelta {
  removed: Block3dVortexKindsRemoval[];
  inserted: Block3dVortexKindsInsertion[];
  moved: Block3dVortexKindsRelocation[];
  modified: Block3dVortexKindsPatchEntry[];
}

export interface Block3dVorticesPatchEntry {
  id: string;
  patch: Block3dVortexTemplatePatch;
}

export interface Block3dVorticesRemoval {
  id: string;
  index: number;
}

export interface Block3dVorticesInsertion {
  index: number;
  row: Block3dVortexTemplate;
}

export interface Block3dVorticesRelocation {
  id: string;
  from: number;
  to: number;
}

export interface Block3dVorticesDelta {
  removed: Block3dVorticesRemoval[];
  inserted: Block3dVorticesInsertion[];
  moved: Block3dVorticesRelocation[];
  modified: Block3dVorticesPatchEntry[];
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

