import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {readFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {CAS_POLICY_V1} from "../../../🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1} from "../../../🗄️cas/🗃️store/🟦️.ts";
import {pinCasV1} from "../../../🗄️cas/📌️pins/🟦️.ts";
import {reclaimCasObjectsV1} from "../../../🗄️cas/🪦️reclaim/🟦️.ts";
import {slotIdV1} from "../../../🪢️slots/🟦️.ts";
import {acquireRemoteCacheLockV1,collectRemoteCacheDirectoryV1,planRemoteCacheCollectionV1,remoteCacheServerRunningV1,renewRemoteCacheLockV1,REMOTE_CACHE_SERVER_LOCK_V1,scanRemoteCacheDirectoryV1,type RemoteCacheStepV1} from "../../🧹️collector/🟦️.ts";
import {createRemoteCacheHandlerV1} from "../../🖥️server/🟦️.ts";
import {RemoteCacheStoreV1} from "../../💾️store/🟦️.ts";
import {auditRemoteCacheDirectoryV1,readRemoteCacheMetaV1,scanRemoteCacheMetaV1} from "../🟦️.ts";
import {declareV1,openTreeDigestsV1} from "../🍂️staleness/🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dir,"../../../🪢️slots/🧪️tests/🐍️oracle/🐍️.py"),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-stale-${label}-`));scratch.push(path);return path;};
const body=(bytes:Uint8Array)=>(async function*(){yield bytes;})();
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const slot=(name:string)=>slotIdV1("rust-unit",[name,"build","",""]);
const fill=(key:string,size=64)=>new Uint8Array(size).fill((key.charCodeAt(0)^key.charCodeAt(key.length-1))&255);
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

const write=(root:string,files:Readonly<Record<string,string>>)=>{for(const [path,text] of Object.entries(files)){const full=join(root,path);mkdirSync(dirname(full),{recursive:true});writeFileSync(full,text);}};
type Mode="off"|"dry-run"|"enforce";
const policy=async(workspace:string,mode:Mode,recheckMilliseconds=0)=>({mode,tree:await openTreeDigestsV1(workspace),examples:5,recheckMilliseconds,manifestNames:["unit.json"]});
const open=async(folder:string,workspace:string,mode:Mode,extra:Partial<Parameters<typeof RemoteCacheStoreV1.open>[0]>={})=>RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,graceMilliseconds:150,staleness:await policy(workspace,mode),...extra});
type Built=Readonly<{source?:readonly [string,string];externs?:readonly string[];produces?:string;toolchain?:string;bytes?:Uint8Array}>;
const declaration=(built:Built)=>declareV1({kind:"rust-unit",toolchain:sha(built.toolchain??"t1"),sources:built.source===undefined?[]:[{path:built.source[0],sha256:sha(built.source[1])}],externs:(built.externs??[]).map(sha)});
const nxBody=(key:string,built:Built)=>new TextEncoder().encode(JSON.stringify({hash:sha(built.produces??"blob:"+key)}));
const put=(store:RemoteCacheStoreV1,key:string,built:Built|null,name="u")=>{const bytes=nxBody(key,built??{});return store.put("main","nx",key,body(bytes),{mode:"keep",declaredLength:bytes.length,...built===null?{}:{slot:{slot:slot(name),declaration:declaration(built)}}});};
const live=async(store:RemoteCacheStoreV1,key:string)=>(await store.read("main","nx",key))!==undefined;
const generationA={"core/lib.rs":"core v1","other/lib.rs":"other v1","mid/lib.rs":"mid v1"};
async function publishGenerationA(store:RemoteCacheStoreV1):Promise<void>{
 await put(store,"core",{source:["core/lib.rs","core v1"],externs:["blob:serde","blob:old"],produces:"blob:core"});
 await put(store,"mid",{source:["mid/lib.rs","mid v1"],externs:["blob:core"],produces:"blob:mid"});
 await put(store,"other",{source:["other/lib.rs","other v1"],externs:["blob:serde"],produces:"blob:other"});
 await put(store,"serde",{produces:"blob:serde"});await put(store,"old",{produces:"blob:old"});await put(store,"plain",null);
}

