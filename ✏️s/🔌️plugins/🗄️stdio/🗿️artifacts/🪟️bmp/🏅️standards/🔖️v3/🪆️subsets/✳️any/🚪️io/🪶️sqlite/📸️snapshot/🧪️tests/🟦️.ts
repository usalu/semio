/** 🪟️ Precise owned BMP relational edits and native admission laws. */
import "./🧬️semantic/🟦️.ts";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import corpus from "../../../../🧫️fixtures/🧬️canonical-byte-authority/🔣️.json";
import {decodeBmpSnapshot,encodeBmpSnapshot} from "../../../💾️binary/📸️snapshot/🟦️.ts";
import {bmpSnapshotToSqliteDatabase,bmpSnapshotFromSqliteDatabase,BMP_SQLITE_SCHEMA} from "../🟦️.ts";
import {defaultBmpSnapshot,parseBmpSnapshot,type BmpSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
async function source(file:string):Promise<BmpSnapshot>{return decodeBmpSnapshot(new Uint8Array(await Bun.file(new URL("../../../../🧫️fixtures/🧬️canonical-byte-authority/"+file,import.meta.url)).arrayBuffer()));}
test("BMP native profiles preserve precise samples and unknown metadata through direct owned SQL",async()=>{
 expect(BMP_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
 for(const entry of corpus.accepted){const snapshot=await source(entry.file),relational=await bmpSnapshotToSqliteDatabase(snapshot),db=Database.deserialize(await exportSqliteDatabase(relational));try{expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT width,height,row_order,profile FROM bmp_image").get()).toEqual({width:entry.width,height:entry.height,row_order:entry.topDown?"topDown":"bottomUp",profile:entry.profile});expect(db.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table'").get()).toEqual({count:6});expect(await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(snapshot);expect(await decodeBmpSnapshot(await encodeBmpSnapshot(snapshot))).toEqual(snapshot);}finally{db.close();}}
 for(const entry of corpus.rejected){const raw=new Uint8Array(await Bun.file(new URL("../../../../🧫️fixtures/🧬️canonical-byte-authority/"+entry.file,import.meta.url)).arrayBuffer());await expect(decodeBmpSnapshot(raw)).rejects.toThrow();}
});
test("BMP independently edited cells update exact logical components and retain reserved data",async()=>{
 const snapshot=await source("direct-rgb24-padding-gap-trailer.bmp"),db=Database.deserialize(await exportSqliteDatabase(await bmpSnapshotToSqliteDatabase(snapshot)));
 try{db.run("UPDATE bmp_pixel_sample SET red=7 WHERE ordinal=1");db.run("UPDATE bmp_image SET x_pixels_per_meter=-2147483648");const result=await bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize())),expected=parseBmpSnapshot(snapshot);if(expected.image.pixels.storage!=="direct")throw Error("direct fixture");expected.image.pixels.samples[1]!.red=7;expected.image.xPixelsPerMeter=-2147483648;expect(result).toEqual(expected);expect(await decodeBmpSnapshot(await encodeBmpSnapshot(result))).toEqual(expected);}finally{db.close();}
});
test("BMP complete ownership, precision, discriminators and occurrence order refuse malformed SQL",async()=>{
 const snapshot=await source("direct-rgb24-padding-gap-trailer.bmp");
 for(const edit of ["DELETE FROM bmp_pixel_sample WHERE ordinal=0","UPDATE bmp_pixel_sample SET ordinal=99 WHERE ordinal=0","UPDATE bmp_pixel_sample SET image=9 WHERE ordinal=0","UPDATE bmp_pixel_sample SET red=256 WHERE ordinal=0","UPDATE bmp_image SET row_order='invented'","UPDATE bmp_image SET reserved_1=65536","UPDATE bmp_image SET width=2147483647,height=2147483647","UPDATE bmp_gap_octet SET ordinal=99 WHERE ordinal=0"]){const db=Database.deserialize(await exportSqliteDatabase(await bmpSnapshotToSqliteDatabase(snapshot)));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(edit);await expect(bmpSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}}
 expect(()=>parseBmpSnapshot({schema:"stdio.bmp",bytes:[]})).toThrow();
});
test("BMP relational projection and reconstruction honor cumulative caller limits and cancellation",async()=>{
 const source=defaultBmpSnapshot(),database=await bmpSnapshotToSqliteDatabase(source);
 for(const options of [{maxRows:0},{maxValueBytes:0},{maxAllocationBytes:1}]){await expect(bmpSnapshotToSqliteDatabase(source,options)).rejects.toThrow();await expect(bmpSnapshotFromSqliteDatabase(database,options)).rejects.toThrow();}
 for(const phase of ["project","reconstruct"]){const abort=new AbortController();let progress=0;const options={signal:abort.signal,onProgress:()=>{progress++;abort.abort();}};await expect(phase==="project"?bmpSnapshotToSqliteDatabase(source,options):bmpSnapshotFromSqliteDatabase(database,options)).rejects.toMatchObject({kind:"canceled"});expect(progress).toBe(1);}
});
