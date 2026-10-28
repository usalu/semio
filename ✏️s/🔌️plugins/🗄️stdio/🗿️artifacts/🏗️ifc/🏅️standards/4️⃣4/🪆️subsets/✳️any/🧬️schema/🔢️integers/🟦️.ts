/** 🔢️ Owned full-width integer scalar names used by the IFC4 GraphQL facets. */
export type IfcGraphqlIntegerScalar = "IfcUInt64" | "IfcInt64" | "IfcIndex";

/** 🛂️ Parses canonical decimal strings without accepting JavaScript numeric coercion. */
export function parseIfcGraphqlInteger(scalar: IfcGraphqlIntegerScalar, value: unknown): bigint {
  if (!["IfcUInt64", "IfcInt64", "IfcIndex"].includes(scalar)) throw new Error("ifc.graphql.unknown-integer-scalar");
  if (typeof value !== "string" || value.length > 21 || !/^(?:0|-?[1-9][0-9]*)$/.test(value)) throw new Error("ifc.graphql.decimal-string-required");
  const integer = BigInt(value), signed = scalar === "IfcInt64";
  if (integer < (signed ? -9223372036854775808n : 0n) || integer > (signed ? 9223372036854775807n : 18446744073709551615n)) throw new Error("ifc.graphql.integer-out-of-range");
  return integer;
}

/** 🖨️ Serializes an owned bigint as its exact decimal transport string. */
export function printIfcGraphqlInteger(scalar: IfcGraphqlIntegerScalar, value: bigint): string {
  if (typeof value !== "bigint") throw new Error("ifc.graphql.bigint-required");
  const decimal = value.toString();
  parseIfcGraphqlInteger(scalar, decimal);
  return decimal;
}

/** 📍️ Converts a transport coordinate only when TypeScript can represent its exact index. */
export function ifcGraphqlIndex(value: unknown): number {
  const index = parseIfcGraphqlInteger("IfcIndex", value);
  if (index > BigInt(Number.MAX_SAFE_INTEGER)) throw new Error("ifc.graphql.index-not-addressable");
  return Number(index);
}
