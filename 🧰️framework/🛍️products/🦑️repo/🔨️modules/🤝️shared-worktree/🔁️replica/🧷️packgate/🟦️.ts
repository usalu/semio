import {spawn} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {chmod,lstat,mkdir,open,opendir,readdir,rename,rm,stat,unlink,writeFile} from "node:fs/promises";
import {createWriteStream,readFileSync,statSync} from "node:fs";
import {join} from "node:path";
import {devNull} from "node:os";
import {createDeflate,deflateSync} from "node:zlib";
import {pipeline} from "node:stream/promises";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {GIT_PINS_V1,resolveGitV1} from "../../🌿️git/🟦️.ts";
import {readPackNamesV1} from "../🥾️bootstrap/🧺️packs/🟦️.ts";

/** 🧷️ The directory of packs below the worktree root, with the trailing slash. */
export const packDirectoryV1=".git/objects/pack/";

/** 🔏️ The decoded `.git/semio-packs.json`: stems the replica may publish and stems (or `multi-pack-index`) whose deletion it may publish. Schema: `./🧬️schema/🔣️.json#/definitions/PackAuthorisation`. */
export type PackAuthorisationV1=Readonly<{publish:ReadonlySet<string>;retire:ReadonlySet<string>}>;

/** 🧹️ What one rescue pass did. Schema: `./🧬️schema/🔣️.json#/definitions/Rescue`. */
export type RescueV1=Readonly<{examined:number;objects:number;retired:number;kept:number}>;

const extensions=new Set(["pack","idx","rev","mtimes","bitmap","keep","promisor"]),authorisationFile=(worktree:string):string=>join(worktree,".git","semio-packs.json"),twelveHours=12*3_600_000;
const none:PackAuthorisationV1={publish:new Set(),retire:new Set()};
const cache=new Map<string,Readonly<{stamp:string;value:PackAuthorisationV1}>>();

/** 🔬️ Splits a worktree-relative path into the stem and extension of a pack file of `.git/objects/pack`; `multi-pack-index` and its derived files have the stem `multi-pack-index`; `undefined` for other paths and unknown names. */
export function packPartsV1(path:string):Readonly<{stem:string;extension:string}>|undefined{
 if(!path.startsWith(packDirectoryV1))return undefined;
 const name=path.slice(packDirectoryV1.length);
 if(name===""||name.includes("/"))return undefined;
 if(name==="multi-pack-index"||name.startsWith("multi-pack-index-"))return {stem:"multi-pack-index",extension:name.slice(name.lastIndexOf(".")+1)};
 const dot=name.lastIndexOf(".");
 if(dot<=0)return undefined;
 const extension=name.slice(dot+1);
 return extensions.has(extension)&&!name.startsWith("tmp_")&&!name.startsWith(".")?{stem:name.slice(0,dot),extension}:undefined;
}

/** 🔏️ Reads the authorisation of a worktree (cached by size and modification time, so the synchronous ignore check stays cheap); absent or unreadable means nothing is authorised. */
export function readPackAuthorisationV1(worktree:string):PackAuthorisationV1{
 const file=authorisationFile(worktree);
 try{
  const info=statSync(file),stamp=`${info.size}:${info.mtimeMs}`,held=cache.get(file);
  if(held?.stamp===stamp)return held.value;
  const raw=JSON.parse(readFileSync(file,"utf8")) as {version?:unknown;publish?:Record<string,number>;retire?:Record<string,number>};
  const value=raw.version===1?{publish:new Set(Object.keys(raw.publish??{})),retire:new Set(Object.keys(raw.retire??{}))}:none;
  cache.set(file,{stamp,value});
  return value;
 }catch{return none;}
}

/** ✍️ Authorises stems for publishing and/or retiring (atomic write, entries older than 12 hours dropped). Called by the consolidator before it places a pack and before it retires packs. */
export async function authorisePacksV1(worktree:string,grant:Readonly<{publish?:readonly string[];retire?:readonly string[]}>,now:number=Date.now()):Promise<void>{
 const file=authorisationFile(worktree);
 let raw:{publish:Record<string,number>;retire:Record<string,number>}={publish:{},retire:{}};
 try{const parsed=JSON.parse(readFileSync(file,"utf8")) as typeof raw;raw={publish:parsed.publish??{},retire:parsed.retire??{}};}catch{}
 for(const table of [raw.publish,raw.retire])for(const [stem,at] of Object.entries(table))if(now-at>twelveHours)delete table[stem];
 for(const stem of grant.publish??[])raw.publish[stem]=now;
 for(const stem of grant.retire??[])raw.retire[stem]=now;
 const temporary=`${file}.${process.pid}.tmp`;
 await writeFile(temporary,JSON.stringify({version:1,...raw}));
 await rename(temporary,file);
}

