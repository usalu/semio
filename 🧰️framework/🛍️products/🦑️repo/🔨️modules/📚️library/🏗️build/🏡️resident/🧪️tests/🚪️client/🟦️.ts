import {describe,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {forwardRepoCommandV1,serviceEligibleV1,serviceModeV1} from "../../🚪️client/🔁️repo/🟨️.mjs";
import {agentIdV1,launcherCapabilityV1} from "../../🚪️client/🟨️.mjs";
import {createRootKeyV1,verifyCapabilityV1} from "../../📨️protocol/🔑️capability/🟨️.mjs";
import {HERE} from "../🟦️.ts";

const fixture=JSON.parse(readFileSync(join(HERE,"🧫️fixtures/📋️eligibility.json"),"utf8"));

describe("which commands go to the service",()=>{
 test("the shared eligibility fixture (also read by the Go launcher) decides every row",()=>{
  for(const row of fixture.eligibility)expect([row.args,serviceEligibleV1(row.args)]).toEqual([row.args,row.eligible]);
 });
 test("service mode: flag beats environment, default auto, unknown values are refused, nothing after -- counts (shared fixture)",()=>{
  for(const row of fixture.modes){
   if(row.mode==="invalid")expect(()=>serviceModeV1(row.args,row.environment)).toThrow(/auto\|off\|require/u);
   else expect([row.args,serviceModeV1(row.args,row.environment)]).toEqual([row.args,row.mode]);
  }
 });
});

describe("agent identity",()=>{
 test("the shared fixture (also read by the Go launcher) decides every row",()=>{
  for(const row of fixture.agents)expect([row,agentIdV1({agent:{id:row.value||row.fallback}})]).toEqual([row,row.agent]);
 });
 test("a launcher token is bound to its worktree and agent and expires after a minute",()=>{
  const key=createRootKeyV1(),caveats=verifyCapabilityV1(key,launcherCapabilityV1(key,1000,{worktree:"440cffa5563d",agent:"codex-1"}))!;
  expect(caveats).toEqual([{k:"scope",v:["status","query","run","watch"]},{k:"notAfter",v:61000},{k:"worktree",v:"440cffa5563d"},{k:"agent",v:"codex-1"}]);
 });
});

describe("forwarding decision",()=>{
 const sink=()=>{const parts:string[]=[];return{parts,write:(t:string)=>{parts.push(String(t));return true;}};};
 const base={root:"/w",environment:{SEMIO_BUILD_TOOL:"bazel",UNRELATED:"x"} as Record<string,string>,languageOf:()=>"de"};
 test("an answered request returns the exit code and sends ids, bound environment and the preferred language order",async()=>{
  const calls:any[]=[];
  const code=await forwardRepoCommandV1({...base,args:["run","a:build","--output=stream"],forward:async o=>{calls.push(o);return{handled:true,exit:7,timing:{}};}});
  expect(code).toBe(7);expect(calls[0]).toMatchObject({command:"run",args:["a:build"],languages:["de","en"],env:{SEMIO_BUILD_TOOL:"bazel"}});expect(calls[0].env.UNRELATED).toBeUndefined();
 });
 test("the preferred language comes first, the other supported one follows; a language the service does not have is unresolved, never English",async()=>{
  const seen:(readonly string[])[]=[];
  for(const language of["en","de"])await forwardRepoCommandV1({...base,args:["run","a:build"],languageOf:()=>language,forward:async o=>{seen.push(o.languages);return{handled:true,exit:0,timing:{}};}});
  expect(seen).toEqual([["en","de"],["de","en"]]);
  const never=async()=>{throw new Error("must not be called");};
  expect(await forwardRepoCommandV1({...base,args:["run","a:build"],languageOf:()=>"fr",forward:never as any})).toBeUndefined();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--service=require"],languageOf:()=>"fr",forward:never as any})).toBe(1);
 });
 test("the reference entry refuses to run without an explicit language and says so in both",()=>{
  const run=Bun.spawnSync(["bun",fileURLToPath(new URL("../../🚪️client/⌨️main/🟨️.mjs",import.meta.url)),"--root","/nowhere","--","ping"]);
  expect(run.exitCode).toBe(2);const text=run.stderr.toString();expect(text).toContain("None of the requested languages");expect(text).toContain("Keine der angeforderten Sprachen");
 });
 test("off, ineligible, unanswered and unresolved language fall back with undefined; require fails instead",async()=>{
  const never=async()=>{throw new Error("must not be called");};
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--service=off"],forward:never as any})).toBeUndefined();
  const leading:any[]=[];
  expect(await forwardRepoCommandV1({...base,args:["--service=auto","run","a:build"],forward:async o=>{leading.push(o);return{handled:true,exit:0,timing:{}};}})).toBe(0);expect(leading[0]).toMatchObject({command:"run",args:["a:build"]});
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--skip-cache"],forward:never as any})).toBeUndefined();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build"],forward:async()=>({handled:false,reason:"down",timing:{}})})).toBeUndefined();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build"],languageOf:()=>{throw new Error("unresolved");},forward:never as any})).toBeUndefined();
  const err=sink();
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--service=require"],stderr:err,forward:async()=>({handled:false,reason:"down",timing:{}})})).toBe(1);expect(err.parts.join("")).toContain("down");
  expect(await forwardRepoCommandV1({...base,args:["run","a:build","--service=require"],languageOf:()=>{throw new Error("unresolved");},forward:never as any})).toBe(1);
 });
});
