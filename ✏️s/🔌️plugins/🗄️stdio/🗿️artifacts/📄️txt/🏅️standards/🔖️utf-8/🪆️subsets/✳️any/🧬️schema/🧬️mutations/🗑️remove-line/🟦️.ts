/** 🧬 remove-line canonical direct payload. */
import { TxtProtobufReader, coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn, txtProtobufKey } from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface RemoveLinePayload { readonly index: number }

const decode = (value: unknown, path: string): RemoveLinePayload => {
  const record = txtExact(value, path, ['index']);
  if (!txtOwn(record, 'index')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index) };
};
export const decodeRemoveLineJson = (value: unknown): RemoveLinePayload => decode(value, 'json');
export const decodeRemoveLineGraphql = (value: unknown): RemoveLinePayload => decode(value, 'graphql.removeLine');
export const decodeRemoveLineProto = (value: unknown): RemoveLinePayload => decode(value, 'protobuf.removeLine');
export const decodeRemoveLineProtobuf = (bytes: Uint8Array): RemoveLinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtMutationDecode('protobuf-wire', 'protobuf.removeLine');
  const reader = new TxtProtobufReader(bytes);
  let index: number | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.removeLine');
    if (field !== 1) return failTxtMutationDecode('protobuf-unknown', 'protobuf.removeLine');
    if (wire !== 0) return failTxtMutationDecode('protobuf-wire', 'protobuf.removeLine.index');
    if (index !== undefined) return failTxtMutationDecode('protobuf-duplicate', 'protobuf.removeLine.index');
    const raw = reader.varint('protobuf.removeLine.index');
    index = raw <= 0xffff_ffffn ? Number(raw) : failTxtMutationDecode('u32', 'protobuf.removeLine.index');
  }
  return index === undefined ? failTxtMutationDecode('keys', 'protobuf.removeLine') : { index };
};
