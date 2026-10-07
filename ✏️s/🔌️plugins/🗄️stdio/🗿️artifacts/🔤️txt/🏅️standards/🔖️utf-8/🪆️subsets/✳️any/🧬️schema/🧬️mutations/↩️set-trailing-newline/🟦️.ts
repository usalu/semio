/** 🧬 set-trailing-newline canonical direct payload. */
import {failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface SetTrailingNewlinePayload { readonly value: boolean }

export const parseSetTrailingNewlinePayload = (value: unknown, path = "SetTrailingNewline"): SetTrailingNewlinePayload => {
  const record = txtExact(value, path, ['value']);
  if (!txtOwn(record, 'value') || typeof record.value !== 'boolean') return failTxtMutationDecode('keys', path);
  return { value: record.value };
};
