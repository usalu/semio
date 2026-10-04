import type { ZipEntry, ZipSnapshot } from '../📸️snapshot/🟦️.ts';
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';
export type ZipMutation =
  | { mutation: 'setSnapshot'; snapshot: ZipSnapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: 'setArchiveComment'; comment: string; commentUtf8: boolean }
  | { mutation: 'addEntry'; entry: ZipEntry; before?: string }
  | { mutation: 'removeEntry'; name: string }
  | { mutation: 'renameEntry'; name: string; newName: string }
  | { mutation: 'setEntryData'; name: string; data: number[] };
