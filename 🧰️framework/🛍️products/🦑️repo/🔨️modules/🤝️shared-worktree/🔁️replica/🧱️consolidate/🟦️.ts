import {spawn} from "node:child_process";
import {randomBytes} from "node:crypto";
import {chmod,mkdir,readdir,readFile,rename,rm,stat,statfs,unlink,writeFile} from "node:fs/promises";
import {join} from "node:path";
import {GIT_PINS_V1,resolveGitV1,sweepGitGarbageV1} from "../../🌿️git/🟦️.ts";
import {planPacksV1,readPackNamesV1} from "../🥾️bootstrap/🧺️packs/🟦️.ts";
import {authorisePacksV1} from "../🧷️packgate/🟦️.ts";
import {foldStoreV1,type ReplicaSinkV1} from "../🟦️.ts";

/** 🧱️ What one consolidation run did. Schema: `./🧬️schema/🔣️.json#/definitions/Consolidation`. */
export type ConsolidationV1=Readonly<{version:1;outcome:"nothing"|"planned"|"consolidated"|"locked"|"space"|"unpublished"|"failed";packsBefore:number;bytesBefore:number;packsAfter?:number;bytesAfter?:number;retired:number;bytesFreed:number;pack?:string;objects?:number}>;

/** 🎚️ The inputs of one run. `apply` false only plans. `storeRoot` makes the run wait until the replica has published the new pack to the store (`published` replaces that check). `faults` stops the run like a crash at one point (`pack-written`: new pack complete in a staging name; `placed`: new pack in `.git/objects/pack`; `published`: store has it; `retire:<n>`: after n retired stems), for tests. */
export type ConsolidationOptionsV1=Readonly<{worktree:string;apply:boolean;storeRoot?:string;signal?:AbortSignal;sink?:ReplicaSinkV1;now?:()=>number;waitMilliseconds?:number;pollMilliseconds?:number;minOrphanAgeMilliseconds?:number;verify?:boolean;needBytes?:number;reserveBytes?:number;space?:(directory:string)=>Promise<Readonly<{free:number;total:number}>|undefined>;published?:(path:string,bytes:number)=>Promise<boolean>;faults?:string}>;

/** 💥️ Thrown by an injected fault; the run leaves everything as a killed process would. */
export class ConsolidationFaultV1 extends Error{constructor(readonly point:string){super(`Injected fault at ${point}`);this.name="ConsolidationFaultV1";}}

const GIB=2**30,lockStaleMilliseconds=6*3_600_000,companionExtensions=["idx","rev","bitmap","mtimes","keep","promisor"] as const,token=():string=>randomBytes(6).toString("hex");
const later=(milliseconds:number):Promise<void>=>new Promise(done=>setTimeout(done,milliseconds));

async function sizeOf(directory:string):Promise<Readonly<{files:number;bytes:number}>>{
 let files=0,bytes=0;
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  const info=await stat(join(directory,name)).catch(()=>undefined);
  if(info?.isFile()===true){files++;bytes+=info.size;}
 }
 return {files,bytes};
}

async function packsOf(directory:string):Promise<number>{return (await readdir(directory).catch(()=>[] as string[])).filter(name=>name.endsWith(".idx")).length;}

async function remove(file:string):Promise<void>{
 try{await unlink(file);}
 catch(error){
  const code=(error as {code?:string}).code;
  if(code==="ENOENT")return;
  await chmod(file,0o666).catch(()=>{});
  await rm(file,{force:true});
 }
}

async function volume(directory:string):Promise<Readonly<{free:number;total:number}>|undefined>{
 try{const info=await statfs(directory);return {free:Number(info.bavail)*Number(info.bsize),total:Number(info.blocks)*Number(info.bsize)};}catch{return undefined;}
}

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as {code?:string}).code==="EPERM";}};

async function takeLock(path:string,now:number):Promise<boolean>{
 try{await mkdir(path);await writeFile(join(path,"pid"),String(process.pid));return true;}
 catch(error){
  if((error as {code?:string}).code!=="EEXIST")throw error;
  const info=await stat(path).catch(()=>undefined),holder=Number(await readFile(join(path,"pid"),"utf8").catch(()=>"0"));
  if(info!==undefined&&(now-info.mtimeMs>lockStaleMilliseconds||holder>0&&!alive(holder))){await rm(path,{recursive:true,force:true});return takeLock(path,now);}
  return false;
 }
}

