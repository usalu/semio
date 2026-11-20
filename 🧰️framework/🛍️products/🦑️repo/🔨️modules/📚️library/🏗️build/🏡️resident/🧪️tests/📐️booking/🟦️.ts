/** 📐️ Contract: what a graph slice books in the capacity gate is sized from this machine, never above a share of its physical memory and never below the least it starts with; a plan on a 16 GB host that learned a peak of 6627 MB asks for 6553 MB (at least 2048 MB), gets what is free and sizes its worker from the grant instead of waiting for ever, and says in English and German what it waits for and for how long.
 * @see ./🐍️oracle/🐍️.py */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {sliceBookingV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {HOST_POLICY_V1} from "../../🟨️.mjs";
import {renderSliceReservationV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json" with {type:"json"};

const elasticity=HOST_POLICY_V1.sliceHostElasticity as {maximumMegabytes:number;hostCapBasisPoints:number;minimumMegabytes:number};
const base={maximum:elasticity.maximumMegabytes,capBasisPoints:elasticity.hostCapBasisPoints,minimum:elasticity.minimumMegabytes,unlearned:HOST_POLICY_V1.sliceUnlearnedMegabytes,declared:null as number|null};
const cases=[{...base,host:16384,learned:6627},{...base,host:16384,learned:8192},{...base,host:16384,learned:0},{...base,host:32768,learned:6627},{...base,host:32768,learned:20000},{...base,host:4096,learned:6627},{...base,host:8192,learned:6627},{...base,host:16384,learned:0,declared:3540},{...base,host:0,learned:6627}];

test("the booking equals an independent Python oracle on 4, 8, 16 and 32 GB hosts and never exceeds the cap of the machine",()=>{
 const run=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url))],{input:JSON.stringify(cases),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);
 const expected=JSON.parse(run.stdout) as [number,number][];
 cases.forEach((item,index)=>{const got=sliceBookingV1({declaredMegabytes:item.declared??undefined,learnedMegabytes:item.learned,unlearnedMegabytes:item.unlearned,maximumMegabytes:item.maximum,hostMegabytes:item.host,hostCapBasisPoints:item.capBasisPoints,minimumMegabytes:item.minimum});expect([got.wantedMegabytes,got.minimumMegabytes],JSON.stringify(item)).toEqual(expected[index]!);});
 expect(expected[0]).toEqual([6553,2048]);expect(expected[1]).toEqual([6553,2048]);expect(expected[2]).toEqual([2048,2048]);expect(expected[3]).toEqual([6627,2048]);expect(expected[4]).toEqual([8192,2048]);expect(expected[5]).toEqual([2048,2048]);
});

test("a plan on a simulated 16 GB host books 6553 of 6627 learned MB with a minimum of 2048, and a worker started on a smaller grant is told its parent budget; a full grant leaves the worker's budget alone",async()=>{
 const asked:{megabytes:number;minimumMegabytes:number}[]=[],started:Record<string,string|undefined>[]=[];
 const harness=(granted:number)=>{const listeners:Record<string,(value:unknown)=>void>={};return createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,graceMilliseconds:50,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,learned:{read:()=>6627,write:()=>undefined},reserve:async({megabytes,minimumMegabytes}:{megabytes:number;minimumMegabytes:number})=>{asked.push({megabytes,minimumMegabytes});return{megabytes:granted,attach:()=>undefined,release:()=>undefined};},createWorker:(_url:unknown,_data:unknown,env:Record<string,string|undefined>)=>{started.push(env);queueMicrotask(()=>listeners.message?.({type:"done",graph:{}}));return{pid:4242,on:(event:string,listener:(value:unknown)=>void)=>{listeners[event]=listener;},terminate:async()=>0};}});};
 const shrunk=harness(2891);await shrunk.run({root:"/tmp/a"});await shrunk.close();
 const full=harness(6553);await full.run({root:"/tmp/b"});await full.close();
 expect(asked).toEqual([{megabytes:6553,minimumMegabytes:2048},{megabytes:6553,minimumMegabytes:2048}]);
 expect(started[0]!["SEMIO_GRAPH_MEMORY_MB"]).toBe("1445");expect(started[1]!["SEMIO_GRAPH_MEMORY_MB"]).toBeUndefined();
});

