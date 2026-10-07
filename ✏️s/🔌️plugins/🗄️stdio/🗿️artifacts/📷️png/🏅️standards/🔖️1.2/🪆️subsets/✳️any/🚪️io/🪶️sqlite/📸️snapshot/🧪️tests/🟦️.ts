/** 📷️ Direct precise PNG samples and independent SQL edits. */
import "./🌈️scanline/🟦️.ts";
import "./🚦️audit/🟦️.ts";
import "./🪆️owner/🟦️.ts";
import {Database} from "bun:sqlite";
import {test,expect} from "bun:test";
import {PNG} from "pngjs";
import fixture from "../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json";
import {parsePngSnapshot,defaultPngSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {encodePngSnapshot,decodePngSnapshot} from "../../../💾️binary/📸️snapshot/🟦️.ts";
import {PNG_SQLITE_SCHEMA,pngSnapshotToSqliteDatabase,pngSnapshotFromSqliteDatabase} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
test("PNG direct relational rows preserve precise samples, metadata and interpreted BLOB ancillary occurrences",async()=>{
 expect(PNG_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../🗄️.sql",import.meta.url)).text());
 for(const row of fixture.cases){const snapshot=parsePngSnapshot(row.snapshot),db=Database.deserialize(await exportSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot)));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT value FROM png_sample ORDER BY ordinal").all().map((r:any)=>r.value)).toEqual(snapshot.image.samples);expect(db.query("SELECT COUNT(*) AS count FROM sqlite_schema WHERE type='table'").get()).toEqual({count:13});expect(await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(snapshot);}finally{db.close();}}
});
test("PNG independent SQL sample edits publish pixels verified by pngjs",async()=>{
 const snapshot=defaultPngSnapshot(),db=Database.deserialize(await exportSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot)));try{db.run("UPDATE png_sample SET value=7 WHERE ordinal=0");const result=await pngSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()));snapshot.image.samples[0]=7;expect(result).toEqual(snapshot);const native=await encodePngSnapshot(result);expect(PNG.sync.read(Buffer.from(native)).data[0]).toBe(7);expect(await decodePngSnapshot(native)).toEqual(result);}finally{db.close();}
});
test("PNG source authority, incomplete grids and inconsistent metadata refuse before publication",async()=>{
 const snapshot=defaultPngSnapshot();
 for(const edit of ["DELETE FROM png_sample WHERE ordinal=0","UPDATE png_sample SET image_id=9 WHERE ordinal=0","UPDATE png_sample SET ordinal=99 WHERE ordinal=0","UPDATE png_sample SET value=256 WHERE ordinal=0","UPDATE png_image SET color_type=5","UPDATE png_image SET width=4294967295,height=4294967295"]){const db=Database.deserialize(await exportSqliteDatabase(await pngSnapshotToSqliteDatabase(snapshot)));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(edit);await expect(pngSnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).rejects.toThrow();}finally{db.close();}}
 expect(()=>parsePngSnapshot({schema:"stdio.png",bytes:[]})).toThrow();
});
test("PNG direct relational ownership and cancellation precede field publication",async()=>{
 const source=defaultPngSnapshot(),database=await pngSnapshotToSqliteDatabase(source);
 for(const options of [{maxRows:0},{maxValueBytes:0},{maxAllocationBytes:1}]){await expect(pngSnapshotToSqliteDatabase(source,options)).rejects.toThrow();await expect(pngSnapshotFromSqliteDatabase(database,options)).rejects.toThrow();}
 for(const phase of ["project","reconstruct"]){const abort=new AbortController();let progress=0;const options={signal:abort.signal,onProgress:()=>{progress++;abort.abort();}};await expect(phase==="project"?pngSnapshotToSqliteDatabase(source,options):pngSnapshotFromSqliteDatabase(database,options)).rejects.toMatchObject({kind:"canceled"});expect(progress).toBe(1);}
});
