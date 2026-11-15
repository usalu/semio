import {spawnSync} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {existsSync,mkdirSync,readdirSync,readFileSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,extname,join,relative,resolve} from "node:path";
import {pathToFileURL} from "node:url";
import {TranspilerV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🔬️source/🟦️.ts";
import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.seal,digestPattern=new RegExp(`^[0-9a-f]{${policy.digestCharacters}}$`,"u");
const loaders:Readonly<Record<string,"ts"|"tsx"|"js"|"jsx">>=Object.freeze({".ts":"ts",".mts":"ts",".cts":"ts",".tsx":"tsx",".js":"js",".mjs":"js",".cjs":"js",".jsx":"jsx"});
const staticImports=/(?:^|[;{}\n])[ \t]*(?:import(?![ \t]+type[ \t{*])(?:[^"';]*?[ \t}*]from)?|export(?![ \t]+type[ \t{*])[ \t]*(?:\*(?:[ \t]+as[ \t]+[\w$]+)?|\{[^}"';]*\})[ \t]*from)[ \t]*(["'])(\.{1,2}\/[^"'\n]+)\1/gu;
const fileLiterals=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1/gu;

/** 🚧️ One place that keeps a module set from being sealed: a file that does not parse, does not load or changed while it was read (`line` and `column` are 0 when the whole file or set is meant). */
export type SealBlockV1=Readonly<{path:string;line:number;column:number;message:string}>;

/** 🏛️ A content-addressed, read-only copy of a module set: `directory` holds the files below their common ancestor, `entries` name the sealed copies of the requested entry files in order, and `stale` lists what blocked a newer capture when the last good snapshot was served instead. */
export type SealedModulesV1=Readonly<{digest:string;directory:string;entries:readonly string[];files:number;fresh:boolean;stale:readonly SealBlockV1[];removed:readonly string[]}>;

/** ⚙️ Seams of one seal: the pause between attempts, the syntax check over a capture and the load check over a staged copy (the failure text or `undefined`). */
export type SealOptionsV1=Readonly<{attempts?:number;settleMilliseconds?:number;checked?:readonly string[];runtime?:string;nowMs?:number;sleep?:(milliseconds:number)=>void;parse?:(files:ReadonlyMap<string,Buffer>)=>readonly SealBlockV1[];boot?:(runtime:string,entry:string)=>string|undefined}>;

const hash=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");
const slashed=(path:string):string=>path.replaceAll("\\","/");
const sleepSyncV1=(milliseconds:number):void=>{Atomics.wait(new Int32Array(new SharedArrayBuffer(4)),0,0,milliseconds);};

/** 📥️ Reads every file the entries can run: static imports and quoted file literals (lazy `require`, `new URL`, `with {type:"json"}`) of every code file, transitively. A file a static import names that cannot be read fails the capture. */
function captureV1(entries:readonly string[]):Map<string,Buffer>{
 const files=new Map<string,Buffer>(),pending=[...entries];let bytes=0;
 while(pending.length){
  const path=pending.pop()!;if(files.has(path))continue;
  const data=readFileSync(path);files.set(path,data);bytes+=data.length;
  if(files.size>policy.maximumFiles||bytes>policy.maximumBytes)throw Error(`the module set exceeds ${policy.maximumFiles} files or ${policy.maximumBytes} bytes`);
  if(!(extname(path) in loaders))continue;
  const text=data.toString("utf8");
  for(const match of text.matchAll(staticImports))pending.push(resolve(dirname(path),match[2]!));
  for(const match of text.matchAll(fileLiterals)){const target=resolve(dirname(path),match[2]!);if(!/\.d\.[cm]?ts$/u.test(target)&&!files.has(target)&&existsSync(target))pending.push(target);}
 }
 return files;
}

/** 🔬️ Parses every code file with the runtime's own transpiler and every JSON file with the JSON parser, and answers every place that does not parse (bounded). */
export function sealSyntaxRowsV1(files:ReadonlyMap<string,Buffer>):readonly SealBlockV1[]{
 const rows:SealBlockV1[]=[],compilers=new Map<string,TranspilerV1>(),add=(path:string,line:number,column:number,message:string)=>{if(rows.length<policy.maximumBlockedRows)rows.push({path,line,column,message:message.replace(/\s+/gu," ").trim().slice(0,300)||"syntax error"});};
 for(const [path,bytes] of files){
  const text=bytes.toString("utf8");
  if(extname(path)===".json"){try{JSON.parse(text);}catch(error){add(path,0,0,String((error as Error).message));}continue;}
  const loader=loaders[extname(path)];if(!loader)continue;
  let compiler=compilers.get(loader);if(!compiler){compiler=new TranspilerV1({loader,target:"runtime"});compilers.set(loader,compiler);}
  try{compiler.transformSync(text.replace(/^#![^\r\n]*/u,""));}catch(error){
   const list=(error as{errors?:readonly unknown[]}).errors?.length?(error as{errors:readonly unknown[]}).errors!:[error];
   for(const item of list){const position=(item as{position?:{line?:number;column?:number}}).position;add(path,position?.line??0,position?.column??0,String((item as Error).message));}
  }
 }
 return rows;
}

/** 🥾️ Imports one sealed entry in a child of the runtime: a renamed export, a missing file or a throwing top level of a half-finished edit fails here, before any compiler call runs it. */
function bootV1(runtime:string,entry:string):string|undefined{
 const run=spawnSync(runtime,["--eval","import(process.env.SEMIO_SEAL_BOOT).then(()=>process.exit(0),error=>{process.stderr.write(String(error&&error.message||error).slice(0,400));process.exit(1);})"],{env:{...process.env,SEMIO_SEAL_BOOT:pathToFileURL(entry).href},encoding:"utf8",timeout:60000,windowsHide:true});
 return run.status===0?undefined:String(run.error?.message??run.stderr??"does not load").replace(/\s+/gu," ").trim()||"does not load";
}

const same=(left:ReadonlyMap<string,Buffer>,right:ReadonlyMap<string,Buffer>):boolean=>left.size===right.size&&[...left].every(([path,bytes])=>right.get(path)?.equals(bytes)===true);
const touch=(path:string,nowMs:number):void=>{try{utimesSync(path,nowMs/1000,nowMs/1000);}catch{}};
const completeV1=(directory:string):boolean=>existsSync(join(directory,policy.marker));

/** 🧹️ Removes snapshots that are neither current, among the newest `keepSnapshots`, nor used within `retainMilliseconds`, and stale stages; answers the removed digests. */
function collectV1(store:string,current:string,nowMs:number):string[]{
 const removed:string[]=[];
 try{
  const rows=readdirSync(store).filter(name=>digestPattern.test(name)).map(name=>({name,at:(()=>{try{return statSync(join(store,name)).mtimeMs;}catch{return 0;}})()})).sort((a,b)=>b.at-a.at);
  rows.forEach((row,index)=>{if(row.name===current||index<policy.keepSnapshots||nowMs-row.at<policy.retainMilliseconds)return;try{rmSync(join(store,row.name),{recursive:true,force:true});removed.push(row.name);}catch{}});
  for(const name of readdirSync(store))if(name.startsWith(".stage-")){try{if(nowMs-statSync(join(store,name)).mtimeMs>3600000)rmSync(join(store,name),{recursive:true,force:true});}catch{}}
 }catch{}
 return removed;
}

function publishV1(store:string,pointer:string,files:ReadonlyMap<string,Buffer>,entries:readonly string[],options:SealOptionsV1,nowMs:number):SealedModulesV1{
 let root=dirname(entries[0]!);for(const path of files.keys())while(relative(root,path).startsWith(".."))root=dirname(root);
 const rows=[...files].map(([path,bytes])=>({path:slashed(relative(root,path)),bytes})).sort((a,b)=>a.path<b.path?-1:1);
 const digest=hash(rows.map(row=>`${row.path}\0${hash(row.bytes)}\n`).join("")).slice(0,policy.digestCharacters),directory=join(store,digest),relativeEntries=entries.map(entry=>slashed(relative(root,entry)));
 let fresh=false;
 if(!completeV1(directory)){
  rmSync(directory,{recursive:true,force:true});
  const stage=join(store,".stage-"+randomBytes(6).toString("hex"));
  try{
   for(const row of rows){const target=join(stage,...row.path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,row.bytes,{mode:0o444});}
   writeFileSync(join(stage,policy.marker),JSON.stringify({version:1,digest,entries:relativeEntries,files:rows.length}));
   for(const entry of options.checked??[]){const failure=(options.boot??bootV1)(options.runtime??process.execPath,join(stage,...slashed(relative(root,entry)).split("/")));if(failure!==undefined)throw Error(`${relative(root,entry)} does not load: ${failure}`);}
   try{renameSync(stage,directory);fresh=true;}catch(error){if(!completeV1(directory))throw error;}
  }finally{rmSync(stage,{recursive:true,force:true});}
 }
 touch(directory,nowMs);
 const temporary=pointer+"."+randomBytes(4).toString("hex");writeFileSync(temporary,JSON.stringify({version:1,digest,entries:relativeEntries}));renameSync(temporary,pointer);
 return Object.freeze({digest,directory,entries:Object.freeze(relativeEntries.map(entry=>join(directory,...entry.split("/")))),files:rows.length,fresh,stale:Object.freeze([]),removed:Object.freeze(collectV1(store,digest,nowMs))});
}

function lastGoodV1(store:string,pointer:string,stale:readonly SealBlockV1[],nowMs:number):SealedModulesV1|undefined{
 try{
  const value=JSON.parse(readFileSync(pointer,"utf8")) as{digest?:unknown;entries?:unknown};
  if(typeof value.digest!=="string"||!digestPattern.test(value.digest)||!Array.isArray(value.entries)||!value.entries.every(entry=>typeof entry==="string"))return undefined;
  const directory=join(store,value.digest);if(!completeV1(directory))return undefined;
  touch(directory,nowMs);
  return Object.freeze({digest:value.digest,directory,entries:Object.freeze((value.entries as string[]).map(entry=>join(directory,...entry.split("/")))),files:0,fresh:false,stale:Object.freeze([...stale]),removed:Object.freeze([])});
 }catch{return undefined;}
}

/** 🏛️ Seals the module set the entries can run into `<root>/sealed/<digest>` and answers the sealed copies of the entries: a session that runs them never sees a later edit of the live files. A capture is accepted only when every file parses, the entries named by `checked` load from the staged copy and a second read finds the same bytes, so a half-finished edit is retried after `settleMilliseconds` (up to `attempts`) and never sealed; when no attempt succeeds the last good snapshot of these entries is served (`stale` names what blocked), and without one this throws with the first blocking place. Snapshots are published by one atomic rename; peers sealing the same bytes share one directory. @see ../🧬️schema/🔣️.json */
export function sealModulesV1(root:string,entries:readonly string[],options:SealOptionsV1={}):SealedModulesV1{
 const store=join(root,policy.directory),live=entries.map(entry=>resolve(entry)),pointer=join(store,policy.pointerPrefix+hash(live.join("\n")).slice(0,policy.digestCharacters)+".json"),nowMs=options.nowMs??Date.now();
 const attempts=options.attempts??policy.attempts,sleep=options.sleep??sleepSyncV1,parse=options.parse??sealSyntaxRowsV1;
 let blocked:readonly SealBlockV1[]=[];
 mkdirSync(store,{recursive:true});
 for(let attempt=0;attempt<attempts;attempt++){
  if(attempt)sleep(options.settleMilliseconds??policy.settleMilliseconds);
  try{
   const first=captureV1(live),rows=parse(first);
   if(rows.length){blocked=rows;continue;}
   if(!same(first,captureV1(live))){blocked=[{path:"",line:0,column:0,message:"the files changed while they were read"}];continue;}
   return publishV1(store,pointer,first,live,options,nowMs);
  }catch(error){blocked=[{path:"",line:0,column:0,message:String((error as Error).message).replace(/\s+/gu," ").trim().slice(0,300)}];}
 }
 const last=lastGoodV1(store,pointer,blocked,nowMs);if(last)return last;
 const first=blocked[0]!;throw Error(`no sealed snapshot of the tooling could be made after ${attempts} attempt(s): ${first.path?`${first.path}:${first.line}:${first.column} `:""}${first.message}`);
}
