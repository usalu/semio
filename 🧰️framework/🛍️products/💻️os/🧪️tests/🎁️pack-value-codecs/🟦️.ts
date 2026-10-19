/** 🎁️ Pack value codecs: byte-exact pack values, dynamic integers and the scene pack codec.
 *
 * @see ../🛠️backbone-wire-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it } from "vitest";
import { clonePackValue, decodePackValue, decodeScenePackValue, encodePackValue, isPackByteVector, isPackInteger, packInt, packUInt, packValueToExactJson } from "@semio-tech/framework/packs";

type PackInteger = import("@semio-tech/framework/packs").PackInteger;

type PackValue = import("@semio-tech/framework/packs").PackValue;

describe("@semio-tech/framework-os PackValueCodec", () => {
  function bytesToHex(bytes: Uint8Array): string {
    return Array.from(bytes)
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
  }
  function hexToBytes(hex: string): Uint8Array {
    const out = new Uint8Array(hex.length / 2);
    for (let i = 0; i < out.length; i++) out[i] = Number.parseInt(hex.substring(i * 2, i * 2 + 2), 16);
    return out;
  }

  const packValueFixtures: ReadonlyArray<readonly [string, PackValue, string]> = [
    ["null", null, "0001011112"],
    ["bool_true", true, "0001011102"],
    ["bool_false", false, "0001011101"],
    ["int_zero", packUInt(0n), "000101110400"],
    ["int_negative_one", packInt(-1n), "000101110301"],
    ["float_pi", 3.14, "00010111051f85eb51b81e0940"],
    ["float_whole_number", 2.0, "00010111050000000000000040"],
    ["string_empty", "", "01000101110600"],
    ["string_escapes", 'hello\nworld with "quotes"', "011968656c6c6f0a776f726c642077697468202271756f746573220101110600"],
    ["array_empty", [], "000101110c00"],
    ["array_ints", [packUInt(1n), packUInt(2n), packUInt(3n)], "000101110c03040104020403"],
    ["object_empty", {}, "000101111000"],
    ["object_mixed", { a: packUInt(1n), b: [true, null] }, "00010111100207016104010701620c020212"],
    [
      "nested_deep",
      { a: { b: { c: [packUInt(1n), packUInt(2n), { d: "leaf" }] } } },
      "01046c6561660101111001070161100107016210010701630c030401040210010701640600",
    ],
  ];

  it.each(packValueFixtures)("decodes real Rust encode_wire_value bytes for %s", (_name, expected, hex) => {
    expect(decodePackValue(hexToBytes(hex))).toEqual(expected);
  });

  it.each(packValueFixtures)("encodes byte-exact against real Rust encode_wire_value output for %s", (_name, value, hex) => {
    expect(bytesToHex(encodePackValue(value))).toBe(hex);
  });

  it.each(packValueFixtures)("round-trips %s through encodePackValue/decodePackValue", (_name, value) => {
    expect(decodePackValue(encodePackValue(value))).toEqual(value);
  });
});

describe("@semio-tech/framework-os PackValueCodec dynamic integers", () => {
  function bytesToHex(bytes: Uint8Array): string {
    return Array.from(bytes)
      .map((byte) => byte.toString(16).padStart(2, "0"))
      .join("");
  }
  function hexToBytes(hex: string): Uint8Array {
    const out = new Uint8Array(hex.length / 2);
    for (let i = 0; i < out.length; i++) out[i] = Number.parseInt(hex.substring(i * 2, i * 2 + 2), 16);
    return out;
  }
  function corpusValue(node: Record<string, any>): PackValue {
    if (typeof node.uint === "string") return packUInt(BigInt(node.uint));
    if (typeof node.int === "string") return packInt(BigInt(node.int));
    if (typeof node.f64LeHex === "string") return new DataView(hexToBytes(node.f64LeHex).buffer).getFloat64(0, true);
    if (Array.isArray(node.list)) return node.list.map(corpusValue);
    if (Array.isArray(node.map)) return Object.fromEntries((node.map as [string, Record<string, any>][]).map(([key, value]) => [key, corpusValue(value)]));
    throw new Error(`unsupported corpus node ${JSON.stringify(node)}`);
  }

  it("covers every declared integer boundary example", async () => {
    const { default: corpus } = await import("../🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json");
    expect(corpus.accept).toHaveLength(6);
    expect(corpus.reject.map((row: { id: string }) => row.id)).toEqual(["truncated-u64", "u64-overflow", "nonminimal-u64", "nonminimal-zigzag-i64"]);
  });

  it("encodes and decodes every accepted corpus row byte-exactly with its exact carrier kind", async () => {
    const { default: corpus } = await import("../🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json");
    for (const row of corpus.accept) {
      const value = corpusValue(row.value);
      expect(bytesToHex(encodePackValue(value)), row.id).toBe(row.wireHex);
      const decoded = decodePackValue(hexToBytes(row.wireHex));
      expect(decoded, row.id).toEqual(value);
      if (row.variant === "uint" || row.variant === "int") {
        expect(isPackInteger(decoded)).toBe(true);
        expect((decoded as PackInteger).kind).toBe(row.variant);
        const magnitude = row.value[row.variant];
        if (typeof magnitude !== "string") throw new Error("integer corpus magnitude is missing");
        expect((decoded as PackInteger).value).toBe(BigInt(magnitude));
      }
      expect(bytesToHex(encodePackValue(decoded)), row.id).toBe(row.wireHex);
    }
  });

  it("rejects every corpus reject row at the documented boundary", async () => {
    const { default: corpus } = await import("../🧫️fixtures/🎒️pack-dynamic-integer-v1/🔣️.json");
    for (const row of corpus.reject) {
      const bytes = hexToBytes(row.wireHex);
      if (row.outcome === "decode-error") expect(() => decodePackValue(bytes), row.id).toThrow();
      else {
        const decoded = decodePackValue(bytes);
        if (!row.decodes) throw new Error("noncanonical corpus decoded value is missing");
        expect(decoded, row.id).toEqual(corpusValue(row.decodes));
        expect(bytesToHex(encodePackValue(decoded)), row.id).not.toBe(row.wireHex);
      }
    }
  });

  it("keeps int and uint distinct, survives nesting, and refuses unminted carriers", () => {
    expect(bytesToHex(encodePackValue(packInt(7n)))).not.toBe(bytesToHex(encodePackValue(packUInt(7n))));
    expect((decodePackValue(encodePackValue(packInt(7n))) as PackInteger).kind).toBe("int");
    const nested = { u: packUInt((1n << 64n) - 1n), rows: [packInt(-(2n ** 63n)), { deep: packUInt(2n ** 53n + 1n) }] };
    const round = decodePackValue(encodePackValue(nested)) as Record<string, PackValue>;
    expect(round).toEqual(nested);
    expect(bytesToHex(encodePackValue(round))).toBe(bytesToHex(encodePackValue(nested)));
    expect(isPackInteger({ kind: "uint", value: 1n })).toBe(false);
    expect(() => encodePackValue({ kind: "uint", value: 1n } as unknown as PackValue)).toThrow("unminted");
    expect(() => packUInt(-1n)).toThrow("u64");
    expect(() => packInt(2n ** 63n)).toThrow("i64");
    expect(() => packUInt(2n ** 64n)).toThrow("u64");
  });

  it("clones carriers structurally and projects only safe integers onto JSON", () => {
    const source = { id: packUInt(2n ** 53n + 1n), rows: [packInt(-7n)] };
    const cloned = clonePackValue(source) as typeof source;
    expect(cloned).toEqual(source);
    expect(isPackInteger(cloned.id)).toBe(true);
    expect(isPackInteger((structuredClone(source) as typeof source).id)).toBe(false);
    expect(packValueToExactJson({ safe: packUInt(9007199254740991n), signed: packInt(-7n), plain: 1.5 })).toEqual({ safe: 9007199254740991, signed: -7, plain: 1.5 });
    expect(() => packValueToExactJson({ big: packUInt(2n ** 53n + 1n) })).toThrow("exactly");
    expect(isPackByteVector([0, 255])).toBe(true);
    expect(isPackByteVector([packUInt(1n)])).toBe(false);
    expect(isPackByteVector([1.5])).toBe(false);
    expect(isPackByteVector([256])).toBe(false);
  });

  it("keeps a negative zero's sign bit, byte-for-byte with Rust normalize_f64", () => {
    expect(bytesToHex(encodePackValue(-0))).toBe("00010111050000000000000080");
    expect(Object.is(decodePackValue(hexToBytes("00010111050000000000000080")), -0)).toBe(true);
  });
});

describe("@semio-tech/framework-os ScenePackCodec", () => {
  it("decodes the byte-exact Rust TableScene fixture without a schema-specific field mirror", () => {
    const hex = "0d02060b636f6c756d6e734a736f6e060f5b7b226964223a226e616d65227d5d0608726f77734a736f6e06025b5d";
    const bytes = new Uint8Array(hex.length / 2);
    for (let index = 0; index < bytes.length; index += 1) bytes[index] = Number.parseInt(hex.slice(index * 2, index * 2 + 2), 16);
    expect(decodeScenePackValue(bytes)).toEqual({ columnsJson: '[{"id":"name"}]', rowsJson: "[]" });
  });
});
