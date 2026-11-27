/** 🪢️ Contract: no unit of a low-resource worker ever waits for ever, holds a connection or a claim while it waits for local admission, or is blocked by an unreachable cache. The wedge of 30 wrappers at 0 % CPU is reproduced with fake wrappers: a live holder that never releases the single compile slot, a cache that stalls every long poll, a cache that never answers.
 * @see ../../⏳️admit/🟦️.ts (`awaitCompileAdmissionV1`)
 * @see ../../🛰️fleet/🟦️.ts (`obtainFleetUnitV1`, `FleetHoldV1`)
 * @see ../../🔒️lock/🟦️.ts (`acquireAloneV1`, `acquireLightSlotV1`, `tryAcquireRemoteWaitV1`) */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {createServer,type Server} from "node:net";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {awaitCompileAdmissionV1,whileClaimParkedV1} from "../../⏳️admit/🟦️.ts";
import {SlotWaitTimeoutV1,acquireAloneV1,acquireLightSlotV1,tryAcquireRemoteWaitV1,tryAcquireSlotV1,tryClaimV1} from "../../🔒️lock/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1} from "../../🛰️fleet/📡️client/🟦️.ts";
import {holdFleetKeyV1,obtainFleetUnitV1,type FleetUnitV1} from "../../🛰️fleet/🟦️.ts";
import {FLEET_POLICY_V1} from "../../🛰️fleet/🪪️identity/🟦️.ts";
import {UNIT_CACHE_POLICY_V1,type AdmitFactsV1} from "../../🔏️key/🟦️.ts";

const scratch:string[]=[],servers:Server[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"wedge-"+label+"-")));scratch.push(path);return path;};
afterAll(()=>{for(const server of servers)server.close();for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const key=(index:number)=>index.toString(16).padStart(64,"0");
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const ENVIRONMENT=(endpoint:string)=>({SEMIO_REMOTE_CACHE_ENDPOINT:endpoint,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TIMEOUT_SECONDS:"1",SEMIO_REMOTE_CACHE_RETRIES:"0"});

type Cache=Readonly<{fetch:typeof fetch;active:Map<string,string>;state:{inflight:number;awaiting:number;maximumAwaiting:number;claims:number;releases:number}}>;
function fakeCache(mode:"grant"|"held"):Cache{
 const active=new Map<string,string>(),state={inflight:0,awaiting:0,maximumAwaiting:0,claims:0,releases:0};
 const answer=(status:number,body:unknown)=>new Response(JSON.stringify(body),{status,headers:{"content-type":"application/json"}});
 const stall=(signal:AbortSignal|null|undefined)=>new Promise<Response>((_,reject)=>{if(signal?.aborted){reject(signal.reason);return;}signal?.addEventListener("abort",()=>reject(signal.reason),{once:true});});
 const fake=(async(input:RequestInfo|URL,init?:RequestInit)=>{
  state.inflight++;
  try{
   const url=new URL(String(input)),method=init?.method??"GET",parts=url.pathname.split("/").filter(Boolean),owner=init?.body?String((JSON.parse(String(init.body)) as {owner?:string}).owner??""):"";
   const claim=parts.indexOf("claim");
   if(claim<0)return parts.includes("slots")?answer(200,{}):answer(404,{});
   const target=parts[claim+2]!,action=parts[claim+3];
   if(action==="await"){state.awaiting++;state.maximumAwaiting=Math.max(state.maximumAwaiting,state.awaiting);try{return await stall(init?.signal);}finally{state.awaiting--;}}
   if(action==="release"){state.releases++;if(active.get(target)===owner)active.delete(target);return answer(200,{});}
   if(action==="renew")return active.get(target)===owner?answer(200,{}):answer(409,{});
   state.claims++;
   if(mode==="held"||active.has(target)&&active.get(target)!==owner)return answer(409,{state:"held",by:"other:9:aa",untilMilliseconds:Date.now()+30000});
   active.set(target,owner);return answer(200,{state:"granted",untilMilliseconds:Date.now()+30000});
  }finally{state.inflight--;}
 }) as typeof fetch;
 return {fetch:fake,active,state};
}
function unitOf(root:string,index:number,world:Readonly<{fetch?:typeof fetch;endpoint?:string}>):FleetUnitV1{
 const settings=readFleetSettingsV1(root,ENVIRONMENT(world.endpoint??"http://fake.invalid"))!,client=createFleetClientV1(settings,world.fetch===undefined?{}:{fetch:world.fetch});
 return {client,root,environment:{},localKey:"1".repeat(64),key:key(index),slot:"2".repeat(64),slotParts:[],crate:"c"+String(index),platform:{version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""},roots:[],cwd:"/w",workspace:undefined,laneRoot:undefined,owner:"wrapper:"+String(index)+":aa",expected:[]};
}
const NOISE=()=>undefined;

test("30 wrappers behind a compile slot whose live holder never lets go hold no claim and no request while they wait, all end within the bound, and say so in both languages",async()=>{
 const root=directory("slot"),cache=fakeCache("grant"),wedged=tryAcquireSlotV1(root,1)!,reports:string[]=[],events:string[]=[],bound=6000;
 let compiled=0;
 const wrapper=async(index:number)=>{
  const unit=unitOf(root,index,{fetch:cache.fetch}),decision=await obtainFleetUnitV1(unit,NOISE,{retries:0});
  expect(decision.outcome).toBe("compile");
  const hold=decision.outcome==="compile"?decision.hold:undefined,admission=await awaitCompileAdmissionV1({root,slots:1,crate:unit.crate,since:Date.now(),environment:{},facts:undefined,alone:false,hold,report:text=>reports.push(text),event:(_,reason)=>{if(reason)events.push(reason);},bounds:{slotMaximumWaitMilliseconds:bound,parkAfterMilliseconds:100}});
  compiled++;
  expect(admission.timedOut).toBe("slot");expect(admission.slot).toBeUndefined();expect(admission.claim).toBe("kept");
  await hold?.release();
 };
 const started=Date.now(),all=Promise.all(Array.from({length:30},(_,index)=>wrapper(index)));
 const parkedAll=()=>events.filter(reason=>reason.startsWith("claim-parked:")).length===30&&cache.active.size===0&&cache.state.inflight===0;
 const limit=Date.now()+bound-200;while(!parkedAll()&&Date.now()<limit)await sleep(20);
 expect(events.filter(reason=>reason.startsWith("claim-parked:")).length,"every waiting unit parked").toBe(30);
 expect(cache.active.size,"every waiting unit gave its fleet claim back").toBe(0);
 expect(cache.state.inflight,"no request is open while the units wait for admission").toBe(0);
 await all;
 expect(Date.now()-started).toBeLessThan(bound+20000);
 expect(compiled).toBe(30);
 expect(cache.active.size).toBe(0);
 expect(reports).toHaveLength(30);
 for(const report of reports){expect(report).toContain("the compile slot stayed taken");expect(report).toContain(" / ");expect(report).toContain("der Compile-Slot blieb");}
 expect(readdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.tickets)).length,"no ticket is left behind").toBe(0);
 wedged.release();
},30000);

