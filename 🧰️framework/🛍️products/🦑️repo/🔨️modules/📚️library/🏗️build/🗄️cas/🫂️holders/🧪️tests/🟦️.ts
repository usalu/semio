/** 🧪️ Tests for the reference holders of the host store: the plan against an independent Python model, protection of live holders under the cap, the capacity eviction of entries together with their objects, and the ageing out of dead roots. */
import {spawnSync} from "node:child_process";
import {existsSync,linkSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {collectCasV1,formatCasCollectionV1} from "../../🧹️collector/🟦️.ts";
import {createCasGuardV1} from "../../🚦️guard/🟦️.ts";
import {casHoldersUsageV1,planCasHolderEvictionV1,readCasHeldV1,keepCasObjectsSyncV1,readCasHoldersV1,reconcileCasKeepersV1,registerCasHolderSyncV1,resetCasHeldCacheV1,type CasHolderPlanBlobV1,type CasHolderPlanEntryV1} from "../🟦️.ts";
import {reclaimCasObjectsV1} from "../../🪦️reclaim/🟦️.ts";
import {createCasStoreV1} from "../../🗃️store/🟦️.ts";
import {CasCapRefusalV1,resetCasGuardsV1} from "../../🚦️guard/🟦️.ts";
import {casUsageHintPathV1} from "../../🧹️collector/🟦️.ts";
import {CAS_POLICY_V1,type CasPolicyV1} from "../../🧬️schema/🟦️.ts";

const directories:string[]=[];
const scratch=(name:string):string=>{const path=mkdtempSync(join(tmpdir(),`cas-holders-${name}-`));directories.push(path);return path;};
afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});
const MIB=1024*1024,sha=(text:string):string=>createHash("sha256").update(text).digest("hex");
const seeded=(seed:number)=>{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};};

test("the capacity plan equals an independent Python model on 300 seeded cases (shared, lost and unfreeable objects, ties, protected shares)",()=>{
 const next=seeded(11),pool=Array.from({length:12},(_,index)=>sha("blob"+index)),cases:{entries:CasHolderPlanEntryV1[];blobs:Record<string,CasHolderPlanBlobV1>;need:number;floors:Record<string,number>}[]=[];
 for(let round=0;round<300;round++){
  const count=1+Math.floor(next()*14),blobs:Record<string,CasHolderPlanBlobV1>={};
  for(const digest of pool)if(next()<0.85)blobs[digest]={size:1+Math.floor(next()*9),freeable:next()<0.7};
  const entries=Array.from({length:count},(_,index)=>({id:"e"+String(index).padStart(2,"0"),holder:"h"+Math.floor(next()*3),access:Math.floor(next()*6),digests:[...new Set(Array.from({length:1+Math.floor(next()*4)},()=>pool[Math.floor(next()*pool.length)]!))]}));
  cases.push({entries,blobs,need:Math.floor(next()*30),floors:Object.fromEntries(["h0","h1","h2"].filter(()=>next()<0.6).map(holder=>[holder,Math.floor(next()*14)]))});
 }
 const run=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️oracle.py",import.meta.url))],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(run.status).toBe(0);
 const expected=JSON.parse(run.stdout) as {entries:string[];digests:string[];freedBytes:number;protectedEntries:number}[];
 cases.forEach((row,index)=>{const plan=planCasHolderEvictionV1(row.entries,new Map(Object.entries(row.blobs)),row.need,new Map(Object.entries(row.floors)));expect({entries:plan.entries,digests:[...plan.digests].sort(),freedBytes:plan.freedBytes,protectedEntries:plan.protectedEntries},`case ${index}`).toEqual(expected[index]!);});
 expect(cases.some((row,index)=>expected[index]!.entries.length>1)).toBe(true);expect(expected.some(row=>row.protectedEntries>0)).toBe(true);
});

