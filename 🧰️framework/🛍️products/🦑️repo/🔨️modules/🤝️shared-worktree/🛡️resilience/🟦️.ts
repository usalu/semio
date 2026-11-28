import {createHash,randomBytes} from "node:crypto";
import {spawn} from "node:child_process";
import {chmod,mkdir,open,readdir,readFile,realpath,rename,rm,stat,truncate,writeFile} from "node:fs/promises";
import {dirname,join,relative,resolve,sep} from "node:path";
import {pathToFileURL} from "node:url";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {importClosureV1,moduleSourcesV1,walkSourcesV1,StoreStalledV1,type FailureSummaryV1} from "./🧱️attestation-support/🟦️.ts";
import type {SidecarStateV1} from "../🔁️replica/🧷️sidecars/🟦️.ts";
import type {TrustV1} from "../🔏️trust/🟦️.ts";
import {processHolderAliveV1} from "../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

/** 🧾️ The report of the self-check: whether every module of the replica loaded and every embedded schema and text catalogue is well formed, how many of each were checked, the digest of the checked code (filled in by the command) and the first error. */
export type SelfCheckV1=Readonly<{version:1;ok:boolean;digest:string|null;modules:number;schemas:number;texts:number;error:string|null}>;

/** 🔖️ One sealed copy of the code a daemon proved to start: `files` are the repository-relative paths with their hashes. */
export type SealedCodeV1=Readonly<{version:1;digest:string;createdMs:number;moduleRoot:string;files:readonly Readonly<{path:string;bytes:number;sha256:string;executable:boolean}>[]}>;

/** 📚️ One sealed snapshot on disk, newest first in listings. */
export type CodeSnapshotV1=Readonly<{digest:string;createdMs:number;directory:string}>;

/** 🩹️ What the daemon shows about itself without asking the store: the store state, the refused new code and the recurring failures per kind. */
export type HealthV1=Readonly<{version:1;wallMs:number;pid:number;code:string;store:"ok"|"stalled"|"unreachable";storeSinceMs:number;pending:Readonly<{code:string;ok:false;reason:string}>|null;failures:readonly FailureSummaryV1[];trust:TrustV1;sidecars?:readonly SidecarStateV1[]}>;

/** 🔤️ The native runtime error codes treated as sharing failures by target mutations. */
export type TargetSharingCodeV1=keyof typeof targetSharingKindsV1;

/** 🧷️ The failed native target mutation. */
export type TargetSharingOperationV1=keyof typeof targetSharingOperationKindsV1;

/** 🪪️ The last sampled target metadata for an EPERM from a staged rename. */
export type RenameEpermTargetV1=keyof typeof renameEpermTargetKindsV1;

/** 🧪️ A pure verdict on a deadline-bounded store state. */
export type StoreStateV1="ok"|"stalled"|"unreachable";

const validators=new Map<string,CompiledJsonSchemaSubset>(),tokenOf=():string=>randomBytes(8).toString("hex");
const bytewise=(left:string,right:string):number=>Buffer.compare(Buffer.from(left),Buffer.from(right));
const posix=(path:string):string=>path.split(sep).join("/");
const gone=(error:unknown):boolean=>(error as {code?:string}).code==="ENOENT";
const sha=(data:Uint8Array|string):string=>createHash("sha256").update(data).digest("hex");

