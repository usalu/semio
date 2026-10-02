import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import schema from "../../🔣️.json";
import intermediate from "../../🧫️fixtures/🫳️ownership/🔣️.json";
import intermediateSchema from "../../🧫️fixtures/🫳️ownership/🧬️schema/🔣️.json";
import type {JpgSnapshot} from "../../🟦️.ts";
import {JPG_SQLITE_SCHEMA,jpgSnapshotToSqliteDatabase,jpgSnapshotFromSqliteDatabase} from "../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("JPG owns independent raster dimensions and partial pixel channels",async()=>{
 const snapshot={...structuredClone(fixture),...structuredClone(intermediate)} as JpgSnapshot;
 const ajv=new Ajv({strict:false});expect(ajv.validate(intermediateSchema,intermediate)).toBe(true);expect(ajv.validate(schema,snapshot)).toBe(true);
 const bytes=await exportSqliteDatabase(await jpgSnapshotToSqliteDatabase(snapshot));const db=Database.deserialize(bytes);
 try{expect(db.query("SELECT ordinal,red,green,blue,alpha FROM jpg_rgba_pixel ORDER BY ordinal").all()).toEqual([{ordinal:0,red:1,green:2,blue:3,alpha:4},{ordinal:1,red:5,green:null,blue:null,alpha:null}]);expect(db.query("SELECT ordinal,red,green,blue FROM jpg_thumbnail_rgb_pixel ORDER BY ordinal").all()).toEqual([{ordinal:0,red:10,green:11,blue:12},{ordinal:1,red:13,green:14,blue:null}]);expect(await jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(bytes))).toEqual(snapshot);}finally{db.close();}
});

test("JPG complete neutral corpus exposes independently editable pixels and table relationships",async()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);expect(JPG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
 const snapshot=structuredClone(fixture) as JpgSnapshot;const database=await jpgSnapshotToSqliteDatabase(snapshot);expect(await jpgSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT c.ordinal,c.component_id,q.id AS quantizer_id,COUNT(t.id) AS definition_count FROM jpg_frame_component c JOIN jpg_quantizer q ON q.id=c.quantizer_id LEFT JOIN jpg_quantization_table t ON t.quantizer_id=q.id GROUP BY c.id ORDER BY c.ordinal").all()).toEqual([{ordinal:0,component_id:255,quantizer_id:255,definition_count:2},{ordinal:1,component_id:255,quantizer_id:42,definition_count:0}]);
  db.run("UPDATE jpg_rgba_pixel SET red=42 WHERE ordinal=0");db.run("UPDATE jpg_quantization_coefficient SET coefficient=12345 WHERE table_id=1 AND zigzag_ordinal=63");db.run("UPDATE jpg_huffman_symbol SET symbol=99 WHERE table_id=1 AND ordinal=1");db.run("UPDATE jpg_segment_octet SET octet=17 WHERE segment_id=1 AND ordinal=2");
  const restored=await jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));snapshot.pixels[0]=42;snapshot.quantTables[0]!.values[63]=12345;snapshot.huffmanTables[0]!.values[1]=99;snapshot.otherSegments[0]!.data[2]=17;expect(restored).toEqual(snapshot);
 }finally{db.close();}
});

test("JPG preserves full widths, duplicate definitions and every optional empty entity",async()=>{
 const zeroId=structuredClone(fixture) as JpgSnapshot;zeroId.quantTables[0]!.id=0;zeroId.frame!.components[0]!.quantTableId=0;expect(await jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await jpgSnapshotToSqliteDatabase(zeroId))))).toEqual(zeroId);
 for(const units of ["aspect","pixelsPerInch","pixelsPerCm"] as const)for(const present of [false,true]){
  const {jfifThumbnail,frame,reEncodeQuality,restartInterval,...document}=fixture as JpgSnapshot;
  const snapshot:JpgSnapshot={...document,schema:"custom 世界",width:4294967295,height:0,pixels:[],jfifDensityUnits:units,quantTables:[],huffmanTables:[{id:0,class:"ac",bits:new Array<number>(16).fill(0),values:[]}],otherSegments:[],...(present?{jfifThumbnail:{width:0,height:255,rgbData:[]},frame:{precision:255,width:0,height:65535,components:[]},reEncodeQuality:0,restartInterval:65535}:{})};
  expect(await jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(await exportSqliteDatabase(await jpgSnapshotToSqliteDatabase(snapshot))))).toEqual(snapshot);
 }
});

test("JPG rejects orphan entities, missing scalar arrays, forged widths and cancellation",async()=>{
 const database=await jpgSnapshotToSqliteDatabase(fixture as JpgSnapshot);
 for(const sql of ["UPDATE jpg_rgba_pixel SET document_id=99","UPDATE jpg_rgba_pixel SET green=NULL,blue=NULL,alpha=NULL WHERE ordinal=0","UPDATE jpg_rgba_pixel SET green=NULL WHERE ordinal=1","UPDATE jpg_rgba_pixel SET ordinal=2 WHERE id=1","UPDATE jpg_rgba_pixel SET ordinal=0 WHERE id=2","UPDATE jpg_frame_component SET ordinal=0 WHERE id=2","DELETE FROM jpg_quantization_coefficient WHERE id=1","DELETE FROM jpg_huffman_code_length WHERE id=1","UPDATE jpg_quantization_coefficient SET table_id=99 WHERE id=1","INSERT INTO jpg_quantizer VALUES(43,1)","UPDATE jpg_frame_component SET quantizer_id=43 WHERE id=1","UPDATE jpg_document SET re_encode_quality=256","UPDATE jpg_huffman_table SET table_class='unknown'","DELETE FROM jpg_thumbnail"]){const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(sql);await expect(jpgSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}}
 await expect(jpgSnapshotToSqliteDatabase(fixture as JpgSnapshot,{maxRows:1})).rejects.toThrow();await expect(jpgSnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();
 const write=new AbortController();await expect(jpgSnapshotToSqliteDatabase({...fixture,width:1000,height:1,pixels:new Array<number>(4000).fill(1)} as JpgSnapshot,{signal:write.signal,onProgress:event=>{if(event.completed>=256)write.abort();}})).rejects.toHaveProperty("name","AbortError");
 const read=new AbortController();await expect(jpgSnapshotFromSqliteDatabase(database,{signal:read.signal,onProgress:()=>read.abort()})).rejects.toHaveProperty("name","AbortError");
});
