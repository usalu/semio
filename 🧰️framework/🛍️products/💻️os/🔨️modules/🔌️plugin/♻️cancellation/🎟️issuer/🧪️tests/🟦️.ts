import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
test("canonical cancellation issuer admits only its exact original creation source",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 for(const row of fixture.cases){
  const database=new Database(":memory:");database.exec("CREATE TABLE births (phase TEXT)");
  const outcome=JSON.stringify(row.sourceIdentity)!==JSON.stringify(fixture.sourceIdentity)?"foreign":row.cancelled?"cancelled":row.items===0||row.capacityBytes===0?"denied":"admitted";
  for(const phase of fixture.phases){database.exec("BEGIN");database.query("INSERT INTO births VALUES (?)").run(phase);database.exec(outcome==="admitted"||outcome==="denied"&&row.items>0&&phase==="sourceLoan"?"COMMIT":"ROLLBACK")}
  expect(outcome).toBe(row.expected);expect(database.query("SELECT phase FROM births").all()).toEqual(row.expectedPhases.map(phase=>({phase})));database.close();
 }
 const issuer=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(issuer.includes("admit_step")).toBe(true);expect(issuer.includes("OriginalCallerSourceParent")).toBe(true);const canonical=readFileSync(resolve(import.meta.dir,"../../🦀️.rs"),"utf8");expect(canonical.includes("impl Default for ToolCancellationHandle")).toBe(false);expect(canonical.includes("CancelToken::root_now()")).toBe(false);
 console.log("[DEBUG] cancellationIssuerExactCreationIdentity=true zeroLoanRetains=true SQLiteAjv=true");
});

test("lease phases retain their exact key before every denied or foreign-key turn",()=>{
 for(const row of fixture.lease.cases){
  const db=new Database(":memory:");db.exec("CREATE TABLE pending (id INTEGER, phase INTEGER)");
  let key:number|undefined;let sourceOwner:number|undefined;let phase=0;
  for(let index=0;index<row.keys.length;index++){const incoming=row.keys[index]!;if(key!==undefined&&(key!==incoming||sourceOwner!==row.sourceOwners[index]))continue;if(!row.admitted[index])continue;key??=incoming;sourceOwner??=row.sourceOwners[index];phase++;db.exec("DELETE FROM pending");db.query("INSERT INTO pending VALUES (?,?)").run(key,phase);}
  expect(db.query("SELECT id FROM pending").get()).toEqual({id:row.expectedKey});expect(phase===fixture.lease.phases.length).toBe(row.expectedPublished);db.close();
 }
 const source=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("fn begin_keyed(&mut self,key:&ToolOperationKey,cx:")).toBe(true);expect(source.includes("PendingLease")).toBe(true);
 console.log("[DEBUG] canonical lease exact retained key denies foreign retry; SQLite independent transaction oracle");
});

test("renewal returns displaced parent aliases to the retained creation witness",()=>{
 const db=new Database(":memory:");for(const row of fixture.renewal){const result=db.query("SELECT COALESCE(? = ? AND ? = ?,0) AS admitted").get(row.creationRoot,row.witness,row.displacedParent,row.witness);expect(result).toEqual({admitted:Number(row.expected)});}db.close();
 const source=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("parts.close_step(self.issued.as_ref().unwrap(),self.admission.source.as_ref().unwrap().cancellation()?,grant)")).toBe(true);
 console.log("[DEBUG] renewal exact original creation-root witness; SQLite null/foreign refusal oracle");
});

test("inline claim custody posts no physical scaffold birth",()=>{
 const db=new Database(":memory:");const row=fixture.retirementInlineTransfer;expect(db.query("SELECT ? AS physicalBirth").get(row.capacityBytes)).toEqual({physicalBirth:row.expectedPhysicalBirth});db.close();
 const source=readFileSync(resolve(import.meta.dir,"../🦀️.rs"),"utf8");expect(source.includes("progress.retained_capacity_bytes=demand.capacity_bytes")).toBe(false);
 console.log("[DEBUG] inline typed claim custody capacity0; later actual scaffold receives separate grant and receipt");
});
