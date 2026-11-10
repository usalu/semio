import {spawnSync} from "node:child_process";
import {test,expect} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync,existsSync,rmdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {JOB_LIMIT_RULES_V1,createFakeJobLimiterV1,jobBreachLearnedPeakV1,jobEstimateV1,jobPolicyForReservationV1,jobPolicyV1,renderEnforcementFallbackV1,renderJobOutcomeV1,runLimitedJobV1,type JobOutcomeV1} from "../🟦️.ts";
import {createCgroupLimiterV1,cgroupPortsV1,detectCgroupV2V1,parseCgroupKeysV1,placeServiceOutsideJobsV1,cgroupMemoryHeadroomV1,releaseOwnServiceGroupV1,sweepEmptyServiceGroupsV1,serviceGroupDirectoryV1,serviceLaunchArgumentsV1,prepareServiceGroupV1,type CgroupPortsV1} from "../🐧️cgroup/🟦️.ts";
import {createWatchdogLimiterV1,parseProcessRowsV1,treeMegabytesV1} from "../🐕️watchdog/🟦️.ts";
import {createJobObjectLimiterV1} from "../🪟️job-object/🟦️.ts";
import {grantJobLimitV1,learnedRunAfterOutcomeV1} from "../🎫️admission/🟦️.ts";
import {selectJobLimiterV1} from "../🎚️select/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import texts from "../📖️texts/🔣️.json";
import fixture from "./🔣️.json";

const python=testPythonV1()!,oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),allocator=fileURLToPath(new URL("./🐍️allocator.py",import.meta.url));
const model=(mode:string,cases:unknown):number[]=>{const done=Bun.spawnSync([python,"-I",oracle,mode,JSON.stringify(cases)]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout));};
const allocate=(allocation:{megabytes:number;stepMegabytes:number;holdMilliseconds:number})=>[allocator,String(allocation.megabytes),String(allocation.stepMegabytes),String(allocation.holdMilliseconds)];
const available=():number=>Number(/MemAvailable:\s+(\d+)/u.exec(readFileSync("/proc/meminfo","utf8"))![1])/1024;

test("the fixture and the texts are closed, German mirrors English",()=>{
 const ajv=new Ajv({strict:false});
 for(const row of fixture.cases)expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Fixture"},row),JSON.stringify(ajv.errors)).toBe(true);
 const holders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]!).sort();
 for(const [key,leaf] of Object.entries(texts.messages)){expect(Object.keys(leaf)).toEqual(["en","de"]);expect(holders(leaf.de),key).toEqual(holders(leaf.en));}
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Policy"},jobPolicyV1(100))).toBe(true);
});

test("the limit of a class equals the independent model",()=>{
 const expected=model("limit",fixture.estimates);
 fixture.estimates.forEach((row,index)=>expect(jobEstimateV1(row.class,row.floorMegabytes,(row as {learnedPeakMegabytes?:number}).learnedPeakMegabytes).limitMegabytes,row.class).toBe(expected[index]!));
});

test("a killed job teaches a larger peak, equal to the independent model",()=>{
 const expected=model("learned",fixture.learned);
 fixture.learned.forEach((row,index)=>expect(jobBreachLearnedPeakV1(row as never),JSON.stringify(row)).toBe(expected[index]!));
});

test("the tree memory equals the independent model",()=>{
 const expected=model("tree",fixture.trees);
 fixture.trees.forEach((row,index)=>expect(treeMegabytesV1(row.rows.map(item=>({...item})),row.root)).toBe(expected[index]!));
 expect(parseProcessRowsV1("  10 1 2048\nbad\n11 10 1024\n")).toEqual([{pid:10,ppid:1,kilobytes:2048},{pid:11,ppid:10,kilobytes:1024}]);
});

test("a reservation is the limit of its jobs",()=>{
 expect(jobPolicyForReservationV1(4000,4).memoryMegabytes).toBe(1000);
 expect(jobPolicyForReservationV1(10,1).memoryMegabytes).toBe(JOB_LIMIT_RULES_V1.minimumLimitMegabytes);
 expect(jobPolicyV1(100).swapMegabytes).toBe(0);
});

