/** 🧬 set-line canonical direct payload. */
import { TxtProtobufReader, coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn, txtProtobufKey, txtProtobufString, txtUnicode } from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface SetLinePayload { readonly index: number; readonly text: string }

const decode = (value: unknown, path: string): SetLinePayload => {
  const record = txtExact(value, path, ['index', 'text']);
  if (!txtOwn(record, 'index') || !txtOwn(record, 'text')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index), text: txtUnicode(record.text, `${path}.text`) };
};
export const decodeSetLineJson = (value: unknown): SetLinePayload => decode(value, 'json');
export const decodeSetLineGraphql = (value: unknown): SetLinePayload => decode(value, 'graphql.setLine');
export const decodeSetLineProto = (value: unknown): SetLinePayload => decode(value, 'protobuf.setLine');
export const decodeSetLineProtobuf = (bytes: Uint8Array): SetLinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtMutationDecode('protobuf-wire', 'protobuf.setLine');
  const reader = new TxtProtobufReader(bytes);
  let index: number | undefined;
  let text: string | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.setLine');
    if (field === 1 && wire === 0) { if (index !== undefined) return failTxtMutationDecode('protobuf-duplicate', 'protobuf.setLine.index'); const raw = reader.varint('protobuf.setLine.index'); index = raw <= 0xffff_ffffn ? Number(raw) : failTxtMutationDecode('u32', 'protobuf.setLine.index'); }
    else if (field === 2 && wire === 2) { if (text !== undefined) return failTxtMutationDecode('protobuf-duplicate', 'protobuf.setLine.text'); text = txtProtobufString(reader.nested('protobuf.setLine.text'), 'protobuf.setLine.text'); }
    else if (field === 1 || field === 2) return failTxtMutationDecode('protobuf-wire', 'protobuf.setLine');
    else return failTxtMutationDecode('protobuf-unknown', 'protobuf.setLine');
  }
  return index === undefined || text === undefined ? failTxtMutationDecode('keys', 'protobuf.setLine') : { index, text };
};
