import {accessSync,constants,statSync} from "node:fs";
import {delimiter,isAbsolute,join} from "node:path";
import policy from "./🔣️.json" with {type:"json"};

/** 🔌️ Supplies host facts without placing package execution outside the caller's command port. */
export interface BrowserSystemPortV1 {
 readonly platform:string;
 readonly root:boolean;
 resolve(name:string):string|null;
 isFile(path:string):boolean;
}
/** 🏃️ Executes a browser prerequisite under the existing dependency phase. */
export type BrowserDependencyCommandV1=(command:string,args:readonly string[],environment:Readonly<Record<string,string|undefined>>)=>Promise<unknown>;

/** 🖥️ Resolves executable host tools using only the caller's explicit environment. */
export function browserSystemPortV1(environment:Readonly<Record<string,string|undefined>>):BrowserSystemPortV1 {
 return {platform:process.platform,root:process.getuid?.()===0,resolve(name){
  for(const directory of (environment.PATH??"").split(delimiter)){
   if(!isAbsolute(directory))continue;
   const path=join(directory,name);
   try{accessSync(path,constants.X_OK);if(statSync(path).isFile())return path;}catch(error){if(!["ENOENT","ENOTDIR","EACCES"].includes((error as NodeJS.ErrnoException).code??""))throw error;}
  }
  return null;
 },isFile(path){try{return statSync(path).isFile();}catch(error){if(["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??""))return false;throw error;}}};
}

/** 🌐️ Installs required Linux libraries before downloading Chromium through the same bounded command port. */
export async function prepareBrowserDependenciesV1(port:BrowserSystemPortV1,execute:BrowserDependencyCommandV1,runtime:string,cli:string,environment:Readonly<Record<string,string|undefined>>,signal:AbortSignal,offline:boolean):Promise<void> {
 signal.throwIfAborted();
 if(offline)throw Error("Browser provisioning requires online authority");
 if(port.platform==="linux"){
  if(!port.isFile(cli))throw Error("Playwright CLI required");
  let manager:string|undefined,executable:string|null=null;
  for(const candidate of policy.order){const resolved=port.resolve(candidate);if(resolved){manager=candidate;executable=resolved;break;}}
  if(!manager||!executable)throw Error("Supported Linux package manager required");
  const row=policy.packages.find(row=>row.manager===manager),apt=manager==="apt-get";
  let command=apt?runtime:executable,args=apt?[cli,"install-deps","chromium"]:[...row!.arguments,...row!.packages];
  const bounded=apt?{...environment,DEBIAN_FRONTEND:"noninteractive"}:environment;
  if(!port.root){
   const sudo=port.resolve("sudo");if(!sudo)throw Error("Noninteractive sudo required");
   if(apt){const env=port.resolve("env");if(!env)throw Error("System env executable required");args=["-n","--",env,"DEBIAN_FRONTEND=noninteractive",command,...args];}
   else args=["-n","--",command,...args];
   command=sudo;
  }
  signal.throwIfAborted();
  await execute(command,args,bounded);
  signal.throwIfAborted();
 }
 await execute(runtime,[cli,"install","chromium"],environment);
 signal.throwIfAborted();
}
