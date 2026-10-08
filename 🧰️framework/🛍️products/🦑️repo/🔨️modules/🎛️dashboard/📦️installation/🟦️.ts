/** 🚀️ Starts an installed immutable Rust dashboard without graph discovery or executable hashing. */
import { existsSync, readFileSync, writeFileSync, renameSync, mkdirSync } from "node:fs";
import { join, relative, resolve, isAbsolute } from "node:path";
import { spawn } from "node:child_process";

const cache=(root:string)=>join(root,".🧬semio/🦑️repo/⚡️cache");
const manifest=(root:string)=>join(cache(root),"🎛️dashboard/installed.json");

/** 🔎️ Whether an installation record exists; a present but invalid record still fails loudly on launch. */
export function dashboardInstalled(root:string):boolean { return existsSync(manifest(root)); }

/** 📍️ Resolves one small installation record; launch never reads the executable contents. */
export function installedDashboard(root:string):string {
  const file=manifest(root);
  if(!existsSync(file))throw new Error("Dashboard is not installed. Run bun run dashboard:install once, then start bun run dashboard.");
  const value=JSON.parse(readFileSync(file,"utf8")),name=process.platform==="win32"?"semio.exe":"semio";
  if(value.version!==1||value.platform!==process.platform||typeof value.path!=="string"||!new RegExp(`^\\.🧬semio/🦑️repo/⚡️cache/tools/dashboard-cli/[a-f0-9]{64}/${name.replace(".","\\.")}$`).test(value.path))throw new Error("Invalid dashboard installation record; run bun run dashboard:install.");
  const executable=resolve(root,value.path);
  if(!existsSync(executable))throw new Error("Installed dashboard executable is missing; run bun run dashboard:install.");
  return executable;
}

/** 🧊️ Publishes immutable executable bytes and one atomic native installation record. */
export async function installDashboard(packageRoot:string,root:string):Promise<string> {
  const {pinExecutableArtifact}=await import("../../../../../🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts");
  const controller=new AbortController(),cancel=()=>controller.abort();
  process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
  try {
    const name=process.platform==="win32"?"semio.exe":"semio";
    const executable=await pinExecutableArtifact(resolve(packageRoot,"dist/build",name),join(cache(root),"tools/dashboard-cli"),"dashboard-installation",{signal:controller.signal,leaseDirectory:join(cache(root),"agents/resource-leases"),onWait:({elapsedMs})=>console.log(`[dashboard] Installing executable (${elapsedMs}ms); Ctrl+C cancels`)});
    const path=relative(root,executable).replaceAll("\\","/");
    if(isAbsolute(path)||path.startsWith("../"))throw new Error("Dashboard installation must stay inside the workspace");
    const target=manifest(root),temporary=`${target}.${process.pid}.tmp`;
    mkdirSync(resolve(target,".."),{recursive:true});
    writeFileSync(temporary,JSON.stringify({version:1,platform:process.platform,path})+"\n");
    renameSync(temporary,target);return executable;
  } finally {process.removeListener("SIGINT",cancel);process.removeListener("SIGTERM",cancel);}
}

/** 🧭️ Recognizes native installed-tool launches while retaining Nx for authoring tasks. */
export function dashboardInvocation(args:readonly string[]):string[]|undefined {
  if(args[0]!=="run")return;
  const prefix="@semio-tech/repo-dashboard-rs:",target=args[1]?.startsWith(prefix)?args[1].slice(prefix.length):undefined;
  if(!target||!["run","daemon","workflow","preferences"].includes(target))return;
  const rest=args.slice(2),separator=rest.indexOf("--"),options=separator<0?rest:rest.slice(0,separator);
  if(options.some(option=>/^--(?:graph|batch|parallel|excludeTaskDependencies)(?:=|$)/.test(option)))return;
  const forwarded=options.filter(option=>!/^--(?:outputStyle|output-style)=stream$/.test(option)&&!["--skip-nx-cache","--skipNxCache"].includes(option));
  return [...(target==="run"?[]:[target]),...forwarded,...(separator<0?[]:rest.slice(separator+1))];
}

/** ▶️ Hands the terminal to the native binary; no setup, graph, build or discovery precedes it. */
export async function launchDashboard(root:string,args:readonly string[]):Promise<number> {
  const executable=installedDashboard(root);
  return await new Promise<number>((accept,reject)=>{
    const child=spawn(executable,args,{cwd:root,env:process.env,stdio:"inherit",windowsHide:true});
    child.once("error",reject);child.once("close",(code,signal)=>accept(code??(signal==="SIGINT"?130:1)));
  });
}