/** 🛡️ Validates a value against one definition of the resilience schema. */
export function requireResilienceShapeV1(definition:"SealedCode"|"SelfCheck"|"Health"|"Failure"|"History"|"RefusedPaths"|"Beat"|"BlockedRestart"|"Fallback"|"BootStage"|"Fixture",value:unknown,message:string):void{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🧰️ The companions of the code: files that no module imports but that provisioning runs from a checkout (the two provision scripts and the three files of the boot program), as `/` paths relative to the module directory. A seal carries them next to the imported code (they are part of the seal and its hashes, not of the code digest), so the kit and a machine with an old checkout get them from the same sealed files. */
export const sealCompanionsV1:readonly string[]=Object.freeze(["🗺️topology/🥾️provision/🐚️.sh","🗺️topology/🥾️provision/🔵️.ps1","🔁️replica/🥾️entry/🟨️.mjs","🔁️replica/🥾️entry/🐚️.sh","🔁️replica/🥾️entry/🔵️.ps1"]);

/** 🧰️ The companions that exist below `moduleRoot`, sorted, as `/` paths relative to it. */
export async function existingCompanionsV1(moduleRoot:string):Promise<string[]>{
 const found:string[]=[];
 for(const path of sealCompanionsV1)if((await stat(resolve(moduleRoot,...path.split("/"))).catch(()=>undefined))?.isFile()===true)found.push(path);
 return found.sort(bytewise);
}

/** 🕳️ Returns the message of the first problem of a text catalogue (`{languages,messages}` with every message in every language and the same `{name}` placeholders everywhere), or `null`. */
export function textsProblemV1(catalogue:unknown):string|null{
 if(typeof catalogue!=="object"||catalogue===null)return "catalogue must be an object";
 const {languages,messages}=catalogue as {languages?:unknown;messages?:unknown};
 if(!Array.isArray(languages)||languages.length===0||languages.some(item=>typeof item!=="string"||item===""))return "languages must be a non-empty list of names";
 if(new Set(languages).size!==languages.length)return "languages must be unique";
 if(typeof messages!=="object"||messages===null||Array.isArray(messages))return "messages must be an object";
 for(const [key,message] of Object.entries(messages)){
  if(typeof message!=="object"||message===null)return `message ${key} must be an object`;
  const keys=Object.keys(message);
  if(keys.length!==languages.length||languages.some(language=>!keys.includes(language as string)))return `message ${key} must have exactly the languages ${languages.join(",")}`;
  let reference:string|undefined;
  for(const language of languages as string[]){
   const text=(message as Record<string,unknown>)[language];
   if(typeof text!=="string"||text==="")return `message ${key} has no text in ${language}`;
   const names=[...new Set([...text.matchAll(/\{([a-z0-9]+)\}/gu)].map(match=>match[1]!))].sort().join(",");
   if(reference===undefined)reference=names;else if(reference!==names)return `message ${key} differs in its placeholders between the languages`;
  }
 }
 return null;
}

/** 🔬️ Loads every module of the replica, compiles every definition of every embedded schema and validates every text catalogue below `root`; it opens no store and no worktree state. Never throws: the first problem comes back in `error`. */
export async function selfCheckV1(root:string,signal?:AbortSignal):Promise<SelfCheckV1>{
 let modules=0,schemas=0,texts=0;
 try{
  const main=process.argv[1]===undefined?undefined:await realpath(process.argv[1]).catch(()=>undefined);
  for(const path of await moduleSourcesV1(root)){
   signal?.throwIfAborted();
   if(main!==undefined&&await realpath(join(root,...path.split("/"))).catch(()=>undefined)===main){modules++;continue;}
   try{await import(pathToFileURL(join(root,...path.split("/"))).href);}catch(error){throw Error(`module ${path} does not load: ${error instanceof Error?error.message:String(error)}`);}
   modules++;
  }
  const documents:string[]=[];
  await walkSourcesV1(root,"",documents,name=>name==="🔣️.json");
  for(const path of documents.sort(bytewise)){
   signal?.throwIfAborted();
   const parts=path.split("/"),kind=parts[parts.length-2];
   if(kind!=="🧬️schema"&&kind!=="📖️texts")continue;
   let document:unknown;
   try{document=JSON.parse(await readFile(join(root,...parts),"utf8"));}catch(error){throw Error(`${path} is not valid JSON: ${(error as Error).message}`);}
   if(kind==="📖️texts"){
    const problem=textsProblemV1(document);
    if(problem!==null)throw Error(`${path}: ${problem}`);
    texts++;
   }else{
    const definitions=(document as {definitions?:Record<string,unknown>}).definitions;
    if(typeof definitions!=="object"||definitions===null)throw Error(`${path}: schema without definitions`);
    for(const name of Object.keys(definitions)){try{compileJsonSchemaSubset({$ref:`#/definitions/${name}`},document as never);}catch(error){throw Error(`${path}: definition ${name} does not compile: ${(error as Error).message}`);}}
    schemas++;
   }
  }
  return {version:1,ok:true,digest:null,modules,schemas,texts,error:null};
 }catch(error){return {version:1,ok:false,digest:null,modules,schemas,texts,error:(error instanceof Error?error.message:String(error)).slice(0,2048)};}
}

/** 🚀️ The outcome of a self-check child process. */
export type SelfCheckRunV1=Readonly<{ok:boolean;reason:string;digest:string|null}>;

/** 🧫️ Runs the self-check in a fresh process: `command` with `args` (the same runtime on the replica script with `self-check`) in `cwd`, killed after `timeoutMs` or when `signal` aborts; success is exit code 0 with a report line that says `ok`. Nothing else is shared with the daemon. */
export function runSelfCheckV1(options:Readonly<{command:string;args:readonly string[];cwd:string;timeoutMs:number;signal?:AbortSignal;environment?:Readonly<Record<string,string|undefined>>}>):Promise<SelfCheckRunV1>{
 return new Promise(done=>{
  let finished=false,out="",err="";
  let child:ReturnType<typeof spawn>;
  const finish=(result:SelfCheckRunV1):void=>{if(finished)return;finished=true;clearTimeout(timer);options.signal?.removeEventListener("abort",abort);done(result);};
  const kill=():void=>{try{child?.kill("SIGKILL");}catch{}};
  const timer=setTimeout(()=>{kill();finish({ok:false,reason:`self-check timed out after ${options.timeoutMs} ms`,digest:null});},options.timeoutMs);
  const abort=():void=>{kill();finish({ok:false,reason:"self-check cancelled",digest:null});};
  if(options.signal?.aborted===true){abort();return;}
  options.signal?.addEventListener("abort",abort,{once:true});
  try{child=spawn(options.command,[...options.args],{cwd:options.cwd,env:{...process.env,...options.environment} as NodeJS.ProcessEnv,stdio:["ignore","pipe","pipe"],windowsHide:true});}
  catch(error){finish({ok:false,reason:`self-check could not start: ${(error as Error).message}`,digest:null});return;}
  child.stdout?.on("data",chunk=>{if(out.length<65536)out+=String(chunk);});
  child.stderr?.on("data",chunk=>{if(err.length<65536)err+=String(chunk);});
  child.on("error",error=>{finish({ok:false,reason:`self-check could not start: ${error.message}`,digest:null});});
  child.on("close",code=>{
   let report:SelfCheckV1|undefined;
   for(const line of out.split("\n").reverse()){
    if(!line.startsWith("{"))continue;
    try{const value=JSON.parse(line) as unknown;requireResilienceShapeV1("SelfCheck",value,"Closed self-check report required");report=value as SelfCheckV1;break;}catch{}
   }
   if(code===0&&report?.ok===true){finish({ok:true,reason:"",digest:report.digest});return;}
   const detail=report?.error??err.trim().split("\n").filter(line=>line!=="").pop()??"";
   finish({ok:false,reason:(`self-check ${code===null?"was killed":`exited with ${code}`}${detail===""?"":`: ${detail}`}`).slice(0,512),digest:report?.digest??null});
  });
 });
}

/** 📁️ The directory below the worktree that holds the sealed code snapshots. */
export function codeDirectoryV1(root:string):string{return join(root,".cache","f","code");}

/** 📚️ Lists the sealed snapshots of a worktree, newest first; a directory without a readable seal is not listed. */
export async function listCodeSnapshotsV1(root:string):Promise<CodeSnapshotV1[]>{
 const base=codeDirectoryV1(root),found:CodeSnapshotV1[]=[];
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!/^[0-9a-f]{64}$/u.test(name))continue;
  try{
   const seal=JSON.parse(await readFile(join(base,name,"sealed.json"),"utf8")) as unknown;
   requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
   if((seal as SealedCodeV1).digest===name)found.push({digest:name,createdMs:(seal as SealedCodeV1).createdMs,directory:join(base,name)});
  }catch{}
 }
 return found.sort((left,right)=>right.createdMs-left.createdMs||(left.digest<right.digest?-1:1));
}

/** ✂️ Keeps the newest `keep` sealed snapshots and removes the others and every unfinished `.tmp-` directory older than an hour; returns the removed digests. */
export async function pruneCodeSnapshotsV1(root:string,keep:number,now:number=Date.now()):Promise<string[]>{
 const removed:string[]=[],base=codeDirectoryV1(root);
 for(const snapshot of (await listCodeSnapshotsV1(root)).slice(Math.max(0,keep))){await rm(snapshot.directory,{recursive:true,force:true});removed.push(snapshot.digest);}
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!name.startsWith(".tmp-"))continue;
  const info=await stat(join(base,name)).catch(()=>undefined);
  if(info!==undefined&&now-info.mtimeMs>3_600_000)await rm(join(base,name),{recursive:true,force:true});
 }
 return removed;
}

/** 🔒️ The inputs of a seal: the worktree root, the module root inside it, the digest the checked code has, how to compute that digest of a copy, the clock, how many snapshots to keep (2) and cancellation. */
export type SealOptionsV1=Readonly<{root:string;moduleRoot:string;digest:string;digestOf:(moduleRoot:string)=>Promise<string>;now?:()=>number;keep?:number;signal?:AbortSignal}>;

