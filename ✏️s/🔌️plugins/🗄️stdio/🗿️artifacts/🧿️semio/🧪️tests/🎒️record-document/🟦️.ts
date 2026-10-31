// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🧪️ Language-agnostic laws of the shared record-document codec, TypeScript side. `🐍️.py` beside this file runs the same laws over the same `🧫️fixtures/🔣️.json`.
 *
 * The Rust record pack of `stdio.semio` is the oracle: every committed `*.pack.semio` it wrote must decode to the JSON snapshot it came from and re-encode to the same
 * bytes, and the primitives must agree with committed vectors and with third-party implementations — `@noble/hashes` for BLAKE3 and the system zlib for deflate.
 * @see ./🧫️fixtures/🔣️.json
 */

import { blake3 as noble } from "@noble/hashes/blake3.js";
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { deflateRawSync, inflateRawSync } from "node:zlib";
import { blake3, buildPack, crc32c, decodePack, decodeRecord, decodeSegment, deflate, DslFloat, encodePack, encodeRecord, encodeSegment, inflate, loadRecordSpec, RecordSpec, Reader, writeVarint, writeZigzag, type DslValue, type RecordFields, type RecordSpecDocument } from "../../🚪️io/💾️binary/🎒️record-document/🟦️.ts";

type Fixture = {
  schema: string;
  packCount: number;
  crc32c: { hex: string; crc: number }[];
  blake3: { size: number; hash: string }[];
  deflate: { source: string; rawHex: string; deflatedHex: string }[];
  subsets: { folder: string; spec: string; envelope: string; packs: string[]; snapshots?: { pack: string; json: string }[] }[];
};

const here = new URL(".", import.meta.url);
const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", here), "utf8")) as Fixture;
const root = new URL("../../", here);
const hex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
const fromHex = (text: string): Uint8Array => Uint8Array.from({ length: text.length / 2 }, (_, index) => Number.parseInt(text.slice(index * 2, index * 2 + 2), 16));
const pattern = (size: number): Uint8Array => Uint8Array.from({ length: size }, (_, index) => (index * 31 + 7) & 0xff);
const read = (path: string): Uint8Array => new Uint8Array(readFileSync(new URL(path, root)));
const specOf = (subset: Fixture["subsets"][number]) => loadRecordSpec(JSON.parse(readFileSync(new URL(subset.spec, root), "utf8")) as RecordSpecDocument);

test("the fixture declares its schema", () => {
  expect(fixture.schema).toBe("semio.stdio.record-document-laws/v1");
});

test("crc32c matches the published vectors", () => {
  for (const row of fixture.crc32c) expect(crc32c(fromHex(row.hex))).toBe(row.crc);
});

test("blake3 matches the committed vectors and @noble/hashes across chunk and tree boundaries", () => {
  for (const row of fixture.blake3) {
    expect(hex(blake3(pattern(row.size)))).toBe(row.hash);
    expect(hex(noble(pattern(row.size)))).toBe(row.hash);
  }
});

test("varints round trip and stay canonical", () => {
  for (const value of [0n, 1n, 127n, 128n, 300n, 16383n, 16384n, 2n ** 32n, 2n ** 63n - 1n, 2n ** 64n - 1n]) expect(new Reader(writeVarint(value)).varintBig()).toBe(value);
  for (const value of [0n, 1n, -1n, 63n, -64n, 2n ** 62n, -(2n ** 63n)]) expect(new Reader(writeZigzag(value)).zigzagBig()).toBe(value);
  expect([...writeZigzag(-1)]).toEqual([1]);
  expect([...writeVarint(300)]).toEqual([0xac, 0x02]);
});

test("a varint wider than 64 bits is refused like the Rust reader refuses it", () => {
  const wide = Uint8Array.of(0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02);
  expect(() => new Reader(wide).varintBig()).toThrow(RangeError);
  expect(() => new Reader(wide).varint()).toThrow(RangeError);
  expect(new Reader(Uint8Array.of(0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01)).varintBig()).toBe(2n ** 64n - 1n);
});

