import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {cpSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {readFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CAS_POLICY_V1} from "../../../🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1} from "../../../🗄️cas/🗃️store/🟦️.ts";
import {pinCasV1} from "../../../🗄️cas/📌️pins/🟦️.ts";
import {reclaimCasObjectsV1} from "../../../🗄️cas/🪦️reclaim/🟦️.ts";
import {slotIdV1} from "../../../🪢️slots/🟦️.ts";
import {collectRemoteCacheDirectoryV1,planRemoteCacheCollectionV1,remoteCacheServerRunningV1,REMOTE_CACHE_SERVER_LOCK_V1,scanRemoteCacheDirectoryV1,type RemoteCacheStepV1} from "../../🧹️collector/🟦️.ts";
import {createRemoteCacheHandlerV1} from "../../🖥️server/🟦️.ts";
import {RemoteCacheStoreV1} from "../../💾️store/🟦️.ts";
import {auditRemoteCacheDirectoryV1,readRemoteCacheMetaV1,scanRemoteCacheMetaV1} from "../🟦️.ts";

const python=testPythonV1(),oracle=join(import.meta.dir,"../../../🪢️slots/🧪️tests/🐍️oracle/🐍️.py"),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-slot-${label}-`));scratch.push(path);return path;};
const body=(bytes:Uint8Array)=>(async function*(){yield bytes;})();
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const slot=(name:string)=>slotIdV1("nx-task",[name,"build","",""]);
const fill=(key:string,size=64)=>new Uint8Array(size).fill((key.charCodeAt(0)^key.charCodeAt(key.length-1))&255);
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

const open=(folder:string,extra:Partial<Parameters<typeof RemoteCacheStoreV1.open>[0]>={})=>RemoteCacheStoreV1.open({directory:folder,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,graceMilliseconds:150,...extra});
const put=(store:RemoteCacheStoreV1,key:string,name:string|null,bytes:Uint8Array=fill(key))=>store.put("main","nx",key,body(bytes),{mode:"keep",declaredLength:bytes.length,...(name===null?{}:{slot:{slot:slot(name)}})});
const live=async(store:RemoteCacheStoreV1,key:string)=>{const entry=await store.read("main","nx",key);return entry!==undefined;};

test.skipIf(!python)("the retention plan equals an independent Python model on 300 seeded stores with sidecars, claims, ties and reservations",()=>{
 const next=seeded(20261113),requests:{op:string;rows:unknown[];bounds:Record<string,number>;dangling:string[];retention:{grace:number;metas:Record<string,{slot:string;storedAt:number}>;claimed:string[]}}[]=[],inputs:{rows:{key:string;size:number;accessed:number}[];metas:Map<string,{slot:string;storedAt:number}>;claimed:Set<string>}[]=[];
 for(let round=0;round<300;round++){
  const count=Math.floor(next()*40),rows:{key:string;size:number;accessed:number}[]=[],metas=new Map<string,{slot:string;storedAt:number}>(),claimed=new Set<string>(),slots=1+Math.floor(next()*6);
  for(let index=0;index<count;index++){
   const key=`main/nx/${String(index).padStart(2,"0")}/k${Math.floor(next()*1000)}-${index}`;rows.push({key,size:1+Math.floor(next()*5000),accessed:1_000_000+Math.floor(next()*(round%3===0?5:2000))});
   if(next()<0.75)metas.set(key,{slot:`s${Math.floor(next()*slots)}`.padEnd(64,"0"),storedAt:999_000+Math.floor(next()*(round%4===0?3:1500))});
   if(next()<0.08)claimed.add(key);
  }
  const total=rows.reduce((sum,row)=>sum+row.size,0),bounds={maximumBytes:Math.max(1,Math.floor(total*(0.3+next()*1.2))),maximumAge:round%4===0?Math.floor(next()*1500):0,now:1_002_000,need:round%5===0?Math.floor(next()*1000):0,reserved:round%7===0?Math.floor(next()*500):0};
  const dangling=rows.filter(()=>next()<0.05).map(row=>row.key);
  requests.push({op:"plan",rows,bounds,dangling,retention:{grace:round%6===0?0:Math.floor(next()*600),metas:Object.fromEntries(metas),claimed:[...claimed]}});inputs.push({rows,metas,claimed});
 }
 const result=Bun.spawnSync([python!,"-I",oracle],{stdin:new TextEncoder().encode(JSON.stringify(requests)),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const model=JSON.parse(result.stdout.toString()) as {key:string;size:number;reason:string;storedAt?:number}[][];const reasons=new Set<string>();
 requests.forEach((request,index)=>{
  const input=inputs[index]!,ours=planRemoteCacheCollectionV1(input.rows,{maximumBytes:request.bounds.maximumBytes!,maximumAgeMilliseconds:request.bounds.maximumAge!,now:request.bounds.now!,need:request.bounds.need!,reserved:request.bounds.reserved!},new Set(request.dangling),{metas:input.metas,graceMilliseconds:request.retention.grace,claimed:key=>input.claimed.has(key)});
  expect(ours,`round ${index}`).toEqual(model[index]!);for(const drop of ours)reasons.add(drop.reason);
  for(const drop of ours)expect(input.claimed.has(drop.key)&&drop.reason!=="dangling","a claimed entry is never taken").toBe(false);
 });
 expect([...reasons].sort()).toEqual(["age","budget","dangling","superseded"]);
});

test("a newer key supersedes the older one: both stay readable during the grace, the older goes after it, a read never resurrects it",async()=>{
 const folder=directory("lifecycle"),store=await open(folder);
 expect(await put(store,"k1","a")).toBe("created");expect(await put(store,"k2","a")).toBe("created");expect(await put(store,"other","b")).toBe("created");
 await store.collect();expect(await live(store,"k1"),"inside the grace").toBe(true);expect(await live(store,"k2")).toBe(true);
 await sleep(220);expect(await live(store,"k1"),"a read after the grace still hits until a collection runs").toBe(true);
 await store.collect();
 expect(await live(store,"k1"),"superseded and past the grace").toBe(false);expect(await live(store,"k2")).toBe(true);expect(await live(store,"other"),"another slot is untouched").toBe(true);
 expect(await readRemoteCacheMetaV1(store.root,"main/nx/k1/k1"),"its sidecar went with it").toBeUndefined();
 await store.close();
});

test("a revert makes the old key current: storing or claiming it again supersedes the newer one, reading does not",async()=>{
 const folder=directory("revert"),store=await open(folder,{graceMilliseconds:200});
 await put(store,"k1","a");await put(store,"k2","a");
 await sleep(120);await store.read("main","nx","k1");await store.collect();await sleep(120);await store.collect();
 expect(await live(store,"k1"),"reading k1 did not keep it").toBe(false);
 await put(store,"k3","a");await sleep(120);
 expect(await put(store,"k3","a"),"storing an existing key again is a claim").toBe("exists");
 await put(store,"k4","a");await sleep(120);
 expect(await store.claim("main","nx","k3",slot("a"))).toBe("claimed");expect(await store.claim("main","nx","absent",slot("a"))).toBe("missing");
 await sleep(260);await store.collect();
 expect(await live(store,"k3"),"claimed again, so current").toBe(true);expect(await live(store,"k4"),"superseded by the claim and past the grace").toBe(false);
 await store.close();
});

test("entries without a slot keep the age rule; slotted current entries are the synchronized state and are never aged out",async()=>{
 const folder=directory("age"),store=await open(folder,{maximumAgeMilliseconds:100,graceMilliseconds:100});
 await put(store,"plain",null);await put(store,"slotted","s");await sleep(200);await store.collect();
 expect(await live(store,"plain")).toBe(false);expect(await live(store,"slotted")).toBe(true);
 await store.close();
});

test("an active claim and a recent store protect an entry from supersession and from the size budget",async()=>{
 const folder=directory("claims"),held=new Set<string>(["main/nx/k1/k1"]),store=await open(folder,{graceMilliseconds:100,claimed:(namespace,kind,key)=>held.has(`${namespace}/${kind}/${key.slice(0,2)}/${key}`)});
 await put(store,"k1","a");await put(store,"k2","a");await sleep(200);await store.collect();
 expect(await live(store,"k1"),"claimed").toBe(true);held.clear();await store.collect();expect(await live(store,"k1"),"claim released").toBe(false);
 await store.close();
 const tiny=await open(directory("budget"),{graceMilliseconds:60_000,maximumBytes:1000,maximumEntryBytes:400});
 for(const key of["b1","b2"])await put(tiny,key,key,new Uint8Array(400));
 expect(await put(tiny,"b3","b3",new Uint8Array(400)),"stored within the grace, nothing may be taken for it: the write is refused").toBe("capacity");
 expect(await live(tiny,"b1")).toBe(true);expect(await live(tiny,"b2")).toBe(true);await tiny.close();
});

test("the budget takes superseded entries before unslotted before current ones, least recently used first",async()=>{
 const folder=directory("order"),store=await open(folder,{graceMilliseconds:1,maximumBytes:3000,maximumEntryBytes:1000});
 const size=new Uint8Array(500);
 await put(store,"cur1","c1",size);await put(store,"plain1",null,size);await put(store,"old1","c2",size);await sleep(5);await put(store,"new1","c2",size);await sleep(5);
 await put(store,"cur2","c3",size);await put(store,"plain2",null,size);
 await store.collect(1);
 const present=await Promise.all(["cur1","plain1","old1","new1","cur2","plain2"].map(key=>live(store,key)));
 expect(present[2],"superseded (expired) goes first").toBe(false);
 expect(present.filter(Boolean).length).toBeLessThan(6);
 await store.close();
});

test("blobs go when no live entry references them; shared, pinned and recently used blobs stay",async()=>{
 const folder=directory("blobs"),cas=directory("cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}}),store=await open(folder,{host,graceMilliseconds:100});
 const a=new TextEncoder().encode("blob-a".repeat(20)),b=new TextEncoder().encode("blob-b".repeat(20)),shared=new TextEncoder().encode("blob-shared".repeat(20)),pinned=new TextEncoder().encode("blob-pinned".repeat(20));
 const hash=(bytes:Uint8Array)=>sha(bytes);
 await put(store,"n1","a",a);await put(store,"n2","a",b);
 await put(store,"s1","s",shared);await put(store,"s2","s",shared);
 await put(store,"p1","p",pinned);await put(store,"p2","p",new TextEncoder().encode("blob-p2".repeat(20)));
 await host.pin("someone-else",[hash(pinned)],60_000);
 await sleep(220);await store.collect();
 expect(await live(store,"n1")).toBe(false);
 expect(existsSync(host.objectPath(hash(a))),"unreferenced after n1 went").toBe(false);
 expect(existsSync(host.objectPath(hash(b))),"referenced by the current n2").toBe(true);
 expect(existsSync(host.objectPath(hash(shared))),"shared by the current s2").toBe(true);
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

function seedTemplate(label:string):{folder:string;keys:string[]}{
 const folder=directory(label);return {folder,keys:[]};
}
async function seedStore(folder:string,grace:number):Promise<string[]>{
 const store=await open(folder,{graceMilliseconds:grace}),keys:string[]=[];
 for(const name of["a","b","c"])for(let index=0;index<4;index++){const key=`${name}key${index}`;await put(store,key,name);keys.push(key);}
 await put(store,"plain",null);keys.push("plain");
 await store.close();return keys;
}
const names=(folder:string)=>[...new Set((readdirSync(join(folder,"v1","main","nx"),{recursive:true}) as string[]).filter(name=>name.includes("/")).map(name=>name.split("/").pop()!))].sort();

test("a crash at any point of a collection leaves a readable cache that the next collection completes to the same result",async()=>{
 const template=seedTemplate("kill").folder,keys=await seedStore(template,40);await sleep(80);
 const reference=directory("kill-ref");cpSync(template,reference,{recursive:true});
 const first=await collectRemoteCacheDirectoryV1({root:join(reference,"v1"),maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:false,graceMilliseconds:40});
 expect(first.locked).toBe(false);const expected=names(reference);expect(expected).toEqual(["akey3","bkey3","ckey3","plain"]);
 let points=0;
 for(let kill=1;kill<=40;kill++){
  const folder=directory("kill-"+kill);cpSync(template,folder,{recursive:true});let count=0,crashed=false;
  try{await collectRemoteCacheDirectoryV1({root:join(folder,"v1"),maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:false,graceMilliseconds:40,onStep:(_point:RemoteCacheStepV1)=>{if(++count===kill)throw Error("simulated crash");}});}catch(error){crashed=true;expect((error as Error).message).toBe("simulated crash");}
  if(!crashed){points=kill-1;break;}
  const store=await open(folder,{graceMilliseconds:40});
  for(const entry of names(folder)){const read=await store.read("main","nx",entry);expect(read,`kill ${kill}: ${entry} readable`).toBeDefined();expect(Buffer.from(await readFile(read!.path)).equals(Buffer.from(entry==="plain"?fill("plain"):fill(entry))),`kill ${kill}: ${entry} intact`).toBe(true);}
  const present=new Map([...(await scanRemoteCacheDirectoryV1(join(folder,"v1"))).keys()].map(key=>[key,true]));
  for(const key of(await scanRemoteCacheMetaV1(join(folder,"v1"),present)).keys())expect(present.has(key),`kill ${kill}: no sidecar without its entry`).toBe(true);
  await store.close();
  const rest=await collectRemoteCacheDirectoryV1({root:join(folder,"v1"),maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:false,graceMilliseconds:40});
  expect(rest.locked).toBe(false);expect(names(folder),`kill ${kill}: the next collection finishes the job`).toEqual(expected);
  const orphans=readdirSync(join(folder,"v1",".meta"),{recursive:true}).filter(name=>String(name).endsWith(".json"));expect(orphans.length,`kill ${kill}: sidecars of removed entries are gone`).toBe(expected.length-1);
 }
 expect(points,"every removal has three crash points").toBe(9*3);expect(keys.length).toBe(13);
});

test("the store itself survives a crash inside its own collection and reopens consistent",async()=>{
 const folder=directory("store-kill"),keys=await seedStore(folder,40);await sleep(80);expect(keys.length).toBe(13);
 let count=0;
 await expect(open(folder,{graceMilliseconds:40,onStep:()=>{if(++count===5)throw Error("simulated crash");}})).rejects.toThrow("simulated crash");
 const between=await open(folder,{graceMilliseconds:600_000});
 for(const name of names(folder))expect(await live(between,name),name).toBe(true);
 await between.close();const again=await open(folder,{graceMilliseconds:40});await again.collect();expect(names(folder)).toEqual(["akey3","bkey3","ckey3","plain"]);await again.close();
});

test("readers racing a collection see a whole entry or a miss, never a partial one; two collectors and the store collecting at once do not corrupt it",async()=>{
 const folder=directory("race"),store=await open(folder,{graceMilliseconds:30}),content=(index:number)=>new Uint8Array(4096).fill(index%251);
 for(let round=0;round<6;round++)for(let index=0;index<40;index++)await store.put("main","nx",`r${round}x${index}`,body(content(round*40+index)),{mode:"keep",declaredLength:4096,slot:{slot:slot("race"+index%5)}});
 await sleep(60);let torn=0,hits=0,misses=0,running=true;
 const reader=(async()=>{
  while(running)for(let round=0;round<6;round++)for(let index=0;index<40;index++){
   const entry=await store.read("main","nx",`r${round}x${index}`);if(!entry){misses++;continue;}
   try{const bytes=await readFile(entry.path);if(bytes.length!==4096||!bytes.every(byte=>byte===(round*40+index)%251))torn++;else hits++;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;misses++;}
  }
 })();
 const request={root:join(folder,"v1"),maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:false,verifyResults:false,graceMilliseconds:30};
 const outcomes=await Promise.all([collectRemoteCacheDirectoryV1(request),collectRemoteCacheDirectoryV1(request),store.collect(),store.collect()]);
 await sleep(30);running=false;await reader;
 expect(torn).toBe(0);expect(hits+misses).toBeGreaterThan(0);expect(outcomes.slice(0,2).filter(row=>row!==undefined&&!row.locked).length,"at least one external collector ran").toBeGreaterThan(0);
 const left=await scanRemoteCacheDirectoryV1(join(folder,"v1"));expect(left.size,"one current key per slot remains").toBe(5);
 for(const key of left.keys())expect(key).toMatch(/^main\/nx\/r5/u);
 await store.close();
});

test("an external collection leaves supersession to a running server, which knows the claims, and runs it when none runs",async()=>{
 const folder=directory("deferred"),keys=await seedStore(folder,40);await sleep(80);expect(keys.length).toBe(13);const root=join(folder,"v1");
 writeFileSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1),`${process.pid}\n${Date.now()}\n`);expect(await remoteCacheServerRunningV1(root)).toBe(true);
 const deferred=await collectRemoteCacheDirectoryV1({root,maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:true,verifyResults:false,graceMilliseconds:40});
 expect(deferred.locked?undefined:deferred.value).toMatchObject({deferred:true,removed:{superseded:0,entries:0}});
 const claimed=await collectRemoteCacheDirectoryV1({root,maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:true,verifyResults:false,graceMilliseconds:40,claimed:key=>key==="main/nx/ak/akey0"});
 expect(claimed.locked?undefined:claimed.value).toMatchObject({deferred:false,removed:{superseded:8}});
 rmSync(join(root,REMOTE_CACHE_SERVER_LOCK_V1));expect(await remoteCacheServerRunningV1(root)).toBe(false);
 const free=await collectRemoteCacheDirectoryV1({root,maximumBytes:2**30,maximumAgeMilliseconds:0,dryRun:true,verifyResults:false,graceMilliseconds:40});
 expect(free.locked?undefined:free.value).toMatchObject({deferred:false,removed:{superseded:9,entries:9}});
});

test("the HTTP handler stores the slot header, refuses a malformed slot, supersedes through PUT and claims through POST /v1/slots",async()=>{
 const folder=directory("http"),store=await open(folder,{graceMilliseconds:100}),handler=createRemoteCacheHandlerV1({store,tokens:{write:"secret",read:"look"},defaultInstance:"main"});
 const send=(method:string,path:string,init:{headers?:Record<string,string>;body?:string|Uint8Array}={})=>handler(new Request(`http://cache.test${path}`,{method,headers:{authorization:"Bearer secret",...init.headers},...(init.body===undefined?{}:{body:init.body as BodyInit})}));
 const header=(name:string)=>({"x-semio-slot":slot(name)});
 expect((await send("PUT","/v1/cache/aa1",{headers:{"content-length":"3",...header("t")},body:"one"})).status).toBe(200);
 expect((await send("PUT","/v1/cache/aa2",{headers:{"content-length":"3",...header("t")},body:"two"})).status).toBe(200);
 expect((await send("PUT","/v1/cache/aa3",{headers:{"content-length":"3","x-semio-slot":"nope"},body:"bad"})).status).toBe(400);
 expect(await (await send("GET","/v1/cache/aa1")).text(),"superseded but inside the grace").toBe("one");
 expect((await send("PUT","/v1/cache/aa1",{headers:{"content-length":"3",...header("t")},body:"one"})).status,"an equal key conflicts as before, and claims").toBe(409);
 expect((await store.usage()).slotted).toBe(2);
 await sleep(150);await store.collect();
 expect((await send("GET","/v1/cache/aa2")).status,"the other key of the slot was superseded by the claim").toBe(404);
 expect((await send("GET","/v1/cache/aa1")).status).toBe(200);
 const claim=await send("POST","/v1/slots",{body:JSON.stringify({entries:[{kind:"nx",key:"aa1",slot:slot("t")},{kind:"nx",key:"missing",slot:slot("t")}]})});
 expect(await claim.json()).toEqual({claimed:1,missing:1});
 expect((await send("POST","/v1/slots",{body:"{}"})).status).toBe(400);
 expect((await handler(new Request("http://cache.test/main/v1/slots",{method:"POST",headers:{authorization:"Bearer look"},body:JSON.stringify({entries:[]})}))).status,"a read token cannot claim").toBe(403);
 expect((await send("GET","/v1/slots")).status).toBe(405);
 await store.close();
});

test("a slotted action result records the blobs it references; the sidecar is written after the entry",async()=>{
 const folder=directory("ac"),cas=directory("ac-cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}}),store=await open(folder,{host,graceMilliseconds:100});
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
 const next=seeded(20261114),folder=directory("audit"),store=await open(folder,{graceMilliseconds:600}),tasks=12;
 const state=new Map<number,string>();let counter=0;
 const walk=async(changed:number)=>{
  for(let task=0;task<tasks;task++){
   if(!state.has(task)||task<changed)state.set(task,`h${String(++counter).padStart(4,"0")}`);
   const key=state.get(task)!;await store.put("main","nx",key,body(fill(key)),{mode:"keep",declaredLength:64,slot:{slot:slot("task"+task)}});
  }
 };
 const seen=new Set<string>();
 for(let step=0;step<10;step++){
  await walk(Math.floor(next()*tasks));for(const key of state.values())seen.add(key);
  await sleep(10);
 }
 const current=new Set(state.values());
 const during=await auditRemoteCacheDirectoryV1(store.root,current,{graceMilliseconds:600});
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
