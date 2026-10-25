import {Database} from "bun:sqlite";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import budgetPolicy from "../../../🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import quota from "../../../../../🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import policy from "../../🔣️policy.json" with {type:"json"};
import cachingPolicy from "../../../../🔣️policy.json" with {type:"json"};
import unitCacheSchema from "../../../../🦀️cargo/🗃️unit-cache/🧬️schema/🔣️.json" with {type:"json"};
import {cacheAreaBudgetBytesV1,cacheNeedRequiredBytesV1,readCacheLedgerV1,type CacheNeedV1} from "../../🟦️.ts";
import {readImageEnvelopeV1} from "../../../../../🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts";
import sweepSchema from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/♻️retirement/🧬️schema/🔣️json/🟦️.ts";
import metadataSchema from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/📦️metadata/🧬️schema/🔣️json/🟦️.ts";
import authoritySchema from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🧬️schema/📜️ticket/🔣️json/🟦️.ts";

const clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value)),ledger=readCacheLedgerV1(policy),GIB=1024**3,unitCachePolicy=unitCacheSchema.definitions.Policy.const;

test("the graph partition is declared large enough for its measured need, derived from the caps of its parts",()=>{
 const need=ledger.needs!.find(row=>row.partition==="graph")!,required=cacheNeedRequiredBytesV1(ledger,need);
 const caps=need.components.map(part=>part.area?cacheAreaBudgetBytesV1(ledger,ledger.areas.find(area=>area.id===part.area)!)!:part.bytes!);
 const database=new Database(":memory:");
 try{
  const row=database.query("SELECT CAST((?1+0.0)*(100+?2)/100 + 0.999999 AS INTEGER)+?3 AS bytes").get(caps.reduce((sum,value)=>sum+value,0),need.headroomPercent,need.floorBytes) as {bytes:number};
  expect(required).toBe(row.bytes);
 }finally{database.close();}
 expect(budgetPolicy.partitions.graph.capacityBytes).toBeGreaterThanOrEqual(required);
 expect(budgetPolicy.partitions.graph.capacityBytes).toBe(6*GIB);
 expect(budgetPolicy.partitions.graph.capacityBytes-required).toBeLessThan(GIB);
 for(const part of need.components)expect(part.measuredBytes).toBeLessThanOrEqual(part.area?cacheAreaBudgetBytesV1(ledger,ledger.areas.find(area=>area.id===part.area)!)!:part.bytes!);
 expect(need.floorBytes).toBe(quota.definitions.OptionalQuota.policy.minimumAvailableBytes);
 const reclaim=ledger.areas.find(area=>area.id==="graph-inference")!.reclaim!;
 expect(reclaim.softAvailableBytes).toBeGreaterThan(need.floorBytes);
 expect(budgetPolicy.partitions.graph.maximumLogicalBytes).toBe(budgetPolicy.partitions.graph.capacityBytes-1048576);
 expect(ledger.areas.find(area=>area.id==="graph")!.recreate).toBe("derived");
});

test("a partition smaller than its measured need, a bad reclaim and a bad recreate declaration are refused",()=>{
 const small=clone(budgetPolicy.partitions);small.graph.capacityBytes=4*GIB;small.graph.maximumLogicalBytes=4*GIB-1048576;
 expect(()=>readCacheLedgerV1(policy,small)).toThrow("Closed cache ledger required");
 const unreclaimed:any=clone(policy);delete unreclaimed.areas.find((row:any)=>row.id==="graph-inference").reclaim;
 expect(()=>readCacheLedgerV1(unreclaimed,small)).toThrow("smaller than its measured need");
 expect(()=>readCacheLedgerV1(unreclaimed)).not.toThrow();
 const mutations:Array<[string,(document:any)=>void]>=[
  ["soft not below target",document=>{const area=document.areas.find((row:any)=>row.id==="graph-inference");area.reclaim.softAvailableBytes=area.reclaim.targetAvailableBytes;}],
  ["target beyond the headroom schema",document=>{document.areas.find((row:any)=>row.id==="graph-inference").reclaim.targetAvailableBytes=3*GIB;}],
  ["soft above what the caps leave free",document=>{const area=document.areas.find((row:any)=>row.id==="graph-inference");area.reclaim.softAvailableBytes=2*GIB+1;area.reclaim.targetAvailableBytes=2*GIB+2;document.areas.find((row:any)=>row.id==="graph-closures").budget={bytes:3*GIB};}],
  ["reclaim on a tree",document=>{const area=document.areas.find((row:any)=>row.id==="go");area.reclaim={softAvailableBytes:1,targetAvailableBytes:2};}],
  ["recreate on a nested area",document=>{document.areas.find((row:any)=>row.id==="graph-inference").recreate="derived";}],
  ["need for an unknown partition",document=>{document.needs[0].partition="nowhere";}],
  ["need for an unknown area",document=>{document.needs[0].components[0].area="nowhere";}],
  ["unknown recreate mode",document=>{document.areas.find((row:any)=>row.id==="graph").recreate="always";}]
 ];
 for(const[name,mutate]of mutations){const document=clone(policy);mutate(document);expect(()=>readCacheLedgerV1(document),name).toThrow();}
});

