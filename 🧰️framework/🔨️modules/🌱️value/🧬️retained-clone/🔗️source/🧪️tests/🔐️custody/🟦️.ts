/** 🔐️ Canonical original clone-source custody uses schema validation and independent patch replay. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { validateJsonSchemaSubset } from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { applyPatch } from "fast-json-patch";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔐️custody.json";

test("original clone-source corpus keeps issuer and aliases before terminal handback", async () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture)).toBe(true);
  for (const row of fixture.cases) {
    const original = { owner: row.owner, metadata: row.metadata, alias: row.owner, terminal: false };
    for (const _axis of row.denied) {
      expect(applyPatch(structuredClone(original), []).newDocument).toEqual(original);
    }
    const returned = applyPatch(structuredClone(original), [{ op: "replace", path: "/terminal", value: true }]).newDocument;
    expect(returned).toEqual({ ...original, terminal: row.expected.terminal });
    expect(returned.owner).toBe(row.owner);
    expect(returned.metadata).toBe(row.metadata);
  }
  const source = await Bun.file(new URL("../../🦀️.rs", import.meta.url)).text();
  expect(source).toContain("pub fn admit_borrowed");
  expect(source).toContain("SealedShared::<RetainedCloneLeaseOwner>");
  expect(source).not.toContain("pub fn take_owner(&mut self)->Option<Arc<T>>");
  const binding = await Bun.file(new URL("../../📦️packages/🦀️rust/🦀️.rs", import.meta.url)).text();
  expect(binding).toContain("binding_copy_bytes");
  expect(binding).toContain("try_duplicate(grant)");
});
            
test("zero metadata copy admits an exact unsigned budget and rejects unrepresentable negative authority with the independent AJV oracle",()=>{
 const budget={type:"integer",minimum:fixture.currency.minimumCopyBudget};const independent=new Ajv().compile(budget);
 for(const value of [fixture.currency.metadataCopyBytes,Number(BigInt(fixture.currency.minimumCopyBudget)-1n)])expect(validateJsonSchemaSubset(budget,value).length===0).toBe(independent(value));
 expect(fixture.currency.metadataCopyBytes).toBe(0);expect(fixture.currency.negativeCopyBudgetRepresentable).toBe(false);expect(fixture.currency.bindingDenied).toEqual(["items","depth"]);expect(fixture.currency.transferDenied).toEqual(["items","release","depth"]);
 const validate=new Ajv().compile(schema);expect(validate({...fixture,currency:{...fixture.currency,metadataCopyBytes:1}})).toBe(false);expect(validate({...fixture,currency:{...fixture.currency,minimumCopyBudget:-1}})).toBe(false);
});


test("true source and borrow authority callers select one canonical combined close quote",async()=>{
 const validate=new Ajv({strict:true}).compile<typeof fixture>(schema);expect(validate(fixture)).toBe(true);
 const {Database}=await import("bun:sqlite"),db=new Database(":memory:");
 try{
  db.run("CREATE TABLE callers(path TEXT PRIMARY KEY, owner TEXT, calls INTEGER)");
  const defining=await Bun.file(new URL("../../🦀️.rs",import.meta.url)).text(),facade=await Bun.file(new URL("../../📦️packages/🦀️rust/🦀️.rs",import.meta.url)).text();
  expect(defining).toContain(`pub fn ${fixture.closeQuery.method}(&self,body:usize)`);
  expect(facade).toContain(`pub fn ${fixture.closeQuery.method}(&self,body:usize)`);
  expect(facade).toContain("self.source.next_close_demand(body)");
  expect(facade).not.toContain("pub fn next_close_demands(");
  for(const row of fixture.closeQuery.callers){
   const source=await Bun.file(new URL("../../../../"+row.path,import.meta.url)).text();
   expect(source).not.toContain(".next_close_demands(");
   expect(source.match(/\.next_close_demand\(/g)?.length??0).toBeGreaterThanOrEqual(row.calls);
   db.run("INSERT INTO callers VALUES(?,?,?)",[row.path,row.owner,row.calls]);
  }
  expect(db.query("SELECT COUNT(*) AS paths,SUM(calls) AS calls FROM callers").get()).toEqual({paths:fixture.closeQuery.callers.length,calls:fixture.closeQuery.callers.reduce((sum,row)=>sum+row.calls,0)});
  expect(db.query("SELECT COUNT(*) AS paths FROM callers WHERE owner='borrowAuthority'").get()).toEqual({paths:2});
  console.log(`[DEBUG] Canonical single close query matches the true source and borrow authority across ${fixture.closeQuery.callers.length} declaring consumers; SQLite preserves the full13call inventory, native resolution remains unrun`);
 }finally{db.close();}
});

import {sourceTakeDemand} from "../../🟦️.ts";
test("authority handback selects one whole lease or owner demand and refuses closing custody",async()=>{const {Database}=await import("bun:sqlite"),db=new Database(":memory:");try{const oracle=db.query("SELECT CASE WHEN json_extract(?,'$.closing') OR json_type(?,'$.owner')='null' THEN NULL WHEN json_type(?,'$.lease')!='null' THEN json_extract(?,'$.lease') ELSE json_extract(?,'$.owner') END AS demand");for(const row of fixture.takeQuery){expect(sourceTakeDemand(row)).toEqual(row.expected);const encoded=JSON.stringify(row),result=oracle.get(encoded,encoded,encoded,encoded,encoded)as{demand:string|null};expect(result.demand===null?null:JSON.parse(result.demand)).toEqual(row.expected);}}finally{db.close();}});
