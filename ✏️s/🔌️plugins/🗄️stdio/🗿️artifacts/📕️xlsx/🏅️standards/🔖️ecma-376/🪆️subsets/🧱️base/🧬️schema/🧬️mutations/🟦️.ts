import type { XlsxCellValue, XlsxSheet, XlsxSnapshot } from '../📸️snapshot/🟦️.ts';
export type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';
export type { XlsxCellVacancyAddress, XlsxWorksheetAddress } from './🧭️cell-vacancy-address/🟦️.ts';
import type { XlsxCellAddress } from './🧭️cell-address/🟦️.ts';
import type { XlsxCellVacancyAddress } from './🧭️cell-vacancy-address/🟦️.ts';
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';

/** 🧬️ Canonical XLSX mutation union. */
export type XlsxMutation =
  | { mutation: 'setSnapshot'; snapshot: XlsxSnapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: 'insertSheet'; sheet: XlsxSheet }
  | { mutation: 'removeSheet'; name: string }
  | { mutation: 'renameSheet'; name: string; newName: string }
  | { mutation: 'setCell'; address: XlsxCellAddress; value: XlsxCellValue }
  | { mutation: 'insertCell'; address: XlsxCellVacancyAddress; value: XlsxCellValue }
  | { mutation: 'removeCell'; address: XlsxCellAddress }
  | { mutation: 'insertSharedString'; value: string }
  | { mutation: 'removeSharedString'; index: number }
  | { mutation: 'setSharedString'; index: number; value: string };