/** 🚫️ Tells whether a path of the pack directory or of `.git/objects/info` (the dumb-transport pack list and the commit graphs, derived caches every git client rewrites) stays on this machine: unknown names, every `multi-pack-index`, and every pack whose `.pack` the fleet does not hold (`known`) unless the consolidator authorised its stem, or the fleet holds no pack at all (`fleetHasPacks` false: the first machine seeds its packs). Other paths are not decided here and answer false. */
export function packLocalV1(path:string,known:(packPath:string)=>boolean,fleetHasPacks:()=>boolean,worktree:string):boolean{
 if(path.startsWith(".git/objects/info/"))return true;
 if(!path.startsWith(packDirectoryV1))return false;
 const parts=packPartsV1(path);
 if(parts===undefined)return true;
 if(parts.stem==="multi-pack-index")return true;
 if(known(`${packDirectoryV1}${parts.stem}.pack`))return false;
 if(readPackAuthorisationV1(worktree).publish.has(parts.stem))return false;
 return fleetHasPacks();
}

async function remove(file:string):Promise<void>{
 try{await unlink(file);}
 catch(error){
  if((error as {code?:string}).code==="ENOENT")return;
  await chmod(file,0o666).catch(()=>{});
  await rm(file,{force:true});
 }
}

function holds(names:Buffer,fanout:Uint32Array,name:Buffer):boolean{
 const first=name[0]!;
 let low=first===0?0:fanout[first-1]!,high=fanout[first]!;
 while(low<high){
  const middle=(low+high)>>>1,order=names.compare(name,0,20,middle*20,middle*20+20);
  if(order===0)return true;
  if(order>0)high=middle;else low=middle+1;
 }
 return false;
}

function openObjectBatch(executable:string,worktree:string,signal?:AbortSignal,exact=false){
 const pins=GIT_PINS_V1.flatMap(([key,value])=>["-c",`${key}=${value}`]),env={...process.env,GIT_OPTIONAL_LOCKS:"0"};
 const local:NodeJS.ProcessEnv=env;
 if(exact){
  for(const name of Object.keys(local))if(name.startsWith("GIT_CONFIG_")||["GIT_CONFIG","GIT_COMMON_DIR","GIT_WORK_TREE","GIT_OBJECT_DIRECTORY","GIT_ALTERNATE_OBJECT_DIRECTORIES","GIT_ALLOW_PROTOCOL","GIT_INDEX_FILE"].includes(name))delete local[name];
  Object.assign(local,{GIT_DIR:join(worktree,".git"),GIT_CONFIG_NOSYSTEM:"1",GIT_CONFIG_GLOBAL:devNull,GIT_TERMINAL_PROMPT:"0",GIT_NO_LAZY_FETCH:"1",GIT_NO_REPLACE_OBJECTS:"1"});
 }
 return spawn(executable,["-C",worktree,...pins,...exact?["-c","protocol.allow=never","-c","core.deltaBaseCacheLimit=16m"]:[],"cat-file","--batch"],{stdio:["pipe","pipe","ignore"],env:local,windowsHide:true,signal});
}

function readObjects(executable:string,worktree:string,names:readonly string[],signal:AbortSignal|undefined):Promise<Map<string,Readonly<{type:string;content:Buffer}>>|undefined>{
 return new Promise(done=>{
  const child=openObjectBatch(executable,worktree,signal);
  const chunks:Buffer[]=[];
  child.stdout.on("data",chunk=>{chunks.push(chunk as Buffer);});
  child.stdin.on("error",()=>{});
  child.on("error",()=>done(undefined));
  child.on("close",code=>{
   if(code!==0){done(undefined);return;}
   const data=Buffer.concat(chunks),found=new Map<string,{type:string;content:Buffer}>();
   let offset=0;
   while(offset<data.length){
    const end=data.indexOf(0x0a,offset);
    if(end<0){done(undefined);return;}
    const [name,type,size]=data.subarray(offset,end).toString("latin1").split(" ");
    if(type===undefined||size===undefined||type==="missing"){done(undefined);return;}
    const length=Number(size);
    found.set(name!,{type,content:data.subarray(end+1,end+1+length)});
    offset=end+1+length+1;
   }
   done(found);
  });
  child.stdin.end(names.join("\n")+"\n");
 });
}

