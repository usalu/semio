/** 🚦️ Contract of native cargo admission: a cargo run (`repo cargo`, whose job count the gate grants) is sized and judged exactly like an independent Python implementation says over the host matrix of `./🧫️fixtures/🔣️.json`, holds its authored verdicts, and on a host that cannot carry one job it starts nothing, prints that it waits and ends on cancellation.
 * @see ./🐍️oracle/🐍️.py
 * @see ../🦀️cargo-scaling/🟦️.ts
 * @see ../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️24/WARDEN-COVERAGE-FOR-EVERY-REPO-TOOLCHAIN-PROCESS-AND-DISK-BOUNDED-SWAP/🚦️native-cargo-admission.md */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {decideHostCapacityV1,readHostCapacityPolicyV1,sizeHostRequestV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import {requirePinnedTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {withBuildCapacityV1} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

setDefaultTimeout(60000);
const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity),python=requirePinnedTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const sampleOf=(host:(typeof fixture.cases)[number]["host"]):HostSampleV1=>({version:1,atMs:1_000_000,platform:"darwin",...host} as HostSampleV1);
const oracle=(cases:readonly object[]):unknown[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({policy:buildPolicy.hostCapacity,cases}),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const gate=(item:(typeof fixture.cases)[number])=>{
 const host=sampleOf(item.host),request=sizeHostRequestV1(policy,{id:"native-cargo",operation:"cargo",preferredJobs:item.preferredJobs,host})!;
 const row=decideHostCapacityV1(policy,host,[],[{request,arrivedAtMs:host.atMs,pid:1}],host.atMs).decisions[0]!;
 return {verdict:row.verdict,jobs:row.jobs,reason:row.reason};
};

test("the authored verdicts hold for every host of the matrix",()=>{for(const item of fixture.cases)expect<unknown>(gate(item),item.name).toEqual(item.expected);});
test("the gate agrees with the independent oracle for every host of the matrix",()=>{expect<unknown>(fixture.cases.map(gate)).toEqual(oracle(fixture.cases));});
test("the matrix covers every way a cargo run is held back and every width it is granted",()=>{
 const reasons=new Set(fixture.cases.map(item=>item.expected.reason)),widths=new Set(fixture.cases.filter(item=>item.expected.verdict==="admit").map(item=>item.expected.jobs));
 for(const reason of ["fits","reduced","memory","pressure-critical","pressure-warn","swap","swap-activity","files","attaches"])expect(reasons.has(reason),reason).toBe(true);
 expect(widths.size).toBeGreaterThanOrEqual(4);
});

const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
test("a cargo run on a host that cannot carry one job starts nothing, says that it waits and ends on cancellation",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"nc-"));roots.push(directory);
 const waiting=fixture.waiting,abort=new AbortController(),lines:string[]=[],timer=setTimeout(()=>abort.abort(),waiting.cancelAfterMilliseconds);
 const waitPolicy=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
 const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:16384,availableMegabytes:waiting.availableMegabytes,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:100,openFilesMaximum:1000000,attachedImages:null});
 let started=false;
 try{
  await expect(withBuildCapacityV1(["cargo","--store=shared","--configuration=prototype","--parallelism=8","--","check","-p","x"],{SEMIO_HOST_CAPACITY_DIRECTORY:directory,SEMIO_OFFLOAD:"never",LANG:"en_US.UTF-8"},async()=>{started=true;},{directory,policy:waitPolicy,strip:args=>args.filter(argument=>!argument.startsWith("--tool")),sample:async()=>({host:host(),swappedOutMegabytes:0}),observe:async()=>45,write:line=>{lines.push(line);},exitCode:()=>0,control:{signal:abort.signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined}})).rejects.toThrow();
 }finally{clearTimeout(timer);}
 expect(started).toBe(false);
 expect(lines.some(line=>/waiting .*for cargo.*memory/u.test(line)),lines.join("\n")).toBe(true);
});