/** ⚙️ Whether a file has an execute bit, `false` on a platform without one. */
export const executableBitV1=async(file:string):Promise<boolean>=>process.platform==="win32"?false:((await stat(file)).mode&0o111)!==0;

/** 🧰️ Whether the companions of a seal are the companions on disk now: every companion that exists below `moduleRoot` is in the seal with the same hash, and the seal holds none that is gone; the execute bit of every sealed file is the one on disk (a `chmod` alone changes no digest). A seal written before the companions or the bits existed is not current. */
export async function companionsCurrentV1(root:string,moduleRoot:string,seal:SealedCodeV1):Promise<boolean>{
 const prefix=posix(relative(root,moduleRoot)),listed=new Map(seal.files.map(file=>[file.path,file.sha256])),present=await existingCompanionsV1(moduleRoot);
 if(process.platform!=="win32")for(const file of seal.files)if(await executableBitV1(resolve(root,...file.path.split("/"))).catch(()=>undefined)!==file.executable)return false;
 for(const path of sealCompanionsV1){
  const full=`${prefix}/${path}`,data=present.includes(path)?await readFile(resolve(moduleRoot,...path.split("/"))).catch(()=>undefined):undefined;
  if(data===undefined?listed.has(full):listed.get(full)!==sha(data))return false;
 }
 return true;
}

/** 🔒️ Seals a copy of the code that was just checked into `.cache/f/code/<digest>/`: the module sources and every file they import, mirrored below `files/` at their repository-relative paths, then `sealed.json` with the hashes, written last into a private `.tmp-` directory that is renamed into place, so a snapshot either is complete or does not exist. The digest of the copy must equal `digest` (else the code changed while it was copied and nothing is sealed). An existing seal of the same digest is only refreshed to be the newest. Afterwards only the newest `keep` snapshots stay. */
export async function sealCodeV1(options:SealOptionsV1):Promise<Readonly<{created:boolean;directory:string|null;reason:string|null}>>{
 const {root,moduleRoot,digest,signal}=options,now=options.now??Date.now,base=codeDirectoryV1(root),target=join(base,digest),moduleRelative=posix(relative(root,moduleRoot));
 if(moduleRelative.startsWith("..")||moduleRelative==="")return {created:false,directory:null,reason:"module-outside-root"};
 const existing=await readFile(join(target,"sealed.json"),"utf8").catch(()=>undefined);
 if(existing!==undefined){
  try{
   const seal=JSON.parse(existing) as SealedCodeV1;
   requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
   if(!await companionsCurrentV1(root,moduleRoot,seal))throw Error("stale-companions");
   const temporary=join(target,`sealed.${tokenOf()}`);
   await writeFile(temporary,JSON.stringify({...seal,createdMs:now()}));await rename(temporary,join(target,"sealed.json"));
   await pruneCodeSnapshotsV1(root,options.keep??2,now());
   return {created:false,directory:target,reason:null};
  }catch{await rm(target,{recursive:true,force:true});}
 }
 const sources=await moduleSourcesV1(moduleRoot),all=[...new Set([...sources,...await importClosureV1(moduleRoot,sources),...await existingCompanionsV1(moduleRoot)])].map(path=>resolve(moduleRoot,...path.split("/")));
 const temporary=join(base,`.tmp-${tokenOf()}`),copied=join(temporary,"files"),files:{path:string;bytes:number;sha256:string;executable:boolean}[]=[];
 try{
  for(const file of all){
   signal?.throwIfAborted();
   const path=posix(relative(root,file));
   if(path.startsWith(".."))throw Error(`outside-root:${path}`);
   const destination=join(copied,...path.split("/")),data=await readFile(file),executable=await executableBitV1(file);
   await mkdir(dirname(destination),{recursive:true});
   await writeFile(destination,data,{mode:executable?0o755:0o644});
   if(executable)await chmod(destination,0o755);
   files.push({path,bytes:data.length,sha256:sha(data),executable});
  }
  if(await options.digestOf(join(copied,...moduleRelative.split("/")))!==digest){await rm(temporary,{recursive:true,force:true});return {created:false,directory:null,reason:"code-changed-while-copying"};}
  const seal:SealedCodeV1={version:1,digest,createdMs:now(),moduleRoot:moduleRelative,files:files.sort((left,right)=>bytewise(left.path,right.path))};
  requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
  const handle=await open(join(temporary,"sealed.json"),"w");
  try{await handle.writeFile(JSON.stringify(seal));await handle.sync();}finally{await handle.close();}
  try{await rename(temporary,target);}catch(error){if(!(await stat(join(target,"sealed.json")).then(()=>true,()=>false)))throw error;await rm(temporary,{recursive:true,force:true});}
 }catch(error){
  await rm(temporary,{recursive:true,force:true});
  if(signal?.aborted===true)throw error;
  return {created:false,directory:null,reason:(error instanceof Error?error.message:String(error)).slice(0,256)};
 }
 await pruneCodeSnapshotsV1(root,options.keep??2,now());
 return {created:true,directory:target,reason:null};
}

/** 🔎️ Re-hashes every file of a sealed snapshot against its seal; the first mismatch or missing file comes back as the reason, `null` when the snapshot is intact. */
export async function verifyCodeSnapshotV1(directory:string):Promise<string|null>{
 try{
  const seal=JSON.parse(await readFile(join(directory,"sealed.json"),"utf8")) as SealedCodeV1;
  requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
  for(const file of seal.files){
   const data=await readFile(join(directory,"files",...file.path.split("/"))).catch(()=>undefined);
   if(data===undefined)return `missing ${file.path}`;
   if(data.length!==file.bytes||sha(data)!==file.sha256)return `changed ${file.path}`;
  }
  return null;
 }catch(error){return error instanceof Error?error.message:String(error);}
}

/** ⏱️ The inputs of the store guard: the reachability probe (a `stat` of the store root), the deadline after which an unanswered store call counts as stalled, the clock and the state-change callback. */
export type StoreGuardOptionsV1=Readonly<{probe:()=>Promise<boolean>;deadlineMs:number;now:()=>number;changed:(state:StoreStateV1)=>void}>;

/** 🛑️ A turn of store work: it finished (`done` with its value), it was abandoned because the store stopped answering (`stalled`), or it was not started because an earlier turn is still outstanding (`busy`). */
export type StoreTurnV1<T>=Readonly<{status:"done";value:T}>|Readonly<{status:"stalled"|"busy"}>;

