import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {cargoScratchBaseV1} from "../../../🧪️tests/🏖️scratch/🟦️.ts";
import {publishUnitV1} from "../../../💾️store/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../📡️client/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";
import {readBuildCountersV1} from "../../📊️counters/🟦️.ts";
import {drainUploadsV1,enqueueUploadV1,retryJobV1,spoolBoundV1,spoolLengthV1,type SpoolJobV1} from "../🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import {fleetCacheLocalPathV1} from "../../../../../../🏗️build/🌐️remote-cache/🛰️fleet/🟦️.ts";
import {keyGapV1,registryCrateDirectoryV1} from "../../../🔏️key/🔗️sources/🟦️.ts";

/** 🔁️ A unit the fleet cache did not take is queued again with a growing delay and given up after the declared attempts; a unit another machine stored meanwhile is not uploaded twice. */
const fleet=FLEET_POLICY_V1,base=cargoScratchBaseV1(),scratch:string[]=[],servers:RemoteCacheServerV1[]=[],proxies:{stop(force?:boolean):void}[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(base,"up-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const proxy of proxies)proxy.stop(true);for(const server of servers)await server.stop().catch(()=>{});if(!process.env.SEMIO_TEST_KEEP)for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const platform={version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""} as const;
const job=(overrides:Partial<SpoolJobV1>={}):SpoolJobV1=>({version:1,key:"4".repeat(64),localKey:"5".repeat(64),slot:"2".repeat(64),crate:"core",cwd:"/w",workspace:null,laneRoot:null,owner:"t:1:aa",claimed:true,costMs:1,platform,queuedMs:1000,...overrides});

test("a failed upload is queued again without its claim, after a delay that doubles up to the declared maximum, until the job is too old (no count of tries ends it); the job validates against the schema",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);const valid=ajv.getSchema(`${schema.$id}#/definitions/SpoolJob`)!;
 const first=retryJobV1(job(),2000)!;
 expect(first.attempts).toBe(1);expect(first.claimed).toBe(false);expect(first.offer).toBe(true);expect(first.notBeforeMs).toBe(2000+fleet.uploader.retryBackoffMilliseconds);expect(valid(first)).toBe(true);
 const second=retryJobV1(first,5000)!;expect(second.attempts).toBe(2);expect(second.notBeforeMs).toBe(5000+2*fleet.uploader.retryBackoffMilliseconds);
 const third=retryJobV1(second,9000)!;expect(third.notBeforeMs).toBe(9000+4*fleet.uploader.retryBackoffMilliseconds);
 const late=retryJobV1({...third,attempts:40},10000)!;expect(late.notBeforeMs,"the delay stops growing").toBe(10000+fleet.uploader.maximumBackoffMilliseconds);expect(late.attempts).toBe(41);
 expect(retryJobV1(job(),1000+fleet.uploader.maximumJobAgeMilliseconds+1),"a job older than the declared age is given up").toBeUndefined();
 expect(valid({...job(),attempts:-1})).toBe(false);expect(valid({...job(),extra:1})).toBe(false);
});

test("a cache that refuses the upload (disk full on the server: PUT 500) gets the unit again once it takes it; a job that is not yet due is left alone; a refusal for good (413) gives the unit up at once, counted once as refused",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 let failing=true,refusals=0,refuse=500;
 const proxy=Bun.serve({port:0,async fetch(request){
  if(request.method==="PUT"&&failing){refusals++;return new Response("refused process.disk.low-space",{status:refuse});}
  return fetch(server.url+new URL(request.url).pathname+new URL(request.url).search,{method:request.method,headers:request.headers,body:request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer()});
 }});proxies.push(proxy);
 const cas=directory("cas"),root=directory("cache"),out=directory("out"),state=join(root,fleet.layout.state,fleet.layout.spool);
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${proxy.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\n");
 const localKey="6".repeat(64),entry=publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined});
 expect(entry).toBeDefined();
 const key="7".repeat(64);
 expect(enqueueUploadV1(root,job({key,localKey,cwd:out,queuedMs:Date.now()}))).toBe(true);
 const client=createFleetClientV1(readFleetSettingsV1(root,environment)!);
 const spooled=()=>readdirSync(state).filter(name=>name.endsWith(".json")).map(name=>({name,job:JSON.parse(readFileSync(join(state,name),"utf8")) as SpoolJobV1}));
 const first=await drainUploadsV1(root,environment,{idleMilliseconds:0});
 expect(first).toMatchObject({uploaded:0,failed:0,retried:1});expect(refusals).toBeGreaterThan(0);
 expect(spooled().length).toBe(1);expect(spooled()[0]!.job.attempts).toBe(1);expect(spooled()[0]!.job.notBeforeMs!).toBeGreaterThan(Date.now());expect(spooled()[0]!.job.claimed).toBe(false);
 const before=refusals;
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0}),"a job that is not due is left alone").toMatchObject({uploaded:0,failed:0});expect(refusals).toBe(before);expect(spoolLengthV1(root)).toBe(1);
 const due=(attempts:number)=>{const row=spooled()[0]!;writeFileSync(join(state,row.name),JSON.stringify({...row.job,attempts,notBeforeMs:0}));};
 due(1);failing=false;
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:1,failed:0});
 expect(spoolLengthV1(root)).toBe(0);expect(await client.hasResult(key)).toBe(true);
 failing=true;const lost="8".repeat(64);
 expect(enqueueUploadV1(root,job({key:lost,localKey,cwd:out,queuedMs:Date.now(),attempts:30}))).toBe(true);
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0}),"a fault that can pass never ends a job by its count").toMatchObject({uploaded:0,failed:0,retried:1});
 expect(spoolLengthV1(root)).toBe(1);
 refuse=413;due(31);
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:0,failed:1});
 expect(spoolLengthV1(root),"refused for good, not queued again").toBe(0);
},120000);

