/** 🛟️ Contract: the operating system's temporary-file cleanup never takes the key of a running graph service and a missing key never fails commands for long. A service touches its key before it ages out and writes it back when it is gone; a client that still finds no key restarts the service once and the next generation writes a new one. Incident of 2026-11-22 03:38: macOS `dirhelper` (daily 03:35, files older than three days in the per-user temporary directory) removed the key file of a service that had run for more than three days; every command failed with `(key)` until the service was restarted by hand.
 * @see ../../📨️protocol/🔑️capability/🛟️keeper/🟨️.mjs
 * @see ../../🚪️client/🛰️plan/🟨️.mjs */
import {afterAll,describe,expect,test} from "bun:test";
import {copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {join} from "node:path";
import {HERE} from "../🟦️.ts";
import {keepRootKeyV1} from "../../📨️protocol/🔑️capability/🛟️keeper/🟨️.mjs";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {HostUnavailableErrorV1,planKeyActionV1,requestPlanV1} from "../../🚪️client/🛰️plan/🟨️.mjs";
import {DISK_BOUNDS_V1,selectTemporaryV1} from "../../../🌡️host-capacity/🧹️disk-recovery/🧺️bounds/🟦️.ts";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";

const GRAPH=join(HERE,"🧫️fixtures/🛰️host-graph.json");
const base=mkdtempSync(join(process.platform==="win32"?process.env.TEMP??"C:/Windows/Temp":"/tmp","kk-"));
afterAll(()=>rmSync(base,{recursive:true,force:true}));
const DAY=86400000,days=(count:number)=>new Date(Date.now()-count*DAY);
const folder=(name:string)=>{const path=join(base,name);mkdirSync(path,{recursive:true});return path;};
const sweepOlderThanThreeDays=(directory:string)=>spawnSync("find",[directory,"-type","f","-mtime","+3","-atime","+3","-delete"]);

describe("the key keeper",()=>{
 test("a missing key is written back byte for byte and private, a different key of a newer generation is left alone",()=>{
  const directory=folder("restore"),path=join(directory,"key"),key=Buffer.from("k".repeat(32)),keeper=keepRootKeyV1({path,key,refreshMilliseconds:3600000});
  expect(keeper.tick()).toBe("restored");
  expect(readFileSync(path)).toEqual(key);
  if(process.platform!=="win32")expect(statSync(path).mode&0o777).toBe(0o600);
  expect(keeper.tick()).toBe("intact");
  writeFileSync(path,"newer generation key");
  expect(keeper.tick()).toBe("intact");
  expect(readFileSync(path,"utf8")).toBe("newer generation key");
  expect(keepRootKeyV1({path:join(directory,"absent","key"),key,refreshMilliseconds:1}).tick()).toBe("failed");
 });
 test("a key and a descriptor that age towards the cleanup are touched before it, a fresh one is not rewritten",()=>{
  const directory=folder("touch"),path=join(directory,"key"),descriptor=join(directory,"host.json");
  writeFileSync(path,"k");writeFileSync(descriptor,"{}");
  utimesSync(path,days(2.5),days(2.5));utimesSync(descriptor,days(2.5),days(2.5));
  const keeper=keepRootKeyV1({path,key:Buffer.from("k"),touch:[descriptor],refreshMilliseconds:DAY});
  expect(keeper.tick()).toBe("refreshed");
  for(const target of[path,descriptor])expect(Date.now()-statSync(target).mtimeMs).toBeLessThan(60000);
  const stamp=statSync(path).mtimeMs;utimesSync(path,new Date(stamp-1000),new Date(stamp-1000));
  expect(keeper.tick()).toBe("intact");
  expect(Math.abs(statSync(path).mtimeMs-(stamp-1000))).toBeLessThan(5);
 });
 test.skipIf(process.platform==="win32")("a cleanup of files older than three days (the system `find` as the independent oracle) takes an unkept key and spares a kept one",()=>{
  const directory=folder("cleanup"),kept=join(directory,"kept"),forgotten=join(directory,"forgotten");
  for(const path of[kept,forgotten]){writeFileSync(path,"k");utimesSync(path,days(4),days(4));}
  keepRootKeyV1({path:kept,key:Buffer.from("k"),refreshMilliseconds:DAY}).tick();
  expect(sweepOlderThanThreeDays(directory).status).toBe(0);
  expect(existsSync(kept)).toBe(true);
  expect(existsSync(forgotten)).toBe(false);
 });
});

describe("the client of a missing key",()=>{
 const missing=Object.assign(new HostUnavailableErrorV1("key"),{missing:true});
 test("waits for the service to write the key back, then restarts the service once, then fails; a damaged key is never retried",()=>{
  expect(planKeyActionV1(missing,0,false)).toBe("wait");
  expect(planKeyActionV1(missing,29,false)).toBe("wait");
  expect(planKeyActionV1(missing,30,false)).toBe("restart");
  expect(planKeyActionV1(missing,30,true)).toBe("fail");
  expect(planKeyActionV1(new HostUnavailableErrorV1("key"),0,false)).toBe("fail");
  expect(planKeyActionV1(new HostUnavailableErrorV1("lost"),0,false)).toBe("fail");
  expect(planKeyActionV1(undefined,0,false)).toBe("fail");
 });
});

describe("a running service and the cleanup of its key",()=>{
 const environmentOf=(name:string)=>{const dir=folder(name);return{dir,env:{...process.env,XDG_RUNTIME_DIR:dir,TMPDIR:dir,LOCALAPPDATA:dir} as Record<string,string>};};
 const serviceOf=(name:string,extra:Record<string,unknown>={})=>{
  const{dir,env}=environmentOf(name),implementation=createHash("sha256").update("impl-"+name).digest("hex"),runtime=hostEndpointsV1(implementation,env),root=join(base,"w-"+name);
  mkdirSync(root,{recursive:true});copyFileSync(GRAPH,join(root,"graph.json"));
  const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>JSON.parse(readFileSync(GRAPH,"utf8"))});
  const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50,...extra}});
  return{dir,env,implementation,runtime,root:realpathSync(root),service};
 };
 test("the service writes a removed key back on its next check and the same request then succeeds without a restart",async()=>{
  const h=serviceOf("heal");
  try{
   await h.service.start();
   const key=readFileSync(h.runtime.key);
   rmSync(h.runtime.key);
   const reply=await requestPlanV1({root:h.root,implementation:h.implementation,targets:["app:test"],agent:"a",language:"en",environment:h.env});
   expect(reply.reply.t).toBe("plan-reply");
   expect(readFileSync(h.runtime.key)).toEqual(key);
  }finally{await h.service.stop("test");}
 });
 test("the service touches an aged key and descriptor, so the daily cleanup finds nothing to take",async()=>{
  const h=serviceOf("age",{keyRefreshMilliseconds:1000});
  try{
   await h.service.start();
   for(const path of[h.runtime.key,h.runtime.descriptor])utimesSync(path,days(5),days(5));
   await Bun.sleep(300);
   h.service.sweep();
   for(const path of[h.runtime.key,h.runtime.descriptor])expect(Date.now()-statSync(path).mtimeMs).toBeLessThan(60000);
   if(process.platform!=="win32"){expect(sweepOlderThanThreeDays(h.dir).status).toBe(0);expect(existsSync(h.runtime.key)).toBe(true);expect(existsSync(h.runtime.descriptor)).toBe(true);}
  }finally{await h.service.stop("test");}
 });
 test("the declared policy refreshes the key far more often than the cleanup age",()=>{
  expect(HOST_POLICY_V1.keyRefreshMilliseconds).toBeGreaterThan(0);
  expect(HOST_POLICY_V1.keyRefreshMilliseconds*3).toBeLessThanOrEqual(3*DAY);
 });
});

describe("the cleanups of the build system and the service state",()=>{
 test("the temporary-directory cleanup never selects the runtime directories of the graph service, whatever their age, nor a key file",()=>{
  const now=Date.UTC(2026,10,22),old=now-400*86400000,row=(name:string,directory=true)=>({name,newestMs:old,bytes:1024,directory});
  const names=[row("semio-repo"),row("sr501"),row("semio-repo-501"),row("key",false),row("h-0123456789ab.sock",false),row("host-0123456789ab.json",false),row("semio-farm-session-1111-X6WOW1")];
  expect(selectTemporaryV1(names,now,DISK_BOUNDS_V1.temporaryDirectories,()=>false)).toEqual(["semio-farm-session-1111-X6WOW1"]);
 });
});