function writePack(executable:string,worktree:string,stage:string,signal:AbortSignal|undefined):Promise<string|undefined>{
 return new Promise(done=>{
  const environment={...process.env,GIT_OPTIONAL_LOCKS:"0"},pins=GIT_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]);
  const list=spawn(executable,["-C",worktree,...pins,"cat-file","--batch-all-objects","--batch-check=%(objectname)"],{stdio:["ignore","pipe","ignore"],env:environment,...signal===undefined?{}:{signal}});
  const pack=spawn(executable,["-C",worktree,...pins,"pack-objects","--delta-base-offset","--window=0","--threads=2","--quiet",stage],{stdio:["pipe","pipe","ignore"],env:environment,...signal===undefined?{}:{signal}});
  let out="",listed=0;
  pack.stdout.on("data",chunk=>{out+=String(chunk);});
  pack.stdin.on("error",()=>{});
  list.stdout.on("data",chunk=>{listed+=1;pack.stdin.write(chunk);});
  list.on("error",()=>{pack.stdin.end();});
  list.on("close",code=>{if(code!==0)pack.kill();pack.stdin.end();});
  pack.on("error",()=>done(undefined));
  pack.on("close",code=>done(code===0&&listed>0&&/^[0-9a-f]{40}$/u.test(out.trim())?out.trim():undefined));
 });
}

async function discard(directory:string,prefix:string):Promise<void>{
 for(const name of await readdir(directory).catch(()=>[] as string[]))if(name.startsWith(prefix))await remove(join(directory,name));
}

/** 🧱️ Consolidates the packs of one git worktree into one: lists every object (`cat-file --batch-all-objects`, loose and packed, reachable or not, so nothing needs an age rule), writes them as one new pack beside the others under a `tmp_consolidate_` staging name (a crash leaves only a swept temporary file), checks it with `git verify-pack`, moves `.pack`, `.rev`, then `.idx` into `.git/objects/pack` (git's own order: a pack exists for readers when its index does), waits until the replica has published it to the store, then compares the index files again with the new pack first and retires only the packs whose every object is in it (and orphans older than an hour): `multi-pack-index`, then per stem `.idx` first, then the rest. Receivers apply deletes only after all creations and only when every fleet index has its pack (`packsComplete`), so the objects are present on every machine at every moment. Interrupted at any point a re-run converges: a placed pack is just another pack, a half-retired stem is an orphan, a staging file is removed at the start. One run at a time per worktree (`.git/semio-consolidate.lock`). Cancellation kills git and leaves nothing but the staging file's removal. */
export async function consolidateGitV1(options:ConsolidationOptionsV1):Promise<ConsolidationV1>{
 const sink=options.sink??(()=>{}),clock=options.now??Date.now,directory=join(options.worktree,".git","objects","pack"),signal=options.signal;
 const before=await sizeOf(directory),packsBefore=await packsOf(directory),base={version:1 as const,packsBefore,bytesBefore:before.bytes};
 const plan=await planPacksV1(directory,signal);
 if(plan.keep.length<=1&&plan.covered.length===0&&plan.orphans.length===0&&plan.garbage.length===0)return {...base,outcome:"nothing",retired:0,bytesFreed:0};
 if(!options.apply)return {...base,outcome:"planned",retired:plan.covered.length+plan.orphans.length,bytesFreed:plan.bytesDropped};
 const executable=await resolveGitV1();
 if(executable===null)return {...base,outcome:"failed",retired:0,bytesFreed:0};
 const lock=join(options.worktree,".git","semio-consolidate.lock");
 if(!await takeLock(lock,clock()))return {...base,outcome:"locked",retired:0,bytesFreed:0};
 try{
  await discard(directory,"tmp_consolidate_");
  await sweepGitGarbageV1(options.worktree,{now:clock()});
  const largest=Math.max(0,...await Promise.all(plan.keep.map(async stem=>(await stat(join(directory,`${stem}.pack`)).catch(()=>undefined))?.size??0)));
  const needed=options.needBytes??Math.max(2*GIB,Math.ceil(largest*1.5)),space=await (options.space??volume)(directory);
  if(space!==undefined&&space.free-needed<(options.reserveBytes??Math.ceil(space.total*0.03)+2*GIB)){sink({version:1,event:"skipped",stage:"consolidate",reason:"space",bytes:needed});return {...base,outcome:"space",retired:0,bytesFreed:0};}
  const stage=join(directory,`tmp_consolidate_${token()}`);
  sink({version:1,event:"progress",stage:"consolidate",done:0,total:4});
  const hash=await writePack(executable,options.worktree,stage,signal);
  signal?.throwIfAborted();
  if(hash===undefined){await discard(directory,"tmp_consolidate_");return {...base,outcome:"failed",retired:0,bytesFreed:0};}
  const staged=`${stage}-${hash}`,stem=`pack-${hash}`;
  if(options.verify!==false){
   const verified=await new Promise<boolean>(done=>{const child=spawn(executable,["-C",options.worktree,"verify-pack",`${staged}.idx`],{stdio:"ignore",...signal===undefined?{}:{signal}});child.on("error",()=>done(false));child.on("close",code=>done(code===0));});
   if(!verified){await discard(directory,"tmp_consolidate_");return {...base,outcome:"failed",retired:0,bytesFreed:0};}
  }
  if(options.faults==="pack-written")throw new ConsolidationFaultV1("pack-written");
  sink({version:1,event:"progress",stage:"consolidate",done:1,total:4});
  await authorisePacksV1(options.worktree,{publish:[stem]},clock());
  await writeFile(join(directory,`${stem}.keep`),"semio-consolidated\n");
  for(const extension of ["pack","rev","idx"])await rename(`${staged}.${extension}`,join(directory,`${stem}.${extension}`)).catch(error=>{if((error as {code?:string}).code!=="ENOENT"||extension!=="rev")throw error;});
  if(options.faults==="placed")throw new ConsolidationFaultV1("placed");
  const packBytes=(await stat(join(directory,`${stem}.pack`))).size,idxBytes=(await stat(join(directory,`${stem}.idx`))).size,objects=(await readPackNamesV1(join(directory,`${stem}.idx`)))?.count??0;
  const wait=options.waitMilliseconds??30*60_000,poll=options.pollMilliseconds??15_000,start=clock();
  const publishedCheck=options.published??(options.storeRoot===undefined?undefined:async(path:string,bytes:number):Promise<boolean>=>{const folded=await foldStoreV1(options.storeRoot!,signal);const entry=folded.manifest.get(path);return entry?.operation==="put"&&entry.bytes===bytes;});
  let published=publishedCheck===undefined;
  while(!published){
   signal?.throwIfAborted();
   published=await publishedCheck!(`.git/objects/pack/${stem}.pack`,packBytes)&&await publishedCheck!(`.git/objects/pack/${stem}.idx`,idxBytes);
   if(!published){if(clock()-start>=wait)return {...base,outcome:"unpublished",retired:0,bytesFreed:0,pack:stem,objects};await later(poll);}
  }
  sink({version:1,event:"progress",stage:"consolidate",done:2,total:4});
  if(options.faults==="published")throw new ConsolidationFaultV1("published");
  const now=clock(),after=await planPacksV1(directory,signal,[stem]),minAge=options.minOrphanAgeMilliseconds??3_600_000,old:string[]=[];
  for(const orphan of after.orphans){
   const info=await stat(join(directory,`${orphan}.pack`)).catch(()=>stat(join(directory,`${orphan}.idx`)).catch(()=>undefined));
   if(info!==undefined&&now-info.mtimeMs>=minAge)old.push(orphan);
  }
  const retire=[...after.covered.filter(name=>name!==stem),...old];
  await authorisePacksV1(options.worktree,{retire:[...retire,"multi-pack-index"]},clock());
  for(const name of await readdir(directory).catch(()=>[] as string[]))if(name==="multi-pack-index"||name.startsWith("multi-pack-index-"))await remove(join(directory,name));
  let retired=0;
  for(const name of retire){
   for(const extension of companionExtensions)await remove(join(directory,`${name}.${extension}`));
   await remove(join(directory,`${name}.pack`));
   retired++;
   sink({version:1,event:"progress",stage:"consolidate",done:retired,total:retire.length,count:retired});
   if(options.faults===`retire:${retired}`)throw new ConsolidationFaultV1(`retire:${retired}`);
  }
  const final=await sizeOf(directory);
  sink({version:1,event:"progress",stage:"consolidate",done:4,total:4,bytes:Math.max(0,before.bytes-final.bytes)});
  return {...base,outcome:"consolidated",packsAfter:await packsOf(directory),bytesAfter:final.bytes,retired,bytesFreed:Math.max(0,before.bytes-final.bytes),pack:stem,objects};
 }catch(error){
  if(!(error instanceof ConsolidationFaultV1)&&signal?.aborted===true)await discard(directory,"tmp_consolidate_");
  throw error;
 }finally{await rm(lock,{recursive:true,force:true});}
}

