import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";

const RETAINED = new Set(["data.drop"]);
const MOVED = new Set(["memory.fill", "memory.copy", "memory.init"]);
const STACK: Record<string, number> = {
  "i32.const": 1, "i64.const": 1, "local.get": 1, "global.get": 1, "memory.size": 1,
  "i32.add": -1, "i32.mul": -1, "local.set": -1, "global.set": -1, "drop": -1, "if": -1, "br_if": -1, "br_table": -1,
  "select": -2, "i32.store": -2, "i64.store": -2, "i32.store8": -2, "memory.fill": -3, "memory.copy": -3, "memory.init": -3,
};
const SIMPLE: Record<number, string> = {
  0x01: "nop", 0x0b: "end", 0x0f: "return", 0x1a: "drop", 0x1b: "select", 0x6a: "i32.add", 0x6c: "i32.mul", 0xa7: "i32.wrap_i64",
};
const LEB: Record<number, string> = {
  0x0c: "br", 0x0d: "br_if", 0x20: "local.get", 0x21: "local.set", 0x22: "local.tee", 0x23: "global.get", 0x24: "global.set",
  0x41: "i32.const", 0x42: "i64.const",
};
const MEMARG: Record<number, string> = {
  0x28: "i32.load", 0x29: "i64.load", 0x2d: "i32.load8_u", 0x36: "i32.store", 0x37: "i64.store", 0x3a: "i32.store8",
};

/** 🔬️ Decodes a LEB128 value and returns the cursor after it. */
function leb(bytes: Uint8Array, at: number): [number, number] {
  let value = 0n, shift = 0n;
  for (;;) {
    const byte = bytes[at++];
    value |= BigInt(byte & 127) << shift;
    shift += 7n;
    if (!(byte & 128)) return [Number(value & 0xffffffffn), at];
  }
}

/** 🔬️ Lists the instruction names of the single function body by an independent opcode decoder. */
function decode(bytes: Uint8Array): string[] {
  let at = 8;
  while (bytes[at] !== 10) {
    const [size, next] = leb(bytes, at + 1);
    at = next + size;
  }
  let [, cursor] = leb(bytes, at + 1);
  [, cursor] = leb(bytes, cursor);
  const [size, bodyStart] = leb(bytes, cursor);
  const end = bodyStart + size;
  let [groups, position] = leb(bytes, bodyStart);
  for (; groups > 0; groups--) [, position] = leb(bytes, position), position++;
  const names: string[] = [];
  while (position < end) {
    const code = bytes[position++];
    if (SIMPLE[code]) names.push(SIMPLE[code]);
    else if (LEB[code]) { names.push(LEB[code]); [, position] = leb(bytes, position); }
    else if (MEMARG[code]) { names.push(MEMARG[code]); [, position] = leb(bytes, position); [, position] = leb(bytes, position); }
    else if (code === 0x04) { names.push("if"); position++; }
    else if (code === 0x0e) {
      names.push("br_table");
      let count: number;
      [count, position] = leb(bytes, position);
      for (let index = 0; index <= count; index++) [, position] = leb(bytes, position);
    } else if (code === 0x3f || code === 0x40) { names.push(code === 0x3f ? "memory.size" : "memory.grow"); position++; }
    else if (code === 0xfc) {
      const [sub, next] = leb(bytes, position);
      position = next;
      if (sub === 8) { names.push("memory.init"); [, position] = leb(bytes, position); position++; }
      else if (sub === 9) { names.push("data.drop"); [, position] = leb(bytes, position); }
      else if (sub === 10) { names.push("memory.copy"); position += 2; }
      else if (sub === 11) { names.push("memory.fill"); position++; }
      else throw new Error(`unknown bulk opcode ${sub}`);
    } else throw new Error(`unknown opcode ${code}`);
  }
  return names;
}

/** 📏️ The instruction vectors agree with an independent decoder, JavaScript WebAssembly and SQL arithmetic on retained items, copy bytes and executed instructions. */
test("metering vectors charge retained items only for real copies, allocations and releases", async () => {
  const fixture = await Bun.file(new URL("🧫️fixtures/🔣️.json", import.meta.url)).json();
  const schema = await Bun.file(new URL("../🧬️schema/🔣️.json", import.meta.url)).json();
  expect(new Ajv({strict: true}).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.calls) {
    const instance = new WebAssembly.Instance(new WebAssembly.Module(Uint8Array.fromHex(row.moduleHex)));
    expect((instance.exports.run as () => number)()).toBe(row.expected);
    expect(row.expected).toBe(row.count);
  }
  for (const row of fixture.cases) {
    const bytes = Uint8Array.fromHex(row.moduleHex);
    const instance = new WebAssembly.Instance(new WebAssembly.Module(bytes));
    expect((instance.exports.run as () => number)()).toBe(row.expected);
    const names = decode(bytes);
    expect(names).toEqual(row.static);
    const remaining = [...names];
    for (const kind of row.kinds) {
      const index = remaining.indexOf(kind);
      expect(index).toBeGreaterThanOrEqual(0);
      remaining.splice(index, 1);
    }
    expect(row.instructions).toBe(row.kinds.length);
    let depth = 0, peak = 0;
    for (const kind of row.kinds) { depth += STACK[kind] ?? 0; peak = Math.max(peak, depth); }
    expect(row.stackPeak).toBe(peak);
    for (const effect of row.retained) {
      expect(RETAINED.has(effect.kind)).toBe(true);
      expect(row.kinds).toContain(effect.kind);
    }
    for (const move of row.moved) {
      expect(MOVED.has(move.kind)).toBe(true);
      expect(row.kinds).toContain(move.kind);
    }
    const db = new Database(":memory:");
    try {
      db.run("CREATE TABLE retained (kind TEXT, bytes INTEGER)");
      for (const effect of row.retained) db.run("INSERT INTO retained VALUES (?, ?)", [effect.kind, effect.bytes]);
      db.run("CREATE TABLE moved (kind TEXT, bytes INTEGER)");
      for (const move of row.moved) db.run("INSERT INTO moved VALUES (?, ?)", [move.kind, move.bytes]);
      expect(db.query("SELECT coalesce(sum(max(1, (bytes + ?1 - 1) / ?1)), 0) AS workUnits FROM moved").get(fixture.workBytesPerUnit)).toEqual({workUnits: row.workUnits});
      expect(db.query("SELECT count(*) AS items, coalesce(sum(bytes), 0) AS copyBytes FROM retained").get()).toEqual({items: row.items, copyBytes: row.copyBytes});
    } finally {
      db.close();
    }
  }
});
