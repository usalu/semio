import {registeredToolExecutableV1} from "../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
/** 🧪️ Tests for the host-wide content-addressed store, its access ledger, pins and least-recently-accessed collector. */
import {spawn,spawnSync} from "node:child_process";
import {appendFileSync,chmodSync,existsSync,realpathSync,symlinkSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {tmpdir} from "node:os";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import traces from "./📋️traces/🔣️.json" with {type:"json"};
import enforcement from "./📋️enforcement/🔣️.json" with {type:"json"};
import {collectCasV1,createCasPlannerV1,formatCasCollectionV1} from "../🧹️collector/🟦️.ts";
import {casEnforceTickV1,createCasGuardV1,resetCasGuardsV1} from "../🚦️guard/🟦️.ts";
import {casEffectiveCapBytesV1,casReserveBytesV1,casReserveStateV1,casPartitionNeedBytesV1,declareCasRoleV1,decideCasEnforcementV1,measureCasPartitionV1,readCasRoleV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import {casUsageHintPathV1,readCasCensusV1,readCasUsageHintV1,type CasCollectionV1} from "../🧹️collector/🟦️.ts";
import {putCasObjectSyncV1} from "../🗃️store/🔁️sync/🟦️.ts";
import {casLedgerShardV1,foldCasShardV1,openCasShardViewV1} from "../📒️ledger/🟦️.ts";
import {isRepoErrorV1,renderRepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {casRootV1,createCasStoreV1} from "../🗃️store/🟦️.ts";
import {CasCapRefusalV1} from "../🚦️guard/🟦️.ts";
import {CAS_LEASE_HEADER_V1,readCasTransientsV1} from "../📌️pins/🟦️.ts";
import {httpRemoteExecutionContentV1} from "../../🌐️remote-execution/🗄️content/🟦️.ts";
import {casPinClosuresV1} from "../📂️areas/🟦️.ts";
import {releaseCasPinsV1} from "../📌️pins/🟦️.ts";
import {casUsageV1} from "../🗃️store/📏️usage/🟦️.ts";
import {CAS_MESSAGES_V1,CAS_POLICY_V1,casTextV1,requireCasPolicyV1,type CasPolicyV1,type CasRoleV1} from "../🧬️schema/🟦️.ts";

const directories:string[]=[];
const scratch=(name:string):string=>{const path=mkdtempSync(join(tmpdir(),`cas-${name}-`));directories.push(path);return path;};
afterAll(()=>{for(const path of directories)rmSync(path,{recursive:true,force:true});});
const seeded=(seed:number)=>{let state=seed>>>0;return()=>{state=(Math.imul(state,1664525)+1013904223)>>>0;return state/4294967296;};};
const sha=(text:string|Uint8Array):string=>createHash("sha256").update(text).digest("hex");
const MIB=1024*1024;

type Case={name?:string;policy:{capBytes:number;highWatermark:number;lowWatermark:number};partitionNeedBytes?:number;externalBytes:number;nowMs:number;areas:{id:string;capBytes:number|null;maxAgeMs:number|null}[];entries:{area:string;id:string;access:number;size:number;protected:boolean}[]};

function plan(row:Case):Record<string,string>{
 const order=new Map(row.areas.map((area,index)=>[area.id,index])),sorted=[...row.entries].sort((left,right)=>order.get(left.area)!-order.get(right.area)!||(left.id<right.id?-1:1));
 const planner=createCasPlannerV1({policy:row.policy,areas:row.areas,externalBytes:row.externalBytes,nowMs:row.nowMs,partitionNeedBytes:row.partitionNeedBytes});
 for(const entry of sorted)planner.add(entry.area,entry.access,entry.size,entry.protected);
 const evaluator=planner.finish().evaluator(),victims:Record<string,string>={};
 for(const entry of sorted.filter(row=>!row.protected)){const reason=evaluator.decide(entry.area,entry.access,entry.size);if(reason)victims[entry.id+"@"+entry.area]=reason;}
 return victims;
}
const oracle=(cases:Case[]):Record<string,string>[]=>{
 const run=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️oracle.py",import.meta.url))],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);
};

test("authored eviction traces give the authored victims, in TypeScript and in the independent Python oracle",()=>{
 const cases=traces.cases as unknown as (Case&{victims:Record<string,string>})[],expected=cases.map(row=>Object.fromEntries(Object.entries(row.victims).map(([id,reason])=>[id+"@"+row.entries.find(entry=>entry.id===id)!.area,reason])));
 const python=oracle(cases.map(row=>structuredClone(row)));
 cases.forEach((row,index)=>{expect(plan(row),row.name).toEqual(expected[index]!);expect(python[index],row.name).toEqual(expected[index]!);});
});

test("seeded random traces: the streaming planner equals the Python least-recently-accessed oracle",()=>{
 const random=seeded(20261109),cases:Case[]=[];
 for(let round=0;round<400;round++){
  const areas=[{id:"store",capBytes:null,maxAgeMs:null},{id:"graph-memo",capBytes:random()<0.7?Math.floor(500+random()*4000):null,maxAgeMs:random()<0.5?Math.floor(2000+random()*8000):null},{id:"build-input-closures",capBytes:random()<0.5?Math.floor(500+random()*4000):null,maxAgeMs:random()<0.5?Math.floor(2000+random()*8000):null}];
  const entries:Case["entries"]=[],count=Math.floor(random()*60);
  for(let index=0;index<count;index++)entries.push({area:areas[Math.floor(random()*3)]!.id,id:"k"+String(index).padStart(4,"0"),access:Math.floor(random()*(random()<0.3?5:10000)),size:Math.floor(1+random()*600),protected:random()<0.15});
  cases.push({policy:{capBytes:Math.floor(1000+random()*8000),highWatermark:0.9,lowWatermark:0.5+Math.floor(random()*3)/10},partitionNeedBytes:random()<0.5?Math.floor(random()*3000):0,externalBytes:Math.floor(random()*500),nowMs:10000,areas,entries});
 }
 const python=oracle(structuredClone(cases));let evicting=0,reasons=new Set<string>();
 cases.forEach((row,index)=>{const actual=plan(row);expect(actual,"case "+index).toEqual(python[index]!);if(Object.keys(actual).length)evicting++;for(const reason of Object.values(actual))reasons.add(reason);});
 expect(evicting).toBeGreaterThan(150);expect([...reasons].sort()).toEqual(["age","area","host"]);
});

test("the policy is authored in both languages and rejects unordered watermarks",()=>{
 for(const key of Object.keys(CAS_MESSAGES_V1) as (keyof typeof CAS_MESSAGES_V1)[]){expect(casTextV1(key,"en").length,key).toBeGreaterThan(0);expect(casTextV1(key,"de").length,key).toBeGreaterThan(0);}
 expect(CAS_POLICY_V1.capBytes).toBe(256*1024**3);expect(CAS_POLICY_V1.areas.map(area=>area.id)).toEqual(["graph-memo"]);
 expect(()=>requireCasPolicyV1({...CAS_POLICY_V1,lowWatermark:0.95})).toThrow();
 expect(casRootV1({},"linux","/h")).toBe("/h/.cache/semio/cas");expect(casRootV1({LOCALAPPDATA:"C:\\L"},"win32","C:\\u")).toContain("semio");expect(casRootV1({SEMIO_CAS_ROOT:"/x"},"linux","/h")).toBe("/x");
});

test("two workspaces writing the same objects concurrently keep one stored copy",async()=>{
 const root=scratch("dedupe"),first=createCasStoreV1({root}),second=createCasStoreV1({root}),blobs=Array.from({length:12},(_,index)=>Buffer.alloc(256*1024+index,index+1));
 const [left,right]=await Promise.all([Promise.all(blobs.map(blob=>first.put(blob))),Promise.all(blobs.map(blob=>second.put(blob)))]);
 expect(left).toEqual(right);expect(left.map(row=>row.hash)).toEqual(blobs.map(blob=>sha(blob)));
 const usage=await casUsageV1(root),one=blobs.reduce((sum,blob)=>sum+blob.length,0);
 expect(usage.objects).toBe(blobs.length);expect(usage.bytes).toBeLessThanOrEqual(one*1.1);
 expect(readdirSync(join(root,"v1","t"))).toEqual([]);
 expect(await first.has([left[0]!.hash,sha("absent")])).toEqual([true,false]);
 expect(Buffer.from((await second.read(left[3]!.hash))!).equals(blobs[3]!)).toBe(true);
 const destination=join(scratch("link"),"deep","copy.bin");expect(["reflink","copy"]).toContain(await first.link(left[1]!.hash,destination));expect(readFileSync(destination).equals(blobs[1]!)).toBe(true);
 expect(await first.link(sha("absent"),destination+"2")).toBeUndefined();
});

test("adoption replaces a workspace file by one shared object and streams are verified",async()=>{
 const root=scratch("adopt"),store=createCasStoreV1({root}),area=scratch("workspace"),file=join(area,"a.bin"),body=Buffer.alloc(300000,7);writeFileSync(file,body);
 const digest=await store.adopt(file);expect(digest.hash).toBe(sha(body));expect(digest.size).toBe(body.length);
 expect((await casUsageV1(root)).objects).toBe(1);
 async function* chunks(){yield body.subarray(0,1000);yield body.subarray(1000);}
 expect((await store.put(chunks())).hash).toBe(digest.hash);expect((await casUsageV1(root)).objects).toBe(1);
 const abort=new AbortController();abort.abort();await expect(store.put(chunks(),abort.signal)).rejects.toBeDefined();expect(readdirSync(join(root,"v1","t"))).toEqual([]);
});

const quietGuard=(root:string,policy:CasPolicyV1=CAS_POLICY_V1,now:()=>number=Date.now)=>createCasGuardV1({root,policy,now,partition:()=>undefined,usage:async()=>0});

function fill(root:string,count:number,sizeBytes:number,clock:{value:number}){
 const policy={...CAS_POLICY_V1,touchGranularityMs:0},store=createCasStoreV1({root,now:()=>clock.value,policy,guard:quietGuard(root,policy,()=>clock.value)});
 return{store,put:async(index:number)=>{clock.value=2_000_000_000+index*1000;const body=Buffer.alloc(sizeBytes,index%251);body.writeUInt32BE(index,0);const digest=await store.put(body);const path=store.objectPath(digest.hash),at=new Date(1_000_000+index*1000);utimesSync(path,at,at);return digest.hash;}};
}
const small:CasPolicyV1={...CAS_POLICY_V1,capBytes:20*MIB,minAgeMs:0,areas:[]};

test("collection frees the oldest objects down to the low watermark, keeps pinned and freshly touched ones, reports progress in both languages",async()=>{
 const root=scratch("gc"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 await store.pin("agent-1",[hashes[0]!],60_000);clock.value+=5;
 const events:string[]=[],started=Date.now();
 const result=await collectCasV1({root,policy:small,now:()=>clock.value,progress:event=>{events.push(event.stage);}});
 expect(result.status).toBe("collected");expect(result.partial).toBe(false);
 const left=await store.has(hashes);expect(left[0]).toBe(true);
 const usage=await casUsageV1(root);expect(usage.bytes).toBeLessThanOrEqual(small.capBytes*small.lowWatermark);expect(usage.bytes).toBeGreaterThan(small.capBytes*small.lowWatermark-2*MIB);
 const evicted=left.map((kept,index)=>kept?-1:index).filter(index=>index>=0);expect(evicted).toEqual(evicted.map((_,position)=>position+1));
 expect(result.evicted).toBe(evicted.length);expect(result.byReason.host).toBe(evicted.length);
 expect(formatCasCollectionV1(result,small,"en")).toContain("freed");expect(formatCasCollectionV1(result,small,"de")).toContain("freigegeben");
 expect(Date.now()-started).toBeLessThan(30000);
 const again=await collectCasV1({root,policy:small,now:()=>clock.value});expect(again.status).toBe("idle");expect(again.evicted).toBe(0);
 expect(formatCasCollectionV1(again,small,"de")).toContain("nichts zu bereinigen");
});

test("a lease protects, an expired lease does not, and a dry run removes nothing",async()=>{
 const root=scratch("lease"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 await store.pin("reader",hashes.slice(0,8),60_000);
 const dry=await collectCasV1({root,policy:small,now:()=>clock.value,dryRun:true});expect(dry.evicted).toBeGreaterThan(0);expect((await casUsageV1(root)).objects).toBe(24);
 await collectCasV1({root,policy:small,now:()=>clock.value});expect((await store.has(hashes.slice(0,8))).every(Boolean)).toBe(true);
 clock.value+=3_600_000+1;
 await collectCasV1({root,policy:{...small,capBytes:5*MIB},now:()=>clock.value});
 expect((await store.has(hashes.slice(0,8))).some(Boolean)).toBe(false);
 await expect(store.pin("bad holder",[],1)).rejects.toThrow();
});

test("an object read between the plan and the removal survives, and a concurrent collector is refused",async()=>{
 const root=scratch("race"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 let racing:Promise<unknown>|undefined,second:string|undefined;
 const result=await collectCasV1({root,policy:small,now:()=>clock.value,beforeEvict:async path=>{
  if(!second)second=(await collectCasV1({root,policy:small,now:()=>clock.value})).status;
  if(path===store.objectPath(hashes[1]!)){clock.value+=10;racing=store.get(hashes[1]!);await racing;}
 }});
 expect(second).toBe("locked");expect(result.skipped).toBeGreaterThanOrEqual(1);expect((await store.has([hashes[1]!]))[0]).toBe(true);
});

test("a manual collection waits for the running one and then collects, and the holder's heartbeat proves its progress",async()=>{
 const root=scratch("join"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock);
 for(let index=0;index<24;index++)await put(index);
 const lock=join(root,"v1","gc.lock"),beats:number[]=[];let waiting:Promise<CasCollectionV1>|undefined;
 const first=await collectCasV1({root,policy:{...small,checkEvery:1,progressMs:0},now:()=>clock.value,beforeEvict:async()=>{
  clock.value+=1000;await new Promise(done=>setTimeout(done,20));beats.push(JSON.parse(readFileSync(lock,"utf8")).atMs);
  waiting??=collectCasV1({root,policy:small,now:()=>clock.value,waitMs:10_000});await new Promise(done=>setTimeout(done,150));
 }});
 expect(first.status).toBe("collected");expect(new Set(beats).size).toBeGreaterThan(1);expect(beats).toEqual([...beats].sort((left,right)=>left-right));
 const joined=await waiting!;expect(joined.status).not.toBe("locked");expect(existsSync(lock)).toBe(false);
});

test("cancellation stops promptly with a consistent store and the next run completes the work",async()=>{
 const root=scratch("cancel"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 const abort=new AbortController();let seen=0;
 const part=await collectCasV1({root,policy:{...small,checkEvery:1,progressMs:0},now:()=>clock.value,signal:abort.signal,progress:event=>{if(event.stage==="evict"&&++seen===2)abort.abort();}});
 expect(part.partial).toBe(true);expect(part.evicted).toBeGreaterThan(0);
 for(const [index,hash] of hashes.entries())if((await store.has([hash]))[0])expect(sha(readFileSync(store.objectPath(hash)))).toBe(hash);else expect(index).toBeLessThan(24);
 const full=await collectCasV1({root,policy:small,now:()=>clock.value});expect(full.partial).toBe(false);
 expect((await casUsageV1(root)).bytes).toBeLessThanOrEqual(small.capBytes*small.lowWatermark);
});

test("killing the collector mid-run leaves only whole objects, a stale lock is taken over, and the next run recovers",async()=>{
 const root=scratch("crash"),clock={value:Date.now()},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<30;index++)hashes.push(await put(index));
 const child=join(scratch("child"),"child.ts");
 writeFileSync(child,`import {collectCasV1} from ${JSON.stringify(fileURLToPath(new URL("../🧹️collector/🟦️.ts",import.meta.url)))};
await collectCasV1({root:${JSON.stringify(root)},policy:${JSON.stringify({...small,checkEvery:1,progressMs:0})},beforeEvict:async()=>{process.stdout.write("x");await new Promise(done=>setTimeout(done,150));}});`);
 const process_=spawn(process.execPath,[child],{stdio:["ignore","pipe","inherit"]});
 await new Promise<void>(done=>{let marks=0;process_.stdout.on("data",()=>{if(++marks>=3){process_.kill("SIGKILL");done();}});process_.on("exit",()=>done());});
 await new Promise(done=>process_.once("exit",done));
 for(const hash of hashes)if((await store.has([hash]))[0])expect(sha(readFileSync(store.objectPath(hash)))).toBe(hash);
 expect(statSync(join(root,"v1","gc.lock")).isFile()).toBe(true);
 const recovered=await collectCasV1({root,policy:small});expect(["collected","idle"]).toContain(recovered.status);
 expect((await casUsageV1(root)).bytes).toBeLessThanOrEqual(small.capBytes*small.lowWatermark);
});

test("workspace source closures are accounted for while the host collector evicts separate CAS objects and graph memo files",async()=>{
 const {casAreasOfWorkspaceV1}=await import("../📂️areas/🟦️.ts"),root=scratch("graph-custody"),workspace=scratch("graph-ws"),home=scratch("graph-home");
 const areas=await casAreasOfWorkspaceV1(workspace,home),closures=areas[0]!.directory,memo=join(home,".cache","semio","repo","wt","journal","graph-memo");
 mkdirSync(closures,{recursive:true});mkdirSync(memo,{recursive:true});
 const after=Date.now(),old=new Date(after-9*86_400_000);
 const put=(path:string,size:number)=>{writeFileSync(path,Buffer.alloc(size,1));utimesSync(path,old,old);return path;};
 const leaf=put(join(closures,sha("closure")+".json"),2*MIB),oldMemo=put(join(memo,"memo.plan"),MIB);
 const object=join(root,"v1","o","ab","c".repeat(62));mkdirSync(join(root,"v1","o","ab"),{recursive:true});put(object,MIB);
 const policy={...CAS_POLICY_V1,capBytes:2*MIB,areas:[...CAS_POLICY_V1.areas,{id:"build-input-closures",capBytes:1,maxAgeMs:1}]};
 const options={root,policy,now:()=>after,partition:{partition:"fixture",totalBytes:100*MIB,freeBytes:5*MIB}};
 const found=await casAreasOfWorkspaceV1(workspace,home),preview=await collectCasV1({...options,areas:found,dryRun:true});
 expect(preview.evicted).toBe(2);expect(existsSync(leaf)&&existsSync(object)&&existsSync(oldMemo)).toBe(true);
 const collected=await collectCasV1({...options,areas:found});
 expect(collected.totals).toMatchObject({totalBytes:4*MIB,protectedBytes:2*MIB,candidateBytes:2*MIB,keptBytes:2*MIB});
 expect(collected.byReason).toEqual({age:1,area:0,host:1});expect(existsSync(leaf)).toBe(true);expect(existsSync(object)||existsSync(oldMemo)).toBe(false);
 const idle=await collectCasV1({root,policy,now:()=>after,areas:found});
 expect(idle.status).toBe("idle");expect(idle.totals.shortfallBytes).toBeGreaterThan(0);
 for(const language of ["en","de"] as const)expect(formatCasCollectionV1(idle,policy,language)).toContain("repo cache gc --area=graph-inference");
});

test("loose areas count against the host cap: optional files age out and are capped, recent files and other data stay",async()=>{
 const root=scratch("areas"),memo=scratch("memo"),closures=scratch("closures"),clock={value:2_000_000_000_000},age=(path:string,ms:number)=>{const at=new Date(clock.value-ms);utimesSync(path,at,at);};
 mkdirSync(join(closures,"a"),{recursive:true});
 const day=86_400_000,make=(directory:string,name:string,size:number,old:number)=>{const path=join(directory,name);writeFileSync(path,Buffer.alloc(size,1));age(path,old);return path;};
 const oldMemo=make(memo,"x.plan",MIB,9*day),freshMemo=make(memo,"y.plan",MIB,day),records=[1,2,3,4].map(index=>make(memo,`r${index}.plan`,MIB,(10-index)*1000+60_000)),partial=make(memo,".z.tmp",MIB,9*day);
 const oldClosure=make(join(closures,"a"),"c.json.br",MIB,8*day),freshClosure=make(join(closures,"a"),"d.json.br",MIB,2*day);
 const bystander=join(root,"other.txt");mkdirSync(root,{recursive:true});writeFileSync(bystander,"user data");
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:100*MIB,minAgeMs:30_000,areas:[{id:"graph-memo",capBytes:3*MIB,maxAgeMs:7*day},{id:"optional-files",capBytes:null,maxAgeMs:7*day}]};
 const result=await collectCasV1({root,policy,now:()=>clock.value,areas:[{id:"graph-memo",directory:memo},{id:"optional-files",directory:closures}]});
 expect(result.byReason.age).toBe(2);expect(() => statSync(oldMemo)).toThrow();expect(() => statSync(oldClosure)).toThrow();
 expect(statSync(freshClosure).isFile()).toBe(true);expect(statSync(partial).isFile()).toBe(true);expect(statSync(bystander).isFile()).toBe(true);
 const remaining=[freshMemo,...records].filter(path=>{try{statSync(path);return true;}catch{return false;}});
 expect(remaining).toEqual([records[2]!,records[3]!]);expect(result.byReason.area).toBe(3);
 const host=await collectCasV1({root,policy:{...policy,capBytes:4*MIB},now:()=>clock.value,areas:[{id:"graph-memo",directory:memo},{id:"optional-files",directory:closures}],externalBytes:2*MIB});
 expect(host.byReason.host).toBeGreaterThan(0);expect(statSync(bystander).isFile()).toBe(true);
});

test("loose files a live process pins survive an aged-out collection of their area, and fall to it once the pin is released",async()=>{
 const root=scratch("pinned-closures"),closures=scratch("pinned-closure-files"),home=scratch("pinned-home"),day=86_400_000,environment={HOME:home,SEMIO_CAS_ROOT:root};
 const digests=["a","b","c"].map(name=>sha("closure-"+name)),paths=digests.map(digest=>{const path=join(closures,digest+".json");writeFileSync(path,Buffer.alloc(1024,1));const at=new Date(Date.now()-9*day);utimesSync(path,at,at);return path;});
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:100*MIB,minAgeMs:30_000,areas:[{id:"optional-files",capBytes:null,maxAgeMs:7*day}]},areas=[{id:"optional-files",directory:closures}];
 const alive=()=>paths.map(path=>existsSync(path));
 await casPinClosuresV1([digests[0]!],environment);
 const pinned=await collectCasV1({root,policy,areas});
 expect(alive()).toEqual([true,false,false]);expect(pinned.byReason.age).toBe(2);expect(pinned.totals.protectedBytes).toBe(1024);
 await releaseCasPinsV1(root,"graph-closures."+process.pid);
 const released=await collectCasV1({root,policy,areas});
 expect(alive()).toEqual([false,false,false]);expect(released.byReason.age).toBe(1);
 await casPinClosuresV1([digests[0]!],{});
 expect(existsSync(join(root,"v1","p","graph-closures."+process.pid+".json"))).toBe(false);
});

test("a pin made while the collector runs protects the object at once", async()=>{
 const root=scratch("latepin"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 let pinned=false;
 const result=await collectCasV1({root,policy:small,now:()=>clock.value,beforeEvict:async path=>{if(!pinned&&path===store.objectPath(hashes[2]!)){pinned=true;await store.pin("late",[hashes[2]!],60_000);}}});
 expect(pinned).toBe(true);expect((await store.has([hashes[2]!]))[0]).toBe(true);expect(result.skipped).toBeGreaterThanOrEqual(1);
});

test("the command reports and collects the host store and the workspace and memo areas, in both languages, and refuses bad arguments",async()=>{
 const {runCasCommandV1}=await import("../⌨️command/🟦️.ts"),{casAreasOfWorkspaceV1}=await import("../📂️areas/🟦️.ts"),root=scratch("cmd"),home=scratch("home"),workspace=scratch("ws"),lines:string[]=[],stages:string[]=[];
 mkdirSync(join(workspace,".cache","b","g","m","build-input-closures"),{recursive:true});mkdirSync(join(home,".cache","semio","repo","wt1","journal","graph-memo"),{recursive:true});
 writeFileSync(join(home,".cache","semio","repo","wt1","journal","graph-memo","old.plan"),"x");
 const old=new Date(Date.now()-30*86_400_000);utimesSync(join(home,".cache","semio","repo","wt1","journal","graph-memo","old.plan"),old,old);
 expect((await casAreasOfWorkspaceV1(workspace,home)).map(area=>area.id)).toEqual(["build-input-closures","graph-memo"]);
 const context={workspace,home,partition:()=>undefined,environment:{SEMIO_CAS_ROOT:root},write:(line:string)=>{lines.push(line);}},ports={signal:new AbortController().signal,progress:async(stage:string)=>{stages.push(stage);}};
 expect(await runCasCommandV1(["status","--lang=de"],context,ports)).toBe(0);expect(lines.join("\n")).toContain("Objekte");expect(lines.join("\n")).toContain("Wirksame Grenze");
 expect(await runCasCommandV1(["status","--json"],context,ports)).toBe(0);expect(JSON.parse(lines.at(-1)!)).toMatchObject({version:1,incrementBytes:CAS_POLICY_V1.incrementBytes,transientLeaseMs:CAS_POLICY_V1.transientLeaseMs,effectiveCapBytes:CAS_POLICY_V1.capBytes,transient:{leased:0,expired:0}});
 for(const [language,planned,adopted] of [["en","would remove 1","could share"],["de","Einträge zur Entfernung: 1","könnten"]] as const){
  expect(await runCasCommandV1(["gc","--dry-run","--lang="+language],context,ports)).toBe(0);
  expect(lines.at(-1)).toContain(planned);
  expect(readFileSync(join(home,".cache","semio","repo","wt1","journal","graph-memo","old.plan"),"utf8")).toBe("x");
  expect(await runCasCommandV1(["adopt","--dry-run","--lang="+language],context,ports)).toBe(0);
  expect(lines.at(-1)).toContain(adopted);
 }
 expect(await runCasCommandV1(["gc","--json"],context,ports)).toBe(0);expect(JSON.parse(lines.at(-1)!)).toMatchObject({version:1,status:"collected",evicted:1,byReason:{age:1}});
 expect(await runCasCommandV1(["gc"],context,ports)).toBe(0);expect(lines.at(-1)).toContain("nothing to collect");
 expect(await runCasCommandV1(["gc","--lang=de"],context,ports)).toBe(0);expect(lines.at(-1)).toContain("nichts zu bereinigen");
 await expect(runCasCommandV1(["gc","--bogus"],context,ports)).rejects.toThrow();
});

test("the remote cache fronts the host store: blobs of two cache directories share one object, bad digests are refused, and an object the host collector removed is a plain miss",async()=>{
 const {RemoteCacheStoreV1}=await import("../../🌐️remote-cache/💾️store/🟦️.ts"),root=scratch("front"),policy:CasPolicyV1={...CAS_POLICY_V1,minAgeMs:0},host=createCasStoreV1({root,policy,guard:quietGuard(root,policy)}),body=Buffer.alloc(100000,9),key=sha(body);
 const open=(directory:string)=>RemoteCacheStoreV1.open({directory,maximumBytes:50*MIB,maximumEntryBytes:10*MIB,maximumAgeMilliseconds:0,host});
 const first=await open(scratch("rc1")),second=await open(scratch("rc2")),stream=async function*(){yield body.subarray(0,500);yield body.subarray(500);};
 expect(await first.put("main","cas",key,stream(),{mode:"content",verify:true,declaredLength:body.length})).toBe("created");
 expect(await second.put("main","cas",key,stream(),{mode:"content",verify:true})).toBe("exists");
 expect((await casUsageV1(root)).objects).toBe(1);
 expect(await second.has("other","cas",key)).toBe(true);expect(Buffer.from((await first.readBytes("main","cas",key,MIB))!).equals(body)).toBe(true);
 expect(await first.put("main","cas",sha("x"),stream(),{mode:"content",verify:true})).toBe("digest-mismatch");
 expect(await first.put("main","cas",key,stream(),{mode:"content",verify:true,declaredLength:5})).toBe("length-mismatch");
 expect((await casUsageV1(root)).objects).toBe(1);expect(readdirSync(join(root,"v1","t"))).toEqual([]);
 rmSync(host.objectPath(key));
 expect(await first.has("main","cas",key)).toBe(false);expect(await first.read("main","cas",key)).toBeUndefined();expect(await first.readBytes("main","cas",key,MIB)).toBeUndefined();
 const small=Buffer.from("action");await first.put("main","ac",sha("a"),(async function*(){yield small;})(),{mode:"replace"});
 expect(await first.has("main","ac",sha("a"))).toBe(true);
 rmSync(join(first.root,"main","ac",sha("a").slice(0,2),sha("a")));
 expect(await first.has("main","ac",sha("a"))).toBe(false);expect(await first.put("main","ac",sha("a"),(async function*(){yield small;})(),{mode:"keep"})).toBe("created");
 await first.close();await second.close();
});

test("adoption makes equal finished files of two workspaces one stored object, leaves young, small, volatile and differently-moded files alone, is idempotent, and an independent Python census agrees",async()=>{
 const {adoptCasV1,casPlacesOfWorkspaceV1}=await import("../🧲️adopt/🟦️.ts"),root=scratch("adopt-cas"),first=scratch("wsA"),second=scratch("wsB"),policy={minBytes:1000,quietMs:60_000,clearWriteBits:true,skipPattern:"(^\\.|\\.(db|log|tmp)$)",places:[{id:"deps",path:"deps",mode:"hardlink" as const,reason:"test"}]};
 const clock=2_000_000_000_000,old=new Date(clock-86_400_000),put=(workspace:string,name:string,body:Buffer|string,mode=0o644,time=old)=>{const path=join(workspace,"deps",name);mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,body);chmodSync(path,mode);utimesSync(path,time,time);return path;};
 const same=Buffer.alloc(5000,3),other=Buffer.alloc(7000,4);
 const a1=put(first,"a/same.bin",same),a2=put(first,"b/copy.bin",same),a3=put(first,"only.bin",other),a4=put(first,"small.bin",Buffer.alloc(10,1)),a5=put(first,"x.log",same),a6=put(first,"young.bin",Buffer.alloc(3000,9),0o644,new Date(clock-1000)),a7=put(first,"tool.sh",same,0o755);
 const b1=put(second,"nested/same.bin",same),b2=put(second,"only.bin",other);
 const run=(workspace:string,dryRun=false)=>adoptCasV1({root,policy,places:casPlacesOfWorkspaceV1(workspace,policy.places),now:()=>clock,dryRun});
 const dry=await run(first,true);expect(dry.dryRun).toBe(true);expect(statSync(a1).nlink).toBe(1);
 const one=await run(first),two=await run(second);
 expect(one.objects).toBe(2);expect(one.skipped).toMatchObject({small:1,volatile:1,young:1});expect(one.skipped.mode).toBe(1);
 expect(statSync(a1).ino).toBe(statSync(a2).ino);expect(statSync(a1).ino).toBe(statSync(b1).ino);expect(statSync(a3).ino).toBe(statSync(b2).ino);
 expect(statSync(a1).mode&0o222).toBe(0);expect(statSync(a7).ino).not.toBe(statSync(a1).ino);expect(statSync(a4).nlink).toBe(1);expect(statSync(a5).nlink).toBe(1);expect(statSync(a6).nlink).toBe(1);
 expect(two.linked).toBe(2);expect(two.savedBytes).toBe(same.length+other.length);expect(readFileSync(b1).equals(same)).toBe(true);expect(readFileSync(b2).equals(other)).toBe(true);
 const again=await run(second);expect(again.linked).toBe(0);expect(again.skipped.linked).toBe(2);
 const objects=readdirSync(join(root,"v1","o")).flatMap(shard=>readdirSync(join(root,"v1","o",shard)));expect(objects.length).toBe(2);
 const python=spawnSync(testPythonV1()??"python3",["-I","-c",`
import hashlib,json,os,sys
import {ownProcessIdentityV1} from "../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";
roots=json.loads(sys.argv[1]);groups={}
for r in roots:
  for d,_,fs in os.walk(r):
    for f in fs:
      p=os.path.join(d,f);s=os.lstat(p)
      if s.st_size<1000 or f.endswith(('.log','.db','.tmp')):continue
      groups.setdefault((hashlib.sha256(open(p,'rb').read()).hexdigest(),s.st_mode&0o7777&~0o222),set()).add(s.st_ino)
print(json.dumps(sorted(len(v) for v in groups.values())))`,JSON.stringify([join(first,"deps"),join(second,"deps")])],{encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);expect(JSON.parse(python.stdout)).toEqual([1,1,1,1]);
 const strict:CasPolicyV1={...CAS_POLICY_V1,capBytes:1000,minAgeMs:0,areas:[]};
 const kept=await collectCasV1({root,policy:strict,now:()=>clock*2});expect(kept.evicted).toBe(0);expect(kept.totals.protectedBytes).toBeGreaterThan(0);
 for(const path of [a1,a2,b1,a3,b2]){chmodSync(path,0o644);rmSync(path);}
 const freed=await collectCasV1({root,policy:strict,now:()=>clock*2});expect(freed.evicted).toBeGreaterThanOrEqual(2);
});

test("Nx task cache entries written through the remote cache land once in the host store and read back, and a collected object is a miss",async()=>{
 const {RemoteCacheStoreV1}=await import("../../🌐️remote-cache/💾️store/🟦️.ts"),root=scratch("nx-front"),host=createCasStoreV1({root}),body=Buffer.alloc(40000,5);
 const open=(directory:string)=>RemoteCacheStoreV1.open({directory,maximumBytes:50*MIB,maximumEntryBytes:10*MIB,maximumAgeMilliseconds:0,host});
 const one=await open(scratch("nx1")),two=await open(scratch("nx2")),stream=async function*(){yield body;};
 expect(await one.put("main","nx","taskhash1",stream(),{mode:"keep"})).toBe("created");
 expect(await two.put("main","nx","other-task",stream(),{mode:"keep"})).toBe("created");
 expect((await casUsageV1(root)).objects).toBe(1);
 expect(await one.put("main","nx","taskhash1",stream(),{mode:"keep"})).toBe("exists");
 const entry=(await one.read("main","nx","taskhash1"))!;expect(entry.size).toBe(body.length);expect(readFileSync(entry.path).equals(body)).toBe(true);
 expect(Buffer.from((await two.readBytes("main","nx","other-task",MIB))!).equals(body)).toBe(true);
 rmSync(host.objectPath(sha(body)));
 expect(await one.has("main","nx","taskhash1")).toBe(false);expect(await one.read("main","nx","taskhash1")).toBeUndefined();
 expect(await one.put("main","nx","taskhash1",stream(),{mode:"keep"})).toBe("created");
 await one.close();await two.close();
});

test("a real Nx task of one workspace is restored from the host-store-backed remote cache by a second workspace with its own empty cache, and the archive is stored once",async()=>{
 const cache=join(process.env.HOME??"",".bun/install/cache"),versions=(()=>{try{return readdirSync(cache).filter(name=>/^nx@\d+\.\d+\.\d+@@@1$/u.test(name)).sort().reverse();}catch{return [];}})(),version=versions.find(name=>existsSync(join(cache,"@nx",`nx-linux-x64-gnu@${name.slice(3,name.indexOf("@@@"))}@@@1`)));
 const installation=join(import.meta.dir,"../../../../../../../../.nx/installation/node_modules");
 if(!version||process.platform!=="linux"||!existsSync(installation)||spawnSync(registeredToolExecutableV1("node"),["--version"]).status!==0)return;
 const number=version.slice(3,version.indexOf("@@@")),nx=join(cache,version,"dist/bin/nx.js"),modules=scratch("nx-mods");
 mkdirSync(join(modules,"@nx"),{recursive:true});symlinkSync(join(cache,"@nx",`nx-linux-x64-gnu@${number}@@@1`),join(modules,"@nx","nx-linux-x64-gnu"),"dir");
 const {startRemoteCacheServerV1}=await import("../../🌐️remote-cache/🖥️server/🟦️.ts"),{nxRemoteCacheEnvironmentV1}=await import("../../🌐️remote-cache/🧶️nx/🟦️.ts"),root=scratch("nx-real"),host=createCasStoreV1({root});
 const server=await startRemoteCacheServerV1({hostStore:host,directory:scratch("nx-real-rc"),host:"127.0.0.1",port:0,maximumBytes:200*MIB,maximumEntryBytes:50*MIB,maximumAgeDays:1,defaultInstance:"main",tokens:{}});
 try{
  const environment=await nxRemoteCacheEnvironmentV1({},{endpoint:server.url});
  expect(Object.keys(environment).sort()).toEqual(["NX_SELF_HOSTED_REMOTE_CACHE_ACCESS_TOKEN","NX_SELF_HOSTED_REMOTE_CACHE_SERVER"]);
  expect(await nxRemoteCacheEnvironmentV1({},{endpoint:"http://127.0.0.1:1"})).toEqual({});expect(await nxRemoteCacheEnvironmentV1({SEMIO_REMOTE_CACHE_NX:"off"},{endpoint:server.url})).toEqual({});
  const make=(name:string)=>{const directory=scratch(name);mkdirSync(join(directory,"a"),{recursive:true});writeFileSync(join(directory,"package.json"),'{"name":"w","private":true}');writeFileSync(join(directory,"nx.json"),'{"analytics":false}');
   writeFileSync(join(directory,"a","project.json"),JSON.stringify({name:"a",targets:{build:{executor:"nx:run-commands",cache:true,options:{command:"mkdir -p out && echo built > out/x.txt",cwd:"{projectRoot}"},inputs:[{runtime:"echo fixed"}],outputs:["{projectRoot}/out"]}}}));return directory;};
  const run=(directory:string)=>new Promise<{status:number|null;stdout:string;stderr:string}>(done=>{const child=spawn(registeredToolExecutableV1("node"),[nx,"run-many","-t","build","-p","a","--outputStyle=static"],{cwd:directory,env:{PATH:process.env.PATH,HOME:process.env.HOME,NODE_PATH:`${modules}:${installation}`,...environment,NX_DAEMON:"false",NX_NO_CLOUD:"true",NX_TUI:"false",NX_CACHE_DIRECTORY:join(directory,".cache"),NX_WORKSPACE_DATA_DIRECTORY:join(directory,".data")}});let stdout="",stderr="";child.stdout.on("data",chunk=>{stdout+=chunk;});child.stderr.on("data",chunk=>{stderr+=chunk;});const timer=setTimeout(()=>child.kill("SIGKILL"),120000);child.on("close",status=>{clearTimeout(timer);done({status,stdout,stderr});});});
  const first=make("nx-w1"),second=make("nx-w2"),one=await run(first);expect(one.status,one.stdout+one.stderr).toBe(0);
  const two=await run(second);expect(two.status,two.stdout+two.stderr).toBe(0);
  expect(readFileSync(join(second,"a","out","x.txt"),"utf8").trim()).toBe("built");expect(two.stdout+two.stderr).toMatch(/remote cache/iu);
  expect((await casUsageV1(root)).objects).toBe(1);
 }finally{await server.stop();}
},600000);

test("a write to an adopted file never reaches another workspace: hard-linked files are read-only, and a mutable place is only ever cloned copy-on-write or left alone",async()=>{
 const {adoptCasV1}=await import("../🧲️adopt/🟦️.ts"),root=scratch("isolate"),one=scratch("iso1"),two=scratch("iso2"),clock=2_000_000_000_000,old=new Date(clock-86_400_000),body=Buffer.alloc(4000,6);
 const put=(workspace:string,place:string,name:string)=>{const path=join(workspace,place,name);mkdirSync(join(path,".."),{recursive:true});writeFileSync(path,body);utimesSync(path,old,old);return path;};
 const rules={minBytes:1000,quietMs:60_000,clearWriteBits:true,skipPattern:"^\\.",places:[]};
 const a=put(one,"cas","f.bin"),b=put(two,"cas","g.bin");
 const places=(workspace:string)=>[{id:"immutable",directory:join(workspace,"cas"),mode:"hardlink" as const},{id:"mutable",directory:join(workspace,"inst"),mode:"reflink" as const}];
 await adoptCasV1({root,policy:rules,places:places(one),now:()=>clock});const second=await adoptCasV1({root,policy:rules,places:places(two),now:()=>clock});
 expect(second.linked).toBe(1);expect(statSync(a).ino).toBe(statSync(b).ino);
 if(process.getuid?.()!==0){expect(()=>writeFileSync(b,"tampered")).toThrow();expect(()=>appendFileSync(a,"x")).toThrow();}
 expect(readFileSync(a).equals(body)).toBe(true);expect(readFileSync(b).equals(body)).toBe(true);
 const m1=put(one,"inst","m.bin"),m2=put(two,"inst","n.bin");
 const ra=await adoptCasV1({root,policy:rules,places:places(one),now:()=>clock}),rb=await adoptCasV1({root,policy:rules,places:places(two),now:()=>clock});
 expect(statSync(m1).ino===statSync(m2).ino,"a mutable place is never hard-linked").toBe(false);
 writeFileSync(m1,"changed in place");expect(readFileSync(m2).equals(body)).toBe(true);expect(statSync(m1).mode&0o200).not.toBe(0);expect(ra.failed+rb.failed).toBe(0);
});

test("the hosted remote cache starts with the service signal, answers its health check on the policy port, stops with the signal, and a second host sees the port as taken without failing",async()=>{
 const {startHostedRemoteCacheV1}=await import("../../🌐️remote-cache/🏠️hosted/🟦️.ts"),root=scratch("hosted"),controller=new AbortController(),refusals:string[]=[];
 const environment={SEMIO_CAS_ROOT:root},first=await startHostedRemoteCacheV1({environment,signal:controller.signal,port:0});
 expect(first).toBeDefined();expect((await fetch(first!.url+"/healthz")).status).toBe(200);
 const port=Number(new URL(first!.url).port),second=await startHostedRemoteCacheV1({environment:{SEMIO_CAS_ROOT:scratch("hosted2")},signal:controller.signal,port,onUnavailable:reason=>{refusals.push(reason);}});
 expect(second).toBeUndefined();expect(refusals).toEqual(["busy"]);
 controller.abort();await first!.stop();await expect(fetch(first!.url+"/healthz",{signal:AbortSignal.timeout(500)})).rejects.toBeDefined();
 expect(existsSync(join(root,"remote-cache","v1"))).toBe(true);
});

const enforcementOracle=():{decisions:string[][];needs:number[]}=>{
 const run=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️enforcement.py",import.meta.url))],{input:JSON.stringify({policy:enforcement.policy,cases:enforcement.cases,needs:enforcement.needs}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);
};
const watermarks=enforcement.policy;

test("the admission rule gives the authored verdicts and the partition need the authored bytes, in TypeScript and in the independent Python oracle",()=>{
 const python=enforcementOracle();
 enforcement.cases.forEach((row,index)=>{
  const decision=decideCasEnforcementV1({policy:watermarks,storeBytes:row.storeBytes??undefined,incomingBytes:row.incomingBytes,partition:row.partition??undefined,collected:row.collected});
  expect([decision.action,decision.reason],row.name).toEqual([row.action,row.reason]);expect(python.decisions[index]!,row.name).toEqual([row.action,row.reason]);
 });
 enforcement.needs.forEach((row,index)=>{expect(casPartitionNeedBytesV1(row,watermarks),row.name).toBe(row.need);expect(python.needs[index],row.name).toBe(row.need);});
 expect(casPartitionNeedBytesV1(undefined,watermarks)).toBe(0);
});

test("the policy refuses unordered partition watermarks",()=>{
 for(const partition of [{highUsedFraction:0.9,lowUsedFraction:0.95,criticalUsedFraction:0.97},{highUsedFraction:0.9,lowUsedFraction:0.8,criticalUsedFraction:0.85},{highUsedFraction:0.9,lowUsedFraction:0.8,criticalUsedFraction:1.2}])expect(()=>requireCasPolicyV1({...CAS_POLICY_V1,partition}),JSON.stringify(partition)).toThrow();
 expect(CAS_POLICY_V1.partition).toEqual({highUsedFraction:0.9,lowUsedFraction:0.8,criticalUsedFraction:0.97});
});

test("a collection frees the least recently accessed objects down to the low share of a partition above its high share, and says so in both languages",async()=>{
 const root=scratch("partition"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:1024*MIB,minAgeMs:0,areas:[]},partition:CasPartitionV1={partition:"/volume",totalBytes:100*MIB,freeBytes:5*MIB};
 const quiet=await collectCasV1({root,policy,now:()=>clock.value,dryRun:true});expect(quiet.evicted).toBe(0);
 const result=await collectCasV1({root,policy,partition,now:()=>clock.value});
 expect(result.evicted).toBe(15);expect(result.freedBytes).toBe(15*MIB);expect(result.byReason.host).toBe(15);
 expect(result.partition).toEqual({partition:"/volume",totalBytes:100*MIB,usedFraction:0.95,needBytes:15*MIB});
 const left=await store.has(hashes);expect(left.slice(0,15).some(Boolean)).toBe(false);expect(left.slice(15).every(Boolean)).toBe(true);
 expect(formatCasCollectionV1(result,policy,"en")).toContain("Partition /volume was 95% used (high watermark 90%, low watermark 80%)");
 expect(formatCasCollectionV1(result,policy,"de")).toContain("Partition /volume war zu 95 % belegt (obere Marke 90 %, untere Marke 80 %)");
 const idle=await collectCasV1({root,policy,partition:{...partition,freeBytes:30*MIB},now:()=>clock.value});expect(idle.status).toBe("idle");
 const stuck=await collectCasV1({root,policy:{...policy,minAgeMs:3_600_000_000},partition,now:()=>clock.value});
 expect(stuck.evicted).toBe(0);expect(formatCasCollectionV1(stuck,policy,"en")).toContain("pressure is not the store's");expect(stuck.totals.partitionShortfallBytes).toBe(15*MIB);expect(stuck.totals.capShortfallBytes).toBe(0);
});

test("a collection on a full disk deletes without creating its lock, and the lock-less run is reported",async()=>{
 const root=scratch("full"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 const full=Object.assign(new Error("ENOSPC: no space left on device, open"),{code:"ENOSPC"});
 const result=await collectCasV1({root,policy:small,now:()=>clock.value,openLock:async()=>{throw full;}});
 expect(result.lockless).toBe(true);expect(result.status).toBe("collected");expect(result.evicted).toBeGreaterThan(0);
 expect(()=>statSync(join(root,"v1","gc.lock"))).toThrow();expect((await casUsageV1(root)).bytes).toBeLessThanOrEqual(small.capBytes*small.lowWatermark);
 expect((await store.has(hashes)).some(Boolean)).toBe(true);
 await expect(collectCasV1({root,policy:small,openLock:async()=>{throw Object.assign(new Error("EACCES"),{code:"EACCES"});}})).rejects.toThrow("EACCES");
});

test("a finished collection leaves a usage hint of what the store and its areas hold, and a dry run leaves none",async()=>{
 const root=scratch("hint"),clock={value:2_000_000_000},{put}=fill(root,0,MIB,clock);
 for(let index=0;index<24;index++)await put(index);
 await collectCasV1({root,policy:small,now:()=>clock.value,dryRun:true});expect(await readCasUsageHintV1(root)).toBeUndefined();
 const result=await collectCasV1({root,policy:small,now:()=>clock.value}),hint=await readCasUsageHintV1(root);
 expect(hint).toEqual({bytes:(await casUsageV1(root)).bytes,atMs:clock.value});expect(result.totals.keptBytes).toBe(hint!.bytes);
 writeFileSync(casUsageHintPathV1(root),"{broken");expect(await readCasUsageHintV1(root)).toBeUndefined();
});

const hinted=(root:string,bytes:number):string=>{mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes,atMs:1}));return root;};
const stub:CasCollectionV1={status:"collected",partial:false,dryRun:false,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:{totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,capShortfallBytes:0,partitionShortfallBytes:0,needBytes:0},byReason:{age:0,area:0,host:0}};
const critical:CasPartitionV1={partition:"/volume",totalBytes:1000*MIB,freeBytes:10*MIB};
const refusal=async(promise:Promise<unknown>):Promise<any>=>promise.then(()=>{throw Error("expected a refusal");},error=>error);

test("a partition at its critical share waits for one collection and is then refused with the typed low-space refusal that names the partition, the free bytes and the gc command",async()=>{
 resetCasGuardsV1();
 const root=hinted(scratch("guard-critical"),0);let collects=0;
 const guard=createCasGuardV1({root,partition:()=>critical,usage:async()=>0,collect:async()=>{collects++;return stub;}});
 const error=await refusal(guard.admit(0)) as {key:string;code:string};
 expect(collects).toBe(1);expect(isRepoErrorV1(error)).toBe(true);expect(error.key).toBe("process.disk.low-space");expect(error.code).toBe("refused");
 const english=renderRepoErrorV1(error as never,"en"),german=renderRepoErrorV1(error as never,"de");
 for(const text of [english,german]){expect(text).toContain("/volume");expect(text).toContain("10 MiB");expect(text).toContain("repo cache cas gc");}
 expect(english).toContain("Partition /volume has only 10 MiB free (99% used, limit 97%)");expect(german).toContain("Partition /volume hat nur noch 10 MiB frei");
 await refusal(guard.admit(0));expect(collects).toBe(1);
});

test("a write that times out stops waiting but the collection it started is not cancelled and finishes, so the store is relieved for the next write",async()=>{
 resetCasGuardsV1();
 const root=hinted(scratch("guard-outlives"),0);let free=10*MIB,collects=0,finished=false,seen:AbortSignal|undefined,release:()=>void=()=>undefined;
 const gate=new Promise<void>(done=>{release=done;});
 const guard=createCasGuardV1({root,partition:()=>({...critical,freeBytes:free}),usage:async()=>0,collect:async request=>{collects++;seen=request.signal;await gate;free=400*MIB;finished=true;return stub;}});
 const writer=new AbortController(),waiting=refusal(guard.admit(0,writer.signal));
 await Bun.sleep(30);writer.abort(new Error("write budget"));
 expect(((await waiting) as Error).message).toBe("write budget");
 expect(finished).toBe(false);expect(seen).toBeUndefined();
 release();await guard.settle();expect(finished).toBe(true);expect(collects).toBe(1);
 expect((await guard.admit(0)).action).toBe("allow");expect(collects).toBe(1);
});

test("a critical partition that the collection relieved lets the write go on",async()=>{
 resetCasGuardsV1();
 const root=hinted(scratch("guard-relieved"),0);let free=10*MIB,collects=0;
 const guard=createCasGuardV1({root,partition:()=>({...critical,freeBytes:free}),usage:async()=>0,collect:async()=>{collects++;free=400*MIB;return stub;}});
 expect((await guard.admit(0)).action).toBe("allow");expect(collects).toBe(1);
});

test("a partition above its high share starts one collection in the background and the write does not wait for it",async()=>{
 resetCasGuardsV1();
 const root=hinted(scratch("guard-high"),0);let collects=0,finished=false,release:()=>void=()=>undefined;
 const gate=new Promise<void>(done=>{release=done;});
 const guard=createCasGuardV1({root,partition:()=>({...critical,freeBytes:80*MIB}),usage:async()=>0,collect:async()=>{collects++;await gate;finished=true;return stub;}});
 expect((await guard.admit(0)).action).toBe("collect-background");expect((await guard.admit(0)).action).toBe("collect-background");
 expect(finished).toBe(false);expect(collects).toBe(1);release();await guard.settle();expect(finished).toBe(true);
 expect((await guard.admit(0)).action).toBe("allow");expect(collects).toBe(1);
});

test("the cap is judged on the usage hint plus what this process added since, and a failed collection never fails the write",async()=>{
 resetCasGuardsV1();
 const root=scratch("guard-cap"),policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:10*MIB};let collects=0;
 mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:int(9.2*MIB),atMs:1}));
 const guard=createCasGuardV1({root,policy,partition:()=>undefined,collect:async()=>{collects++;throw Error("scan failed");}});
 expect((await guard.admit(0)).action).toBe("collect-background");await guard.settle();expect(collects).toBe(1);
 resetCasGuardsV1();
 const second=createCasGuardV1({root,policy,partition:()=>undefined,collect:async()=>{collects++;return stub;}});
 expect((await second.admit(0)).action).toBe("collect-background");await second.settle();
 resetCasGuardsV1();writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:int(8*MIB),atMs:2}));
 const third=createCasGuardV1({root,policy,partition:()=>undefined,collect:async()=>{collects++;return stub;}});
 expect((await third.admit(0)).action).toBe("allow");third.account(3*MIB);await third.admit(0);expect(collects).toBe(3);
});
const int=(value:number):number=>Math.floor(value);

