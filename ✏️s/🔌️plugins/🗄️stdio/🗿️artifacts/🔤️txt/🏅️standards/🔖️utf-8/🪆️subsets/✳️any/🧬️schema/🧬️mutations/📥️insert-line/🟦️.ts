/** 🧬 insert-line canonical direct payload. */
import {coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn, txtUnicode} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface InsertLinePayload { readonly index: number; readonly text: string }

const decode = (value: unknown, path: string): InsertLinePayload => {
  const record = txtExact(value, path, ['index', 'text']);
  if (!txtOwn(record, 'index') || !txtOwn(record, 'text')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index), text: txtUnicode(record.text, `${path}.text`) };
};
export const decodeInsertLineJson = (value: unknown): InsertLinePayload => decode(value, 'json');
export const decodeInsertLineGraphql = (value: unknown): InsertLinePayload => decode(value, 'graphql.insertLine');
export const decodeInsertLineProto = (value: unknown): InsertLinePayload => decode(value, 'protobuf.insertLine');