test("the typed failure renders in English and German and never without a language",()=>{
 const outcome:JobOutcomeV1={kind:"memory-exceeded",enforcement:"cgroup-v2",limitMegabytes:160,peakMegabytes:161,exitCode:null,signal:"SIGKILL",durationMilliseconds:10};
 for(const language of ["en","de"] as const){const text=renderJobOutcomeV1(outcome,"tsc",language)!;expect(text).toContain("160");expect(text).toContain("161");expect(text).toContain("tsc");}
 expect(renderJobOutcomeV1(outcome,"tsc","en")).not.toBe(renderJobOutcomeV1(outcome,"tsc","de"));
 expect(renderJobOutcomeV1({...outcome,kind:"ok"},"tsc","en")).toBeUndefined();
 expect(()=>renderJobOutcomeV1(outcome,"tsc","" as never)).toThrow();
 expect(renderEnforcementFallbackV1("tsc",160,"de")).toContain("Wächter");
});

test("the fake limiter drives the runner: an overrun is a typed outcome, a fit is ok, a cancel is cancelled",async()=>{
 const base={job:"fake",command:python,args:["-I","-c","import time;time.sleep(0.3)"],stdio:"ignore" as const};
 expect((await runLimitedJobV1({...base,policy:jobPolicyV1(100),limiter:createFakeJobLimiterV1({peakMegabytes:150})})).kind).toBe("memory-exceeded");
 const fit=await runLimitedJobV1({...base,args:["-I","-c","pass"],policy:jobPolicyV1(100),limiter:createFakeJobLimiterV1({peakMegabytes:50})});
 expect(fit.kind).toBe("ok");expect(fit.peakMegabytes).toBe(50);
 const controller=new AbortController(),running=runLimitedJobV1({...base,args:["-I","-c","import time;time.sleep(30)"],policy:jobPolicyV1(100),limiter:createFakeJobLimiterV1({peakMegabytes:1}),signal:controller.signal});
 setTimeout(()=>controller.abort(),100);
 expect((await running).kind).toBe("cancelled");
 expect((await runLimitedJobV1({...base,args:["-I","-c","raise SystemExit(3)"],policy:jobPolicyV1(100),limiter:createFakeJobLimiterV1({peakMegabytes:1})})).exitCode).toBe(3);
});

function fakeKernel():{ports:CgroupPortsV1;root:string;files:Map<string,string>}{
 const root=mkdtempSync(join(tmpdir(),"jl-")),files=new Map<string,string>(),dirs=new Set<string>([root]);
 const ports:CgroupPortsV1={root,read:path=>{const text=files.get(path);if(text===undefined)throw Object.assign(Error("ENOENT"),{code:"ENOENT"});return text;},write:(path,text)=>{files.set(path,text);},mkdir:path=>{if(dirs.has(path))throw Object.assign(Error("EEXIST"),{code:"EEXIST"});dirs.add(path);},rmdir:path=>{dirs.delete(path);},list:path=>[...dirs].filter(row=>row.startsWith(path+"/")).map(row=>row.slice(path.length+1)),writable:path=>path.includes("user@"),own:()=>"/user.slice/user@1/app.slice/tab.scope",sleep:()=>Promise.resolve(),pid:42};
 files.set(join(root,"cgroup.controllers"),"cpuset cpu io memory pids\n");
 return {ports,root,files};
}

test("the cgroup limiter writes the limits, detects the oom kill and removes its cgroup",async()=>{
 const {ports,root,files}=fakeKernel();
 files.set(join(root,"user.slice/user@1/cgroup.subtree_control"),"cpu memory pids\n");
 const base=detectCgroupV2V1(ports)!;
 expect(base).toBe(join(root,"user.slice/user@1",JOB_LIMIT_RULES_V1.cgroupDirectoryName));
 expect(files.get(join(base,"cgroup.subtree_control"))).toBe("+pids");
 const handle=await createCgroupLimiterV1(base,ports).create(jobPolicyV1(100,{pids:50,cpuShares:200}),"job");
 const directory=join(base,"job-42-1"),mb=1048576;
 expect(files.get(join(directory,"memory.max"))).toBe(String(100*mb));
 expect(files.get(join(directory,"memory.swap.max"))).toBe("0");
 expect(files.get(join(directory,"memory.high"))).toBeUndefined();
 expect(files.get(join(directory,"memory.oom.group"))).toBe("1");
 expect(files.get(join(directory,"pids.max"))).toBe("50");
 expect(files.get(join(directory,"cpu.weight"))).toBe("200");
 const service=await createCgroupLimiterV1(base,ports).create(jobPolicyV1(100,{kind:"service"}),"daemon");
 expect(files.get(join(base,"job-42-2","memory.high"))).toBe(String(90*mb));await service.release();
 await handle.attach(7);expect(files.get(join(directory,"cgroup.procs"))).toBe("7");
 let breach:unknown;handle.onBreach(found=>{breach=found;});
 files.set(join(directory,"memory.current"),String(60*mb));files.set(join(directory,"memory.peak"),String(99*mb));
 files.set(join(directory,"memory.events"),"low 0\nhigh 3\nmax 12\noom 1\noom_kill 1\n");
 const usage=await handle.usage();
 expect(usage.peakMegabytes).toBe(99);expect(breach).toEqual({peakMegabytes:100,limitMegabytes:100});
 const final=await handle.release();
 expect(final).toEqual({peakMegabytes:99,breached:true});expect(files.get(join(directory,"cgroup.kill"))).toBe("1");
 expect(parseCgroupKeysV1("a 1\nb 2\n")).toEqual({a:1,b:2});
 rmSync(root,{recursive:true,force:true});
});