test.skipIf(!python)("the retention plan equals an independent Python model on 300 seeded stores with verdicts, sidecars, claims, ties and reservations",()=>{
 const next=seeded(20261122),states=["live","grace","stale","unreached","undeclared"] as const,requests:{op:string;rows:unknown[];bounds:Record<string,number>;dangling:string[];retention:{grace:number;metas:Record<string,{slot:string;storedAt:number}>;claimed:string[];verdicts:Record<string,string>|null}}[]=[],inputs:{rows:{key:string;size:number;accessed:number}[];metas:Map<string,{slot:string;storedAt:number}>;claimed:Set<string>;verdicts:Map<string,{state:typeof states[number]}>|undefined}[]=[];
 for(let round=0;round<300;round++){
  const count=Math.floor(next()*40),rows:{key:string;size:number;accessed:number}[]=[],metas=new Map<string,{slot:string;storedAt:number}>(),claimed=new Set<string>(),verdicts=new Map<string,{state:typeof states[number]}>();
  for(let index=0;index<count;index++){
   const key=`main/nx/${String(index).padStart(2,"0")}/k${Math.floor(next()*1000)}-${index}`;rows.push({key,size:1+Math.floor(next()*5000),accessed:1_000_000+Math.floor(next()*(round%3===0?5:2000))});
   if(next()<0.75)metas.set(key,{slot:`s${Math.floor(next()*3)}`.padEnd(64,"0"),storedAt:999_000+Math.floor(next()*(round%4===0?3:1500))});
   if(next()<0.08)claimed.add(key);
   if(next()<0.8)verdicts.set(key,{state:states[Math.floor(next()*states.length)]!});
  }
  const total=rows.reduce((sum,row)=>sum+row.size,0),bounds={maximumBytes:Math.max(1,Math.floor(total*(0.3+next()*1.2))),maximumAge:round%4===0?Math.floor(next()*1500):0,now:1_002_000,need:round%5===0?Math.floor(next()*1000):0,reserved:round%7===0?Math.floor(next()*500):0};
  const dangling=rows.filter(()=>next()<0.05).map(row=>row.key),enforcing=round%5!==0;
  requests.push({op:"plan",rows,bounds,dangling,retention:{grace:round%6===0?0:Math.floor(next()*600),metas:Object.fromEntries(metas),claimed:[...claimed],verdicts:enforcing?Object.fromEntries([...verdicts].map(([key,verdict])=>[key,verdict.state])):null}});
  inputs.push({rows,metas,claimed,verdicts:enforcing?verdicts:undefined});
 }
 const result=Bun.spawnSync([python!,"-I",oracle],{stdin:new TextEncoder().encode(JSON.stringify(requests)),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const model=JSON.parse(result.stdout.toString()) as {key:string;size:number;reason:string;storedAt?:number}[][];const reasons=new Set<string>();
 requests.forEach((request,index)=>{
  const input=inputs[index]!,ours=planRemoteCacheCollectionV1(input.rows,{maximumBytes:request.bounds.maximumBytes!,maximumAgeMilliseconds:request.bounds.maximumAge!,now:request.bounds.now!,need:request.bounds.need!,reserved:request.bounds.reserved!},new Set(request.dangling),{metas:input.metas,graceMilliseconds:request.retention.grace,claimed:key=>input.claimed.has(key),...input.verdicts===undefined?{}:{verdicts:input.verdicts}});
  expect(ours,`round ${index}`).toEqual(model[index]!);for(const drop of ours)reasons.add(drop.reason);
  for(const drop of ours)expect(input.claimed.has(drop.key)&&drop.reason!=="dangling","a claimed entry is never taken").toBe(false);
 });
 expect([...reasons].sort()).toEqual(["age","budget","dangling","stale"]);
});

test("generation A built, one crate edited: that unit and its dependents are retired after the grace, the others and the entry that declared nothing stay, blobs and sidecars follow",async()=>{
 const tree=directory("tree"),folder=directory("edit"),store=await open(folder,tree,"enforce");write(tree,generationA);await publishGenerationA(store);
 await sleep(200);await store.collect();for(const key of["core","mid","other","serde","old","plain"])expect(await live(store,key),`${key} while the tree is generation A`).toBe(true);
 expect(store.stalenessStatus()).toMatchObject({mode:"enforce",declared:5,live:5,retired:0,stale:{},unreached:0});
 write(tree,{"core/lib.rs":"core v2"});await sleep(20);await store.collect();
 expect(await live(store,"core"),"edited").toBe(false);expect(await live(store,"mid"),"consumes the edited unit").toBe(false);
 expect(await live(store,"other"),"independent").toBe(true);expect(await live(store,"serde"),"still consumed by a live unit").toBe(true);expect(await live(store,"plain")).toBe(true);
 expect(await readRemoteCacheMetaV1(store.root,"main/nx/co/core"),"its sidecar went with it").toBeUndefined();
 const report=store.stalenessStatus()!;expect(report).toMatchObject({mode:"enforce",retired:3,stale:{edited:1,extern:1},unreached:1});expect(report.examples.length).toBeGreaterThan(0);
 expect(await live(store,"old"),"a registry unit nothing live consumes any more goes in the same sweep").toBe(false);
 expect(readFileSync(join(store.root,".retention","sweeps.jsonl"),"utf8").trim().split("\n").length,"every sweep is logged").toBeGreaterThan(2);
 expect(JSON.parse(readFileSync(join(store.root,".retention","retired.json"),"utf8")).length,"the blobs of retired entries are remembered").toBeGreaterThan(1);
 await store.close();
});

test("a deleted crate and its dependents are retired; a unit stored within the grace is never taken; a registry unit unreached is retired only after the grace",async()=>{
 const tree=directory("tree-del"),folder=directory("del"),store=await open(folder,tree,"enforce",{graceMilliseconds:300});write(tree,generationA);await publishGenerationA(store);
 rmSync(join(tree,"core","lib.rs"));await store.collect();
 for(const key of["core","mid","old"])expect(await live(store,key),`${key} is stale or unreached but stored within the grace`).toBe(true);
 await sleep(350);await store.collect();
 expect(await live(store,"core"),"deleted").toBe(false);expect(await live(store,"mid")).toBe(false);expect(await live(store,"old"),"unreached beyond the grace").toBe(false);
 expect(await live(store,"other")).toBe(true);expect(await live(store,"serde")).toBe(true);
 expect(store.stalenessStatus()).toMatchObject({stale:{deleted:1,extern:1}});
 await store.close();
});

test("dry-run only records what it would retire: counts, the first examples and a jsonl line, nothing is removed; off judges nothing",async()=>{
 const tree=directory("tree-dry"),folder=directory("dry"),store=await open(folder,tree,"dry-run");write(tree,{...generationA,"core/lib.rs":"core v2"});await publishGenerationA(store);
 await sleep(200);await store.collect();
 for(const key of["core","mid","other","serde","old","plain"])expect(await live(store,key),key).toBe(true);
 const report=store.stalenessStatus()!;expect(report).toMatchObject({mode:"dry-run",retired:0,stale:{edited:1,extern:1}});expect(report.examples[0]).toContain("core");
 expect(readFileSync(join(store.root,".retention","sweeps.jsonl"),"utf8")).toContain('"mode":"dry-run"');
 await store.close();
 const off=await open(directory("off"),tree,"off");await publishGenerationA(off);await sleep(200);await off.collect();
 for(const key of["core","mid","old"])expect(await live(off,key),key).toBe(true);expect(off.stalenessStatus()).toBeUndefined();expect(existsSync(join(off.root,".retention"))).toBe(false);await off.close();
});

test("a claim keeps what an entry declared; an active claim protects a stale entry; entries that declared nothing age out while live ones never do",async()=>{
 const tree=directory("tree-claim"),folder=directory("claim"),held=new Set<string>(["main/nx/co/core"]),store=await open(folder,tree,"enforce",{maximumAgeMilliseconds:100,graceMilliseconds:100,claimed:(namespace,kind,key)=>held.has(`${namespace}/${kind}/${key.slice(0,2)}/${key}`)});
 write(tree,{...generationA,"core/lib.rs":"core v2"});await publishGenerationA(store);
 expect(await store.claim("main","nx","core",slot("u"))).toBe("claimed");expect((await readRemoteCacheMetaV1(store.root,"main/nx/co/core"))?.declaration?.sources).toEqual([{path:"core/lib.rs",sha256:sha("core v1")}]);
 await sleep(200);await store.collect();
 expect(await live(store,"core"),"claimed").toBe(true);expect(await live(store,"plain"),"declared nothing and idle beyond the age").toBe(false);expect(await live(store,"other"),"live entries are the synchronized state").toBe(true);
 held.clear();await store.collect();expect(await live(store,"core"),"claim released").toBe(false);
 await store.close();
 const tiny=await open(directory("budget"),tree,"enforce",{graceMilliseconds:60_000,maximumBytes:1000,maximumEntryBytes:400});
 for(const key of["b1","b2"])await tiny.put("main","nx",key,body(new Uint8Array(400)),{mode:"keep",declaredLength:400,slot:{slot:slot(key)}});
 expect(await tiny.put("main","nx","b3",body(new Uint8Array(400)),{mode:"keep",declaredLength:400,slot:{slot:slot("b3")}}),"stored within the grace, nothing may be taken for it: the write is refused").toBe("capacity");
 await tiny.close();
});

test("the budget takes stale and unreached entries before undeclared before live ones, least recently used first",async()=>{
 const tree=directory("tree-order"),store=await open(directory("order"),tree,"enforce",{graceMilliseconds:1,maximumBytes:3000,maximumEntryBytes:1000});write(tree,{"a/lib.rs":"a v1"});
 const size=new Uint8Array(500),withSize=(key:string,built:Built|null)=>store.put("main","nx",key,body(size),{mode:"keep",declaredLength:500,slot:{slot:slot(key),...built===null?{}:{declaration:declaration(built)}}});
 await withSize("live1",{source:["a/lib.rs","a v1"]});await withSize("plain1",null);await withSize("reg1",{});await sleep(5);await withSize("live2",{source:["a/lib.rs","a v1"]});await withSize("plain2",null);
 await store.collect(1);
 const present=await Promise.all(["live1","plain1","reg1","live2","plain2"].map(key=>live(store,key)));
 expect(present[2],"the unreached registry unit goes first").toBe(false);expect(present[0]&&present[3],"live entries stay").toBe(true);
 await store.close();
});

test("blobs go when no live entry references them; shared, pinned and recently used blobs stay",async()=>{
 const tree=directory("tree-blobs"),folder=directory("blobs"),cas=directory("cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}}),store=await open(folder,tree,"enforce",{host,graceMilliseconds:100});
 write(tree,{"n/lib.rs":"n v2","s/lib.rs":"s v1","p/lib.rs":"p v1"});
 const a=new TextEncoder().encode("blob-a".repeat(20)),shared=new TextEncoder().encode("blob-shared".repeat(20)),pinned=new TextEncoder().encode("blob-pinned".repeat(20)),hash=(bytes:Uint8Array)=>sha(bytes);
 const withBlob=(key:string,bytes:Uint8Array,built:Built)=>store.put("main","nx",key,body(bytes),{mode:"keep",declaredLength:bytes.length,slot:{slot:slot(key),declaration:declaration(built)}});
 await withBlob("n1",a,{source:["n/lib.rs","n v1"]});await withBlob("s1",shared,{source:["s/lib.rs","s v1"]});await withBlob("s2",shared,{source:["n/lib.rs","n v1"]});await withBlob("p1",pinned,{source:["p/lib.rs","p v0"]});
 await host.pin("someone-else",[hash(pinned)],60_000);
 await sleep(220);await store.collect();
 expect(await live(store,"n1")).toBe(false);expect(existsSync(host.objectPath(hash(a))),"unreferenced after n1 went").toBe(false);
 expect(await live(store,"s2"),"stale too").toBe(false);expect(await live(store,"s1")).toBe(true);expect(existsSync(host.objectPath(hash(shared))),"shared by the live s1").toBe(true);
 expect(existsSync(host.objectPath(hash(pinned))),"a live pin of another holder protects it").toBe(true);
 await store.close();
});

test("reclaim applies the pin, hard-link and minimum-age protections of the host store itself",async()=>{
 const cas=directory("reclaim"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:60_000}});
 const digest=await host.put(new TextEncoder().encode("young"));
 const young=await reclaimCasObjectsV1(host,[digest.hash]);expect(young).toMatchObject({entries:0,recent:1});expect(existsSync(host.objectPath(digest.hash))).toBe(true);
 const old=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}}),other=await old.put(new TextEncoder().encode("old"));
 await pinCasV1(cas,CAS_POLICY_V1,"holder",[other.hash],60_000,Date.now());
 expect((await reclaimCasObjectsV1(old,[other.hash])).pinned).toBe(1);expect(existsSync(old.objectPath(other.hash))).toBe(true);
 await old.release("holder");
 expect(await reclaimCasObjectsV1(old,[other.hash],{dryRun:true})).toMatchObject({entries:1,bytes:3});expect(existsSync(old.objectPath(other.hash)),"a dry run removes nothing").toBe(true);
 expect(await reclaimCasObjectsV1(old,[other.hash,other.hash,"not-a-digest"])).toMatchObject({entries:1,bytes:3});expect(existsSync(old.objectPath(other.hash))).toBe(false);
 expect((await reclaimCasObjectsV1(old,[other.hash])).missing).toBe(1);
});

