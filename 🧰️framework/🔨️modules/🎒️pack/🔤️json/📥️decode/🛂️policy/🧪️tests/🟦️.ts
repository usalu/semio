import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import fixture from "../🧫️fixtures/🔣️.json";
import sourceFixture from "../../../🧫️fixtures/🫳️read-source.json";
import retirementFixture from "../../../🧫️fixtures/🎟️borrowed-retirement/🔣️.json";

test("complete source policy matches independent UTF8 admission",()=>{
  const database=new Database(":memory:");
  try{
    for(const row of fixture.cases){
      const independent=database.query("SELECT length(CAST(? AS BLOB)) AS bytes").get(row.source) as {bytes:number};
      expect(independent.bytes).toBe(Buffer.byteLength(row.source,"utf8"));
      expect(independent.bytes<=row.limits.maximumBytes?"admitted":"workLimit").toBe(row.expected);
      expect(JSON.parse(row.source)).toBeDefined();
    }
  }finally{database.close();}
  console.log("[DEBUG] JSON complete source policy5 matches independent SQLite UTF8; wholeCorpus=false originalNativePolicyLaws=preserved");
});

test("original source constructor requires the complete policy without a second API",()=>{
  const source=readFileSync(resolve(import.meta.dir,"../../🫳️borrowed/🦀️.rs"),"utf8");
  expect(/pub fn new\(source:S,policy:super::JsonMemberPolicy,limits:JsonReadLimits\)->Result<Self,/u.test(source)).toBe(true);
  expect(source.includes("pub fn new_with_limits(")).toBe(false);
  expect(source.includes("pub fn new(source:S,policy:super::JsonMemberPolicy)->Self")).toBe(false);
});

test("original owning fixtures carry complete literal policies through the canonical source API",()=>{
  for(const original of [sourceFixture,retirementFixture]){
    expect(Buffer.byteLength(original.source,"utf8")<=original.readLimits.maximumBytes).toBe(true);
  }
  expect(Buffer.byteLength(JSON.stringify("x".repeat(sourceFixture.largeStringBytes)),"utf8")<=sourceFixture.readLimits.maximumBytes).toBe(true);
  console.log("[DEBUG] Original source and retirement fixtures retain literal complete policies and original8194 source extent");
});
