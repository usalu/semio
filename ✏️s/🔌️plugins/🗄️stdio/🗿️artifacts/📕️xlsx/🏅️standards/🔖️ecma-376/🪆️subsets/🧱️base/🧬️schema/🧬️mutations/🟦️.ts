import type { XlsxCellValue, XlsxSheet } from '../📸️snapshot/🟦️.ts';
export type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';
export type { XlsxCellVacancyAddress, XlsxWorksheetAddress } from './🧭️cell-vacancy-address/🟦️.ts';
import type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';
import type { XlsxCellVacancyAddress } from './🧭️cell-vacancy-address/🟦️.ts';

/** 🧬️ Canonical XLSX mutation union. */
export type XlsxMutation =
  | { mutation: 'insertSheet'; sheet: XlsxSheet; index?: number }
  | { mutation: 'removeSheet'; name: string }
  | { mutation: 'renameSheet'; name: string; newName: string }
  | { mutation: 'setCell'; address: XlsxCellAddress; value: XlsxCellValue }
  | { mutation: 'insertCell'; address: XlsxCellVacancyAddress; value: XlsxCellValue }
  | { mutation: 'removeCell'; address: XlsxCellAddress }
  | { mutation: 'insertSharedString'; value: string; index?: number }
  | { mutation: 'removeSharedString'; index: number }
  | { mutation: 'setSharedString'; index: number; value: string };
