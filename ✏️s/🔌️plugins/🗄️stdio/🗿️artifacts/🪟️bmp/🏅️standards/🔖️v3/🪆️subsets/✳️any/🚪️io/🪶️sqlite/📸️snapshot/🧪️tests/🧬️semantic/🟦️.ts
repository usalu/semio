/** 🪶️ Owned BMP relational structure has no encoded carrier or padding ownership. */
import {Database} from "bun:sqlite";
import {test,expect} from "bun:test";
import {BMP_SQLITE_SCHEMA,bmpSnapshotToSqliteDatabase,bmpSnapshotFromSqliteDatabase} from "../../🟦️.ts";
import fixture from "../../../../../🧫️fixtures/🧬️owned-native-samples/🔣️.json";
import {parseBmpSnapshot} from "../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {exportSqliteDatabase} from "@semio-tech/framework";
test("BMP SQL contains only typed image metadata, precise sample variants and owned opaque occurrences",async()=>{
 const expected=["bmp_image","bmp_palette_entry","bmp_pixel_index","bmp_pixel_sample","bmp_gap_octet","bmp_trailer_octet"],db=new Database(":memory:");try{db.exec(BMP_SQLITE_SCHEMA);expect(db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY rowid").all().map((r:any)=>r.name)).toEqual(expected);}finally{db.close();}
 for(const row of fixture.cases){const snapshot=parseBmpSnapshot(row.snapshot),database=await bmpSnapshotToSqliteDatabase(snapshot),independent=Database.deserialize(await exportSqliteDatabase(database));try{expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(independent.query("SELECT COUNT(*) AS count FROM bmp_pixel_index").get()).toEqual({count:snapshot.image.pixels.storage==="indexed"?snapshot.image.pixels.indices.length:0});expect(independent.query("SELECT COUNT(*) AS count FROM bmp_pixel_sample").get()).toEqual({count:snapshot.image.pixels.storage==="direct"?snapshot.image.pixels.samples.length:0});expect(await bmpSnapshotFromSqliteDatabase(database)).toEqual(snapshot);}finally{independent.close();}}
});
