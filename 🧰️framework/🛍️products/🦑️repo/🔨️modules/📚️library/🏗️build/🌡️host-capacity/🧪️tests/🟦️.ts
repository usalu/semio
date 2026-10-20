import {afterAll,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {existsSync,mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../🔣️policy.json" with {type:"json"};
import catalog from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {BUILD_HOST_CAPACITY_POLICY_V1,buildCapacityAskV1,createCapacityControlV1,grantedBuildArgumentsV1,withBuildCapacityV1} from "../🟦️.ts";
import {runCapacityCommandV1} from "../⌨️command/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hb-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});

test("The authored host capacity policy is schema-valid and covers exactly the gated catalogue operations plus the ad-hoc command block",()=>{
 expect(BUILD_HOST_CAPACITY_POLICY_V1).toEqual(readHostCapacityPolicyV1(buildPolicy.hostCapacity));
 const names=new Set(catalog.operations.map(row=>row.name));
 for(const row of BUILD_HOST_CAPACITY_POLICY_V1.operations)expect(names.has(row.name)||row.name==="command",row.name).toBe(true);
 for(const light of ["tools","reset","cache","preflight","capacity"])expect(BUILD_HOST_CAPACITY_POLICY_V1.operations.some(row=>row.name===light),light).toBe(false);
 for(const heavy of ["run","run-many","affected","exec","watch","cargo","show","graph","dependencies"])expect(BUILD_HOST_CAPACITY_POLICY_V1.operations.some(row=>row.name===heavy),heavy).toBe(true);
});

test("Invocations ask for the operation, the jobs and the kinds of their targets, and light commands ask for nothing",()=>{
 for(const item of fixture.asks)expect(buildCapacityAskV1(item.args,item.environment)??null,item.name).toEqual(item.expect as never);
});

test("A grant rewrites the parallelism option and never touches what follows the separator",()=>{
 for(const item of fixture.rewrites)expect(grantedBuildArgumentsV1(item.args,item.operation,item.jobs),item.args.join(" ")).toEqual(item.expect);
});

test("A heavy invocation runs under a reduced grant with rewritten arguments and inherited environment, then restores the environment",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={SEMIO_BUILD_PARALLEL:"10"},lines:string[]=[];
 const result=await withBuildCapacityV1(["--tool=bazel","run","@x/y:typecheck","--parallel=10"],environment,async granted=>{
  expect(granted.handle).toMatchObject({jobs:2,inherited:false});
  expect(granted.args).toEqual(["--tool=bazel","run","@x/y:typecheck","--parallel=2"]);
  expect(environment.SEMIO_BUILD_PARALLEL).toBe("2");
  expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBe(granted.handle!.id);
  expect(JSON.parse(environment.SEMIO_HOST_CAPACITY_PLAN!)).toMatchObject({jobs:2,cpu:2,memoryMegabytes:8192,fixedMegabytes:1536});
  expect(readdirSync(join(directory,"grants")).length).toBe(1);
  return "done";
 },{directory,policy:fast,strip,write:line=>lines.push(line),sample:async()=>({host:host(),swappedOutMegabytes:0})});
 expect(result).toBe("done");
 expect(environment.SEMIO_BUILD_PARALLEL).toBe("10");
 expect(environment.SEMIO_HOST_CAPACITY_GRANT).toBeUndefined();
 expect(environment.SEMIO_HOST_CAPACITY_PLAN).toBeUndefined();
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 expect(lines.at(-1)).toMatch(/admitted run with 2 of 10 job\(s\)/u);
});

test("Light commands and fixed-block operations leave the arguments and the parallelism alone, light ones without touching the gate",async()=>{
 const directory=join(fresh(),"gate"),environment:NodeJS.ProcessEnv={SEMIO_BUILD_PARALLEL:"7"};
 await withBuildCapacityV1(["cache","status"],environment,async granted=>{expect(granted).toEqual({args:["cache","status"],handle:undefined});},{directory,policy:fast,strip});
 expect(existsSync(directory)).toBe(false);
 await withBuildCapacityV1(["show","projects"],environment,async granted=>{expect(granted.args).toEqual(["show","projects"]);expect(environment.SEMIO_BUILD_PARALLEL).toBe("7");expect(granted.handle).toMatchObject({jobs:1,reservedMegabytes:5120});},{directory,policy:fast,strip,sample:async()=>({host:host(),swappedOutMegabytes:0})});
});

test("A nested invocation inherits the grant of its parent through the environment and waits for nothing",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={};
 await withBuildCapacityV1(["run","@x/y:build","--parallel=2"],environment,async outer=>{
  const nested:NodeJS.ProcessEnv={...environment};
  await withBuildCapacityV1(["run","@x/z:build","--parallel=2"],nested,async inner=>expect(inner.handle).toMatchObject({inherited:true,id:outer.handle!.id}),{directory,policy:fast,strip,sample:async()=>({host:host({availableMegabytes:7000}),swappedOutMegabytes:0})});
 },{directory,policy:fast,strip,sample:async()=>({host:host(),swappedOutMegabytes:0})});
});