async function seedStore(folder:string,tree:string,grace:number):Promise<string[]>{
 const store=await open(folder,tree,"enforce",{graceMilliseconds:grace}),keys:string[]=[];
 for(const name of["a","b","c"]){write(tree,{[`${name}/lib.rs`]:`${name} v3`});for(let index=0;index<4;index++){const key=`${name}key${index}`;await put(store,key,{source:[`${name}/lib.rs`,`${name} v${index}`],produces:"blob:"+key},name);keys.push(key);}}
 await put(store,"plain",null);keys.push("plain");
 await store.close();return keys;
}
const names=(folder:string)=>[...new Set((readdirSync(join(folder,"v1","main","nx"),{recursive:true}) as string[]).filter(name=>name.includes("/")).map(name=>name.split("/").pop()!))].sort();
const external=(folder:string,tree:TreeLike,extra:Record<string,unknown>={})=>({root:join(folder,"v1"),maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:false,graceMilliseconds:40,staleness:{mode:"enforce" as const,tree},...extra});
type TreeLike=Awaited<ReturnType<typeof openTreeDigestsV1>>;

test("a crash at any point of a collection leaves a readable cache that the next collection completes to the same result",async()=>{
 const tree=directory("tree-kill"),template=directory("kill"),keys=await seedStore(template,tree,40);await sleep(80);const digests=await openTreeDigestsV1(tree);
 const reference=directory("kill-ref");cpSync(template,reference,{recursive:true});
 const first=await collectRemoteCacheDirectoryV1(external(reference,digests));
 expect(first.locked).toBe(false);const expected=names(reference);expect(expected).toEqual(["akey3","bkey3","ckey3","plain"]);
 let points=0;
 for(let kill=1;kill<=40;kill++){
  const folder=directory("kill-"+kill);cpSync(template,folder,{recursive:true});let count=0,crashed=false;
  try{await collectRemoteCacheDirectoryV1(external(folder,digests,{onStep:(_point:RemoteCacheStepV1)=>{if(++count===kill)throw Error("simulated crash");}}));}catch(error){crashed=true;expect((error as Error).message).toBe("simulated crash");}
  if(!crashed){points=kill-1;break;}
  const store=await open(folder,tree,"off",{graceMilliseconds:40});
  for(const entry of names(folder)){const read=await store.read("main","nx",entry);expect(read,`kill ${kill}: ${entry} readable`).toBeDefined();expect(JSON.parse(await readFile(read!.path,"utf8")).hash,`kill ${kill}: ${entry} intact`).toBe(sha("blob:"+entry));}
  const present=new Map([...(await scanRemoteCacheDirectoryV1(join(folder,"v1"))).keys()].map(key=>[key,true]));
  for(const key of(await scanRemoteCacheMetaV1(join(folder,"v1"),present)).keys())expect(present.has(key),`kill ${kill}: no sidecar without its entry`).toBe(true);
  await store.close();
  const rest=await collectRemoteCacheDirectoryV1(external(folder,digests));
  expect(rest.locked).toBe(false);expect(names(folder),`kill ${kill}: the next collection finishes the job`).toEqual(expected);
  const orphans=readdirSync(join(folder,"v1",".meta"),{recursive:true}).filter(name=>String(name).endsWith(".json"));expect(orphans.length,`kill ${kill}: sidecars of removed entries are gone`).toBe(expected.length-1);
 }
 expect(points,"every removal has three crash points").toBe(9*3);expect(keys.length).toBe(13);
});

