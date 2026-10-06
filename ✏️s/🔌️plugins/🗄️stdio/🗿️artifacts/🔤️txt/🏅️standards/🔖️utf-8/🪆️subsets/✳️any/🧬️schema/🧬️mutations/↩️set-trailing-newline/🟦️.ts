/** 🧬 set-trailing-newline canonical direct payload. */
import {failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface SetTrailingNewlinePayload { readonly value: boolean }

const decode = (value: unknown, path: string): SetTrailingNewlinePayload => {
  const record = txtExact(value, path, ['value']);
  if (!txtOwn(record, 'value') || typeof record.value !== 'boolean') return failTxtMutationDecode('keys', path);
  return { value: record.value };
};
export const decodeSetTrailingNewlineJson = (value: unknown): SetTrailingNewlinePayload => decode(value, 'json');
export const decodeSetTrailingNewlineGraphql = (value: unknown): SetTrailingNewlinePayload => decode(value, 'graphql.setTrailingNewline');
export const decodeSetTrailingNewlineProto = (value: unknown): SetTrailingNewlinePayload => decode(value, 'protobuf.setTrailingNewline');