/** 🚧️ A deadline guard around a store that may hang: a blocked filesystem call cannot be cancelled in Node or Bun (it holds a worker thread until the kernel returns), so the guard never waits for one. `reachable` probes the store with at most one outstanding probe and answers `false` once a probe is older than the deadline; `turn` runs a body of store work with an abort signal and abandons it (aborting the signal, so no new call starts once the blocked one returns) as soon as a probe has been outstanding for the deadline and the work has not called `beat` within the deadline either (an upload that moves bytes, flushes or verifies is alive, whatever a probe on the same saturated mount says; a multi-gigabyte pack was abandoned and restarted from zero in a loop before); `call` runs one store call under the deadline. While the state is not `ok` no turn and no call starts, so at most the already blocked calls plus one probe are outstanding. When a probe answers again and the abandoned work has returned the state is `ok` again without a restart. */
export type StoreGuardV1={state():StoreStateV1;since():number;beat():void;reachable():Promise<boolean>;turn<T>(work:(signal:AbortSignal)=>Promise<T>,outer?:AbortSignal):Promise<StoreTurnV1<T>>;call<T>(work:()=>Promise<T>):Promise<T>;outstanding():number};

const leakedCallsMaximumV1=4;

/** 🏭️ Creates a store guard. A call or probe that never returns holds a worker thread for ever: it is abandoned and forgotten at once, so it cannot hold the guard (`turn` and `call` are free again as soon as the store answers a probe, five deadlines after the abandonment), but at most four such leaked threads are tolerated before the guard stays stalled, because the thread pool must keep threads for the calls that work; a probe that has not answered after three deadlines is orphaned the same way and a new one is started, so a store that hangs one call does not stay stalled for ever. */
export function createStoreGuardV1(options:StoreGuardOptionsV1):StoreGuardV1{
 let state:StoreStateV1="ok",since=options.now(),beatMs=options.now(),probing:{promise:Promise<void>;startedAt:number;orphan:boolean}|undefined,turning=false,calling=false,abandoned=0,abandonedAt=0,leaked=0;
 const set=(next:StoreStateV1):void=>{if(next===state)return;state=next;since=options.now();options.changed(next);};
 const wait=(milliseconds:number):Readonly<{promise:Promise<"late">;cancel:()=>void}>=>{let timer:ReturnType<typeof setTimeout>|undefined;return {promise:new Promise<"late">(done=>{timer=setTimeout(()=>{done("late");},Math.max(0,milliseconds));timer.unref?.();}),cancel:()=>{clearTimeout(timer);}};};
 const mayRecover=():boolean=>abandoned===0||options.now()-abandonedAt>5*options.deadlineMs;
 const startProbe=():void=>{
  if(probing!==undefined){
   if(options.now()-probing.startedAt<=3*options.deadlineMs||leaked>=leakedCallsMaximumV1)return;
   probing.orphan=true;leaked++;probing=undefined;
  }
  const entry:{promise:Promise<void>;startedAt:number;orphan:boolean}={promise:Promise.resolve(),startedAt:options.now(),orphan:false};
  const settle=(ok:boolean):void=>{
   if(entry.orphan){leaked--;return;}
   probing=undefined;
   if(ok){if(state!=="ok"&&mayRecover())set("ok");}else set("unreachable");
  };
  entry.promise=Promise.resolve().then(options.probe).then(settle,()=>{settle(false);});
  probing=entry;
 };
 const abandon=():void=>{abandoned++;abandonedAt=options.now();set("stalled");};
 return {
  state:()=>state,
  since:()=>since,
  beat:()=>{beatMs=options.now();},
  outstanding:()=>(turning?1:0)+(calling?1:0)+(probing===undefined?0:1),
  async reachable(){
   startProbe();
   const current=probing;
   if(current===undefined)return state==="ok";
   const timer=wait(options.deadlineMs-(options.now()-current.startedAt));
   const winner=await Promise.race([current.promise.then(()=>"done" as const),timer.promise]);
   timer.cancel();
   if(winner==="late"&&probing===current&&state==="ok")set("stalled");
   return state==="ok";
  },
  async turn<T>(work:(signal:AbortSignal)=>Promise<T>,outer?:AbortSignal):Promise<StoreTurnV1<T>>{
   if(state!=="ok"||leaked>=leakedCallsMaximumV1)return {status:"stalled"};
   if(turning)return {status:"busy"};
   turning=true;
   const controller=new AbortController(),relay=():void=>{controller.abort(outer?.reason);};
   if(outer?.aborted===true)relay();else outer?.addEventListener("abort",relay,{once:true});
   const running=Promise.resolve().then(()=>work(controller.signal));
   let stopped=false,handedOff=false,timer:ReturnType<typeof setTimeout>|undefined;
   const release=():void=>{turning=false;outer?.removeEventListener("abort",relay);};
   const finish=():void=>{abandoned--;leaked--;release();startProbe();};
   try{
    return await new Promise<StoreTurnV1<T>>((resolveTurn,rejectTurn)=>{
     const watch=():void=>{
      if(stopped)return;
      startProbe();
      if(probing!==undefined&&options.now()-probing.startedAt>=options.deadlineMs&&options.now()-beatMs>=options.deadlineMs){
       stopped=true;handedOff=true;abandon();leaked++;turning=false;controller.abort(new StoreStalledV1());
       running.then(finish,finish);
       resolveTurn({status:"stalled"});return;
      }
      timer=setTimeout(watch,Math.max(20,Math.min(1000,options.deadlineMs/4)));timer.unref?.();
     };
     timer=setTimeout(watch,Math.max(20,Math.min(1000,options.deadlineMs/4)));timer.unref?.();
     running.then(value=>{if(stopped)return;stopped=true;clearTimeout(timer);resolveTurn({status:"done",value});},error=>{if(stopped)return;stopped=true;clearTimeout(timer);rejectTurn(error);});
    });
   }finally{if(!handedOff)release();}
  },
  async call<T>(work:()=>Promise<T>):Promise<T>{
   if(state!=="ok"||calling||leaked>=leakedCallsMaximumV1)throw new StoreStalledV1();
   calling=true;
   const running=Promise.resolve().then(work).then(value=>({value}),error=>({error})),timer=wait(options.deadlineMs),first=await Promise.race([running,timer.promise]);
   if(first==="late"){abandon();leaked++;calling=false;running.then(()=>{abandoned--;leaked--;startProbe();});throw new StoreStalledV1();}
   timer.cancel();calling=false;
   if("error" in first)throw first.error;
   return first.value;
  }
 };
}

/** 🔁️ The recurrence tracker input: the clock, the window after which a repeated failure is announced again (60 s) and how many keys are remembered (256). */
export type FailuresOptionsV1=Readonly<{now:()=>number;windowMs?:number;capacity?:number}>;

/** 🔁️ Counts failures per key so that a recurring one never goes silent: the first occurrence is announced, a repeat only once per window (with its total count); `clear` forgets the key when the operation worked again, `summary` lists the failure kinds with their lifetime count and last time. */
export type FailuresV1={note(key:string,message:string,incomingWorktree?:boolean,nativeCode?:TargetSharingCodeV1,operation?:TargetSharingOperationV1,target?:RenameEpermTargetV1):Readonly<{announce:boolean;count:number}>;clear(key:string):void;has(key:string):boolean;summary(limit?:number):readonly FailureSummaryV1[];diagnostics(limit?:number):readonly FailureSummaryV1[];size():number;forgetOlderThan(milliseconds:number):number};