test("the store itself survives a crash inside its own collection and reopens consistent",async()=>{
 const tree=directory("tree-skill"),folder=directory("store-kill"),keys=await seedStore(folder,tree,40);await sleep(80);expect(keys.length).toBe(13);
 let count=0;
 await expect(open(folder,tree,"enforce",{graceMilliseconds:40,onStep:()=>{if(++count===5)throw Error("simulated crash");}})).rejects.toThrow("simulated crash");
 const between=await open(folder,tree,"off",{graceMilliseconds:600_000});
 for(const name of names(folder))expect(await live(between,name),name).toBe(true);
 await between.close();const again=await open(folder,tree,"enforce",{graceMilliseconds:40});await again.collect();expect(names(folder)).toEqual(["akey3","bkey3","ckey3","plain"]);await again.close();
});

test("readers racing a collection see a whole entry or a miss, never a partial one; two collectors and the store collecting at once do not corrupt it",async()=>{
 const tree=directory("tree-race"),folder=directory("race"),store=await open(folder,tree,"enforce",{graceMilliseconds:30}),content=(index:number)=>new Uint8Array(4096).fill(index%251);
 for(let index=0;index<5;index++)write(tree,{[`race${index}.rs`]:"v5"});
 for(let round=0;round<6;round++)for(let index=0;index<40;index++)await store.put("main","nx",`r${round}x${index}`,body(content(round*40+index)),{mode:"keep",declaredLength:4096,slot:{slot:slot("race"+index%5),declaration:declaration({source:[`race${index%5}.rs`,`v${round}`]})}});
 await sleep(60);let torn=0,hits=0,misses=0,running=true;
 const reader=(async()=>{
  while(running)for(let round=0;round<6;round++)for(let index=0;index<40;index++){
   const entry=await store.read("main","nx",`r${round}x${index}`);if(!entry){misses++;continue;}
   try{const bytes=await readFile(entry.path);if(bytes.length!==4096||!bytes.every(byte=>byte===(round*40+index)%251))torn++;else hits++;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;misses++;}
  }
 })();
 const digests=await openTreeDigestsV1(tree),request=external(folder,digests);
 const outcomes=await Promise.all([collectRemoteCacheDirectoryV1(request),collectRemoteCacheDirectoryV1(request),store.collect(),store.collect()]);
 await sleep(30);running=false;await reader;
 expect(torn).toBe(0);expect(hits+misses).toBeGreaterThan(0);expect(outcomes.slice(0,2).filter(row=>row!==undefined&&!row.locked).length,"at least one external collector ran").toBeGreaterThan(0);
 const left=await scanRemoteCacheDirectoryV1(join(folder,"v1"));expect(left.size,"only the entries of the current tree remain").toBe(40);
 for(const key of left.keys())expect(key).toMatch(/^main\/nx\/r5/u);
 await store.close();
});

