/** 🧬️ Ifc2x3Artifact schema. */
export interface Part21Decimal { negative: boolean; coefficient: string; scale: number; exponent?: number; }
export type Part21Value =
  | { kind: 'ref'; value: number }
  | { kind: 'str'; value: string }
  | { kind: 'enum'; value: string }
  | { kind: 'int'; value: number }
  | { kind: 'real'; value: Part21Decimal }
  | { kind: 'list'; values: Part21Value[] }
  | { kind: 'typed'; typeName: string; values: Part21Value[] }
  | { kind: 'unset' }
  | { kind: 'derived' };
export interface Part21Entity { typeName: string; arguments: Part21Value[]; }
export interface Part21Instance { id: number; entities: Part21Entity[]; }
export interface Part21Header { fileDescription: Part21Value[]; fileName: Part21Value[]; fileSchema: Part21Value[]; }
export interface Part21Document { header: Part21Header; instances: Part21Instance[]; }
export interface Ifc2x3EdmPreamble {
  producer: string; module: string; creationDate: string; host: string; database: string; databaseVersion: string;
  databaseCreationDate: string; schema: string; model: string; modelCreationDate: string; headerModel: string;
  headerModelCreationDate: string; user: string; group: string; license: string; options: string;
}
export interface Ifc2x3Artifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ document: Part21Document;
  /** @state artifact */ edmPreamble?: Ifc2x3EdmPreamble;
}
