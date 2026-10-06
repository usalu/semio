import {expect,test} from "bun:test";
import {DWG_SQLITE_SCHEMA,dwgSnapshotToSqliteDatabase,dwgSnapshotFromSqliteDatabase,dwgSnapshotValidateSqliteSubset,type DwgSnapshot} from "@semio-tech/stdio-dwg";
import {dwgSnapshotFixture} from "../🧰️support/🟦️.ts";

test("DWG built package exposes the complete typed relational provider",async()=>{
  const snapshot:DwgSnapshot=dwgSnapshotFixture,database=await dwgSnapshotToSqliteDatabase(snapshot);
  expect(database.tables.length).toBe(277);expect(DWG_SQLITE_SCHEMA).toContain("CREATE TABLE dwg_constraint_node");
  const restored:DwgSnapshot=await dwgSnapshotFromSqliteDatabase(database);expect(restored).toEqual(snapshot);
  await dwgSnapshotValidateSqliteSubset(restored,{artifactKind:"s.stdio.dwg",standard:"ac1018",subset:"*"},database);
  await dwgSnapshotValidateSqliteSubset(restored,{artifactKind:"s.stdio.dwg",standard:"ac1024",subset:"*"},database);
  console.log("[DEBUG] Built DWG package public types and both version relational APIs verified");
},60000);
