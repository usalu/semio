/** 🧫️ BMP typed header, palette entries and canonical RGBA grid corpus. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { parseBmpSnapshot, type BmpSnapshot } from "../../🟦️.ts";
import { BMP_SQLITE_SCHEMA, bmpSnapshotToSqliteDatabase, bmpSnapshotFromSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input = fixture as BmpSnapshot;
test("BMP shared handwritten header, palette and RGBA grid expose independent editable SQL", async () => {
  expect(BMP_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  expect(parseBmpSnapshot(input)).toEqual(input);
  const database = await bmpSnapshotToSqliteDatabase(input);
  expect(await bmpSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT width,height,row_order,x_pixels_per_meter,y_pixels_per_meter FROM bmp_document").get()).toEqual({ width: 2, height: 2, row_order: "top_down", x_pixels_per_meter: -1200, y_pixels_per_meter: 3400 });
    expect(db.query("SELECT red,green,blue,alpha FROM bmp_pixel ORDER BY y,x").all()).toEqual(Array.from({ length: 4 }, (_, index) => ({ red: input.pixels[index*4], green: input.pixels[index*4+1], blue: input.pixels[index*4+2], alpha: input.pixels[index*4+3] })));
    expect(db.query("SELECT blue,green,red,reserved FROM bmp_palette_entry ORDER BY ordinal").all()).toEqual(input.palette.map(entry => ({ blue: entry.b, green: entry.g, red: entry.r, reserved: entry.reserved })));
    db.run("UPDATE bmp_pixel SET red=7,alpha=31 WHERE x=1 AND y=0");
    db.run("UPDATE bmp_palette_entry SET reserved=99 WHERE ordinal=0");
    const edited = await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.pixels.slice(4,8)).toEqual([7,255,255,31]);
    expect(edited.palette[0]!.reserved).toBe(99);
    expect(edited.rowOrder).toBe("topDown");
  } finally { db.close(); }
});

test("BMP independently edited grid completeness, coordinate uniqueness, channels and header widths reject", async () => {
  for (const edit of [
    "DELETE FROM bmp_pixel WHERE id=1",
    "UPDATE bmp_pixel SET x=99 WHERE id=1",
    "UPDATE bmp_pixel SET x=0,y=0 WHERE id=2",
    "UPDATE bmp_pixel SET document_id=999 WHERE id=1",
    "UPDATE bmp_pixel SET alpha=256 WHERE id=1",
    "UPDATE bmp_palette_entry SET ordinal=99 WHERE id=1",
    "UPDATE bmp_palette_entry SET blue=-1 WHERE id=1",
    "UPDATE bmp_document SET planes=65536",
    "UPDATE bmp_document SET x_pixels_per_meter=2147483648",
    "UPDATE bmp_document SET width=4294967295,height=4294967295",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await bmpSnapshotToSqliteDatabase(input)));
    try { db.run("PRAGMA ignore_check_constraints=ON"); db.run(edit); await expect(bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("BMP row order variants, empty grids and full unsigned metadata preserve typed snapshots", async () => {
  for (const rowOrder of ["topDown","bottomUp"] as const) expect(await bmpSnapshotFromSqliteDatabase(await bmpSnapshotToSqliteDatabase({ ...input, rowOrder }))).toEqual({ ...input, rowOrder });
  const empty = { ...input, width: 0, height: 0, palette: [], pixels: [], headerSize: 4294967295, imageSize: 4294967295, xPixelsPerMeter: -2147483648, planes: 65535 };
  expect(await bmpSnapshotFromSqliteDatabase(await bmpSnapshotToSqliteDatabase(empty))).toEqual(empty);
  const noncanonical = { ...input, schema: "typed.bmp.header108", headerSize: 108, bitsPerPixel: 32, palette: [], pixels: input.pixels.map((value, index) => index % 4 === 3 ? 17 : value) };
  const nativeFree = Database.deserialize(await exportSqliteDatabase(await bmpSnapshotToSqliteDatabase(noncanonical)));
  try {
    expect(nativeFree.query("SELECT schema,header_size,bits_per_pixel FROM bmp_document").get()).toEqual({ schema: noncanonical.schema, header_size: 108, bits_per_pixel: 32 });
    expect(await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(nativeFree.serialize())))).toEqual(noncanonical);
  } finally { nativeFree.close(); }
  for (const value of [{ ...input, pixels: [] },{ ...input, pixels: [...input.pixels.slice(0,-1),256] },{ ...input, planes: 65536 }]) await expect(bmpSnapshotToSqliteDatabase(value)).rejects.toThrow();
});

test("BMP bounds and cancellation precede grid reconstruction and entity allocation", async () => {
  const database = await bmpSnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 },{ maxValueBytes: 0 }]) {
    await expect(bmpSnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(bmpSnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
  const large = { ...input, width: 1000, height: 1, pixels: Array.from({ length: 4000 }, () => 255) };
  const controller = new AbortController();
  let events = 0;
  await expect(bmpSnapshotToSqliteDatabase(large, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  const restore = new AbortController();
  await expect(bmpSnapshotFromSqliteDatabase(database, { signal: restore.signal, onProgress: () => restore.abort() })).rejects.toMatchObject({ name: "AbortError" });
});
