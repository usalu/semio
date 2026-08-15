import type { ZipEntry, ZipSnapshot } from '../📸️snapshot/🟦️component.ts';
export type ZipMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: ZipSnapshot }
  | { mutation: 'setArchiveComment'; comment: string }
  | { mutation: 'addEntry'; entry: ZipEntry }
  | { mutation: 'removeEntry'; name: string }
  | { mutation: 'renameEntry'; name: string; newName: string }
  | { mutation: 'setEntryData'; name: string; data: number[] };
