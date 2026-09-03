/** 🧬 set-line-ending canonical direct payload. */
import { TxtProtobufReader, failTxtMutationDecode, txtExact, txtOwn, txtProtobufKey } from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface SetLineEndingPayload { readonly value: 'lf' | 'crLf' }

const decode = (value: unknown, path: string): SetLineEndingPayload => {
  const record = txtExact(value, path, ['value']);
  if (!txtOwn(record, 'value') || (record.value !== 'lf' && record.value !== 'crLf')) return failTxtMutationDecode('keys', path);
  return { value: record.value };
};
export const decodeSetLineEndingJson = (value: unknown): SetLineEndingPayload => decode(value, 'json');
export const decodeSetLineEndingGraphql = (value: unknown): SetLineEndingPayload => {
  const record = txtExact(value, 'graphql.setLineEnding', ['value']);
  if (!txtOwn(record, 'value') || (record.value !== 'LF' && record.value !== 'CR_LF')) return failTxtMutationDecode('keys', 'graphql.setLineEnding');
  return { value: record.value === 'LF' ? 'lf' : 'crLf' };
};
export const decodeSetLineEndingProto = (value: unknown): SetLineEndingPayload => {
  const record = txtExact(value, 'protobuf.setLineEnding', ['value']);
  if (!txtOwn(record, 'value') || (record.value !== 'LF' && record.value !== 'CR_LF')) return failTxtMutationDecode('keys', 'protobuf.setLineEnding');
  return { value: record.value === 'LF' ? 'lf' : 'crLf' };
};
export const decodeSetLineEndingProtobuf = (bytes: Uint8Array): SetLineEndingPayload => {
  if (!(bytes instanceof Uint8Array)) return failTxtMutationDecode('protobuf-wire', 'protobuf.setLineEnding');
  const reader = new TxtProtobufReader(bytes);
  let value: 'lf' | 'crLf' | undefined;
  while (reader.remaining) {
    const [field, wire] = txtProtobufKey(reader, 'protobuf.setLineEnding');
    if (field !== 1) return failTxtMutationDecode('protobuf-unknown', 'protobuf.setLineEnding');
    if (wire !== 0) return failTxtMutationDecode('protobuf-wire', 'protobuf.setLineEnding.value');
    if (value !== undefined) return failTxtMutationDecode('protobuf-duplicate', 'protobuf.setLineEnding.value');
    const raw = reader.varint('protobuf.setLineEnding.value');
    value = raw === 0n ? 'lf' : raw === 1n ? 'crLf' : failTxtMutationDecode('protobuf-wire', 'protobuf.setLineEnding.value');
  }
  return value === undefined ? failTxtMutationDecode('keys', 'protobuf.setLineEnding') : { value };
};