test("a store without a usage hint is measured in the background, never collected on a guess",async()=>{
 resetCasGuardsV1();
 const root=scratch("guard-measure");let scans=0,collects=0;
 const guard=createCasGuardV1({root,partition:()=>undefined,usage:async()=>{scans++;return 5;},collect:async()=>{collects++;return stub;}});
 expect((await guard.admit(0)).reason).toBe("unknown-usage");await guard.settle();expect(scans).toBe(1);expect((await guard.admit(0)).action).toBe("allow");expect(collects).toBe(0);
});

test("a write that fails on a full disk starts the collection, any other failure does not",async()=>{
 resetCasGuardsV1();
 const root=hinted(scratch("guard-relieve"),0);let collects=0;
 const guard=createCasGuardV1({root,partition:()=>undefined,usage:async()=>0,collect:async()=>{collects++;return stub;}});
 guard.relieve(Object.assign(new Error("EACCES"),{code:"EACCES"}));await guard.settle();expect(collects).toBe(0);
 guard.relieve(new Error("SQLiteError: database or disk is full"));await guard.settle();expect(collects).toBe(1);
});

test("the store refuses a put and an adoption on a critical partition with the typed refusal and leaves no temporary file, and a roomy partition changes nothing",async()=>{
 resetCasGuardsV1();
 const root=scratch("guard-store"),full=createCasGuardV1({root,partition:()=>critical,usage:async()=>0,collect:async()=>stub}),store=createCasStoreV1({root,guard:full}),file=join(scratch("guard-workspace"),"a.bin");
 writeFileSync(file,Buffer.alloc(70000,3));
 expect(((await refusal(store.put(Buffer.from("payload")))) as {key:string}).key).toBe("process.disk.low-space");
 expect(((await refusal(store.adopt(file))) as {key:string}).key).toBe("process.disk.low-space");
 expect(existsSync(join(root,"v1","t"))?readdirSync(join(root,"v1","t")):[]).toEqual([]);expect((await casUsageV1(root)).objects).toBe(0);
 expect(((await refusal(store.putChecked(Buffer.from("x"),{size:1}))) as {key:string}).key).toBe("process.disk.low-space");
 const roomy=createCasStoreV1({root,guard:quietGuard(root)}),digest=await roomy.put(Buffer.from("payload"));expect(digest.size).toBe(7);expect((await roomy.adopt(file)).size).toBe(70000);
});

