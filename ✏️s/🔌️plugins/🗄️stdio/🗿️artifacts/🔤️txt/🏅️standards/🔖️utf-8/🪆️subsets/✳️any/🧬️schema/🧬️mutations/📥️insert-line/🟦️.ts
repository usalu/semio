/** 🧬 insert-line canonical direct payload. */
import {coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn, txtUnicode} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface InsertLinePayload { readonly index: number; readonly text: string }

export const parseInsertLinePayload = (value: unknown, path = "InsertLine"): InsertLinePayload => {
  const record = txtExact(value, path, ['index', 'text']);
  if (!txtOwn(record, 'index') || !txtOwn(record, 'text')) return failTxtMutationDecode('keys', path);
  return { index: coerceTxtMutationUInt32Variable(record.index), text: txtUnicode(record.text, `${path}.text`) };
};
