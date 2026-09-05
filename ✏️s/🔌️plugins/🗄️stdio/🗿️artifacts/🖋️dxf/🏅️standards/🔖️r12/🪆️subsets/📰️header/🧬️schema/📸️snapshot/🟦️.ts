/** 🧬️ DxfSnapshot schema facet — complete per DXF R12 ASCII spec. Mirrors
 * `📸️snapshot/🦀️.rs` field-for-field (camelCase). */

/** 🧮 Typed union over DXF group-code value kinds (string/integer/double/point-component). */
export type DxfValue =
  | { kind: 'str'; value: string }
  | { kind: 'int'; value: number }
  | { kind: 'double'; value: number }
  | { kind: 'point'; value: [number, number, number] };

export type DxfGroupCode = [number, DxfValue];

/** 🏷️ One raw DXF group-code/value pair — raw-retention unit only. */
export interface DxfTag {
  code: number;
  value: string;
}

/** 🏷️ One `$VAR` header entry. */
export interface DxfHeaderVar {
  name: string;
  groupCode: number;
  value: DxfValue;
  extraGroupCodes?: DxfGroupCode[];
}

/** 🗂️ `LAYER` table entry. */
export interface DxfLayer {
  name: string;
  color: number;
  linetype: string;
  flags: number;
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🗂️ `STYLE` table entry. */
export interface DxfStyle {
  name: string;
  flags: number;
  fontName: string;
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🗂️ `LTYPE` table entry. */
export interface DxfLinetype {
  name: string;
  flags: number;
  description: string;
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🗂️ The three name-keyed table kinds this codec typed-models. */
export interface DxfTables {
  layers: DxfLayer[];
  styles: DxfStyle[];
  linetypes: DxfLinetype[];
}

/** 🕳️ Raw retention for any table kind other than LAYER/STYLE/LTYPE. */
export interface DxfOtherTable {
  name: string;
  tags: DxfTag[];
}

/** 📍 One `POLYLINE` vertex record. */
export interface DxfVertex {
  x: number;
  y: number;
  z: number;
  bulge: number;
  unknownGroupCodes?: DxfGroupCode[];
}

type Vec3 = [number, number, number];

/** 📐️ The R12 entity set this codec types directly; `other` retains any unmodeled kind. */
export type DxfEntity =
  | { line: { start: Vec3; end: Vec3; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { circle: { center: Vec3; radius: number; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { arc: { center: Vec3; radius: number; startAngle: number; endAngle: number; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { polyline: { vertices: DxfVertex[]; closed: boolean; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { text: { position: Vec3; height: number; value: string; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { solid: { points: [Vec3, Vec3, Vec3, Vec3]; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { insert: { blockName: string; position: Vec3; scale: Vec3; rotation: number; layer: string; unknownGroupCodes?: DxfGroupCode[] } }
  | { other: { kind: string; groupCodes?: DxfGroupCode[] } };

/** 🧱 One `BLOCK` — name, base point, its own nested entity list. */
export interface DxfBlock {
  name: string;
  basePoint: Vec3;
  entities: DxfEntity[];
  unknownGroupCodes?: DxfGroupCode[];
}

/** 🧬️ Complete `stdio.dxf` (r12) document snapshot. */
export interface DxfSnapshot {
  schema: string;
  headerVars: DxfHeaderVar[];
  tables: DxfTables;
  otherTables: DxfOtherTable[];
  blocks: DxfBlock[];
  entities: DxfEntity[];
}