test("an external collection leaves retirement to a running server, which knows the claims, and runs it when none runs",async()=>{
 const tree=directory("tree-def"),folder=directory("deferred"),keys=await seedStore(folder,tree,40);await sleep(80);expect(keys.length).toBe(13);const root=join(folder,"v1"),digests=await openTreeDigestsV1(tree);
 writeFileSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1),`${process.pid}\n${Date.now()}\n`);expect(await remoteCacheServerRunningV1(root)).toBe(true);
 const deferred=await collectRemoteCacheDirectoryV1(external(folder,digests,{dryRun:true}));
 expect(deferred.locked?undefined:deferred.value).toMatchObject({deferred:true,removed:{stale:0,entries:0}});
 const claimed=await collectRemoteCacheDirectoryV1(external(folder,digests,{dryRun:true,claimed:(key:string)=>key==="main/nx/ak/akey0"}));
 expect(claimed.locked?undefined:claimed.value).toMatchObject({deferred:false,removed:{stale:8},staleness:{mode:"enforce",stale:{edited:9}}});
 rmSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1));expect(await remoteCacheServerRunningV1(root)).toBe(false);
 const free=await collectRemoteCacheDirectoryV1(external(folder,digests,{dryRun:true}));
 expect(free.locked?undefined:free.value).toMatchObject({deferred:false,removed:{stale:9,entries:9}});
 const planOnly=await collectRemoteCacheDirectoryV1(external(folder,digests,{dryRun:true,staleness:{mode:"dry-run",tree:digests}}));
 expect(planOnly.locked?undefined:planOnly.value).toMatchObject({removed:{stale:0},staleness:{mode:"dry-run",stale:{edited:9}}});
});

