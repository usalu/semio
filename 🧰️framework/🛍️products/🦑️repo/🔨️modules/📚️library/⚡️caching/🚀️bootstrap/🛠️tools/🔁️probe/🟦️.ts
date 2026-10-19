import {readFileSync} from "node:fs";
import {join} from "node:path";
import {openNxSdkV1} from "../🔌️sdk/🟦️.ts";
import type {CacheProbeInputV1,CacheProbeToolV1} from "../../../../🏗️build/⚙️operations/🟦️.ts";

/** 🧭️ Maps one neutral probe input onto its Nx `inputs` spelling. */
function nxProbeInputV1(input:CacheProbeInputV1):string|Readonly<{env:string}>{
 if("file" in input)return "{projectRoot}/"+input.file;
 if("named" in input)return input.named;
 return {env:input.env};
}

/** 🏭️ Renders the neutral cache probe as an isolated Nx workspace and invokes Nx for one of its tasks. */
export function nxCacheProbeV1(repositoryRoot:string):CacheProbeToolV1{
 const sdk=openNxSdkV1(join(repositoryRoot,"package.json"));
 return Object.freeze({
  tool:"nx",
  label:"Nx",
  version:()=>JSON.parse(readFileSync(join(sdk.packageDirectory(),"package.json"),"utf8")).version as string,
  files:specification=>({
   "nx.json":{useDaemonProcess:false,cacheDirectory:".nx/cache",maxCacheSize:specification.maxCacheSize,namedInputs:Object.fromEntries(Object.entries(specification.namedInputs).map(([name,inputs])=>[name,inputs.map(nxProbeInputV1)]))},
   "package.json":{name:"cache-verification",private:true,nx:{includedScripts:[]}},
   ".gitignore":[specification.ignore[0],".nx",...specification.ignore.slice(1)].join("\n")+"\n",
   "project.json":{name:specification.project,targets:Object.fromEntries(specification.targets.map(target=>[target.name,{executor:"nx:run-commands",cache:target.cache,...(target.dependsOn?{dependsOn:target.dependsOn}:{}),...(target.inputs?{inputs:target.inputs.map(nxProbeInputV1)}:{}),outputs:target.outputs.map(output=>"{projectRoot}/"+output),options:{command:target.command}}]))}
  }),
  invocation:(specification,task,fixture,environment)=>{
   const nx:Record<string,string|undefined>={...environment,NX_DAEMON:"false",NX_ISOLATE_PLUGINS:"false",NX_TUI:"false",NX_NATIVE_COMMAND_RUNNER:"false",NX_WORKSPACE_DATA_DIRECTORY:join(fixture,".nx","workspace-data")};
   for(const key of Object.keys(nx))if(key.startsWith("NX_TASK_")||["NX_SKIP_NX_CACHE","NX_SKIP_REMOTE_CACHE"].includes(key))delete nx[key];
   delete nx.NX_FORCE_REUSE_CACHED_GRAPH;
   return {command:"node",arguments:[sdk.cli(),"run",`${specification.project}:${task}`,"--output-style=static"],environment:nx};
  }
 });
}