test("the synchronous put is refused on a critical partition, starts the collection, and goes on when the partition is roomy",async()=>{
 resetCasGuardsV1();
 const root=scratch("guard-sync");let collects=0;
 const full=createCasGuardV1({root,partition:()=>critical,usage:async()=>0,collect:async()=>{collects++;return stub;}});
 expect(()=>putCasObjectSyncV1(root,"bytes",full)).toThrow();await full.settle();expect(collects).toBe(1);expect(existsSync(join(root,"v1","o"))).toBe(false);
 const hash=putCasObjectSyncV1(root,"bytes",quietGuard(root));expect(hash).toBe(sha("bytes"));expect(putCasObjectSyncV1(root,"bytes",full)).toBe(hash);
});

test("a service tick keeps the host store inside its watermarks: it collects above the high share, waits for it, does nothing below it and honours the cancellation",async()=>{
 resetCasGuardsV1();
 const root=hinted(scratch("tick"),0);let collects=0,free=80*MIB;
 const ports={root,partition:()=>({...critical,freeBytes:free}),usage:async()=>0,collect:async()=>{collects++;free=500*MIB;return stub;}};
 await casEnforceTickV1(ports);expect(collects).toBe(1);
 resetCasGuardsV1();await casEnforceTickV1(ports);expect(collects).toBe(1);
 const abort=new AbortController();abort.abort();await expect(casEnforceTickV1({...ports,signal:abort.signal})).rejects.toBeDefined();
});

