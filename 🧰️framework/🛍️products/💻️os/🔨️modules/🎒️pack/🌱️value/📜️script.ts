import { readFileSync } from "node:fs";
import { join } from "node:path";

/** 🧮️ Checks the literal strict-wire corpus with an independent BigInt/UTF-8 accounting oracle. */
export async function proveWireValueMaterializationFixture(repoRoot: string): Promise<void> {
  const assert = (await import("node:assert/strict")).default;
  const Ajv = (await import("ajv")).default;
  const equal = (await import("fast-deep-equal")).default;
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value");
  const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/🧮️wire-materialization/🔣️.json"), "utf8"));
  const contract = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(contract);
  const validate = ajv.getSchema(`${contract.$id}#/$defs/WireValueMaterialization`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  assert.deepEqual(fixture.cases.map((row: { id: string }) => row.id), contract.$defs.WireValueMaterializationCase.properties.id.enum);
  assert.deepEqual(fixture.bridge, { fieldId: 1, rootTag: "11", exact: true });
  const { decodePackValue, packValueToExactJson } = await import("../../../🟦️.ts");
  for (const row of fixture.cases) {
    const bytes = Buffer.from(row.rawHex, "hex");
    const limits = row.limits;
    let offset = 0, used = 0n;
    const refuse = (outcome: "limit" | "malformed"): never => { throw { outcome }; };
    const charge = (amount: bigint): void => {
      used += amount;
      if (used > BigInt(limits.maxTotalAlloc) || used > 18_446_744_073_709_551_615n) refuse("limit");
    };
    const byte = (): number => {
      if (offset >= bytes.length) return refuse("malformed");
      return bytes[offset++];
    };
    const integer = (): bigint => {
      let value = 0n;
      for (let index = 0; index < 10; index++) {
        const next = byte();
        if (index === 9 && (next & 0xfe) !== 0) return refuse("malformed");
        value |= BigInt(next & 0x7f) << BigInt(index * 7);
        if ((next & 0x80) === 0) return value;
      }
      return refuse("malformed");
    };
    const text = (): string => {
      const length = integer();
      if (length > BigInt(limits.maxSegmentLen)) return refuse("limit");
      if (length > BigInt(bytes.length - offset)) return refuse("malformed");
      charge(length);
      const start = offset;
      offset += Number(length);
      try { return new TextDecoder("utf-8", { fatal: true }).decode(bytes.subarray(start, offset)); }
      catch { return refuse("malformed"); }
    };
    const symbols: string[] = [];
    const string = (tag: number): string => {
      if (tag === 7) return text();
      if (tag !== 6) return refuse("malformed");
      const index = integer();
      if (index >= BigInt(symbols.length)) return refuse("malformed");
      const value = symbols[Number(index)];
      charge(BigInt(Buffer.byteLength(value, "utf8")));
      return value;
    };
    const value = (depth: number): unknown => {
      if (depth > limits.maxDepth) return refuse("limit");
      const tag = byte();
      if (tag === 0x12) return null;
      if (tag === 1 || tag === 2) return tag === 2;
      if (tag === 3) { const raw = integer(); return Number((raw >> 1n) ^ -(raw & 1n)); }
      if (tag === 4) return Number(integer());
      if (tag === 5) {
        if (bytes.length - offset < 8) return refuse("malformed");
        const number = new DataView(bytes.buffer, bytes.byteOffset + offset, 8).getFloat64(0, true);
        offset += 8;
        return number;
      }
      if (tag === 6 || tag === 7) return string(tag);
      if (tag !== 0x0c && tag !== 0x10) return refuse("malformed");
      const count = integer();
      if (count > BigInt(limits.maxItems)) return refuse("limit");
      charge(count * 64n);
      const values: unknown[] = [], entries: [string, unknown][] = [];
      for (let index = 0n; index < count; index++) {
        if (tag === 0x0c) values.push(value(depth + 1));
        else entries.push([string(byte()), value(depth + 1)]);
      }
      return tag === 0x0c ? values : Object.fromEntries(entries);
    };
    let actual: { outcome: string; value?: unknown };
    try {
      if (bytes.length > limits.maxFileLen) refuse("limit");
      const count = integer();
      if (count > BigInt(limits.maxSymbols)) refuse("limit");
      charge(count * 32n);
      for (let index = 0n; index < count; index++) symbols.push(text());
      if (integer() !== 1n || integer() !== BigInt(fixture.bridge.fieldId) || byte() !== 0x11) refuse("malformed");
      const decoded = value(0);
      if (offset !== bytes.length) refuse("malformed");
      actual = { outcome: "accepted", value: decoded };
    } catch (error) {
      if (!error || typeof error !== "object" || !("outcome" in error)) throw error;
      actual = error as { outcome: string };
    }
    assert.equal(actual.outcome, row.expect.outcome, row.id);
    if (actual.outcome === "accepted") {
      assert(equal(actual.value, row.expect.value), row.id + " independent JSON output");
      assert(equal(packValueToExactJson(decodePackValue(bytes)), row.expect.value), row.id + " first-party Pack output");
      assert.deepEqual(JSON.parse(JSON.stringify(actual.value)), row.expect.value, row.id + " JSON oracle");
    }
  }
  const owner = readFileSync(join(root, "🦀️.rs"), "utf8");
  const store = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"), "utf8");
  for (const marker of ["pub fn decode_value_record_body_exact(", "wire value materialization exceeds max_total_alloc", "fn decode_inline_symbols(", "fn value_slots<T>(", "copy_decoded_string(value, ctx.materialization.as_ref())"]) assert(owner.includes(marker), marker);
  assert(store.includes("decode_wire_value_with_options(bytes, &PackDecodeOptions::default())"));
  console.log(`[DEBUG] wire-value materialization: AJV=1 BigInt-accounting=1 JSON+deep-equal+Pack=1 cases=${fixture.cases.length}; native decoder behavior remains a separate exact law`);
}
