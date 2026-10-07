/** 🔣️ Decoded transport enum spelling at the physical boundary. */
import type {SetLineEndingPayload} from "../../../../🧬️schema/🧬️mutations/🔚️set-line-ending/🟦️.ts";
import {txtExact, txtOwn, failTxtMutationDecode} from "../../../../🧬️schema/🔨️modules/🧬️mutation-support/🟦️.ts";
export const decodeSetLineEndingProto = (value: unknown): SetLineEndingPayload => {
  const record = txtExact(value, 'protobuf.setLineEnding', ['value']);
  if (!txtOwn(record, 'value') || (record.value !== 'LF' && record.value !== 'CR_LF')) return failTxtMutationDecode('keys', 'protobuf.setLineEnding');
  return { value: record.value === 'LF' ? 'lf' : 'crLf' };
};