const blockedReasonKindsV1=new Set<string>(schema.definitions.BlockedReasons.const),targetSharingKindsV1=schema.definitions.TargetSharingKinds.const,targetSharingOperationKindsV1=schema.definitions.TargetSharingOperationKinds.const,renameEpermTargetKindsV1=schema.definitions.RenameEpermTargetKinds.const;

/** 🏭️ Creates a recurrence tracker. Supplemental blocked details fill spare report slots after totals. Native target sharing codes and operations follow the existing reasons and incoming subset. */
export function createFailuresV1(options:FailuresOptionsV1):FailuresV1{
 const window=options.windowMs??60_000,capacity=options.capacity??256,entries=new Map<string,{count:number;lastMs:number;announcedMs:number}>(),kinds=new Map<string,{count:number;lastMs:number}>(),blocked=new Map<string,{count:number;lastMs:number}>();
 const sharingReason="blocked-sharing-violation";
 const sharingCodes=new Map<string,{count:number;lastMs:number}>(),sharingOperations=new Map<string,{count:number;lastMs:number}>(),renameEpermTargets=new Map<string,{count:number;lastMs:number}>();
 const record=(counts:Map<string,{count:number;lastMs:number}>,kind:string,lastMs:number):void=>{counts.set(kind,{count:(counts.get(kind)?.count??0)+1,lastMs});};
 const incomingSharing={kind:schema.definitions.IncomingWorktreeSharingKind.const,count:0,lastMs:0};
 const rank=(left:FailureSummaryV1,right:FailureSummaryV1):number=>right.lastMs-left.lastMs||(left.kind<right.kind?-1:1);
 const rows=(items:ReadonlyMap<string,{count:number;lastMs:number}>):FailureSummaryV1[]=>[...items].map(([kind,value])=>({kind,...value}));
 const summary=(limit=8):readonly FailureSummaryV1[]=>[...rows(kinds).filter(item=>item.kind!=="loop-blocked"),...blockedFailuresV1()].sort(rank).slice(0,limit);
 const kindOf=(key:string):string=>{const head=key.split(":")[0]!.toLowerCase().replace(/[^a-z0-9-]/gu,"-").replace(/^[^a-z]+/u,"");return (head===""?"failure":head).slice(0,40);};
 return {
  note(key,message,incomingWorktree,nativeCode,operation,target){
   const now=options.now(),category=kindOf(key),entry=entries.get(key)??{count:0,lastMs:now,announcedMs:-Infinity},kind=kinds.get(category)??{count:0,lastMs:now};
   entry.count++;entry.lastMs=now;kind.count++;kind.lastMs=now;
   if(category==="blocked"){
    const candidate=message.length<=32?message.toLowerCase():"other",reason="blocked-"+(blockedReasonKindsV1.has(candidate)?candidate:"other");
    record(blocked,reason,now);
    if(reason===sharingReason){
     if(nativeCode!==undefined){
      record(sharingCodes,targetSharingKindsV1[nativeCode],now);
      if(operation!==undefined)record(sharingOperations,targetSharingOperationKindsV1[operation],now);
      if(nativeCode==="EPERM"&&operation==="rename"&&target!==undefined)record(renameEpermTargets,renameEpermTargetKindsV1[target],now);
     }
     if(incomingWorktree){incomingSharing.count++;incomingSharing.lastMs=now;}
    }
   }
   const announce=entry.count===1||now-entry.announcedMs>=window;
   if(announce)entry.announcedMs=now;
   entries.delete(key);entries.set(key,entry);kinds.set(category,kind);
   if(entries.size>capacity)entries.delete(entries.keys().next().value as string);
   return {announce,count:entry.count};
  },
  clear:key=>{entries.delete(key);},
  has:key=>entries.has(key),
  size:()=>entries.size,
  summary,
  diagnostics(limit=8){
   const totals=summary(limit);
   if(totals.length>=limit||!totals.some(row=>row.kind==="blocked"))return totals;
   const details=[...totals,...rows(blocked).sort(rank).slice(0,limit-totals.length)];
   if(details.length<limit&&incomingSharing.count>0)details.push({...incomingSharing});
   const targetRows=rows(renameEpermTargets).sort((left,right)=>right.count-left.count||rank(left,right));
   return [...details,...[...rows(sharingCodes).sort(rank),...rows(sharingOperations).sort(rank),...targetRows].slice(0,limit-details.length)];
  },
  forgetOlderThan(milliseconds){
   const limit=options.now()-milliseconds;
   let removed=0;
   for(const [key,entry] of entries)if(entry.lastMs<limit){entries.delete(key);removed++;}
   return removed;
  }
 };
}

/** 🧹️ Removes preserved overwritten files that are older than `retainMs` (the directory name starts with the 13-digit millisecond of the hlc that overwrote them) and then the oldest ones beyond `maxEntries` directories; returns how many directories were removed. */
export async function pruneLostV1(directory:string,options:Readonly<{now:number;retainMs:number;maxEntries:number}>):Promise<number>{
 const names=(await readdir(directory).catch(()=>[] as string[])).filter(name=>/^[0-9]{13}-/u.test(name)).sort();
 let removed=0,left=names.length;
 for(const name of names){
  if(options.now-Number(name.slice(0,13))>options.retainMs||left>options.maxEntries){await rm(join(directory,name),{recursive:true,force:true});removed++;left--;}
 }
 return removed;
}

/** 📜️ Rotates a log file that this process does not own exclusively (a service manager keeps it open for appending): when it exceeds `maxBytes` its last `keepBytes` bytes are kept as `<file>.1` and the file is truncated in place, which appending writers survive; returns whether it rotated. */
export async function rotateLogV1(file:string,options:Readonly<{maxBytes:number;keepBytes:number}>):Promise<boolean>{
 const info=await stat(file).catch(()=>undefined);
 if(info===undefined||!info.isFile()||info.size<=options.maxBytes)return false;
 const handle=await open(file,"r");
 let tail:Buffer;
 try{
  const length=Math.min(options.keepBytes,info.size);
  tail=Buffer.alloc(length);
  let done=0;
  while(done<length){const {bytesRead}=await handle.read(tail,done,length-done,info.size-length+done);if(bytesRead===0)break;done+=bytesRead;}
  tail=tail.subarray(0,done);
 }finally{await handle.close();}
 const temporary=`${file}.${tokenOf()}.tmp`;
 await writeFile(temporary,tail);await rename(temporary,`${file}.1`);await truncate(file,0);
 return true;
}

