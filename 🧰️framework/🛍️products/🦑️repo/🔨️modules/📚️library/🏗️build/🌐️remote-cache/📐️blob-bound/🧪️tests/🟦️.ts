import {afterAll,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {CAS_POLICY_V1} from "../../../🗄️cas/🧬️schema/🟦️.ts";
import {createCasStoreV1} from "../../../🗄️cas/🗃️store/🟦️.ts";
import {casReserveBytesV1} from "../../../🗄️cas/🌡️partition/🟦️.ts";
import {casUsageHintPathV1,recordCasMeasurementV1} from "../../../🗄️cas/🧹️collector/🟦️.ts";
import {readCasPinsV1} from "../../../🗄️cas/📌️pins/🟦️.ts";
import {slotIdV1} from "../../../🪢️slots/🟦️.ts";
import {planRemoteCacheBlobBoundV1} from "../../🧹️collector/🟦️.ts";
import {remoteCacheBlobBoundV1} from "../../🖥️server/🟦️.ts";
import {RemoteCacheStoreV1} from "../../💾️store/🟦️.ts";
import {declareV1,openTreeDigestsV1} from "../../🏷️retention/🍂️staleness/🟦️.ts";

const python=testPythonV1(),scratch:string[]=[];
const directory=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`rcache-blob-${label}-`));scratch.push(path);return path;};
const sha=(value:Uint8Array|string)=>createHash("sha256").update(value).digest("hex");
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const body=(bytes:Uint8Array)=>(async function*(){yield bytes;})();
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
function seeded(seed:number):()=>number{return ()=>{seed=(seed+0x6d2b79f5)|0;let value=Math.imul(seed^(seed>>>15),1|seed);value^=value+Math.imul(value^(value>>>7),61|value);return ((value^(value>>>14))>>>0)/4294967296;};}

const oracle=`
import json,sys
out=[]
for request in json.load(sys.stdin):
    rows=request["rows"];sizes=request["sizes"];bound=request["bound"]
    holders={}
    for row in rows:
        for h in set(row["refs"]):holders[h]=holders.get(h,0)+1
    before=sum(sizes[h] for h in holders)
    if before<=bound:
        out.append({"before":before,"after":before,"keys":[]});continue
    target=bound*9//10;after=before;keys=[]
    order=sorted([r for r in rows if not r["guarded"] and r["refs"]],key=lambda r:(r["tier"],r["accessed"],r["key"]))
    for row in order:
        if after<=target:break
        for h in set(row["refs"]):
            holders[h]-=1
            if holders[h]==0:after-=sizes[h]
        keys.append(row["key"])
    out.append({"before":before,"after":after,"keys":keys})
print(json.dumps(out))
`;

test.skipIf(!python)("the blob bound plan equals an independent Python model on 300 seeded stores with shared blobs, tiers, guards and ties",()=>{
 const next=seeded(20261123),requests:{rows:{key:string;accessed:number;tier:number;refs:string[];guarded:boolean}[];sizes:Record<string,number>;bound:number}[]=[];
 for(let round=0;round<300;round++){
  const blobs=Array.from({length:1+Math.floor(next()*25)},(_,index)=>`b${index}`),sizes=Object.fromEntries(blobs.map(blob=>[blob,1+Math.floor(next()*900)])),rows=[];
  for(let index=0;index<Math.floor(next()*40);index++)rows.push({key:`main/nx/${String(index).padStart(2,"0")}/k${index}`,accessed:Math.floor(next()*(round%3===0?4:500)),tier:Math.floor(next()*3),refs:Array.from({length:Math.floor(next()*4)},()=>blobs[Math.floor(next()*blobs.length)]!),guarded:next()<0.1});
  const total=Object.values(sizes).reduce((sum,size)=>sum+size,0);
  requests.push({rows,sizes,bound:Math.max(1,Math.floor(total*(0.2+next()*1.1)))});
 }
 const result=Bun.spawnSync([python!,"-I","-c",oracle],{stdin:new TextEncoder().encode(JSON.stringify(requests)),stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const model=JSON.parse(result.stdout.toString()) as {before:number;after:number;keys:string[]}[];let evicting=0;
 requests.forEach((request,index)=>{
  const ours=planRemoteCacheBlobBoundV1(request.rows,hash=>request.sizes[hash]!,request.bound);
  expect({before:ours.before,after:ours.after,keys:[...ours.keys]},`round ${index}`).toEqual(model[index]!);
  if(ours.keys.length>0)evicting++;
  for(const key of ours.keys)expect(request.rows.find(row=>row.key===key)!.guarded,"a guarded entry is never taken").toBe(false);
 });
 expect(evicting).toBeGreaterThan(100);
});

test("the server bound is a share of the host store's cap in force and follows the cap",async()=>{
 const root=directory("cap"),small={root,policy:{...CAS_POLICY_V1,capBytes:2**30}},large={root,policy:{...CAS_POLICY_V1,capBytes:2**31}};
 expect(await remoteCacheBlobBoundV1(small,50)).toBe(2**29);
 expect(await remoteCacheBlobBoundV1(large,50)).toBe(2**30);
 expect(await remoteCacheBlobBoundV1(large,25)).toBe(2**29);
 expect(await remoteCacheBlobBoundV1(large,undefined)).toBeUndefined();
});

test("the server bound shrinks as the free space nears the hard reserve and is judged on the newest measurement, not on the last collection",async()=>{
 const root=directory("free"),host={root,policy:CAS_POLICY_V1},total=926*2**30,reserve=casReserveBytesV1(CAS_POLICY_V1,{totalBytes:total,role:"cache-host"}),at=(free:number)=>()=>({partition:"/volume",totalBytes:total,freeBytes:free,role:"cache-host" as const});
 mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:60*2**30,atMs:Date.now()-3600000}));
 const stale=await remoteCacheBlobBoundV1(host,50,at(reserve+20*2**30));
 expect(await recordCasMeasurementV1(root,100*2**30,Date.now())).toBe(true);
 const fresh=await remoteCacheBlobBoundV1(host,50,at(reserve+20*2**30)),tight=await remoteCacheBlobBoundV1(host,50,at(reserve+2*2**30)),roomy=await remoteCacheBlobBoundV1(host,50,at(reserve+80*2**30));
 expect(stale).toBeLessThan(fresh!);
 expect(tight).toBeLessThan(fresh!);
 expect(tight).toBeLessThanOrEqual(Math.floor((100*2**30+2*2**30)/2));
 expect(roomy).toBeGreaterThan(fresh!);
});

