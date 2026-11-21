import { Database } from "bun:sqlite";
import { createHash } from "node:crypto";
import { expect, test } from "bun:test";
import sharp from "sharp";
import fixture from "./receiving.json";

test("Raster wire image frames retain normalized RGBA bytes", async () => {
  const { width, height, frames, frameDelays } = fixture.asset;
  const protocol = await Bun.file(new URL("./📡️.protocol.semio", import.meta.url)).text();
  expect(protocol).toContain(`record add-layer-asset tag=${fixture.asset.tag}`);
  expect(frames.length).toBe(frameDelays.length);
  expect(Buffer.from(fixture.asset.icc).toString("base64")).toBe(fixture.asset.iccBase64);
  for (const [index, frame] of frames.entries()) {
    const bytes = Buffer.from(frame);
    expect(bytes.toString("base64")).toBe(fixture.asset.frameBase64[index]);
    const data = await sharp(bytes, { raw: { width, height, channels: 4 } }).png().toBuffer();
    const image = await sharp(data).raw().toBuffer({ resolveWithObject: true });
    expect(image.info.channels).toBe(4);
    expect([...image.data]).toEqual(frame);
  }
});

test("A paged Raster frame retains compact binary octets", async () => {
  const frame = fixture.pagedFrame;
  const pixels = Buffer.from(Array.from(
    { length: frame.width * frame.height * 4 },
    (_, index) => (index * frame.multiplier + frame.offset) % 256,
  ));
  const png = await sharp(pixels, { raw: { width: frame.width, height: frame.height, channels: 4 } }).png().toBuffer();
  const decoded = await sharp(png).ensureAlpha().raw().toBuffer();
  expect(decoded.byteLength).toBe(frame.byteLength);
  expect(createHash("sha256").update(decoded).digest("hex")).toBe(frame.sha256);
});

test("Raster wire selection expands to the same ordered coverage", () => {
  using database = new Database(":memory:");
  database.run("CREATE TABLE span (start INTEGER PRIMARY KEY, length INTEGER, coverage INTEGER)");
  for (const row of fixture.selection.spans) {
    database.run("INSERT INTO span VALUES (?, ?, ?)", [row.start, row.length, row.coverage]);
  }
  const coverage = database.query(`
    WITH RECURSIVE rank(value) AS (
      SELECT 0 UNION ALL SELECT value+1 FROM rank WHERE value+1 < ?
    )
    SELECT COALESCE(span.coverage, 0) FROM rank LEFT JOIN span
      ON value >= start AND value < start + length ORDER BY value
  `).values(fixture.selection.dense.length).flat();
  expect(coverage).toEqual(fixture.selection.dense);
  expect([...Buffer.from(fixture.samples.hex, "hex")]).toEqual(fixture.samples.bytes);
});
