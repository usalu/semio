import {registeredToolExecutableV1} from "../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
/** 🧪️ Tests for the host-wide content-addressed store, its access ledger, pins and least-recently-accessed collector. */
import {spawn,spawnSync} from "node:child_process";
import {appendFileSync,chmodSync,existsSync,realpathSync,symlinkSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
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
import {casPartitionNeedBytesV1,decideCasEnforcementV1,type CasPartitionV1} from "../🌡️partition/🟦️.ts";
import {casUsageHintPathV1,readCasUsageHintV1,type CasCollectionV1} from "../🧹️collector/🟦️.ts";
import {putCasObjectSyncV1} from "../🗃️store/🔁️sync/🟦️.ts";
import {casLedgerShardV1,foldCasShardV1,openCasShardViewV1} from "../📒️ledger/🟦️.ts";
import {isRepoErrorV1,renderRepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {casRootV1,createCasStoreV1} from "../🗃️store/🟦️.ts";
import {casUsageV1} from "../🗃️store/📏️usage/🟦️.ts";
import {CAS_MESSAGES_V1,CAS_POLICY_V1,casTextV1,requireCasPolicyV1,type CasPolicyV1} from "../🧬️schema/🟦️.ts";

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
 expect(CAS_POLICY_V1.capBytes).toBe(32*1024**3);expect(CAS_POLICY_V1.areas.map(area=>area.id)).toEqual(["graph-memo","build-input-closures"]);
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

test("loose areas count against the host cap: closures and memo files age out and are capped, recent files and other data stay",async()=>{
 const root=scratch("areas"),memo=scratch("memo"),closures=scratch("closures"),clock={value:2_000_000_000_000},age=(path:string,ms:number)=>{const at=new Date(clock.value-ms);utimesSync(path,at,at);};
 mkdirSync(join(closures,"a"),{recursive:true});
 const day=86_400_000,make=(directory:string,name:string,size:number,old:number)=>{const path=join(directory,name);writeFileSync(path,Buffer.alloc(size,1));age(path,old);return path;};
 const oldMemo=make(memo,"x.plan",MIB,9*day),freshMemo=make(memo,"y.plan",MIB,day),records=[1,2,3,4].map(index=>make(memo,`r${index}.plan`,MIB,(10-index)*1000+60_000)),partial=make(memo,".z.tmp",MIB,9*day);
 const oldClosure=make(join(closures,"a"),"c.json.br",MIB,8*day),freshClosure=make(join(closures,"a"),"d.json.br",MIB,2*day);
 const bystander=join(root,"other.txt");mkdirSync(root,{recursive:true});writeFileSync(bystander,"user data");
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:100*MIB,minAgeMs:30_000,areas:[{id:"graph-memo",capBytes:3*MIB,maxAgeMs:7*day},{id:"build-input-closures",capBytes:null,maxAgeMs:7*day}]};
 const result=await collectCasV1({root,policy,now:()=>clock.value,areas:[{id:"graph-memo",directory:memo},{id:"build-input-closures",directory:closures}]});
 expect(result.byReason.age).toBe(2);expect(() => statSync(oldMemo)).toThrow();expect(() => statSync(oldClosure)).toThrow();
 expect(statSync(freshClosure).isFile()).toBe(true);expect(statSync(partial).isFile()).toBe(true);expect(statSync(bystander).isFile()).toBe(true);
 const remaining=[freshMemo,...records].filter(path=>{try{statSync(path);return true;}catch{return false;}});
 expect(remaining).toEqual([records[2]!,records[3]!]);expect(result.byReason.area).toBe(3);
 const host=await collectCasV1({root,policy:{...policy,capBytes:4*MIB},now:()=>clock.value,areas:[{id:"graph-memo",directory:memo},{id:"build-input-closures",directory:closures}],externalBytes:2*MIB});
 expect(host.byReason.host).toBeGreaterThan(0);expect(statSync(bystander).isFile()).toBe(true);
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
 expect(await runCasCommandV1(["status","--lang=de"],context,ports)).toBe(0);expect(lines.at(-1)).toContain("Objekte");
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
 expect(result.partition).toEqual({partition:"/volume",usedFraction:0.95,needBytes:15*MIB});
 const left=await store.has(hashes);expect(left.slice(0,15).some(Boolean)).toBe(false);expect(left.slice(15).every(Boolean)).toBe(true);
 expect(formatCasCollectionV1(result,policy,"en")).toContain("Partition /volume was 95% used (high watermark 90%, low watermark 80%)");
 expect(formatCasCollectionV1(result,policy,"de")).toContain("Partition /volume war zu 95 % belegt (obere Marke 90 %, untere Marke 80 %)");
 const idle=await collectCasV1({root,policy,partition:{...partition,freeBytes:30*MIB},now:()=>clock.value});expect(idle.status).toBe("idle");
 const stuck=await collectCasV1({root,policy:{...policy,minAgeMs:3_600_000_000},partition,now:()=>clock.value});
 expect(stuck.evicted).toBe(0);expect(formatCasCollectionV1(stuck,policy,"en")).toContain("The partition stays");
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
const stub:CasCollectionV1={status:"collected",partial:false,dryRun:false,scanned:0,evicted:0,freedBytes:0,busy:0,skipped:0,totals:{totalBytes:0,protectedBytes:0,candidateBytes:0,plannedBytes:0,keptBytes:0,shortfallBytes:0,needBytes:0},byReason:{age:0,area:0,host:0}};
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