test("the command reports the partition of the store and collects it down to the partition watermark, in both languages",async()=>{
 const {runCasCommandV1}=await import("../⌨️command/🟦️.ts"),root=scratch("cmd-partition"),home=scratch("cmd-home"),workspace=scratch("cmd-ws"),lines:string[]=[];
 const clock=Date.now(),old=new Date(clock-86_400_000);
 for(let index=0;index<8;index++){const path=join(root,"v1","o","a"+index);mkdirSync(path,{recursive:true});const file=join(path,"f".repeat(61)+index);writeFileSync(file,Buffer.alloc(MIB,index));utimesSync(file,old,old);}
 const context={workspace,home,environment:{SEMIO_CAS_ROOT:root},write:(line:string)=>{lines.push(line);},partition:()=>({partition:"/volume",totalBytes:100*MIB,freeBytes:5*MIB})},ports={signal:new AbortController().signal,progress:async()=>undefined};
 expect(await runCasCommandV1(["status","--lang=en"],context,ports)).toBe(0);expect(lines.at(-1)).toContain("Partition /volume: 95% used, 0 GiB free");
 expect(await runCasCommandV1(["status","--json"],context,ports)).toBe(0);expect(JSON.parse(lines.at(-1)!).partition).toMatchObject({partition:"/volume",usedFraction:0.95});
 expect(await runCasCommandV1(["gc","--lang=de"],context,ports)).toBe(0);expect(lines.at(-1)).toContain("Partition /volume war zu 95 % belegt");
 expect((await casUsageV1(root)).objects).toBe(0);
});