test("a server that runs for days is not taken for dead: the lock is renewed, taken back from nobody, never taken from a living owner and never deleted by a process that lost it",async()=>{
 const root=directory("lock");
 const first=await acquireRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1);expect(first.held).toBe(false);
 writeFileSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1),`${process.pid}\n${Date.now()-7*3600_000}\n`);expect(await remoteCacheServerRunningV1(root),"older than the six-hour rule").toBe(false);
 expect(await renewRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1)).toBe(true);expect(await remoteCacheServerRunningV1(root),"renewed").toBe(true);
 writeFileSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1),`2147483646\n${Date.now()}\n`);expect(await renewRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1),"names a dead process").toBe(true);
 expect(readFileSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1),"utf8").split("\n")[0]).toBe(String(process.pid));
 writeFileSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1),`${process.ppid}\n${Date.now()}\n`);expect(await renewRemoteCacheLockV1(root,REMOTE_CACHE_SERVER_LOCK_V1),"names another living process").toBe(false);
 if(!first.held)await first.release();expect(readFileSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1),"utf8").split("\n")[0],"the owner that lost it does not delete the lock of its successor").toBe(String(process.ppid));
});

const varint=(value:number):number[]=>{const out:number[]=[];let rest=value;while(rest>=0x80){out.push((rest&0x7f)|0x80);rest=Math.floor(rest/0x80);}out.push(rest);return out;};
const named=(files:readonly {path:string;bytes:Uint8Array}[]):Uint8Array=>{
 const parts:number[]=[];
 for(const file of files){const text=new TextEncoder().encode(sha(file.bytes)),path=new TextEncoder().encode(file.path),digest=[0x0a,0x40,...text,0x10,...varint(file.bytes.length)],entry=[0x0a,...varint(path.length),...path,0x12,...varint(digest.length),...digest];parts.push(0x12,...varint(entry.length),...entry);}
 return new Uint8Array(parts);
};

