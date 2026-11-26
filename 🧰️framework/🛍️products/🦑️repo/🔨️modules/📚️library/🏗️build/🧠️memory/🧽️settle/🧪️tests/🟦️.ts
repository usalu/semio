import{expect,test}from"bun:test";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
import{SETTLE_MESSAGES_V1,SETTLE_POLICY_V1,renderSettleMessageV1,reportRetainedGraphMemoryV1,settleGraphMemoryV1}from"../🟨️.mjs";

/** 🧽️ Contract: after a plan is answered the process collects, shrinks and logs one bilingual line; it does nothing on a serving machine; between waves it names the bounded caches that still hold memory.
 * @see ../🧬️schema/🔣️.json */
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const MB=1048576,holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();

test("messages and policy equal the schema file; both languages have the same keys and placeholders",()=>{
 expect(SETTLE_MESSAGES_V1).toEqual(schema.definitions.Messages.const);expect(SETTLE_POLICY_V1).toEqual(schema.definitions.Policy.const);
 expect(Object.keys(SETTLE_MESSAGES_V1.en)).toEqual(Object.keys(SETTLE_MESSAGES_V1.de));
 for(const key of Object.keys(SETTLE_MESSAGES_V1.en))expect(holes(SETTLE_MESSAGES_V1.de[key as "answered"])).toEqual(holes(SETTLE_MESSAGES_V1.en[key as "answered"]));
 expect(SETTLE_MESSAGES_V1.en.answered).toBe("graph: plan answered, managed {managed} MB, resident {resident} MB (released {released} MB)");
});

test("the result matches its schema, the line carries managed, resident and released, one line per language",async()=>{
 const state={managed:900*MB,resident:5000*MB,collections:0,shrinks:0},lines:string[]=[];
 const result=await settleGraphMemoryV1({limitBytes:4915*MB,environment:{SEMIO_LANGUAGE:"en,de"},report:line=>lines.push(line),release:()=>{state.managed=300*MB;},ports:{sample:()=>state.managed,collect:()=>{state.collections++;}},resident:()=>state.resident,shrink:()=>{state.shrinks++;state.resident=2100*MB;}});
 const ajv=new Ajv({strict:false});ajv.addSchema(schema,"s");expect(ajv.validate({$ref:"s#/definitions/Result"},result)).toBe(true);
 expect(result).toEqual({managedBeforeMegabytes:900,managedMegabytes:300,residentBeforeMegabytes:5000,residentMegabytes:2100,releasedMegabytes:2900});
 expect(state.collections).toBeGreaterThanOrEqual(1);expect(state.shrinks).toBe(1);
 expect(lines[0]).toBe("graph: plan answered, managed 300 MB, resident 2100 MB (released 2900 MB)");
 expect(lines.length).toBe(renderSettleMessageV1("answered",{managed:1,resident:1,released:1},{SEMIO_LANGUAGE:"en,de"}).length);
});

test("a serving machine (no plan limit) is not touched and says nothing, SEMIO_GRAPH_PHASES=1 and force override",async()=>{
 const lines:string[]=[];let collected=0;const ports={sample:()=>0,collect:()=>{collected++;}};
 expect(await settleGraphMemoryV1({limitBytes:Infinity,environment:{},report:line=>lines.push(line),ports})).toBeUndefined();expect(lines).toEqual([]);expect(collected).toBe(0);
 expect(await settleGraphMemoryV1({limitBytes:Infinity,environment:{SEMIO_GRAPH_PHASES:"1"},report:line=>lines.push(line),ports,shrink:()=>undefined})).toBeDefined();expect(lines.length).toBeGreaterThan(0);
 lines.length=0;expect(await settleGraphMemoryV1({limitBytes:Infinity,environment:{},force:true,report:line=>lines.push(line),ports,shrink:()=>undefined})).toBeDefined();
});

test("the retained line names the largest bounded caches and is reported without a plan limit (the whole read has none)",()=>{
 const lines:string[]=[],stats=()=>[{name:"a",entries:1,bytes:5*MB},{name:"b",entries:9,bytes:50*MB},{name:"c",entries:0,bytes:0}];
 reportRetainedGraphMemoryV1({limitBytes:Infinity,environment:{},report:line=>lines.push(line),stats,ports:{sample:()=>1*MB,collect:()=>undefined},resident:()=>2*MB});expect(lines.length).toBeGreaterThan(0);lines.length=0;
 reportRetainedGraphMemoryV1({limitBytes:4915*MB,environment:{},report:line=>lines.push(line),stats,ports:{sample:()=>100*MB,collect:()=>undefined},resident:()=>200*MB});
 expect(lines[0]).toBe("graph: after the wave managed 100 MB, resident 200 MB; largest collections: b 9 entries 50 MB, a 1 entries 5 MB");
});

test("REAL heap: 200 MB of garbage a plan left is gone after a settling (managed falls by most of it)",async()=>{
 let garbage:object[]|undefined=Array.from({length:3_000_000},(_,at)=>({at,text:"graph-record-"+at}));
 const before=process.memoryUsage().heapUsed;let released=0;
 const result=await settleGraphMemoryV1({force:true,environment:{},report:()=>undefined,release:()=>{released=garbage!.length;garbage=undefined;}});
 expect(released).toBe(3_000_000);expect(result!.managedBeforeMegabytes).toBeGreaterThan(Math.round(before/MB)-5);
 expect(result!.managedBeforeMegabytes-result!.managedMegabytes).toBeGreaterThan(80);
});

test("the default retained census names the registered collections of the whole read next to the bounded caches",async()=>{
 const{registerSliceCollectionsV1}=await import("../../📋️slice-report/🟨️.mjs"),lines:string[]=[];
 registerSliceCollectionsV1("test-wave-census",()=>[{name:"test:per-owner",entries:7,bytes:300*MB}]);
 reportRetainedGraphMemoryV1({limitBytes:4915*MB,environment:{},report:line=>lines.push(line),ports:{sample:()=>100*MB,collect:()=>undefined},resident:()=>200*MB});
 expect(lines[0]).toContain("largest collections: test:per-owner 7 entries 300 MB");
});