test("a shard view reads every segment once in batches, sees what live segments append, and does not look again at segments nobody wrote for the quiet time",async()=>{
 const root=scratch("view"),directory=casLedgerShardV1(root,"ab"),digest=(index:number)=>"ab"+String(index).padStart(62,"0"),day=86_400_000,now=Date.now();
 mkdirSync(directory,{recursive:true});
 for(let index=0;index<200;index++){const file=join(directory,`w${index}-old.jsonl`);writeFileSync(file,JSON.stringify({d:digest(index%50),t:1000+index})+"\n");const at=new Date(now-2*day);utimesSync(file,at,at);}
 writeFileSync(join(directory,"young.jsonl"),JSON.stringify({d:digest(7),t:5000})+"\n");
 const view=await openCasShardViewV1(root,"ab",{nowMs:now,quietMs:3_600_000}),folded=await foldCasShardV1(root,"ab");
 expect(new Map(view.snapshot())).toEqual(folded);expect(view.latest(digest(7))).toBe(5000);expect(view.latest(digest(60))).toBe(0);
 appendFileSync(join(directory,"young.jsonl"),JSON.stringify({d:digest(7),t:6000})+"\n"+JSON.stringify({d:digest(60),t:6001})+"\n");
 appendFileSync(join(directory,"w3-old.jsonl"),JSON.stringify({d:digest(3),t:9999})+"\n");
 await view.refresh();expect(view.latest(digest(7))).toBe(6000);expect(view.latest(digest(60))).toBe(6001);expect(view.latest(digest(3))).toBeLessThan(9999);
 writeFileSync(join(directory,"late.jsonl"),JSON.stringify({d:digest(61),t:7000})+"\n");await view.refresh();expect(view.latest(digest(61))).toBe(7000);
 const live=await openCasShardViewV1(root,"ab");expect(live.latest(digest(3))).toBe(9999);
});

test("a fresh guard reuses a recent matching automatic over-cap collection",async()=>{
 const corpus=enforcement.automaticCompletion;
 const expected=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️enforcement.py",import.meta.url))],{input:JSON.stringify({automaticCompletion:corpus}),encoding:"utf8"});
 expect(expected.status,expected.stderr).toBe(0);
 expect(JSON.parse(expected.stdout)).toEqual(corpus.rows.map(row=>row.collects));
 for(const row of corpus.rows){
  resetCasGuardsV1();
  const root=scratch("scoped-gc"),home=scratch("scoped-home"),area=join(home,".cache","semio","repo","private","journal","graph-memo");
  mkdirSync(area,{recursive:true});
  const first={value:corpus.clock},policy={...CAS_POLICY_V1,capBytes:corpus.capBytes,minAgeMs:corpus.minAgeMs},partition={...critical,freeBytes:400*MIB};
  const store=createCasStoreV1({root,policy,now:()=>first.value,guard:quietGuard(root,policy,()=>first.value)});
  await store.put(Buffer.alloc(row.priorBytes,3));
  writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:row.priorBytes,atMs:first.value-1}));
  const issuer=createCasGuardV1({root,home,policy,now:()=>first.value,partition:()=>partition});
  await issuer.admit(0);await issuer.settle();
  const persisted=JSON.parse(readFileSync(casUsageHintPathV1(root),"utf8"));
  expect([persisted.version,persisted.bytes,persisted.atMs]).toEqual([1,row.priorBytes,first.value]);
  expect(persisted.automaticScope).toMatch(/^[0-9a-f]{64}$/u);
  if(row.name==="matching"){
   const entry=new URL("../🚦️guard/🟦️.ts",import.meta.url).href,clock=first.value+row.advance;
   const script=`import{createCasGuardV1}from${JSON.stringify(entry)};
const guard=createCasGuardV1({root:${JSON.stringify(root)},home:${JSON.stringify(home)},policy:${JSON.stringify(policy)},partition:()=>(${JSON.stringify(partition)}),now:()=>${clock}});
console.log(JSON.stringify(await guard.admit(0)));
await guard.settle();`;
   const child=Bun.spawn([process.execPath,"-e",script],{stdout:"pipe",stderr:"pipe",windowsHide:true});
   const [out,err,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
   expect(status,err).toBe(0);
   expect(JSON.parse(out)).toEqual({action:"allow",reason:"fits"});
   expect((await readCasUsageHintV1(root))?.atMs).toBe(first.value);
  }
  if(row.scope==="manual")delete persisted.automaticScope;
  if(row.timestamp==="future")persisted.atMs=first.value+row.advance+1;
  if(row.scope==="manual"||row.timestamp==="future")writeFileSync(casUsageHintPathV1(root),JSON.stringify(persisted));
  if(row.scope==="area"){
   const moved=area+"-old";renameSync(area,moved);mkdirSync(area,{recursive:true});
  }
  resetCasGuardsV1();
  let calls=0,measures=0;
  const reader=createCasGuardV1({root,home,policy:{...policy,capBytes:row.cap,minAgeMs:row.scope==="age"?policy.minAgeMs/2:policy.minAgeMs},now:()=>first.value+row.advance,partition:()=>row.critical==="transition"?++measures>1?critical:partition:row.critical?critical:partition,collect:async()=>{calls++;return stub;}});
  if(row.priorBytes<row.cap){await reader.admit(0);await reader.settle();reader.account(row.cap-row.priorBytes+50);}
  if(row.critical)expect((await refusal(reader.admit(0))).key,row.name).toBe("process.disk.low-space");
  else expect((await reader.admit(0)).action,row.name).toBe(row.collects>0?"collect-background":"allow");
  await reader.settle();
  expect(calls,row.name).toBe(row.collects);
  if(row.name==="matching"){
   resetCasGuardsV1();
   const sync=createCasGuardV1({root,home,policy,now:()=>first.value+row.advance,partition:()=>partition,collect:async()=>{calls++;return stub;},usage:async()=>{throw Error("receipt supplies usage");}});
   expect(sync.admitSync(0).action).toBe("measure");
   await sync.settle();expect(calls).toBe(0);
   await sync.enforce();expect(calls).toBe(1);
   sync.relieve(Object.assign(Error("disk full"),{code:"ENOSPC"}));await sync.settle();expect(calls).toBe(2);
  }
 }
},60000);

