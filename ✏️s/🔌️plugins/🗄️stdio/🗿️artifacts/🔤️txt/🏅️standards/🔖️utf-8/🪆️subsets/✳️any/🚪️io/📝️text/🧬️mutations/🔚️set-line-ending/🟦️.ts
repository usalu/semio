/** 🔣️ Decoded transport enum spelling at the physical boundary. */
import type {SetLineEndingPayload} from "../../../../🧬️schema/🧬️mutations/🔚️set-line-ending/🟦️.ts";
import {txtExact, txtOwn, failTxtMutationDecode} from "../../../../🧬️schema/🔨️modules/🧬️mutation-support/🟦️.ts";
export const decodeSetLineEndingGraphql = (value: unknown): SetLineEndingPayload => {
  const record = txtExact(value, 'graphql.setLineEnding', ['value']);
  if (!txtOwn(record, 'value') || (record.value !== 'LF' && record.value !== 'CR_LF')) return failTxtMutationDecode('keys', 'graphql.setLineEnding');
  return { value: record.value === 'LF' ? 'lf' : 'crLf' };
};
