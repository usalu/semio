import {afterEach,expect,test} from "bun:test";
import {appendFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import {registeredToolExecutableV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {runRemoteExecutionAgentV1} from "../../🛰️agent/🟦️.ts";
import {remoteExecutionDiskNeedBytesV1,remoteExecutionDiskSlotsV1,remoteExecutionFreeBytesV1} from "../../🛰️agent/💽️disk/🟦️.ts";
import {readRemoteExecutionPolicyV1,requireRemoteExecutionShapeV1} from "../../🟦️.ts";
import {startClusterV1,eventuallyV1,type ClusterV1} from "../🧰️cluster/🟦️.ts";

const bun=registeredToolExecutableV1("bun"),scratch:string[]=[],children:ReturnType<typeof Bun.spawn>[]=[];
let cluster:ClusterV1|undefined;
afterEach(async()=>{for(const child of children.splice(0))child.kill("SIGKILL");await cluster?.stop().catch(()=>{});cluster=undefined;for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const temporary=(label:string):string=>{const path=mkdtempSync(join(testScratchRootV1("rexec-disk"),label+"-"));scratch.push(path);return path;};
const GIB=2**30,disk={reserveGigabytes:3,hostReserveGigabytes:0,perActionGigabytes:0.5},authored={reserveGigabytes:0.5,hostReserveGigabytes:3,perActionGigabytes:0.5};

test("the authored disk policy is the measured one and a worker needs 8 GiB for ten slots, 3.5 GiB for the first",()=>{
 expect(readRemoteExecutionPolicyV1().worker.disk).toEqual(authored);
 expect(remoteExecutionDiskNeedBytesV1(disk,10)).toBe(8*GIB);
 for(const [free,slots] of [[60,10],[8,10],[7.9,9],[5.9,5],[4.2,2],[3.5,1],[3.49,0],[3,0],[0.2,0]] as const)expect(remoteExecutionDiskSlotsV1({freeBytes:free*GIB,concurrency:10,disk}).slots,`${free} GiB`).toBe(slots);
 expect(remoteExecutionDiskSlotsV1({freeBytes:undefined,concurrency:10,disk}).slots,"a volume that cannot be asked does not restrict").toBe(10);
 expect(remoteExecutionDiskSlotsV1({freeBytes:1*GIB,concurrency:4,disk:{reserveGigabytes:0,hostReserveGigabytes:0,perActionGigabytes:0}}).slots).toBe(4);
 expect(remoteExecutionDiskSlotsV1({freeBytes:5.9*GIB,concurrency:10,disk})).toEqual({slots:5,freeBytes:5.9*GIB,needBytes:8*GIB,oneActionBytes:3.5*GIB,volume:"work"});
 const own=remoteExecutionFreeBytesV1(join(testScratchRootV1("rexec-disk"),"does","not","exist"));
 expect(own===undefined||own>0).toBe(true);
});

test("a refusal for lack of disk is a valid task result and the load row of a machine that refuses for it is valid",()=>{
 requireRemoteExecutionShapeV1("TaskResult",{version:1,outcome:"refused",status:{code:8,message:"The machine has no headroom: disk"},actionResult:null,refusal:"disk"},"closed");
 expect(()=>requireRemoteExecutionShapeV1("TaskResult",{version:1,outcome:"refused",status:{code:8,message:"x"},actionResult:null,refusal:"bogus"},"closed")).toThrow();
});

test("the slot rule agrees with an independent implementation on 400 random volumes",()=>{
 let seed=7;const next=():number=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const cases=Array.from({length:400},()=>({free:Math.round(next()*30*GIB),concurrency:1+Math.floor(next()*16),reserve:Math.floor(next()*6*4)/4,per:Math.floor(next()*6+1)/4}));
 const python=spawnSync("python3",["-I","-c","import json,sys,math\nfor c in json.load(sys.stdin):\n print(max(0,min(c['concurrency'],math.floor((c['free']-c['reserve']*2**30)/(c['per']*2**30)))))"],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 const oracle=python.stdout.trim().split("\n").map(Number);
 cases.forEach((c,index)=>expect(remoteExecutionDiskSlotsV1({freeBytes:c.free,concurrency:c.concurrency,disk:{reserveGigabytes:c.reserve,hostReserveGigabytes:0,perActionGigabytes:c.per}}).slots,JSON.stringify(c)).toBe(oracle[index]!));
});

test("the work volume is not charged the machine's reserve: the machine's reserve is asked of the host volume, and the smaller count of the two limits and is the one reported; equal to an independent implementation on 400 random pairs",()=>{
 expect(remoteExecutionDiskSlotsV1({freeBytes:1.5*GIB,hostFreeBytes:40*GIB,concurrency:10,disk:authored}),"a partition image with 1.5 GiB free serves two actions").toEqual({slots:2,freeBytes:1.5*GIB,needBytes:5.5*GIB,oneActionBytes:1*GIB,volume:"work"});
 expect(remoteExecutionDiskSlotsV1({freeBytes:30*GIB,hostFreeBytes:3.4*GIB,concurrency:10,disk:authored})).toEqual({slots:0,freeBytes:3.4*GIB,needBytes:8*GIB,oneActionBytes:3.5*GIB,volume:"host"});
 expect(remoteExecutionDiskSlotsV1({freeBytes:0.4*GIB,hostFreeBytes:40*GIB,concurrency:10,disk:authored}).slots,"a full image still refuses").toBe(0);
 expect(remoteExecutionDiskSlotsV1({freeBytes:undefined,hostFreeBytes:undefined,concurrency:10,disk:authored}).slots).toBe(10);
 let seed=11;const next=():number=>{seed=(seed*1103515245+12345)%2147483648;return seed/2147483648;};
 const cases=Array.from({length:400},()=>({work:Math.round(next()*30*GIB),host:Math.round(next()*30*GIB),concurrency:1+Math.floor(next()*16),reserve:Math.floor(next()*4*4)/4,hostReserve:Math.floor(next()*6*4)/4,per:Math.floor(next()*6+1)/4}));
 const python=spawnSync("python3",["-I","-c","import json,sys,math\ndef s(f,r,p,c):\n return max(0,min(c,math.floor((f-r*2**30)/(p*2**30))))\nfor c in json.load(sys.stdin):\n print(min(s(c['work'],c['reserve'],c['per'],c['concurrency']),s(c['host'],c['hostReserve'],c['per'],c['concurrency'])))"],{input:JSON.stringify(cases),encoding:"utf8"});
 expect(python.status,python.stderr).toBe(0);
 const oracle=python.stdout.trim().split("\n").map(Number);
 cases.forEach((c,index)=>expect(remoteExecutionDiskSlotsV1({freeBytes:c.work,hostFreeBytes:c.host,concurrency:c.concurrency,disk:{reserveGigabytes:c.reserve,hostReserveGigabytes:c.hostReserve,perActionGigabytes:c.per}}).slots,JSON.stringify(c)).toBe(oracle[index]!));
});

test("a worker on a volume that runs short of disk stays registered, says disk-full with the numbers, and serves again within seconds of the space coming back, with no restart",async()=>{
 cluster=await startClusterV1({workers:0,root:temporary("cluster"),scheduler:{refusalCooldownSeconds:1}});
 const root=temporary("agent"),controller=new AbortController();let free=1.2*GIB;
 const host={version:1,atMs:Date.now()+86_400_000,platform:"darwin",cores:8,totalMegabytes:65536,availableMegabytes:60000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:0.5,openFiles:1000,openFilesMaximum:200000,attachedImages:0};
 const agent=runRemoteExecutionAgentV1({root,environment:{SEMIO_UNIT_CACHE_TOOLCHAIN:"rustc 1.0.0-nightly (abcdef 2026-07-20)\nhost: aarch64-apple-darwin\n",SEMIO_FLEET_ORIGIN:"g1-disk"},origin:"g1-disk",scheduler:cluster.server.workerUrl,concurrency:4,directory:join(root,"work"),sandbox:"isolated-root",materialize:"copy",maximumBlobBytes:1048576,inlineOutputBytes:0,loadIntervalSeconds:1,signal:controller.signal,view:async()=>({host,grants:[],waiters:[]}) as never,disk,freeBytes:()=>free,admitRetryMilliseconds:300,actions:"rustc",network:"deny",confinement:"best-effort"});
 const load=async():Promise<any>=>(await fetch(cluster!.server.workerUrl+"/v1/fleet/load")).json();
 const row=async():Promise<any>=>(await load()).machines.find((machine:any)=>machine.origin==="g1-disk");
 const workers=async():Promise<{id:string}[]>=>((await (await fetch(cluster!.server.workerUrl+"/status")).json()) as {workers:{id:string}[]}).workers;
 await eventuallyV1(async()=>(await row())?.worker?.why?.code==="disk-full",30000,"a worker with 1.2 GiB free reports disk-full");
 const short=await row();
 expect(short.worker.state).toBe("refusing");expect(short.headroom).toBe(false);expect(short.worker.why.en).toMatch(/^too little free disk space for the worker: 1\.2 GiB free on \S+, 0 of 4 slots usable, one action needs 3\.5 GiB$/u);expect(short.worker.why.de).toMatch(/^zu wenig freier Speicherplatz für den Worker: 1\.2 GiB frei auf \S+, 0 von 4 Plätzen nutzbar, eine Aktion braucht 3\.5 GiB$/u);
 requireRemoteExecutionShapeV1("FleetLoad",await load(),"closed");
 const before=await workers();expect(before.length,"the worker is registered while it refuses for lack of disk").toBe(1);
 free=40*GIB;
 await eventuallyV1(async()=>{const now=await row();return now?.worker?.state==="idle"&&now.worker.why===undefined&&now.headroom===true;},30000,"the worker serves again once the space is there");
 const after=await workers();expect(after.map(worker=>worker.id),"the same registration: nothing restarted").toEqual(before.map(worker=>worker.id));
 free=3.2*GIB;
 await eventuallyV1(async()=>(await row())?.worker?.why?.code==="disk-full",30000,"and refuses again when the space goes");
 controller.abort();await agent.catch(()=>undefined);
},120000);

const launcher=(directory:string,entryBody:string,restart:string,extraEnvironment:Record<string,string>={})=>{
 const entry=join(directory,"entry.ts"),runner=join(directory,"runner.ts"),sidecar=join(import.meta.dir,"..","..","🛰️sidecar","🟦️.ts");
 writeFileSync(entry,entryBody);
 writeFileSync(runner,`import {runRemoteExecutionSidecarLauncherV1} from ${JSON.stringify(sidecar)};\nawait runRemoteExecutionSidecarLauncherV1("agent",${JSON.stringify(entry)},process.env,${restart});\n`);
 const child=Bun.spawn([bun,runner],{stdout:"pipe",stderr:"pipe",cwd:directory,env:{...process.env,...extraEnvironment}});children.push(child);
 return {child,entry,log:join(directory,"starts.log"),stderr:async():Promise<string>=>new Response(child.stderr as ReadableStream).text()};
};
const parkAndLeaveOnTerm=`process.on("SIGTERM",()=>process.exit(0));setInterval(()=>{},1000);`;

test("a role that crashes is started again by the launcher after a short back-off, and comes back once what it needed is there again; the launcher itself ends only when told to",async()=>{
 const directory=temporary("launcher"),log=join(directory,"starts.log"),late=join(directory,"late.ts");
 const body=`import {appendFileSync} from "node:fs";appendFileSync(${JSON.stringify(log)},"start\\n");await import(${JSON.stringify(late)});appendFileSync(${JSON.stringify(log)},"serving\\n");${parkAndLeaveOnTerm}\n`;
 const run=launcher(directory,body,"{initialMilliseconds:50,maximumMilliseconds:200,stableMilliseconds:60000}");
 await eventuallyV1(()=>existsSync(log)&&readFileSync(log,"utf8").split("\n").filter(line=>line==="start").length>=4,20000,"the child is started again and again while its module is missing");
 expect(readFileSync(log,"utf8")).not.toContain("serving");
 writeFileSync(late,"export const ready=true;\n");
 await eventuallyV1(()=>readFileSync(log,"utf8").includes("serving"),20000,"the role is back within moments of the module returning");
 const alive=await Promise.race([run.child.exited,Bun.sleep(500).then(()=>"running")]);expect(alive,"the launcher survived the crashes").toBe("running");
 run.child.kill("SIGTERM");expect(await run.child.exited,"told to stop, the launcher ends cleanly").toBe(0);
 const text=await run.stderr();
 expect(text).toMatch(/agent ended with status 1, starting again in 0\.05 s/u);expect(text).toMatch(/Cannot find module/u);
},60000);

test("the back-off doubles up to its maximum, a stop during a back-off ends the launcher at once, and a code change (status 75) restarts without a back-off",async()=>{
 const directory=temporary("backoff"),log=join(directory,"starts.log");
 const crash=`import {appendFileSync} from "node:fs";appendFileSync(${JSON.stringify(log)},Date.now()+"\\n");process.exit(1);\n`;
 const first=launcher(directory,crash,"{initialMilliseconds:100,maximumMilliseconds:400,stableMilliseconds:60000}");
 await eventuallyV1(()=>existsSync(log)&&readFileSync(log,"utf8").split("\n").filter(Boolean).length>=5,20000,"five starts");
 const stamps=readFileSync(log,"utf8").split("\n").filter(Boolean).map(Number),gaps=stamps.slice(1).map((value,index)=>value-stamps[index]!);
 expect(gaps[0]!,"first gap about the initial back-off").toBeLessThan(350);expect(Math.max(...gaps),"no gap beyond the maximum plus the start time").toBeLessThan(1500);expect(gaps[3]!,"the later gaps are the maximum").toBeGreaterThan(300);
 first.child.kill("SIGTERM");expect(await first.child.exited).toBe(0);
 rmSync(log,{force:true});
 const slow=launcher(directory,crash,"{initialMilliseconds:30000,maximumMilliseconds:30000,stableMilliseconds:60000}");
 await eventuallyV1(()=>existsSync(log),20000,"started once");await Bun.sleep(300);
 const stopped=Date.now();slow.child.kill("SIGTERM");expect(await slow.child.exited).toBe(0);expect(Date.now()-stopped,"a stop is not held up by the back-off").toBeLessThan(3000);
 rmSync(log,{force:true});
 const changed=launcher(directory,`import {appendFileSync} from "node:fs";appendFileSync(${JSON.stringify(log)},Date.now()+"\\n");if(readFileSync(${JSON.stringify(log)},"utf8").split("\\n").filter(Boolean).length<3)process.exit(75);${parkAndLeaveOnTerm}\nimport {readFileSync} from "node:fs";\n`,"{initialMilliseconds:30000,maximumMilliseconds:30000,stableMilliseconds:60000}");
 await eventuallyV1(()=>existsSync(log)&&readFileSync(log,"utf8").split("\n").filter(Boolean).length>=3,15000,"status 75 restarts quickly, whatever the back-off");
 changed.child.kill("SIGTERM");expect(await changed.child.exited).toBe(0);
},90000);