test("CAS admission only trusts a completed collection of its actual scope",async()=>{
 const rows=enforcement.localCompletion;
 const python=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️enforcement.py",import.meta.url))],{input:JSON.stringify({localCompletion:rows}),encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 expect(JSON.parse(python.stdout)).toEqual(rows.map(row=>row.expected));
 for(const row of rows){
  const root=hinted(scratch("scoped-local"),0),home=scratch("scoped-local-home");let pressure=row.origin,calls=0;
  const partition=()=>pressure==="critical"?critical:{...critical,freeBytes:80*MIB};
  const collect=()=>{
   calls++;
   if(row.result==="throw")throw Error("collector failed before it returned");
   return Promise.resolve({...stub,status:row.result==="locked"?"locked":"collected",partial:row.result==="partial",dryRun:row.result==="dry",...row.result==="lockless"?{lockless:true as const}:{}});
  };
  const first=createCasGuardV1({root,home,partition,collect});
  if(pressure==="critical")expect((await refusal(first.admit(0))).key,row.name).toBe("process.disk.low-space");
  else await first.admit(0);
  await first.settle();expect(calls,row.name).toBe(1);
  pressure=row.request;
  const guard=row.owner==="same"?first:createCasGuardV1({root,home,policy:{...CAS_POLICY_V1,capBytes:CAS_POLICY_V1.capBytes/2},partition,collect});
  if(row.refused)expect((await refusal(guard.admit(0))).key,row.name).toBe("process.disk.low-space");
  else await guard.admit(0);
  await guard.settle();expect(calls,row.name).toBe(1+row.expected);
 }
},60000);

test("critical admission waits for a different in-flight scope, then coalesces the needed relief",async()=>{
 const root=hinted(scratch("critical-flight"),0);
 let pressure=80*MIB,release=()=>undefined,overlap=0,active=0;
 const stop=new Promise<void>(resolve=>{release=resolve;});
 const measured:number[]=[];
 const guard=createCasGuardV1({root,partition:()=>({...critical,freeBytes:pressure}),collect:async request=>{
  active++;overlap=Math.max(overlap,active);
  const free=request.partition!.freeBytes;measured.push(free);
  if(free===80*MIB)await stop;
  active--;return stub;
 }});
 await guard.admit(0);expect(measured).toEqual([80*MIB]);
 pressure=10*MIB;
 const pending=[refusal(guard.admit(0)),refusal(guard.admit(0))];
 try{await new Promise(resolve=>setTimeout(resolve,20));expect(measured).toEqual([80*MIB]);}
 finally{release();}
 for(const error of await Promise.all(pending))expect(error.key).toBe("process.disk.low-space");
 expect(measured).toEqual([80*MIB,10*MIB]);expect(overlap).toBe(1);
});

test("a native critical collection's local scope replaces an earlier high-pressure scan",async()=>{
 const root=hinted(scratch("native-critical-scope"),0),home=scratch("native-critical-home");
 let at=2_000_000_000,freeBytes=80*MIB;
 const make=()=>createCasGuardV1({root,home,partition:()=>({...critical,freeBytes}),now:()=>at});
 const high=make();await high.admit(0);await high.settle();
 const first=await readCasUsageHintV1(root);expect(first?.atMs).toBe(at);
 freeBytes=10*MIB;at++;
 expect((await refusal(make().admit(0))).key).toBe("process.disk.low-space");
 const second=await readCasUsageHintV1(root);expect(second?.atMs).toBe(at);
 expect(second?.automaticScope).not.toBe(first?.automaticScope);
 at++;
 expect((await refusal(make().admit(0))).key).toBe("process.disk.low-space");
 expect(await readCasUsageHintV1(root)).toEqual(second);
});

test("direct CAS relief is applied at the current pressure after an unrelated automatic scan",async()=>{
 for(const mode of ["explicit","disk-full"] as const){
  const root=hinted(scratch(mode),0);
  let freeBytes=80*MIB,release=()=>undefined;
  const blocked=new Promise<void>(resolve=>{release=resolve;}),scans:number[]=[];
  const guard=createCasGuardV1({root,partition:()=>({...critical,freeBytes}),collect:async request=>{
   scans.push(request.partition!.freeBytes);if(scans.length===1)await blocked;return stub;
  }});
  await guard.admit(0);expect(scans,mode).toEqual([80*MIB]);
  freeBytes=10*MIB;
  const done=mode==="explicit"?guard.enforce():(guard.relieve(Object.assign(Error("disk full"),{code:"ENOSPC"})),guard.settle());
  release();await done;expect(scans,mode).toEqual([80*MIB,10*MIB]);
 }
});

test("the cap in force is sized per role from the partition: share, floor, free room above the high watermark; the language-agnostic vectors, the independent Python oracle and the implementation agree",()=>{
 const GiB=2**30,vectors=JSON.parse(readFileSync(new URL("./📋️capacity/🔣️.json",import.meta.url),"utf8")) as {capBytes:number;minimumCapBytes:number;sizing:typeof CAS_POLICY_V1.sizing;rows:{role:CasRoleV1;total:number;free:number;store:number|null}[];expected:number[]};
 const oracle=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️capacity.py",import.meta.url))],{input:JSON.stringify(vectors),encoding:"utf8"});
 expect(oracle.status,oracle.stderr).toBe(0);const independent=JSON.parse(oracle.stdout) as number[];
 expect(independent).toEqual(vectors.expected);expect(vectors.sizing).toEqual(CAS_POLICY_V1.sizing);expect(vectors.capBytes).toBe(CAS_POLICY_V1.capBytes);
 vectors.rows.forEach((row,index)=>expect(casEffectiveCapBytesV1(CAS_POLICY_V1,{totalBytes:row.total,freeBytes:row.free,role:row.role},row.store??undefined),JSON.stringify(row)).toBe(vectors.expected[index]!));
 const at=(role:CasRoleV1,total:number,free:number,store?:number)=>casEffectiveCapBytesV1(CAS_POLICY_V1,{totalBytes:total*GiB,freeBytes:free*GiB,role},store===undefined?undefined:store*GiB)/GiB;
 expect(at("cache-host",926,175,27.75)).toBeGreaterThan(120);expect(at("cache-host",926,175,61.75)).toBeGreaterThan(120);expect(at("cache-host",926,175,61.75)*0.5).toBeGreaterThan(60);expect(at("cache-host",926,80,61.75)).toBeLessThan(at("cache-host",926,175,61.75));expect(at("cache-host",926,175)).toBe(60);expect(at("builder",256,100)).toBe(20);expect(at("worker",256,100)).toBe(7.5);
 expect(at("cache-host",926,33,41.6)).toBeLessThan(60);expect(at("cache-host",926,5,36.3)).toBeLessThanOrEqual(Math.ceil(36.3));
 expect(casEffectiveCapBytesV1(CAS_POLICY_V1,{totalBytes:926*GiB,freeBytes:175*GiB})).toBe(casEffectiveCapBytesV1(CAS_POLICY_V1,{totalBytes:926*GiB,freeBytes:175*GiB,role:"builder"}));
 expect(at("cache-host",926,175)*0.5).toBeGreaterThanOrEqual(CAS_POLICY_V1.sizing.serverBytes/GiB);
});

test("a machine's role is declared by the environment or by a live record the fleet cache server renews, the strongest live role wins, and a record expires",()=>{
 const root=scratch("role"),t0=1_000_000,policy=CAS_POLICY_V1;
 expect(readCasRoleV1(root,policy,{},t0)).toBe("builder");
 declareCasRoleV1(root,"worker",policy,t0);expect(readCasRoleV1(root,policy,{},t0+1)).toBe("worker");
 declareCasRoleV1(root,"cache-host",policy,t0);expect(readCasRoleV1(root,policy,{},t0+1)).toBe("cache-host");
 expect(readCasRoleV1(root,policy,{SEMIO_CAS_ROLE:"worker"},t0+1)).toBe("worker");expect(readCasRoleV1(root,policy,{SEMIO_CAS_ROLE:"nonsense"},t0+1)).toBe("cache-host");
 expect(readCasRoleV1(root,policy,{},t0+policy.sizing.roleTtlMs+1)).toBe("builder");
 expect(measureCasPartitionV1(root)?.role).toBeDefined();
});

test("a collection above the high watermark frees only one increment below it when it runs continuously, and down to the low watermark when run by hand; the independent Python oracle agrees",()=>{
 const grid=enforcement.increments,run=spawnSync(testPythonV1()??"python3",["-I",fileURLToPath(new URL("./🐍️enforcement.py",import.meta.url))],{input:JSON.stringify({increments:grid}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);const python=JSON.parse(run.stdout) as number[];
 grid.rows.forEach((row,index)=>{
  const planner=createCasPlannerV1({policy:{capBytes:grid.cap,highWatermark:grid.high,lowWatermark:grid.low,incrementBytes:grid.increment},incremental:row.incremental,areas:[{id:"store",capBytes:null,maxAgeMs:null}],externalBytes:0,nowMs:1e9});
  for(let entry=0;entry<row.total/10;entry++)planner.add("store",entry,10,false);
  expect(planner.finish().totals.needBytes,row.name).toBe(row.need);expect(python[index],row.name).toBe(row.need);
 });
});

test("the service tick collects continuously in small steps: a store a little above its high watermark loses one increment, and one below it is left alone",async()=>{
 resetCasGuardsV1();
 const root=scratch("continuous"),clock={value:2_000_000_000},{put}=fill(root,0,MIB,clock),policy:CasPolicyV1={...small,incrementBytes:2*MIB,minAgeMs:0},home=scratch("continuous-home");
 for(let index=0;index<19;index++)await put(index);
 await casEnforceTickV1({root,policy,home,partition:()=>undefined,sweep:false});
 expect((await casUsageV1(root)).bytes).toBe(16*MIB);
 resetCasGuardsV1();await casEnforceTickV1({root,policy,home,partition:()=>undefined,sweep:false});
 expect((await casUsageV1(root)).bytes).toBe(16*MIB);
 const manual=await collectCasV1({root,policy,now:()=>clock.value});expect(manual.status).toBe("idle");
});

test("input blobs are leased: protected while the lease runs, the first and always collected after it ended unless something read them later, and never pinned",async()=>{
 const root=scratch("transient"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<6;index++)hashes.push(await put(index));
 const big:CasPolicyV1={...CAS_POLICY_V1,minAgeMs:0,transientLeaseMs:60_000};
 await store.lease(hashes.slice(0,3),60_000);
 const leased=await readCasTransientsV1(root);expect([...leased.keys()].sort()).toEqual(hashes.slice(0,3).sort());
 const during=await collectCasV1({root,policy:big,now:()=>clock.value});expect(during.evicted).toBe(0);expect((await store.has(hashes)).every(Boolean)).toBe(true);
 const after=clock.value+120_000;
 const second=createCasStoreV1({root,now:()=>after,policy:{...CAS_POLICY_V1,touchGranularityMs:0},guard:quietGuard(root)});await second.touch(hashes[1]!);
 const done=await collectCasV1({root,policy:big,now:()=>after});
 expect(done.byReason.age).toBe(2);expect(done.evicted).toBe(2);
 expect(await store.has(hashes)).toEqual([false,true,false,true,true,true]);
 expect((await readCasPinsOf(root)).size).toBe(0);
});

const readCasPinsOf=async(root:string):Promise<Map<string,string>>=>(await import("../📌️pins/🟦️.ts")).readCasPinsV1(root,Date.now());

test("a store of a cache server refuses a write that stays over its cap after one bounded collection, lets it go on when the collection relieved it, and an ordinary store never refuses",async()=>{
 resetCasGuardsV1();
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:100*MIB,minAgeMs:0,admissionBudgetMs:250};let collects=0,mode:"relieve"|"keep"|"hang"="relieve",hung=false;let used=150*MIB;
 const root=scratch("refuse-cap"),over=()=>{mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:used,atMs:Date.now()}));};
 const ports=(refuseOverCap:boolean)=>({root,policy,refuseOverCap,partition:()=>({partition:"/v",totalBytes:100000*MIB,freeBytes:50000*MIB}),usage:async()=>used,
  collect:async()=>{collects++;if(mode==="hang"){await new Promise(done=>setTimeout(done,3000));hung=true;}if(mode==="relieve")used=60*MIB;over();return stub;}});
 over();
 const refusing=createCasGuardV1(ports(true));
 await refusing.admit(MIB);expect(collects).toBe(1);
 resetCasGuardsV1();used=150*MIB;over();mode="keep";collects=0;
 const keeping=createCasGuardV1(ports(true));const refused=await refusal(keeping.admit(MIB));
 expect(refused).toBeInstanceOf(CasCapRefusalV1);expect(String(refused.message)).toContain("Der Cache-Speicher");expect(collects).toBe(1);
 resetCasGuardsV1();used=150*MIB;over();mode="hang";collects=0;
 const hanging=createCasGuardV1(ports(true)),timed=await refusal(hanging.admit(MIB));
 expect(timed).toBeInstanceOf(CasCapRefusalV1);expect(hung,"refused at the admission budget, before the hanging collection ended").toBe(false);
 await hanging.settle();
 resetCasGuardsV1();used=150*MIB;over();mode="keep";
 const ordinary=createCasGuardV1(ports(false));await expect(ordinary.admit(MIB)).resolves.toBeDefined();
});