test("a service started inside a job is placed in its own cgroup beside the jobs, with the service limits",()=>{
 const {ports,root,files}=fakeKernel();
 files.set(join(root,"user.slice/user@1/cgroup.subtree_control"),"cpu memory pids\n");
 expect(placeServiceOutsideJobsV1(4242,{name:"graph",limitMegabytes:3072,softRatioBasisPoints:9000},ports)).toBe(true);
 const directory=join(root,"user.slice/user@1","semio-service-graph"),mb=1048576;
 expect(files.get(join(directory,"cgroup.procs"))).toBe("4242");
 expect(files.get(join(directory,"memory.max"))).toBe(String(3072*mb));
 expect(files.get(join(directory,"memory.high"))).toBe(String(Math.floor(3072*mb*0.9)));
 expect(files.get(join(directory,"memory.swap.max"))).toBe("0");
 expect(directory.startsWith(join(root,"user.slice/user@1",JOB_LIMIT_RULES_V1.cgroupDirectoryName))).toBe(false);
 expect(placeServiceOutsideJobsV1(1,{name:"graph",limitMegabytes:1,softRatioBasisPoints:9000},{...ports,read:()=>{throw Error("none");}})).toBe(false);
 rmSync(root,{recursive:true,force:true});
});

test("the headroom is the smallest memory.max up the tree and the current bytes of the own cgroup",()=>{
 const {ports,root,files}=fakeKernel(),own=ports.own().split("/").filter(Boolean),mb=1048576;
 expect(cgroupMemoryHeadroomV1(ports)).toBeUndefined();
 files.set(join(root,...own,"memory.current"),String(700*mb));
 expect(cgroupMemoryHeadroomV1(ports)).toEqual({limitBytes:Infinity,currentBytes:700*mb});
 files.set(join(root,...own,"memory.max"),String(4096*mb));files.set(join(root,...own.slice(0,-1),"memory.max"),String(3000*mb));files.set(join(root,"memory.max"),"max");
 expect(cgroupMemoryHeadroomV1(ports)).toEqual({limitBytes:own.length>1?3000*mb:4096*mb,currentBytes:700*mb});
 rmSync(root,{recursive:true,force:true});
});

test("no delegated cgroup means no cgroup limiter",()=>{
 const {ports,root}=fakeKernel();
 expect(detectCgroupV2V1(ports)).toBeUndefined();
 expect(detectCgroupV2V1({...ports,read:()=>{throw Error("none");}})).toBeUndefined();
 const choice=selectJobLimiterV1(jobPolicyV1(100),{platform:"linux",cgroup:ports,watchdog:{table:()=>[],kill:()=>undefined}});
 expect(choice.limiter.enforcement).toBe("watchdog");expect(choice.fellBack).toBe(true);
 expect(selectJobLimiterV1(jobPolicyV1(100,{enforcement:"watchdog"}),{platform:"linux"}).fellBack).toBe(false);
 expect(selectJobLimiterV1(jobPolicyV1(100),{platform:"darwin"}).limiter.enforcement).toBe("watchdog");
 expect(selectJobLimiterV1(jobPolicyV1(100),{platform:"win32",helper:()=>undefined}).limiter.enforcement).toBe("watchdog");
 expect(selectJobLimiterV1(jobPolicyV1(100),{platform:"win32",helper:()=>process.execPath}).limiter.enforcement).toBe("job-object");
 rmSync(root,{recursive:true,force:true});
});

