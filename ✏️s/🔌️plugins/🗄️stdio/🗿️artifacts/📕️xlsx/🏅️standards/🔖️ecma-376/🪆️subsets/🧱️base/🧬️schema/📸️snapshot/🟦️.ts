import type { XmlDocument } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';

import type {OpcPackage} from '../../../../../../../🎒️zip/📦️opc/🟦️.ts';
export type {OpcPart,OpcRelationship,OpcPackage} from '../../../../../../../🎒️zip/📦️opc/🟦️.ts';
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