test("a transient input write is admitted over the cap while the partition has room, an ordinary write of the same store is refused, and a critical partition still refuses both",async()=>{
 resetCasGuardsV1();
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:100*MIB,minAgeMs:0,admissionBudgetMs:40};let collects=0;const used=150*MIB;
 const root=scratch("transient-admit");mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:used,atMs:Date.now()}));
 const room:CasPartitionV1={partition:"/v",totalBytes:100000*MIB,freeBytes:50000*MIB,role:"worker"} as CasPartitionV1,ports=(partition:CasPartitionV1)=>({root,policy,refuseOverCap:true,partition:()=>partition,usage:async()=>used,collect:async()=>{collects++;return stub;}});
 const guard=createCasGuardV1(ports(room));
 await expect(guard.admit(MIB,undefined,true)).resolves.toBeDefined();await guard.settle();
 expect(await refusal(guard.admit(MIB))).toBeInstanceOf(CasCapRefusalV1);
 resetCasGuardsV1();
 const tight=createCasGuardV1(ports(critical));
 expect(await refusal(tight.admit(MIB,undefined,true))).toBeDefined();
});

test("a released input lease ends after the grace unless another action still holds the blob, a renewal of the released action leases again, and requester leases (named or anonymous) last at most the requester maximum",async()=>{
 const root=scratch("lease-release"),policy=CAS_POLICY_V1,{leaseCasTransientV1,releaseCasTransientV1}=await import("../📌️pins/🟦️.ts");
 const [shared,alone,anonymous]=["a","b","c"].map(letter=>letter.repeat(64)),t0=1_000_000_000_000,lease=Math.min(policy.transientLeaseMs,policy.requesterLeaseMs),grace=policy.transientGraceMs;
 await leaseCasTransientV1(root,policy,"w1",[shared,alone],t0,undefined,"first");await leaseCasTransientV1(root,policy,"w2",[shared],t0+10,undefined,"second");await leaseCasTransientV1(root,policy,"w1",[anonymous],t0);
 expect(await readCasTransientsV1(root)).toEqual(new Map([[shared,t0+10+lease],[alone,t0+lease],[anonymous,t0+lease]]));
 await releaseCasTransientV1(root,policy,"w3",[shared,alone],t0+1000,"first");
 expect(await readCasTransientsV1(root)).toEqual(new Map([[shared,t0+10+lease],[alone,t0+1000+grace],[anonymous,t0+lease]]));
 await releaseCasTransientV1(root,policy,"w3",[shared],t0+2000,"second");
 expect((await readCasTransientsV1(root)).get(shared)).toBe(t0+2000+grace);
 await leaseCasTransientV1(root,policy,"w1",[alone],t0+3000,undefined,"first");
 expect((await readCasTransientsV1(root)).get(alone)).toBe(t0+3000+lease);
});

test("the content client names the action of an input lease and ends it with DELETE, and a 507 on an input upload is retried with a bounded pause",async()=>{
 const seen:string[]=[];let refusals=2;
 const content=httpRemoteExecutionContentV1({endpoint:"http://cache.test",retries:0,fetch:(async(url:string|URL|Request,init?:RequestInit)=>{seen.push(`${init?.method} ${new Headers(init?.headers).get(CAS_LEASE_HEADER_V1)??"-"}`);return init?.method==="PUT"&&refusals-->0?new Response("Insufficient storage",{status:507}):new Response(null,{status:200});}) as typeof fetch});
 const hash="d".repeat(64),{outageTolerantRemoteExecutionContentV1}=await import("../../🌐️remote-execution/🗄️content/🟦️.ts"),pauses:number[]=[];
 const patient=outageTolerantRemoteExecutionContentV1(content,{seconds:30,initialMilliseconds:250,maximumMilliseconds:2000},async ms=>{pauses.push(ms);});
 await patient.write("main",{hash,size:1},new Uint8Array([1]),undefined,"input:abc");await patient.release!("main",[hash],"input:abc");
 expect(seen).toEqual(["PUT input:abc","PUT input:abc","PUT input:abc","DELETE input:abc"]);expect(pauses).toEqual([250,500]);
});

test("the content client marks the blobs of an action as inputs and plain output uploads carry no mark",async()=>{
 const seen:string[]=[],content=httpRemoteExecutionContentV1({endpoint:"http://cache.test",retries:0,fetch:(async(url:string|URL|Request,init?:RequestInit)=>{seen.push(`${init?.method} ${new URL(String(url)).pathname} ${new Headers(init?.headers).get(CAS_LEASE_HEADER_V1)??"-"}`);return new Response(null,{status:200});}) as typeof fetch});
 const hash="a".repeat(64);
 await content.has("main",hash,undefined,"input");await content.has("main",hash);
 await content.write("main",{hash,size:1},new Uint8Array([1]),undefined,"input");await content.write("main",{hash,size:1},new Uint8Array([1]));
 expect(seen).toEqual([`HEAD /main/cas/${hash} input`,`HEAD /main/cas/${hash} -`,`PUT /main/cas/${hash} input`,`PUT /main/cas/${hash} -`]);
});

test("lease segments are read incrementally, rotate at their declared size, and compaction drops the ended leases of idle segments",async()=>{
 const root=scratch("segments"),{leaseCasTransientV1,compactCasTransientsV1}=await import("../📌️pins/🟦️.ts"),policy={...CAS_POLICY_V1,transientSegmentBytes:300},t0=Date.now();
 const digests=Array.from({length:12},(_,index)=>String(index).padStart(2,"0").repeat(32));
 for(const [index,digest] of digests.entries()){await leaseCasTransientV1(root,policy,"w1",[digest],t0+index,undefined,"a"+index);expect((await readCasTransientsV1(root)).size).toBe(index+1);}
 const directory=join(root,"v1","x");expect(readdirSync(directory).length).toBeGreaterThan(2);
 expect(await readCasTransientsV1(root)).toEqual(new Map(digests.map((digest,index)=>[digest,t0+index+Math.min(policy.transientLeaseMs,policy.requesterLeaseMs)])));
 expect(await compactCasTransientsV1(root,policy,t0+policy.transientLeaseMs*3,async()=>true)).toBe(12);
 expect(readdirSync(directory)).toEqual([]);expect((await readCasTransientsV1(root)).size).toBe(0);
});

test("the tick sweeps objects nobody references below every watermark in bounded steps and never a pinned one",async()=>{
 resetCasGuardsV1();
 const root=scratch("sweep"),clock={value:2_000_000_000},{put}=fill(root,0,MIB,clock),policy:CasPolicyV1={...small,capBytes:200*MIB,sweepStepBytes:3*MIB,sweepEveryMs:3_600_000},home=scratch("sweep-home"),hashes:string[]=[];
 for(let index=0;index<10;index++)hashes.push(await put(index));
 const {pinCasV1}=await import("../📌️pins/🟦️.ts");await pinCasV1(root,policy,"keeper",[hashes[0]!,hashes[9]!],3_600_000,Date.now());
 const used:number[]=[];
 for(let tick=0;tick<4;tick++){await casEnforceTickV1({root,policy,home,partition:()=>undefined});used.push((await casUsageV1(root)).bytes/MIB);}
 expect(used).toEqual([7,4,2,2]);
 for(const hash of [hashes[0]!,hashes[9]!])expect(existsSync(join(root,"v1","o",hash.slice(0,2),hash.slice(2)))).toBe(true);
});

test("a refused manual collection names the process that holds the lock and how long ago it progressed",async()=>{
 const root=scratch("locked");mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(join(root,"v1","gc.lock"),JSON.stringify({...ownProcessIdentityV1(),atMs:Date.now()-5000}));
 const result=await collectCasV1({root,policy:small});
 expect(result.status).toBe("locked");expect(result.lockedBy?.pid).toBe(process.pid);
 expect(formatCasCollectionV1(result,small,"en")).toContain(`process ${process.pid} holds the lock`);expect(formatCasCollectionV1(result,small,"de")).toContain(`Prozess ${process.pid} hält die Sperre`);
});

test("the hard free-space reserve per role is the larger of its partition share and its floor (72 GiB or more on the 926 GiB cache host), and the cap in force never lets the store grow into it",()=>{
 const GiB=2**30,reserve=(role:CasRoleV1,total:number)=>casReserveBytesV1(CAS_POLICY_V1,{totalBytes:total*GiB,role})/GiB;
 expect(reserve("cache-host",926)).toBeGreaterThanOrEqual(72);expect(reserve("cache-host",926)).toBeCloseTo(74.08,2);expect(reserve("cache-host",256)).toBe(72);expect(reserve("builder",256)).toBe(16);expect(reserve("worker",256)).toBeCloseTo(10.24,2);expect(reserve("worker",30)).toBe(8);expect(reserve("cache-host",100)).toBe(50);
 const at=(free:number,store:number)=>casEffectiveCapBytesV1(CAS_POLICY_V1,{totalBytes:926*GiB,freeBytes:free*GiB,role:"cache-host"},store*GiB)/GiB,step=0.25,caps:number[]=[];
 for(const free of [130,110,90,76,74,70,60,50,30]){const cap=at(free,90);caps.push(cap);expect(cap).toBeLessThanOrEqual(90+free-reserve("cache-host",926)+0.0001);if(free<=110)expect(cap).toBeGreaterThan(90+free-reserve("cache-host",926)-step-0.0001);}
 expect(caps).toEqual([...caps].sort((left,right)=>right-left));expect(at(74,90)).toBeLessThan(90);expect(at(76,90)).toBeGreaterThanOrEqual(90);
 const state=casReserveStateV1(CAS_POLICY_V1,{totalBytes:926*GiB,freeBytes:60*GiB,role:"cache-host"});expect(state.distanceBytes).toBeLessThan(0);expect(state.reserveBytes).toBe(casReserveBytesV1(CAS_POLICY_V1,{totalBytes:926*GiB,role:"cache-host"}));
});

test("a cap that includes the reserve frees candidates but evicts no held entry: the shortfall that may touch held entries is judged against the proportional cap",()=>{
 const run=(heldCapBytes?:number)=>{
  const planner=createCasPlannerV1({policy:{capBytes:20,highWatermark:0.9,lowWatermark:0.75,...heldCapBytes===undefined?{}:{heldCapBytes}},areas:[{id:"store",capBytes:null,maxAgeMs:null}],externalBytes:0,nowMs:1e9});
  for(let index=0;index<4;index++)planner.add("store",index,10,true);for(let index=0;index<4;index++)planner.add("store",index,10,false);
  return planner.finish().totals;
 };
 expect(run()).toMatchObject({plannedBytes:40,capShortfallBytes:25});expect(run(1000)).toMatchObject({plannedBytes:40,capShortfallBytes:0});
});

