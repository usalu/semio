/** 🧫️ PNG full typed ancillary, text, decoded grid and unknown chunk relational corpus. */
import { Database } from "bun:sqlite";
import { expect,test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import { parsePngSnapshot,parsePngTransparency,parsePngBackground,parsePngChunkMarker,type PngSnapshot } from "../../🟦️.ts";
import { PNG_SQLITE_SCHEMA,pngSnapshotToSqliteDatabase,pngSnapshotFromSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "@semio-tech/framework";
const input = fixture as PngSnapshot;
test("PNG typed union guards require each variant's actual payload",()=>{
  for(const parse of [parsePngTransparency,parsePngBackground]) {
    expect(()=>parse({colorType:"rgb",r:1})).toThrow();
    expect(()=>parse({colorType:"grayscale"})).toThrow();
  }
  expect(()=>parsePngTransparency({colorType:"indexed"})).toThrow();
  expect(()=>parsePngBackground({colorType:"indexed"})).toThrow();
  expect(()=>parsePngChunkMarker({chunk:"text"})).toThrow();
  expect(()=>parsePngChunkMarker({chunk:"unknown"})).toThrow();
  expect(parsePngChunkMarker({chunk:"ihdr"})).toEqual({chunk:"ihdr"});
});
test("PNG shared sixteen-table corpus exposes all typed metadata and editable semantic relationships",async () => {
  expect(PNG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
  expect(parsePngSnapshot(input)).toEqual(input);
  const database = await pngSnapshotToSqliteDatabase(input);
  expect(await pngSnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT width,height,bit_depth,color_type,interlace FROM png_document").get()).toEqual({width:2,height:1,bit_depth:4,color_type:3,interlace:1});
    expect(db.query("SELECT keyword,content FROM png_text ORDER BY ordinal").all()).toEqual(input.textChunks.map(text=>({keyword:text.keyword,content:text.value})));
    expect(db.query("SELECT t.content FROM png_chunk_sequence s JOIN png_text t ON t.id=s.text_id ORDER BY s.ordinal").all()).toEqual([{content:"second"},{content:"Grünes Bild 🎨"}]);
    expect(db.query("SELECT value FROM png_unknown_chunk_byte ORDER BY ordinal").all()).toEqual(input.unknownChunks[0]!.data.map(value=>({value})));
    expect(db.query("SELECT pixels_per_unit_y FROM png_physical_dimensions").get()).toEqual({pixels_per_unit_y:4294967295});
    db.run("UPDATE png_text SET content='SQL Text 🌠' WHERE ordinal=0");
    db.run("UPDATE png_unknown_chunk_byte SET value=42 WHERE ordinal=0");
    db.run("UPDATE png_pixel SET alpha=99 WHERE x=0 AND y=0");
    const edited = await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.textChunks[0]!.value).toBe("SQL Text 🌠");
    expect(edited.unknownChunks[0]!.data[0]).toBe(42);
    expect(edited.pixels[3]).toBe(99);
    expect(edited.chunkOrder).toEqual(input.chunkOrder);
    expect(edited.chrm).toEqual(input.chrm);
  } finally {db.close();}
});

test("PNG independently edited ownership, optional shapes, widths and grids reject",async () => {
  for(const edit of [
    "UPDATE png_chunk_sequence SET text_id=999 WHERE chunk_kind='text'",
    "UPDATE png_unknown_chunk_byte SET unknown_chunk_id=999 WHERE id=1",
    "UPDATE png_palette_entry SET palette_id=999 WHERE id=1",
    "UPDATE png_transparency_alpha SET transparency_id=999 WHERE id=1",
    "UPDATE png_pixel SET x=0,y=0 WHERE id=2",
    "DELETE FROM png_pixel WHERE id=1",
    "UPDATE png_unknown_chunk_byte SET ordinal=99 WHERE id=1",
    "UPDATE png_transparency SET gray=1",
    "UPDATE png_background SET red=1",
    "UPDATE png_modification_time SET year=65536",
    "UPDATE png_unknown_chunk SET kind_octet_1=256",
    "UPDATE png_text SET chunk_kind='unknown'",
    "UPDATE png_gamma SET gamma_times_100000=4294967296",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await pngSnapshotToSqliteDatabase(input)));
    try{db.run("PRAGMA ignore_check_constraints=ON");db.run(edit);await expect(pngSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}
    finally{db.close();}
  }
});

test("PNG optionals retain palette absence versus empty presence and typed union variants",async () => {
  const empty: PngSnapshot = {schema:"empty.png",width:0,height:0,bitDepth:255,colorType:"rgba",interlace:false,textChunks:[],pixels:[],chunkOrder:[],unknownChunks:[]};
  for(const plte of [undefined,[]]) {
    const value={...empty,plte};
    expect(await pngSnapshotFromSqliteDatabase(await pngSnapshotToSqliteDatabase(value))).toEqual(value);
  }
  for(const trns of [{colorType:"indexed" as const,alpha:[]},{colorType:"grayscale" as const,gray:65535},{colorType:"rgb" as const,r:0,g:65535,b:17}]) {
    for(const bkgd of [{colorType:"indexed" as const,index:255},{colorType:"grayscale" as const,gray:65535},{colorType:"rgb" as const,r:1,g:2,b:3}]) {
      const value={...empty,trns,bkgd,gama:0};
      expect(await pngSnapshotFromSqliteDatabase(await pngSnapshotToSqliteDatabase(value))).toEqual(value);
    }
  }
});

test("PNG aggregate bounds, intrinsic byte widths and marker references validate before allocation",async () => {
  const database=await pngSnapshotToSqliteDatabase(input);
  for(const options of [{maxRows:0},{maxValueBytes:0}]) {
    await expect(pngSnapshotToSqliteDatabase(input,options)).rejects.toThrow("limit");
    await expect(pngSnapshotFromSqliteDatabase(database,options)).rejects.toThrow("limit");
  }
  for(const value of [{...input,pixels:[]},{...input,unknownChunks:[{kind:[1,2,3],data:[]}]},{...input,chunkOrder:[{chunk:"unknown" as const,index:99}]}]) await expect(pngSnapshotToSqliteDatabase(value)).rejects.toThrow();
});

test("PNG large intrinsic-byte counting and reconstruction honor cancellation",async () => {
  const many={...input,unknownChunks:[{kind:[1,2,3,4],data:Array.from({length:1000},()=>255)}]};
  const controller=new AbortController();let events=0;
  await expect(pngSnapshotToSqliteDatabase(many,{signal:controller.signal,onProgress:()=>{if(++events===2)controller.abort();}})).rejects.toMatchObject({name:"AbortError"});
  const database=await pngSnapshotToSqliteDatabase(input),restore=new AbortController();
  await expect(pngSnapshotFromSqliteDatabase(database,{signal:restore.signal,onProgress:()=>restore.abort()})).rejects.toMatchObject({name:"AbortError"});
});
