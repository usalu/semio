/** 🧬 remove-line canonical direct payload. */
import {coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface RemoveLinePayload { readonly index: number }

export const parseRemoveLinePayload = (value: unknown, path = "RemoveLine"): RemoveLinePayload => {
  const record = txtExact(value, path, ['index']);
  if (!txtOwn(record, 'index')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index) };
};