const looseHeader=(type:string,bytes:number)=>Buffer.from(`${type} ${bytes}\0`);
async function writeLoose(worktree:string,name:string,type:string,content:Buffer|AsyncIterable<Buffer>,signal?:AbortSignal):Promise<boolean>{
 const directory=join(worktree,".git","objects",name.slice(0,2)),file=join(directory,name.slice(2)),buffered=Buffer.isBuffer(content);
 if(buffered&&await lstat(file).then(()=>true,()=>false))return true;
 await mkdir(directory,{recursive:true});
 if(!buffered&&!(await lstat(directory)).isDirectory())return false;
 const temporary=`${file}.tmp-${randomBytes(5).toString("hex")}`;
 try{
  if(buffered)await writeFile(temporary,deflateSync(Buffer.concat([looseHeader(type,content.length),content]),{level:1}),{mode:0o444,flag:"wx"});
  else await pipeline(content,createDeflate({level:1}),createWriteStream(temporary,{flags:"wx",mode:0o444}),{signal});
  signal?.throwIfAborted();
  try{await rename(temporary,file);}catch(error){if(!buffered)throw error;}
  return buffered?lstat(file).then(()=>true,()=>false):true;
 }finally{await rm(temporary,{force:true}).catch(()=>{});}
}

/** 🛟️ Deals with the complete packs (pack and index, older than `minAgeMilliseconds`) of this machine that the fleet does not hold and that were not authorised: every object they carry that no fleet pack holds and no loose file exists for is written as a loose object (the replica then publishes it like any other file), and once nothing is missing any more the local pack is deleted (index first). Dead fragments (a pack without index, an index without pack) older than an hour are deleted too. A pack with more than `limit` missing objects, or one whose objects git cannot read, stays and is counted in `kept`. Nothing of the fleet's packs is touched. */
export async function rescueLocalPacksV1(worktree:string,known:(packPath:string)=>boolean,options:Readonly<{now?:number;minAgeMilliseconds?:number;limit?:number;signal?:AbortSignal}>={}):Promise<RescueV1>{
 const {signal}=options;
 signal?.throwIfAborted();
 const directory=join(worktree,".git","objects","pack"),now=options.now??Date.now(),minAge=options.minAgeMilliseconds??120_000,limit=options.limit??200_000,auth=readPackAuthorisationV1(worktree);
 const groups=new Map<string,Map<string,number>>();
 for(const name of await readdir(directory).catch(()=>[] as string[])){
  signal?.throwIfAborted();
  const parts=packPartsV1(`${packDirectoryV1}${name}`);
  if(parts===undefined||parts.stem==="multi-pack-index")continue;
  const info=await stat(join(directory,name)).catch(()=>undefined);
  if(info===undefined)continue;
  const group=groups.get(parts.stem)??new Map<string,number>();
  group.set(parts.extension,info.mtimeMs);groups.set(parts.stem,group);
 }
 const locals:string[]=[],shared:string[]=[];
 let objects=0,retired=0,kept=0;
 for(const [stem,group] of groups){
  signal?.throwIfAborted();
  const complete=group.has("pack")&&group.has("idx");
  if(known(`${packDirectoryV1}${stem}.pack`)||auth.publish.has(stem)){
   if(complete)shared.push(stem);
   continue;
  }
  const newest=Math.max(...group.values());
  if(complete){if(now-newest>=minAge)locals.push(stem);}
  else if(now-newest>=3_600_000){for(const extension of ["idx",...extensions])await remove(join(directory,`${stem}.${extension}`));await remove(join(directory,`${stem}.pack`));retired++;}
 }
 signal?.throwIfAborted();
 if(locals.length===0)return {examined:0,objects,retired,kept};
 const tables:Array<NonNullable<Awaited<ReturnType<typeof readPackNamesV1>>>>=[];
 for(const stem of shared){
  signal?.throwIfAborted();
  const table=await readPackNamesV1(join(directory,`${stem}.idx`));
  if(table!==undefined)tables.push(table);
 }
 signal?.throwIfAborted();
 const git=await resolveGitV1();
 for(const stem of locals){
  signal?.throwIfAborted();
  const table=await readPackNamesV1(join(directory,`${stem}.idx`));
  if(table===undefined||git===null){kept++;continue;}
  const missing:string[]=[];
  for(let index=0;index<table.count&&missing.length<=limit;index++){
   const name=table.names.subarray(index*20,index*20+20);
   if(tables.some(other=>holds(other.names,other.fanout,name)))continue;
   const hex=name.toString("hex");
   if(await lstat(join(worktree,".git","objects",hex.slice(0,2),hex.slice(2))).then(()=>true,()=>false))continue;
   missing.push(hex);
  }
  if(missing.length>limit){kept++;continue;}
  let complete=true;
  if(missing.length>0){
   const found=await readObjects(git,worktree,missing,signal);
   if(found===undefined)complete=false;
   else for(const name of missing){const item=found.get(name);if(item===undefined||!await writeLoose(worktree,name,item.type,item.content)){complete=false;break;}objects++;}
  }
  if(!complete){kept++;continue;}
  for(const extension of ["idx","rev","bitmap","mtimes","keep","promisor","pack"])await remove(join(directory,`${stem}.${extension}`));
  retired++;
 }
 return {examined:locals.length,objects,retired,kept};
}

