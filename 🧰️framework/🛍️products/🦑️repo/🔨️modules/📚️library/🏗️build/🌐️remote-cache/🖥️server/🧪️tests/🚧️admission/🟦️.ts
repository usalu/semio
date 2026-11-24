import {afterAll,expect,test} from "bun:test";
import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {casUsageHintPathV1} from "../../../../🗄️cas/🧹️collector/🟦️.ts";
import {resetCasGuardsV1} from "../../../../🗄️cas/🚦️guard/🟦️.ts";
import {readCasTransientsV1,CAS_LEASE_HEADER_V1} from "../../../../🗄️cas/📌️pins/🟦️.ts";
import {createCasStoreV1} from "../../../../🗄️cas/🗃️store/🟦️.ts";
import {CAS_POLICY_V1,type CasPolicyV1} from "../../../../🗄️cas/🧬️schema/🟦️.ts";
import {startRemoteCacheServerV1} from "../../🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MIB=1024*1024,sha=(bytes:Uint8Array):string=>createHash("sha256").update(bytes).digest("hex");

test("a 507 names the bound and the largest holders, an input upload is admitted over the cap, ends its lease on DELETE, and a bare DELETE is refused",async()=>{
 resetCasGuardsV1();
 const base=mkdtempSync(join(tmpdir(),"rcache-admission-"));scratch.push(base);
 const policy:CasPolicyV1={...CAS_POLICY_V1,capBytes:4*MIB,minimumCapBytes:1,minAgeMs:0,admissionBudgetMs:200,transientGraceMs:1000},root=join(base,"cas"),host=createCasStoreV1({root,policy,refuseOverCap:true});
 const held=[randomBytes(MIB),randomBytes(MIB),randomBytes(MIB),randomBytes(MIB),randomBytes(MIB)],digests:string[]=[];
 for(const bytes of held)digests.push((await host.put(bytes)).hash);
 await host.pin("test-holder",digests,600000);
 mkdirSync(join(root,"v1"),{recursive:true});writeFileSync(casUsageHintPathV1(root),JSON.stringify({version:1,bytes:5*MIB,atMs:Date.now()}));
 const server=await startRemoteCacheServerV1({directory:join(base,"dir"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"t"},validation:"off",hostStore:host});
 const auth={authorization:"Bearer t"},put=(bytes:Uint8Array,extra:Record<string,string>={})=>fetch(`${server.url}/main/cas/${sha(bytes)}`,{method:"PUT",headers:{...auth,...extra},body:bytes});
 try{
  const plain=randomBytes(MIB),refused=await put(plain),text=await refused.text();
  expect(refused.status).toBe(507);expect(text).toContain("Insufficient storage");expect(text).toContain("cap in force");expect(text).toContain("largest holders");expect(text).toContain("Grenze");
  const input=randomBytes(MIB),admitted=await put(input,{[CAS_LEASE_HEADER_V1]:"input:action-one"});
  expect(admitted.status).toBe(200);await admitted.arrayBuffer();
  const key=sha(input),leased=(await readCasTransientsV1(root)).get(key);expect(leased).toBeGreaterThan(Date.now()+policy.requesterLeaseMs/2);expect(leased!).toBeLessThanOrEqual(Date.now()+policy.requesterLeaseMs);
  const released=await fetch(`${server.url}/main/cas/${key}`,{method:"DELETE",headers:{...auth,[CAS_LEASE_HEADER_V1]:"input:action-one"}});
  expect(released.status).toBe(200);await released.arrayBuffer();
  expect((await readCasTransientsV1(root)).get(key)!).toBeLessThan(Date.now()+policy.transientGraceMs+2000);
  expect((await fetch(`${server.url}/main/cas/${key}`,{method:"HEAD",headers:auth})).status).toBe(200);
  const forbidden=await fetch(`${server.url}/main/cas/${key}`,{method:"DELETE",headers:{[CAS_LEASE_HEADER_V1]:"input"}});expect([401,403]).toContain(forbidden.status);await forbidden.arrayBuffer();
  const bare=await fetch(`${server.url}/main/cas/${key}`,{method:"DELETE",headers:auth});expect(bare.status).toBe(405);expect(bare.headers.get("allow")).toBe("GET, HEAD, PUT, DELETE");await bare.arrayBuffer();
 }finally{await server.stop();}
},60000);

