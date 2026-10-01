import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {dwgSnapshotFixture as snapshot} from "../../🧫️fixtures/🪶️sqlite/🟦️.ts";
import fixture from "../../🧫️fixtures/🪶️sqlite/🎭️bodies/🔣️.json";
import {dwgSnapshotToSqliteDatabase,dwgSnapshotFromSqliteDatabase,dwgSnapshotValidateSqliteSubset} from "../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

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
