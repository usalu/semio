import {existsSync,readdirSync,readFileSync} from "node:fs";
import {join,relative,sep} from "node:path";
import type {BreachRecord} from "../../🚨️policy/🔖️breach/🟦️.ts";

const BUILD="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const SERVICE=BUILD+"/📥️caller/🛰️service/";
const CONSUMERS:Readonly<Record<string,{readonly calls:number;readonly readers:readonly string[]}>>={
 [BUILD+"/📥️caller/🟦️.ts"]:{calls:3,readers:["capabilities","role","declared"]},
 [BUILD+"/🧭️invocation/🟦️.ts"]:{calls:1,readers:["caller"]},
 [BUILD+"/🔧️tools/🧶️nx/📨️envelope/🟦️.ts"]:{calls:1,readers:["role"]},
};
const ALLOWED_READERS:Readonly<Record<string,readonly string[]>>={[BUILD+"/📥️caller/🟦️.ts"]:["declared"]};
const HOLDERS:Readonly<Record<string,readonly string[]>>={
 declaredCapabilitiesV1:[SERVICE+"🟦️.ts",BUILD+"/📥️caller/🟦️.ts",BUILD+"/🧭️invocation/🟦️.ts",BUILD+"/🔧️tools/🧶️nx/📨️envelope/🟦️.ts"],
 activeServiceTicketV1:[SERVICE+"🟦️.ts"],
 holdServiceInvocationV1:[SERVICE+"🟦️.ts",BUILD+"/⌨️command/⚙️runtime/🟦️.ts"],
 createServiceAdmissionV1:[SERVICE+"🟦️.ts"],
 createServiceGrantV1:[SERVICE+"🟦️.ts",BUILD+"/📥️caller/📤️child/🟦️.ts"],
 createBuildBootstrapCapabilitiesV1:[BUILD+"/📜️script.ts",BUILD+"/⌨️command/⚙️runtime/🟦️.ts",BUILD+"/📥️caller/📤️child/🟦️.ts"],
};
const RESIDENT=BUILD+"/🏡️resident/";
const ADMISSION=SERVICE+"🟦️.ts";
const RUNTIME=BUILD+"/⌨️command/⚙️runtime/🟦️.ts";
const UPWARD=/📤️child|📥️caller\/🟦️\.ts|createBuildBootstrapCapabilitiesV1/u;
const PORT=/readonly bootstrap:ServiceBootstrapPortV1/u;
const INJECTION=/bootstrap:\(await import\("[^"]*📥️caller\/📤️child\/🟦️\.ts"\)\)\.createBuildBootstrapCapabilitiesV1/u;

function sources(directory:string,base:string,out:string[]=[]):string[]{
 if(!existsSync(directory))return out;
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  if(entry.name==="node_modules"||entry.name==="🧪️tests"||entry.name==="tests")continue;
  const path=join(directory,entry.name);
  if(entry.isDirectory())sources(path,base,out);
  else if(/\.(ts|mjs)$/u.test(entry.name))out.push(relative(base,path).split(sep).join("/"));
 }
 return out;
}

/** 🛰️ Fails when the service invocation is anything but the one declared kind: every launch check of the authority core reads the declaration through `declaredCapabilitiesV1`, only the declared holders mention the admission and the door computation, the admission imports nothing above it and receives the door computation through its injected `bootstrap` port, the one holder of the admission injects exactly the child bootstrap, and the resident service never imports the admission. */
export function policyServiceInvocationBreaches(repoRoot:string):BreachRecord[]{
 const breaches:BreachRecord[]=[],text=new Map<string,string>();
 const read=(file:string)=>{if(!text.has(file))text.set(file,readFileSync(join(repoRoot,file),"utf8"));return text.get(file)!;};
 const breach=(rule:string,file:string,summary:string,solution:string)=>breaches.push({id:"service-invocation-"+rule+"-"+file,summary,kind:"repo/service-invocation",scope:file,priority:"high",autofixable:false,reason:"A resident service may declare plain task commands only through the one service invocation kind of the authority core; any other way is a backdoor that widens what a client could run one-shot.",solution});
 for(const[file,expected]of Object.entries(CONSUMERS)){
  if(!existsSync(join(repoRoot,file))){breach("missing",file,file+" is a launch check of the authority core and is missing","Restore the file or update the consumers of the service invocation law.");continue;}
  const source=read(file),calls=source.split("declaredCapabilitiesV1(").length-1;
  if(calls!==expected.calls)breach("calls",file,file+" reads the declaration "+calls+" times, expected "+expected.calls,"Every launch check must read capabilities through declaredCapabilitiesV1; update the law only together with a new check site.");
  const names=new Set([...source.matchAll(/\b([A-Za-z_]\w*)\.launches\b/gu)].map(match=>match[1]!));
  for(const name of names){
   if((ALLOWED_READERS[file]??[]).includes(name))continue;
   if(!new RegExp("\\b"+name+"=[^;]*declaredCapabilitiesV1\\(","u").test(source))breach("reader",file,file+" reads "+name+".launches without deriving "+name+" from declaredCapabilitiesV1","Derive the variable with declaredCapabilitiesV1(capabilities) before reading its launches.");
  }
 }
 for(const file of sources(join(repoRoot,BUILD),repoRoot)){
  const source=read(file);
  for(const[identifier,holders]of Object.entries(HOLDERS))
   if(new RegExp("\\b"+identifier+"\\b","u").test(source)&&!holders.includes(file))breach("holder",file,file+" mentions "+identifier+" but only "+holders.map(holder=>holder.split("/").slice(-2).join("/")).join(", ")+" may","Route the call through the declared holder instead of using the admission directly.");
  if(file.startsWith(RESIDENT)&&/🛰️service\/🟦️\.ts/u.test(source))breach("resident",file,file+" imports the authority admission","The resident service shares only the session primitives and the schema with the authority core; it never declares launches.");
 }
 if(existsSync(join(repoRoot,ADMISSION))){
  const source=read(ADMISSION);
  if(UPWARD.test(source))breach("upward",ADMISSION,ADMISSION+" imports the layer above the admission","The admission depends only on its contract and session primitives; the door computation reaches it through the injected bootstrap port.");
  if(!PORT.test(source))breach("port",ADMISSION,ADMISSION+" declares no bootstrap port for the door computation","Hold the service invocation with an injected `bootstrap:ServiceBootstrapPortV1` instead of importing the build bootstrap.");
 }
 if(existsSync(join(repoRoot,RUNTIME))){
  const source=read(RUNTIME);
  if(source.includes("holdServiceInvocationV1(")&&!INJECTION.test(source))breach("injection",RUNTIME,RUNTIME+" holds the service invocation without injecting the child bootstrap","Pass `bootstrap:(await import(child)).createBuildBootstrapCapabilitiesV1` where the admission is held.");
 }
 return breaches;
}
