/** 🧬️ PlySnapshot schema — complete per PLY's generic element/property system. */

export type PlyScalarType = 'char' | 'uChar' | 'short' | 'uShort' | 'int' | 'uInt' | 'float' | 'double';

/** 🧩️ One `property` declaration inside an `element` block. */
export type PlyProperty =
  | { form: 'scalar'; name: string; kind: PlyScalarType }
  | { form: 'list'; name: string; countKind: PlyScalarType; valueKind: PlyScalarType };

/** 🔣️ One typed cell value (adjacently tagged: `kind` + `value`). */
export type PlyValue =
  | { kind: 'char'; value: number }
  | { kind: 'uChar'; value: number }
  | { kind: 'short'; value: number }
  | { kind: 'uShort'; value: number }
  | { kind: 'int'; value: number }
  | { kind: 'uInt'; value: number }
  | { kind: 'float'; value: number }
  | { kind: 'double'; value: number }
  | { kind: 'list'; value: PlyValue[] };

/** 📏 One element instance's data — one `PlyValue` per declared property, same order. */
export interface PlyRow {
  values: PlyValue[];
}

/** 🧱 One `element <name> <count>` block. */
export interface PlyElement {
  name: string;
  count: number;
  properties: PlyProperty[];
  rows: PlyRow[];
}

export type PlyFormat = 'ascii' | 'binaryLittleEndian' | 'binaryBigEndian';

/** 📸️ Persisted `stdio.ply` snapshot. */
export interface PlySnapshot {
  schema: string;
  format: PlyFormat;
  comments: string[];
  elements: PlyElement[];
}