/** 🧲️ The exact blob prerequisites available through current fleet packs, verified loose content, or neither. */
export type StagePackedObjectsV1=Readonly<{covered:readonly string[];loose:readonly string[];pending:readonly string[]}>;
const stagePackedPolicy=schema.definitions.StagePackedPolicy.const;
const stageOid=/^(?:[0-9a-f]{40}|[0-9a-f]{64})$/u;
const stageIndex=/^(pack-([0-9a-f]{40}))\.idx$/u;
const stamp=(value:Awaited<ReturnType<typeof statPackFile>>)=>value===undefined?"":[value.dev,value.ino,value.size,value.mtimeNs,value.ctimeNs].join(":");
const statPackFile=async(path:string)=>lstat(path,{bigint:true}).then(value=>value.isFile()?value:undefined,()=>undefined);

async function stagedPackNames(worktree:string,stem:string,digest:string,signal?:AbortSignal){
 const directory=join(worktree,".git","objects","pack"),pack=join(directory,stem+".pack"),idx=join(directory,stem+".idx");
 const [packInfo,idxInfo]=await Promise.all([statPackFile(pack),statPackFile(idx)]);
 if(packInfo===undefined||idxInfo===undefined||packInfo.size<32n||packInfo.size>BigInt(Number.MAX_SAFE_INTEGER)||idxInfo.size<1072n||idxInfo.size>BigInt(stagePackedPolicy.maximumIndexBytes))return;
 const packFile=await open(pack,"r").catch(()=>undefined);
 if(packFile===undefined)return;
 let indexFile:Awaited<ReturnType<typeof open>>|undefined;
 try{
  signal?.throwIfAborted();
  indexFile=await open(idx,"r");
  if(stamp(await packFile.stat({bigint:true}))!==stamp(packInfo)||stamp(await indexFile.stat({bigint:true}))!==stamp(idxInfo))return;
  const head=Buffer.alloc(12),trailer=Buffer.alloc(20),indexFooter=Buffer.alloc(40);
  if((await packFile.read(head,0,12,0)).bytesRead!==12||head.toString("ascii",0,4)!=="PACK"||![2,3].includes(head.readUInt32BE(4)))return;
  if((await packFile.read(trailer,0,20,Number(packInfo.size)-20)).bytesRead!==20||trailer.toString("hex")!==digest)return;
  if((await indexFile.read(indexFooter,0,40,Number(idxInfo.size)-40)).bytesRead!==40||!indexFooter.subarray(0,20).equals(trailer))return;
  const hash=createHash("sha1");
  for await(const bytes of indexFile.createReadStream({start:0,end:Number(idxInfo.size)-21,autoClose:false,signal}))hash.update(bytes);
  if(!hash.digest().equals(indexFooter.subarray(20)))return;
  const table=await readPackNamesV1(idx,{maximumBytes:stagePackedPolicy.maximumIndexBytes,expectedIdentity:stamp(idxInfo),signal});
  if(table===undefined||table.count!==head.readUInt32BE(8)||stamp(await statPackFile(pack))!==stamp(packInfo)||stamp(await statPackFile(idx))!==stamp(idxInfo))return;
  return table;
 }catch{signal?.throwIfAborted();}finally{await Promise.allSettled([packFile.close(),indexFile?.close()]);}
}

