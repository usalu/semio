/** 🦀️ Contract of the Cargo grant: an ad-hoc Cargo invocation of the tool registry is admitted into the one machine-wide ledger, sized by what its subcommand and package really peaked at, limited in jobs by the memory that is left, waits with cancellation and teaches its class only when it succeeded.
 * @see ../../🦀️cargo-grant/🧬️schema/🔣️.json
 * @see ./🐍️oracle/🐍️.py */
import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {setTimeout as sleep} from "node:timers/promises";
import {estimateHostRatioV1,readHostCapacityPolicyV1,readHostEstimateStoreV1,type HostSampleV1} from "../../🟦️.ts";
import {createToolAdmissionV1} from "../../../../🧰️tools/🏃️run/🟦️.ts";
import {CARGO_GRANT_POLICY_V1,capCargoGrantJobsV1,cargoGrantEnvironmentV1,cargoGrantJobsV1,cargoGrantLineageV1,cargoHostTripleV1,cargoPinnedToolchainV1,readCargoPinnedToolchainV1}from"../../🦀️cargo-grant/🟦️.ts";
import {testPythonV1}from"../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import schema from"../../🦀️cargo-grant/🧬️schema/🔣️.json";
import decision from"../🧫️fixtures/🔣️.json";
import cases from"./🧫️fixtures/🔣️.json";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"cg-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1({...decision.policy,wait:{...decision.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const oracle=fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url));
const python=(input:unknown):any=>{const result=spawnSync(testPythonV1()??"python3",["-I",oracle],{input:JSON.stringify(input),encoding:"utf8",maxBuffer:1<<26});expect(result.status,result.stderr).toBe(0);return JSON.parse(result.stdout);};
const triple="aarch64-apple-darwin";

test("the fixtures are valid against the schema",()=>{
 const ajv=new Ajv({strict:false});ajv.addSchema(schema,"grant");
 const check=(definition:string,rows:readonly unknown[])=>{const validate=ajv.getSchema(`grant#/definitions/${definition}`)!;for(const row of rows)expect(validate(row),JSON.stringify(validate.errors)).toBe(true);};
 check("LineageCase",cases.lineage);check("JobsCase",cases.jobs);check("CapCase",cases.caps);check("Toolchain",cases.toolchains);
});

test("lineage, jobs, caps and toolchains are written down exactly",()=>{
 for(const row of cases.lineage)expect(cargoGrantLineageV1(row.arguments),row.name).toEqual(row.lineage);
 for(const row of cases.jobs)expect(cargoGrantJobsV1(row.arguments,row.environment,row.cores),row.name).toBe(row.jobs);
 for(const row of cases.caps)expect(capCargoGrantJobsV1(row.arguments,row.granted),row.name).toEqual(row.capped);
 for(const row of cases.toolchains)expect(cargoPinnedToolchainV1(row.file,row.triple),row.name).toBe(row.toolchain);
});

test("every case and a generated grid agree with the independent Python model, whose TOML parser is a third-party implementation",()=>{
 const grid:{arguments:string[];environment:Record<string,string>;cores:number;granted:number}[]=[];
 for(const subcommand of["check","test","build","+nightly check"])for(const packageFlag of[[],["-p","a"],["--package=b"],["--workspace"]])for(const jobsFlag of[[],["-j","3"],["-j7"],["--jobs=9"],["--jobs","0"],["--jobs=x"]])for(const tail of[[],["--","-j","2"]])for(const environment of[{},{CARGO_BUILD_JOBS:"5"},{CARGO_BUILD_JOBS:"zero"}])for(const cores of[1,10])for(const granted of[1,4])grid.push({arguments:[...subcommand.split(" "),...packageFlag,...jobsFlag,...tail],environment,cores,granted});
 const answer=python({lineage:[...cases.lineage,...grid],jobs:[...cases.jobs,...grid],caps:[...cases.caps,...grid],toolchains:cases.toolchains});
 [...cases.lineage,...grid].forEach((row,index)=>expect(cargoGrantLineageV1(row.arguments),JSON.stringify(row)).toEqual(answer.lineage[index]));
 [...cases.jobs,...grid].forEach((row,index)=>expect(cargoGrantJobsV1(row.arguments,row.environment,row.cores),JSON.stringify(row)).toBe(answer.jobs[index]));
 [...cases.caps,...grid].forEach((row,index)=>expect(capCargoGrantJobsV1(row.arguments,row.granted),JSON.stringify(row)).toEqual(answer.caps[index]));
 cases.toolchains.forEach((row,index)=>expect(cargoPinnedToolchainV1(row.file,row.triple),row.name).toBe(answer.toolchains[index]));
 expect(grid.length).toBeGreaterThan(1000);
});