const quietGuard=(root:string,policy:CasPolicyV1,now:()=>number)=>createCasGuardV1({root,policy,now,partition:()=>undefined,usage:async()=>0});
const world=(name:string,policy:CasPolicyV1)=>{
 const root=scratch(name),clock={value:2_000_000_000},wide:CasPolicyV1={...policy,capBytes:1024*1024*MIB},store=createCasStoreV1({root,now:()=>clock.value,policy:wide,guard:quietGuard(root,wide,()=>clock.value)});
 let counter=0;
 const put=async():Promise<string>=>{const index=counter++;clock.value=2_000_000_000+index*1000;const body=Buffer.alloc(MIB,index%251);body.writeUInt32BE(index,0);const digest=await store.put(body);const at=new Date(1_000_000+index*1000);utimesSync(store.objectPath(digest.hash),at,at);return digest.hash;};
 const holder=(label:string)=>{
  const dir=scratch("root-"+label),log=join(dir,"y","evictions.jsonl");
  const declare=(at:number)=>registerCasHolderSyncV1(root,policy,{kind:"unit-cache",root:dir,indexes:[{directory:join(dir,"e"),references:["outputs.*.sha256","stdout","stderr"]}],evictionLog:log},at);
  const entry=(name:string,digests:string[])=>{const key=sha(label+name),directory=join(dir,"e",key.slice(0,2),key);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"000000000000001-aaaaaa.json"),JSON.stringify({version:1,key,outputs:digests.map((d,index)=>({name:"o"+index,sha256:d,bytes:MIB,executable:false})),stderr:null,stdout:null,createdMs:1}));return join(directory,"000000000000001-aaaaaa.json");};
  return{dir,log,declare,entry};
 };
 return{root,clock,store,put,holder};
};

test("a collection over the cap takes unreferenced objects first and never an object a live holder references",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:20*MIB,minAgeMs:0,areas:[],touchGranularityMs:0},w=world("protect",policy),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await w.put());
 const a=w.holder("a"),b=w.holder("b");
 for(let index=0;index<3;index++){a.entry("e"+index,[hashes[index*2]!,hashes[index*2+1]!]);b.entry("e"+index,[hashes[6+index*2]!,hashes[6+index*2+1]!]);}
 a.declare(w.clock.value);b.declare(w.clock.value);
 const held=hashes.slice(0,12),loose=hashes.slice(12);
 const result=await collectCasV1({root:w.root,policy,now:()=>w.clock.value});
 expect(result.status).toBe("collected");
 expect((await w.store.has(held)).every(Boolean)).toBe(true);
 expect(result.evicted).toBe(9);expect((await w.store.has(loose)).filter(Boolean).length).toBe(3);
 expect(result.holders).toMatchObject({holders:2,entries:6,heldBlobs:12,evictedEntries:0,reason:"capacity"});
 expect(existsSync(a.log)).toBe(false);
});

test("when only held objects remain over the cap the least recently used entries leave with their objects and the reason is recorded",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:10*MIB,holderProtectedShare:0,minAgeMs:0,areas:[],touchGranularityMs:0},w=world("capacity",policy),hashes:string[]=[];
 for(let index=0;index<14;index++)hashes.push(await w.put());
 const a=w.holder("a"),b=w.holder("b"),files:string[]=[],owner:string[]=[];
 for(let index=0;index<6;index++){const holder=index%2===0?a:b;files.push(holder.entry("e"+index,[hashes[index*2]!,hashes[index*2+1]!]));owner.push(holder===a?"a":"b");}
 a.declare(w.clock.value);b.declare(w.clock.value);w.clock.value+=60_000;
 const result=await collectCasV1({root:w.root,policy,now:()=>w.clock.value});
 expect(result.holders).toMatchObject({evictedEntries:3,evictedBlobs:6,reason:"capacity"});
 expect(result.holders!.freedBytes).toBe(6*MIB);
 expect(files.map(file=>existsSync(file))).toEqual([false,false,false,true,true,true]);
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([...Array(6).fill(false),...Array(6).fill(true),false,false]);
 for(const file of files.filter(existsSync)){const entry=JSON.parse(readFileSync(file,"utf8")) as {outputs:{sha256:string}[]};expect((await w.store.has(entry.outputs.map(row=>row.sha256))).every(Boolean)).toBe(true);}
 const logA=readFileSync(a.log,"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(logA).toHaveLength(1);expect(logA[0]).toMatchObject({version:1,capacity:2,capacityBytes:4*MIB,reason:"capacity"});
 expect(JSON.parse(readFileSync(b.log,"utf8").trim())).toMatchObject({capacity:1,capacityBytes:2*MIB,reason:"capacity"});
 const dry=await collectCasV1({root:w.root,policy:{...policy,capBytes:4*MIB},dryRun:true,now:()=>w.clock.value});
 expect(dry.holders!.evictedEntries).toBeGreaterThan(0);expect(files.filter(existsSync)).toHaveLength(3);
});

