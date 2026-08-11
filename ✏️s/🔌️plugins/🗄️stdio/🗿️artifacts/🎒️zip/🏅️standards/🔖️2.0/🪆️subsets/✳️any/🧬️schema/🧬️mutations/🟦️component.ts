/** 🧬️ ZipMutation union — discriminated on `mutation`, mirroring the Rust `ZipMutation` enum. */

import type { ZipEntry, ZipExtraField, ZipCompressionMethod } from '../📸️snapshot/🟦️component.ts';
import type { ZipSnapshot } from '../📸️snapshot/🟦️component.ts';

export type ZipMutation =
  | { mutation: 'noMutation' }
  | { mutation: 'setSnapshot'; snapshot: ZipSnapshot }
  | { mutation: 'setArchiveComment'; comment: string }
  | { mutation: 'addEntry'; index: number; entry: ZipEntry }
  | { mutation: 'removeEntry'; name: string }
  | { mutation: 'renameEntry'; name: string; newName: string }
  | { mutation: 'setEntryData'; name: string; data: number[] }
  | { mutation: 'setEntryMethod'; name: string; method: ZipCompressionMethod }
  | { mutation: 'setEntryTimestamps'; name: string; dosDate: number; dosTime: number; unixMtime?: number | null }
  | { mutation: 'setEntryFlags'; name: string; flags: number }
  | { mutation: 'setEntryVersions'; name: string; versionMadeBy: number; versionNeeded: number }
  | { mutation: 'setEntryAttributes'; name: string; internalAttrs: number; externalAttrs: number }
  | { mutation: 'setEntryExtra'; name: string; localExtra: ZipExtraField[]; centralExtra: ZipExtraField[] }
  | { mutation: 'setEntryComment'; name: string; comment: string };