test("the publish gate: an entry whose sources differ from the tree is refused as stale-on-arrival, one the replica catches up with is accepted, dry-run only counts, and the status reports it",async()=>{
 const tree=directory("tree-http"),folder=directory("http"),store=await open(folder,tree,"enforce"),handler=createRemoteCacheHandlerV1({store,tokens:{write:"secret",read:"look"},defaultInstance:"main"});write(tree,{"core/lib.rs":"core v2"});
 const send=(method:string,path:string,init:{headers?:Record<string,string>;body?:string|Uint8Array}={})=>handler(new Request(`http://cache.test${path}`,{method,headers:{authorization:"Bearer secret",...init.headers},...(init.body===undefined?{}:{body:init.body as BodyInit})}));
 const publish=async(key:string,built:Built)=>{
  const manifest=new TextEncoder().encode(JSON.stringify({version:1,key,retention:declaration(built)})),out=new TextEncoder().encode("rlib of "+key);
  for(const bytes of[manifest,out])expect((await send("PUT",`/cas/${sha(bytes)}`,{headers:{"content-length":String(bytes.length)},body:bytes})).status).toBe(200);
  const result=named([{path:"unit.json",bytes:manifest},{path:"out/lib.rlib",bytes:out}]);
  return send("PUT",`/ac/${sha(key)}`,{headers:{"content-length":String(result.length),"x-semio-slot":slot("core")},body:result});
 };
 expect((await publish("fresh",{source:["core/lib.rs","core v2"]})).status,"matches the tree").toBe(200);
 const refused=await publish("old",{source:["core/lib.rs","core v1"]});expect(refused.status).toBe(422);expect(await refused.text()).toContain("stale-on-arrival: edited core/lib.rs");
 expect((await send("HEAD",`/ac/${sha("old")}`)).status,"nothing was stored").toBe(404);expect((await send("HEAD",`/ac/${sha("fresh")}`)).status).toBe(200);
 await store.collect();
 expect(store.stalenessStatus()?.refused).toEqual({"stale-on-arrival":1});
 const status=await (await send("GET","/status")).json() as {retention?:{mode:string;refused:Record<string,number>}};expect(status.retention).toMatchObject({mode:"enforce",refused:{"stale-on-arrival":1}});
 expect((await publish("gone",{source:["missing.rs","x"]})).status,"a deleted source is refused too").toBe(422);
 expect((await publish("tool",{source:["core/lib.rs","core v2"],toolchain:"t9"})).status,"without a toolchain policy the toolchain is not judged").toBe(200);
 expect((await send("GET",`/ac/${sha("fresh")}`)).status).toBe(200);
 await store.close();
 const lagging=await RemoteCacheStoreV1.open({directory:directory("lag"),maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,staleness:await policy(tree,"enforce",300)}),lag=createRemoteCacheHandlerV1({store:lagging,tokens:{write:"secret"},defaultInstance:"main"});
 const manifest=new TextEncoder().encode(JSON.stringify({version:1,retention:declaration({source:["core/lib.rs","core v3"]})})),out=new TextEncoder().encode("rlib");
 for(const bytes of[manifest,out])await lag(new Request(`http://cache.test/cas/${sha(bytes)}`,{method:"PUT",headers:{authorization:"Bearer secret","content-length":String(bytes.length)},body:bytes}));
 const result=named([{path:"unit.json",bytes:manifest},{path:"out/lib.rlib",bytes:out}]);
 setTimeout(()=>{writeFileSync(join(tree,"core/lib.rs"),"core v3");},100);
 const late=await lag(new Request(`http://cache.test/ac/${sha("late")}`,{method:"PUT",headers:{authorization:"Bearer secret","content-length":String(result.length),"x-semio-slot":slot("core")},body:result}));
 expect(late.status,"the replica caught up during the recheck").toBe(200);await lagging.close();
 const dry=await open(directory("dry-http"),tree,"dry-run"),dryHandler=createRemoteCacheHandlerV1({store:dry,tokens:{write:"secret"},defaultInstance:"main"});
 for(const bytes of[manifest,out])await dryHandler(new Request(`http://cache.test/cas/${sha(bytes)}`,{method:"PUT",headers:{authorization:"Bearer secret","content-length":String(bytes.length)},body:bytes}));
 writeFileSync(join(tree,"core/lib.rs"),"core v4");
 expect((await dryHandler(new Request(`http://cache.test/ac/${sha("dry")}`,{method:"PUT",headers:{authorization:"Bearer secret","content-length":String(result.length),"x-semio-slot":slot("core")},body:result}))).status,"dry-run accepts").toBe(200);
 await dry.collect();expect(dry.stalenessStatus()?.refused).toEqual({"would-refuse-stale-on-arrival":1});await dry.close();
});

test("the HTTP handler stores the slot header as a dedupe hint, refuses a malformed slot and claims through POST /v1/slots; the slot never retires anything",async()=>{
 const tree=directory("tree-slot"),folder=directory("slotted"),store=await open(folder,tree,"enforce",{graceMilliseconds:100}),handler=createRemoteCacheHandlerV1({store,tokens:{write:"secret",read:"look"},defaultInstance:"main"});
 const send=(method:string,path:string,init:{headers?:Record<string,string>;body?:string|Uint8Array}={})=>handler(new Request(`http://cache.test${path}`,{method,headers:{authorization:"Bearer secret",...init.headers},...(init.body===undefined?{}:{body:init.body as BodyInit})}));
 const header=(name:string)=>({"x-semio-slot":slot(name)});
 expect((await send("PUT","/v1/cache/aa1",{headers:{"content-length":"3",...header("t")},body:"one"})).status).toBe(200);
 expect((await send("PUT","/v1/cache/aa2",{headers:{"content-length":"3",...header("t")},body:"two"})).status).toBe(200);
 expect((await send("PUT","/v1/cache/aa3",{headers:{"content-length":"3","x-semio-slot":"nope"},body:"bad"})).status).toBe(400);
 expect((await send("PUT","/v1/cache/aa1",{headers:{"content-length":"3",...header("t")},body:"one"})).status,"an equal key conflicts as before").toBe(409);
 expect((await store.usage()).slotted).toBe(2);
 await sleep(150);await store.collect();
 expect((await send("GET","/v1/cache/aa1")).status,"a newer key of the slot does not retire the older one").toBe(200);expect((await send("GET","/v1/cache/aa2")).status).toBe(200);
 const claim=await send("POST","/v1/slots",{body:JSON.stringify({entries:[{kind:"nx",key:"aa1",slot:slot("t")},{kind:"nx",key:"missing",slot:slot("t")}]})});
 expect(await claim.json()).toEqual({claimed:1,missing:1});
 expect((await send("POST","/v1/slots",{body:"{}"})).status).toBe(400);
 expect((await handler(new Request("http://cache.test/main/v1/slots",{method:"POST",headers:{authorization:"Bearer look"},body:JSON.stringify({entries:[]})}))).status,"a read token cannot claim").toBe(403);
 expect((await send("GET","/v1/slots")).status).toBe(405);
 await store.close();
});