/** 📍️ The path of the health file the daemon keeps for `status`. */
export function healthPathV1(root:string):string{return join(root,".cache","f","health.json");}

/** ✍️ Replaces the health file through tmp + rename; it is advisory, so there is no fsync. */
export async function writeHealthV1(root:string,health:HealthV1):Promise<void>{
 requireResilienceShapeV1("Health",health,"Closed health required");
 const file=healthPathV1(root),temporary=`${file}.${tokenOf()}.tmp`;
 await mkdir(dirname(file),{recursive:true});
 try{await writeFile(temporary,JSON.stringify(health));await rename(temporary,file);}catch(error){await rm(temporary,{force:true});throw error;}
}

/** 👀️ Reads the health file; `undefined` when it is missing, torn or invalid. */
export async function readHealthV1(root:string):Promise<HealthV1|undefined>{
 try{
  const value=JSON.parse(await readFile(healthPathV1(root),"utf8")) as unknown;
  requireResilienceShapeV1("Health",value,"Closed health required");
  return value as HealthV1;
 }catch{return undefined;}
}

/** 🚫️ One path this machine refuses to carry, with the reason and the wall time it was last refused. */
export type RefusedPathV1=Readonly<{path:string;reason:string;wallMs:number}>;

/** 🚫️ The record of refused paths (`.cache/f/refused-paths.json`): `total` counts the refusals of paths that were not among the `recent` ones, `recent` keeps the `REFUSED_PATH_LIMIT_V1` newest distinct ones, newest last. */
export type RefusedPathsV1=Readonly<{version:1;total:number;recent:readonly RefusedPathV1[]}>;

/** 🚫️ How many distinct refused paths the record keeps. */
export const REFUSED_PATH_LIMIT_V1=200;

/** 📍️ The path of the refusal record the daemon keeps for `status`. */
export function refusedPathsPathV1(root:string):string{return join(root,".cache","f","refused-paths.json");}

/** 🚫️ Adds a refusal to the record without changing it in place: a path already in `recent` moves to the end with its new reason and time, a path not in `recent` counts into `total`, and the oldest entries beyond `REFUSED_PATH_LIMIT_V1` drop out. Paths longer than the schema allows are cut. */
export function noteRefusedPathV1(record:RefusedPathsV1,path:string,reason:string,wallMs:number):RefusedPathsV1{
 const kept=record.recent.filter(item=>item.path!==path),fresh=kept.length===record.recent.length,entry:RefusedPathV1={path:path.slice(0,4096),reason:reason.slice(0,200),wallMs};
 return {version:1,total:record.total+(fresh?1:0),recent:[...kept,entry].slice(-REFUSED_PATH_LIMIT_V1)};
}

/** 🚫️ The empty refusal record. */
export const NO_REFUSED_PATHS_V1:RefusedPathsV1={version:1,total:0,recent:[]};

/** ✍️ Replaces the refusal record through tmp + rename; it is advisory, so there is no fsync. */
export async function writeRefusedPathsV1(root:string,record:RefusedPathsV1):Promise<void>{
 requireResilienceShapeV1("RefusedPaths",record,"Closed refused paths required");
 const file=refusedPathsPathV1(root),temporary=`${file}.${tokenOf()}.tmp`;
 await mkdir(dirname(file),{recursive:true});
 try{await writeFile(temporary,JSON.stringify(record));await rename(temporary,file);}catch(error){await rm(temporary,{force:true});throw error;}
}

/** 👀️ Reads the refusal record; `undefined` when it is missing, torn or invalid. */
export async function readRefusedPathsV1(root:string):Promise<RefusedPathsV1|undefined>{
 try{
  const value=JSON.parse(await readFile(refusedPathsPathV1(root),"utf8")) as unknown;
  requireResilienceShapeV1("RefusedPaths",value,"Closed refused paths required");
  return value as RefusedPathsV1;
 }catch{return undefined;}
}

/** 🕰️ The code this machine has run: digest and the time it was first run successfully, and the digests it refused. Kept in `.cache/f/code/history.json` (at most 64 runs, 64 refusals). */
export type RunHistoryV1=Readonly<{version:1;runs:readonly Readonly<{digest:string;firstMs:number}>[];refused:readonly string[]}>;

const historyPath=(root:string):string=>join(codeDirectoryV1(root),"history.json");

/** 👀️ Reads the run history; empty when there is none or it is torn. */
export async function readRunHistoryV1(root:string):Promise<RunHistoryV1>{
 try{
  const value=JSON.parse(await readFile(historyPath(root),"utf8")) as unknown;
  requireResilienceShapeV1("History",value,"Closed run history required");
  return value as RunHistoryV1;
 }catch{return {version:1,runs:[],refused:[]};}
}

async function writeRunHistoryV1(root:string,history:RunHistoryV1):Promise<void>{
 const file=historyPath(root),temporary=`${file}.${tokenOf()}.tmp`;
 await mkdir(dirname(file),{recursive:true});
 try{await writeFile(temporary,JSON.stringify(history));await rename(temporary,file);}catch(error){await rm(temporary,{force:true});throw error;}
}

/** ✍️ Records that this machine ran `digest` successfully; the first time is kept. Returns the new history. */
export async function recordRunV1(root:string,digest:string,now:number=Date.now()):Promise<RunHistoryV1>{
 const history=await readRunHistoryV1(root);
 if(history.runs.some(run=>run.digest===digest))return history;
 const next:RunHistoryV1={version:1,runs:[...history.runs,{digest,firstMs:now}].slice(-64),refused:history.refused};
 await writeRunHistoryV1(root,next);
 return next;
}

/** ↩️ Removes a digest from the refused list of this machine (a refusal that the machine itself has disproved by running the code). Returns the new history. */
export async function withdrawRefusedV1(root:string,digest:string):Promise<RunHistoryV1>{
 const history=await readRunHistoryV1(root);
 if(!history.refused.includes(digest))return history;
 const next:RunHistoryV1={version:1,runs:history.runs,refused:history.refused.filter(item=>item!==digest)};
 await writeRunHistoryV1(root,next);
 return next;
}

/** 🚫️ Records a digest the self-check rejected. */
export async function recordRefusedV1(root:string,digest:string):Promise<RunHistoryV1>{
 const history=await readRunHistoryV1(root);
 if(history.refused.includes(digest))return history;
 const next:RunHistoryV1={version:1,runs:history.runs,refused:[...history.refused,digest].slice(-64)};
 await writeRunHistoryV1(root,next);
 return next;
}

