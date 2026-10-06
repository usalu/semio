import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {dwgSnapshotFixture as snapshot} from "🧰️support/🟦️.ts";
import fixture from "../🧫️fixtures/🎭️bodies/🔣️.json";
import encodingFixture from "../🧫️fixtures/📏️encoding.json";
import {dwgSnapshotToSqliteDatabase,dwgSnapshotFromSqliteDatabase,dwgSnapshotValidateSqliteSubset} from "../🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

test("DWG semantic guard admits relational aliases and rejects independently edited state",async()=>{
  const database=await dwgSnapshotToSqliteDatabase(snapshot),native=Database.deserialize(await exportSqliteDatabase(database));
  try{
    native.run("UPDATE dwg_annotation_scale SET name='guard must compare owned entity state'");
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const edited=await importSqliteDatabase(native.serialize());
    for(const dialect of fixture.sqliteDialects)await expect(dwgSnapshotValidateSqliteSubset(snapshot,dialect,edited)).rejects.toThrow();
    const restored=await dwgSnapshotFromSqliteDatabase(edited);for(const dialect of fixture.sqliteDialects)await dwgSnapshotValidateSqliteSubset(restored,dialect,edited);
    const identity=(native.query("SELECT id FROM dwg_object WHERE body_kind='annotation_scale'").get() as {id:number}).id;
    native.run("UPDATE dwg_object_reactor_handle SET object_id=-17 WHERE object_id=?",[identity]);native.run("UPDATE dwg_object_referenced_handle SET object_id=-17 WHERE object_id=?",[identity]);native.run("UPDATE dwg_extended_entity_data SET object_id=-17 WHERE object_id=?",[identity]);native.run("UPDATE dwg_annotation_scale SET id=-17");native.run("UPDATE dwg_object SET id=-17 WHERE id=?",[identity]);
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const aliased=await importSqliteDatabase(native.serialize());expect(await dwgSnapshotFromSqliteDatabase(aliased)).toEqual(restored);for(const dialect of fixture.sqliteDialects)await dwgSnapshotValidateSqliteSubset(restored,dialect,aliased);
  }finally{native.close();}
},60000);

test("DWG shared native-control corpus retains exact Unicode and independently counted domain rows",async()=>{
  const candidate={...snapshot,schema:encodingFixture.controlledNative.text.repeat(encodingFixture.controlledNative.repeat)},database=await dwgSnapshotToSqliteDatabase(candidate),native=Database.deserialize(await exportSqliteDatabase(database));
  try{
    expect(native.query("SELECT schema FROM dwg_document").get()).toEqual({schema:candidate.schema});
    const names=native.query("SELECT name FROM sqlite_schema WHERE type='table'").all() as {name:string}[];
    const rows=names.reduce((total,{name})=>total+(native.query("SELECT COUNT(*) AS n FROM "+name).get() as {n:number}).n,0);
    expect(rows).toBe(database.tables.reduce((total,table)=>total+table.rows.length,0));expect(rows).toBeGreaterThan(candidate.drawing.objects.length+1);
    await expect(dwgSnapshotToSqliteDatabase(candidate,{maxRows:rows-1})).rejects.toThrow();await expect(dwgSnapshotFromSqliteDatabase(database,{maxRows:rows-1})).rejects.toThrow();
  }finally{native.close();}
},60000);

test("DWG complete owned snapshot keeps all body families exact through independent SQLite",async()=>{
  const database=await dwgSnapshotToSqliteDatabase(snapshot);const native=Database.deserialize(await exportSqliteDatabase(database));
  try{
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(native.query("SELECT DISTINCT body_kind FROM dwg_object ORDER BY body_kind").all()).toEqual([...fixture.bodyKinds].sort().map(body_kind=>({body_kind})));
    expect(await dwgSnapshotFromSqliteDatabase(await importSqliteDatabase(native.serialize()))).toEqual(snapshot);
    native.run("UPDATE dwg_annotation_scale SET name='edited independently'");const edited=await dwgSnapshotFromSqliteDatabase(await importSqliteDatabase(native.serialize()));
    const annotation=edited.drawing.objects.find(object=>object.body?.kind==="annotationScale")!.body!;if(annotation.kind!=="annotationScale")throw new Error("kind");expect(annotation.value.name).toBe("edited independently");
    console.log("[DEBUG] DWG complete277-table typed snapshot roundtrip and independent relational edit verified");
  }finally{native.close();}
},60000);

test("DWG complete provider cancels during expensive projection and reconstruction",async()=>{
  const database=await dwgSnapshotToSqliteDatabase(snapshot),project=new AbortController(),reconstruct=new AbortController();
  let projected=0,reconstructed=0;
  await expect(dwgSnapshotToSqliteDatabase(snapshot,{signal:project.signal,onProgress:progress=>{if(progress.phase==="projectSnapshot"&&progress.completed>=256){projected=progress.completed;project.abort();}}})).rejects.toThrow();
  await expect(dwgSnapshotFromSqliteDatabase(database,{signal:reconstruct.signal,onProgress:progress=>{if(progress.phase==="reconstructSnapshot"&&progress.completed>=256){reconstructed=progress.completed;reconstruct.abort();}}})).rejects.toThrow();
  expect(projected).toBeGreaterThanOrEqual(256);expect(reconstructed).toBeGreaterThanOrEqual(256);
  await expect(dwgSnapshotToSqliteDatabase(snapshot,{maxValueBytes:1})).rejects.toThrow();await expect(dwgSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();
  console.log("[DEBUG] DWG actual mid-operation progress cancellation and aggregate data bounds verified");
},60000);

test("DWG exact AC1018/AC1024 guards, identity, budgets and cancellation are observable",async()=>{
  const database=await dwgSnapshotToSqliteDatabase(snapshot);
  for(const dialect of fixture.sqliteDialects)await dwgSnapshotValidateSqliteSubset(snapshot,dialect,database);
  for(const dialect of fixture.invalidSqliteDialects)await expect(dwgSnapshotValidateSqliteSubset(snapshot,dialect,database)).rejects.toThrow();
  await expect(dwgSnapshotValidateSqliteSubset({...snapshot,version:"different"},{artifactKind:"s.stdio.dwg",standard:"ac1024",subset:"*"},database)).rejects.toThrow();
  await expect(dwgSnapshotToSqliteDatabase(snapshot,{maxRows:1})).rejects.toThrow();await expect(dwgSnapshotFromSqliteDatabase(database,{maxRows:1})).rejects.toThrow();
  const controller=new AbortController();controller.abort();await expect(dwgSnapshotToSqliteDatabase(snapshot,{signal:controller.signal})).rejects.toThrow();await expect(dwgSnapshotFromSqliteDatabase(database,{signal:controller.signal})).rejects.toThrow();await expect(dwgSnapshotValidateSqliteSubset(snapshot,{artifactKind:"s.stdio.dwg",standard:"ac1024",subset:"*"},database,{signal:controller.signal})).rejects.toThrow();
  console.log("[DEBUG] DWG exact two-version guard plus identity, resource and cancellation refusals verified");
},60000);
