/** 📐️ Contract: what a graph slice books in the capacity gate is sized from this machine, never above a share of its physical memory and never below the least it starts with; a plan on a 16 GB host that learned a peak of 6627 MB asks for 6553 MB (at least 2048 MB), gets what is free and sizes its worker from the grant instead of waiting for ever, and says in English and German what it waits for and for how long.
 * @see ./🐍️oracle/🐍️.py */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {createSlicePoolV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🟨️.mjs";
import {sliceBookingV1} from "../../🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {HOST_POLICY_V1} from "../../🟨️.mjs";
import {renderTextV1} from "../../📨️protocol/🎚️policy/🟨️.mjs";

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

test("a plan that waits for its reservation says in English and German what it waits for, how much is free above the floor and when it gives up",async()=>{
 const progress:any[]=[],controller=new AbortController();
 const pool=createSlicePoolV1({workerUrl:new URL("file:///unused.mjs"),maximum:1,heartbeatMilliseconds:20,hostMegabytes:16384,elasticity,unlearnedMegabytes:2048,learned:{read:()=>6627,write:()=>undefined},headroom:()=>({limitBytes:Infinity,currentBytes:0,availableBytes:7830*1048576}),reserveFacts:()=>({floorMegabytes:2048,limitSeconds:120}),reserve:({signal}:{signal:AbortSignal})=>new Promise((_resolve,reject)=>{signal.addEventListener("abort",()=>reject(signal.reason),{once:true});}),createWorker:()=>{throw new Error("no worker may start");}});
 const running=pool.run({root:"/tmp/c"},{signal:controller.signal,onProgress:(_ms:number,detail:any)=>progress.push(detail)}).then(()=>undefined,error=>error);
 const until=async()=>{const at=Date.now();while(progress.length<1&&Date.now()-at<5000)await new Promise(done=>setTimeout(done,10));};await until();
 expect(progress[0]).toMatchObject({phase:"reservation",total:6553,decision:{key:"reserving",used:6553,budget:5782,minimum:2048,floor:2048,limit:120}});
 const decision=progress[0].decision,text=renderTextV1("host-slice-owners-"+decision.key,{seconds:5,phase:"reservation",done:0,total:6553,owners:"",used:decision.used,budget:decision.budget,kind:"",minimum:decision.minimum,floor:decision.floor,limit:decision.limit},["en","de"])!;
 for(const language of ["en","de"] as const){for(const number of ["6553","2048","5782","120"])expect(text[language],language).toContain(number);expect(text[language]).not.toMatch(/\{[a-z]+\}/u);}
 expect(text.en).toContain("waiting for memory");expect(text.de).toContain("wartet auf Speicher");
 controller.abort(new Error("stop"));expect((await running).message).toBe("stop");await pool.close();
});
