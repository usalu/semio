import type { ZipEntry, ZipSnapshot } from '../📸️snapshot/🟦️.ts';
export type ZipMutation =
  | { mutation: 'setSnapshot'; snapshot: ZipSnapshot }
  | { mutation: 'setArchiveComment'; comment: string; commentUtf8: boolean }
  | { mutation: 'addEntry'; entry: ZipEntry; before?: string }
  | { mutation: 'removeEntry'; name: string }
  | { mutation: 'renameEntry'; name: string; newName: string }
  | { mutation: 'setEntryData'; name: string; data: number[] };
