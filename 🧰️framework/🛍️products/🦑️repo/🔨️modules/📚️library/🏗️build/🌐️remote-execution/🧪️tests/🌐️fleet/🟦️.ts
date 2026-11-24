import {afterEach,expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync} from "node:fs";
import {join} from "node:path";
import {registeredToolExecutableV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {requireRemoteExecutionShapeV1} from "../../🟦️.ts";
import {stretchedBudgetMillisecondsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {attachClusterV1,eventuallyV1 as eventuallyUnscaledV1,startClusterV1,writeHostViewV1,type ClusterV1} from "../🧰️cluster/🟦️.ts";
const eventuallyV1=(condition:()=>boolean|Promise<boolean>,milliseconds:number,message:string):Promise<void>=>eventuallyUnscaledV1(condition,stretchedBudgetMillisecondsV1(milliseconds),message);

const bun=registeredToolExecutableV1("bun"),here=import.meta.dir,entry=join(here,"..","..","🏃️entry","🟦️.ts"),serve=join(here,"..","🧰️cluster","🚀️serve.ts");
type Spawned=ReturnType<typeof Bun.spawn>;
type Fleet=Readonly<{attached:ReturnType<typeof attachClusterV1>;scheduler:Spawned;urls:{grpc:string;worker:string;cache:string};directory:string;view:(name:string,host?:Partial<Record<string,unknown>>,extra?:Parameters<typeof writeHostViewV1>[2])=>void;workers:Map<string,Spawned>;load:()=>Promise<any>;spawnWorker:(name:string)=>Spawned}>;
const fleets:Fleet[]=[],clusters:ClusterV1[]=[],scratch:string[]=[];
afterEach(async()=>{
 for(const fleet of fleets.splice(0)){fleet.attached.stop();for(const child of [...fleet.workers.values(),fleet.scheduler]){child.kill("SIGTERM");}await Promise.all([...fleet.workers.values(),fleet.scheduler].map(child=>Promise.race([child.exited,Bun.sleep(8000)])));for(const child of [...fleet.workers.values(),fleet.scheduler])child.kill("SIGKILL");}
 for(const cluster of clusters.splice(0))await cluster.stop();
 for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});
});
const temporary=():string=>{const path=mkdtempSync(join(testScratchRootV1("rexec-fleet"),"scratch-"));scratch.push(path);return path;};
const text=async(stream:ReadableStream<Uint8Array>|number|undefined):Promise<string>=>{
 if(typeof stream==="number"||stream===undefined)throw Error("a pipe is required");
 const reader=stream.getReader();let out="";
 for(;;){const row=await reader.read();if(row.done)break;out+=new TextDecoder().decode(row.value);if(out.includes("\n"))break;}
 reader.releaseLock();return out;
};

async function startFleet(names:readonly string[],scheduler:Record<string,unknown>={}):Promise<Fleet>{
 const directory=temporary(),child=Bun.spawn([bun,serve],{stdout:"pipe",stderr:"inherit",env:{...process.env,SCHEDULER:JSON.stringify({leaseSeconds:2,lingerSeconds:1,maximumAttempts:2,loadExpireSeconds:4,refusalCooldownSeconds:2,...scheduler})}});
 const urls=JSON.parse(await text(child.stdout)) as {grpc:string;worker:string;cache:string},workers=new Map<string,Spawned>();
 const view=(name:string,host:Partial<Record<string,unknown>>={},extra:Parameters<typeof writeHostViewV1>[2]={})=>{writeHostViewV1(join(directory,`${name}.view.json`),host,extra);};
 const spawnWorker=(name:string):Spawned=>{
  view(name);
  const proc=Bun.spawn([bun,entry,"--join",urls.worker,"--name",name,"--origin",name,"--concurrency","2","--sandbox","isolated-root","--work-directory",join(directory,`work-${name}`),"--load-interval","1","--lang","en"],{stdout:"ignore",stderr:"ignore",cwd:directory,env:{...process.env,SEMIO_REMOTE_EXECUTION_HOST_VIEW:join(directory,`${name}.view.json`)}});
  workers.set(name,proc);return proc;
 };
 const load=async():Promise<any>=>{const body=await (await fetch(`${urls.worker}/v1/fleet/load`)).json();requireRemoteExecutionShapeV1("FleetLoad",body,"fleet load");return body;};
 const fleet:Fleet={attached:attachClusterV1(urls),scheduler:child,urls,directory,view,workers,load,spawnWorker};
 fleets.push(fleet);
 for(const name of names)spawnWorker(name);
 await eventuallyV1(async()=>{const table=await load();return names.every(name=>table.machines.some((row:any)=>row.origin===name&&row.worker.state!=="absent"));},40000,"every worker must register and report its load");
 return fleet;
}
const row=(table:any,name:string):any=>table.machines.find((machine:any)=>machine.origin===name);
const workerOf=(run:{response:any}):string=>run.response.result?.execution_metadata?.worker??"";

