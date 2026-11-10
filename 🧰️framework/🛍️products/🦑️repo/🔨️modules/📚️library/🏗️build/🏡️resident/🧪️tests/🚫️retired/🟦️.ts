/** 🚫️ Contract: the per-worktree resident service, its warm pool, delegated executors and forwarding client are retired (the per-host graph service replaced them). The files wait for deletion, but nothing may import, export or name them, and the `--service` option is gone. */
import {expect,test} from "bun:test";
import {readdirSync,readFileSync}from "node:fs";
import {join,relative,sep}from "node:path";
import {fileURLToPath}from "node:url";

const build=fileURLToPath(new URL("../../../",import.meta.url)),resident=join(build,"🏡️resident"),library=join(build,"..");
const retired=["🏡️resident/🏊️delegate","🏡️resident/🏭️executor","🏡️resident/🚪️client/🟨️","🏡️resident/🚪️client/⌨️main","🏡️resident/🚪️client/🔁️repo","🏡️resident/🏛️service/🟨️","🏡️resident/🏗️composition/🟨️"];
const relativeRetired=["🏊️delegate","🏭️executor","🚪️client/🟨️","🚪️client/⌨️main","🚪️client/🔁️repo","🏛️service/🟨️.mjs","🏗️composition/🟨️.mjs"];
const names=["forwardRepoCommandV1","forwardCommandV1","serviceEligibleV1","serviceModeV1","createResidentServiceV1","createWarmPoolV1","createDelegatedExecutionPortV1","createForegroundRunnerV1","createExecutorPoolV1","loadBuildGraphInChildV1","buildGraphPlanInChildV1"];
const dead=(path:string)=>[...retired,...relativeRetired.map(row=>"🏡️resident/"+row)].some(row=>path.split(sep).join("/").includes(row))||/🧪️tests\/(🏊️delegate|🏭️executor|🚪️client|🏛️service|⌨️command|🖥️fixture-server)\//u.test(path.split(sep).join("/"))||path.endsWith("🔥️warm/🏊️pool.ts")||path.includes("🕸️graph/🧠️memo/");
const listed=(directory:string)=>{try{return readdirSync(directory,{withFileTypes:true});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return[];throw error;}};
const walk=(directory:string):string[]=>listed(directory).flatMap(entry=>entry.name==="node_modules"||entry.name===".git"?[]:entry.isDirectory()?walk(join(directory,entry.name)):/\.(ts|mjs|mts)$/u.test(entry.name)?[join(directory,entry.name)]:[]);
const live=walk(library).filter(file=>!dead(relative(library,file))&&!file.endsWith("🚫️retired/🟦️.ts"));

test("no live source imports a retired module or names a retired entry point",()=>{
 const hits:string[]=[];
 for(const file of live){
  let text:string;try{text=readFileSync(file,"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}
  for(const match of text.matchAll(/(?:from\s*|import\s*\(\s*|new URL\(\s*)["']([^"']+)["']/gu)){
   const specifier=match[1]!;
   if(/🏊️delegate|🏭️executor|🚪️client\/(🟨️|⌨️main|🔁️repo)|🏛️service\/🟨️\.(mjs|d\.mts)|🏗️composition\/🟨️\.(mjs|d\.mts)/u.test(specifier))hits.push(relative(library,file)+" imports "+specifier);
  }
  for(const name of names)if(text.includes(name))hits.push(relative(library,file)+" names "+name);
 }
 expect(hits).toEqual([]);
});

test("the resident entry exports only the host service, its protocol and the journal pieces",()=>{
 const entry=readFileSync(join(resident,"🟨️.mjs"),"utf8");
 for(const name of names)expect(entry).not.toContain(name);
 expect(entry).toContain("createHostServiceV1");
});

test("the `--service` option is not part of the command line",()=>{
 const catalog=readFileSync(join(build,"⚙️operations/🔣️catalog.json"),"utf8");
 expect(catalog).not.toContain('"--service"');
 const invocation=readFileSync(join(build,"🧭️invocation/🟦️.ts"),"utf8");
 expect(invocation).not.toContain("--service");
});
