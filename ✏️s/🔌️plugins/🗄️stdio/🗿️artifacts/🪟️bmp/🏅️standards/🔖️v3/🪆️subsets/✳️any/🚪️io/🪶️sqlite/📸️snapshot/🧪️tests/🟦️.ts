/** 🧫️ BMP byte-owner semantic SQL and independent native-octet assertions. */
import "./🧬️semantic/🟦️.ts";
import { Database } from "bun:sqlite";
import { expect,test } from "bun:test";
import fixture from "../🧫️fixtures/🔣️.json";
import semantic from "../🧫️fixtures/🧬️semantic/🔣️.json";
import corpus from "../../../../🧫️fixtures/🧬️canonical-byte-authority/🔣️.json";
import { parseBmpSnapshot,type BmpSnapshot } from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import { BMP_SQLITE_SCHEMA,bmpSnapshotToSqliteDatabase,bmpSnapshotFromSqliteDatabase } from "../🟦️.ts";
import { bmpByteLayout,bmpWord } from "../../../🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase } from "@semio-tech/framework";
import controlFixture from "../🧫️fixtures/🛬️native-control/🔣️.json";

import Ajv2020 from "ajv/dist/2020";
import {NativeDecodeControl as BudgetDecodeControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {SqliteAllocationControl as BudgetAllocationControl} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
const input=fixture as BmpSnapshot;
async function source(file:string):Promise<BmpSnapshot>{return{schema:"stdio.bmp",bytes:Array.from(new Uint8Array(await Bun.file(new URL("../../../../🧫️fixtures/🧬️canonical-byte-authority"+file,import.meta.url)).arrayBuffer()))};}
function native32(width:number):BmpSnapshot{const bytes=new Uint8Array(54+width*4),view=new DataView(bytes.buffer);bytes[0]=66;bytes[1]=77;view.setUint32(10,54,true);view.setUint32(14,40,true);view.setInt32(18,width,true);view.setInt32(22,1,true);view.setUint16(26,1,true);view.setUint16(28,32,true);view.setUint32(34,4294967295,true);for(let i=54;i<bytes.length;i++)bytes[i]=controlFixture.bytePattern[(i-54)%4]!;return{schema:controlFixture.ownedSchema,bytes:Array.from(bytes)};}
test("BMP handwritten native headers, samples and separate unknown octets expose independent editable SQL",async()=>{
 expect(BMP_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
 expect(parseBmpSnapshot(input)).toEqual(input);
 const database=await bmpSnapshotToSqliteDatabase(input);expect(await bmpSnapshotFromSqliteDatabase(database)).toEqual(input);
 const db=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT width,signed_height,bits_per_pixel,x_pixels_per_meter,y_pixels_per_meter FROM bmp_info_header").get()).toEqual({width:3,signed_height:2,bits_per_pixel:24,x_pixels_per_meter:2835,y_pixels_per_meter:2835});
  expect(db.query("SELECT reserved_1,reserved_2,pixel_offset FROM bmp_file_header").get()).toEqual({reserved_1:0x1234,reserved_2:0x5678,pixel_offset:57});
  expect(db.query("SELECT value FROM bmp_gap_octet ORDER BY ordinal").all()).toEqual([222,173,190].map(value=>({value})));
  expect(db.query("SELECT value FROM bmp_trailer_octet ORDER BY ordinal").all()).toEqual([254,237,250,206].map(value=>({value})));
  expect(db.query("SELECT x,y,red,green,blue,unused_bits FROM bmp_pixel_sample ORDER BY y,x").all()).toEqual([{x:0,y:0,red:255,green:0,blue:0,unused_bits:0},{x:1,y:0,red:0,green:255,blue:0,unused_bits:0},{x:2,y:0,red:0,green:0,blue:255,unused_bits:0},{x:0,y:1,red:255,green:255,blue:0,unused_bits:0},{x:1,y:1,red:255,green:0,blue:255,unused_bits:0},{x:2,y:1,red:255,green:255,blue:255,unused_bits:0}]);
  db.run("UPDATE bmp_pixel_sample SET red=7 WHERE x=1 AND y=0");db.run("UPDATE bmp_info_header SET x_pixels_per_meter=-2147483648");
  const edited=await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize()))),expected=Uint8Array.from(input.bytes);expected[74]=7;new DataView(expected.buffer).setInt32(38,-2147483648,true);expect(edited.bytes).toEqual(Array.from(expected));expect(bmpByteLayout(edited.bytes).state).toBe("valid_layout");
 }finally{db.close();}
 const indexed=await source("indexed-rgb4-duplicate-palette.bmp"),palette=Database.deserialize(await exportSqliteDatabase(await bmpSnapshotToSqliteDatabase(indexed)));
 try{palette.run("UPDATE bmp_palette_entry SET reserved=99 WHERE ordinal=0");const edited=await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(palette.serialize()))),expected=[...indexed.bytes];expected[57]=99;expect(edited.bytes).toEqual(expected);}finally{palette.close();}
});
test("bmp borrowed native preflight contract has literal octet and refusal authorities", async()=>{
 
 expect(controlFixture["preflight"]["encodings"]).toEqual(["binary","text"]);expect(controlFixture["preflight"]["ownershipBytes"]).toEqual(0);expect(controlFixture["preflight"]["byteAuthority"]).toEqual("actualLiteralOutputUTF8OrOctets");expect(controlFixture["preflight"]["refusalKinds"]["file"]).toEqual("ownershipLimit");expect(controlFixture["preflight"]["refusalKinds"]["cancellation"]).toEqual("canceled");expect(controlFixture["preflight"]["cancelAt"]).toEqual("firstBorrowedCheckpoint");expect(controlFixture["preflight"]["retirementRefund"]).toEqual(false);expect(controlFixture["expectedRows"]).toEqual(1027);expect(controlFixture["allocationRole"]).toEqual("cumulativeOwnedBacking");
 const policy=controlFixture.preflight;
 expect(policy).toEqual({"encodings":["binary","text"],"ownershipBytes":0,"byteAuthority":"actualLiteralOutputUTF8OrOctets","refusalKinds":{"file":"ownershipLimit","cancellation":"canceled"},"cancelAt":"firstBorrowedCheckpoint","retirementRefund":false});
 
 
 const literal=controlFixture.fieldText,octets=Buffer.from(literal,"utf8");
 expect(Buffer.byteLength(literal,"utf8")).toBe(new TextEncoder().encode(literal).length);
 expect(octets.toString("utf8")).toBe(literal);
 const independent=new Database(":memory:");
 try{independent.run("CREATE TABLE literal_output(value TEXT NOT NULL)");independent.run("INSERT INTO literal_output VALUES(?)",[literal]);expect(independent.query("SELECT length(CAST(value AS BLOB)) AS bytes FROM literal_output").get()).toEqual({bytes:octets.byteLength});}finally{independent.close();}
 expect(policy.ownershipBytes).toBe(0);expect(policy.retirementRefund).toBe(false);
 expect(controlFixture.allocationRole).toBe("cumulativeOwnedBacking");
 expect(controlFixture.maxAllocationBytes).toBe(1);
 
 const primitive=Buffer.from(controlFixture.bytePattern.slice(0,controlFixture.maxAllocationBytes+1));
 expect(primitive.byteLength).toBe(controlFixture.maxAllocationBytes+1);
 const ownership=new BudgetAllocationControl({maxAllocationBytes:controlFixture.maxAllocationBytes});
 await expect(ownership.stage(maximum=>new BudgetDecodeControl(maximum,()=>true),native=>native.copyBytes(primitive))).rejects.toMatchObject({kind:"ownershipLimit"});
 expect(ownership.remainingBytes()).toBe(controlFixture.maxAllocationBytes);
 const semanticLimits={maxAllocationBytes:primitive.byteLength,maxValueBytes:0};const semantic=new BudgetAllocationControl(semanticLimits);
 const copied=await semantic.stage(maximum=>new BudgetDecodeControl(maximum,()=>true),native=>native.copyBytes(primitive));
 expect(Buffer.from(copied)).toEqual(primitive);expect(semantic.remainingBytes()).toBe(0);

});


