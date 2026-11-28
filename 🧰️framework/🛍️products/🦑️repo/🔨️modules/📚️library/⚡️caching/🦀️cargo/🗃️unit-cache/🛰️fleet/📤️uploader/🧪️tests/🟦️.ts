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
import {drainUploadsV1,enqueueUploadV1,retryJobV1,startUploaderV1,type SpoolJobV1} from "../🟦️.ts";
import {spoolBoundV1,spoolLengthV1} from "../../📏️spool-size/🟦️.ts";
import {claimHeldV1} from "../../../🔒️lock/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json" with {type:"json"};
import policy from "../../../../../../🏗️build/🔣️policy.json" with {type:"json"};
import {readCasPinsV1} from "../../../../../../🏗️build/🗄️cas/📌️pins/🟦️.ts";
import {collectCasV1} from "../../../../../../🏗️build/🗄️cas/🧹️collector/🟦️.ts";
import {CAS_POLICY_V1} from "../../../../../../🏗️build/🗄️cas/🧬️schema/🟦️.ts";
import {existsSync as present} from "node:fs";
const pins=(root:string):Set<string>=>{try{return new Set((readdirSync(join(root,"v1","p")).filter(name=>name.endsWith(".json")).flatMap(name=>(JSON.parse(readFileSync(join(root,"v1","p",name),"utf8")) as {digests:string[]}).digests)));}catch{return new Set();}};
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
 expect(held.notBeforeMs,"an outage rewrites no job: the drainer waits as a whole").toBeUndefined();expect(held.attempts,"waiting uses no attempt").toBeUndefined();
 const before=readFileSync(join(waiting,row),"utf8"),stop=new AbortController();setTimeout(()=>stop.abort(),1500);
 await drainUploadsV1(root,environment,{idleMilliseconds:60000,signal:stop.signal});
 expect(readFileSync(join(waiting,row),"utf8"),"a long outage leaves the job file untouched").toBe(before);
 rmSync(downFile,{force:true});
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

