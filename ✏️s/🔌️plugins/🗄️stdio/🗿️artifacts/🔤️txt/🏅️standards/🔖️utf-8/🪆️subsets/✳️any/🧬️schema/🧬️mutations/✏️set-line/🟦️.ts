/** 🧬 set-line canonical direct payload. */
import {coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn, txtUnicode} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface SetLinePayload { readonly index: number; readonly text: string }

const decode = (value: unknown, path: string): SetLinePayload => {
  const record = txtExact(value, path, ['index', 'text']);
  if (!txtOwn(record, 'index') || !txtOwn(record, 'text')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index), text: txtUnicode(record.text, `${path}.text`) };
};
export const decodeSetLineJson = (value: unknown): SetLinePayload => decode(value, 'json');
export const decodeSetLineGraphql = (value: unknown): SetLinePayload => decode(value, 'graphql.setLine');
export const decodeSetLineProto = (value: unknown): SetLinePayload => decode(value, 'protobuf.setLine');
