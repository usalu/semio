import { test, expect } from "bun:test";
import Ajv from "ajv";
import { applyPatch } from "fast-json-patch";
import {consumeRetainedProgress,remainingRetainedGrant} from "../../../⏱️budget/🟨️.js";
import { readFileSync, existsSync } from "node:fs";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };

test("retained step context neutral ledger alias and whole-frame authority", () => {
  for (const row of fixture.cases) {
    const original = { operation: row.operation, generation: row.generation, aliases: 1, ledger: true };
    for (let turn = 0; turn < row.turns; turn++) {
      const held = applyPatch(structuredClone(original), [{ op: "replace", path: "/aliases", value: 2 }], true).newDocument;
      expect(held.operation).toBe(original.operation);
      expect(held.generation).toBe(original.generation);
      expect(applyPatch(held, [{ op: "replace", path: "/aliases", value: 1 }], true).newDocument).toEqual(original);
    }
    expect(applyPatch(structuredClone(original), [{ op: "replace", path: "/ledger", value: false }, { op: "replace", path: "/aliases", value: 0 }], true).newDocument.ledger).toBe(false);
  }
});

test("retained context owns one ledger and uses atomic unique teardown", () => {
  const path = new URL("../🦀️.rs", import.meta.url);
  expect(existsSync(path)).toBe(true);
  if (!existsSync(path)) return;
  const source = readFileSync(path, "utf8");
  for (const method of ["birth_bytes", "context", "next_close_copy_byte_demand", "next_close_capacity_byte_demand", "next_close_release_byte_demand", "next_close_depth_demand", "close_step", "terminal_is_empty"]) expect(source).toContain(method);
  expect(fixture.closeGrant).toEqual({copyBytes:0,capacityBytes:0,release:"exact-original-ledger-birth",depth:1,terminalReceipt:"complete-original-progress"});
  expect(source).toContain("Arc::try_unwrap");
  expect(source).toContain("StepContext::with_payload_ledger");
  expect(source).not.toContain("Arc::strong_count");
});


test("original job budget consumes only actual independent retained receipts", () => {
  const law = JSON.parse(readFileSync(new URL("../../../⏱️budget/🧫️fixtures/🔣️.json", import.meta.url), "utf8")).retained;
  let original = {...law.grant};let actual={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0};
  for (const receipt of law.receipts) {
    const next = applyPatch({...original}, [
      {op:"replace", path:"/maximumItems", value:original.maximumItems-receipt.copiedItems},
      {op:"replace", path:"/maximumCopyBytes", value:original.maximumCopyBytes-receipt.copiedBytes},
      {op:"replace", path:"/maximumCapacityBytes", value:original.maximumCapacityBytes-receipt.retainedCapacityBytes},
      {op:"replace", path:"/maximumReleaseBytes", value:original.maximumReleaseBytes-receipt.releasedBytes},
    ], true).newDocument;
    for(const value of Object.values(next)) expect(value).toBeGreaterThanOrEqual(0);
    expect(next.maximumDepth).toBe(law.grant.maximumDepth);actual=consumeRetainedProgress(law.grant,actual,receipt);expect(remainingRetainedGrant(law.grant,actual)).toEqual(next);original=next;
  }
  expect(original).toEqual(law.remaining);
  expect(Buffer.from(law.source).byteLength).toBe(law.receipts[1].copiedBytes);
  const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");
  for(const method of ["pub retained: RetainedCloneGrant","pub fn retained_grant","pub fn consume_retained","pub fn retained_progress"])expect(source).toContain(method);
  expect(source).toContain("budget.retained");
  console.log("[DEBUG] original job neutral retained wallet RFC6902 and UTF8 byte oracle preserve all independent axes beside fuel/deadline");
});

test("retained context constructor accepts the original full grant and returns its actual birth receipt",()=>{
 expect(fixture.birthGrant).toEqual({copyBytes:0,capacity:"exact-original-ledger-birth",releaseBytes:0,depth:1,receipt:"same-actual-allocator-birth",deniedAxes:["items","capacity","depth"]});
 const source=readFileSync(new URL("../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>");expect(source).toContain("grant.maximum_depth==0");expect(source).toContain("retained_capacity_bytes:Self::birth_bytes()");
});

test("original refused Job close retains all actual failed currencies",()=>{
 const source=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");expect(source).toContain("Refused { kind:ValueRefusalKind, progress:RetainedCloneProgress }");expect(source).toContain("progress:error.retained_progress()");
 const actual=applyPatch({copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0},[{op:"replace",path:"/copiedItems",value:1},{op:"replace",path:"/retainedCapacityBytes",value:64}],true).newDocument;expect(actual).toEqual(fixture.failedCloseReceipt);
});

test("original failed Job ingress preserves same actual external receipt before refusal",()=>{
 const law=JSON.parse(readFileSync(new URL("../⚠️failure/🧫️fixtures/🔣️.json",import.meta.url),"utf8")),schema=JSON.parse(readFileSync(new URL("../⚠️failure/🧬️schema/🔣️.json",import.meta.url),"utf8"));expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);
 const original=Buffer.alloc(law.producerBytes),pointer=original.buffer,grant={maximumItems:1,maximumCopyBytes:0,maximumCapacityBytes:law.incomingCapacityBytes,maximumReleaseBytes:64,maximumDepth:3},recipient={copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0},actual={copiedItems:1,copiedBytes:0,retainedCapacityBytes:original.byteLength,releasedBytes:0};let error:unknown;
 try{consumeRetainedProgress(grant,recipient,actual);}catch(value){error=value;}expect(error).toBeInstanceOf(RangeError);expect((error as {retainedProgress:typeof actual}).retainedProgress).toBe(actual);expect(recipient).toEqual(actual);expect(remainingRetainedGrant(grant,recipient).maximumCapacityBytes).toBe(law.remainingCapacityBytes);expect(original.buffer).toBe(pointer);
 expect(applyPatch({receipt:recipient,original:true},[{op:"replace",path:"/original",value:false}],true,false).newDocument.receipt).toEqual(actual);
});