/** 🔙️ Why code on disk must not be adopted automatically, or `undefined`: its digest was refused before, or it was run before and has been superseded by the code that runs now (a revert, for instance by an older copy of the files arriving from a machine that republishes its stale tree). */
export function suspectCodeV1(history:RunHistoryV1,running:string,digest:string):string|undefined{
 if(history.refused.includes(digest))return "refused before";
 const was=history.runs.find(run=>run.digest===digest),now=history.runs.find(run=>run.digest===running);
 if(was!==undefined&&digest!==running&&(now===undefined||was.firstMs<now.firstMs))return "ran before and was superseded (revert)";
 return undefined;
}

/** 🚦️ The start-time verdict on the code of a worktree: the reason to prefer a sealed snapshot over it, or `null`. The digest is computed by the caller. */
export async function startRejectionV1(root:string,digest:string):Promise<string|null>{
 const history=await readRunHistoryV1(root),newest=history.runs.at(-1);
 return suspectCodeV1(history,newest?.digest??"",digest)??null;
}

/** 💓️ What the watchdog thread of a running daemon writes every second to `.cache/f/boot/beat.json`, from a thread that a blocked event loop cannot stop: the process, the boot stage that supervises it, the phase (`boot` while the code loads, `open` while the state opens, `run` once the daemon started), the code it runs and where that came from (`worktree`, `snapshot` or `store`), how long the event loop has not answered (`lagMs`) and what the main thread last declared it was doing (`activity`, see `activityV1`). */
export type BeatV1=Readonly<{version:1;pid:number;startMs:number;parent:number;startedMs:number;wallMs:number;phase:"boot"|"open"|"run";base:string;source:"worktree"|"snapshot"|"store";lagMs:number;activity?:string;longestMs?:number;longestActivity?:string}>;

/** 🪂️ The guard of a machine that runs a fallback code instead of the code of its worktree: where the code came from and why the worktree code was not used. */
export type FallbackV1=Readonly<{guard:"code-fallback";source:"snapshot"|"store";reason:string}>;

/** 📍️ The path of the beat file. */
export function beatPathV1(root:string):string{return join(root,".cache","f","boot","beat.json");}

const staleBeatMilliseconds=20_000;

/** 👀️ Reads the beat of a daemon that lives: the file is valid, its process exists and its last write is at most 20 s old. A daemon whose event loop is blocked still has such a beat, which is how `status` and the lock tell a busy daemon from a dead one. */
export async function readBeatV1(root:string,now:number=Date.now()):Promise<BeatV1|undefined>{
 try{
  const value=JSON.parse(await readFile(beatPathV1(root),"utf8")) as unknown;
  requireResilienceShapeV1("Beat",value,"Closed beat required");
  const beat=value as BeatV1;
  return processHolderAliveV1(beat)&&now-beat.wallMs<=staleBeatMilliseconds?beat:undefined;
 }catch{return undefined;}
}

/** 🪂️ Reads the fallback guard that the boot stage hands to the daemon in `SEMIO_FLEET_CODE_FALLBACK`; `undefined` when the daemon runs the code of its worktree or the value is not a closed guard. */
export function readFallbackV1(environment:Readonly<Record<string,string|undefined>>):FallbackV1|undefined{
 const raw=environment.SEMIO_FLEET_CODE_FALLBACK;
 if(raw===undefined||raw==="")return undefined;
 try{const value=JSON.parse(raw) as unknown;requireResilienceShapeV1("Fallback",value,"Closed fallback required");return value as FallbackV1;}catch{return undefined;}
}

/** 🏷️ What the main thread of the daemon can declare it is doing, in the order of the shared slot: the watchdog thread copies the last declaration into every beat, so the boot stage knows where a blocked loop sat without a stack. */
export const activityNamesV1=["idle","open","follow","publish","walk","hot","sweep","file","attest","sidecars","health","tend","maintain","compact","git","drift","hash","journal","persist","settle"] as const;

/** 🏷️ The declarable activities. */
export type ActivityV1=typeof activityNamesV1[number];

const activitySymbol=Symbol.for("semio.fleet.activity"),restartSymbol=Symbol.for("semio.fleet.blockedRestart");

/** 🏷️ Declares what the main thread is about to do (one store into the shared slot of the watchdog; a no-op where no watchdog runs, as in tests and one-shot commands). */
export function activityV1(name:ActivityV1):void{
 (globalThis as unknown as Record<symbol,((name:ActivityV1)=>void)|undefined>)[activitySymbol]?.(name);
}

/** 🧱️ The blocks of the event loop that this process recorded (`kind` `loop-blocked`, how often and when last: the boot stage counts every wake-up that came `blockMs` or more too late) and the ends that the boot stage made of earlier daemons of this machine (`kind` `loop-blocked-restart`, how often and when last, and `loop-blocked-in-<activity>` for the activity the last blocked daemon sat in, so the fleet report names what blocks a machine without reading its disk). Empty when none happened. */
export function blockedFailuresV1():readonly FailureSummaryV1[]{
 const holder=globalThis as unknown as Record<symbol,{count:number;lastMs:number;activity?:string}|undefined>,found=holder[Symbol.for("semio.fleet.blocked")],ended=holder[restartSymbol];
 return [...found===undefined||found.count===0?[]:[{kind:"loop-blocked",count:found.count,lastMs:found.lastMs}],...ended===undefined||ended.count===0?[]:[{kind:"loop-blocked-restart",count:ended.count,lastMs:ended.lastMs},...ended.activity===undefined?[]:[{kind:"loop-blocked-in-"+ended.activity,count:ended.count,lastMs:ended.lastMs}]]];
}

/** 📍️ The path of the record that the boot stage leaves when it ends a blocked daemon. */
export function blockedRestartPathV1(root:string):string{return join(root,".cache","f","boot","loop-blocked-restart.json");}

/** 📖️ Reads the record of the last end of a blocked daemon; `undefined` when there is none or it is not closed. */
export async function readBlockedRestartV1(root:string):Promise<BlockedRestartV1|undefined>{
 try{
  const value=JSON.parse(await readFile(blockedRestartPathV1(root),"utf8")) as unknown;
  requireResilienceShapeV1("BlockedRestart",value,"Closed blocked-restart record required");
  return value as BlockedRestartV1;
 }catch{return undefined;}
}

/** 🪦️ What the boot stage records before it ends a daemon whose event loop stayed blocked (or whose watchdog fell silent) in the run phase: the running count on this machine, the process, the lag, what the main thread declared it was doing, the longest block the watchdog measured (`longestMs` and the activity it sat in), and the stack sample taken from outside (`sample`, a file name below `.cache/f/boot`, null where the platform has no sampler). */
export type BlockedRestartV1=Readonly<{version:1;count:number;pid:number;wallMs:number;lagMs:number;phase:"boot"|"open"|"run";reason:"blocked"|"silent";activity:string|null;sample:string|null;longestMs?:number;longestActivity?:string|null}>;