test("a cap never exceeds the grant and never touches the arguments of the test binary",()=>{
 for(const granted of[1,2,5])for(const flag of[["-j","99"],["-j99"],["--jobs=99"],["--jobs","99"]]){
  const capped=capCargoGrantJobsV1(["test",...flag,"--","-j","99"],granted);
  expect(cargoGrantJobsV1(capped,{},10)).toBe(granted);
  expect(capped.slice(capped.indexOf("--"))).toEqual(["--","-j","99"]);
 }
});

test("the pinned toolchain of this repository is resolved, from any directory below it, like the rustup proxy would, and a caller's own choice wins",()=>{
 const here=dirname(fileURLToPath(import.meta.url)),pinned=readCargoPinnedToolchainV1(here,triple);
 expect(pinned).toMatch(/^nightly-\d{4}-\d{2}-\d{2}-aarch64-apple-darwin$/u);
 expect(cargoGrantEnvironmentV1(3,{},here,triple)).toEqual({CARGO_BUILD_JOBS:"3",RUSTUP_TOOLCHAIN:pinned!});
 expect(cargoGrantEnvironmentV1(3,{RUSTUP_TOOLCHAIN:"stable"},here,triple)).toEqual({CARGO_BUILD_JOBS:"3"});
 const bare=fresh();mkdirSync(join(bare,"a","b"),{recursive:true});writeFileSync(join(bare,"rust-toolchain.toml"),'[toolchain]\nchannel = "nightly-2030-01-01"\n');
 expect(readCargoPinnedToolchainV1(join(bare,"a","b"),"x86_64-unknown-linux-gnu")).toBe("nightly-2030-01-01-x86_64-unknown-linux-gnu");
 expect(cargoHostTripleV1("darwin","arm64")).toBe(triple);expect(cargoHostTripleV1("linux","x64")).toBe("x86_64-unknown-linux-gnu");expect(cargoHostTripleV1("win32","x64")).toBe("x86_64-pc-windows-msvc");
 expect(CARGO_GRANT_POLICY_V1.operation).toBe("cargo");
});

const sample=(available:number)=>async()=>({host:{version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:16384,availableMegabytes:available,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10} as HostSampleV1,swappedOutMegabytes:0});
const ask=(args:readonly string[])=>({tool:"cargo",operation:"cargo",kind:"build" as const,label:"cargo",arguments:args,cwd:dirname(fileURLToPath(import.meta.url))});
const logged:string[]=[],admission=(directory:string,available:number,peak=0)=>createToolAdmissionV1(policy,{directory,environment:{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},write:line=>{logged.push(line);},gate:{sample:sample(available),observe:async()=>peak}});

test("the jobs follow the memory that is left above the floor: a roomy host grants more jobs than a tight one, never fewer than one, and the grant is handed over as CARGO_BUILD_JOBS",async()=>{
 const granted:number[]=[];
 for(const available of[16000,12000,8500]){
  await admission(fresh(),available)(ask(["check","-p","a","-j","10"]),async grant=>{granted.push(grant.jobs);expect(grant.environment.CARGO_BUILD_JOBS).toBe(String(grant.jobs));expect(grant.environment.RUSTUP_TOOLCHAIN).toMatch(/^nightly-/u);return{status:0};});
 }
 expect(granted[0]!).toBeGreaterThan(granted[2]!);expect(granted[0]!).toBeGreaterThanOrEqual(granted[1]!);expect(granted[1]!).toBeGreaterThanOrEqual(granted[2]!);expect(granted[2]!).toBeGreaterThanOrEqual(1);expect(granted[0]!).toBeLessThanOrEqual(10);
},30000);