test("a shared object stays while a live entry references it: the older entry leaves with its own object only",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:3*MIB,holderProtectedShare:0,minAgeMs:0,areas:[],touchGranularityMs:0},w=world("shared",policy),hashes:string[]=[];
 for(let index=0;index<3;index++)hashes.push(await w.put());
 const a=w.holder("a"),b=w.holder("b");
 const old=a.entry("old",[hashes[0]!,hashes[1]!]),young=b.entry("young",[hashes[1]!,hashes[2]!]);
 a.declare(w.clock.value);b.declare(w.clock.value);w.clock.value+=60_000;
 const result=await collectCasV1({root:w.root,policy,now:()=>w.clock.value});
 expect(result.holders).toMatchObject({evictedEntries:1,evictedBlobs:1,freedBytes:MIB});
 expect([existsSync(old),existsSync(young)]).toEqual([false,true]);
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([false,true,true]);
});

test("an entry published while the collector scans is alive, not dead: the capacity stage never removes a unit whose objects are on disk, even when the need cannot be met",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:MIB,holderProtectedShare:0,minAgeMs:0,areas:[],touchGranularityMs:0,checkEvery:1,progressMs:0},w=world("in-flight",policy),hashes:string[]=[];
 for(let index=0;index<7;index++)hashes.push(await w.put());
 const a=w.holder("a"),old=[a.entry("e0",[hashes[0]!,hashes[1]!]),a.entry("e1",[hashes[2]!,hashes[3]!]),a.entry("e2",[hashes[4]!,hashes[5]!])];
 linkSync(w.store.objectPath(hashes[4]!),join(a.dir,"workspace-link"));a.declare(w.clock.value);w.clock.value+=60_000;
 let fresh:{file:string;hash:string}|undefined;
 const result=await collectCasV1({root:w.root,policy,now:()=>w.clock.value,progress:async event=>{if(event.stage!=="scan"||fresh!==undefined)return;const body=Buffer.alloc(MIB,7);body.writeUInt32BE(9999,0);const hash=(await w.store.put(body)).hash;fresh={hash,file:a.entry("fresh",[hash])};}});
 expect(fresh).toBeDefined();
 expect(existsSync(fresh!.file)).toBe(true);
 expect((await w.store.has([fresh!.hash])).map(Boolean)).toEqual([true]);
 expect(result.holders!.evictedEntries).toBe(3);
 expect(old.map(file=>existsSync(file))).toEqual([false,false,false]);
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([false,false,false,false,true,false,false]);
});

test("a holder that is not renewed or whose root vanished ages out, and its objects are plain candidates again",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:20*MIB,minAgeMs:0,areas:[],touchGranularityMs:0,holderTtlMs:60_000,holderRenewMs:10_000},w=world("expiry",policy),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await w.put());
 const live=w.holder("live"),dead=w.holder("dead"),gone=w.holder("gone");
 live.entry("e",hashes.slice(0,4));dead.entry("e",hashes.slice(4,8));gone.entry("e",hashes.slice(8,12));
 const at=w.clock.value;live.declare(at);dead.declare(at);gone.declare(at);
 expect((await readCasHoldersV1(w.root,at+1)).length).toBe(3);
 live.declare(at+20_000);
 rmSync(gone.dir,{recursive:true,force:true});
 const rows=await readCasHoldersV1(w.root,at+61_000);
 expect(rows.map(row=>row.root)).toEqual([live.dir]);
 expect(readdirSync(join(w.root,"v1","h"))).toHaveLength(1);
 resetCasHeldCacheV1();
 const held=await readCasHeldV1(w.root,at+61_000);
 expect([...held.digests.keys()].sort()).toEqual(hashes.slice(0,4).sort());
 const result=await collectCasV1({root:w.root,policy,now:()=>at+61_000});
 expect((await w.store.has(hashes.slice(0,4))).every(Boolean)).toBe(true);
 expect(result.holders).toMatchObject({holders:1,heldBlobs:4});
});

test("reclaim leaves the objects of a live entry and the usage report names every holder with its bytes",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,minAgeMs:0,areas:[],touchGranularityMs:0},w=world("reclaim",policy),hashes:string[]=[];
 for(let index=0;index<4;index++)hashes.push(await w.put());
 const a=w.holder("a");a.entry("e",[hashes[0]!,hashes[1]!]);a.declare(w.clock.value);resetCasHeldCacheV1();
 const result=await reclaimCasObjectsV1(w.store,hashes,{now:()=>w.clock.value});
 expect(result).toMatchObject({entries:2,held:2});
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([true,true,false,false]);
 await w.store.pin("server-1",[hashes[0]!],60_000);
 const usage=await casHoldersUsageV1(w.root,policy,w.clock.value);
 expect(usage.rows.map(row=>[row.kind,row.digests,row.bytes])).toEqual([["pin",1,MIB],["unit-cache",2,2*MIB]]);
 expect(usage.referencedBytes).toBe(2*MIB);
});