test("while the cache is unreachable a queued unit waits untouched and is not counted as skipped; it is uploaded once the cache answers, and only a job past its age is given up, counted as expired",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv2"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const cas=directory("cas2"),root=directory("cache2"),out=directory("out2"),workspace=directory("work2");
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\n");
 const localKey="9".repeat(64),key="a".repeat(64);
 expect(publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined})).toBeDefined();
 expect(enqueueUploadV1(root,job({key,localKey,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
 const downFile=join(root,fleet.layout.state,fleet.layout.down);mkdirSync(join(root,fleet.layout.state),{recursive:true});
 writeFileSync(downFile,Array.from({length:fleet.breaker.downFaults},()=>String(Date.now())).join("\n")+"\n");
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:0,failed:0,dropped:0,deferred:1});
 expect(spoolLengthV1(root),"the job waits").toBe(1);
 expect(readBuildCountersV1(workspace).unit.skipped,"waiting is no skip").toBe(0);
 const waiting=join(root,fleet.layout.state,fleet.layout.spool),row=readdirSync(waiting).filter(name=>name.endsWith(".json"))[0]!,held=JSON.parse(readFileSync(join(waiting,row),"utf8")) as SpoolJobV1;
 expect(held.notBeforeMs!,"it is due again after the retry delay").toBeGreaterThan(Date.now());expect(held.attempts,"waiting uses no attempt").toBeUndefined();
 writeFileSync(join(waiting,row),JSON.stringify({...held,notBeforeMs:0}));rmSync(downFile,{force:true});
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:1,failed:0});
 expect(spoolLengthV1(root)).toBe(0);expect(await createFleetClientV1(readFleetSettingsV1(root,environment)!).hasResult(key)).toBe(true);
 expect(enqueueUploadV1(root,job({key:"b".repeat(64),localKey,cwd:out,queuedMs:Date.now()-fleet.uploader.maximumJobAgeMilliseconds-1,claimed:false}))).toBe(true);
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:0,dropped:1});
 expect(readBuildCountersV1(workspace).skipReasons).toEqual({"upload-expired":1});
},120000);