test("BMP independent semantic edits reject incomplete grids, widths, masks and variant relationships",async()=>{
 for(const edit of ["DELETE FROM bmp_pixel_sample WHERE id=1","UPDATE bmp_pixel_sample SET x=99 WHERE id=1","UPDATE bmp_pixel_sample SET x=0,y=0 WHERE id=2","UPDATE bmp_pixel_sample SET document_id=999 WHERE id=1","UPDATE bmp_pixel_sample SET red=256 WHERE id=1","UPDATE bmp_pixel_sample SET unused_bits=1 WHERE id=1","UPDATE bmp_info_header SET planes=65536","UPDATE bmp_info_header SET x_pixels_per_meter=2147483648","UPDATE bmp_info_header SET width=2147483647,signed_height=2147483647","DELETE FROM bmp_gap_octet WHERE id=1","UPDATE bmp_row_padding_octet SET ordinal=99 WHERE id=1","UPDATE bmp_document SET state='literal_octets',diagnostic='invented'"]){
  const db=Database.deserialize(await exportSqliteDatabase(await bmpSnapshotToSqliteDatabase(input)));
  try{db.run("PRAGMA ignore_check_constraints=ON");db.run(edit);await expect(bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}
 }
 for(const [name,edit]of [["indexed-rgb4-duplicate-palette.bmp","UPDATE bmp_palette_entry SET ordinal=99 WHERE id=1"],["indexed-rgb4-duplicate-palette.bmp","UPDATE bmp_palette_entry SET blue=-1 WHERE id=1"],["direct-bitfields16-565.bmp","UPDATE bmp_channel_mask SET mask=1 WHERE ordinal IN (0,1)"]] as const){
  const db=Database.deserialize(await exportSqliteDatabase(await bmpSnapshotToSqliteDatabase(await source(name))));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(edit);await expect(bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}
 }
});
test("BMP every native profile, signed row order and complete literal owner states preserve exact bytes",async()=>{
 for(const entry of corpus.accepted){const snapshot=await source(entry.file),database=await bmpSnapshotToSqliteDatabase(snapshot);expect(await bmpSnapshotFromSqliteDatabase(database)).toEqual(snapshot);const layout=bmpByteLayout(snapshot.bytes);expect(layout.state).toBe("valid_layout");if(layout.state==="valid_layout"){expect(layout.layout.width).toBe(entry.width);expect(layout.layout.height).toBe(entry.height);expect(layout.layout.signedHeight<0).toBe(entry.topDown);expect(snapshot.bytes.length).toBe(entry.sourceBytes);}}
 for(const literal of semantic.literalCases){const snapshot={schema:literal.schema,bytes:literal.bytes},database=await bmpSnapshotToSqliteDatabase(snapshot),db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("SELECT state,diagnostic FROM bmp_document").get()).toEqual({state:"literal_octets",diagnostic:literal.diagnostic});expect(db.query("SELECT value FROM bmp_literal_octet ORDER BY ordinal").all()).toEqual(literal.bytes.map(value=>({value})));expect(await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);}finally{db.close();}}
 for(const entry of corpus.rejected){const snapshot=await source(entry.file);expect(bmpByteLayout(snapshot.bytes).state).toBe("literal_octets");expect(await bmpSnapshotFromSqliteDatabase(await bmpSnapshotToSqliteDatabase(snapshot))).toEqual(snapshot);}
 for(const snapshot of [{...input,schema:"arbitrary.\0引用😀"},(()=>{const bytes=[...input.bytes];bytes.splice(18,4,255,255,255,255);return{schema:"stdio.bmp",bytes};})()])expect(await bmpSnapshotFromSqliteDatabase(await bmpSnapshotToSqliteDatabase(snapshot))).toEqual(snapshot);
 const orphan=await source("indexed-rgb4-duplicate-palette.bmp"),offset=bmpWord(orphan.bytes,10,4);orphan.bytes[offset]=(orphan.bytes[offset]!&15)|240;expect(bmpByteLayout(orphan.bytes).state).toBe("valid_layout");expect(await bmpSnapshotFromSqliteDatabase(await bmpSnapshotToSqliteDatabase(orphan))).toEqual(orphan);
 const empty={schema:"stdio.bmp",bytes:input.bytes.slice(0,54)};empty.bytes.splice(2,4,0,0,0,0);empty.bytes.splice(10,4,54,0,0,0);empty.bytes.splice(18,8,0,0,0,0,0,0,0,0);expect(bmpByteLayout(empty.bytes).state).toBe("valid_layout");expect(await bmpSnapshotFromSqliteDatabase(await bmpSnapshotToSqliteDatabase(empty))).toEqual(empty);
 for(const bytes of [[256],[-1],[0.5]])await expect(bmpSnapshotToSqliteDatabase({schema:"stdio.bmp",bytes})).rejects.toThrow();
});
test("BMP cumulative bounds and cancellation precede relational reconstruction and byte publication",async()=>{
 const database=await bmpSnapshotToSqliteDatabase(input);
 for(const options of [{maxRows:0},{maxValueBytes:0},{maxAllocationBytes:1}]){await expect(bmpSnapshotToSqliteDatabase(input,options)).rejects.toThrow("limit");await expect(bmpSnapshotFromSqliteDatabase(database,options)).rejects.toThrow("limit");}
 const controller=new AbortController();let events=0;await expect(bmpSnapshotToSqliteDatabase(native32(1000),{signal:controller.signal,onProgress:()=>{if(++events===2)controller.abort();}})).rejects.toMatchObject({kind:"canceled"});expect(events).toBe(2);
 const restore=new AbortController();await expect(bmpSnapshotFromSqliteDatabase(database,{signal:restore.signal,onProgress:()=>restore.abort()})).rejects.toMatchObject({kind:"canceled"});
});
test("BMP native control corpus retains complete real word fields and unknown byte ownership",async()=>{
 
 const owned=native32(controlFixture.workItems),database=await bmpSnapshotToSqliteDatabase(owned),file=await exportSqliteDatabase(database),independent=Database.deserialize(file);
 expect(database.tables.reduce((sum,table)=>sum+table.rows.length,0)).toBe(controlFixture.expectedRows);
 try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(independent.query("SELECT schema FROM bmp_document").get()).toEqual({schema:controlFixture.ownedSchema});expect(independent.query("SELECT declared_image_size FROM bmp_info_header").get()).toEqual({declared_image_size:4294967295});expect(independent.query("SELECT unused_bits FROM bmp_pixel_sample LIMIT 1").get()).toEqual({unused_bits:2147483648});}finally{independent.close();}
 expect(await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(file))).toEqual(owned);
});