test("the Bazel workspaces reclaim rule may aim beyond the graph headroom schema, but never leave less free space than its soft threshold or stand on the wrong partition",()=>{
 const rule=policy.areas.find(row=>row.id==="bazel-workspaces")?.reclaim;if(!rule)throw Error("The Bazel workspaces area declares a reclaim rule");
 expect(rule.targetAvailableBytes).toBeGreaterThan(2684354560);expect(()=>readCacheLedgerV1(policy)).not.toThrow();
 const beyond:any=clone(policy);beyond.areas.find((row:any)=>row.id==="bazel-workspaces").reclaim.softAvailableBytes=22*GIB;beyond.areas.find((row:any)=>row.id==="bazel-workspaces").reclaim.targetAvailableBytes=23*GIB;
 expect(()=>readCacheLedgerV1(beyond)).toThrow();
 const inverted:any=clone(policy);Object.assign(inverted.areas.find((row:any)=>row.id==="bazel-workspaces").reclaim,{softAvailableBytes:rule.targetAvailableBytes,targetAvailableBytes:rule.softAvailableBytes});
 expect(()=>readCacheLedgerV1(inverted)).toThrow();
});

test("the envelope an image was born with is read from its image.json, and nothing else counts",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"cgn-"));
 try{
  expect(await readImageEnvelopeV1(directory)).toBeNull();
  writeFileSync(join(directory,"image.json"),JSON.stringify({version:1,maximumPhysicalBytes:4294967296}));
  expect(await readImageEnvelopeV1(directory)).toBe(4294967296);
  writeFileSync(join(directory,"image.json"),JSON.stringify({maximumPhysicalBytes:"4"}));
  expect(await readImageEnvelopeV1(directory)).toBeNull();
  writeFileSync(join(directory,"image.json"),"not json");
  expect(await readImageEnvelopeV1(directory)).toBeNull();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the measured need is a closed document in the schema",()=>{
 const need:CacheNeedV1={partition:"graph",measuredOn:"2026-10-21",headroomPercent:0,floorBytes:0,components:[{id:"x",bytes:10,measuredBytes:1}]};
 expect(cacheNeedRequiredBytesV1(ledger,need)).toBe(10);
 expect(cacheNeedRequiredBytesV1(ledger,{...need,headroomPercent:25,floorBytes:5})).toBe(18);
});

test("the rust partition is declared large enough for its measured need: caps of its parts, headroom, and the one compile that must always fit",async()=>{
 const need=ledger.needs!.find(row=>row.partition==="rust")!,required=cacheNeedRequiredBytesV1(ledger,need),rust=budgetPolicy.partitions.rust;
 const caps=need.components.map(part=>part.area?cacheAreaBudgetBytesV1(ledger,ledger.areas.find(area=>area.id===part.area)!)!:part.bytes!);
 const database=new Database(":memory:");
 try{
  const row=database.query("SELECT CAST((?1+0.0)*(100+?2)/100 + 0.999999 AS INTEGER)+?3 AS bytes").get(caps.reduce((sum,value)=>sum+value,0),need.headroomPercent,need.floorBytes) as {bytes:number};
  expect(required).toBe(row.bytes);
  const fit=database.query("SELECT ?1>=?2 AS fits,?1-?2<?3 AS tight,?4+?5 AS logical,?6 AS declared").get(rust.capacityBytes,required,GIB,rust.maximumLogicalBytes,1048576,rust.capacityBytes) as {fits:number;tight:number;logical:number;declared:number};
  expect(fit).toEqual({fits:1,tight:1,logical:rust.capacityBytes,declared:rust.capacityBytes});
 }finally{database.close();}
 expect(rust.capacityBytes).toBe(48*GIB);
 for(const part of need.components)expect(part.measuredBytes,part.id).toBeLessThanOrEqual(part.area?cacheAreaBudgetBytesV1(ledger,ledger.areas.find(area=>area.id===part.area)!)!:part.bytes!);
 const strict=cachingPolicy.storage.strictStorage;
 expect(strict.capacityBytes).toBe(rust.capacityBytes);expect(strict.filesystemBytes).toBe(rust.capacityBytes-1048576);
 expect(need.floorBytes).toBe(strict.maximumWorkspaceBytes);
 expect(need.components.find(part=>part.id==="compiler-workspaces")!.bytes).toBe(2*strict.maximumWorkspaceBytes);
 const sum=Object.values(budgetPolicy.partitions).reduce((total,row)=>total+row.capacityBytes,0);
 expect(sum).toBeLessThanOrEqual(budgetPolicy.maximumBytes-budgetPolicy.maximumControlBytes);expect(budgetPolicy.maximumBytes-sum).toBeLessThan(GIB);
});

