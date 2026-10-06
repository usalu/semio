/** 🧬 set-line-ending canonical direct payload. */
import {failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
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