test("a job put off while the credential was denied waits for a drainer of that credential but is taken at once by a drainer with another credential, not after the retry delay",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv-cred"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const cas=directory("cas-cred"),root=directory("cache-cred"),out=directory("out-cred"),right={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas},wrong={...right,SEMIO_REMOTE_CACHE_TOKEN:"wrong-token"};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\n");
 const localKey="8".repeat(64),key="c".repeat(64),denied=readFleetSettingsV1(root,wrong)!.identity;
 expect(publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined})).toBeDefined();
 expect(enqueueUploadV1(root,job({key,localKey,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
 mkdirSync(join(root,fleet.layout.state),{recursive:true});writeFileSync(join(root,fleet.layout.state,fleet.layout.denied),denied);
 expect(await drainUploadsV1(root,wrong,{idleMilliseconds:0})).toMatchObject({uploaded:0,failed:0,dropped:0,deferred:1});
 const waiting=join(root,fleet.layout.state,fleet.layout.spool),held=JSON.parse(readFileSync(join(waiting,readdirSync(waiting).find(name=>name.endsWith(".json"))!),"utf8")) as SpoolJobV1;
 const ajv=new Ajv({strict:false});ajv.addSchema(schema);expect(ajv.getSchema(`${schema.$id}#/definitions/SpoolJob`)!(held)).toBe(true);
 expect(held.deferredBy).toBe(denied);expect(held.notBeforeMs!,"put off by the retry delay").toBeGreaterThan(Date.now());
 expect(await drainUploadsV1(root,wrong,{idleMilliseconds:0}),"the same credential leaves the job until it is due").toMatchObject({uploaded:0,failed:0,dropped:0});
 expect(spoolLengthV1(root)).toBe(1);
 expect(await drainUploadsV1(root,right,{idleMilliseconds:0}),"another credential takes it at once").toMatchObject({uploaded:1,failed:0});
 expect(spoolLengthV1(root)).toBe(0);expect(await createFleetClientV1(readFleetSettingsV1(root,right)!).hasResult(key)).toBe(true);
},120000);

test("offers of local hits fill at most half of the spool, so a unit compiled meanwhile is still queued; at the full spool a compiled unit is refused; the bound follows the free room of the disk between the declared minimum and maximum",()=>{
 const root=directory("cap"),bound=64,half=bound/2;
 const key=(index:number)=>index.toString(16).padStart(16,"0").padEnd(64,"0");
 for(let index=0;index<half;index++)expect(enqueueUploadV1(root,job({key:key(index),offer:true,claimed:false,queuedMs:1000+index}),bound)).toBe(true);
 expect(enqueueUploadV1(root,job({key:key(half),offer:true,claimed:false}),bound)).toBe(false);
 for(let index=half;index<bound;index++)expect(enqueueUploadV1(root,job({key:key(index+1),queuedMs:5000+index}),bound)).toBe(true);
 expect(spoolLengthV1(root)).toBe(bound);
 expect(enqueueUploadV1(root,job({key:key(bound+2),queuedMs:9999}),bound)).toBe(false);
 const measured=spoolBoundV1(root);expect(measured).toBeGreaterThanOrEqual(fleet.spool.minimumEntries);expect(measured).toBeLessThanOrEqual(fleet.spool.maximumEntries);
});

test("the cache is full or busy for a while and 20 spooled units are all still there; once it takes them every one arrives, none was counted as failed; a full spool gives room again after the drain",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv4"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 let status=507;
 const proxy=Bun.serve({port:0,async fetch(request){
  if(request.method==="PUT"&&status!==200)return new Response("later",{status});
  return fetch(server.url+new URL(request.url).pathname+new URL(request.url).search,{method:request.method,headers:request.headers,body:request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer()});
 }});proxies.push(proxy);
 const cas=directory("cas4"),root=directory("cache4"),out=directory("out4"),workspace=directory("work4"),state=join(root,fleet.layout.state,fleet.layout.spool);
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${proxy.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\n");
 const localKey="d".repeat(64);publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined});
 const keys=Array.from({length:20},(_,index)=>(index+1).toString(16).padStart(16,"e").padEnd(64,"f")),bound=20;
 for(const [index,key] of keys.entries())expect(enqueueUploadV1(root,job({key,localKey,cwd:out,queuedMs:Date.now()+index,claimed:false}),bound)).toBe(true);
 expect(enqueueUploadV1(root,job({key:"1".repeat(64),localKey,cwd:out,queuedMs:Date.now()+99,claimed:false}),bound),"the spool is at its bound").toBe(false);
 const due=()=>{for(const name of readdirSync(state).filter(name=>name.endsWith(".json"))){const row=JSON.parse(readFileSync(join(state,name),"utf8")) as SpoolJobV1;writeFileSync(join(state,name),JSON.stringify({...row,notBeforeMs:0}));}};
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:0,failed:0,retried:20});
 expect(spoolLengthV1(root),"full: every unit waits").toBe(20);
 status=503;due();
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:0,failed:0,retried:20});
 expect(spoolLengthV1(root),"busy: every unit waits").toBe(20);
 status=200;due();
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:20,failed:0,dropped:0});
 expect(spoolLengthV1(root)).toBe(0);
 const client=createFleetClientV1(readFleetSettingsV1(root,environment)!);
 for(const key of keys)expect(await client.hasResult(key)).toBe(true);
 expect(readBuildCountersV1(workspace).skipReasons,"nothing was lost, nothing counted").toEqual({});
 expect(enqueueUploadV1(root,job({key:"2".repeat(64),localKey,cwd:out,queuedMs:Date.now(),claimed:false}),bound),"room again").toBe(true);
},180000);