test("keeper links protect held objects from a collector and a reclaimer that know nothing of holders, and are released with the entry",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:2*MIB,minAgeMs:0,areas:[],touchGranularityMs:0},w=world("keepers",policy),hashes:string[]=[];
 for(let index=0;index<12;index++)hashes.push(await w.put());
 const a=w.holder("a");a.entry("e",[hashes[0]!,hashes[1]!]);
 const id=a.declare(w.clock.value);resetCasHeldCacheV1();
 expect(keepCasObjectsSyncV1(w.root,policy,id,[hashes[0]!,hashes[1]!])).toBe(2);
 expect(lstatSync(w.store.objectPath(hashes[0]!)).nlink).toBe(2);
 const oldReclaimer=(hash:string):boolean=>{const path=w.store.objectPath(hash);if(lstatSync(path).nlink>1)return false;rmSync(path);return true;};
 expect(hashes.map(oldReclaimer)).toEqual([false,false,...Array(10).fill(true)]);
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([true,true,...Array(10).fill(false)]);
 const dir=a.dir;rmSync(join(dir,"e"),{recursive:true,force:true});
 const held=await readCasHeldV1(w.root,w.clock.value);
 expect(held.digests.size).toBe(0);
 expect(await reconcileCasKeepersV1(w.root,policy,held)).toMatchObject({linked:0,released:2});
 expect((await reclaimCasObjectsV1(w.store,hashes,{now:()=>w.clock.value})).entries).toBe(2);
 expect(existsSync(join(w.root,"v1","k",id))).toBe(false);
});

const capRoom=async(name:string,heldCount:number,looseCount:number,ageMs=3_600_000)=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:20*MIB,areas:[],touchGranularityMs:0},w=world(name,policy),hashes:string[]=[];
 for(let index=0;index<heldCount+looseCount;index++)hashes.push(await w.put());
 const now=w.clock.value+3_600_000;
 const old=new Date(now-ageMs);for(const hash of hashes)utimesSync(w.store.objectPath(hash),old,old);
 const young=new Date(now-60_000);utimesSync(w.store.objectPath(hashes[hashes.length-1]!),young,young);
 const a=w.holder("a");if(heldCount>0)a.entry("e",hashes.slice(0,heldCount));a.declare(now);resetCasHeldCacheV1();resetCasGuardsV1();
 mkdirSync(join(w.root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(w.root),JSON.stringify({version:1,bytes:(heldCount+looseCount)*MIB,atMs:now-3_600_000}));
 const guard=createCasGuardV1({root:w.root,policy,now:()=>now,home:scratch("home"),partition:()=>undefined,refuseOverCap:true,usage:async()=>(heldCount+looseCount)*MIB});
 const store=createCasStoreV1({root:w.root,policy,now:()=>now,guard,refuseOverCap:true});
 return{w,hashes,store,guard,now};
};

test("a write at the cap collects the unreferenced objects at once and is admitted; held objects and objects of the last minutes stay",async()=>{
 const r=await capRoom("admit-unreferenced",3,20,300_000);
 const digest=await r.store.put(Buffer.alloc(MIB,251));
 expect(digest.size).toBe(MIB);
 const present=(await r.w.store.has(r.hashes)).map(Boolean);
 expect(present.slice(0,3)).toEqual([true,true,true]);
 expect(present[present.length-1]).toBe(true);
 expect(present.filter(Boolean).length).toBeLessThanOrEqual(15);expect(present.filter(Boolean).length).toBeGreaterThanOrEqual(4);
 await r.guard.settle();
});

test("a write is refused with a typed error when every object over the cap is younger than the minimum age, and nothing is deleted",async()=>{
 const r=await capRoom("refuse-young",0,22,60_000);
 await expect(r.store.put(Buffer.alloc(MIB,252))).rejects.toBeInstanceOf(CasCapRefusalV1);
 expect((await r.w.store.has(r.hashes)).every(Boolean)).toBe(true);
 await r.guard.settle();
});

const PARTITION=(freeMib:number)=>({partition:"fixture",totalBytes:100*MIB,freeBytes:freeMib*MIB});