test("a slotted action result records the blobs it references; the sidecar is written after the entry",async()=>{
 const folder=directory("ac"),cas=directory("ac-cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}}),store=await RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,host,graceMilliseconds:100});
 const blob=new TextEncoder().encode("alpha\n"),hash=sha(blob);
 expect(await store.put("main","cas",hash,body(blob),{mode:"content",verify:true,declaredLength:blob.length})).toBe("created");
 const pointer=(value:string)=>{const digest=new Uint8Array([0x0a,0x40,...new TextEncoder().encode(value),0x10,0x06]);return new Uint8Array([0x12,digest.length+5,0x0a,0x01,0x70,0x12,digest.length,...digest]);};
 expect(await store.put("main","ac",sha("one"),body(pointer(hash)),{mode:"replace",slot:{slot:slot("u")}})).toBe("created");
 const meta=await readRemoteCacheMetaV1(store.root,`main/ac/${sha("one").slice(0,2)}/${sha("one")}`);
 expect(meta).toMatchObject({version:1,slot:slot("u"),refs:[hash]});
 const metas=await scanRemoteCacheMetaV1(store.root,new Map([[`main/ac/${sha("one").slice(0,2)}/${sha("one")}`,1]]));expect(metas.size).toBe(1);
 mkdirSync(join(store.root,".meta","main","ac","zz"),{recursive:true});writeFileSync(join(store.root,".meta","main","ac","zz","orphan.json"),"{}");
 expect((await scanRemoteCacheMetaV1(store.root,new Map())).size,"a sidecar without its entry is removed").toBe(0);
 expect(existsSync(join(store.root,".meta","main","ac","zz","orphan.json"))).toBe(false);
 expect(readFileSync(join(store.root,"main","ac",sha("one").slice(0,2),sha("one"))).length).toBeGreaterThan(0);
 await store.close();
});

test("the audit against the key set of the current state is the independent oracle: after a walk through ten states and one collection only that state remains",async()=>{
 const startedAt=Date.now(),next=seeded(20261114),tree=directory("tree-audit"),folder=directory("audit"),store=await open(folder,tree,"enforce",{graceMilliseconds:600}),tasks=12;
 const state=new Map<number,string>(),version=new Map<number,number>();let counter=0;
 const walk=async(changed:number)=>{
  for(let task=0;task<tasks;task++){
   if(!state.has(task)||task<changed){state.set(task,`h${String(++counter).padStart(4,"0")}`);version.set(task,(version.get(task)??0)+1);write(tree,{[`t${task}.rs`]:`v${version.get(task)}`});}
   const key=state.get(task)!;await store.put("main","nx",key,body(fill(key)),{mode:"keep",declaredLength:64,slot:{slot:slot("task"+task),declaration:declaration({source:[`t${task}.rs`,`v${version.get(task)}`]})}});
  }
 };
 const seen=new Set<string>();
 for(let step=0;step<10;step++){await walk(Math.floor(next()*tasks));for(const key of state.values())seen.add(key);await sleep(10);}
 const current=new Set(state.values());
 const during=await auditRemoteCacheDirectoryV1(store.root,current,{graceMilliseconds:600,now:startedAt+500});
 expect(during.stray,"inside the grace nothing is stray").toEqual([]);expect(during.grace).toBeGreaterThan(0);expect(during.current).toBe(tasks);
 await sleep(700);
 const before=await auditRemoteCacheDirectoryV1(store.root,current,{graceMilliseconds:600});
 expect(before.stray.length,"before the collection the history is stray").toBe(seen.size-tasks);expect(before.grace).toBe(0);
 await store.collect();
 const after=await auditRemoteCacheDirectoryV1(store.root,current,{graceMilliseconds:600});
 expect(after).toMatchObject({stray:[],grace:0,current:tasks,unslotted:0});
 const onDisk=new Set([...(await scanRemoteCacheDirectoryV1(store.root)).keys()].map(key=>key.split("/").pop()!));
 expect([...onDisk].sort(),"exactly the keys of the current state are on disk").toEqual([...current].sort());
 const stale=new Set([...current].slice(1));
 expect((await auditRemoteCacheDirectoryV1(store.root,stale,{graceMilliseconds:600})).stray.length,"a key the state does not need is reported once its grace passed").toBe(1);
 await store.close();
});

test("reclaim never removes a blob that a live operation leases as its input, and takes it once the lease ended",async()=>{
 const cas=directory("reclaim-lease"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}}),digest=await host.put(new TextEncoder().encode("input of a queued final"));
 await host.lease([digest.hash],undefined,"op-live");
 expect(await reclaimCasObjectsV1(host,[digest.hash])).toMatchObject({entries:0,leased:1});expect(existsSync(host.objectPath(digest.hash))).toBe(true);
 await host.releaseLease([digest.hash],"op-live");
 expect(await reclaimCasObjectsV1(host,[digest.hash]),"the short grace after the release still protects").toMatchObject({entries:0,leased:1});
 expect(await reclaimCasObjectsV1(host,[digest.hash],{now:()=>Date.now()+3_600_000})).toMatchObject({entries:1,leased:0});expect(existsSync(host.objectPath(digest.hash))).toBe(false);
});
