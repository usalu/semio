/** 📸️ set-snapshot canonical direct payload. */
import { parseTxtSnapshot, type TxtSnapshot } from '../../📸️snapshot/🟦️.ts';
import {failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';

export interface SetSnapshotPayload { readonly snapshot: TxtSnapshot }

export const parseSetSnapshotPayload = (value: unknown, path = "SetSnapshot"): SetSnapshotPayload => {
  const record = txtExact(value, path, ['snapshot']);
  if (!txtOwn(record, 'snapshot')) return failTxtMutationDecode('keys', path);
  return { snapshot: parseTxtSnapshot(record.snapshot, `${path}.snapshot`) };
};

