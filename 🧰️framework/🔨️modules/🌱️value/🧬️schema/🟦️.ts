// #region 🧬️Schema
/** 🧬️ Schema leaf: canonical TS mirror of `🔣️.json` for the 🌱️value module — the scope that owns the
 * shared decimal-string scalar primitives every other schema module references rather than restates
 * (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/📋️cross-partition-requests.md`
 * row 151, `📋️execution-contract.md` §A). A `u64` does not survive an IEEE-754 double, so it travels
 * as a canonical decimal string; these parsers are the runtime entry point contract §A requires
 * beside the erased types, and they enforce exactly the `pattern` the JSON half declares.
 * @see 🔣️.json for the normative draft-07 definitions. */

/** 🚫️ Raised when a value does not satisfy the export it was parsed as. */
export class ValueSchemaError extends Error {
  constructor(exported: string, at: string, reason: string) {
    super(`${exported} at ${at} ${reason}`);
    this.name = "ValueSchemaError";
  }
}

const U64_PATTERN =
  /^(0|[1-9][0-9]{0,18}|1[0-7][0-9]{18}|18[0-3][0-9]{17}|184[0-3][0-9]{16}|1844[0-5][0-9]{15}|18446[0-6][0-9]{14}|184467[0-3][0-9]{13}|1844674[0-3][0-9]{12}|184467440[0-6][0-9]{10}|1844674407[0-2][0-9]{9}|18446744073[0-6][0-9]{8}|1844674407370[0-8][0-9]{6}|18446744073709[0-4][0-9]{5}|184467440737095[0-4][0-9]{4}|18446744073709550[0-9]{3}|18446744073709551[0-5][0-9]{2}|1844674407370955160[0-9]|1844674407370955161[0-4]|18446744073709551615)$/u;

/** 🔢️ `$defs.U64` — a `u64` on the wire: canonical decimal, no sign, no leading zero, 0 … 2⁶⁴−1. */
export type U64 = string;

/** 🔢️ `$defs.NonZeroU64` — `U64` minus `"0"`, the actor family's identity/generation narrowing. */
export type NonZeroU64 = string;

/** 🔎️ Parses `$defs.U64`, throwing `ValueSchemaError` on anything the JSON half rejects. */
export function parseU64(value: unknown, at = "U64"): U64 {
  if (typeof value !== "string") throw new ValueSchemaError("U64", at, "is not a string");
  if (value.length > 20 || !U64_PATTERN.test(value)) throw new ValueSchemaError("U64", at, "is not a canonical decimal u64");
  return value;
}

/** 🔎️ Parses `$defs.NonZeroU64` — `parseU64` plus the single excluded value `"0"`. */
export function parseNonZeroU64(value: unknown, at = "NonZeroU64"): NonZeroU64 {
  const decimal = parseU64(value, at);
  if (decimal === "0") throw new ValueSchemaError("NonZeroU64", at, "is the reserved zero sentinel");
  return decimal;
}
// #endregion 🧬️Schema