/** 🗄️ The directory of the code snapshots below a store root: `<store>/v1/bootstrap/code/<digest>/{sealed.json,files/}`, the layout of a local sealed snapshot, so one verification reads both. */
export function storeCodeDirectoryV1(store:string):string{return join(store,"v1","bootstrap","code");}

/** 📚️ Lists the valid code snapshots of a store, newest first. */
export async function listStoreCodeV1(store:string):Promise<CodeSnapshotV1[]>{
 const base=storeCodeDirectoryV1(store),found:CodeSnapshotV1[]=[];
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!/^[0-9a-f]{64}$/u.test(name))continue;
  try{
   const seal=JSON.parse(await readFile(join(base,name,"sealed.json"),"utf8")) as unknown;
   requireResilienceShapeV1("SealedCode",seal,"Closed sealed code required");
   if((seal as SealedCodeV1).digest===name)found.push({digest:name,createdMs:(seal as SealedCodeV1).createdMs,directory:join(base,name)});
  }catch{}
 }
 return found.sort((left,right)=>right.createdMs-left.createdMs||(left.digest<right.digest?-1:1));
}

/** ⏲️ Maximum interval between retries of an otherwise eligible snapshot publication. */
export const CODE_SNAPSHOT_PUBLICATION_RETRY_MAXIMUM_V1=schema.definitions.CodeSnapshotPublicationPolicy.const.retryMaximumMilliseconds;

/** 📤️ Publishes the sealed local snapshot of `digest` to the store so that a machine whose worktree code and local snapshots are all unusable can fetch working code: the files are re-verified against their seal, copied into a private `.tmp-` directory next to the target with `sealed.json` written last, and renamed into `<store>/v1/bootstrap/code/<digest>`, so a snapshot in the store either is complete or does not exist. A present intact snapshot is left alone, a damaged one is replaced. */
export async function publishCodeSnapshotV1(options:Readonly<{root:string;store:string;digest:string;originId:string}>):Promise<Readonly<{published:boolean;reason:string|null}>>{
 const local=join(codeDirectoryV1(options.root),options.digest),base=storeCodeDirectoryV1(options.store),target=join(base,options.digest);
 const broken=await verifyCodeSnapshotV1(local);
 if(broken!==null)return {published:false,reason:`local snapshot: ${broken}`};
 if(await verifyCodeSnapshotV1(target)===null)return {published:false,reason:"present"};
 const temporary=join(base,`.tmp-${options.originId}-${tokenOf()}`),seal=JSON.parse(await readFile(join(local,"sealed.json"),"utf8")) as SealedCodeV1;
 try{
  for(const file of seal.files){
   const destination=join(temporary,"files",...file.path.split("/"));
   await mkdir(dirname(destination),{recursive:true});
   await writeFile(destination,await readFile(join(local,"files",...file.path.split("/"))),{mode:file.executable?0o755:0o644});
   if(file.executable)await chmod(destination,0o755);
  }
  const handle=await open(join(temporary,"sealed.json"),"w");
  try{await handle.writeFile(JSON.stringify(seal));await handle.sync();}finally{await handle.close();}
  const aside=join(base,`.tmp-${options.originId}-${tokenOf()}-old`);
  await rename(target,aside).catch(()=>undefined);
  try{await rename(temporary,target);}catch(error){await rm(aside,{recursive:true,force:true}).catch(()=>{});if(await verifyCodeSnapshotV1(target)!==null)throw error;await rm(temporary,{recursive:true,force:true});return {published:false,reason:"present"};}
  await rm(aside,{recursive:true,force:true}).catch(()=>{});
  return {published:true,reason:null};
 }catch(error){await rm(temporary,{recursive:true,force:true}).catch(()=>{});return {published:false,reason:(error instanceof Error?error.message:String(error)).slice(0,256)};}
}

/** ✂️ Keeps the newest `keep` code snapshots of a store and every digest in `protect` (the digests that a live origin attests), removes the others and every unfinished `.tmp-` directory older than an hour; returns the removed digests. */
export async function pruneStoreCodeV1(store:string,keep:number,protect:ReadonlySet<string>=new Set(),now:number=Date.now()):Promise<string[]>{
 const base=storeCodeDirectoryV1(store),removed:string[]=[];
 for(const snapshot of (await listStoreCodeV1(store)).slice(Math.max(0,keep))){
  if(protect.has(snapshot.digest))continue;
  await rm(snapshot.directory,{recursive:true,force:true});removed.push(snapshot.digest);
 }
 for(const name of await readdir(base).catch(()=>[] as string[])){
  if(!name.startsWith(".tmp-")&&!name.startsWith("._"))continue;
  const info=await stat(join(base,name)).catch(()=>undefined);
  if(info!==undefined&&now-info.mtimeMs>3_600_000)await rm(join(base,name),{recursive:true,force:true});
 }
 return removed;
}

/** 🪞️ Copies the boot stage (`📜️script.ts` of the module) to `.cache/f/boot/stage.ts` with `stage.json` (its sha256), both through tmp + rename and the signature last, so the worktree entry, which prefers a copy whose hash matches the signature, never runs a torn copy. Called by `join` and `seed` (provisioning) and by a healthy daemon, so a machine that joined before the stage existed gets it from the next healthy daemon. Returns whether the copy changed. */
export async function installBootStageV1(root:string,script:string):Promise<Readonly<{installed:boolean}>>{
 const data=await readFile(script),digest=sha(data),directory=join(root,".cache","f","boot"),copy=join(directory,"stage.ts"),signature=join(directory,"stage.json");
 const known=await readFile(signature,"utf8").then(text=>(JSON.parse(text) as {sha256?:string}).sha256,()=>undefined),present=await readFile(copy).then(bytes=>sha(bytes),()=>undefined);
 if(known===digest&&present===digest)return {installed:false};
 await mkdir(directory,{recursive:true});
 const first=`${copy}.${tokenOf()}.tmp`,second=`${signature}.${tokenOf()}.tmp`;
 try{
  await writeFile(first,data);await rm(signature,{force:true});await rename(first,copy);
  await writeFile(second,JSON.stringify({version:1,sha256:digest,bytes:data.length,wallMs:Date.now()}));await rename(second,signature);
 }catch(error){await rm(first,{force:true});await rm(second,{force:true});throw error;}
 return {installed:true};
}

/** 🔁️ Re-exported from its own module so that a consumer of the yield alone does not load this replica self-check module. @see ./⏳️yielding/🟦️.ts */
export {yieldingV1} from "./⏳️yielding/🟦️.ts";

/** 🧱️ Re-exported from their own module so that a consumer of these alone does not load this replica self-check module. @see ./🧱️attestation-support/🟦️.ts */
export {importClosureV1,moduleSourcesV1,restartJitterMillisecondsV1,StoreStalledV1,type FailureSummaryV1} from "./🧱️attestation-support/🟦️.ts";