test("a unit that finds the compile slot free never parks its claim and never touches the broker",async()=>{
 const root=directory("free"),cache=fakeCache("grant"),unit=unitOf(root,1,{fetch:cache.fetch}),hold=(await holdFleetKeyV1(unit))!;
 let parks=0;
 const admission=await awaitCompileAdmissionV1({root,slots:1,crate:"c1",since:Date.now(),environment:{SEMIO_BUILD_BROKER:join(root,"no-such.sock"),SEMIO_BUILD_PASS:"p"},facts:{crate:"c1"} as unknown as AdmitFactsV1,alone:false,hold:{park:async()=>{parks++;},resume:async()=>"granted"},report:NOISE,event:NOISE});
 expect(admission.slot).toBeDefined();expect(admission.waitedMs).toBeLessThan(1000);expect(parks).toBe(0);expect(cache.active.size).toBe(1);
 admission.slot!.release();await hold.release();expect(cache.active.size).toBe(0);
});

test("a claim that another machine took while the unit waited is reported, the unit compiles on and its hold is lost",async()=>{
 const root=directory("lost"),cache=fakeCache("grant"),wedged=tryAcquireSlotV1(root,1)!,reports:string[]=[],unit=unitOf(root,2,{fetch:cache.fetch}),hold=(await holdFleetKeyV1(unit))!;
 const stealing=sleep(500).then(()=>{cache.active.set(unit.key,"idle-machine:7:bb");});
 const admission=await awaitCompileAdmissionV1({root,slots:1,crate:"c2",since:Date.now(),environment:{},facts:undefined,alone:false,hold,report:text=>reports.push(text),event:NOISE,bounds:{slotMaximumWaitMilliseconds:900,parkAfterMilliseconds:100}});
 await stealing;
 expect(admission.claim).toBe("lost");expect(hold.lost()).toBe(true);expect(reports.some(text=>text.includes("took the fleet claim")&&text.includes("übernommen"))).toBe(true);
 expect(cache.active.get(unit.key)).toBe("idle-machine:7:bb");
 await hold.release();expect(cache.active.get(unit.key),"a lost hold never releases the claim of the machine that took it").toBe("idle-machine:7:bb");
 wedged.release();
},15000);

