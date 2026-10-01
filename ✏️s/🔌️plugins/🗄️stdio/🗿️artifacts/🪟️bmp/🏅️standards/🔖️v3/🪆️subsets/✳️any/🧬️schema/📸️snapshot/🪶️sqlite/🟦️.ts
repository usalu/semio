/** 🪟️ BMP typed header, ordered palette and canonical RGBA pixel entities. */
import type { BmpSnapshot } from "../🟦️.ts";
import { artifactSqliteCheckpoint, artifactSqliteDatabase, artifactSqliteDocument, artifactSqliteDocumentReference, artifactSqliteInteger, artifactSqliteOrderedRows, artifactSqliteTables, artifactSqliteText, artifactSqliteTextBytes, artifactSqliteValueBudget, type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase, SqliteRow, SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

/** 🏛️ Handcrafted schema byte-equal to the adjacent SQL asset. */
export const BMP_SQLITE_SCHEMA = "CREATE TABLE bmp_document (\n  id INTEGER PRIMARY KEY CHECK (id = 1),\n  schema TEXT NOT NULL,\n  header_size INTEGER NOT NULL CHECK (header_size BETWEEN 0 AND 4294967295),\n  width INTEGER NOT NULL CHECK (width BETWEEN 0 AND 4294967295),\n  height INTEGER NOT NULL CHECK (height BETWEEN 0 AND 4294967295),\n  row_order TEXT NOT NULL CHECK (row_order IN ('bottom_up', 'top_down')),\n  planes INTEGER NOT NULL CHECK (planes BETWEEN 0 AND 65535),\n  bits_per_pixel INTEGER NOT NULL CHECK (bits_per_pixel BETWEEN 0 AND 65535),\n  compression INTEGER NOT NULL CHECK (compression BETWEEN 0 AND 4294967295),\n  image_size INTEGER NOT NULL CHECK (image_size BETWEEN 0 AND 4294967295),\n  x_pixels_per_meter INTEGER NOT NULL CHECK (x_pixels_per_meter BETWEEN -2147483648 AND 2147483647),\n  y_pixels_per_meter INTEGER NOT NULL CHECK (y_pixels_per_meter BETWEEN -2147483648 AND 2147483647),\n  colors_used INTEGER NOT NULL CHECK (colors_used BETWEEN 0 AND 4294967295),\n  colors_important INTEGER NOT NULL CHECK (colors_important BETWEEN 0 AND 4294967295)\n);\nCREATE TABLE bmp_palette_entry (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n  ordinal INTEGER NOT NULL CHECK (ordinal >= 0),\n  blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255),\n  green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),\n  red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255),\n  reserved INTEGER NOT NULL CHECK (reserved BETWEEN 0 AND 255)\n);\nCREATE TABLE bmp_pixel (\n  id INTEGER PRIMARY KEY,\n  document_id INTEGER NOT NULL REFERENCES bmp_document(id),\n  x INTEGER NOT NULL CHECK (x >= 0),\n  y INTEGER NOT NULL CHECK (y >= 0),\n  red INTEGER NOT NULL CHECK (red BETWEEN 0 AND 255),\n  green INTEGER NOT NULL CHECK (green BETWEEN 0 AND 255),\n  blue INTEGER NOT NULL CHECK (blue BETWEEN 0 AND 255),\n  alpha INTEGER NOT NULL CHECK (alpha BETWEEN 0 AND 255)\n);\n";
function integer(value: number, minimum = 0, maximum = 4294967295): bigint {
  if (!Number.isSafeInteger(value) || value < minimum || value > maximum) throw new Error("BMP integer is outside its declared width");
  return BigInt(value);
}
function read(row: SqliteRow, column: number, minimum = 0, maximum = 4294967295): number {
  const value = artifactSqliteInteger(row, column);
  if (value < BigInt(minimum) || value > BigInt(maximum)) throw new Error("BMP integer is outside its declared width");
  return Number(value);
}
function grid(width: number, height: number): number {
  integer(width); integer(height);
  const count = width * height;
  if (!Number.isSafeInteger(count) || !Number.isSafeInteger(count * 4)) throw new Error("BMP pixel count overflow");
  return count;
}
function header(snapshot: BmpSnapshot): SqliteValue[] {
  const order = snapshot.rowOrder === "topDown" ? "top_down" : snapshot.rowOrder === "bottomUp" ? "bottom_up" : undefined;
  if (!order) throw new Error("BMP row order is unknown");
  artifactSqliteTextBytes(snapshot.schema);
  return [1n, snapshot.schema, integer(snapshot.headerSize), integer(snapshot.width), integer(snapshot.height), order, integer(snapshot.planes,0,65535), integer(snapshot.bitsPerPixel,0,65535), integer(snapshot.compression), integer(snapshot.imageSize), integer(snapshot.xPixelsPerMeter,-2147483648,2147483647), integer(snapshot.yPixelsPerMeter,-2147483648,2147483647), integer(snapshot.colorsUsed), integer(snapshot.colorsImportant)];
}

/** 📤️ Project one explicit RGBA entity per canonical top-origin grid position. */
export async function bmpSnapshotToSqliteDatabase(snapshot: BmpSnapshot, options: ArtifactSqliteOptions = {}): Promise<SqliteDatabase> {
  const count = grid(snapshot.width, snapshot.height);
  if (snapshot.pixels.length !== count * 4) throw new Error("BMP pixels must contain one RGBA tuple per grid position");
  const total = count + snapshot.palette.length;
  await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
  if (total + 1 > (options.maxRows ?? 1_000_000)) throw new Error("BMP SQLite row limit");
  const document = header(snapshot);
  artifactSqliteValueBudget(count * 64 + snapshot.palette.length * 56 + 96 + artifactSqliteTextBytes(snapshot.schema) + (document[5] as string).length, options);
  for (let ordinal = 0; ordinal < snapshot.palette.length; ordinal++) {
    const entry = snapshot.palette[ordinal]!;
    integer(entry.b,0,255); integer(entry.g,0,255); integer(entry.r,0,255); integer(entry.reserved,0,255);
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
  }
  for (let index = 0; index < snapshot.pixels.length; index++) {
    integer(snapshot.pixels[index]!,0,255);
    if ((index + 1) % 1024 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", 0, total);
  }
  const palette: SqliteRow[] = [];
  const pixels: SqliteRow[] = [];
  for (let ordinal = 0; ordinal < snapshot.palette.length; ordinal++) {
    const entry = snapshot.palette[ordinal]!;
    const id = BigInt(ordinal + 1);
    palette.push({ rowid: id, values: [id, 1n, BigInt(ordinal), integer(entry.b,0,255), integer(entry.g,0,255), integer(entry.r,0,255), integer(entry.reserved,0,255)] });
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", ordinal + 1, total);
  }
  for (let ordinal = 0; ordinal < count; ordinal++) {
    const id = BigInt(ordinal + 1);
    const offset = ordinal * 4;
    pixels.push({ rowid: id, values: [id, 1n, BigInt(ordinal % snapshot.width), BigInt(Math.floor(ordinal / snapshot.width)), integer(snapshot.pixels[offset]!,0,255), integer(snapshot.pixels[offset+1]!,0,255), integer(snapshot.pixels[offset+2]!,0,255), integer(snapshot.pixels[offset+3]!,0,255)] });
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "projectSnapshot", palette.length + ordinal + 1, total);
  }
  const database = artifactSqliteDatabase(BMP_SQLITE_SCHEMA, [[{ rowid: 1n, values: document }], palette, pixels], options);
  await artifactSqliteCheckpoint(options, "projectSnapshot", total, total);
  return database;
}

/** 📥️ Reconstruct a complete unique RGBA grid and its exact typed BMP metadata. */
export async function bmpSnapshotFromSqliteDatabase(database: SqliteDatabase, options: ArtifactSqliteOptions = {}): Promise<BmpSnapshot> {
  const totalRows = database.tables.reduce((sum, table) => sum + table.rows.length, 0);
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", 0, totalRows);
  const [documents, paletteRows, pixelRows] = await artifactSqliteTables(database, BMP_SQLITE_SCHEMA, options);
  const document = artifactSqliteDocument(documents!);
  const width = read(document,3);
  const height = read(document,4);
  const count = grid(width,height);
  if (count !== pixelRows!.length) throw new Error("BMP grid must have exactly one pixel per position");
  const order = artifactSqliteText(document,5);
  const rowOrder = order === "top_down" ? "topDown" : order === "bottom_up" ? "bottomUp" : undefined;
  if (!rowOrder) throw new Error("BMP row order is unknown");
  const snapshot: BmpSnapshot = { schema: artifactSqliteText(document,1), headerSize: read(document,2), width, height, rowOrder, planes: read(document,6,0,65535), bitsPerPixel: read(document,7,0,65535), compression: read(document,8), imageSize: read(document,9), xPixelsPerMeter: read(document,10,-2147483648,2147483647), yPixelsPerMeter: read(document,11,-2147483648,2147483647), colorsUsed: read(document,12), colorsImportant: read(document,13), palette: [], pixels: new Array<number>(count * 4) };
  const seen = new Uint8Array(count);
  const total = count + paletteRows!.length;
  for (const row of artifactSqliteOrderedRows(paletteRows!,2)) {
    artifactSqliteDocumentReference(row,1);
    snapshot.palette.push({ b: read(row,3,0,255), g: read(row,4,0,255), r: read(row,5,0,255), reserved: read(row,6,0,255) });
    if (snapshot.palette.length % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", snapshot.palette.length, total);
  }
  for (let ordinal = 0; ordinal < pixelRows!.length; ordinal++) {
    const row = pixelRows![ordinal]!;
    artifactSqliteDocumentReference(row,1);
    const x = read(row,2), y = read(row,3);
    if (x >= width || y >= height) throw new Error("BMP pixel coordinates exceed the grid");
    const position = y * width + x;
    if (seen[position]) throw new Error("BMP pixel coordinates must be unique");
    seen[position] = 1;
    for (let channel = 0; channel < 4; channel++) snapshot.pixels[position * 4 + channel] = read(row,4+channel,0,255);
    if ((ordinal + 1) % 256 === 0) await artifactSqliteCheckpoint(options, "reconstructSnapshot", snapshot.palette.length + ordinal + 1, total);
  }
  await artifactSqliteCheckpoint(options, "reconstructSnapshot", total, total);
  return snapshot;
}

