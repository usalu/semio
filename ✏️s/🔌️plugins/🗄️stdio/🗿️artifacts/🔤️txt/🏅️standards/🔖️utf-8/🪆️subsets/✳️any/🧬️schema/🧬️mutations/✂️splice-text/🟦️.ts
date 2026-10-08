/** 🧬 splice-text canonical direct payload. */
import {coerceTxtMutationUInt32Variable, failTxtMutationDecode, txtExact, txtOwn, txtUnicode} from '../../🔨️modules/🧬️mutation-support/🟦️.ts';
export interface TextSplice { readonly offset: number; readonly delete: number; readonly insert: string }
export interface SpliceTextPayload { readonly splices: readonly TextSplice[] }

export const parseSpliceTextPayload = (value: unknown, path = "SpliceText"): SpliceTextPayload => {
  const record = txtExact(value, path, ['splices']);
  if (!txtOwn(record, 'splices') || !Array.isArray(record.splices)) return failTxtMutationDecode('keys', path);
  return {
    splices: record.splices.map((entry: unknown, index: number): TextSplice => {
      const at = `${path}.splices[${index}]`;
      const row = txtExact(entry, at, ['offset', 'delete', 'insert']);
      if (!txtOwn(row, 'offset') || !txtOwn(row, 'delete') || !txtOwn(row, 'insert')) return failTxtMutationDecode('keys', at);
      return { offset: coerceTxtMutationUInt32Variable(row.offset), delete: coerceTxtMutationUInt32Variable(row.delete), insert: txtUnicode(row.insert, `${at}.insert`) };
    }),
  };
};
