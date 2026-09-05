/** 🔺️ SemioCadDiff schema — real facet mirror of `🦀️.rs` (source of truth). Sparse,
 * handcrafted, name/handle-keyed triples over `../📸️snapshot/🟦️.ts`'s domain types. */
import type { CadBlock, CadEntity, CadLayer, SemioPoint2 } from "../📸️snapshot/🟦️";

export interface NamedModified<D> {
  key: string;
  diff: D;
}
export interface NamedTripleDiff<D, T> {
  removed: string[];
  modified: NamedModified<D>[];
  added: T[];
}

export interface CadLayerDiff {
  colorIndex?: number;
  lineType?: string;
  visible?: boolean;
}

export interface CadEntityRecordDiff {
  layer?: string;
  /** whole-value replaced (weak value, never sub-diffed) */
  entity?: CadEntity;
}

export interface CadBlockDiff {
  basePoint?: SemioPoint2;
  entities?: NamedTripleDiff<CadEntityRecordDiff, import("../📸️snapshot/🟦️").CadEntityRecord>;
}

export type CadLayersDiff = NamedTripleDiff<CadLayerDiff, CadLayer>;
export type CadBlocksDiff = NamedTripleDiff<CadBlockDiff, CadBlock>;
export type CadEntitiesDiff = NamedTripleDiff<CadEntityRecordDiff, import("../📸️snapshot/🟦️").CadEntityRecord>;

export interface SemioCadDiff {
  layers?: CadLayersDiff;
  blocks?: CadBlocksDiff;
  entities?: CadEntitiesDiff;
}