test("a build with another credential hands the spool over even within the spawn throttle of the last start: the drainer of the old credential, which lives as long as a job waits, ends and a successor uploads the job",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv-hand"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const cas=directory("cas-hand"),root=directory("cache-hand"),out=directory("out-hand"),state=join(root,fleet.layout.state),right={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas},wrong={...right,SEMIO_REMOTE_CACHE_TOKEN:"wrong-token"};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\n");
 const localKey="d".repeat(64),key="e".repeat(64),denied=readFleetSettingsV1(root,wrong)!.identity,successor=readFleetSettingsV1(root,right)!.identity;
 expect(publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined})).toBeDefined();
 expect(enqueueUploadV1(root,job({key,localKey,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
 mkdirSync(state,{recursive:true});writeFileSync(join(state,fleet.layout.denied),denied);writeFileSync(join(state,fleet.layout.uploaderIdentity),denied);writeFileSync(join(state,fleet.layout.spawned),denied);
 const old=drainUploadsV1(root,wrong,{idleMilliseconds:fleet.uploader.maximumJobAgeMilliseconds});
 while(!claimHeldV1(state,"uploader"))await new Promise(done=>setTimeout(done,20));
 expect(startUploaderV1(root,{...process.env,...right,SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS:"300"}),"the start for the new credential is not throttled by the start of the old one").toBe(true);
 expect(readFileSync(join(state,fleet.layout.uploaderIdentity),"utf8")).toBe(successor);expect(readFileSync(join(state,fleet.layout.spawned),"utf8")).toBe(successor);
 expect(startUploaderV1(root,{...process.env,...right}),"a storm of the same credential starts one successor").toBe(false);
 expect(await old,"the drainer of the old credential ends instead of holding the lock").toMatchObject({uploaded:0,failed:0,dropped:0});
 while(spoolLengthV1(root)>0||claimHeldV1(state,"uploader"))await new Promise(done=>setTimeout(done,100));
 expect(await createFleetClientV1(readFleetSettingsV1(root,right)!).hasResult(key),"the successor uploaded the job").toBe(true);
},120000);

test("a worker that ends idle leaves no spawn throttle behind (it clears the spawn stamp): a unit queued right after it ended starts a new worker at once and is uploaded",async()=>{
 const server=await startRemoteCacheServerV1({directory:directory("srv-idle"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(server);
 const cas=directory("cas-idle"),root=directory("cache-idle"),out=directory("out-idle"),state=join(root,fleet.layout.state),environment={...process.env,SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas,SEMIO_UNIT_CACHE_UPLOADER_IDLE_MS:"200"};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libcore-1.rlib"),"rlib bytes");writeFileSync(join(out,"core-1.d"),`${out}/core-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{1}\n");
 const localKey="f".repeat(64),client=createFleetClientV1(readFleetSettingsV1(root,environment)!);
 expect(publishUnitV1(root,{key:localKey,crate:"core",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libcore-1.rlib"),join(out,"core-1.d")],stdout:"",stderr:"",stampDirectory:undefined})).toBeDefined();
 const ended=async(key:string)=>{while(await client.hasResult(key)!==true||spoolLengthV1(root)>0||claimHeldV1(state,"uploader")||present(join(state,fleet.layout.spawned)))await new Promise(done=>setTimeout(done,50));};
 for(const [round,key] of["1a","1b"].map((prefix,index)=>[index,prefix.repeat(32)] as const)){
  expect(enqueueUploadV1(root,job({key,localKey,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
  expect(startUploaderV1(root,environment),round===0?"the first unit starts the worker":"the unit after an idle end is not throttled by the start before it").toBe(true);
  await ended(key);
 }
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

test("a unit as large as the fleet cache admits is shared: the client bounds equal the server's largest entry; a unit beyond them ends once as upload-oversize, never as upload-failed, and a unit within them is uploaded",async()=>{
 const server=policy.remoteCache.server;
 expect(fleet.limits.maximumEntryBytes).toBe(Math.min(server.maximumEntryBytes,Math.floor(server.maximumBytes/2)));
 expect(fleet.limits.maximumBlobBytes).toBe(fleet.limits.maximumEntryBytes);
 expect(fleet.limits.maximumUploadBytesInFlight).toBeLessThan(fleet.limits.maximumEntryBytes*fleet.limits.maximumUploadJobs);
 const cache=await startRemoteCacheServerV1({directory:directory("srv5"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(cache);
 const cas=directory("cas5"),root=directory("cache5"),out=directory("out5"),workspace=directory("work5");
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:cache.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas,SEMIO_UNIT_CACHE_WORKSPACE_ROOT:workspace};
 process.env.SEMIO_CAS_ROOT=cas;
 const rlib=Buffer.alloc(3*2**20,7);rlib.writeUInt32BE(5,0);
 writeFileSync(join(out,"libbig-1.rlib"),rlib);writeFileSync(join(out,"big-1.d"),`${out}/big-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{5}\n");
 const small="a".repeat(64),large="b".repeat(64);
 for(const key of [small,large])expect(publishUnitV1(root,{key,crate:"big",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libbig-1.rlib"),join(out,"big-1.d")],stdout:"",stderr:"",stampDirectory:undefined})).toBeDefined();
 const entryDirectory=join(root,"e",large.slice(0,2),large),file=join(entryDirectory,readdirSync(entryDirectory).find(name=>name.endsWith(".json"))!),entry=JSON.parse(readFileSync(file,"utf8")) as {outputs:{bytes:number}[]};
 entry.outputs[0]!.bytes=fleet.limits.maximumEntryBytes+1;writeFileSync(file,JSON.stringify(entry));
 expect(enqueueUploadV1(root,job({key:"c".repeat(64),localKey:small,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
 expect(enqueueUploadV1(root,job({key:"d".repeat(64),localKey:large,cwd:out,queuedMs:Date.now()+1,claimed:false}))).toBe(true);
 expect(await drainUploadsV1(root,environment,{idleMilliseconds:0})).toMatchObject({uploaded:1,failed:1});
 expect(spoolLengthV1(root)).toBe(0);
 const client=createFleetClientV1(readFleetSettingsV1(root,environment)!);
 expect(await client.hasResult("c".repeat(64))).toBe(true);expect(await client.hasResult("d".repeat(64))).toBe(false);
 expect(readBuildCountersV1(workspace).skipReasons).toEqual({"upload-oversize":1});
},120000);

test("a unit waiting in the spool is pinned in the host store while the drainer holds it: a capacity eviction over the cap keeps its entry and objects, and the pin leaves with the upload",async()=>{
 const cache=await startRemoteCacheServerV1({directory:directory("srv6"),host:"127.0.0.1",port:0,maximumBytes:2**30,maximumEntryBytes:2**28,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});servers.push(cache);
 let open:()=>void=()=>{};const gate=new Promise<void>(done=>{open=done;});
 const proxy=Bun.serve({port:0,async fetch(request){
  if(request.method==="PUT")await gate;
  return fetch(cache.url+new URL(request.url).pathname+new URL(request.url).search,{method:request.method,headers:request.headers,body:request.method==="GET"||request.method==="HEAD"?undefined:await request.arrayBuffer()});
 }});proxies.push(proxy);
 const cas=directory("cas6"),root=directory("cache6"),out=directory("out6");
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:`http://127.0.0.1:${proxy.port}`,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token",SEMIO_CAS_ROOT:cas};
 process.env.SEMIO_CAS_ROOT=cas;
 writeFileSync(join(out,"libpin-1.rlib"),"pinned rlib bytes");writeFileSync(join(out,"pin-1.d"),`${out}/pin-1.d: ${out}/lib.rs\n`);writeFileSync(join(out,"lib.rs"),"pub fn id()->u32{6}\n");
 const localKey="e".repeat(64),entry=publishUnitV1(root,{key:localKey,crate:"pin",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libpin-1.rlib"),join(out,"pin-1.d")],stdout:"",stderr:"",stampDirectory:undefined})!;
 const digests=entry.outputs.map(row=>row.sha256),entryDirectory=join(root,"e",localKey.slice(0,2),localKey),other="9".repeat(64);
 writeFileSync(join(out,"libpin-1.rlib"),"unspooled rlib bytes");publishUnitV1(root,{key:other,crate:"pin",cwd:out,outDir:out,laneRoot:undefined,outputs:[join(out,"libpin-1.rlib"),join(out,"pin-1.d")],stdout:"",stderr:"",stampDirectory:undefined});
 expect(enqueueUploadV1(root,job({key:"f".repeat(64),localKey,cwd:out,queuedMs:Date.now(),claimed:false}))).toBe(true);
 const drained=drainUploadsV1(root,environment,{idleMilliseconds:0});
 for(let tries=0;tries<200;tries++){const pins=await readCasPinsV1(cas,Date.now());if(digests.every(digest=>pins.has(digest)))break;await Bun.sleep(25);}
 expect(digests.every(digest=>pins(cas).has(digest)),"the waiting unit is pinned").toBe(true);
 const collected=await collectCasV1({root:cas,policy:{...CAS_POLICY_V1,capBytes:1,minimumCapBytes:1,holderProtectedShare:0,minAgeMs:0,areas:[]}});
 expect(collected.status).toBe("collected");
 expect(readdirSync(entryDirectory).filter(name=>name.endsWith(".json")).length,"the entry survives the capacity eviction").toBe(1);
 expect(present(join(root,"e",other.slice(0,2),other))&&readdirSync(join(root,"e",other.slice(0,2),other)).some(name=>name.endsWith(".json")),"a unit that waits for nothing is evicted").toBe(false);
 expect(digests.every(digest=>present(join(cas,"v1","o",digest.slice(0,CAS_POLICY_V1.fanOutHexLength),digest.slice(CAS_POLICY_V1.fanOutHexLength))))).toBe(true);
 open();
 expect(await drained).toMatchObject({uploaded:1});
 expect(digests.some(digest=>pins(cas).has(digest)),"the pin leaves with the upload").toBe(false);
},120000);

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
 writeFileSync(path,JSON.stringify({version:1,fetchedAtMs:Date.now(),checkedAtMs:Date.now(),selectedEndpoint:"http://127.0.0.1:1",candidates:["http://127.0.0.1:1"],selectedExecution:null,facts:{version:1,group:"g1",generation:1,publishedAtMs:Date.now(),publisher:{origin:"m01",directory:"/x",port:1},cache:{instance:"main",access:"read-write",protocols:["bazel-http","nx-self-hosted"],protocolVersion:1,transport:"trusted-lan"},auth:{scheme:"semio-fleet-v1",roles:["cache-read","cache-write","submit","worker","admin"]}}}));
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