test("deflate reproduces every segment the Rust writer produced", () => {
  for (const row of fixture.deflate) expect(hex(deflate(fromHex(row.rawHex)))).toBe(row.deflatedHex);
});

test("deflate output is read back by the system zlib, which also reads the Rust bytes", () => {
  for (const size of [0, 1, 3, 258, 259, 1000, 70000]) {
    const raw = Uint8Array.from([...pattern(size), ...new Uint8Array(Math.floor(size / 2))]);
    expect(new Uint8Array(inflateRawSync(deflate(raw)))).toEqual(raw);
  }
  for (const row of fixture.deflate) expect(hex(new Uint8Array(inflateRawSync(fromHex(row.deflatedHex))))).toBe(row.rawHex);
});

test("a corrupted segment is refused by its crc", () => {
  const frame = encodeSegment(3, Uint8Array.of(1, 1, 97));
  expect([...decodeSegment(new Reader(frame)).payload]).toEqual([1, 1, 97]);
  const damaged = Uint8Array.from(frame);
  damaged[3] = damaged[3]! ^ 1;
  expect(() => decodeSegment(new Reader(damaged))).toThrow();
});

test("every exported record spec hashes to the hash it carries", () => {
  for (const subset of fixture.subsets) expect(specOf(subset).envelope).toBe(subset.envelope);
});

test("every committed Rust pack decodes and re-encodes to its own bytes", () => {
  let count = 0;
  for (const subset of fixture.subsets) {
    const spec = specOf(subset);
    for (const pack of subset.packs) {
      const data = read(pack);
      expect(encodePack(spec, decodePack(spec, data))).toEqual(data);
      expect(encodePack(spec, decodePack(spec, data, { omitNulls: true }))).toEqual(data);
      count += 1;
    }
  }
  expect(count).toBe(fixture.packCount);
});

test("decoded packs equal the snapshot json the Rust derive wrote", () => {
  let pairs = 0;
  for (const subset of fixture.subsets) {
    const spec = specOf(subset);
    for (const pair of subset.snapshots ?? []) {
      const expected = JSON.parse(new TextDecoder().decode(read(pair.json))) as Record<string, unknown>;
      expect(decodePack(spec, read(pair.pack), { narrowF32: true })).toEqual(expected);
      expect(encodePack(spec, expected)).toEqual(read(pair.pack));
      pairs += 1;
    }
  }
  expect(pairs).toBeGreaterThanOrEqual(1);
});

test("a pack of another subset is refused", () => {
  const [first, second] = fixture.subsets;
  expect(() => decodePack(specOf(first!), read(second!.packs[0]!))).toThrow();
});

test("a flipped body byte is refused", () => {
  const subset = fixture.subsets[0]!;
  const data = Uint8Array.from(read(subset.packs[0]!));
  data[Math.floor(data.length / 2)] = data[Math.floor(data.length / 2)]! ^ 0x40;
  expect(() => decodePack(specOf(subset), data)).toThrow();
});

test("a document that names an unknown key or variant is refused", () => {
  const subset = fixture.subsets.find((row) => row.folder.endsWith("model"))!;
  const spec = specOf(subset);
  const document = decodePack(spec, read(subset.packs[0]!)) as { spatial: Record<string, unknown>[] };
  expect(() => encodePack(spec, { ...document, unknown: 1 })).toThrow();
  expect(() => encodePack(spec, { ...document, spatial: [{ ...document.spatial[0], kind: "galaxy" }] })).toThrow();
});

