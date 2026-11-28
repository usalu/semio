/** 🪪️ Contract: the shared resident service carries no lease owner, and the script process of a request carries the owner of the client that asked. A stage runner of the readiness protocol reaps every process that carries the owner token of a stopped try (`SEMIO_LEASE_OWNER`): a service that kept the token of the client that started it carried the work of every other client under it, so one stopped try killed reads of unrelated clients ("Original Script process terminated"). The declared rows list the inherited environment, the names the service never keeps (direct spawn and launchd job), the environment of the worker of one request, the frames that carry an owner and the pool that serves three requests of different owners.
 * @see ../../🏗️composition/🌐️host/🟨️.mjs `spawnHostServiceV1`
 * @see ../../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs `allowedEnvironmentV1`
 * @see ../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs `sliceWorkerEnvironmentV1`
 * @see ../../🧬️schema/🔣️.json `PlanRequest.owner` */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {spawnHostServiceV1} from "../../🏗️composition/🌐️host/🟨️.mjs";
import {allowedEnvironmentV1} from "../../🏗️composition/🌐️host/🍎️launchd/🟨️.mjs";
import {createSlicePoolV1,shareKeyV1,sliceWorkerEnvironmentV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {validatorV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {waitFor} from "../🟦️.ts";

const rows=JSON.parse(readFileSync(new URL("./🔣️.json",import.meta.url),"utf8"));
const base=mkdtempSync(join(process.platform==="win32"?tmpdir():"/tmp","ro-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const PROBE=fileURLToPath(new URL("./🔎️env-probe.mjs",import.meta.url));
const OWNER="SEMIO_LEASE_OWNER";

test("a service started by a client that runs under a stage owner carries no lease owner and no other invocation ownership",async()=>{
 const out=join(base,"service-env.json"),directory=join(base,"runtime");
 const start=spawnHostServiceV1({command:[process.execPath,PROBE,out],environment:{PATH:process.env.PATH??"",SEMIO_SERVICE_LAUNCHD:"off",...rows.service.inherited}});
 const handle=start({runtime:{directory,log:join(base,"service.log")}}) as {pid:number}|undefined;
 try{
  expect(await waitFor(()=>existsSync(out)&&readFileSync(out,"utf8")!=="",15000,50)).toBe(true);
  const seen=JSON.parse(readFileSync(out,"utf8"));
  for(const name of rows.service.absent)expect(seen[name],name).toBeUndefined();
  for(const name of rows.service.kept)expect(seen[name],name).toBe(rows.service.inherited[name]);
 }finally{if(handle?.pid)try{process.kill(handle.pid,"SIGKILL");}catch{}}
},30000);

test("the launchd job carries no lease owner and no invocation ownership of its asker",()=>{
 const{SEMIO_HOST_CAPACITY_GRANT:_removedBeforeTheJob,...asked}=rows.service.inherited,allowed=allowedEnvironmentV1({PATH:"/p",...asked});
 expect(Object.keys(allowed).sort()).toEqual([...rows.service.launchd].sort());
});

for(const row of rows.environments)test("worker environment: "+row.id,()=>{
 const environment=sliceWorkerEnvironmentV1(row.base,row.request);
 expect(environment[OWNER]??null).toBe(row.owner);
 for(const [name,value] of Object.entries(row.base))if(name!==OWNER)expect(environment[name]).toBe(value);
});

for(const row of rows.frames)test("frame: "+row.id,()=>{
 const frame={t:"plan",id:1,workspace:"0123456789ab",root:"/w",targets:["a:b"],agent:"a",priority:1,language:"en",owner:row.owner};
 expect(validatorV1("PlanRequest")(frame).length===0).toBe(row.valid);
});

test("loads of different owners never share a worker and each worker carries its own owner, whatever the service inherited",async()=>{
 const previous=process.env[OWNER];process.env[OWNER]="ready:cccccccccccc:workspace:3";
 const seen:{name:string;owner:string|null}[]=[],finish:(()=>void)[]=[];
 try{
  const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:rows.pool.workers,heartbeatMilliseconds:20,graceMilliseconds:50,hostMegabytes:16384,reaper:{reap:async()=>0,killNow:()=>undefined},reserve:async({megabytes}:{megabytes:number})=>({megabytes,attach:()=>undefined,release:()=>undefined}),
   createWorker:(_url:URL,data:any,environment:Record<string,string>)=>{const listeners:Record<string,(value:unknown)=>void>={};seen.push({name:data.name,owner:environment[OWNER]??null});finish.push(()=>listeners.message?.({type:"done",graph:{by:data.name}}));return{pid:undefined,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});
  const answers=rows.pool.requests.map((request:{name:string;owner?:string})=>pool.run({root:"/fixture",arguments:["run","a:test"],...request}));
  expect(await waitFor(()=>seen.length===rows.pool.workers,10000,20)).toBe(true);
  for(const done of finish)done();
  await Promise.all(answers);await pool.close();
  expect(Object.fromEntries(seen.map(row=>[row.name,row.owner]))).toEqual(rows.pool.expected);
  expect(new Set(rows.pool.requests.map((request:{owner?:string})=>shareKeyV1({root:"/fixture",arguments:["run","a:test"],...request}))).size).toBe(rows.pool.workers);
 }finally{if(previous===undefined)delete process.env[OWNER];else process.env[OWNER]=previous;}
},30000);
