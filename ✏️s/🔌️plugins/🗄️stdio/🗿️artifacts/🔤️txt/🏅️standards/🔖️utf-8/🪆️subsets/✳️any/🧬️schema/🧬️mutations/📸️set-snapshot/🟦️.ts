/** 📸️ set-snapshot canonical direct payload. */
import { parseTxtSnapshot, type TxtSnapshot } from '../../📸️snapshot/🟦️.ts';
import {failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';

export interface SetSnapshotPayload { readonly snapshot: TxtSnapshot }

const decode = (value: unknown, path: string): SetSnapshotPayload => {
  const record = txtExact(value, path, ['snapshot']);
  if (!txtOwn(record, 'snapshot')) return failTxtMutationDecode('keys', path);
  return { snapshot: parseTxtSnapshot(record.snapshot, `${path}.snapshot`) };
};

export const decodeSetSnapshotJson = (value: unknown): SetSnapshotPayload => decode(value, 'json');
export const decodeSetSnapshotGraphql = (value: unknown): SetSnapshotPayload => decode(value, 'graphql.setSnapshot');
export const decodeSetSnapshotProto = (value: unknown): SetSnapshotPayload => decode(value, 'protobuf.setSnapshot');