test("the unit cache keeps its collection shares, one compile of room above the high-water mark and the carried content inside the rust partition's declared areas",()=>{
 const gc=unitCachePolicy.gc,strict=cachingPolicy.storage.strictStorage,budget=strict.capacityBytes-strict.maximumWorkspaceBytes,unit=ledger.areas.find(area=>area.id==="unit-cache")!;
 expect(Math.floor(budget*gc.casShare)+Math.floor(budget*gc.laneShare)+GIB).toBeLessThanOrEqual(cacheAreaBudgetBytesV1(ledger,unit)!);
 expect(strict.filesystemBytes*(1-gc.highWaterShare)).toBeGreaterThanOrEqual(strict.maximumWorkspaceBytes);
 expect(strict.filesystemBytes*(1-gc.lowWaterShare)).toBeGreaterThan(strict.filesystemBytes*(1-gc.highWaterShare));
 const rust=ledger.areas.find(area=>area.id==="rust")!;
 expect(rust.recreate).toBe("carried");expect(rust.carry).toEqual(["cargo/home","u/o","u/e","u/a"]);
 for(const path of rust.carry!)expect(ledger.areas.some(area=>area.kind==="nested"&&area.partition==="rust"&&path.startsWith(area.directory+"/")||area.directory===path),path).toBe(true);
 const mutations:Array<[string,(document:any)=>void]>=[
  ["carry without carried",document=>{delete document.areas.find((row:any)=>row.id==="rust").recreate;}],
  ["carried without carry",document=>{delete document.areas.find((row:any)=>row.id==="rust").carry;}],
  ["carry outside every nested area",document=>{document.areas.find((row:any)=>row.id==="rust").carry=["somewhere/else"];}],
  ["carry that climbs",document=>{document.areas.find((row:any)=>row.id==="rust").carry=["u/../x"];}]
 ];
 for(const[name,mutate]of mutations){const document=clone(policy);mutate(document);expect(()=>readCacheLedgerV1(document),name).toThrow();}
 const small=clone(budgetPolicy.partitions);small.rust.capacityBytes=16*GIB;small.rust.maximumLogicalBytes=16*GIB-1048576;
 expect(()=>readCacheLedgerV1(policy,small)).toThrow("smaller than its measured need");
});

test("the leases partition is declared large enough for the store the sweep keeps: entries at their bytes, the headroom file, the nested test stores and one claim turn of floor",()=>{
 const need=ledger.needs!.find(row=>row.partition==="leases")!,required=cacheNeedRequiredBytesV1(ledger,need),leases=budgetPolicy.partitions.leases,store=sweepSchema.definitions.Store.const,idle=sweepSchema.definitions.Idle.const,part=(id:string)=>need.components.find(row=>row.id===id)!;
 const database=new Database(":memory:");
 try{
  const caps=need.components.map(row=>row.bytes!),row=database.query("SELECT CAST((?1+0.0)*(100+?2)/100 + 0.999999 AS INTEGER)+?3 AS bytes").get(caps.reduce((sum,value)=>sum+value,0),need.headroomPercent,need.floorBytes) as {bytes:number};
  expect(required).toBe(row.bytes);
  const fit=database.query("SELECT ?1>=?2 AS fits,?1-?2<?3 AS tight,?4+?5 AS logical,?6 AS declared").get(leases.capacityBytes,required,16*1048576,leases.maximumLogicalBytes,4194304,leases.capacityBytes) as {fits:number;tight:number;logical:number;declared:number};
  expect(fit).toEqual({fits:1,tight:1,logical:leases.capacityBytes,declared:leases.capacityBytes});
  const mix=database.query("SELECT ?1*?2 AS resources,?3*?4 AS claims,?5*?6 AS nested").get(store.resourceEntries,store.databaseBytes,store.claimEntries,store.claimBytes,store.nestedStores,store.nestedStoreBytes) as {resources:number;claims:number;nested:number};
  expect(part("resources").bytes).toBe(mix.resources);expect(part("claims").bytes).toBe(mix.claims);expect(part("nested-stores").bytes).toBe(mix.nested);
 }finally{database.close();}
 expect(leases.capacityBytes).toBe(64*1048576);
 expect(part("headroom").bytes).toBe(metadataSchema.properties.capacityBytes.const);
 expect(need.floorBytes).toBe(metadataSchema.properties.capacityBytes.const);
 expect(need.floorBytes).toBeGreaterThanOrEqual(authoritySchema.definitions.ClaimRetentionPolicy.properties.maximumClaimsPerTurn.const*store.claimBytes);
 expect(store.nestedStoreBytes).toBeGreaterThan(metadataSchema.properties.capacityBytes.const);
 for(const row of need.components)expect(row.measuredBytes,row.id).toBeLessThanOrEqual(row.bytes!);
 expect(idle.pressureFreeRatio*leases.capacityBytes).toBeGreaterThanOrEqual(need.floorBytes);
 expect(idle.nestedIdleMilliseconds).toBeGreaterThan(idle.idleMilliseconds);
 expect(ledger.areas.find(area=>area.id==="leases")!.recreate).toBe("derived");
 const small=clone(budgetPolicy.partitions);small.leases.capacityBytes=48*1048576;small.leases.maximumLogicalBytes=48*1048576-1048576;
 expect(()=>readCacheLedgerV1(policy,small)).toThrow("smaller than its measured need");
});