test("parallel reads of blobs are not serialised behind the bookkeeping of transient input leases: 64 simultaneous GETs next to hundreds of leased uploads all answer 200 within a few hundred milliseconds each",async()=>{
 resetCasGuardsV1();
 const base=mkdtempSync(join(tmpdir(),"rcache-parallel-"));scratch.push(base);
 const policy:CasPolicyV1={...CAS_POLICY_V1,minAgeMs:0},root=join(base,"cas"),host=createCasStoreV1({root,policy,refuseOverCap:true});
 const server=await startRemoteCacheServerV1({directory:join(base,"dir"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"t"},validation:"off",hostStore:host});
 const auth={authorization:"Bearer t"},blobs=Array.from({length:300},()=>randomBytes(16384));
 try{
  for(let index=0;index<blobs.length;index+=30)await Promise.all(blobs.slice(index,index+30).map((bytes,offset)=>fetch(`${server.url}/main/cas/${sha(bytes)}`,{method:"PUT",headers:{...auth,[CAS_LEASE_HEADER_V1]:`input:action-${index+offset}`},body:bytes}).then(response=>response.arrayBuffer())));
  const started=performance.now(),times=await Promise.all(blobs.slice(0,64).map(async bytes=>{const begin=performance.now(),response=await fetch(`${server.url}/main/cas/${sha(bytes)}`,{headers:auth});expect(response.status).toBe(200);expect((await response.arrayBuffer()).byteLength).toBe(bytes.length);return performance.now()-begin;}));
  expect(Math.max(...times)).toBeLessThan(1500);expect(performance.now()-started).toBeLessThan(3000);
 }finally{await server.stop();}
},60000);

test("an existence check under a lease writes the lease before it answers, for an absent blob too, and both it and an upload answer 503 when the lease cannot be written",async()=>{
 resetCasGuardsV1();
 const base=mkdtempSync(join(tmpdir(),"rcache-lease-first-"));scratch.push(base);
 const policy:CasPolicyV1={...CAS_POLICY_V1,minimumCapBytes:1,minAgeMs:0},root=join(base,"cas"),host=createCasStoreV1({root,policy,refuseOverCap:true});
 const open=(store:typeof host,name:string)=>startRemoteCacheServerV1({directory:join(base,name),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"t"},validation:"off",hostStore:store});
 const auth={authorization:"Bearer t"},lease={[CAS_LEASE_HEADER_V1]:"input:op-live"};
 const server=await open(host,"dir");
 try{
  const bytes=randomBytes(1000),key=sha(bytes),absent=sha(randomBytes(32));
  expect((await fetch(`${server.url}/main/cas/${key}`,{method:"PUT",headers:auth,body:bytes})).status).toBe(200);
  expect((await fetch(`${server.url}/main/cas/${key}`,{method:"HEAD",headers:{...auth,...lease}})).status).toBe(200);
  expect((await fetch(`${server.url}/main/cas/${absent}`,{method:"HEAD",headers:{...auth,...lease}})).status).toBe(404);
  const leases=await readCasTransientsV1(root);expect(leases.get(key)).toBeGreaterThan(Date.now());expect(leases.get(absent),"leased before the answer, so a blob that appears next is protected from its first moment").toBeGreaterThan(Date.now());
 }finally{await server.stop();}
 const broken=await open({...host,lease:async()=>{throw Error("disk full");}} as typeof host,"broken");
 try{
  const bytes=randomBytes(500),key=sha(bytes);
  const head=await fetch(`${broken.url}/main/cas/${key}`,{method:"HEAD",headers:{...auth,...lease}});expect(head.status).toBe(503);await head.arrayBuffer();
  const put=await fetch(`${broken.url}/main/cas/${key}`,{method:"PUT",headers:{...auth,...lease},body:bytes});expect(put.status).toBe(503);await put.arrayBuffer();
 }finally{await broken.stop();}
},60000);
