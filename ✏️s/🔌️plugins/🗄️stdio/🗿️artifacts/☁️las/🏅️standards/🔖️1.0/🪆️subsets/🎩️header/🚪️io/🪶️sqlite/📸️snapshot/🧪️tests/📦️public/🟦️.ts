import {expect,test} from "bun:test";
import {LAS_SQLITE_SCHEMA,lasSnapshotToSqliteDatabase,lasSnapshotFromSqliteDatabase,lasSnapshotValidateSqliteSubset,type LasSnapshot} from "@semio-tech/stdio-las";
import {lasSnapshotFixture} from "../🧰️support/🟦️.ts";
test("LAS built package exports its full typed semantic SQLite provider",async()=>{
  const snapshot:LasSnapshot=lasSnapshotFixture,database=await lasSnapshotToSqliteDatabase(snapshot);
  expect(database.tables.length).toBe(8);expect(LAS_SQLITE_SCHEMA).toContain("CREATE TABLE las_point_gps");
  const restored:LasSnapshot=await lasSnapshotFromSqliteDatabase(database);expect(restored).toEqual(snapshot);
  await lasSnapshotValidateSqliteSubset(restored,{artifactKind:"s.stdio.las",standard:"1.0",subset:"*"},database);
  console.log("[DEBUG] LAS package public typed model and all relational APIs verified");
},30000);
