/** 🔺️ DxfDiff schema facet — handcrafted sparse diff. Mirrors `🔺️diff/🦀️component.rs`
 * field-for-field (camelCase). Name-keyed removed/modified/added triples for
 * headerVars/layers/styles/linetypes; index-keyed triples for blocks/entities. No
 * full-replace `snapshot` slot anywhere. */

import type {
  DxfBlock, DxfEntity, DxfGroupCode, DxfHeaderVar, DxfLayer, DxfLinetype, DxfStyle, DxfVertex,
} from '../📸️snapshot/🟦️component.ts';

//#region HeaderVarDiff
export interface DxfHeaderVarDiff {
  groupCode?: number;
  value?: import('../📸️snapshot/🟦️component.ts').DxfValue;
  extraGroupCodes?: DxfGroupCode[];
}
export interface DxfHeaderVarModified { name: string; diff: DxfHeaderVarDiff; }
export interface DxfHeaderVarAdded { index: number; headerVar: DxfHeaderVar; }
export interface DxfHeaderVarsDiff {
  removed?: string[];
  modified?: DxfHeaderVarModified[];
  added?: DxfHeaderVarAdded[];
}
//#endregion

//#region TableDiffs
export interface DxfLayerDiff { color?: number; linetype?: string; flags?: number; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfLayerModified { name: string; diff: DxfLayerDiff; }
export interface DxfLayerAdded { index: number; layer: DxfLayer; }
export interface DxfLayersDiff { removed?: string[]; modified?: DxfLayerModified[]; added?: DxfLayerAdded[]; }

export interface DxfStyleDiff { flags?: number; fontName?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfStyleModified { name: string; diff: DxfStyleDiff; }
export interface DxfStyleAdded { index: number; style: DxfStyle; }
export interface DxfStylesDiff { removed?: string[]; modified?: DxfStyleModified[]; added?: DxfStyleAdded[]; }

export interface DxfLinetypeDiff { flags?: number; description?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfLinetypeModified { name: string; diff: DxfLinetypeDiff; }
export interface DxfLinetypeAdded { index: number; linetype: DxfLinetype; }
export interface DxfLinetypesDiff { removed?: string[]; modified?: DxfLinetypeModified[]; added?: DxfLinetypeAdded[]; }

export interface DxfTablesDiff {
  layers?: DxfLayersDiff;
  styles?: DxfStylesDiff;
  linetypes?: DxfLinetypesDiff;
}
//#endregion

//#region EntityDiff
export interface DxfLineDiff { start?: [number, number, number]; end?: [number, number, number]; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfCircleDiff { center?: [number, number, number]; radius?: number; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfArcDiff { center?: [number, number, number]; radius?: number; startAngle?: number; endAngle?: number; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfPolylineDiff { vertices?: DxfVertex[]; closed?: boolean; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfTextDiff { position?: [number, number, number]; height?: number; value?: string; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfSolidDiff { points?: [[number, number, number], [number, number, number], [number, number, number], [number, number, number]]; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfInsertDiff { blockName?: string; position?: [number, number, number]; scale?: [number, number, number]; rotation?: number; layer?: string; unknownGroupCodes?: DxfGroupCode[]; }
export interface DxfOtherDiff { groupCodes?: DxfGroupCode[]; }

/** 🔺️ `replace` fires when the entity KIND changes at this index; otherwise a kind-specific
 * sparse field diff (the plan's json/xml "Replace on kind change" rule). */
export type DxfEntityDiff =
  | { replace: { entity: DxfEntity } }
  | { line: DxfLineDiff }
  | { circle: DxfCircleDiff }
  | { arc: DxfArcDiff }
  | { polyline: DxfPolylineDiff }
  | { text: DxfTextDiff }
  | { solid: DxfSolidDiff }
  | { insert: DxfInsertDiff }
  | { other: DxfOtherDiff };

export interface DxfEntityModified { index: number; diff: DxfEntityDiff; }
export interface DxfEntityAdded { index: number; entity: DxfEntity; }
/** 🔺️ Index-keyed triple — reused for both `DxfSnapshot.entities` and each `DxfBlock.entities`. */
export interface DxfEntitiesDiff { removed?: number[]; modified?: DxfEntityModified[]; added?: DxfEntityAdded[]; }
//#endregion

//#region BlockDiff
export interface DxfBlockDiff {
  name?: string;
  basePoint?: [number, number, number];
  entities?: DxfEntitiesDiff;
  unknownGroupCodes?: DxfGroupCode[];
}
export interface DxfBlockModified { index: number; diff: DxfBlockDiff; }
export interface DxfBlockAdded { index: number; block: DxfBlock; }
export interface DxfBlocksDiff { removed?: number[]; modified?: DxfBlockModified[]; added?: DxfBlockAdded[]; }
//#endregion

/** 🔺️ Diff for `stdio.dxf`. `schema` is an identity field and never appears here. */
export interface DxfDiff {
  headerVars?: DxfHeaderVarsDiff;
  tables?: DxfTablesDiff;
  blocks?: DxfBlocksDiff;
  entities?: DxfEntitiesDiff;
}