test("a cache whose holder never answers the long poll cannot park every job of a machine: at most the policy's waiters wait, the others compile at once and all end at the bound",async()=>{
 const root=directory("holder"),cache=fakeCache("held"),policy={maximumWaitMilliseconds:1200,minimumWaitMilliseconds:300,retries:0,maximumRemoteWaiters:4,retryBackoffMilliseconds:5},reasons:string[]=[];
 const started=Date.now();
 await Promise.all(Array.from({length:30},async(_,index)=>{const decision=await obtainFleetUnitV1(unitOf(root,index,{fetch:cache.fetch}),NOISE,policy);expect(decision.outcome).toBe("compile");if(decision.outcome==="compile")reasons.push(decision.reason);}));
 expect(Date.now()-started).toBeLessThan(8000);
 expect(cache.state.maximumAwaiting).toBeLessThanOrEqual(4);
 expect(reasons.filter(reason=>reason==="waiters-full").length).toBeGreaterThanOrEqual(26);
 expect(reasons).toHaveLength(30);
 expect(readdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.slots)).filter(name=>name.startsWith("await-")),"every wait place is given back").toEqual([]);
},20000);

test("an unreachable cache never blocks local compiles: a server that accepts and never answers costs each unit one bound, not the compile",async()=>{
 const root=directory("hung"),server=createServer(socket=>{socket.on("error",()=>undefined);});servers.push(server);
 await new Promise<void>(done=>server.listen(0,"127.0.0.1",done));
 const endpoint="http://127.0.0.1:"+String((server.address() as {port:number}).port),started=Date.now();let compiled=0;
 await Promise.all(Array.from({length:30},async(_,index)=>{
  const unit=unitOf(root,index,{endpoint}),decision=await obtainFleetUnitV1(unit,NOISE,{retries:0});
  expect(decision.outcome).toBe("compile");
  const admission=await awaitCompileAdmissionV1({root,slots:30,crate:unit.crate,since:Date.now(),environment:{},facts:undefined,alone:false,hold:decision.outcome==="compile"?decision.hold:undefined,report:NOISE,event:NOISE});
  compiled++;admission.slot?.release();
 }));
 expect(compiled).toBe(30);expect(Date.now()-started).toBeLessThan(15000);
},30000);

test("the whole-host wait and the offload slot wait are bounded and leave no claim behind",async()=>{
 const root=directory("alone"),wedged=tryAcquireSlotV1(root,1)!,started=Date.now();
 const alone=await acquireAloneV1(root,1,{maximumWaitMilliseconds:500}).then(()=>undefined,error=>error);
 expect(alone).toBeInstanceOf(SlotWaitTimeoutV1);expect(Date.now()-started).toBeLessThan(5000);
 expect(readdirSync(join(root,UNIT_CACHE_POLICY_V1.layout.locks)).filter(name=>name.startsWith("alone"))).toEqual([]);
 const light=[await acquireLightSlotV1(root,1)],refused=await acquireLightSlotV1(root,1,{maximumWaitMilliseconds:400}).then(()=>undefined,error=>error);
 expect(refused).toBeInstanceOf(SlotWaitTimeoutV1);
 for(const claim of light)claim.release();wedged.release();
 const places=[0,1,2,3].map(()=>tryAcquireRemoteWaitV1(root,3));
 expect(places.filter(place=>place!==undefined)).toHaveLength(3);expect(places[3]).toBeUndefined();
 for(const place of places)place?.release();
 expect(tryClaimV1(join(root,UNIT_CACHE_POLICY_V1.layout.slots),"await-0")).toBeDefined();
 expect(tryAcquireRemoteWaitV1(root,0)).toBeUndefined();
});