test("a successful run teaches the class of its own package, a failed or a signalled run teaches nothing, and another package borrows only the class above it",async()=>{
 const directory=fresh(),now=Date.now();
 await admission(directory,16000,3000)(ask(["check","-p","heavy"]),async()=>{await sleep(80);return{status:0};});
 await admission(directory,16000,90)(ask(["check","-p","light"]),async()=>{await sleep(80);return{status:0};});
 await admission(directory,16000,9000)(ask(["check","-p","broken"]),async()=>{await sleep(80);return{status:101};});
 await admission(directory,16000,9000)(ask(["check","-p","killed"]),async()=>{await sleep(80);return{status:null,signal:"SIGKILL"};});
 const store=readHostEstimateStoreV1(directory);
 expect(Object.keys(store.classes).filter(name=>name.startsWith("cargo")).sort()).toEqual(["cargo","cargo:check","cargo:check:heavy","cargo:check:light"]);
 const ratio=(lineage:readonly string[])=>estimateHostRatioV1(policy,store,lineage,now).ratioBasisPoints;
 expect(ratio(["cargo","cargo:check","cargo:check:heavy"])).toBeGreaterThan(ratio(["cargo","cargo:check","cargo:check:light"]));
},30000);

test("a learned heavy crate reserves more than a learned light one, so the same host admits the light one alone and the heavy one only after it",async()=>{
 const directory=fresh();
 for(let index=0;index<4;index++){logged.length=0;await admission(directory,16000,3500)(ask(["check","-p","heavy","-j","1"]),async()=>{await sleep(80);return{status:0};});await admission(directory,16000,120)(ask(["check","-p","light","-j","1"]),async()=>{await sleep(80);return{status:0};});}
 const order:string[]=[];
 const light=admission(directory,8900)(ask(["check","-p","light","-j","1"]),async()=>{order.push("light:start");await sleep(120);order.push("light:end");return{status:0};});
 await sleep(40);
 const heavy=admission(directory,8900)(ask(["check","-p","heavy","-j","1"]),async()=>{order.push("heavy:start");await sleep(10);order.push("heavy:end");return{status:0};});
 await Promise.all([light,heavy]);
 expect(order.indexOf("light:end"),logged.join("\n")).toBeLessThan(order.indexOf("heavy:start"));
},30000);

test("admission waits while memory is short, reports progress, and a cancellation stops the wait without starting the work",async()=>{
 const directory=fresh(),abort=new AbortController(),lines:string[]=[];let started=false;
 const wait=createToolAdmissionV1(policy,{directory,environment:{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},write:line=>{lines.push(line);},gate:{sample:sample(500),observe:async()=>0}})(ask(["check"]),async()=>{started=true;return{status:0};},abort.signal);
 const outcome=wait.then(()=>"done",()=>"stopped");
 await sleep(150);abort.abort();
 expect(await outcome).toBe("stopped");expect(started).toBe(false);expect(lines.some(line=>/waiting|wait/iu.test(line))).toBe(true);
},30000);

test("two independent invocations share one ledger: while one holds most of the memory the other waits, and both finish",async()=>{
 const directory=fresh(),order:string[]=[];
 const one=admission(directory,9000)(ask(["check","-p","one","-j","10"]),async grant=>{order.push(`one:start:${grant.jobs}`);await sleep(150);order.push("one:end");return{status:0};});
 await sleep(40);
 const two=admission(directory,9000)(ask(["check","-p","two","-j","10"]),async grant=>{order.push(`two:start:${grant.jobs}`);order.push("two:end");return{status:0};});
 await Promise.all([one,two]);
 expect(order.filter(row=>row.endsWith(":end")).length).toBe(2);
 const oneStarted=Number(order[0]!.split(":")[2]);expect(oneStarted).toBeLessThan(10);
},30000);