test("the watchdog kills the tree at the limit and reports the peak",async()=>{
 const policy=jobPolicyV1(200,{enforcement:"watchdog"}),before=available(),started=Date.now();
 const outcome=await runLimitedJobV1({job:"watchdog",policy,limiter:selectJobLimiterV1(policy).limiter,command:python,args:["-I",...allocate({megabytes:1024,stepMegabytes:8,holdMilliseconds:50})],stdio:"ignore"});
 expect(outcome.kind).toBe("memory-exceeded");expect(outcome.enforcement).toBe("watchdog");
 expect(outcome.peakMegabytes).toBeGreaterThanOrEqual(200);expect(Date.now()-started).toBeLessThan(3000);
 expect(available()).toBeGreaterThan(before-1536);
 const fit=await runLimitedJobV1({job:"fit",policy,limiter:createWatchdogLimiterV1(),command:python,args:["-I",...allocate({megabytes:16,stepMegabytes:8,holdMilliseconds:20})],stdio:"ignore"});
 expect(fit.kind).toBe("ok");
});

test.if(process.platform==="linux"&&detectCgroupV2V1()!==undefined)("a real cgroup stops an allocator at its limit within 500 ms of reaching it while a sibling survives",async()=>{
 const policy=jobPolicyV1(160),choice=selectJobLimiterV1(policy),sibling=Bun.spawn([python,"-I","-c","import time;time.sleep(30)"],{stdio:["ignore","ignore","ignore"]});
 expect(choice.limiter.enforcement).toBe("cgroup-v2");
 const floor=available(),started=Date.now(),fixtureCase=fixture.cases[1]!;
 const outcome=await runLimitedJobV1({job:"cgroup",policy,limiter:choice.limiter,command:python,args:["-I",...allocate(fixtureCase.allocate)],stdio:"ignore"});
 expect(outcome.kind).toBe("memory-exceeded");expect(outcome.enforcement).toBe("cgroup-v2");
 expect(outcome.signal).toBe("SIGKILL");expect(outcome.peakMegabytes).toBeLessThanOrEqual(policy.memoryMegabytes);
 expect(Date.now()-started).toBeLessThan(fixtureCase.expect.killedWithinMilliseconds!);
 expect(sibling.killed||sibling.exitCode!==null).toBe(false);sibling.kill();
 expect(available()).toBeGreaterThan(floor-policy.memoryMegabytes-64);
 const fit=await runLimitedJobV1({job:"cgroup-fit",policy,limiter:choice.limiter,command:python,args:["-I",...allocate(fixture.cases[0]!.allocate)],stdio:"ignore"});
 expect(fit.kind).toBe("ok");expect(fit.peakMegabytes).toBeGreaterThan(0);
});

test("the job object limiter speaks the helper frames and reports a breach",async()=>{
 let closed=false;const args:string[][]=[];
 const limiter=createJobObjectLimiterV1({start:given=>{args.push([...given]);async function* lines(){yield '{"event":"ready"}';yield '{"event":"usage","peakMegabytes":40}';yield "not json";yield '{"event":"breach","peakMegabytes":101}';}return {lines:lines(),close:()=>{closed=true;},done:Promise.resolve()};}});
 const handle=await limiter.create(jobPolicyV1(100,{pids:9}),"job");let breach:unknown;handle.onBreach(found=>{breach=found;});
 await handle.attach(1234);const final=await handle.release();
 expect(args[0]).toEqual(["--attach","1234","--memory-mb","100","--pids","9","--poll-ms",String(JOB_LIMIT_RULES_V1.cgroupPollMilliseconds)]);
 expect(breach).toEqual({peakMegabytes:101,limitMegabytes:100});expect(final).toEqual({peakMegabytes:101,breached:true});expect(closed).toBe(true);
});