for(const row of fixture.cases)test("reservation reports "+row.id,async()=>{
 const progress:any[]=[],controller=new AbortController(),db=new Database(":memory:");
 let probes=0,started=0,awaiting=false;
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,heartbeatMilliseconds:20,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,
  learned:{read:()=>6627,write:()=>undefined},headroom:()=>{probes++;return{limitBytes:Infinity,currentBytes:0,availableBytes:7830*1048576};},
  reserve:({signal,onWait}:{signal:AbortSignal;onWait:(row:unknown)=>void})=>new Promise((_resolve,reject)=>{
   if(row.wait!==null)onWait(row.wait);
   awaiting=true;
   signal.addEventListener("abort",()=>reject(signal.reason),{once:true});
  }),createWorker:()=>{started++;throw Error("worker started before grant");}});
 const running=pool.run({root:"/fixture",arguments:[]},{signal:controller.signal,onProgress:(_ms:number,detail:unknown)=>progress.push(detail)}).then(()=>undefined,error=>error);
 const until=async(test:()=>boolean)=>{const at=Date.now();while(!test()&&Date.now()-at<5000)await Bun.sleep(10);return test();};
 try{
  expect(await until(()=>awaiting)).toBe(true);
  const initialProbes=probes;
  expect(await until(()=>progress.length>=3)).toBe(true);
  expect(probes).toBe(initialProbes);
  expect(progress[0]).toEqual({phase:"reservation",reservation:{requested:6553,...row.wait===null?{}:{wait:row.wait}}});
  const text=renderSliceReservationV1(progress[0].reservation,5,["en","de"]);
  const oracle=db.query<{reason:string},[string]>("select coalesce(json_extract(?1, '$.reason'),'unknown') reason").get(JSON.stringify(row.wait))!.reason;
  expect(progress[0].reservation.wait?.reason??"unknown").toBe(oracle);
  expect(text.en).toContain(row.en);expect(text.de).toContain(row.de);
  for(const output of [text.en,text.de]){
   expect(output).toContain("6553");
   expect(output).not.toMatch(/running \(|0 (?:of|von)|\{[a-z]+\}/u);
   if(row.wait!==null)for(const number of [row.wait.active,row.wait.queued,row.wait.spareMegabytes,row.wait.floorMegabytes,row.wait.waitedSeconds])expect(output).toContain(String(number));
   else expect(output).not.toContain("7830");
  }
  expect(renderSliceReservationV1({requested:6553,granted:2891},5,["de"])).toEqual({de:expect.stringContaining("2891")});
 }finally{
  controller.abort(Error("stop"));
  expect((await running).message).toBe("stop");
  const count=progress.length;await Bun.sleep(50);expect(progress.length).toBe(count);
  expect(started).toBe(0);await pool.close();db.close();
 }
});

/** 📨️ Carries the reservation through the resident planner and its normal progress frame. */
test("resident plan frames carry gate progress in both languages",()=>{
 const root=mkdtempSync(join(tmpdir(),"rv-")),module=(path:string)=>JSON.stringify(new URL(path,import.meta.url).href);
 const script=`
import{createHostServiceV1}from ${module("../../🏛️service/🌐️host/🟨️.mjs")};
import{createGraphPlannerPortV1}from ${module("../../🔌️ports/🧭️plan/🟨️.mjs")};
import{requestPlanV1}from ${module("../../🚪️client/🛰️plan/🟨️.mjs")};
import{hostEndpointsV1}from ${module("../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs")};
const root=process.cwd(),implementation="a".repeat(64),environment={...process.env,XDG_RUNTIME_DIR:"r",TMPDIR:"r",LOCALAPPDATA:root};
const runtime=hostEndpointsV1(implementation,environment),frames=[],wait=JSON.parse(process.argv[1]);
const planner=createGraphPlannerPortV1({loadSlice:async({onProgress})=>{
 onProgress(1000,{phase:"reservation",reservation:{requested:6553}});
 onProgress(5000,{phase:"reservation",reservation:{requested:6553,wait}});
 onProgress(6000,{phase:"reservation",reservation:{requested:6553,granted:2891}});
 return {graph:{version:1,projects:{},dependencies:[]}};
}});
const service=createHostServiceV1({runtime,implementation,ports:{planner}});
await service.start();
try{await requestPlanV1({root,implementation,targets:["*:*"],arguments:["show","projects"],agent:"booking",language:"de",environment,onProgress:row=>frames.push(row)});}
finally{await service.stop("test");}
console.log(JSON.stringify(frames.filter(row=>row.phase==="slice")));`;
 try{
  const result=spawnSync(process.execPath,["-e",script,JSON.stringify(fixture.cases[0]!.wait)],{cwd:root,encoding:"utf8",timeout:15000});
  expect(result.status,result.stderr).toBe(0);
  const frames=JSON.parse(result.stdout.trim());
  expect(frames.map((row:{done:number;total:number})=>[row.done,row.total])).toEqual([[1000,0],[5000,0],[6000,0]]);
  expect(frames[0].message).toMatchObject({en:expect.stringContaining("waiting for a host capacity decision"),de:expect.stringContaining("Kapazitätsentscheidung")});
  expect(frames[1].message).toMatchObject({en:expect.stringContaining("an earlier request is still waiting"),de:expect.stringContaining("eine frühere Anfrage wartet noch")});
  expect(frames[2].message).toMatchObject({en:expect.stringContaining("granted 2891 MB"),de:expect.stringContaining("2891 MB")});
 }finally{rmSync(root,{recursive:true,force:true});}
},20000);
