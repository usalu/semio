/** 🧬 set-line-ending canonical direct payload. */
import {failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface SetLineEndingPayload { readonly value: 'lf' | 'crLf' }

export const parseSetLineEndingPayload = (value: unknown, path = "SetLineEnding"): SetLineEndingPayload => {
  const record = txtExact(value, path, ['value']);
  if (!txtOwn(record, 'value') || (record.value !== 'lf' && record.value !== 'crLf')) return failTxtMutationDecode('keys', path);
  return { value: record.value };
};