test("admission and limit are one number and a killed job raises the next reservation",()=>{
 expect(grantJobLimitV1({reservedMegabytes:3000}).memoryMegabytes).toBe(3000);
 const killed:JobOutcomeV1={kind:"memory-exceeded",enforcement:"cgroup-v2",limitMegabytes:1000,peakMegabytes:1000,exitCode:null,signal:"SIGKILL",durationMilliseconds:1};
 expect(learnedRunAfterOutcomeV1({peakMegabytes:900,status:"ok"},killed)).toEqual({peakMegabytes:1250,status:"failed"});
 expect(learnedRunAfterOutcomeV1({peakMegabytes:900,status:"ok"},{...killed,kind:"ok"})).toEqual({peakMegabytes:900,status:"ok"});
 expect(learnedRunAfterOutcomeV1({peakMegabytes:900,status:"cancelled"},killed).status).toBe("cancelled");
 expect(learnedRunAfterOutcomeV1({peakMegabytes:900,status:"ok"},undefined).peakMegabytes).toBe(900);
});

test("a service that ends leaves its cgroup, and empty service cgroups of ended services are swept while a group that holds a process stays",()=>{
 const {ports,root,files}=fakeKernel();
 files.set(join(root,"user.slice/user@1/cgroup.subtree_control"),"cpu memory pids\n");
 const parent=join(root,"user.slice/user@1"),place=(name:string,pid:number)=>placeServiceOutsideJobsV1(pid,{name,limitMegabytes:3072,softRatioBasisPoints:9000},ports);
 expect(serviceGroupDirectoryV1("graph-a",ports)).toBe(join(parent,"semio-service-graph-a"));
 place("graph-a",1);place("graph-b",2);place("graph-c",3);
 files.set(join(parent,"semio-service-graph-a/cgroup.procs"),"");files.set(join(parent,"semio-service-graph-b/cgroup.procs"),"2\n");files.set(join(parent,"semio-service-graph-c/cgroup.procs"),"");
 expect(sweepEmptyServiceGroupsV1("graph-",join(parent,"semio-service-graph-c"),ports),"empty groups go, the kept and the occupied stay").toEqual(["semio-service-graph-a"]);
 expect(ports.list(parent).filter(name=>name.startsWith("semio-service-")).sort()).toEqual(["semio-service-graph-b","semio-service-graph-c"]);
 expect(releaseOwnServiceGroupV1(ports),"the process is not inside a service group").toBe(false);
 const own={...ports,own:()=>"/user.slice/user@1/semio-service-graph-b"};
 expect(releaseOwnServiceGroupV1(own)).toBe(true);
 expect(files.get(join(parent,"cgroup.procs"))).toBe("42");
 expect(ports.list(parent).filter(name=>name==="semio-service-graph-b")).toEqual([]);
 rmSync(root,{recursive:true,force:true});
});

test("a service is born inside its own cgroup: the launcher writes its pid to the group before it becomes the service, and the group is prepared with the service limits first",()=>{
 const {ports,root,files}=fakeKernel();
 files.set(join(root,"user.slice/user@1/cgroup.subtree_control"),"cpu memory pids\n");
 const directory=prepareServiceGroupV1({name:"graph-x",limitMegabytes:3072,softRatioBasisPoints:9000},ports)!,mb=1048576;
 expect(directory).toBe(join(root,"user.slice/user@1","semio-service-graph-x"));
 expect(files.get(join(directory,"memory.max"))).toBe(String(3072*mb));expect(files.get(join(directory,"memory.oom.group"))).toBe("1");expect(files.get(join(directory,"cgroup.procs"))).toBeUndefined();
 if(process.platform==="win32")return;
 const dir=mkdtempSync(join(tmpdir(),"launcher-")),procs=join(dir,"cgroup.procs"),argv=serviceLaunchArgumentsV1(procs,[process.execPath,"-e","console.log(process.pid)"]);
 try{
  const run=spawnSync(argv[0]!,argv.slice(1),{encoding:"utf8"});
  expect(run.status).toBe(0);
  expect(readFileSync(procs,"utf8").trim(),"the pid the group recorded is the pid of the service (the shell exec'd it)").toBe(run.stdout.trim());
  const refused=serviceLaunchArgumentsV1(join(dir,"missing","cgroup.procs"),[process.execPath,"-e","console.log('started')"]),started=spawnSync(refused[0]!,refused.slice(1),{encoding:"utf8"});
  expect(started.stdout.trim(),"a refused placement still starts the service").toBe("started");
 }finally{rmSync(dir,{recursive:true,force:true});rmSync(root,{recursive:true,force:true});}
});
