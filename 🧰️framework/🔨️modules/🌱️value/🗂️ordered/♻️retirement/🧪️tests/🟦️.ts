import {test,expect} from "bun:test";
import Ajv from "ajv";
import stableStringify from "fast-json-stable-stringify";
import "../../../♻️retirement/📋️queue/🧪️tests/🟦️.ts";

type Corpus={cases:{id:string,key:string,keyCapacity:number,value:string,valueCapacity:number}[],grants:number[],expected:{terminalOwners:number,stepBirthBytes:number,reportedReleaseEqualsAllocator:boolean,actualReleaseFitsGrant:boolean,sourcePayloadPointerPreserved:boolean}};
const fixture:Corpus=await Bun.file(new URL("../🧫️fixtures/🔣️.json",import.meta.url)).json();
const schema=await Bun.file(new URL("../🧬️schema/🔣️.json",import.meta.url)).json();
test("ordered retirement grants separate logical work from whole physical release",()=>{
  const ajv=new Ajv({strict:true});ajv.addSchema(schema);const grant=ajv.compile({$ref:schema.$id+"#/$defs/Grant"});const progress=ajv.compile({$ref:schema.$id+"#/$defs/Progress"});
  for(const maximum of fixture.grants){expect(grant({maximumItems:1,maximumCopyBytes:maximum,maximumCapacityBytes:0,maximumReleaseBytes:0,maximumDepth:64})).toBe(true);expect(progress({copiedItems:1,copiedBytes:maximum,retainedCapacityBytes:0,releasedBytes:0,complete:false})).toBe(true);}
  expect(grant({maximumItems:1,maximumBytes:64})).toBe(false);expect(grant({maximumItems:1,maximumCopyBytes:0,maximumCapacityBytes:0,maximumReleaseBytes:-1,maximumDepth:64})).toBe(false);
  expect(fixture.expected).toEqual({terminalOwners:0,stepBirthBytes:0,reportedReleaseEqualsAllocator:true,actualReleaseFitsGrant:true,sourcePayloadPointerPreserved:true});
});
test("ordered retirement payload corpus matches independent unique UTF8 map ownership",()=>{
  for(const row of fixture.cases){const map=new Map([[row.key,row.value]]);const entries=[...map].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b)));expect(stableStringify(Object.fromEntries(map))).toBe(JSON.stringify(Object.fromEntries(entries)));expect(Buffer.byteLength(row.key)).toBeLessThan(row.keyCapacity);expect(Buffer.byteLength(row.value)).toBeLessThan(row.valueCapacity);expect(map.get(row.key)).toBe(row.value);}
});

test("ordered mutation contract and operation corpus match independent unique UTF8 maps",async()=>{
  const contract=await Bun.file(new URL("../../🩹️update/🧬️schema/🔣️.json",import.meta.url)).json();
  type Operation={kind:"set"|"remove",key:string,value?:number};
  type Mutations={cases:{id:string,initial:[string,number][],operations:Operation[]}[],grants:number[],cancelAfterSteps:number[]};
  const mutations:Mutations=await Bun.file(new URL("../../🩹️update/🧫️fixtures/🔣️.json",import.meta.url)).json();
  const ajv=new Ajv({strict:true});ajv.addSchema(contract);const operation=ajv.compile({$ref:contract.$id+"#/$defs/Operation"});const demand=ajv.compile({$ref:contract.$id+"#/$defs/Demand"});
  expect(demand({copyBytes:0,capacityBytes:0,depth:0})).toBe(true);expect(demand({copyBytes:1,capacityBytes:-1,depth:1})).toBe(false);expect(operation({kind:"remove",key:"a",value:1})).toBe(false);
  for(const row of mutations.cases){const map=new Map(row.initial);const external:Record<string,number>=Object.fromEntries(row.initial);for(const change of row.operations){expect(operation(change)).toBe(true);if(change.kind==="set"){if(change.value===undefined)throw Error("set fixture requires numeric value");map.set(change.key,change.value);external[change.key]=change.value;}else{map.delete(change.key);delete external[change.key];}const entries=[...map].sort(([a],[b])=>Buffer.compare(Buffer.from(a),Buffer.from(b)));expect(JSON.stringify(Object.fromEntries(entries))).toBe(stableStringify(external));}}
});