test("an input write is refused when it would grow into the reserve, even a transient one, and the refusal names the reserve; with room it is admitted",async()=>{
 resetCasGuardsV1();
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:100*MIB,minAgeMs:0,admissionBudgetMs:40},root=scratch("reserve-admit");mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:150*MIB,atMs:Date.now()}));
 const ports=(freeBytes:number)=>({root,policy,refuseOverCap:true,partition:()=>({partition:"/v",totalBytes:100000*MIB,freeBytes,role:"worker"} as CasPartitionV1),usage:async()=>150*MIB,collect:async()=>stub});
 await expect(createCasGuardV1(ports(50000*MIB)).admit(MIB,undefined,true)).resolves.toBeDefined();
 resetCasGuardsV1();
 const error=await refusal(createCasGuardV1(ports(6000*MIB)).admit(MIB,undefined,true));
 expect(error).toBeInstanceOf(CasCapRefusalV1);expect(error.message).toContain("free-space reserve");expect(error.message).toContain("Freiraum-Reserve");expect(error.reserve.role).toBe("worker");
});

test("a lease taken while the collector runs protects the object at once: the lease table is read again right before every removal", async()=>{
 const root=scratch("lateleash"),clock={value:2_000_000_000},{store,put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<24;index++)hashes.push(await put(index));
 let leased=false;
 const result=await collectCasV1({root,policy:small,now:()=>clock.value,beforeEvict:async path=>{if(!leased&&path===store.objectPath(hashes[2]!)){leased=true;await store.lease([hashes[2]!],undefined,"op-live");}}});
 expect(leased).toBe(true);expect((await store.has([hashes[2]!]))[0]).toBe(true);expect(result.skipped).toBeGreaterThanOrEqual(1);
});

test("the sweep never removes a blob leased by a live operation, whatever its age, and takes the blobs of operations that ended",async()=>{
 resetCasGuardsV1();
 const root=scratch("sweep-live"),clock={value:2_000_000_000},{put}=fill(root,0,MIB,clock),policy:CasPolicyV1={...small,capBytes:200*MIB,sweepStepBytes:64*MIB,sweepEveryMs:3_600_000},home=scratch("sweep-live-home"),hashes:string[]=[];
 for(let index=0;index<10;index++)hashes.push(await put(index));
 const {leaseCasTransientV1}=await import("../📌️pins/🟦️.ts");
 await leaseCasTransientV1(root,policy,"scheduler",[hashes[0]!,hashes[9]!],Date.now(),undefined,"op-live");
 await leaseCasTransientV1(root,policy,"scheduler",[hashes[1]!,hashes[2]!],Date.now()-7_200_000,1000,"op-ended");
 for(let tick=0;tick<3;tick++)await casEnforceTickV1({root,policy,home,partition:()=>undefined});
 const present=(hash:string):boolean=>existsSync(join(root,"v1","o",hash.slice(0,2),hash.slice(2)));
 expect([hashes[0]!,hashes[9]!].map(present),"leased by a live operation").toEqual([true,true]);
 expect(hashes.slice(1,9).some(present),"everything else nobody references or leases is swept").toBe(false);
});

test("a requester's lease lasts at most the requester maximum and ends with the operation lease of its action once the scheduler holds that operation, whatever lease the client asked for",async()=>{
 const root=scratch("lease-bound"),policy=CAS_POLICY_V1,{leaseCasTransientV1,releaseCasTransientV1}=await import("../📌️pins/🟦️.ts");
 const [blob,other]=["d","e"].map(letter=>letter.repeat(64)),t0=1_000_000_000_000;
 await leaseCasTransientV1(root,policy,"client",[blob!,other!],t0,policy.transientLeaseMs,"act");
 expect(await readCasTransientsV1(root)).toEqual(new Map([[blob!,t0+policy.requesterLeaseMs],[other!,t0+policy.requesterLeaseMs]]));
 await leaseCasTransientV1(root,policy,"scheduler",[blob!],t0+1000,undefined,"op-act");
 expect((await readCasTransientsV1(root)).get(blob!),"the operation holds it for its lifetime").toBe(t0+1000+policy.transientLeaseMs);
 await releaseCasTransientV1(root,policy,"scheduler",[blob!],t0+5000,"op-act");
 expect((await readCasTransientsV1(root)).get(blob!),"operation ended: only the grace is left, the requester's ten minutes are gone").toBe(t0+5000+policy.transientGraceMs);
 expect((await readCasTransientsV1(root)).get(other!)).toBe(t0+policy.requesterLeaseMs);
});

test("the unreferenced sweep runs while another collector holds the lock, frees only what nobody references and leaves the census; a full collection still waits for the lock",async()=>{
 resetCasGuardsV1();
 const root=scratch("sweep-locked"),clock={value:2_000_000_000},{put}=fill(root,0,MIB,clock),hashes:string[]=[];
 for(let index=0;index<6;index++)hashes.push(await put(index));
 const {pinCasV1}=await import("../📌️pins/🟦️.ts");await pinCasV1(root,small,"keeper",[hashes[0]!],3_600_000,Date.now());
 const lock=join(root,"v1","gc.lock");writeFileSync(lock,JSON.stringify({...ownProcessIdentityV1(),atMs:Date.now()}));clock.value+=60_000;
 expect((await collectCasV1({root,policy:small,now:()=>clock.value})).status).toBe("locked");
 const swept=await collectCasV1({root,policy:small,unreferencedOnly:true,now:()=>clock.value});
 expect(swept).toMatchObject({status:"collected",lockless:true,evicted:5});expect(existsSync(lock)).toBe(true);
 expect(existsSync(join(root,"v1","o",hashes[0]!.slice(0,2),hashes[0]!.slice(2)))).toBe(true);
 const census=await readCasCensusV1(root);
 expect(census).toMatchObject({objects:1,bytes:MIB});expect(census!.classes.pinned).toEqual({objects:1,bytes:MIB});expect(census!.classes.unreferenced).toEqual({objects:0,bytes:0});
 expect(census!.holders[0]).toMatchObject({kind:"pin",holder:"keeper",objects:1,bytes:MIB});
});

test("below the hard reserve a transient input is admitted only after the sweep freed enough unreferenced data to keep the reserve; with only pinned data it is refused and nothing held is touched",async()=>{
 const stored=(root:string):number=>{let bytes=0;const base=join(root,"v1","o");for(const shard of existsSync(base)?readdirSync(base):[])for(const name of readdirSync(join(base,shard)))bytes+=statSync(join(base,shard,name)).size;return bytes;};
 const policy:CasPolicyV1={...small,capBytes:100*MIB},present=(root:string,hash:string):boolean=>existsSync(join(root,"v1","o",hash.slice(0,2),hash.slice(2)));
 const scenario=async(name:string,pinned:number,freeAt:(root:string)=>number)=>{
  resetCasGuardsV1();
  const root=scratch(name),clock={value:2_000_000_000},{put}=fill(root,0,MIB,clock),hashes:string[]=[];
  for(let index=0;index<8;index++)hashes.push(await put(index));
  const {pinCasV1}=await import("../📌️pins/🟦️.ts");await pinCasV1(root,policy,"keeper",hashes.slice(0,pinned),3_600_000,Date.now());
  const guard=createCasGuardV1({root,policy,refuseOverCap:true,partition:path=>({partition:"/v",totalBytes:100000*MIB,freeBytes:freeAt(path),role:"worker"}),usage:async()=>stored(root)});
  return{root,hashes,admitted:guard.admit(MIB,undefined,true)};
 };
 const reserve=casReserveBytesV1(policy,{totalBytes:100000*MIB,role:"worker"});
 const roomy=await scenario("reserve-sweep",2,root=>reserve-2*MIB+(8*MIB-stored(root)));
 await expect(roomy.admitted).resolves.toBeDefined();
 expect(roomy.hashes.map(hash=>present(roomy.root,hash))).toEqual([true,true,false,false,false,false,false,false]);
 const full=await scenario("reserve-held",8,()=>reserve-2*MIB);
 const error=await refusal(full.admitted);
 expect(error).toBeInstanceOf(CasCapRefusalV1);expect(error.reserve.role).toBe("worker");
 expect(full.hashes.every(hash=>present(full.root,hash))).toBe(true);
});

test("the status answers from the census without walking the store, in both languages",async()=>{
 const root=scratch("status-census"),home=scratch("status-home"),clock={value:2_000_000_000},{put}=fill(root,0,MIB,clock);
 for(let index=0;index<3;index++)await put(index);
 await collectCasV1({root,policy:small,unreferencedOnly:true,unreferencedStepBytes:MIB,now:()=>clock.value});
 const {runCasCommandV1}=await import("../⌨️command/🟦️.ts"),lines:string[]=[],ports={signal:new AbortController().signal,progress:async()=>{}};
 const run=async(language:"en"|"de")=>{lines.length=0;await runCasCommandV1(["status",`--lang=${language}`],{workspace:home,environment:{SEMIO_CAS_ROOT:root,HOME:home},write:line=>{lines.push(line);},home,partition:()=>undefined},ports);return lines.join("\n");};
 const english=await run("en");expect(english).toContain("2 objects");expect(english).toContain("Census of the last complete scan");expect(english).toContain("unreferenced 0 GiB");
 expect(await run("de")).toContain("Zählung des letzten vollständigen Durchgangs");
});

test("a removed object leaves the census under the class and size the scan counted it with; one the scan never counted, or one removed twice, changes nothing, so no class goes below zero",async()=>{
 const {forgetCensusV1}=await import("../🧹️collector/🟦️.ts");
 const zero=()=>({objects:0,bytes:0}),census={version:1 as const,atMs:0,objects:2,bytes:3*MIB,classes:{pinned:zero(),held:zero(),leased:zero(),linked:zero(),expired:zero(),young:{objects:1,bytes:MIB},unreferenced:{objects:1,bytes:2*MIB}},holders:[]};
 const counted=new Map([["a".repeat(64),{hash:"a".repeat(64),size:MIB,kind:"young" as const}],["b".repeat(64),{hash:"b".repeat(64),size:2*MIB,kind:"unreferenced" as const}]]);
 forgetCensusV1(census,counted,"a".repeat(64));
 expect(census.classes.young,"counted young, removed after it turned unreferenced: leaves young").toEqual({objects:0,bytes:0});
 expect(census.classes.unreferenced).toEqual({objects:1,bytes:2*MIB});
 forgetCensusV1(census,counted,"a".repeat(64));forgetCensusV1(census,counted,"c".repeat(64));
 expect(census.classes.young,"removed twice or stored after the scan: nothing changes").toEqual({objects:0,bytes:0});
 expect(census).toMatchObject({objects:1,bytes:2*MIB});
 expect(Object.values(census.classes).every(row=>row.objects>=0&&row.bytes>=0)).toBe(true);
});

/** 🔓️ Contract: the Bun content cache is adopted only by copy-on-write clones, never by shared read-only hard links, because Bun copies cached packages with their mode and patches the copies in place (a 0444 copy fails every patch with EACCES on Linux). */
test("the bun cache place of the adoption policy is private and writable",async()=>{
 const {CAS_ADOPTION_V1}=await import("../🧬️schema/🟦️.ts"),bun=CAS_ADOPTION_V1.places.find(place=>place.id==="bun-cache")!;
 expect(bun.mode).toBe("reflink");
 const {adoptCasV1}=await import("../🧲️adopt/🟦️.ts"),root=mkdtempSync(join(tmpdir(),"adopt-bun-")),workspace=join(root,"ws"),cache=join(workspace,"bun-cache");
 try{
  mkdirSync(cache,{recursive:true});for(const name of["a","b"])writeFileSync(join(cache,name),"x".repeat(5000));
  const old=new Date(Date.now()-3_600_000);for(const name of["a","b"])utimesSync(join(cache,name),old,old);
  await adoptCasV1({root:join(root,"store"),places:[{id:"bun-cache",directory:cache,mode:bun.mode}],policy:{...CAS_ADOPTION_V1,minBytes:1000,quietMs:60_000}});
  for(const name of["a","b"]){const facts=statSync(join(cache,name));expect(facts.nlink).toBe(1);if(process.platform!=="win32")expect(facts.mode&0o200).toBe(0o200);}
 }finally{rmSync(root,{recursive:true,force:true});}
});
