/** 🧪️ Space's neutral full unsigned64 state agrees with an independently authored SQLite file. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import{exportSqliteDatabase,importSqliteDatabase}from"../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json" with {type:"json"};
import {parseSSpaceArtifact} from "../../../🟦️.ts";
import type{CreateArtifact}from"../../../🧬️mutations/🌱create-artifact/🟦️.ts";
import type{TouchArtifact}from"../../../🧬️mutations/🕒touch-artifact/🟦️.ts";
const schema=await Bun.file(new URL("../../🪶️sqlite/🗄️.sql",import.meta.url)).text();
function snapshot(){return{...fixture.snapshot,artifacts:fixture.snapshot.artifacts.map(row=>({...row,createdAtMs:BigInt(row.createdAtMs),updatedAtMs:BigInt(row.updatedAtMs)}))};}
test("Space timestamps preserve every native unsigned64 occurrence and duplicate ID",()=>{
 const owned=snapshot();const independent=new Database(":memory:");try{
  independent.exec(schema);independent.run("INSERT INTO space_document VALUES(1,?,?)",[owned.schema,owned.spaceId]);
  const insert=independent.prepare("INSERT INTO space_artifact VALUES(?,?,?,?,?,?,?,?,?,?,?,?,?)");
  for(const[index,row]of owned.artifacts.entries())insert.run(index+1,1,index,row.id,row.name,row.kindId,row.schema,Number(row.createdAtMs>>32n),Number(row.createdAtMs&4294967295n),row.createdBy,Number(row.updatedAtMs>>32n),Number(row.updatedAtMs&4294967295n),row.updatedBy);
  expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  const rows=independent.query("SELECT artifact_id,created_at_ms_high AS high,created_at_ms_low AS low FROM space_artifact ORDER BY ordinal").all()as{artifact_id:string;high:number;low:number}[];
  expect(rows.map(row=>(BigInt(row.high)<<32n)|BigInt(row.low))).toEqual(owned.artifacts.map(row=>row.createdAtMs));
  expect(parseSSpaceArtifact(owned)).toEqual(owned);
  const create:CreateArtifact={artifact:owned.artifacts[0]!},touch:TouchArtifact={id:create.artifact.id,updatedAtMs:create.artifact.updatedAtMs,updatedBy:create.artifact.updatedBy};expect(touch.updatedAtMs).toBe(9007199254740993n);
  for(const value of fixture.invalidTimes)expect(()=>parseSSpaceArtifact({...owned,artifacts:[{...owned.artifacts[0],createdAtMs:BigInt(value)}]})).toThrow();
 }finally{independent.close();}
});
test("Space entity joins preserve order, words and independent SQL edits and surrogate aliases",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts"),owned=snapshot();
 const database=await own.spaceSnapshotToSqliteDatabase(owned);
 expect(database.tables.map(table=>table.sql+";\n").join("")).toBe(schema);
 const independent=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(independent.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(independent.query("SELECT a.artifact_id AS id,d.artifact_kind AS kind FROM space_artifact a JOIN space_artifact_dialect d ON d.id=a.id ORDER BY a.ordinal").all()).toEqual(owned.artifacts.map(row=>({id:row.id,kind:row.dialect.artifactKind})));
  independent.exec("BEGIN");for(const sql of fixture.renumberSql)independent.exec(sql);independent.exec("COMMIT");
  const renamed=await importSqliteDatabase(independent.serialize());
  expect(await own.spaceSnapshotFromSqliteDatabase(renamed)).toEqual(owned);
  await own.validateSpaceSnapshotSqliteDialect(owned,fixture.dialect,renamed);
  independent.query("UPDATE space_artifact SET name=?,created_at_ms_high=?,created_at_ms_low=? WHERE ordinal=0").run(fixture.editedName,fixture.editedCreatedHigh,fixture.editedCreatedLow);
  const changed=await importSqliteDatabase(independent.serialize());
  const restored=await own.spaceSnapshotFromSqliteDatabase(changed);
  expect(restored.artifacts[0]!.name).toBe(fixture.editedName);
  expect(restored.artifacts[0]!.createdAtMs).toBe((BigInt(fixture.editedCreatedHigh)<<32n)|BigInt(fixture.editedCreatedLow));
  await expect(own.validateSpaceSnapshotSqliteDialect(owned,fixture.dialect,changed)).rejects.toThrow();
 }finally{independent.close();}
});
test("Space independently authored SQL admits complete state and refuses broken occurrence ownership",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts"),independent=new Database(":memory:");try{
  independent.exec(schema);independent.exec("PRAGMA application_id=1397576526;PRAGMA user_version=1;INSERT INTO space_document VALUES(-17,'','');INSERT INTO space_artifact VALUES(-100,-17,0,'','','','',4294967295,4294967295,'',0,0,'');INSERT INTO space_artifact_dialect VALUES(-100,'','','')");
  expect((await own.spaceSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).artifacts[0]!.createdAtMs).toBe(18446744073709551615n);
  independent.exec("UPDATE space_artifact SET ordinal=1");await expect(own.spaceSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).rejects.toThrow();
  independent.exec("UPDATE space_artifact SET ordinal=0;PRAGMA ignore_check_constraints=ON;UPDATE space_artifact SET updated_at_ms_high=4294967296");await expect(own.spaceSnapshotFromSqliteDatabase(await importSqliteDatabase(independent.serialize()))).rejects.toThrow();
 }finally{independent.close();}
});
test("Space exact coordinates and bounded stages cover large ordered collections",async()=>{
 const own=await import("../../🪶️sqlite/🟦️.ts"),owned=snapshot(),database=await own.spaceSnapshotToSqliteDatabase(owned);
 for(const dialect of fixture.invalidDialects)await expect(own.validateSpaceSnapshotSqliteDialect(owned,dialect,database)).rejects.toThrow();
 for(const options of[{maxRows:4},{maxValueBytes:8}]){await expect(own.spaceSnapshotToSqliteDatabase(owned,options)).rejects.toThrow();await expect(own.spaceSnapshotFromSqliteDatabase(database,options)).rejects.toThrow();}
 const controller=new AbortController();controller.abort();await expect(own.spaceSnapshotToSqliteDatabase(owned,{signal:controller.signal})).rejects.toThrow();
 const many={...owned,artifacts:Array.from({length:fixture.deepRepeat},()=>({...owned.artifacts[0]!}))};const cancelled=new AbortController();let interior=false;
 await expect(own.spaceSnapshotToSqliteDatabase(many,{signal:cancelled.signal,onProgress(event){if(event.phase==="projectSnapshot"&&event.completed>=256&&event.completed<event.total){interior=true;cancelled.abort();}}})).rejects.toThrow();expect(interior).toBe(true);
 const manyDatabase=await own.spaceSnapshotToSqliteDatabase(many),readCancelled=new AbortController();let readInterior=false;
 await expect(own.spaceSnapshotFromSqliteDatabase(manyDatabase,{signal:readCancelled.signal,onProgress(event){if(event.phase==="reconstructSnapshot"&&event.completed>=256&&event.completed<event.total){readInterior=true;readCancelled.abort();}}})).rejects.toThrow();expect(readInterior).toBe(true);
});
