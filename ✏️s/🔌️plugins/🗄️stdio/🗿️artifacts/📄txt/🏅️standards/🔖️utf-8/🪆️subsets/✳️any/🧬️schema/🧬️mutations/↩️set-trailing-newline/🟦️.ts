/** 🧬 set-trailing-newline canonical direct payload. */
import { TxtProtobufReader, failTxtMutationDecode, txtExact, txtOwn, txtProtobufKey } from '../../🔨️modules/🧬️mutation-support/🟦️component.ts';
export interface SetTrailingNewlinePayload { readonly value: boolean }

const decode = (value: unknown, path: string): SetTrailingNewlinePayload => {
  const record = txtExact(value, path, ['value']);
  if (!txtOwn(record, 'value') || typeof record.value !== 'boolean') return failTxtMutationDecode('keys', path);
  return { value: record.value };
};
export const decodeSetTrailingNewlineJson = (value: unknown): SetTrailingNewlinePayload => decode(value, 'json');
export const decodeSetTrailingNewlineGraphql = (value: unknown): SetTrailingNewlinePayload => decode(value, 'graphql.setTrailingNewline');
export const decodeSetTrailingNewlineProto = (value: unknown): SetTrailingNewlinePayload => decode(value, 'protobuf.setTrailingNewline');
export const decodeSetTrailingNewlineProtobuf = (bytes: Uint8Array): SetTrailingNewlinePayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtMutationDecode('protobuf-wire', 'protobuf.setTrailingNewline');
  const reader = new TxtProtobufReader(bytes);
  let value: boolean | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.setTrailingNewline');
    if (field !== 1) return failTxtMutationDecode('protobuf-unknown', 'protobuf.setTrailingNewline');
    if (wire !== 0) return failTxtMutationDecode('protobuf-wire', 'protobuf.setTrailingNewline.value');
    if (value !== undefined) return failTxtMutationDecode('protobuf-duplicate', 'protobuf.setTrailingNewline.value');
    const raw = reader.varint('protobuf.setTrailingNewline.value');
    if (raw !== 0n && raw !== 1n) return failTxtMutationDecode('protobuf-wire', 'protobuf.setTrailingNewline.value');
    value = raw === 1n;
  }
  return value === undefined ? failTxtMutationDecode('keys', 'protobuf.setTrailingNewline') : { value };
};