test.skipIf(process.platform==="win32")("the managed cargo entry sends every compiling subcommand through the Cargo ingress gate and every other one through the tool registry, whatever its arguments",()=>{
 const shim=fileURLToPath(new URL("../../../../../../🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims/cargo",import.meta.url)),directory=fresh(),record=join(directory,"called.json");
 writeFileSync(join(directory,"repo"),`#!/bin/sh\n"${process.execPath}" -e 'require("fs").writeFileSync(process.argv[1],JSON.stringify(process.argv.slice(2)))' "${record}" "$@"\n`);chmodSync(join(directory,"repo"),0o755);
 for(const row of cases.shims){
  const result=spawnSync(shim,row.arguments,{env:{PATH:`${directory}:${process.env.PATH}`},encoding:"utf8"});
  expect(result.status,result.stderr).toBe(0);expect(JSON.parse(readFileSync(record,"utf8")),row.name).toEqual(row.called);
 }
});

const shimDirectory=fileURLToPath(new URL("../../../../../../🛍️products/🦑️repo/🔨️modules/🚀️launcher/🚪️shims/",import.meta.url));
const compilingOf=(posix:string):string[]=>/^\s*([a-z|]+)\)\s*$/mu.exec(posix)![1]!.split("|");

test("the Windows entry maps every command line like the POSIX entry: same compiling subcommands, same two targets, read with cmd's own rules (percent expansion, quote stripping, case-sensitive string comparison)",()=>{
 const posix=readFileSync(join(shimDirectory,"cargo"),"utf8"),batch=readFileSync(join(shimDirectory,"cargo.cmd"),"utf8");
 const listed=/for %%c in \(([^)]*)\) do if "%~1"=="%%c"/u.exec(batch)![1]!.split(" ").filter(Boolean);
 expect([...listed].sort()).toEqual([...compilingOf(posix)].sort());
 expect(batch).toContain("set SEMIO_CARGO_STORE=isolated");expect(batch).toContain("set SEMIO_BUILD_MODE=prototype");
 const gate=/^\s*(repo cargo [^\r\n]*)$/mu.exec(batch)![1]!,registry=/^(repo tools exec cargo [^\r\n]*)$/mu.exec(batch)![1]!;
 expect(gate).toBe("repo cargo --store=%SEMIO_CARGO_STORE% --configuration=%SEMIO_BUILD_MODE% -- %*");expect(registry).toBe("repo tools exec cargo -- %*");
 const run=(line:readonly string[]):readonly string[]=>{
  const first=(line[0]??"").replace(/^"(.*)"$/u,"$1"),expand=(template:string):string[]=>template.replace(/^repo /u,"").split(" ").flatMap(token=>token==="%*"?[...line]:[token.replace("%SEMIO_CARGO_STORE%","isolated").replace("%SEMIO_BUILD_MODE%","prototype")]);
  return expand(listed.includes(first)?gate:registry);
 };
 for(const row of cases.shims)expect(run(row.arguments),row.name).toEqual(row.called);
 expect(run(["Build"]),"cmd compares strings case-sensitively without /i, like the POSIX case").toEqual(["tools","exec","cargo","--","Build"]);
 expect(run(['"build"']),"quotes around the first argument are stripped by %~1").toEqual(["cargo","--store=isolated","--configuration=prototype","--",'"build"']);
});

test.skipIf(process.platform!=="win32")("on Windows the batch entry itself calls repo with the mapped arguments",()=>{
 const directory=fresh(),record=join(directory,"called.json");
 writeFileSync(join(directory,"repo.cmd"),`@"${process.execPath}" -e "require('fs').writeFileSync(process.argv[1],JSON.stringify(process.argv.slice(2)))" "${record}" %*\r\n`);
 for(const row of cases.shims){
  const result=spawnSync("cmd.exe",["/d","/c",join(shimDirectory,"cargo.cmd"),...row.arguments],{env:{...process.env,PATH:`${directory};${process.env.PATH}`},encoding:"utf8"});
  expect(result.status,result.stderr).toBe(0);expect(JSON.parse(readFileSync(record,"utf8")),row.name).toEqual(row.called);
 }
});
