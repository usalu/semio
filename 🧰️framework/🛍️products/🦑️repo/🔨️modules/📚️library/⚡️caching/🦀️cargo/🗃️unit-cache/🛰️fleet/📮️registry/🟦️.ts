import {createHash,randomBytes} from "node:crypto";
import {mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,writeFileSync} from "node:fs";
import {dirname,join} from "node:path";
import {casRootV1} from "../../../../../🏗️build/🗄️cas/🗃️store/🟦️.ts";
import {FLEET_POLICY_V1} from "../🪪️identity/🟦️.ts";

type Environment=Readonly<Record<string,string|undefined>>;
const policy=FLEET_POLICY_V1.uploader,secret=/(TOKEN|SECRET|PASSWORD|CREDENTIAL|API_?KEY|PRIVATE)/iu,needed=/^(SEMIO_.*|XDG_.*|PATH|HOME|USER|USERPROFILE|LOCALAPPDATA|APPDATA|TMPDIR|TEMP|TMP|SystemRoot|LANG)$/u;

/** 📮️ One cache root of this machine whose spool the machine's long-lived sidecar drains when no upload worker lives: the root, the cache identity (`endpoint|generation|token hash`) the jobs were queued under and the environment an upload worker needs, without any name that looks like a secret. */
export type SpoolRegistrationV1=Readonly<{version:1;root:string;identity:string;environment:Readonly<Record<string,string>>}>;

/** 🗂️ Where the registrations of the machine live: `unit-spools` below the host store root (`SEMIO_CAS_ROOT`), so every workspace, scratch root and lane root of the user is found by one sweep. */
export const spoolRegistryDirectoryV1=(environment:Environment):string=>join(casRootV1(environment),FLEET_POLICY_V1.layout.spoolRegistry);

const fileOf=(root:string,environment:Environment):string=>join(spoolRegistryDirectoryV1(environment),createHash("sha256").update(root).digest("hex").slice(0,32)+".json");

/** 🔓️ The environment an upload worker is started with: the `SEMIO_*` names of the build (the cache endpoint, access, generation, roots) and the few host names a process needs, minus every name that looks like a secret (Cargo's per-unit variables stay out: they differ with every unit and the worker needs none). */
export function publicEnvironmentV1(environment:Environment):Readonly<Record<string,string>>{
 const kept:Record<string,string>={};
 for(const [name,value] of Object.entries(environment))if(value!==undefined&&needed.test(name)&&!secret.test(name))kept[name]=value;
 return kept;
}

/** 📝️ Records that the spool of `root` may hold jobs and how to drain them (atomically; rewritten when the identity changed and otherwise at most every `uploader.registryRefreshMilliseconds`). Never throws, never waits for the network. */
export function registerSpoolV1(root:string,environment:Environment,identity:string):void{
 try{
  const file=fileOf(root,environment);
  try{if((JSON.parse(readFileSync(file,"utf8")) as SpoolRegistrationV1).identity===identity&&Date.now()-statSync(file).mtimeMs<policy.registryRefreshMilliseconds)return;}catch{}
  const row:SpoolRegistrationV1={version:1,root,identity,environment:publicEnvironmentV1(environment)};
  mkdirSync(dirname(file),{recursive:true});
  const temporary=file+"."+randomBytes(4).toString("hex")+".tmp";writeFileSync(temporary,JSON.stringify(row));renameSync(temporary,file);
 }catch{}
}

/** 📖️ Every registration of the machine with the age of its file; a file that is not of the closed shape is ignored. */
export function readSpoolRegistryV1(environment:Environment):readonly Readonly<{registration:SpoolRegistrationV1;ageMs:number;file:string}>[]{
 const directory=spoolRegistryDirectoryV1(environment),rows:{registration:SpoolRegistrationV1;ageMs:number;file:string}[]=[];
 let names:string[]=[];try{names=readdirSync(directory).filter(name=>name.endsWith(".json"));}catch{}
 for(const name of names){
  const file=join(directory,name);
  try{
   const value=JSON.parse(readFileSync(file,"utf8")) as SpoolRegistrationV1;
   if(value.version!==1||typeof value.root!=="string"||typeof value.identity!=="string"||typeof value.environment!=="object"||value.environment===null)continue;
   rows.push({registration:value,ageMs:Date.now()-statSync(file).mtimeMs,file});
  }catch{}
 }
 return rows;
}

/** 🗑️ Forgets one registration (a root that is gone, or whose spool has been empty for `uploader.registryRetentionMilliseconds`: the next queued job registers it again). */
export function forgetSpoolV1(file:string):void{try{rmSync(file,{force:true});}catch{}}