const bytesOf=(name:string,size:number)=>new TextEncoder().encode(name.repeat(Math.ceil(size/name.length)).slice(0,size));
async function fixture(mode:"dry-run"|"enforce",bound:{value:number|undefined}){
 const tree=directory("tree"),folder=directory("store"),cas=directory("cas"),host=createCasStoreV1({root:cas,policy:{...CAS_POLICY_V1,minAgeMs:0,partition:{lowUsedFraction:0.98,highUsedFraction:0.99,criticalUsedFraction:0.999}}});
 writeFileSync(join(tree,"a.rs"),"a v1");
 const store=await RemoteCacheStoreV1.open({directory:folder,host,maximumBytes:2**30,maximumEntryBytes:2**22,maximumAgeMilliseconds:0,graceMilliseconds:60,blobBoundBytes:()=>bound.value,staleness:{mode,tree:await openTreeDigestsV1(tree),examples:5,recheckMilliseconds:0,manifestNames:["unit.json"]}});
 const declared=(name:string,source:string)=>({slot:slotIdV1("rust-unit",[name,"build","",""]),declaration:declareV1({kind:"rust-unit",toolchain:sha("t1"),sources:[{path:"a.rs",sha256:sha(source)}],externs:[]})});
 const put=async(key:string,size:number,slot?:ReturnType<typeof declared>)=>{const bytes=bytesOf(key,size);await store.put("main","nx",key,body(bytes),{mode:"keep",declaredLength:bytes.length,...slot===undefined?{}:{slot}});await sleep(3);return sha(bytes);};
 return {store,host,cas,declared,put,live:async(key:string)=>(await store.read("main","nx",key))!==undefined};
}

for(const mode of ["dry-run","enforce"] as const)test(`${mode}: over the bound the store evicts undeclared entries oldest first, then declared ones, unpins their blobs and accepts the next write`,async()=>{
 const bound={value:undefined as number|undefined},{store,host,cas,declared,put,live}=await fixture(mode,bound);
 const hashes:Record<string,string>={};
 for(const key of ["u1","u2","u3"])hashes[key]=await put(key,1000);
 for(const key of ["d1","d2"])hashes[key]=await put(key,1000,declared(key,"a v1"));
 await sleep(120);
 await store.collect();expect(store.usage().blobs.boundBytes,"no bound, nothing evicted").toBeNull();
 for(const key of ["u1","u2","u3","d1","d2"])expect(await live(key)).toBe(true);
 const pinnedBefore=await store.pinLive();expect(pinnedBefore).toBe(2);
 bound.value=4500;await store.collect();
 expect(store.usage().blobs).toMatchObject({boundBytes:4500});
 expect(await live("u1"),"the oldest undeclared entry went first").toBe(false);expect(await live("u2"),"down to nine tenths of the bound only").toBe(true);
 expect(await live("u3")).toBe(true);expect(await live("d1")).toBe(true);expect(await live("d2")).toBe(true);
 expect(existsSync(host.objectPath(hashes.u1!)),"its blob left the host store").toBe(false);
 expect(store.usage().blobs.referencedBytes!).toBeLessThanOrEqual(4050);
 bound.value=1500;await store.collect();
 expect(await live("u2")).toBe(false);expect(await live("u3")).toBe(false);expect(await live("d1"),"declared entries go after the undeclared ones, least recently used first").toBe(false);expect(await live("d2")).toBe(true);
 expect(await store.pinLive(),"the pinned set shrinks with the evicted entries").toBe(1);
 expect([...(await readCasPinsV1(cas,Date.now())).keys()]).toEqual([hashes.d2!]);
 expect(await put("fresh",1000),"the next write is accepted").toBe(sha(bytesOf("fresh",1000)));
 expect(await live("fresh")).toBe(true);
 expect(store.usage().blobs.evictedBytes).toBeGreaterThanOrEqual(4000);expect(store.usage().blobs.referencedBytes!).toBeLessThanOrEqual(1350);
 await store.close();
});
