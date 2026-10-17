import type { XmlDocument } from '@semio-tech/stdio-xml/standards/1.0/subsets/base/schema/snapshot';

import type {OpcPackage} from '@semio-tech/stdio-zip/opc';
export type {OpcPart,OpcRelationship,OpcPackage} from '@semio-tech/stdio-zip/opc';
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