test("a broker that never admits is waited out at its bound, then the slot is taken and the bilingual line names the bound",async()=>{
 const root=mkdtempSync(join(tmpdir(),"wh-")),endpoint=join(root,"b.sock"),reports:string[]=[];scratch.push(root);
 const broker=createServer(socket=>{socket.on("error",()=>undefined);let text="";socket.on("data",chunk=>{text+=chunk;if(!text.includes("\n"))return;text="";socket.end(JSON.stringify({v:1,op:"headroom",wait:true})+"\n");});});servers.push(broker);
 await new Promise<void>(done=>broker.listen(endpoint,done));
 const facts={crate:"syn",package:"syn",version:"2.0.0",target:"host",crateTypes:["lib"],outDir:"/o",extraFilename:"-1",source:"/s/lib.rs",manifestDir:"/s",primary:false,buildScriptOutDir:null,externs:[]} as unknown as AdmitFactsV1;
 const admission=await awaitCompileAdmissionV1({root,slots:1,crate:"syn",since:Date.now(),environment:{SEMIO_BUILD_BROKER:endpoint,SEMIO_BUILD_PASS:"p1"},facts,alone:false,report:text=>reports.push(text),event:NOISE,bounds:{headroomMaximumWaitMilliseconds:300,headroomPauseMilliseconds:20}});
 expect(admission.timedOut).toBe("headroom");expect(admission.slot).toBeDefined();expect(reports[0]).toContain("did not admit the unit");expect(reports[0]).toContain("nicht zugelassen");
 admission.slot!.release();
});

test("a wait that fails still takes the parked claim back before the error goes on, and a short wait never parks",async()=>{
 const calls:string[]=[],hold={park:async()=>{calls.push("park");},resume:async()=>{calls.push("resume");return "granted" as const;}};
 const failed=await whileClaimParkedV1(hold,50,async()=>{await sleep(300);throw Error("slot-bound");}).then(()=>undefined,error=>error as Error);
 expect(failed?.message).toBe("slot-bound");expect(calls).toEqual(["park","resume"]);
 calls.length=0;
 const quick=await whileClaimParkedV1(hold,500,async()=>{await sleep(20);return 7;});
 expect(quick).toEqual({value:7,claim:"kept"});expect(calls).toEqual([]);
 expect(await whileClaimParkedV1(undefined,10,async()=>{await sleep(50);return 1;})).toEqual({value:1,claim:"none"});
 const lost=await whileClaimParkedV1({park:async()=>undefined,resume:async()=>"lost" as const},10,async()=>{await sleep(80);return 2;});
 expect(lost.claim).toBe("lost");
});

test("the build script run bounds its compile slot and its offload slot and parks its claim while it waits",()=>{
 const source=readFileSync(new URL("../../🏗️buildscript/🟦️.ts",import.meta.url),"utf8");
 expect(source).toMatch(/acquireLightSlotV1\([^)]*maximumWaitMilliseconds/u);
 expect(source).toMatch(/acquireSlotV1\([^)]*maximumWaitMilliseconds/u);
 expect(source).toContain("whileClaimParkedV1(input.hold");
});

test("the wrapper has no unbounded wait left: every slot wait passes its bound and the wrapper uses the admission member",()=>{
 const wrapper=readFileSync(new URL("../../📜️script.ts",import.meta.url),"utf8"),admit=readFileSync(new URL("../../⏳️admit/🟦️.ts",import.meta.url),"utf8");
 expect(wrapper).not.toContain("acquireAloneV1(");
 expect(wrapper.match(/acquireSlotV1\(settings/gu)).toHaveLength(1);expect(wrapper).toContain("AbortSignal.timeout(waitMilliseconds)");
 expect(wrapper).toMatch(/acquireLightSlotV1\([^)]*maximumWaitMilliseconds/u);
 expect(admit).toMatch(/acquireAloneV1\([^)]*maximumWaitMilliseconds/u);expect(admit).toMatch(/acquireSlotV1\([^)]*maximumWaitMilliseconds/u);
 for(const name of ["slotTimeout","aloneTimeout","lightTimeout","headroomTimeout","claimParked","claimLostAfterPark"])expect(admit+wrapper).toContain(name);
 expect(FLEET_POLICY_V1.claim.maximumRemoteWaiters).toBeGreaterThan(0);
 expect(UNIT_CACHE_POLICY_V1.slot.aloneMaximumWaitMilliseconds).toBeGreaterThan(UNIT_CACHE_POLICY_V1.slot.maximumWaitMilliseconds-1);
});
