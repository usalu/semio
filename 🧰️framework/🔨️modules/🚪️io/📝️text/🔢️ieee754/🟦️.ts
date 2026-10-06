/** 🔢️ JSON transport admission for exact IEEE scalar identities. */
import {ValueError} from "../../../🌱️value/⚠️refusal/🟦️.ts";
import {parseBinary64,parseBinary32,binary64,binary32,type Binary64,type Binary32} from "../../../🌱️value/🔢️ieee754/🟦️.ts";
/** 🚚️ Admit a binary64 carrier as the value schema's `Binary64Transport` spells it — the exact word (`{bits}` as a bigint, or as its
 * 16 lowercase hex digits on a JSON wire) or a plain JSON number — the reader every artifact twin uses, since an editable float input
 * arrives as a number (`framework/value/schema.json#/$defs/Binary64Transport`). */
export function parseBinary64Transport(value: unknown): Binary64 {
  if (typeof value === "number") return binary64(value);
  if (value !== null && typeof value === "object" && "bits" in value && typeof value.bits === "string") {
    if (!/^[0-9a-f]{16}$/.test(value.bits)) throw new ValueError("invalidValue","binary64 word requires 16 lowercase hex digits");
    return { bits: BigInt("0x" + value.bits) };
  }
  return parseBinary64(value);
}
/** 🚛️ `parseBinary64Transport`'s binary32 twin: the exact word (`{bits}` as an integer, or as its 8 lowercase hex digits) or a plain
 * JSON number captured at the native binary32 width. */
export function parseBinary32Transport(value: unknown): Binary32 {
  if (typeof value === "number") return binary32(value);
  if (value !== null && typeof value === "object" && "bits" in value && typeof value.bits === "string") {
    if (!/^[0-9a-f]{8}$/.test(value.bits)) throw new ValueError("invalidValue","binary32 word requires 8 lowercase hex digits");
    return { bits: parseInt(value.bits, 16) };
  }
  return parseBinary32(value);
}