test("every machine appears in the live table with its platform key, capacity, pressure, headroom and worker state, and a machine that falls silent expires",async()=>{
 const fleet=await startFleet(["w0","w1","w2"]),table=await fleet.load();
 expect(table.machines.map((machine:any)=>machine.origin)).toEqual(["w0","w1","w2"]);
 for(const machine of table.machines){
  expect(machine.platform.key).toMatch(/^(macos|linux|windows)-(arm64|amd64)-([0-9a-f]{16}|none)$/u);expect(machine.capacity).toEqual({cores:8,memoryMegabytes:65536});
  expect(machine).toMatchObject({pressure:"normal",headroom:true,freeMegabytes:55904,worker:{state:"idle",sandbox:"isolated-root",confinement:"none",network:"unenforced",concurrency:2,running:0},offloadedIn:0,refused:0});
  expect(machine.ageSeconds).toBeLessThanOrEqual(3);
 }
 fleet.workers.get("w2")!.kill("SIGKILL");
 await eventuallyV1(async()=>(await fleet.load()).machines.length===2,20000,"a silent machine must leave the table");
 expect((await fleet.load()).machines.map((machine:any)=>machine.origin)).toEqual(["w0","w1"]);
});

test("a worker whose machine reports critical pressure receives nothing, the others share the work and the least loaded one is preferred",async()=>{
 const fleet=await startFleet(["w0","w1","w2"]);
 fleet.view("w2",{pressure:"critical",availableMegabytes:900});
 await eventuallyV1(async()=>{const machine=row(await fleet.load(),"w2");return machine.pressure==="critical"&&machine.headroom===false&&machine.worker.state==="refusing";},20000,"the pressure must reach the table");
 const runs=await Promise.all(Array.from({length:8},async(_,index)=>fleet.attached.execute(await fleet.attached.action({arguments:["/bin/sh","-c","sleep 0.4; echo ok > out.txt"],outputPaths:["out.txt"],salt:`share-${index}`}))));
 const used=new Set(runs.map(workerOf));
 for(const run of runs)expect(run.response.status.code).toBe(0);
 expect(used.has("w2"),"w2 is under critical pressure").toBe(false);expect([...used].sort()).toEqual(["w0","w1"]);
 const after=await fleet.load();expect(row(after,"w2").offloadedIn).toBe(0);expect(row(after,"w0").offloadedIn+row(after,"w1").offloadedIn).toBe(8);
 fleet.view("w0",{load:15});fleet.view("w2");
 await eventuallyV1(async()=>{const table=await fleet.load();return row(table,"w2").headroom===true&&row(table,"w0").load===15;},20000,"the new loads must reach the table");
 const sequential=[];for(let index=0;index<3;index++)sequential.push(workerOf(await fleet.attached.execute(await fleet.attached.action({arguments:["/bin/sh","-c","echo hi > out.txt"],outputPaths:["out.txt"],salt:`least-${index}`}))));
 expect(sequential.every(name=>name==="w1"||name==="w2"),`the busy w0 is avoided, got ${sequential}`).toBe(true);
});

