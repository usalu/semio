import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";
import failureSchema from "../🧬️schema/⚠️frontier.json";
import failureFixture from "../🧫️fixtures/⚠️frontier.json";

test("failed original frontiers author work without inventing copy or backing effects",()=>{
  expect(new Ajv({strict:true,allErrors:true}).compile(failureSchema)(failureFixture)).toBe(true);
  const database=new Database(":memory:");
  try{
    const value=JSON.parse(failureFixture.sources[0]) as string;
    expect((database.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(value) as {bytes:number}).bytes).toBe(failureFixture.copyPrefixBytes);
    expect(()=>JSON.parse(failureFixture.sources[3])).toThrow();
    for(const row of failureFixture.refusals){expect(row.copiedItems).toBe(1);expect([row.copiedBytes,row.retainedCapacityBytes,row.releasedBytes]).toEqual([0,0,0]);}
  }finally{database.close();}
  const source=readFileSync(resolve(import.meta.dir,"../../../🦀️.rs"),"utf8"),start=source.indexOf("fn step_source<"),end=source.indexOf("fn advance<",start),body=source.slice(start,end);
  expect(body).toContain("operation_result?");
  expect(body).not.toContain("copied_bytes:demand.copy_bytes");
  expect(body).not.toContain("checked_add(error.retained_progress())");
});

test("post-effect cancellation receipts retain independently authored work",()=>{
  const validate=new Ajv({strict:true,allErrors:true}).compile(schema);
  expect(validate(fixture)).toBe(true);
  const database=new Database(":memory:");
  try{
    const value=JSON.parse(fixture.source) as string;
    const independent=database.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(value) as {bytes:number};
    expect(independent.bytes).toBe(4);
    for(const row of fixture.cases){
      expect(row.receipt.copiedItems).toBe(row.grant.maximumItems);
      expect(row.receipt.copiedBytes).toBeLessThanOrEqual(row.grant.maximumCopyBytes);
      expect(row.receipt.retainedCapacityBytes).toBeLessThanOrEqual(row.grant.maximumCapacityBytes);
      expect(row.receipt.releasedBytes).toBeLessThanOrEqual(row.grant.maximumReleaseBytes);
      if(row.frontier==="stringCopy")expect(row.receipt.copiedBytes).toBe(independent.bytes);
      if(row.frontier==="stringCapacity")expect(row.receipt.retainedCapacityBytes).toBe(independent.bytes);
      for(const axis of Object.keys(row.grant)){
        const missing=structuredClone(fixture) as Record<string,unknown>;
        delete (missing.preparationGrant as Record<string,unknown>)[axis];
        expect(validate(missing)).toBe(false);
      }
    }
  }finally{database.close();}
  console.log("[DEBUG] JSON post-effect receipt3 strict schema and independent SQLite UTF8 byte ownership passed");
});

test("actual grammar publishes physical progress before the post-effect cancellation checkpoint",()=>{
  const source=readFileSync(resolve(import.meta.dir,"../../../🦀️.rs"),"utf8");
  const start=source.indexOf("fn step_source<"),end=source.indexOf("fn advance<",start);
  expect(start>=0&&end>start).toBe(true);
  const body=source.slice(start,end),receipt=body.indexOf("self.normal_step_progress=self.normal_step_progress.checked_add("),checkpoint=body.indexOf("control.step()?");
  expect(receipt>=0&&checkpoint>receipt).toBe(true);
});
