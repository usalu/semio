/** 👷️ Independent worker policy denial and original-owner conservation. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv/dist/2020.js";
import {applyPatch} from "fast-json-patch";
import {readFileSync} from "node:fs";
const fixture=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8"));
const schema=JSON.parse(readFileSync(new URL("../🔣️.json",import.meta.url),"utf8"));
test("worker retirement fixed policy refuses oversize original owners before effects",async()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(fixture.policy)).toBe(true);
 expect(validate({...fixture.policy,maximum_release_bytes:3145728})).toBe(false);
 const db=new Database(":memory:");
 db.run("CREATE TABLE demand(copy_bytes INTEGER,capacity_bytes INTEGER,release_bytes INTEGER,depth INTEGER)");
 const expected:boolean[]=[];
 for(const row of fixture.cases){
  db.run("DELETE FROM demand");
  db.run("INSERT INTO demand VALUES(?,?,?,?)",row.demand.copy_bytes,row.demand.capacity_bytes,row.demand.release_bytes,row.demand.depth);
  const oracle=db.query("SELECT copy_bytes<=? AND capacity_bytes<=? AND release_bytes<=? AND depth<=? AS admitted FROM demand").get(fixture.policy.maximum_copy_bytes,fixture.policy.maximum_capacity_bytes,fixture.policy.maximum_release_bytes,fixture.policy.maximum_depth) as {admitted:number};
  expect(Boolean(oracle.admitted)).toBe(row.admitted);expected.push(Boolean(oracle.admitted));
  const original={owner:"original",capacity:row.demand.release_bytes,turns:0};
  const after=applyPatch(structuredClone(original),row.admitted?[{op:"replace",path:"/turns",value:1}]:[],true).newDocument;
  expect(after.owner).toBe(original.owner);expect(after.capacity).toBe(original.capacity);expect(after.turns).toBe(Number(row.admitted));
 }
 db.close();
 const implementation=await import("../🟦️.ts");
 expect(implementation.UI_WORKER_RETIREMENT_POLICY).toEqual(fixture.policy);
 for(let i=0;i<fixture.cases.length;i++)expect(implementation.uiWorkerRetirementPermits(fixture.cases[i].demand)).toBe(expected[i]);
 console.log(`[DEBUG] fixed worker policy original ownership: ${fixture.cases.length} SQLite/RFC6902 cases; release ceiling=${fixture.policy.maximum_release_bytes}`);
});

test("worker retirement structural refusal has an independent SQLite identity",async()=>{
 const db=new Database(":memory:");db.run("CREATE TABLE demand(copy_bytes INTEGER,capacity_bytes INTEGER,release_bytes INTEGER,depth INTEGER)");
 const implementation=await import("../🟦️.ts");
 for(const row of fixture.cases){
  db.run("DELETE FROM demand");db.run("INSERT INTO demand VALUES(?,?,?,?)",row.demand.copy_bytes,row.demand.capacity_bytes,row.demand.release_bytes,row.demand.depth);
  const oracle=db.query("SELECT CASE WHEN depth>? THEN 'depthLimit' WHEN capacity_bytes>? THEN 'ownershipLimit' WHEN release_bytes>? THEN 'ownershipLimit' WHEN copy_bytes>? THEN 'workLimit' ELSE NULL END AS reason FROM demand").get(fixture.policy.maximum_depth,fixture.policy.maximum_capacity_bytes,fixture.policy.maximum_release_bytes,fixture.policy.maximum_copy_bytes) as {reason:string|null};
  expect(implementation.uiWorkerRetirementRefusal(row.demand)).toBe(oracle.reason);
 }
 expect(fixture.refusal.owner).toBe("retain");expect(fixture.refusal.automaticRetry).toBe(false);db.close();
 console.log("[DEBUG] independent SQLite worker refusal identity matches the original fixed policy");
});
