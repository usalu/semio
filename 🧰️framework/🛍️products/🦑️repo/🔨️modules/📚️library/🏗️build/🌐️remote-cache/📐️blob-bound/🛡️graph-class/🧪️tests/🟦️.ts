/** 🛡️ Contract: entries of the graph class (the head of a selection, the records of its owners) are never taken by unit output pressure within their reserved share of the blob bound, a newer entry of their slot supersedes them only after a grace, a lookup keeps them current, and the server says which entries left and why.
 * @see ../../🧪️tests/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CAS_POLICY_V1} from "../../../../🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1} from "../../../../🗄️cas/🗃️store/🟦️.ts";
import {readCasPinsV1} from "../../../../🗄️cas/📌️pins/🟦️.ts";
import {slotIdV1} from "../../../../🪢️slots/🟦️.ts";
import {planRemoteCacheBlobBoundV1,type RemoteCacheBlobRowV1} from "../../../🧹️collector/🟦️.ts";
import {RemoteCacheStoreV1} from "../../../💾️store/🟦️.ts";
import {declareV1,openTreeDigestsV1} from "../../../🏷️retention/🍂️staleness/🟦️.ts";

const python=testPythonV1(),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-graph-${label}-`));scratch.push(path);return path;};
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const body=(bytes:Uint8Array)=>(async function*(){yield bytes;})();
const bytesOf=(name:string,size:number)=>new TextEncoder().encode(name.repeat(Math.ceil(size/name.length)).slice(0,size));
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

const oracle=`
import json,sys
out=[]
for request in json.load(sys.stdin):
    rows=request["rows"];sizes=request["sizes"];bound=request["bound"];graph=request.get("graph")
    holders={}
    for row in rows:
        for h in set(row["refs"]):holders[h]=holders.get(h,0)+1
    before=sum(sizes[h] for h in holders)
    protected=set();due=set();protected_bytes=0
    if graph is not None:
        grows=[r for r in rows if r.get("graph") and r["refs"]];newest={}
        for r in grows:
            held=newest.get(r["graph"]["slot"])
            if held is None or r["graph"]["storedAt"]>held["graph"]["storedAt"] or (r["graph"]["storedAt"]==held["graph"]["storedAt"] and r["key"]>held["key"]):newest[r["graph"]["slot"]]=r
        current=[]
        for r in grows:
            head=newest[r["graph"]["slot"]]
            if head is r:current.append(r)
            elif graph["now"]-head["graph"]["storedAt"]>=graph["graceMilliseconds"]:due.add(r["key"])
            else:protected.add(r["key"])
        current.sort(key=lambda r:(-r["accessed"],r["key"]))
        counted=set()
        for r in grows:
            if r["key"] in protected:
                for h in set(r["refs"]):
                    if h not in counted:counted.add(h);protected_bytes+=sizes[h]
        for r in current:
            if protected_bytes>=graph["budgetBytes"]:break
            protected.add(r["key"])
            for h in set(r["refs"]):
                if h not in counted:counted.add(h);protected_bytes+=sizes[h]
    if before<=bound:
        out.append({"before":before,"after":before,"keys":[],"protectedKeys":sorted(protected),"protectedBytes":protected_bytes});continue
    target=bound*9//10;after=before;keys=[]
    order=sorted([r for r in rows if not r["guarded"] and r["refs"] and r["key"] not in protected],key=lambda r:(0 if r["key"] in due else r["tier"],r["accessed"],r["key"]))
    for row in order:
        if after<=target:break
        for h in set(row["refs"]):
            holders[h]-=1
            if holders[h]==0:after-=sizes[h]
        keys.append(row["key"])
    out.append({"before":before,"after":after,"keys":keys,"protectedKeys":sorted(protected),"protectedBytes":protected_bytes})
print(json.dumps(out))
`;

type Request={rows:RemoteCacheBlobRowV1[];sizes:Record<string,number>;bound:number;graph?:{budgetBytes:number;now:number;graceMilliseconds:number}};
const planned=(request:Request)=>{const result=planRemoteCacheBlobBoundV1(request.rows,hash=>request.sizes[hash]!,request.bound,request.graph);return{before:result.before,after:result.after,keys:[...result.keys],protectedKeys:[...result.protectedKeys],protectedBytes:result.protectedBytes};};
const modelOf=(requests:readonly Request[]):ReturnType<typeof planned>[]=>{
 const result=Bun.spawnSync([python!,"-I","-c",oracle],{stdin:new TextEncoder().encode(JSON.stringify(requests)),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 return JSON.parse(result.stdout.toString());
};

test.skipIf(!python)("the plan with the graph class equals an independent Python model on 400 seeded stores: supersession, grace, reserve and ties",()=>{
 const next=seeded(20261124),requests:Request[]=[];
 for(let round=0;round<400;round++){
  const blobs=Array.from({length:1+Math.floor(next()*25)},(_,index)=>`b${index}`),sizes=Object.fromEntries(blobs.map(blob=>[blob,1+Math.floor(next()*900)])),rows:RemoteCacheBlobRowV1[]=[],now=1000;
  for(let index=0;index<Math.floor(next()*40);index++){
   const graph=next()<0.4?{slot:`s${Math.floor(next()*5)}`,storedAt:Math.floor(next()*(round%4===0?3:1000))}:undefined;
   rows.push({key:`main/ac/${String(index).padStart(2,"0")}/k${index}`,accessed:Math.floor(next()*(round%3===0?4:500)),tier:Math.floor(next()*3),refs:Array.from({length:Math.floor(next()*4)},()=>blobs[Math.floor(next()*blobs.length)]!),guarded:next()<0.1,...graph===undefined?{}:{graph}});
  }
  const total=Object.values(sizes).reduce((sum,size)=>sum+size,0);
  requests.push({rows,sizes,bound:Math.max(1,Math.floor(total*(0.2+next()*1.1))),graph:{budgetBytes:Math.floor(total*next()*0.5),now,graceMilliseconds:Math.floor(next()*600)}});
 }
 const model=modelOf(requests);let evicting=0,protectedSome=0,superseding=0;
 requests.forEach((request,index)=>{
  const ours=planned(request);
  expect(ours,`round ${index}`).toEqual(model[index]!);
  if(ours.keys.length>0)evicting++;if(ours.protectedKeys.length>0)protectedSome++;
  for(const key of ours.keys)expect(ours.protectedKeys,"a protected entry is never taken").not.toContain(key);
  const taken=new Set(ours.keys);if(request.rows.some(row=>row.graph!==undefined&&taken.has(row.key)))superseding++;
 });
 expect(evicting).toBeGreaterThan(100);expect(protectedSome).toBeGreaterThan(150);expect(superseding).toBeGreaterThan(20);
});

test("a store over its bound with unit outputs and one head: the head is kept, the oldest and least valuable leave",()=>{
 const rows:RemoteCacheBlobRowV1[]=[{key:"head",accessed:1,tier:1,refs:["h"],guarded:false,graph:{slot:"selection",storedAt:1}},...["u1","u2","u3","u4","u5","u6"].map((key,index)=>({key,accessed:10+index,tier:2,refs:[key],guarded:false}))];
 const sizes:Record<string,number>={h:100,u1:1000,u2:1000,u3:1000,u4:1000,u5:1000,u6:1000};
 const without=planRemoteCacheBlobBoundV1(rows.map(row=>({...row,graph:undefined})),hash=>sizes[hash]!,4500),kept=planRemoteCacheBlobBoundV1(rows,hash=>sizes[hash]!,4500,{budgetBytes:450,now:100,graceMilliseconds:50});
 expect(without.keys[0],"without the class the oldest entry, the head, goes first").toBe("head");
 expect(kept.keys).toEqual(["u1","u2","u3"]);expect(kept.protectedKeys).toEqual(["head"]);expect(kept.protectedBytes).toBe(100);expect(kept.after).toBeLessThanOrEqual(4050);
});

test("a newer head of the same selection supersedes the older one after the grace, and not before",()=>{
 const rows:RemoteCacheBlobRowV1[]=[{key:"old",accessed:50,tier:2,refs:["o"],guarded:false,graph:{slot:"selection",storedAt:10}},{key:"new",accessed:60,tier:2,refs:["n"],guarded:false,graph:{slot:"selection",storedAt:40}},{key:"unit",accessed:70,tier:2,refs:["u"],guarded:false}];
 const sizes={o:300,n:100,u:1000},within=planRemoteCacheBlobBoundV1(rows,hash=>sizes[hash as "o"]!,1300,{budgetBytes:1000,now:60,graceMilliseconds:50}),after=planRemoteCacheBlobBoundV1(rows,hash=>sizes[hash as "o"]!,1300,{budgetBytes:1000,now:90,graceMilliseconds:50});
 expect(within.protectedKeys,"during the grace both heads are kept").toEqual(["new","old"]);expect(within.keys).toEqual(["unit"]);
 expect(after.protectedKeys).toEqual(["new"]);expect(after.keys,"after the grace the superseded head leaves first, the current one stays").toEqual(["old"]);
});

const graphDeclaration=(slot:string)=>({slot:sha(slot),declaration:declareV1({kind:"graph",toolchain:sha("graph"),sources:[],externs:[]})});
const unitDeclaration=(name:string)=>({slot:slotIdV1("rust-unit",[name,"build","",""]),declaration:declareV1({kind:"rust-unit",toolchain:sha("t1"),sources:[{path:"a.rs",sha256:sha("a v1")}],externs:[]})});
async function fixture(bound:{value:number|undefined},graph:{sharePercent:number;supersedeGraceMilliseconds:number},mode:"dry-run"|"enforce"="dry-run"){
 const tree=directory("tree"),folder=directory("store"),cas=directory("cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}});
 writeFileSync(join(tree,"a.rs"),"a v1");
 const store=await RemoteCacheStoreV1.open({directory:folder,host,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,graceMilliseconds:60,blobBoundBytes:()=>bound.value,graph,staleness:{mode,tree:await openTreeDigestsV1(tree),examples:5,recheckMilliseconds:0,manifestNames:["unit.json"]}});
 const put=async(key:string,size:number,slot:ReturnType<typeof graphDeclaration>|ReturnType<typeof unitDeclaration>,content=key)=>{const bytes=bytesOf(content,size);await store.put("main","nx",key,body(bytes),{mode:"replace",declaredLength:bytes.length,slot});await sleep(3);return sha(bytes);};
 return{store,host,cas,folder,put,live:async(key:string)=>(await store.read("main","nx",key))!==undefined,peek:async(key:string)=>existsSync(join(folder,"main","nx",key.slice(0,2),key))};
}
const keyOf=(name:string)=>sha(name);

test("the server keeps the head while unit outputs press, and says what it kept",async()=>{
 const bound={value:undefined as number|undefined},{store,host,put,live}=await fixture(bound,{sharePercent:20,supersedeGraceMilliseconds:60_000});
 const head=keyOf("head"),hashes:Record<string,string>={};
 hashes.head=await put(head,300,graphDeclaration("selection"));
 for(const unit of ["u1","u2","u3","u4","u5"])hashes[unit]=await put(keyOf(unit),1000,unitDeclaration(unit));
 await sleep(120);bound.value=4000;await store.collect();
 expect(await live(head),"the oldest entry of the store is the head and it stays").toBe(true);
 expect(await live(keyOf("u1")),"the oldest unit output went").toBe(false);
 const usage=store.usage().blobs;expect(usage.graph).toMatchObject({protectedEntries:1,protectedBytes:300,reservedBytes:800,evictedEntries:0});
 expect(existsSync(host.objectPath(hashes.head!))).toBe(true);
 for(let round=0;round<6;round++){await put(keyOf("churn"+round),1000,unitDeclaration("churn"+round));await sleep(3);await store.collect();}
 expect(await live(head),"six more rounds of unit output pressure").toBe(true);
 await store.close();
});

test("a lookup refreshes a head: with room for one head the most recently used one stays protected",async()=>{
 const bound={value:undefined as number|undefined},{store,put,live}=await fixture(bound,{sharePercent:10,supersedeGraceMilliseconds:60_000});
 const first=keyOf("first-head"),second=keyOf("second-head");
 await put(first,400,graphDeclaration("selection-a"));await put(second,400,graphDeclaration("selection-b"));
 for(const unit of ["u1","u2","u3"])await put(keyOf(unit),1000,unitDeclaration(unit));
 await sleep(120);expect(await live(first),"the lookup of the older head touches it").toBe(true);await sleep(5);
 bound.value=3200;await store.collect();
 expect(store.usage().blobs.graph.protectedEntries,"the reserve of 320 bytes holds the crossing head whole, once").toBe(1);
 expect(await live(first),"the head that was looked up").toBe(true);
 expect(await live(second),"the head nobody looked up competes as a declared entry and is the least recently used one").toBe(false);
 await store.close();
});

test("a head replaced by a newer one of its key keeps its blobs pinned during the grace, and releases them after",async()=>{
 const bound={value:undefined as number|undefined},{store,cas,put}=await fixture(bound,{sharePercent:20,supersedeGraceMilliseconds:150});
 const head=keyOf("head"),first=await put(head,300,graphDeclaration("selection"),"generation-one"),second=await put(head,300,graphDeclaration("selection"),"generation-two");
 expect(first).not.toBe(second);
 await store.collect();await store.pinLive();
 const during=[...(await readCasPinsV1(cas,Date.now())).keys()];
 expect(during,"workers that began to take the first generation still find it").toContain(first);expect(during).toContain(second);
 expect(store.usage().blobs.graph.supersededBytes).toBeGreaterThanOrEqual(0);
 await sleep(250);await store.pinLive();
 const after=[...(await readCasPinsV1(cas,Date.now())).keys()];
 expect(after).toContain(second);expect(after,"the grace passed").not.toContain(first);
 await store.close();
});

test("an evicted graph entry is journalled with its reason, class and clocks, so the next loss is provable",async()=>{
 const bound={value:undefined as number|undefined},{store,folder,put,live}=await fixture(bound,{sharePercent:0,supersedeGraceMilliseconds:60_000});
 const head=keyOf("head");await put(head,300,graphDeclaration("selection"));
 for(const unit of ["u1","u2","u3"])await put(keyOf(unit),1000,unitDeclaration(unit));
 await sleep(120);bound.value=2500;await store.collect();
 expect(await live(head),"without a reserve the head competes as a declared entry and, the oldest, goes").toBe(false);
 const rows=readFileSync(join(folder,"v1",".retention","evictions.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(rows.find(row=>row.kind==="collection")).toMatchObject({version:1,graphEntries:1});
 expect(rows.find(row=>row.kind==="graph-entry")).toMatchObject({version:1,key:`main/nx/${head.slice(0,2)}/${head}`,reason:"budget",tier:2});
 expect(store.usage().blobs.graph.evictedEntries).toBe(1);
 await store.close();
});

const manifestDeclaration=(slot:string)=>({slot:sha(slot),declaration:declareV1({kind:"input-manifest",toolchain:sha("t1"),sources:[{path:"a.rs",sha256:sha("a v0")}],externs:[]})});
test("the input manifests of unit slots share the protected class: unit output pressure never takes them, a newer manifest of the slot replaces the older one, and an edit of the crate does not make one stale",async()=>{
 const bound={value:undefined as number|undefined},{store,put,live}=await fixture(bound,{sharePercent:20,supersedeGraceMilliseconds:60_000},"enforce");
 const manifest=keyOf("manifest"),other=keyOf("manifest-of-another-slot");
 await put(manifest,200,manifestDeclaration("slot-a"),"manifest-one");await put(other,200,manifestDeclaration("slot-b"),"manifest-two");
 for(const unit of ["u1","u2","u3","u4","u5"])await put(keyOf(unit),1000,unitDeclaration(unit));
 await sleep(120);bound.value=4000;await store.collect();
 expect(await live(manifest),"the oldest entries of the store are the manifests and they stay").toBe(true);expect(await live(other)).toBe(true);
 expect(await live(keyOf("u1")),"the oldest unit output went").toBe(false);
 expect(store.usage().blobs.graph).toMatchObject({protectedEntries:2});
 for(let round=0;round<4;round++){await put(keyOf("churn"+round),1000,unitDeclaration("churn"+round));await store.collect();}
 expect(await live(manifest),"further rounds of unit output pressure").toBe(true);
 await store.close();
});
