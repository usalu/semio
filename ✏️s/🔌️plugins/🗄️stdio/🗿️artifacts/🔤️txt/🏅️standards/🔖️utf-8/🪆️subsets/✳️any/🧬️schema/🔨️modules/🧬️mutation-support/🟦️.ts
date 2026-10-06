/** 🧰 TXT mutation transport primitives. */
export type TxtMutationDecodeErrorCode = 'record' | 'keys' | 'u32' | 'unicode';

export class TxtMutationDecodeError extends Error {
  constructor(readonly code: TxtMutationDecodeErrorCode, readonly path: string) { super(`txt.mutation.${code}:${path}`); }
}

export type TxtWireRecord = Record<string, unknown>;
export const txtMaximumU32 = 0xffff_ffff;
export const failTxtMutationDecode = (code: TxtMutationDecodeErrorCode, path: string): never => { throw new TxtMutationDecodeError(code, path); };
export const txtOwn = (value: object, key: string): boolean => Object.prototype.hasOwnProperty.call(value, key);
export const txtWireRecord = (value: unknown, path: string): TxtWireRecord => {
  if (typeof value !== 'object' || value === null || Array.isArray(value) || (Object.getPrototypeOf(value) !== Object.prototype && Object.getPrototypeOf(value) !== null)) return failTxtMutationDecode('record', path);
  return value as TxtWireRecord;
};
export const txtExact = (value: unknown, path: string, allowed: readonly string[]): TxtWireRecord => {
  const record = txtWireRecord(value, path);
  if (Reflect.ownKeys(record).some((key) => typeof key !== 'string' || !allowed.includes(key))) return failTxtMutationDecode('keys', path);
  return record;
};
export const coerceTxtMutationUInt32Variable = (value: unknown): number => typeof value === 'number' && Number.isFinite(value) && Number.isInteger(value) && value >= 0 && value <= txtMaximumU32 ? value : failTxtMutationDecode('u32', 'graphql.uint32.variable');
export const coerceTxtMutationUInt32Literal = (kind: string, value: unknown): number => kind === 'IntValue' && typeof value === 'string' && /^(?:0|[1-9][0-9]*)$/.test(value) ? coerceTxtMutationUInt32Variable(Number(value)) : failTxtMutationDecode('u32', 'graphql.uint32.literal');
export const txtUnicode = (value: unknown, path: string): string => {
  if (typeof value !== 'string') return failTxtMutationDecode('unicode', path);
  for (let index = 0; index < value.length; index += 1) {
    const code = value.charCodeAt(index);
    if (code >= 0xd800 && code <= 0xdbff) {
      const following = value.charCodeAt(index + 1);
      if (!Number.isInteger(following) || following < 0xdc00 || following > 0xdfff) return failTxtMutationDecode('unicode', path);
      index += 1;
    } else if (code >= 0xdc00 && code <= 0xdfff) return failTxtMutationDecode('unicode', path);
  }
  return value;
};
