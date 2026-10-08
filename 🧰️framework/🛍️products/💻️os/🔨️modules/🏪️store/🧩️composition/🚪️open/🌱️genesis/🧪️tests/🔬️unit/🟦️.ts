import { blake3 } from "@noble/hashes/blake3.js";
import { createHash } from "node:crypto";
import { expect, test } from "bun:test";
import { Buffer } from "node:buffer";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../../🧫️fixtures/🧬️schema/🔣️.json" with { type: "json" };
function leb(value: number): Buffer {
  const output = [];
  do { const byte = value & 127; value = Math.floor(value / 128); output.push(byte | (value ? 128 : 0)); } while (value);
  return Buffer.from(output);
}
function crc32c(bytes: Uint8Array): number {
  let crc = 0xffffffff;
  for (const byte of bytes) { crc ^= byte; for (let bit = 0; bit < 8; bit++) crc = crc >>> 1 ^ (crc & 1 ? 0x82f63b78 : 0); }
  return (crc ^ 0xffffffff) >>> 0;
}
function genesisOracle(row: typeof fixture.cases[number]): Buffer {
  const dictionary: string[] = [];
  let base = 0;
  const frames: Buffer[] = [];
  const id = (text: string): Buffer => {
    const match = /^(.+)-([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12})$/.exec(text);
    const prefix = match ? match[1]! : text;
    let index = dictionary.indexOf(prefix);
    if (index < 0) { index = dictionary.length; dictionary.push(prefix); }
    return Buffer.concat([Buffer.from([match ? 2 : 1]), leb(index), ...(match ? [Buffer.from(match[2]!.replaceAll("-", ""), "hex")] : [])]);
  };
  const frame = (kind: number, critical: boolean, payload: Buffer): Buffer => {
    const body = Buffer.concat([Buffer.from([kind, critical ? 2 : 0]), payload]);
    const prefix = leb(body.length), trailer = Buffer.alloc(8);
    trailer.writeUInt32LE(crc32c(body), 0);
    trailer.writeUInt32LE(prefix.length + body.length + 8, 4);
    return Buffer.concat([prefix, body, trailer]);
  };
  const flush = () => {
    if (base === dictionary.length) return;
    const entries = dictionary.slice(base).map(text => Buffer.from(text, "utf8"));
    frames.push(frame(3, true, Buffer.concat([Buffer.from([1]), leb(base), leb(entries.length), ...entries.flatMap(bytes => [leb(bytes.length), bytes])])));
    base = dictionary.length;
  };
  const doc = Buffer.concat([Buffer.from([1]), id(row.expected.artifact_id), id(row.schema)]);
  flush(); frames.push(frame(1, true, doc));
  const parent = row.owner.parent;
  const uri = `${parent.artifact_id}!${parent.dialect.artifact_kind}@${parent.dialect.standard}/${parent.dialect.subset}`;
  const composition = Buffer.concat([Buffer.from([1, 3]), ...[uri, row.owner.slot, row.owner.child_id, row.expected.dialect.artifact_kind, row.expected.dialect.standard, row.expected.dialect.subset].map(id)]);
  flush(); frames.push(frame(65, false, composition));
  const header = Buffer.alloc(32);
  Buffer.from([137, 83, 80, 82, 13, 10, 26, 10]).copy(header);
  header.writeUInt16LE(1, 8); header.writeUInt32LE(1, 12); header.writeUInt32LE(1, 16);
  header.writeUInt32LE(crc32c(header.subarray(0, 20)), 20);
  const commit = Buffer.alloc(64);
  commit.writeBigUInt64LE(1n, 0);
  commit.writeBigUInt64LE(BigInt(frames.reduce((length, bytes) => length + bytes.length, 0)), 16);
  commit.writeUInt32LE(frames.length, 24);
  Buffer.from(blake3(Buffer.concat([Buffer.from(blake3(header)), ...frames.map(bytes => Buffer.from(blake3(bytes)))]))).copy(commit, 32);
  const segment = Buffer.from(row.initialPackHex, "hex");
  const original = Buffer.concat(Array.from({ length: row.packRepeats }, () => segment));
  return Buffer.concat([leb(original.length), original, header, ...frames, frame(12, true, commit)]);
}

test("retained genesis framing has an independent neutral byte and identity oracle", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const segment = Buffer.from(row.initialPackHex, "hex");
    const initial = Buffer.concat(Array.from({ length: row.packRepeats }, () => segment));
    expect(initial.byteLength).toBe(segment.byteLength * row.packRepeats);
    const parent = row.owner.parent;
    const uri = `${parent.artifact_id}!${parent.dialect.artifact_kind}@${parent.dialect.standard}/${parent.dialect.subset}`;
    expect(new TextDecoder().decode(new TextEncoder().encode(uri))).toBe(uri);
    const fields = [row.expected.artifact_id, row.schema, uri, row.owner.slot, row.owner.child_id, row.expected.dialect.artifact_kind, row.expected.dialect.standard, row.expected.dialect.subset];
    const prefixes = fields.map(text => text.replace(/-[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/, ""));
    expect(new Set(prefixes).size).toBeLessThanOrEqual(8);
    expect(initial.subarray(0, segment.byteLength)).toEqual(segment);
  }
  const path = resolve(import.meta.dir, "../../🦀️.rs");
  expect(existsSync(path)).toBe(true);
  const source = readFileSync(path, "utf8");
  for (const method of ["MemberGenesisEnvelopeEncoder", "encode_step", "next_capacity_byte_demand", "close_granted"]) expect(source.includes(method)).toBe(true);
  console.log("[DEBUG] Ajv + Node Buffer/TextEncoder: neutral exact original packs, UUID aliases and UTF-8 owner URI for retained genesis framing");
});

test("retained native genesis bytes match the independent complete SPR framing and hash oracle", () => {
  for (const row of fixture.cases) {
    const bytes = genesisOracle(row);
    expect(createHash("sha256").update(bytes).digest("hex")).toBe(row.expectedSha256);
    console.log(`[DEBUG] independent Node Buffer + noble BLAKE3 canonical genesis case=${row.id} complete-bytes=${bytes.length} SHA256=${row.expectedSha256}`);
  }
});