test("one stalled upload does not stall the others: the drain runs a sliding pool, so units behind a slow one are uploaded while it hangs",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv3"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const slow="a".repeat(64),stallMs=2500;let finishedWhileStalled=0,stalled=false;
 const proxy=Bun.serve({port:0,async fetch(request){
  const url=new URL(request.url),body=request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer();
  const isResult=request.method==="PUT"&&url.pathname.includes("/ac/");
  if(isResult&&url.pathname.includes(slow)){stalled=true;await new Promise(done=>setTimeout(done,stallMs));stalled=false;}
  const answer=await fetch(server.url+url.pathname+url.search,{method:request.method,headers:request.headers,body});
  if(isResult&&stalled&&!url.pathname.includes(slow))finishedWhileStalled++;
  return answer;
 }});proxies.push(proxy);
 const cas=directory("cas3"),root=directory("cache3"),out=directory("out3");
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${proxy.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\n");
 const localKey="9".repeat(64);publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined});
 expect(enqueueUploadV1(root,job({key:slow,localKey,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
 for(let index=1;index<=7;index++)expect(enqueueUploadV1(root,job({key:index.toString(16).padStart(16,"b").padEnd(64,"c"),localKey,cwd:out,queuedMs:Date.now()+index,claimed:false}))).toBe(true);
 const started=Date.now(),result=await drainUploadsV1(root,environment,{idleMilliseconds:0});
 expect(result).toMatchObject({uploaded:8,failed:0});expect(finishedWhileStalled,"all seven others finished while the slow one hung").toBe(7);expect(Date.now()-started).toBeGreaterThanOrEqual(stallMs-100);
},120000);

test("a process that names the endpoint but not the access (or names it empty) gets the access of the fleet facts, read-only only without facts: it is no longer silently read-only",()=>{
 const workspace=directory("facts"),root=directory("facts-cache");
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:1",SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace};
 expect(readFleetSettingsV1(root,{...environment,SEMIO_REMOTE_CACHE_ACCESS:""})!.upload,"no facts, empty access").toBe(false);
 const path=fleetCacheLocalPathV1(workspace);mkdirSync(join(path,".."),{recursive:true});
 writeFileSync(path,JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:"http://127.0.0.1:1",facts:{version:1,group:"g1",generation:1,publishedAtMs:Date.now(),publisher:{directory:"/x",port:1,host:"h"},cache:{endpoints:["http://127.0.0.1:1"],instance:"main",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1,transport:"trusted-lan"},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]}}}));
 expect(readFleetSettingsV1(root,{...environment,SEMIO_REMOTE_CACHE_ACCESS:""})!.upload,"empty access, facts grant read-write").toBe(true);
 expect(readFleetSettingsV1(root,environment)!.upload).toBe(true);
 expect(readFleetSettingsV1(root,{...environment,SEMIO_REMOTE_CACHE_ACCESS:"read-only"})!.upload,"an explicit access wins").toBe(false);
});

test("the gap of a unit is derivable more often: a file below an unpacked registry crate, and the compiler resource root, are no gap; a manifest is the part that a derive's file makes of it (see the manifest test)",()=>{
 const info=(dependencies:string[],environment:string[]=[])=>`out: ${dependencies.join(" ")}\n`+environment.map(name=>`# env-dep:${name}=x\n`).join("");
 const crate="/h/.cargo/registry/src/index.crates.io-1/serde_core-1.0.229";
 expect(registryCrateDirectoryV1(crate)).toBe(true);expect(registryCrateDirectoryV1("/w/crates/core")).toBe(false);
 const covered=new Set([crate+"/src/lib.rs"]);
 expect(keyGapV1({cwd:"/w",covered,keyedNames:new Set(),depInfo:info([crate+"/src/lib.rs",crate+"/src/de/mod.rs"]),crateDirectory:crate})).toEqual([]);
 expect(keyGapV1({cwd:"/w",covered,keyedNames:new Set(),depInfo:info([crate+"/src/lib.rs",crate+"/src/de/mod.rs"])})).toEqual([crate+"/src/de/mod.rs"]);
 expect(keyGapV1({cwd:"/w",covered,keyedNames:new Set(),depInfo:info([crate+"/src/lib.rs"],["SEMIO_COMPILER_RESOURCE_ROOT"]),crateDirectory:crate})).toEqual([]);
 expect(keyGapV1({cwd:"/w",covered,keyedNames:new Set(),depInfo:info([crate+"/src/lib.rs"],["SOMETHING_ELSE"]),crateDirectory:crate})).toEqual(["env:SOMETHING_ELSE"]);
});
