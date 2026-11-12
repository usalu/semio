import{test,expect}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{fileURLToPath}from"node:url";
import{join}from"node:path";
import{mkdirSync}from"node:fs";
import{uptime}from"node:os";
import{heapStats}from"bun:jsc";
import Ajv from"ajv";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixtureSchema from"./🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import moduleSchema from"../🧬️schema/🔣️.json";
import*as memory from"../🟨️.mjs";
/** 🧪️ A private scratch directory created by the test itself: the temporary directory a previous test file of the same process was granted may be retired already. */
const scratchDirectory=(prefix:string):string=>{mkdirSync(tmpdir(),{recursive:true});return mkdtempSync(join(tmpdir(),prefix));};

const MEGABYTE=1048576;
const oraclePath=fileURLToPath(new URL("./🐍️.py",import.meta.url)),python=testPythonV1();

/** 🐍️ Runs the independent Python model and returns its JSON answer. */
function oracle(args:string[]):any{
 const done=Bun.spawnSync([python!,"-I",oraclePath,...args]);
 if(done.exitCode!==0)throw Error("Oracle failed: "+new TextDecoder().decode(done.stderr));
 return JSON.parse(new TextDecoder().decode(done.stdout));
}

/** 📄️ Writes a JSON document the oracle reads. */
function written(directory:string,name:string,value:unknown):string{
 const path=join(directory,name);writeFileSync(path,JSON.stringify(value));return path;
}

test("the fixture is closed, the policy and the texts equal their declaration",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 expect(fixture.policy).toEqual(moduleSchema.definitions.Policy.const);
 expect(fixture.messages).toEqual(moduleSchema.definitions.Messages.const);
 expect(JSON.parse(JSON.stringify(memory.GRAPH_MEMORY_POLICY_V1))).toEqual(moduleSchema.definitions.Policy.const);
 expect(JSON.parse(JSON.stringify(memory.GRAPH_MEMORY_MESSAGES_V1))).toEqual(moduleSchema.definitions.Messages.const);
 for(const key of Object.keys(moduleSchema.definitions.Messages.const.en))expect(Object.keys(moduleSchema.definitions.Messages.const.de)).toContain(key);
 const holders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]!).sort();
 for(const key of Object.keys(moduleSchema.definitions.Messages.const.en) as ("waiting"|"serial"|"resumed"|"learning")[]){
  const english=holders(moduleSchema.definitions.Messages.const.en[key]),german=holders(moduleSchema.definitions.Messages.const.de[key]);
  expect(german,key).toEqual(english);for(const name of english)expect(["budget","used","running","kind"],key).toContain(name);
 }
});