const hostileGraph = { schema: "semio.pack.record-spec/v1", envelope: "hostile", schemaHash: "", graph: [[{ id: 1, key: "v", optional: false, flatten: false, shape: { kind: "value" } }, { id: 2, key: "rows", optional: true, flatten: false, shape: { kind: "table", record: 1 } }], [{ id: 1, key: "n", optional: false, flatten: false, shape: { kind: "uint" } }]] } as RecordSpecDocument;
const hostileSpec = loadRecordSpec({ ...hostileGraph, schemaHash: hex(new RecordSpec(hostileGraph).schemaHash()) });
const hostilePack = (body: Uint8Array): Uint8Array => buildPack(hostileSpec.schemaHash(), Uint8Array.of(0), body, []);
const dslRecord = (value: DslValue): RecordFields => new Map([[1, { kind: "value", value }]]);
const hostileValueHex = "8953504b0d0a1a0a010000000100000001000000ef65ef880000000000000000030303016300001d75fe6e04034b62636414146067e78c8f2f28ca2fc98f8f676164674ce461646162674eca4c67f90f05f5ec2ce59929a970fe7f4676e69cfc7266243e6b716e624e0e0b2b3b6b5a4e7e62092b03187cb007003a8adaca0103302e6348addadbcc79f048cc5ecb99c92f2dfe2c8f5ec35bbc5480cd7a99d49d951b9fae38a51dcca8c0cd0004498c0c0c00d1e7c4d10000007aa3646053504b464f4f543101000000010000007e0000000000000038000000000000001101000000000000a5fd061bc8a594b70e83de0ccdff6158fac82066ef6db399f4ff289b53d97e020000000000000000b66f0ae9";

test("an inflated segment is capped at the size it declares, whatever the deflate stream expands to", () => {
  const bomb = new Uint8Array(deflateRawSync(new Uint8Array(1 << 24)));
  expect(bomb.length).toBeLessThan(1 << 15);
  expect(() => inflate(bomb, 16)).toThrow(RangeError);
  expect(() => inflate(bomb, 2 ** 40)).toThrow(RangeError);
  expect(inflate(bomb, 1 << 24).length).toBe(1 << 24);
});

test("a table that declares more rows than the bytes can hold is refused before any row is allocated", () => {
  const body = concat3(Uint8Array.of(1, 2, 0x14), writeVarint(2 ** 40), writeVarint(0));
  expect(() => decodeRecord(hostileSpec, hostilePack(body))).toThrow(RangeError);
  const lists = concat3(Uint8Array.of(1, 1, 0x11, 0x0c), writeVarint(2 ** 40));
  expect(() => decodeRecord(hostileSpec, hostilePack(lists))).toThrow(RangeError);
});

test("a chunk id outside the pack is a RangeError", () => {
  const body = concat3(Uint8Array.of(1, 1, 0x09, 1), writeVarint(7));
  expect(() => decodeRecord(hostileSpec, hostilePack(body))).toThrow(RangeError);
});

test("dynamic values keep a __proto__ key and every 64-bit integer exact, byte-equal to the Python codec", () => {
  const value = JSON.parse('{"__proto__":1,"a":[2]}') as Record<string, DslValue>;
  value.big = 2n ** 63n - 1n;
  value.wide = 2n ** 64n - 1n;
  value.low = -(2n ** 63n);
  value.small = 5;
  value.float = new DslFloat(1);
  const inner = encodeRecord(hostileSpec, dslRecord(value));
  expect(hex(inner)).toBe(hostileValueHex);
  const back = decodeRecord(hostileSpec, inner).get(1)!.value as Record<string, DslValue>;
  expect(Object.getPrototypeOf(back)).toBeNull();
  expect(Object.keys(back)).toEqual(["__proto__", "a", "big", "wide", "low", "small", "float"]);
  expect(Object.getOwnPropertyDescriptor(back, "__proto__")!.value).toBe(1);
  expect([back.big, back.wide, back.low, back.small]).toEqual([2n ** 63n - 1n, 2n ** 64n - 1n, -(2n ** 63n), 5]);
  expect(hex(encodeRecord(hostileSpec, decodeRecord(hostileSpec, inner)))).toBe(hostileValueHex);
});

function concat3(...parts: Uint8Array[]): Uint8Array {
  return Uint8Array.from(parts.flatMap((part) => [...part]));
}