test("a partition at 96 % used with unreferenced objects present removes only the unreferenced ones and evicts no held entry, and the report names the store's share",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:1024*MIB,minAgeMs:0,areas:[],touchGranularityMs:0},w=world("pressure",policy),hashes:string[]=[];
 for(let index=0;index<20;index++)hashes.push(await w.put());
 const a=w.holder("a"),files=[0,1,2,3,4,5].map(index=>a.entry("e"+index,[hashes[index*2]!,hashes[index*2+1]!]));
 a.declare(w.clock.value);w.clock.value+=60_000;
 const options={root:w.root,policy,now:()=>w.clock.value,partition:PARTITION(4)};
 const preview=await collectCasV1({...options,dryRun:true});
 expect(preview.totals.partitionShortfallBytes).toBeGreaterThan(0);expect(preview.totals.capShortfallBytes).toBe(0);
 expect(preview.holders).toMatchObject({evictedEntries:0,evictedBlobs:0});expect(preview.evicted).toBe(8);
 const result=await collectCasV1(options);
 expect(result.evicted).toBe(8);expect(result.holders).toMatchObject({holders:1,entries:6,evictedEntries:0});
 expect(files.every(file=>existsSync(file))).toBe(true);
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([...Array(12).fill(true),...Array(8).fill(false)]);
 expect(existsSync(a.log)).toBe(false);
 for(const language of ["en","de"] as const)expect(formatCasCollectionV1(result,policy,language)).toMatch(language==="en"?/not the store's/u:/nicht vom Speicher/u);
});

test("over its own cap with nothing unreferenced the store evicts least recently used held entries but never below the protected share of a live holder",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:4*MIB,holderProtectedShare:0.5,minAgeMs:0,areas:[],touchGranularityMs:0},w=world("floor",policy),hashes:string[]=[];
 for(let index=0;index<12;index++)hashes.push(await w.put());
 const a=w.holder("a"),b=w.holder("b"),files:string[]=[];
 for(let index=0;index<6;index++)files.push((index%2===0?a:b).entry("e"+index,[hashes[index*2]!,hashes[index*2+1]!]));
 a.declare(w.clock.value);b.declare(w.clock.value);w.clock.value+=60_000;
 const result=await collectCasV1({root:w.root,policy,now:()=>w.clock.value,role:"operator"});
 expect(result.totals.capShortfallBytes).toBeGreaterThan(0);
 expect(result.holders).toMatchObject({evictedEntries:4,evictedBlobs:8,protectedEntries:2,freedBytes:8*MIB});
 expect(files.map(file=>existsSync(file))).toEqual([false,false,false,false,true,true]);
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([...Array(8).fill(false),...Array(4).fill(true)]);
 const row=JSON.parse(readFileSync(a.log,"utf8").trim().split("\n")[0]!) as Record<string,unknown>;
 expect(row).toMatchObject({reason:"capacity",role:"operator",pid:process.pid});
 expect(formatCasCollectionV1(result,policy,"en")).toContain("stayed protected");
});

test("gc --unreferenced removes only objects nothing references and never a held, pinned, leased or young object, even over the cap; the dry run shows the same plan",async()=>{
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:2*MIB,minAgeMs:60_000,areas:[],touchGranularityMs:0},w=world("unreferenced",policy),hashes:string[]=[];
 for(let index=0;index<16;index++)hashes.push(await w.put());
 w.clock.value+=3_600_000;
 const a=w.holder("a"),file=a.entry("e",hashes.slice(0,4));a.declare(w.clock.value);
 await w.store.pin("server-1",[hashes[4]!],600_000);await w.store.lease([hashes[5]!],600_000);
 const young=new Date(w.clock.value-1000);utimesSync(w.store.objectPath(hashes[6]!),young,young);
 const options={root:w.root,policy,now:()=>w.clock.value,unreferencedOnly:true,areas:[]};
 const preview=await collectCasV1({...options,dryRun:true});
 expect(preview).toMatchObject({unreferencedOnly:true,evicted:9,dryRun:true});expect(preview.holders).toMatchObject({evictedEntries:0});
 expect((await w.store.has(hashes)).every(Boolean)).toBe(true);
 const result=await collectCasV1(options);
 expect(result.evicted).toBe(9);expect(result.freedBytes).toBe(9*MIB);expect(result.holders).toMatchObject({evictedEntries:0,evictedBlobs:0});
 expect((await w.store.has(hashes)).map(Boolean)).toEqual([...Array(7).fill(true),...Array(9).fill(false)]);
 expect(existsSync(file)).toBe(true);
 expect(formatCasCollectionV1(preview,policy,"en")).toContain("held entries evicted: 0");expect(formatCasCollectionV1(preview,policy,"de")).toContain("verdrängte gehaltene Einträge: 0");
});