test("the language choice, the messages and the owner concurrency of a headroom equal the independent model",()=>{
 const directory=scratchDirectory("memory-law-");
 try{
  const expected=oracle(["expect",written(directory,"fixture.json",fixture)]);
  fixture.languages.forEach((row,index)=>{expect(memory.selectGraphMemoryLanguagesV1(row.environment),row.name).toEqual(row.expect as ("en"|"de")[]);expect(expected.languages[index],row.name).toEqual(row.expect);});
  fixture.messageCases.forEach((row,index)=>{
   const actual=memory.renderGraphMemoryMessagesV1(row.key as "waiting",row.fields as Record<string,number>,row.environment);
   expect(actual).toEqual(row.expect);expect(actual).toEqual(expected.messages[index]);
  });
  expect(()=>memory.renderGraphMemoryMessagesV1("unknown" as "waiting",{},{LANG:"de_DE"})).toThrow();
  expect(Object.isFrozen(memory.GRAPH_MEMORY_POLICY_V1)&&Object.isFrozen(memory.GRAPH_MEMORY_MESSAGES_V1.de)).toBe(true);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** ⏱️ A deterministic clock: timers fire in time order, every firing is followed by a full flush of the pending promise work. */
class FakeTime{
 clock=0;timers:{at:number;sequence:number;fire:()=>void}[]=[];sequence=0;
 now=()=>this.clock;
 sleep=(milliseconds:number,signal?:AbortSignal)=>new Promise<void>((resolve,reject)=>{
  if(signal?.aborted){reject(signal.reason);return;}
  const timer={at:this.clock+milliseconds,sequence:this.sequence++,fire:()=>{signal?.removeEventListener("abort",abort);resolve();}},abort=()=>{this.timers=this.timers.filter(row=>row!==timer);reject(signal!.reason);};
  signal?.addEventListener("abort",abort,{once:true});this.timers.push(timer);
 });
 at(milliseconds:number,action:()=>void){this.timers.push({at:milliseconds,sequence:this.sequence++,fire:action});}
 async settle(){for(let round=0;round<8;round++){await Promise.resolve();await new Promise<void>(done=>setImmediate(done));}}
 async drive(done:()=>boolean){
  await this.settle();
  while(!done()){
   this.timers.sort((a,b)=>a.at-b.at||a.sequence-b.sequence);
   const next=this.timers.shift();if(!next)throw Error("Deadlock: no timer left and the scenario is not finished");
   this.clock=Math.max(this.clock,next.at);next.fire();await this.settle();
  }
 }
}

test("the graph child heap is the declared size and equals the independent model",()=>{
 const directory=scratchDirectory("memory-child-heap-");
 try{
  const expected=oracle(["expect",written(directory,"fixture.json",fixture)]),bytes=memory.graphChildHeapBytesV1();
  expect(expected.childHeap).toEqual({bytes,grantIndependent:true});
  expect(bytes).toBe(moduleSchema.definitions.Policy.const.childHeapMegabytes*MEGABYTE);
  expect(memory.graphChildHeapBytesV1({...memory.GRAPH_MEMORY_POLICY_V1,childHeapMegabytes:1024})).toBe(1024*MEGABYTE);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the default sample reads constant-time counters, never a heap walk",()=>{
 const ports=memory.createMemoryPortsV1(),retained:object[]=[];
 for(let index=0;index<400000;index++)retained.push({index,text:"x".repeat(12)+index,list:[index,index+1,{inner:index}]});
 const started=performance.now();let last=0;for(let round=0;round<200;round++)last=ports.sample();
 const perCall=(performance.now()-started)/200,counters=process.memoryUsage();
 expect(perCall).toBeLessThan(5);
 expect(Math.abs(last-memory.graphMemoryPressureBytesV1(counters))/last).toBeLessThan(0.25);expect(last).toBeGreaterThanOrEqual((counters.heapUsed+counters.external)*0.75);
 expect(retained.length).toBe(400000);
});

test("the governor refuses a missing worker count or headroom and rejects every waiter on abort",async()=>{
 expect(()=>memory.createMemoryGovernorV1({workers:0,headroom:()=>MEGABYTE})).toThrow();
 expect(()=>memory.createMemoryGovernorV1({workers:1} as never)).toThrow();
 expect(()=>memory.createMemoryGovernorV1({workers:1,headroom:()=>MEGABYTE,estimateBytes:0})).toThrow();
 const controller=new AbortController();controller.abort(Error("stopped"));
 const governor=memory.createMemoryGovernorV1({workers:1,headroom:()=>MEGABYTE,signal:controller.signal});
 await expect(governor.run(()=>1)).rejects.toThrow("stopped");
});

test("interning keeps the JSON bytes, shares equal long strings and never touches keys or short strings",()=>{
 const interner=memory.createStringInternerV1({minimumLength:8});
 const source={"a-long-key-that-is-never-shared":["/Users/x/semio/path/one.ts","/Users/x/semio/path/one.ts","short"],nested:{list:[{value:"/Users/x/semio/path/one.ts",number:7,flag:true,none:null}],text:"🧰️framework/📦️packages/🟦️typescript"},more:["🧰️framework/📦️packages/🟦️typescript"]};
 const text=JSON.stringify(source),interned=memory.internJsonStringsV1(JSON.parse(text),interner);
 expect(JSON.stringify(interned)).toBe(text);
 expect(interner.stats().hits).toBe(3);expect(interner.stats().entries).toBe(2);
 const bounded=memory.createStringInternerV1({maximumCharacters:40,minimumLength:8});
 for(let index=0;index<10;index++)bounded.share("0123456789".repeat(2)+index);
 expect(bounded.stats().clears).toBeGreaterThan(0);expect(bounded.stats().characters).toBeLessThanOrEqual(40);
 expect(bounded.share("abc")).toBe("abc");
});

/** 📏️ The live managed bytes after a full collection. */
function managed():number{Bun.gc(true);const statistics=heapStats();return statistics.heapSize+statistics.extraMemorySize;}

test("interned results cost a fraction of the memory of the same parsed results",()=>{
 const long="{workspaceRoot}/"+"🧰️framework/📦️packages/🟦️typescript/".repeat(8)+"file.ts",text=JSON.stringify({inputs:Array.from({length:20},(_,index)=>long+"?"+index%5),targets:{build:{inputs:[long]}}});
 const retain=(interner?:ReturnType<typeof memory.createStringInternerV1>)=>Array.from({length:2000},()=>{const value=JSON.parse(text);return interner?memory.internJsonStringsV1(value,interner):value;});
 const before=managed(),plain=retain(),plainBytes=managed()-before;
 const afterPlain=managed(),shared=retain(memory.createStringInternerV1()),sharedBytes=managed()-afterPlain;
 expect(plain.length).toBe(shared.length);
 expect(plainBytes).toBeGreaterThan(20*MEGABYTE);
 expect(sharedBytes).toBeLessThan(plainBytes/4);
});

test("a governor leaves no abort listener on a long-lived signal once disposed: a stage that runs a chunk of projects per call must not keep the environment of every chunk alive through the listener",async()=>{
 const listeners=new Set<unknown>(),signal={aborted:false,reason:undefined,throwIfAborted(){},addEventListener:(_type:string,listener:unknown)=>listeners.add(listener),removeEventListener:(_type:string,listener:unknown)=>listeners.delete(listener)} as unknown as AbortSignal;
 const governors=Array.from({length:5},()=>memory.createMemoryGovernorV1({workers:2,signal,headroom:()=>1<<30}));
 expect(listeners.size).toBe(5);
 for(const governor of governors){expect(await governor.run(async()=>7)).toBe(7);governor.dispose();}
 expect(listeners.size).toBe(0);
});

/** 🚦️ Runs `workers` owners of `ownerMegabytes` against a modeled cgroup and answers how many ran at once. */
async function peakOwners(row:typeof fixture.headroom[number],estimateMegabytes?:number){
 const time=new FakeTime(),limit=row.limitMegabytes*MEGABYTE,margin=limit*memory.GRAPH_MEMORY_POLICY_V1.safetyMarginBasisPoints/10000;
 let live=0,running=0,peak=0,done=0;
 const headroom=()=>limit-row.currentMegabytes*MEGABYTE-live-margin;
 const governor=memory.createMemoryGovernorV1({workers:row.workers,headroom,estimateBytes:(estimateMegabytes??row.ownerMegabytes)*MEGABYTE,now:time.now,sleep:time.sleep});
 const task=async()=>{running++;peak=Math.max(peak,running);live+=row.ownerMegabytes*MEGABYTE;await time.sleep(5000);live-=row.ownerMegabytes*MEGABYTE;running--;done++;};
 const all=Array.from({length:row.workers*2},()=>governor.run(task));
 await time.drive(()=>done===row.workers*2);await Promise.all(all);governor.dispose();
 return{peak,cost:governor.stats().ownerCostMegabytes};
}

test("with a 4 GB cgroup, 2.5 GB in use and 200 MB owners more than one owner runs at a time, and every row equals the independent model",async()=>{
 const directory=scratchDirectory("memory-headroom-");
 try{
  const expected=oracle(["expect",written(directory,"fixture.json",fixture)]);
  for(const[index,row]of fixture.headroom.entries()){const{peak}=await peakOwners(row);expect(peak,row.name).toBe(expected.headroom[index]);}
  expect((await peakOwners(fixture.headroom[0]!)).peak).toBeGreaterThan(1);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the peak of an owner is learned from the headroom the running owners consume, and a wrong first guess is corrected",async()=>{
 const row=fixture.headroom[0]!,low=await peakOwners(row,20),high=await peakOwners(row,900);
 expect(low.cost,"a first guess below the truth is raised to the consumed headroom").toBe(row.ownerMegabytes);
 expect(high.peak,"a first guess above the headroom still runs one owner and then learns").toBeGreaterThanOrEqual(1);
});

test("the service headroom is the cgroup limit less the usage less the safety margin, and the host answers when no cgroup is readable",()=>{
 const margin=memory.GRAPH_MEMORY_POLICY_V1.safetyMarginBasisPoints/10000;
 expect(memory.serviceMemoryHeadroomBytesV1({cgroup:()=>({limitBytes:4096*MEGABYTE,currentBytes:2560*MEGABYTE})})).toBe(Math.floor(4096*MEGABYTE*(1-margin)-2560*MEGABYTE));
 expect(memory.serviceMemoryHeadroomBytesV1({cgroup:()=>({limitBytes:Infinity,currentBytes:1000*MEGABYTE}),total:()=>8192*MEGABYTE})).toBe(Math.floor(8192*MEGABYTE*(1-margin)-1000*MEGABYTE));
 expect(memory.serviceMemoryHeadroomBytesV1({cgroup:()=>undefined,total:()=>8192*MEGABYTE,free:()=>3000*MEGABYTE})).toBe(Math.floor(8192*MEGABYTE*(1-margin)-5192*MEGABYTE));
});

test("the admission law of one more owner equals the independent model on every row",()=>{
 const directory=scratchDirectory("memory-admission-");
 try{
  const expected=oracle(["expect",written(directory,"fixture.json",fixture)]);
  fixture.admission.forEach((row,index)=>{
   const actual=memory.graphOwnerAdmissionV1({headroom:row.headroomMegabytes*MEGABYTE,running:row.running,inflight:row.inflight.map(owner=>({estimate:owner.estimateMegabytes*MEGABYTE,taken:owner.takenMegabytes*MEGABYTE})),estimate:row.estimateMegabytes*MEGABYTE,learned:row.learned});
   expect(actual,row.name).toBe(row.expect);expect(expected.admission[index],row.name).toBe(row.expect);
  });
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** 🚦️ Runs the tasks of a scenario through a governor against a modeled cgroup; every task takes the same time and grows the usage by its growth while it runs. */
async function runScenario(row:typeof fixture.scenarios[number],peaks:Record<string,number>){
 const time=new FakeTime(),limit=row.limitMegabytes*MEGABYTE,margin=limit*memory.GRAPH_MEMORY_POLICY_V1.safetyMarginBasisPoints/10000,total=limit-margin-row.currentMegabytes*MEGABYTE;
 let live=0,running=0,peak=0,done=0,maximumLive=0;
 const governor=memory.createMemoryGovernorV1({workers:row.workers,headroom:()=>total-live,estimateBytes:row.guessMegabytes*MEGABYTE,peaks:{read:()=>({...peaks}),write:value=>{for(const key of Object.keys(peaks))delete peaks[key];Object.assign(peaks,value);}},now:time.now,sleep:time.sleep});
 const all=row.tasks.map(task=>governor.run(async()=>{running++;peak=Math.max(peak,running);live+=task.growthMegabytes*MEGABYTE;maximumLive=Math.max(maximumLive,live);await time.sleep(5000);live-=task.growthMegabytes*MEGABYTE;running--;done++;},{kind:task.kind}));
 await time.drive(()=>done===row.tasks.length);await Promise.all(all);governor.dispose();
 return{peak,maximumLive,total};
}

test("owners of a kind without a learned peak run alone, learned and persisted peaks admit more, and no scenario exceeds the headroom, equal to the independent model",async()=>{
 const directory=scratchDirectory("memory-scenario-");
 try{
  const expected=oracle(["expect",written(directory,"fixture.json",fixture)]);
  for(const[index,row]of fixture.scenarios.entries()){
   const peaks=Object.fromEntries(Object.entries(row.persisted).map(([kind,megabytes])=>[kind,megabytes*MEGABYTE])),{peak,maximumLive,total}=await runScenario(row,peaks);
   expect(peak,row.name).toBe(row.expectPeak);expect(expected.scenarios[index].peak,row.name).toBe(row.expectPeak);
   expect(Object.fromEntries(Object.entries(peaks).map(([kind,bytes])=>[kind,Math.round(bytes/MEGABYTE)])),row.name).toEqual(row.expectLearned);expect(expected.scenarios[index].learned,row.name).toEqual(row.expectLearned);
   if(row.tasks.every(task=>task.growthMegabytes*MEGABYTE<=total))expect(maximumLive,row.name).toBeLessThanOrEqual(total);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the learned peaks of one governor persist into the next, which then admits concurrently at once",async()=>{
 const row=fixture.scenarios[0]!,peaks:Record<string,number>={};
 expect((await runScenario({...row,persisted:{}},peaks)).peak).toBe(row.expectPeak);
 const second=await runScenario({...row,tasks:row.tasks.slice(0,2)},peaks);
 expect(second.peak).toBe(2);
});

test("the learned owner peaks persist in a file, a missing or damaged file reads as nothing learned",()=>{
 const directory=scratchDirectory("memory-peaks-");
 try{
  const path=join(directory,"deep","peaks.json"),store=memory.createGraphOwnerPeaksFileV1(path);
  expect(store.read()).toEqual({});
  store.write({"language:cargo":262144000});expect(memory.createGraphOwnerPeaksFileV1(path).read()).toEqual({"language:cargo":262144000});
  writeFileSync(path,"{broken");expect(store.read()).toEqual({});
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a declared memory limit replaces the cgroup and charges the resident memory of this process",()=>{
 const margin=memory.GRAPH_MEMORY_POLICY_V1.safetyMarginBasisPoints/10000,name=memory.GRAPH_MEMORY_POLICY_V1.limitEnvironment;
 expect(memory.serviceMemoryHeadroomBytesV1({environment:{[name]:"4096"},usage:()=>1500*MEGABYTE,cgroup:()=>{throw Error("the cgroup must not be read");}})).toBe(Math.floor(4096*MEGABYTE*(1-margin)-1500*MEGABYTE));
 expect(memory.serviceMemoryHeadroomBytesV1({environment:{[name]:"nonsense"},cgroup:()=>({limitBytes:4096*MEGABYTE,currentBytes:2560*MEGABYTE})})).toBe(Math.floor(4096*MEGABYTE*(1-margin)-2560*MEGABYTE));
});

test("the governor reports pressure only when the host floor is crossed, never because a learned peak is large: a poisoned peak must not flush the shared state after every owner",()=>{
 const floor=memory.GRAPH_MEMORY_POLICY_V1.hostAvailableFloorMegabytes;let free=(floor+1000)*MEGABYTE;
 const governor=memory.createMemoryGovernorV1({workers:2,headroom:()=>free,estimateBytes:300*MEGABYTE,peaks:{read:()=>({rust:5000*MEGABYTE}),write:()=>undefined}});
 expect(governor.pressure(),"a headroom far below the learned peak but above the floor").toBe(false);
 free=(floor-1)*MEGABYTE;expect(governor.pressure()).toBe(true);free=(floor+1)*MEGABYTE;expect(governor.pressure()).toBe(false);
 const unlearned=memory.createMemoryGovernorV1({workers:2,headroom:()=>(floor-200)*MEGABYTE,estimateBytes:300*MEGABYTE});expect(unlearned.pressure()).toBe(true);
 governor.dispose();unlearned.dispose();
});

test("every admission decision reaches the decide port with its closed key, megabytes and kind, in addition to the bilingual lines",async()=>{
 const time=new FakeTime(),decisions:{key:string;used:number;budget:number;kind?:string}[]=[],lines:string[]=[];let live=0;
 const governor=memory.createMemoryGovernorV1({workers:2,headroom:()=>1386*MEGABYTE-live,estimateBytes:256*MEGABYTE,now:time.now,sleep:time.sleep,decide:decision=>decisions.push(decision),report:line=>lines.push(line),environment:{LANG:"en_US"}});
 let done=0;const task=async()=>{live+=250*MEGABYTE;await time.sleep(12000);live-=250*MEGABYTE;done++;};
 const all=[governor.run(task,{kind:"language:cargo"}),governor.run(task,{kind:"language:cargo"})];
 await time.drive(()=>done===2);await Promise.all(all);governor.dispose();
 const learning=decisions.find(row=>row.key==="learning");
 expect(learning).toEqual({key:"learning",used:256,budget:expect.any(Number),kind:"language:cargo"});
 expect(lines.some(line=>line.includes("language:cargo")||line.includes("one project"))).toBe(true);
 for(const row of decisions)expect(["waiting","serial","resumed","learning"]).toContain(row.key);
});

test("on a host that reports only machine-wide headroom the peak of an owner is the growth of this process, never what other processes took meanwhile",async()=>{
 const learn=async(attributed:boolean)=>{
  const time=new FakeTime(),total=64000*MEGABYTE;let own=0,other=0,done=0;
  const governor=memory.createMemoryGovernorV1({workers:2,headroom:()=>total-own-other,...(attributed?{attributable:()=>own}:{}),estimateBytes:20*MEGABYTE,now:time.now,sleep:time.sleep});
  const task=async()=>{own+=50*MEGABYTE;other+=1000*MEGABYTE;await time.sleep(5000);own-=50*MEGABYTE;done++;};
  const all=Array.from({length:4},()=>governor.run(task,{kind:"language:rust"}));
  await time.drive(()=>done===4);await Promise.all(all);const cost=governor.stats().kinds["language:rust"]!;governor.dispose();return cost;
 };
 expect(await learn(false),"without the own counter the noise of other processes is charged to the owner").toBeGreaterThanOrEqual(1000);
 expect(await learn(true),"with the own counter only the growth of this process is charged").toBe(50);
});