/** 🔄️ Runs `consolidateGitV1` now and then every `minutes` (the group server's relay), only when the plan shows waste of at least `minimumWasteBytes` (default 1 GiB) or garbage; reports each outcome as a `progress` event of stage `consolidate`; a failure is an `error` event and the loop goes on. */
export async function refreshConsolidationV1(options:Omit<ConsolidationOptionsV1,"apply">&Readonly<{minutes:number;minimumWasteBytes?:number}>,signal:AbortSignal,sink:ReplicaSinkV1=()=>{}):Promise<void>{
 while(!signal.aborted){
  try{
   const plan=await planPacksV1(join(options.worktree,".git","objects","pack"),signal);
   if(plan.bytesDropped>=(options.minimumWasteBytes??GIB)||plan.garbage.some(name=>name.startsWith("tmp_"))){
    const result=await consolidateGitV1({...options,apply:true,signal,sink});
    sink({version:1,event:"progress",stage:"consolidate",reason:result.outcome,count:result.retired,bytes:result.bytesFreed});
   }
  }catch(error){if(signal.aborted)break;sink({version:1,event:"error",stage:"consolidate",message:error instanceof Error?error.message:String(error)});}
  await new Promise<void>(done=>{
   const finish=()=>{clearTimeout(timer);signal.removeEventListener("abort",finish);done();};
   const timer=setTimeout(finish,options.minutes*60_000);signal.addEventListener("abort",finish,{once:true});
  });
 }
}
