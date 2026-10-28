import { describe, expect, test } from "../../../../🧪️test/🟦️.ts";
import { RuntimeCapabilityMissingV1 } from "../../../../🗣️messages/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with { type: "json" };
import { createNodeNativeSystemV1, nodeNativeResultV1, nodeNativeWordV1, type NodeNativeCoreV1 } from "../🟦️.ts";

const types = Object.keys(schema.definitions.Protocol.const.types);
const calls: { address: bigint; words: bigint[] }[] = [];
const memory = new Map<bigint, Uint8Array>();
const core: NodeNativeCoreV1 = {
  open: path => { if (path === "missing") throw new Error("dlopen failed"); return 7n; },
  symbol: (_handle, name) => { if (name === "absent") throw new Error("not found"); return BigInt(name.length) + 100n; },
  close: () => 0n,
  call: (address, ...words) => { calls.push({ address, words }); return 0xfffffffffffffff6n; },
  addressOf: view => 5000n + BigInt(view.byteOffset),
  peek: (address, length) => {
    const start = [...memory.keys()].find(key => key <= address && address < key + BigInt(memory.get(key)!.length)), bytes = new Uint8Array(Number(length));
    if (start !== undefined) bytes.set(memory.get(start)!.subarray(Number(address - start), Number(address - start) + Number(length)));
    return bytes.buffer as ArrayBuffer;
  },
  view: (address, length) => new ArrayBuffer(Number(length) + Number(address % 1n)),
};

describe("the Node native system over a core", () => {
  test("the type table is the contract's closed set and names every type to itself", () => {
    const system = createNodeNativeSystemV1(core);
    expect(Object.keys(system.FFIType).sort()).toEqual([...types].sort());
    for (const type of types) expect((system.FFIType as Record<string, string>)[type]).toBe(type);
  });

  test("return words are truncated and sign-extended per declared type like the Bun binding", () => {
    const rows: [string, bigint, unknown][] = [
      ["i8", 0x170n, 0x70], ["i8", 0x80n, -128], ["u8", 0x1ffn, 255], ["i16", 0x9c40n, -25536], ["u16", 0x19c40n, 40000],
      ["i32", 0xffffffffn, -1], ["u32", 0x1ffffffffn, 4294967295], ["i64", 0xfffffffffffffffbn, -5n], ["u64", 0xfffffffffffffffbn, 0xfffffffffffffffbn],
      ["bool", 0x100n, false], ["bool", 0x1n, true], ["ptr", 0n, null], ["ptr", 4096n, 4096], ["void", 5n, undefined],
    ];
    for (const [type, word, expected] of rows) expect(nodeNativeResultV1(core, type, word), `${type} ${word}`).toBe(expected);
  });

  test("argument words wrap negatives, booleans, nulls, fractions and bigints, and keep buffers alive", () => {
    const keep: unknown[] = [];
    const word = (type: string, value: unknown): bigint => nodeNativeWordV1(core, type, value, keep);
    expect(word("i32", -2)).toBe(0xfffffffffffffffen);
    expect(word("i32", true)).toBe(1n);
    expect(word("i32", null)).toBe(0n);
    expect(word("i32", -7.9)).toBe(BigInt.asUintN(64, -7n));
    expect(word("i64", -5n)).toBe(0xfffffffffffffffbn);
    expect(word("u64", -5)).toBe(0xfffffffffffffffbn);
    expect(word("bool", "x")).toBe(1n);
    expect(word("ptr", null)).toBe(0n);
    expect(word("ptr", 4096)).toBe(4096n);
    expect(word("ptr", 12n)).toBe(12n);
    expect(word("ptr", Buffer.alloc(4))).toBe(5000n + BigInt(Buffer.alloc(4).byteOffset));
    expect(word("cstring", "é")).toBeGreaterThan(0n);
    expect(keep.length).toBe(2);
    expect(() => word("i32", {})).toThrow("i32");
    expect(() => word("ptr", Symbol("x"))).toThrow("ptr");
    expect(() => word("f64", 1)).toThrow("f64");
    expect(() => word("nope", 1)).toThrow("nope");
  });

  test("a bound symbol passes one word per declared argument and decodes the return register", () => {
    calls.length = 0;
    const library = createNodeNativeSystemV1(core).dlopen("lib", { abs: { args: ["i32", "ptr"], returns: "i32" }, tiny: { args: [], returns: "u8" } });
    expect((library.symbols as any).abs(-1)).toBe(-10);
    expect((library.symbols as any).tiny()).toBe(246);
    expect(calls[0]).toEqual({ address: 103n, words: [0xffffffffffffffffn, 0n] });
    expect(calls[1]!.words).toEqual([]);
  });

  test("refusals: missing library, missing symbol, floats, too many arguments, closed library, callbacks, unbounded views", () => {
    const system = createNodeNativeSystemV1(core);
    expect(() => system.dlopen("missing", {})).toThrow("dlopen failed");
    expect(() => system.dlopen("lib", { absent: { args: [], returns: "i32" } })).toThrow("absent");
    expect(() => system.dlopen("lib", { x: { args: ["f64"], returns: "i32" } })).toThrow("f64");
    expect(() => system.dlopen("lib", { x: { args: [], returns: "f32" } })).toThrow("f32");
    expect(() => system.dlopen("lib", { x: { args: Array(9).fill("i32"), returns: "i32" } })).toThrow("x");
    const library = system.dlopen("lib", { x: { args: [], returns: "i32" } });
    library.close();
    library.close();
    expect(() => (library.symbols as any).x()).toThrow("lib");
    expect(() => new system.JSCallback(() => 1, { args: [], returns: "void" })).toThrow(RuntimeCapabilityMissingV1);
    expect(() => system.toArrayBuffer(1)).toThrow("toArrayBuffer");
  });

  test("every refusal message exists in English and German with the same placeholders", async () => {
    const messages = (await import("../../../../🧬️schema/🗣️messages.json", { with: { type: "json" } })).default as Record<string, { en: string; de: string }>;
    for (const id of ["nativeType", "nativeArgument", "nativeFloat", "nativeLength", "nativeByteLength", "nativeArity", "nativeClosed", "nativeSymbol", "reasonNativeCallback"]) {
      const row = messages[id]!;
      expect(row.en.length).toBeGreaterThan(10);
      expect(row.de.length).toBeGreaterThan(10);
      expect([...row.en.matchAll(/\{(\w+)\}/gu)].map(m => m[1]).sort()).toEqual([...row.de.matchAll(/\{(\w+)\}/gu)].map(m => m[1]).sort());
    }
  });

  test("typed reads decode little-endian memory and cstrings stop at NUL", () => {
    memory.set(900n, Uint8Array.from([0x80, 0xff, 1, 0x80, 0xff, 0x7f, 0xff, 0x7f, 1, 0, 0, 0, 0, 0, 0, 0x80]));
    const { read } = createNodeNativeSystemV1(core);
    expect(read.i8(900)).toBe(-128);
    expect(read.u16(900, 2)).toBe(32769);
    expect(read.i32(900, 4)).toBe(2147450879);
    expect(read.i64(900, 8)).toBe(-9223372036854775807n);
    expect(read.u64(900, 8)).toBe(9223372036854775809n);
    expect(read.ptr(900, 8)).toBe(9223372036854775808);
    memory.set(2000n, Uint8Array.from([0x68, 0xc3, 0xa9, 0]));
    expect(nodeNativeResultV1(core, "cstring", 2000n)).toBe("hé");
    expect(nodeNativeResultV1(core, "cstring", 0n)).toBe(null);
  });
});