test("An interrupt while waiting cancels the wait and leaves nothing in the queue",async()=>{
 const directory=fresh(),environment:NodeJS.ProcessEnv={},emitter=new EventEmitter();
 const sample=async()=>({host:host({availableMegabytes:6600}),swappedOutMegabytes:0}),control=createCapacityControlV1(emitter as never);
 const waiting=withBuildCapacityV1(["run","@x/y:build","--parallel=2"],environment,async()=>"never",{directory,policy:fast,strip,sample,control:control.control,write:()=>undefined});
 await sleep(80);
 expect(readdirSync(join(directory,"queue")).length).toBe(1);
 emitter.emit("SIGINT");
 await expect(waiting).rejects.toThrow("Host capacity wait cancelled by SIGINT");
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
 control.dispose();
 expect(emitter.listenerCount("SIGINT")).toBe(0);
});

test("repo capacity status prints one line per fact in English and German and a JSON snapshot on request",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],json:string[]=[];
 const sample=async()=>({host:host(),swappedOutMegabytes:0}),context=(environment:NodeJS.ProcessEnv,write:(line:string)=>void)=>({environment,write,directory,policy:fast,sample});
 await withBuildCapacityV1(["run","@x/y:build","--parallel=2"],{},async()=>{
  expect(await runCapacityCommandV1(["status"],context({LANG:"en_US.UTF-8"},line=>english.push(line)))).toBe(0);
  expect(await runCapacityCommandV1(["status"],context({LANG:"de_DE.UTF-8"},line=>german.push(line)))).toBe(0);
  expect(await runCapacityCommandV1(["status","--json"],context({},line=>json.push(line)))).toBe(0);
 },{directory,policy:fast,strip,sample:async()=>({host:host(),swappedOutMegabytes:0}),pid:process.pid});
 expect(english[0]).toMatch(/^\[capacity\] \d+ MB available of \d+ MB \(floor \d+ MB, -?\d+ MB spare\), pressure \w+, swap \S+ MB, load \S+, open files \S+, attached images \S+; 1 running, 0 waiting$/u);
 expect(english[1]).toMatch(/^\[capacity\]   running \S+ run: 2 job\(s\), \d+ MB reserved, \S+ MB observed, pid \d+, since \d+s$/u);
 expect(german[0]).toMatch(/verfügbar von .* laufen, 0 warten$/u);
 expect(german[1]).toContain("läuft");
 expect(JSON.parse(json.join("\n"))).toMatchObject({grants:[{operation:"run"}],waiters:[]});
});

test("repo capacity run admits an arbitrary command, hands it the grant and returns its exit code",async()=>{
 const directory=fresh(),lines:string[]=[],context={environment:{...process.env,LANG:"en_US.UTF-8"} as NodeJS.ProcessEnv,write:(line:string)=>lines.push(line),directory,policy:fast,sample:async()=>({host:host(),swappedOutMegabytes:0})};
 expect(await runCapacityCommandV1(["run","--memory=100","--","node","-e","process.exit(process.env.SEMIO_HOST_CAPACITY_GRANT&&JSON.parse(process.env.SEMIO_HOST_CAPACITY_PLAN).fixedMegabytes===100?0:3)"],context)).toBe(0);
 expect(await runCapacityCommandV1(["run","--memory=100","--","node","-e","process.exit(7)"],context)).toBe(7);
 expect(lines.some(line=>/admitted command: 1 job\(s\), 100 MB reserved/u.test(line))).toBe(true);
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 await expect(runCapacityCommandV1(["run","--memory=100"],context)).rejects.toThrow("requires a command");
 await expect(runCapacityCommandV1(["run","--memory=0","--","true"],context)).rejects.toThrow("positive integer");
 await expect(runCapacityCommandV1(["run","--operation=nonsense","--","true"],context)).rejects.toThrow("Unknown capacity operation");
 await expect(runCapacityCommandV1(["frobnicate"],context)).rejects.toThrow("Expected repo capacity status|run");
});
