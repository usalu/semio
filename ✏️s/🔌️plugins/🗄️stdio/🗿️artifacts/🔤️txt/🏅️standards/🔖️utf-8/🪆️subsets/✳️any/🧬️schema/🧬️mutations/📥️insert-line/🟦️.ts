/** 🧬 insert-line canonical direct payload. */
import { TxtProtobufReader, coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn, txtProtobufKey, txtProtobufString, txtUnicode } from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface InsertLinePayload { readonly index: number; readonly text: string }

const decode = (value: unknown, path: string): InsertLinePayload => {
  const record = txtExact(value, path, ['index', 'text']);
  if (!txtOwn(record, 'index') || !txtOwn(record, 'text')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index), text: txtUnicode(record.text, `${path}.text`) };
};
export const decodeInsertLineJson = (value: unknown): InsertLinePayload => decode(value, 'json');
export const decodeInsertLineGraphql = (value: unknown): InsertLinePayload => decode(value, 'graphql.insertLine');
export const decodeInsertLineProto = (value: unknown): InsertLinePayload => decode(value, 'protobuf.insertLine');
export const decodeInsertLineProtobuf = (bytes: Uint8Array): InsertLinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtMutationDecode('protobuf-wire', 'protobuf.insertLine');
  const reader = new TxtProtobufReader(bytes);
  let index: number | undefined;
  let text: string | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.insertLine');
    if (field === 1 && wire === 0) { if (index !== undefined) return failTxtMutationDecode('protobuf-duplicate', 'protobuf.insertLine.index'); const raw = reader.varint('protobuf.insertLine.index'); index = raw <= 0xffff_ffffn ? Number(raw) : failTxtMutationDecode('u32', 'protobuf.insertLine.index'); }
    else if (field === 2 && wire === 2) { if (text !== undefined) return failTxtMutationDecode('protobuf-duplicate', 'protobuf.insertLine.text'); text = txtProtobufString(reader.nested('protobuf.insertLine.text'), 'protobuf.insertLine.text'); }
    else if (field === 1 || field === 2) return failTxtMutationDecode('protobuf-wire', 'protobuf.insertLine');
    else return failTxtMutationDecode('protobuf-unknown', 'protobuf.insertLine');
  }
  return index === undefined || text === undefined ? failTxtMutationDecode('keys', 'protobuf.insertLine') : { index, text };
};
