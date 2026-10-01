import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import schema from "../../🔣️.json";
import type { GifSnapshot } from "../../🟦️.ts";
import { GIF87_SQLITE_SCHEMA, gifSnapshotToSqliteDatabase, gifSnapshotFromSqliteDatabase } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

test("GIF87 shared full snapshot corpus exposes editable indexed grids and palettes",async()=>{
 expect(new Ajv({strict:false}).validate(schema,fixture)).toBe(true);
 expect(GIF87_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text());
 const snapshot=structuredClone(fixture) as GifSnapshot;const database=await gifSnapshotToSqliteDatabase(snapshot);expect(await gifSnapshotFromSqliteDatabase(database)).toEqual(snapshot);
 const db=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(db.query("SELECT p.color_index,c.red FROM gif87_pixel p JOIN gif87_image i ON i.id=p.image_id JOIN gif87_global_color c ON c.ordinal=p.color_index WHERE i.ordinal=0 AND p.x=0").get()).toEqual({color_index:0,red:255});
  db.run("UPDATE gif87_global_color SET red=42 WHERE ordinal=0");db.run("UPDATE gif87_pixel SET color_index=1 WHERE image_id=1 AND x=0");snapshot.gct!.colors[0]!.r=42;snapshot.images[0]!.indices[0]=1;
  expect(await gifSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).toEqual(snapshot);
 }finally{db.close();}
});

test("GIF87 exact option presence, grid invariants and bounded cancellation",async()=>{
 const snapshot=structuredClone(fixture)as GifSnapshot;const database=await gifSnapshotToSqliteDatabase(snapshot);
 for(const change of ["UPDATE gif87_pixel SET image_id=999 WHERE id=1","UPDATE gif87_pixel SET x=0 WHERE image_id=1","DELETE FROM gif87_pixel WHERE id=1","UPDATE gif87_image SET ordinal=0 WHERE ordinal=1","UPDATE gif87_local_color SET palette_id=999","UPDATE gif87_global_color SET red=256 WHERE ordinal=0","UPDATE gif87_image SET interlace=2 WHERE ordinal=0"]){const db=Database.deserialize(await exportSqliteDatabase(database));try{db.run("PRAGMA ignore_check_constraints=ON");db.run(change);if(change.includes("local_color"))db.run("INSERT INTO gif87_local_color VALUES(1,999,0,0,0,0)");await expect(gifSnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();}finally{db.close();}}
 for(const gct of [null,{sorted:false,colors:[]}]){const value:GifSnapshot={schema:"custom 世界",width:4294967295,height:0,gct,backgroundColorIndex:255,pixelAspectRatio:255,images:[]};expect(await gifSnapshotFromSqliteDatabase(await gifSnapshotToSqliteDatabase(value))).toEqual(value);}
 await expect(gifSnapshotToSqliteDatabase(snapshot,{maxRows:1})).rejects.toThrow();await expect(gifSnapshotFromSqliteDatabase(database,{maxValueBytes:0})).rejects.toThrow();
 const controller=new AbortController();snapshot.images[0]!.width=2000;snapshot.images[0]!.indices=new Array<number>(2000).fill(1);await expect(gifSnapshotToSqliteDatabase(snapshot,{signal:controller.signal,onProgress:event=>{if(event.completed>=256)controller.abort();}})).rejects.toHaveProperty("name","AbortError");
});
