import refusalFixture from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/⚠️refusal/🧫️fixtures/🔣️.json";
const canceledKind=refusalFixture.cases.find(item=>item.id==="canceled-projection")!.expectedKind;
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../../../../🧬️schema/📸️snapshot/🔣️.json";
import intermediate from "../../../../🧬️schema/📸️snapshot/🧫️fixtures/🫳️ownership/🔣️.json";

import type {JpgSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {JPG_SQLITE_SCHEMA,jpgSnapshotToSqliteDatabase,jpgSnapshotFromSqliteDatabase} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("JPG owns independent raster dimensions and partial pixel channels",async()=>{
 const snapshot={...structuredClone(fixture),image:{...structuredClone(fixture.image),...structuredClone(intermediate)}} as JpgSnapshot;
 const ajv=new Ajv({strict:false});expect(ajv.validate(schema,snapshot)).toBe(true);
 const bytes=await exportSqliteDatabase(await jpgSnapshotToSqliteDatabase(snapshot));const db=Database.deserialize(bytes);
 try{expect(db.query("SELECT ordinal,red,green,blue,alpha FROM jpg_rgba_pixel ORDER BY ordinal").all()).toEqual([{ordinal:0,red:1,green:2,blue:3,alpha:4},{ordinal:1,red:5,green:null,blue:null,alpha:null}]);expect(db.query("SELECT ordinal,red,green,blue FROM jpg_thumbnail_rgb_pixel ORDER BY ordinal").all()).toEqual([{ordinal:0,red:10,green:11,blue:12},{ordinal:1,red:13,green:14,blue:null}]);expect(await jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);}finally{db.close();}
});

test("JPG complete neutral corpus exposes independently editable pixels and table relationships",async()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);expect(JPG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
 const snapshot=structuredClone(fixture) as JpgSnapshot;const database=await jpgSnapshotToSqliteDatabase(snapshot);expect(await jpgSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  db.run("UPDATE jpg_rgba_pixel SET red=42 WHERE ordinal=0");db.run("UPDATE jpg_segment_octet SET octet=17 WHERE segment_id=1 AND ordinal=2");
  const restored=await jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));snapshot.image.pixels[0]=42;snapshot.image.otherSegments[0]!.data[2]=17;expect(restored).toEqual(snapshot);
 }finally{db.close();}
});

test("JPG preserves full widths, duplicate definitions and every optional empty entity",async()=>{
 for(const units of ["aspect","pixelsPerInch","pixelsPerCm"] as const)for(const present of [false,true]){
  const {jfifThumbnail,...image}=(fixture as JpgSnapshot).image;
  const snapshot:JpgSnapshot={schema:"custom 世界",image:{...image,width:4294967295,height:0,pixels:[],jfifDensityUnits:units,otherSegments:[],...(present?{jfifThumbnail:{width:0,height:255,rgbData:[]}}:{})}};
  expect(await jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await jpgSnapshotToSqliteDatabase(snapshot))))).toEqual(snapshot);
 }
});

test("JPG rejects orphan entities, missing scalar arrays, forged widths and cancellation",async()=>{
 const database=await jpgSnapshotToSqliteDatabase(fixture as JpgSnapshot);
 for(const sql of ["UPDATE jpg_rgba_pixel SET document_id=99","UPDATE jpg_rgba_pixel SET green=NULL,blue=NULL,alpha=NULL WHERE ordinal=0","UPDATE jpg_rgba_pixel SET green=NULL WHERE ordinal=1","UPDATE jpg_rgba_pixel SET ordinal=2 WHERE id=1","UPDATE jpg_rgba_pixel SET ordinal=0 WHERE id=2","UPDATE jpg_document SET x_density=65536","DELETE FROM jpg_thumbnail"]){const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}}
 await expect(jpgSnapshotToSqliteDatabase(fixture as JpgSnapshot,{maxRows:1})).rejects.toThrow();await expect(jpgSnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();
 const write=new AbortController();await expect(jpgSnapshotToSqliteDatabase({...fixture,image:{...fixture.image,width:1000,height:1,pixels:new Array<number>(4000).fill(1)}} as JpgSnapshot,{signal:write.signal,onProgress:event=>{if(event.completed>=256)write.abort();}})).rejects.toHaveProperty("kind",canceledKind);
 const read=new AbortController();await expect(jpgSnapshotFromSqliteDatabase(database,{signal:read.signal,onProgress:()=>read.abort()})).rejects.toHaveProperty("kind",canceledKind);
});
