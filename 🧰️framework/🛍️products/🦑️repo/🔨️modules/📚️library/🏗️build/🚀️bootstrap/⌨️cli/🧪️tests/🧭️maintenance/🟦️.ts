/** 🧭️ Qualifies only canonical declared maintenance identifiers without running a maintenance owner. */
import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import {readNativeRepoRecoveryPlanV1} from "../../🟦️.ts";
import protocolSchema from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";

const protocol=protocolSchema.$defs.Protocol.const;
const manifests=()=>[protocol.maintenance,protocol.reset].map(group=>({project:group.project,owner:group.owner,value:JSON.parse(readFileSync(join(process.cwd(),group.owner,"📋️project.json"),"utf8"))}));

test("canonical maintenance owner schemas match actual declared target identities",()=>{
 const rows=manifests(),ajv=new Ajv2020({strict:true});
 expect(ajv.validate(protocolSchema.$defs.MaintenanceOwner,rows[0]!.value)).toBe(true);
 expect(ajv.validate(protocolSchema.$defs.ResetOwner,rows[1]!.value)).toBe(true);
 for(const group of [protocol.maintenance,protocol.reset])expect(Object.keys(group.targets).sort()).toEqual(fixture.bindings.filter(row=>row.project===group.project).map(row=>row.target).sort());
});

test("neutral maintenance routing matches independent SQLite over actual manifest data",()=>{
 expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE specs(project TEXT,target TEXT,kind TEXT,operation TEXT);CREATE TABLE owners(project TEXT,doc TEXT);CREATE TABLE options(value TEXT)");
  for(const row of fixture.bindings)db.query("INSERT INTO specs VALUES (?,?,?,?)").run(row.project,row.target,row.kind,row.operation);
  for(const row of manifests())db.query("INSERT INTO owners VALUES (?,?)").run(row.project,JSON.stringify(row.value));
  for(const option of fixture.options)db.query("INSERT INTO options VALUES (?)").run(option);
  for(const row of fixture.cases){
   const reference=db.query("WITH args AS(SELECT key i,value FROM json_each(?1)) SELECT s.kind,s.operation FROM specs s JOIN owners o ON o.project=s.project JOIN json_each(o.doc,'$.targets') t ON t.key=s.target WHERE json_extract(?1,'$[0]')='run' AND json_extract(?1,'$[1]')=s.project||':'||s.target AND NOT EXISTS(SELECT 1 FROM args WHERE i>=2 AND value NOT IN(SELECT value FROM options))").get(JSON.stringify(row.args))as{kind:string;operation:string}|null;
   const result=readNativeRepoRecoveryPlanV1(process.cwd(),row.args);expect(reference?.kind??null,row.id).toBe(row.kind);expect(reference?.operation??null,row.id).toBe(row.operation);expect<unknown>(result?.kind??null,row.id).toBe(row.kind);expect<unknown>(result?.target??null,row.id).toBe(row.operation);if(result)expect(Object.isFrozen(result)).toBe(true);
  }
 }finally{db.close();}
});
