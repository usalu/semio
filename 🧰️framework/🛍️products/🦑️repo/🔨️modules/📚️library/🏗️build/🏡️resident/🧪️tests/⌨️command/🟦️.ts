import {afterAll,describe,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {runResidentCommandV1,RESIDENT_EXCLUDES_V1} from "../../⌨️command/🟨️.mjs";
import {createHostServiceV1} from "../../🏛️service/🌐️host/🟨️.mjs";
import {createGraphPlannerPortV1} from "../../🔌️ports/🧭️plan/🟨️.mjs";
import {HOST_POLICY_V1} from "../../📨️protocol/🎚️policy/🟨️.mjs";
import {hostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import {cleanup,shortTempDirectory,sleep,tempRoot,waitFor} from "../🟦️.ts";

const cleanups:(()=>Promise<void>|void)[]=[];afterAll(async()=>{for(const fn of cleanups.reverse())await fn();});
const sink=()=>{const parts:string[]=[];return{parts,write:(t:string)=>{parts.push(String(t));return true;},text:()=>parts.join("")};};

const implementation="2".repeat(64);
const planner=createGraphPlannerPortV1({identify:async()=>"k".repeat(64),loadGraph:async()=>({version:1,projects:{},dependencies:[]})});

async function hostFor(){
 const base=shortTempDirectory("sr-");cleanups.push(()=>cleanup(base));
 const environment={...process.env,XDG_RUNTIME_DIR:base,TMPDIR:base,LOCALAPPDATA:base} as Record<string,string>,runtime=hostEndpointsV1(implementation,environment);
 const service=createHostServiceV1({runtime,implementation,ports:{planner},policy:{residentCheckMilliseconds:50}});
 await service.start();cleanups.push(()=>service.stop("test"));
 return{environment,service,runtime};
}

describe("repo service host",()=>{
 test("status prints the running service, usage and a missing service are told in the requested language",async()=>{
  const s=await hostFor(),root=tempRoot("cmd-");cleanups.push(()=>cleanup(root));
  const run=async(args:string[],languages=["de","en"],environment=s.environment)=>{const out=sink(),err=sink();const exit=await runResidentCommandV1({root,args:["host",...args],languages,environment,stdout:out,stderr:err,host:{implementation,command:[process.execPath,"-e","process.exit(3)"],runForeground:async()=>0}});return{exit,out:out.text(),err:err.text()};};
  const json=await run(["status","--output=json"]);expect(json.exit).toBe(0);const document=JSON.parse(json.out);expect(document.pid).toBe(process.pid);expect(document.memory.rssMb).toBeGreaterThan(0);expect(document.plans.total).toBeGreaterThanOrEqual(0);
  const human=await run(["status"],["en"]);expect(human.exit).toBe(0);expect(human.out).toContain("pid "+process.pid);expect(human.out).toContain("Service memory:");expect(human.out).toContain("Plans:");expect(human.out.trimStart().startsWith("{")).toBe(false);
  const german=await run(["status"],["de"]);expect(german.out).toContain("Speicher des Dienstes:");expect(german.out).toContain("Pläne:");
  const usage=await run(["frobnicate"],["de"]);expect(usage.exit).toBe(1);expect(usage.err).toContain("Verwendung");
  expect((await run(["frobnicate"],["en"])).err).toContain("Usage");
  const elsewhere=shortTempDirectory("sr-");cleanups.push(()=>cleanup(elsewhere));
  const down=await run(["status"],["en"],{...s.environment,XDG_RUNTIME_DIR:elsewhere,TMPDIR:elsewhere,LOCALAPPDATA:elsewhere});expect(down.exit).toBe(1);expect(down.out.length).toBeGreaterThan(0);
  expect((await run(["run"],["en"])).exit).toBe(0);
 });

 test("start reports a service that cannot be started, in the requested language",async()=>{
  const root=tempRoot("cmd-start-");cleanups.push(()=>cleanup(root));
  const base=shortTempDirectory("sr-");cleanups.push(()=>cleanup(base));
  const environment={...process.env,XDG_RUNTIME_DIR:base,TMPDIR:base,LOCALAPPDATA:base} as Record<string,string>;
  const out=sink(),err=sink();
  const exit=await runResidentCommandV1({root,args:["host","start"],languages:["en"],environment,stdout:out,stderr:err,host:{implementation:"3".repeat(64),command:[process.execPath,"-e","process.exit(3)"],runForeground:async()=>0,policy:{...HOST_POLICY_V1,startTimeoutMilliseconds:2000}}});
  expect(exit).toBe(1);expect(err.text().length).toBeGreaterThan(0);expect(out.text()).toBe("");
 },30000);

 test("excludes keep caches, VCS data and dependency trees out of the watch",()=>{
  expect(RESIDENT_EXCLUDES_V1.rooted).toContain(".git");expect(RESIDENT_EXCLUDES_V1.names).toContain("node_modules");expect(RESIDENT_EXCLUDES_V1.rooted.length).toBeLessThanOrEqual(8+8);
 });
});
