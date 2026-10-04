import type { DwgSnapshot } from '../📸️snapshot/🟦️.ts';
import type { SnapshotPatch } from '../../../../../../../../📇️registry/🧬️contract/✏️editing/🩹️patch/🟦️.ts';
export type DwgMutation =
  | { mutation: 'setSnapshot'; snapshot: DwgSnapshot }
  | { readonly mutation: 'patchSnapshot'; readonly patch: SnapshotPatch }
  | { mutation: 'setVersionInfo'; version: string; maintenanceVersion: number; codepage: number };