test("a worker killed in the middle of an action: the action completes on another worker and the scheduler counts one retry",async()=>{
 const fleet=await startFleet(["w0","w1"]),directory=temporary(),marker=join(directory,"started");
 const action=await fleet.attached.action({arguments:["/bin/sh","-c",`echo $$ >> ${marker}; sleep 3; echo survived > out.txt`],outputPaths:["out.txt"],salt:"killed"}),run=fleet.attached.execute(action);
 await eventuallyV1(()=>existsSync(marker),20000,"the action must start");
 const table=await fleet.load(),victim=table.tasks.find((task:any)=>task.stage==="EXECUTING")?.worker as string;
 expect(["w0","w1"]).toContain(victim);fleet.workers.get(victim)!.kill("SIGKILL");
 const result=await run;
 expect(result.response.status.code).toBe(0);expect(workerOf(result)).not.toBe(victim);expect(["w0","w1"]).toContain(workerOf(result));
 expect((await (await fetch(`${fleet.urls.worker}/status`)).json()).counters.retried).toBe(1);
});

test("cancelling the call stops the remote process: its whole tree is killed and the scheduler holds nothing afterwards",async()=>{
 const fleet=await startFleet(["w0"]),directory=temporary(),pidFile=join(directory,"pid");
 const action=await fleet.attached.action({arguments:["/bin/sh","-c",`sleep 120 & echo $! > ${pidFile}.child; echo $$ > ${pidFile}; wait`],salt:"cancel"}),abort=new AbortController(),run=fleet.attached.execute(action,{signal:abort.signal}).catch(()=>undefined);
 await eventuallyV1(()=>existsSync(`${pidFile}.child`)&&readFileSync(pidFile,"utf8").trim()!=="",20000,"the remote process must start");
 const shell=Number(readFileSync(pidFile,"utf8").trim()),child=Number(readFileSync(`${pidFile}.child`,"utf8").trim()),alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
 expect(alive(shell)&&alive(child)).toBe(true);
 abort.abort();await run;
 await eventuallyV1(()=>!alive(shell)&&!alive(child),20000,"the remote process tree must be killed");
 await eventuallyV1(async()=>(await fleet.load()).tasks.length===0,10000,"the scheduler must hold no task");
});

test("semio-offload=if-idle answers RESOURCE_EXHAUSTED at once when no machine but the requester has headroom, and otherwise runs on another machine, never on the requester",async()=>{
 const fleet=await startFleet(["w0","w1","w2"]),offload=(salt:string)=>fleet.attached.action({arguments:["/bin/sh","-c","echo ok > out.txt"],outputPaths:["out.txt"],salt,platform:[{name:"semio-offload",value:"if-idle"},{name:"semio-origin",value:"w0"}]});
 for(const name of ["w1","w2"])fleet.view(name,{pressure:"critical",availableMegabytes:900});
 await eventuallyV1(async()=>{const table=await fleet.load();return ["w1","w2"].every(name=>row(table,name).headroom===false);},20000,"the pressure must reach the table");
 const started=performance.now(),declined=await fleet.attached.execute(await offload("declined"));
 expect(declined.response.status.code,"RESOURCE_EXHAUSTED").toBe(8);expect(declined.stages).not.toContain("EXECUTING");expect(performance.now()-started).toBeLessThan(stretchedBudgetMillisecondsV1(3000));
 expect((await (await fetch(`${fleet.urls.worker}/status`)).json()).counters.offloadRejected).toBe(1);
 fleet.view("w2");
 await eventuallyV1(async()=>row(await fleet.load(),"w2").headroom===true,20000,"w2 must have headroom again");
 const taken=await fleet.attached.execute(await offload("taken"));
 expect(taken.response.status.code).toBe(0);expect(workerOf(taken)).toBe("w2");
 const table=await fleet.load();expect(row(table,"w0").offloadedOut,"only the action that was really offloaded counts").toBe(1);
});

