import type { XlsxCellValue, XlsxSheet } from '../📸️snapshot/🟦️.ts';
import type { XmlAttr, XmlDocument, XmlNode } from '../../../../../../../📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts';
export type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';
export type { XlsxCellVacancyAddress, XlsxWorksheetAddress } from './🧭️cell-vacancy-address/🟦️.ts';
import type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';
import type { XlsxCellVacancyAddress } from './🧭️cell-vacancy-address/🟦️.ts';

/** 🧩️ Everything the workbook holds for one sheet beyond its typed cells; written verbatim so a removed sheet returns exactly as it stood. */
export interface XlsxSheetSlot {
  readonly attrs: readonly XmlAttr[];
  readonly relationship: { readonly id: string; readonly relType: string; readonly target: string; readonly targetMode: 'internal' | 'external' };
  readonly part_path: string;
  readonly content_type: string;
  readonly document: XmlDocument;
  readonly part_relationships: readonly { readonly id: string; readonly relType: string; readonly target: string; readonly targetMode: 'internal' | 'external' }[];
}

/** 🧬️ Canonical XLSX mutation union. */
export type XlsxMutation =
  | { mutation: 'insertSheet'; sheet: XlsxSheet; index?: number; slot?: XlsxSheetSlot }
  | { mutation: 'removeSheet'; name: string }
  | { mutation: 'renameSheet'; name: string; newName: string }
  | { mutation: 'setCell'; address: XlsxCellAddress; value: XlsxCellValue; node?: XmlNode }
  | { mutation: 'insertCell'; address: XlsxCellVacancyAddress; value: XlsxCellValue; node?: XmlNode }
  | { mutation: 'removeCell'; address: XlsxCellAddress }
  | { mutation: 'insertSharedString'; value: string; index?: number; node?: XmlNode }
  | { mutation: 'removeSharedString'; index: number }
  | { mutation: 'setSharedString'; index: number; value: string; node?: XmlNode }
  | { mutation: 'setRelationship'; owner: string; id: string; relType: string; target: string; external?: boolean; index?: number }
  | { mutation: 'removeRelationship'; owner: string; id: string }
  | { mutation: 'setContentType'; isOverride?: boolean; name: string; contentType: string; index?: number }
  | { mutation: 'removeContentType'; isOverride?: boolean; name: string };
