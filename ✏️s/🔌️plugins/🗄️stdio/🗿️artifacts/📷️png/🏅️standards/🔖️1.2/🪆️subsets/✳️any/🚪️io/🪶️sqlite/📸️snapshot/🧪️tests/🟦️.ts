/** 🧫️ Complete logical PNG owner and interpreted native SQLite laws. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { PNG as IndependentPng } from "pngjs";
import fixture from "../🧫️fixtures/🔣️.json";
import { parsePngSnapshot, type PngSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { PNG_SQLITE_SCHEMA, pngSnapshotFromSqliteDatabase, pngSnapshotToSqliteDatabase } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

const input = parsePngSnapshot(fixture) as PngSnapshot;

async function serialized(snapshot: PngSnapshot = input): Promise<Uint8Array> {
  return exportSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot));
}

test("PNG SQLite interprets the exact native owner and reconstructs it byte-for-byte", async () => {
  expect(PNG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql", import.meta.url)).text());
  const bytes = await serialized();
  const database = Database.deserialize(bytes);
  try {
    expect(database.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(database.query("SELECT id,schema,role FROM png_document").get()).toEqual({id:1,schema:"stdio.png",role:"native"});
    expect(database.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table'").get()).toEqual({count:24});
    expect(database.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table' AND sql LIKE '%BLOB%'").get()).toEqual({count:0});
    expect(database.query("SELECT COUNT(*) AS count FROM png_literal_octet").get()).toEqual({count:0});
  } finally {
    database.close();
  }
  expect(await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(input);
});

test("PNG SQLite round-trip preserves independent decoder pixels and source profile", async () => {
  const before = IndependentPng.sync.read(Buffer.from(input.bytes));
  const restored = await pngSnapshotFromSqliteDatabase(await pngSnapshotToSqliteDatabase(input));
  const after = IndependentPng.sync.read(Buffer.from(restored.bytes));
  expect(restored.bytes).toEqual(input.bytes);
  expect({ width: after.width, height: after.height, depth: after.depth, colorType: after.colorType }).toEqual({ width: before.width, height: before.height, depth: before.depth, colorType: before.colorType });
  expect([...after.data]).toEqual([...before.data]);
});

test("PNG SQLite rejects non-byte input and malformed singleton storage", async () => {
  for (const bytes of [[-1], [256], [1.5]]) {
    await expect(pngSnapshotToSqliteDatabase({ schema: "stdio.png", bytes })).rejects.toThrow("outside u8");
  }
  for (const edit of ["DELETE FROM png_document", "UPDATE png_document SET role='literal'"]) {
    const database = Database.deserialize(await serialized());
    try {
      database.run(edit);
      await expect(pngSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(database.serialize())))).rejects.toThrow();
    } finally {
      database.close();
    }
  }
});

test("PNG SQLite projection and reconstruction honor cancellation during bounded copies", async () => {
  const large: PngSnapshot = { schema: "stdio.png", bytes: Array.from({ length: 200_000 }, (_, index) => index & 255) };
  const projection = new AbortController();
  let projected = false;
  await expect(pngSnapshotToSqliteDatabase(large, { signal: projection.signal, onProgress: event => { if (event.phase === "projectSnapshot" && event.completed < event.total) { projected = true; projection.abort(); } } })).rejects.toMatchObject({ kind: "canceled" });
  expect(projected).toBe(true);

  const database = await pngSnapshotToSqliteDatabase(large);
  const reconstruction = new AbortController();
  let reconstructed = false;
  await expect(pngSnapshotFromSqliteDatabase(database, { signal: reconstruction.signal, onProgress: event => { if (event.phase === "reconstructSnapshot" && event.completed < event.total) { reconstructed = true; reconstruction.abort(); } } })).rejects.toMatchObject({ kind: "canceled" });
  expect(reconstructed).toBe(true);
});

import "./🚦️audit/🟦️.ts";
import "./🪆️owner/🟦️.ts";


test("PNG exact native owner exposes independently editable IHDR and ordered chunk entities",async()=>{
  const neutral=await Bun.file(new URL("../🧫️fixtures/🧩️semantic/🔣️.json",import.meta.url)).json();
  
  const {default:Ajv}=await import("ajv");
  expect(neutral["role"]).toEqual("exactNativeChunks");expect(neutral["ihdr"]).toEqual({"width":4,"height":1,"bitDepth":2,"colorType":3,"compression":0,"filter":0,"interlace":0});expect(neutral["chunkKinds"]).toEqual(["IHDR","PLTE","tRNS","bKGD","IDAT","IEND"]);expect(neutral["edit"]).toEqual({"table":"png_ihdr","field":"width","value":3});expect(neutral["editedRgba"]).toEqual([255,0,0,255,255,0,0,64,0,255,0,128]);expect(neutral["compressedPixels"]).toEqual("unclosed");
  const independent=Database.deserialize(await serialized());
  try{
    expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const header=independent.query("SELECT width,height,bit_depth AS bitDepth,color_type AS colorType,compression,filter,interlace FROM png_ihdr").get();
    expect(header).toEqual(neutral.ihdr);
    expect(independent.query("SELECT kind FROM png_chunk ORDER BY ordinal").all().map((row:any)=>row.kind)).toEqual(neutral.chunkKinds);
    expect(await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).toEqual(input);
    independent.query("UPDATE png_ihdr SET width=?").run(neutral.edit.value);
    const edited=await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()));
    const decoded=IndependentPng.sync.read(Buffer.from(edited.bytes));
    expect(decoded.width).toBe(3);expect(decoded.height).toBe(1);
    expect([...decoded.data]).toEqual(neutral.editedRgba);
    expect(edited.schema).toBe(input.schema);
    expect(edited.bytes).not.toEqual(input.bytes);
  }finally{independent.close();}
});

import "./🌈️scanline/🟦️.ts";

import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {pngSnapshotToSqliteDatabase as normSemanticProject,pngSnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";
test("PNG complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(7206);expect(normSemanticContract["tableWidths"]).toEqual({"png_background":4,"png_chromaticity":9,"png_chunk":4,"png_deflate_block":9,"png_deflate_code":4,"png_deflate_length":5,"png_deflate_token":7,"png_document":3,"png_gamma":2,"png_idat":3,"png_ihdr":8,"png_inflated_tail":4,"png_literal_octet":4,"png_packed_remainder":3,"png_palette":6,"png_physical":4,"png_sample":4,"png_scanline":9,"png_srgb":2,"png_stream":7,"png_text":7,"png_time":7,"png_transparency":4,"png_unknown_octet":4});expect(normSemanticContract["cases"]).toEqual([{"id":"nativeCanonicalAsset","rawBytes":145,"rows":45,"valueBytes":1612,"role":"native","sha256":"a9e37b14842b4e6c0413da868aa7214ce1cfceaa690bdc579119e997f5e5f392"},{"id":"sourceJsonNativeImage","rawBytes":120,"rows":28,"valueBytes":1079,"role":"native","sha256":"a39b62ca21cda8572574a8d4c454c9d2f6f9eabd542b6ed44ef38b5f9e9d3a2f"},{"id":"emptyLiteral","rawBytes":0,"rows":1,"valueBytes":24,"role":"literal","sha256":"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"},{"id":"invalidRawLiteral","rawBytes":3,"rows":4,"valueBytes":120,"role":"literal","sha256":"47ffa3ea45a70b8a41c2c0825df323c00a8b7a01c1ea06083cc41dddcc001123"}]);
 for(const item of normSemanticContract.cases){const carrier=item.id==="nativeCanonicalAsset"?[...new Uint8Array(await Bun.file(new URL("../../../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-multi-idat-private.png",import.meta.url)).arrayBuffer())]:item.id==="emptyLiteral"?[]:item.id==="invalidRawLiteral"?[0,255,1]:fixture.bytes;const source=parsePngSnapshot({schema:"stdio.png",bytes:carrier});expect(carrier.length).toBe(item.rawBytes);expect(new Bun.CryptoHasher("sha256").update(new Uint8Array(carrier)).digest("hex")).toBe(item.sha256);const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const oracle=Database.deserialize(bytes);try{expect(oracle.query("SELECT role FROM png_document").get()).toEqual({role:item.role});}finally{oracle.close();}const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:24,maxColumns:9};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:23},{...limits,maxColumns:8}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] PNG complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