async function materializeStageBlobs(executable:string,worktree:string,names:readonly string[],written:Set<string>,signal?:AbortSignal):Promise<void>{
 signal?.throwIfAborted();
 const child=openObjectBatch(executable,worktree,signal,true);
 const closed=new Promise<void>(done=>{child.on("error",()=>{});child.once("close",()=>done());});
 child.stdin.on("error",()=>{});
 child.stdin.end(names.join("\n")+"\n");
 const reader=child.stdout[Symbol.asyncIterator]() as AsyncIterator<Buffer>,header=Buffer.alloc(stagePackedPolicy.maximumHeaderBytes);
 let held:Buffer=Buffer.alloc(0);
 const more=async():Promise<boolean>=>{
  signal?.throwIfAborted();
  if(held.length>0)return true;
  const next=await reader.next();
  if(next.done)return false;
  held=next.value;return true;
 };
 const line=async():Promise<string>=>{
  let seen=0;
  while(await more()){
   const lf=held.indexOf(10),size=lf<0?held.length:lf;
   if(seen+size>header.length)break;
   held.copy(header,seen,0,size);seen+=size;held=held.subarray(size+(lf<0?0:1));
   if(lf>=0)return header.toString("ascii",0,seen);
  }
  throw Error("Invalid exact Git object response header");
 };
 const canonical=async function*(oid:string,size:number):AsyncGenerator<Buffer>{
  const prefix=looseHeader("blob",size),hash=createHash(oid.length===40?"sha1":"sha256").update(prefix);
  yield prefix;
  let left=size;
  while(left>0&&await more()){
   const count=Math.min(left,held.length),bytes=held.subarray(0,count);
   held=held.subarray(count);left-=count;hash.update(bytes);yield bytes;
  }
  if(left!==0||!await more()||held[0]!==10||hash.digest("hex")!==oid)throw Error("Git blob failed canonical verification");
  held=held.subarray(1);
 };
 try{
  for(const oid of names){
   const row=await line();
   if(row===oid+" missing")continue;
   const response=/^([0-9a-f]+) (blob|tree|commit|tag) (0|[1-9][0-9]*)$/u.exec(row),size=Number(response?.[3]);
   if(response?.[1]!==oid||!Number.isSafeInteger(size))break;
   if(response[2]!=="blob"){
    let left=size;
    while(left>0&&await more()){const count=Math.min(left,held.length);held=held.subarray(count);left-=count;}
    if(left!==0||!await more()||held[0]!==10)break;
    held=held.subarray(1);continue;
   }
   if(!await writeLoose(worktree,oid,"blob",canonical(oid,size),signal))break;
   written.add(oid);
  }
 }catch{signal?.throwIfAborted();}finally{
  child.stdout.destroy();
  if(child.exitCode===null)try{child.kill();}catch{}
  await closed;
 }
 signal?.throwIfAborted();
}

/** 🧮️ Resolves only explicit staged blob IDs. A covered ID requires a current complete fleet pack pair. Other readable blobs become canonical loose objects. */
export async function prepareStagePackedObjectsV1(worktree:string,oids:readonly string[],livePackPath:(path:string)=>boolean,signal?:AbortSignal):Promise<StagePackedObjectsV1>{
 signal?.throwIfAborted();
 const ordered=[...new Set(oids)],missing=new Set(ordered.filter(oid=>stageOid.test(oid))),covered=new Set<string>(),loose=new Set<string>();
 if([...missing].some(oid=>oid.length===40)){
  const directory=await opendir(join(worktree,".git","objects","pack")).catch(()=>undefined);
  if(directory!==undefined)for await(const item of directory){
   signal?.throwIfAborted();
   if(missing.size===0)break;
   const match=stageIndex.exec(item.name);
   if(match===null||!item.isFile())continue;
   const [,stem,hash]=match,pack=packDirectoryV1+stem+".pack",idx=packDirectoryV1+item.name;
   if(!livePackPath(pack)||!livePackPath(idx))continue;
   const table=await stagedPackNames(worktree,stem!,hash!,signal);
   if(table===undefined||!livePackPath(pack)||!livePackPath(idx))continue;
   for(const oid of missing)if(oid.length===40&&holds(table.names,table.fanout,Buffer.from(oid,"hex"))){missing.delete(oid);covered.add(oid);}
  }
 }
 const names=[...missing],git=names.length===0?null:await resolveGitV1();
 if(git!==null)for(let at=0;at<names.length;at+=stagePackedPolicy.maximumBatchObjects)
  await materializeStageBlobs(git,worktree,names.slice(at,at+stagePackedPolicy.maximumBatchObjects),loose,signal);
 signal?.throwIfAborted();
 return {covered:ordered.filter(oid=>covered.has(oid)),loose:ordered.filter(oid=>loose.has(oid)),pending:ordered.filter(oid=>!covered.has(oid)&&!loose.has(oid))};
}
