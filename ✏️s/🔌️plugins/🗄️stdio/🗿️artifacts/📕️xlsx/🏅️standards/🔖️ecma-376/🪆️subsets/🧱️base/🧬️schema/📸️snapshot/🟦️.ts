import type { XmlDocument } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';

export interface OpcPart { path: string; contentType: string; bytes: number[] }
export interface OpcRelationship { id: string; relType: string; target: string; targetMode: 'internal' | 'external' }
export interface OpcPackage {
  parts: OpcPart[];
  contentTypes: { defaults: [string, string][]; overrides: [string, string][] };
  relationships: Record<string, OpcRelationship[]>;
  comment: string;
}
export type XlsxCellValue =
  | { kind: 'number'; value: number }
  | { kind: 'sharedString'; value: number }
  | { kind: 'inlineString'; value: string }
  | { kind: 'boolean'; value: boolean }
  | { kind: 'error'; value: string }
  | { kind: 'formula'; expr: string; cached?: XlsxCellValue | null }
  | { kind: 'empty' };
export interface XlsxCell { row: number; col: number; value: XlsxCellValue }
export interface XlsxSheet { name: string; cells: XlsxCell[] }
export interface XlsxWorkbook { sheets: XlsxSheet[]; sharedStrings: string[] }
export interface XlsxXmlPart { path: string; contentType: string; document: XmlDocument }
export interface XlsxSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ opc: OpcPackage;
  /** @state artifact */ xmlParts: XlsxXmlPart[];
}
