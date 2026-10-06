/** 📷️ Independent PNG and zlib laws for the exact SQLite carrier. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { PNG } from "pngjs";
import { inflateSync } from "node:zlib";
import corpus from "../../🧫️fixtures/🚦️audit/🔣️.json";
import type { PngSnapshot } from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { pngSnapshotFromSqliteDatabase, pngSnapshotToSqliteDatabase } from "../../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const hex = (value: string): Buffer => Buffer.from(value, "hex");
function crc(bytes: Uint8Array): number {
  let value = 0xffffffff;
  for (const byte of bytes) {
    value ^= byte;
    for (let bit = 0; bit < 8; bit++) value = (value >>> 1) ^ ((value & 1) ? 0xedb88320 : 0);
  }
  return (~value) >>> 0;
}
function png(input: { chunks: readonly { tag: string; dataHex: string }[]; corruptCrc?: number }): Buffer {
  const chunks = input.chunks.map((chunk, index) => {
    const data = hex(chunk.dataHex);
    const tag = Buffer.from(chunk.tag, "ascii");
    const frame = Buffer.alloc(data.length + 12);
    frame.writeUInt32BE(data.length, 0);
    tag.copy(frame, 4);
    data.copy(frame, 8);
    frame.writeUInt32BE((crc(Buffer.concat([tag, data])) ^ (input.corruptCrc === index ? 1 : 0)) >>> 0, data.length + 8);
    return frame;
  });
  return Buffer.concat([Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]), ...chunks]);
}

test("PNG neutral profiles keep exact source bytes through SQLite and reopen independently", async () => {
  for (const item of corpus.validPng) {
    const source = png(item);
    const independent = PNG.sync.read(source);
    expect([...independent.data]).toEqual(item.pixels);
    const snapshot: PngSnapshot = { schema: "stdio.png", bytes: [...source] };
    const restored = await pngSnapshotFromSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot));
    expect(restored).toEqual(snapshot);
    expect([...PNG.sync.read(Buffer.from(restored.bytes)).data]).toEqual(item.pixels);
    const database = Database.deserialize(await exportSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot)));
    try {
      expect(database.query("SELECT role FROM png_document WHERE id=1").get()).toEqual({role:"native"});
      expect(database.query("SELECT kind FROM png_chunk ORDER BY ordinal").all().map((row:any)=>row.kind)).toEqual(item.chunks.map(chunk=>chunk.tag));
      expect((await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(database.serialize()))).bytes).toEqual([...source]);
    } finally {
      database.close();
    }
  }
});

test("PNG Latin-1, compressed text and international text remain exact source chunks", async () => {
  const item = corpus.validPng.find(value => value.id === "latin1-and-itext")!;
  const source = png(item);
  expect([...PNG.sync.read(source).data]).toEqual(item.pixels);
  const restored = await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await pngSnapshotToSqliteDatabase({ schema: "stdio.png", bytes: [...source] }))));
  expect(restored.bytes).toEqual([...source]);
  for (const chunk of item.chunks.filter(value => ["tEXt", "zTXt", "iTXt"].includes(value.tag))) expect(Buffer.from(restored.bytes).includes(hex(chunk.dataHex))).toBe(true);
});

test("PNG zlib corpus independently enforces CINFO distance and dictionary identity", () => {
  const zlib = corpus.zlib;
  expect(inflateSync(hex(zlib.valid32KiBHex)).toString("hex")).toBe(zlib.expectedPayloadHex);
  expect(inflateSync(hex(zlib.valid256Hex)).toString("hex")).toBe(zlib.smallPayloadHex);
  expect(() => inflateSync(hex(zlib.advertised256InvalidDistanceHex))).toThrow(/distance/i);
  expect(() => inflateSync(hex(zlib.dictionaryStreamHex))).toThrow(/dictionary/i);
  expect(inflateSync(hex(zlib.dictionaryStreamHex), { dictionary: hex(zlib.dictionaryHex) }).toString("hex")).toBe(zlib.dictionaryPayloadHex);
  expect(() => inflateSync(hex(zlib.wrongDictionaryIdHex), { dictionary: hex(zlib.dictionaryHex) })).toThrow(/dictionary/i);
});

test("PNG malformed CRC, critical chunk and ordering corpus is independently refused", () => {
  for (const id of ["crc-mismatch", "unknown-critical", "early-iend", "ihdr-not-first"]) {
    const item = corpus.invalidPng.find(value => value.id === id)!;
    expect(() => PNG.sync.read(png(item))).toThrow();
  }
});