test("every worker agent advertises semio-pin=<its origin> without a flag, and a pinned action (semio-offload=queue, semio-pin=<origin>) runs on exactly that worker, never on another and never on its requester; a pin nobody offers fails back at once",async()=>{
 const fleet=await startFleet(["w0","w1","w2"]),pinned=(pin:string,salt:string)=>fleet.attached.action({arguments:["/bin/sh","-c","echo ok > out.txt"],outputPaths:["out.txt"],salt,platform:[{name:"semio-offload",value:"queue"},{name:"semio-origin",value:"w0"},{name:"semio-pin",value:pin}]});
 const workers=(await (await fetch(`${fleet.urls.worker}/status`)).json()).workers as {name:string;properties:{name:string;value:string}[]}[];
 for(const name of ["w0","w1","w2"])expect(workers.find(worker=>worker.name===name)?.properties.find(property=>property.name==="semio-pin")?.value).toBe(name);
 for(let index=0;index<3;index++)for(const target of ["w1","w2"]){const run=await fleet.attached.execute(await pinned(target,`pin-${target}-${index}`));expect(run.response.status.code).toBe(0);expect(workerOf(run)).toBe(target);}
 expect((await fleet.attached.execute(await pinned("w0","pin-self"))).response.status.code,"the requester is never its own pin").toBe(14);
 expect((await fleet.attached.execute(await pinned("w9","pin-none"))).response.status.code,"no machine offers it").toBe(14);
});

test("an action goes only to workers of the requester's toolchain build: a worker of another build never takes it",async()=>{
 const cluster=await startClusterV1({workers:0,scheduler:{maximumAttempts:2}});clusters.push(cluster);
 const mine=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:"aaaaaaaaaaaaaaaa"}],other=[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"macos"},{name:"semio-toolchain",value:"bbbbbbbbbbbbbbbb"}];
 cluster.startWorker({name:"other-build",origin:"other-build",properties:other,sandbox:"isolated-root"});cluster.startWorker({name:"same-build",origin:"same-build",properties:mine,sandbox:"isolated-root"});
 await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===2,15000,"both workers register");
 const runs=await Promise.all(Array.from({length:4},async(_,index)=>cluster.execute(await cluster.action({arguments:["/bin/sh","-c","echo ok > out.txt"],outputPaths:["out.txt"],salt:`tc-${index}`,platform:[{name:"semio-toolchain",value:"aaaaaaaaaaaaaaaa"}]}))));
 for(const run of runs)expect(workerOf(run)).toBe("same-build");
 const none=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","echo ok > out.txt"],outputPaths:["out.txt"],salt:"tc-none",platform:[{name:"semio-toolchain",value:"cccccccccccccccc"},{name:"semio-origin",value:"someone"}]}));
 expect(none.response.status.code,"no worker offers that build: failed back at once").toBe(14);
});

test("a worker's own admission can refuse an action it was given: the action returns to the queue at once and another worker takes it",async()=>{
 const cluster=await startClusterV1({workers:0,scheduler:{refusalCooldownSeconds:5}});clusters.push(cluster);
 let calls=0;
 cluster.startWorker({name:"a-refuses",origin:"a-refuses",sandbox:"isolated-root",admit:async()=>{calls++;return calls===2?{admit:false,reason:"memory"}:{admit:true};}});
 await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===1,15000,"the first worker registers");
 cluster.startWorker({name:"b-takes",origin:"b-takes",sandbox:"isolated-root"});
 await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===2,15000,"the second worker registers");
 const started=performance.now(),run=await cluster.execute(await cluster.action({arguments:["/bin/sh","-c","echo ok > out.txt"],outputPaths:["out.txt"],salt:"refused"}));
 expect(run.response.status.code).toBe(0);expect(workerOf(run)).toBe("b-takes");expect(performance.now()-started).toBeLessThan(stretchedBudgetMillisecondsV1(4000));
 expect(cluster.server.scheduler.status().counters.refused).toBe(1);expect(cluster.events.some(event=>event.startsWith("refused"))).toBe(true);expect(cluster.workerEvents.some(event=>event.includes("a-refuses refused"))).toBe(true);
});

