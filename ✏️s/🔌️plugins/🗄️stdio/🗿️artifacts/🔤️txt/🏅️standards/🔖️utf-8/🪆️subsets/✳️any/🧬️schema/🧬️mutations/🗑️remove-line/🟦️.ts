/** 🧬 remove-line canonical direct payload. */
import {coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface RemoveLinePayload { readonly index: number }

const decode = (value: unknown, path: string): RemoveLinePayload => {
  const record = txtExact(value, path, ['index']);
  if (!txtOwn(record, 'index')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index) };
};
export const decodeRemoveLineJson = (value: unknown): RemoveLinePayload => decode(value, 'json');
export const decodeRemoveLineGraphql = (value: unknown): RemoveLinePayload => decode(value, 'graphql.removeLine');
export const decodeRemoveLineProto = (value: unknown): RemoveLinePayload => decode(value, 'protobuf.removeLine');
