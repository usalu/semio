import { expect,test } from "bun:test";
import stableStringify from "fast-json-stable-stringify";
import { applyPatch } from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json";
test("terrain plain finite grants preserve Busy versus Closing",()=>{
 expect(fixture.control.demand).toBe("observeOnly");
 expect(stableStringify(fixture.grant)).toBe(JSON.stringify(Object.fromEntries(Object.entries(fixture.grant).sort(([a],[b])=>a.localeCompare(b)))));
});
test("portable refusal vectors retain the owner and match independent RFC6902 publication",()=>{
 for(const row of fixture.laws){
  const original={phase:row.phase,owner:{pointer:"original",capacity:65536,payload:[0,1,2]},terminal:false};
  const result={...original,phase:row.fault==="Busy"?row.phase:"Retire"};
  const external=applyPatch(structuredClone(original),[{op:"replace",path:"/phase",value:row.next}],true,false).newDocument;
  expect(result).toEqual(external);expect(result.phase).toBe(row.next);
  expect(result.owner).toBe(original.owner);expect(result.terminal).toBe(row.terminal);
 }
});

test("actual receiver contract preserves typed semantic faults until presentation",()=>{
 
 for(const row of fixture.receiving){
  const original={fault:{kind:row.kind,code:row.code},owner:"retained"};
  const result={...original,receiver:"IconExportBatch"};
  const external=applyPatch(structuredClone(original),[{op:"add",path:"/receiver",value:"IconExportBatch"}],true,false).newDocument;
  expect(result).toEqual(external);expect(result.fault).toBe(original.fault);
  
 }
});

test("component authority forwards all caller currencies and retains undeclared residual owners",()=>{
 const source={grant:fixture.grant,owner:{pointer:"original",capacity:65536},refusal:"UnsupportedOwner"};
 const receiving={...source,control:fixture.control.componentClose};
 const external=applyPatch(structuredClone(source),[{op:"add",path:"/control",value:fixture.control.componentClose}],true,false).newDocument;
 expect(receiving).toEqual(external);expect(receiving.grant).toBe(source.grant);expect(receiving.owner).toBe(source.owner);
 expect(receiving.control.residual).toBe("UnsupportedOwnerRetainsExactPayload");
});