test("a requested action is rescheduled once on another machine when its worker leaves, then fails back to the requester; with no other machine it fails back at once",async()=>{
 const cluster=await startClusterV1({workers:0,scheduler:{maximumAttempts:2}});clusters.push(cluster);
 const directory=temporary(),marker=join(directory,"starts"),lines=():number=>existsSync(marker)?readFileSync(marker,"utf8").split("\n").filter(Boolean).length:0;
 const workers=new Map([["a",cluster.startWorker({name:"a",origin:"a",sandbox:"isolated-root"})],["b",cluster.startWorker({name:"b",origin:"b",sandbox:"isolated-root"})]]);
 await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===2,15000,"both workers register");
 const action=await cluster.action({arguments:["/bin/sh","-c",`echo $0 >> ${marker}; sleep 30`],salt:"twice",platform:[{name:"semio-origin",value:"requester"}]}),run=cluster.execute(action);
 await eventuallyV1(()=>lines()===1,15000,"first start");
 const first=cluster.server.scheduler.fleetLoad().tasks as {worker:string}[],firstWorker=first.find(task=>task.worker!==null)!.worker;
 await workers.get(firstWorker)!.stop();await eventuallyV1(()=>lines()===2,15000,"the second machine takes it");
 const secondWorker=firstWorker==="a"?"b":"a";await workers.get(secondWorker)!.stop();
 const result=await run;
 expect(result.response.status.code).toBe(14);expect(result.response.status.message).toMatch(/run the action locally/u);
 const alone=await startClusterV1({workers:0,scheduler:{maximumAttempts:2}});clusters.push(alone);
 const only=alone.startWorker({name:"only",origin:"only",sandbox:"isolated-root"});await eventuallyV1(()=>alone.server.scheduler.status().workers.length===1,15000,"the only worker registers");
 const run2=alone.execute(await alone.action({arguments:["/bin/sh","-c",`echo x >> ${marker}.2; sleep 30`],salt:"alone",platform:[{name:"semio-origin",value:"requester"}]}));
 await eventuallyV1(()=>existsSync(`${marker}.2`),15000,"started on the only worker");await only.stop();
 const failed=await run2;expect(failed.response.status.code).toBe(14);expect(failed.response.status.message).toMatch(/no other machine/u);
});

test("two platforms in miniature: a macOS action is never taken by the Linux worker and a Linux action never by the macOS worker, a worker without a compiler takes no toolchain-bound action, and an action for a platform nobody offers fails back at once",async()=>{
 const cluster=await startClusterV1({workers:0,scheduler:{maximumAttempts:2}});clusters.push(cluster);
 const properties=(family:string,toolchain?:string)=>[{name:"Arch",value:"arm64"},{name:"OSFamily",value:family},...toolchain?[{name:"semio-toolchain",value:toolchain}]:[]];
 const wanted=(family:string,toolchain:string)=>[{name:"Arch",value:"arm64"},{name:"OSFamily",value:family},{name:"semio-toolchain",value:toolchain}];
 cluster.startWorker({name:"mac",origin:"mac",properties:properties("macos","aaaaaaaaaaaaaaaa"),sandbox:"isolated-root"});
 cluster.startWorker({name:"linux",origin:"linux",properties:properties("linux","aaaaaaaaaaaaaaaa"),sandbox:"isolated-root"});
 cluster.startWorker({name:"linux-no-rustc",origin:"linux-no-rustc",properties:properties("linux"),sandbox:"isolated-root"});
 await eventuallyV1(()=>cluster.server.scheduler.status().workers.length===3,15000,"all three workers register");
 const run=async(salt:string,platform:{name:string;value:string}[])=>cluster.execute(await cluster.action({arguments:["/bin/sh","-c","echo ok > out.txt"],outputPaths:["out.txt"],salt,platform}));
 const mac=await Promise.all(Array.from({length:4},(_,index)=>run(`mac-${index}`,wanted("macos","aaaaaaaaaaaaaaaa")))),linux=await Promise.all(Array.from({length:4},(_,index)=>run(`linux-${index}`,wanted("linux","aaaaaaaaaaaaaaaa"))));
 for(const result of mac)expect(workerOf(result)).toBe("mac");
 for(const result of linux)expect(workerOf(result)).toBe("linux");
 expect((await run("windows",wanted("windows","aaaaaaaaaaaaaaaa"))).response.status.code,"nobody offers Windows: refused as unrunnable, the requester compiles locally").not.toBe(0);
 expect((await run("linux-other-build",wanted("linux","cccccccccccccccc"))).response.status.code,"nobody offers that compiler build: refused, the requester compiles locally").not.toBe(0);
 const bare=await run("linux-any-toolchain",[{name:"Arch",value:"arm64"},{name:"OSFamily",value:"linux"}]);
 expect(["linux","linux-no-rustc"]).toContain(workerOf(bare));
});
