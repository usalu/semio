import type { XlsxCellValue, XlsxSheet, XlsxSnapshot } from '../📸️snapshot/🟦️.ts';
export type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';
import type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';

/** 🧬️ Canonical XLSX mutation union. */
export type XlsxMutation =
  | { mutation: 'setSnapshot'; snapshot: XlsxSnapshot }
  | { mutation: 'insertSheet'; sheet: XlsxSheet }
  | { mutation: 'removeSheet'; name: string }
  | { mutation: 'renameSheet'; name: string; newName: string }
  | { mutation: 'setCell'; address: XlsxCellAddress; value: XlsxCellValue }
  | { mutation: 'removeCell'; address: XlsxCellAddress }
  | { mutation: 'insertSharedString'; value: string }
  | { mutation: 'removeSharedString'; index: number }
  | { mutation: 'setSharedString'; index: number; value: string };
