import {createHash,randomBytes} from "node:crypto";
import {createReadStream} from "node:fs";
import {copyFile,lstat,mkdir,open,readFile,readdir,rename,stat,unlink,utimes} from "node:fs/promises";
import {dirname,join,sep} from "node:path";
import {compileJsonSchemaSubset,type CompiledJsonSchemaSubset} from "../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧾️ One journal line: a put, delete or link of one shared path, ordered by its hybrid logical clock. */
export type FleetEventV1=Readonly<{version:1;hlc:string;operation:"put"|"delete"|"link";path:string;sha256?:string;bytes?:number;executable?:boolean;target?:string;base:string|null}>;

/** 🧭️ The published extent of one origin: readers trust the first `bytes` bytes of `segment` and every earlier segment. */
export type FleetHeadV1=Readonly<{version:1;segment:number;bytes:number;events:number;hlc:string}>;

/** 📍️ The next unread position inside one origin's segments. */
export type FleetCursorV1=Readonly<{segment:number;bytes:number}>;

/** ⏱️ The last issued or observed hybrid logical clock value. */
export type FleetClockV1=Readonly<{ms:number;counter:number}>;

/** 🗃️ The winning state of one path in a manifest; a delete is a tombstone. */
export type FleetManifestEntryV1=Readonly<{hlc:string;operation:"put"|"delete"|"link";sha256?:string;bytes?:number;executable?:boolean;target?:string}>;

const snapshotChunk=1<<20,headRevalidateMilliseconds=5000,segmentBytes=4*1024*1024,segmentEvents=4096,futureMilliseconds=60_000,maximumCounter=999_999,maximumMilliseconds=9_999_999_999_999;
const hlcPattern=/^([0-9]{13})-([0-9]{6})-([a-z0-9][a-z0-9-]{0,39})$/u,originPattern=/^[a-z0-9][a-z0-9-]{0,39}$/u,hexPattern=/^[0-9a-f]{64}$/u;
const validators=new Map<string,CompiledJsonSchemaSubset>(),resolvedStores=new Map<string,Readonly<{root:string;origins:string;blobs:string;tmp:string;snapshots:string}>>(),knownHeads=new Map<string,Readonly<{signature:string;at:number;text:string;head:FleetHeadV1}>>();
const check=(definition:string,value:unknown,message:string):void=>{
 let validator=validators.get(definition);
 if(validator===undefined){validator=compileJsonSchemaSubset({$ref:`#/definitions/${definition}`},schema);validators.set(definition,validator);}
 const errors=validator.validate(value);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
};
const missing=(error:unknown):boolean=>(error as {code?:string}).code==="ENOENT";
const exists=async(file:string):Promise<boolean>=>{try{await stat(file);return true;}catch(error){if(missing(error))return false;throw error;}};
const segmentName=(segment:number):string=>`${String(segment).padStart(12,"0")}.jsonl`;
const byKey=(left:readonly [string,unknown],right:readonly [string,unknown]):number=>left[0]<right[0]?-1:left[0]>right[0]?1:0;
const stage=(tmp:string,originId:string):string=>join(tmp,`${originId}-${process.pid}-${randomBytes(8).toString("hex")}`);
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as {code?:string}).code==="EPERM";}};

/** 🧹️ Reclaims staging files that a crash (`kill -9`) left in `<store>/v1/tmp`; returns how many files and bytes were removed. With `originId` it removes that origin's own leftovers at once: names `<originId>-<pid>-<hex>` whose process is dead and the older `<originId>-<hex>` names; a name of a live process (a `compact` running beside the daemon) stays. With `olderThanMilliseconds` it removes any file whose newest timestamp (modification or inode change) is older than that, which also covers the relay's files and the ones of other origins, and must be far beyond the longest write; `.nfs*` placeholders stay. A store without `tmp` yields nothing and is never created. */
export async function reclaimStagingV1(storeRoot:string,options:Readonly<{originId?:string;olderThanMilliseconds?:number;now?:number}>={}):Promise<Readonly<{files:number;bytes:number}>>{
 const {tmp}=storePathsV1(storeRoot),own=options.originId===undefined?undefined:new RegExp(`^${options.originId}-(?:([0-9]+)-)?[0-9a-f]{16}$`,"u"),now=options.now??Date.now();
 let files=0,bytes=0;
 for(const name of await readdir(tmp).catch(()=>[] as string[])){
  if(name.startsWith(".nfs"))continue;
  const found=own?.exec(name),file=join(tmp,name),info=await lstat(file).catch(()=>undefined);
  if(info===undefined||!info.isFile())continue;
  const dead=found!==undefined&&found!==null&&(found[1]===undefined||!alive(Number(found[1])));
  const old=options.olderThanMilliseconds!==undefined&&now-Math.max(info.mtimeMs,info.ctimeMs)>options.olderThanMilliseconds;
  if(!dead&&!old)continue;
  await unlink(file).then(()=>{files++;bytes+=info.size;},()=>{});
 }
 return {files,bytes};
}

/** ⏰️ Issues the next clock value for a local event: `max(wall, last.ms)`, counter +1 when the millisecond did not advance. */
export function tickClockV1(last:FleetClockV1,wallMs:number):FleetClockV1{
 if(!Number.isSafeInteger(wallMs)||wallMs<0)throw Error("Fleet wall clock must be a non-negative integer");
 const ms=Math.max(wallMs,last.ms),counter=ms>last.ms?0:last.counter+1;
 if(ms>maximumMilliseconds||counter>maximumCounter)throw Error("Fleet clock exhausted");
 return {ms,counter};
}

/** 🔀️ Folds a received hlc into the clock (`max`) and refuses one more than 60 seconds ahead of the wall clock. */
export function mergeClockV1(last:FleetClockV1,receivedHlc:string,wallMs:number):FleetClockV1{
 const received=parseHlcV1(receivedHlc);
 if(received.ms>wallMs+futureMilliseconds)throw Error(`Fleet hlc is ${received.ms-wallMs} ms ahead of the wall clock: ${receivedHlc}`);
 return received.ms>last.ms||(received.ms===last.ms&&received.counter>last.counter)?{ms:received.ms,counter:received.counter}:{ms:last.ms,counter:last.counter};
}

/** 🖨️ Renders a clock value as the fixed-width hlc string that sorts like the total order. */
export function formatHlcV1(clock:FleetClockV1,originId:string):string{
 requireOriginIdV1(originId);
 if(!Number.isSafeInteger(clock.ms)||clock.ms<0||clock.ms>maximumMilliseconds||!Number.isSafeInteger(clock.counter)||clock.counter<0||clock.counter>maximumCounter)throw Error("Fleet clock out of range");
 return `${String(clock.ms).padStart(13,"0")}-${String(clock.counter).padStart(6,"0")}-${originId}`;
}

/** 🔬️ Splits an hlc string into its milliseconds, counter and origin. */
export function parseHlcV1(hlc:string):Readonly<{ms:number;counter:number;originId:string}>{
 const match=typeof hlc==="string"?hlcPattern.exec(hlc):null;
 if(!match)throw Error(`Closed fleet hlc required: ${String(hlc)}`);
 return {ms:Number(match[1]),counter:Number(match[2]),originId:match[3]!};
}

/** 🏆️ Decides whether a candidate beats the current state of a path: the greater hlc string wins, equal never wins. */
export function winsV1(candidateHlc:string,currentHlc:string|undefined):boolean{
 return currentHlc===undefined||candidateHlc>currentHlc;
}

/** 🛡️ Validates one event against the closed schema and refuses paths with `.` or `..` components. */
export function requireEventV1(value:unknown):FleetEventV1{
 check("Event",value,"Closed fleet event required");
 const event=value as FleetEventV1;
 if(event.path.split("/").some(part=>part==="."||part===".."))throw Error("Closed fleet event required: path must stay relative");
 return event;
}

/** 🆔️ Admits only a lowercase ASCII origin id of at most 40 characters. */
export function requireOriginIdV1(value:string):string{
 if(typeof value!=="string"||!originPattern.test(value))throw Error(`Closed fleet origin id required: ${String(value)}`);
 return value;
}

/** 🗂️ Resolves the fixed directories of a store below `<storeRoot>/v1`. */
export function storePathsV1(storeRoot:string):Readonly<{root:string;origins:string;blobs:string;tmp:string;snapshots:string}>{
 const known=resolvedStores.get(storeRoot);
 if(known!==undefined)return known;
 const root=join(storeRoot,"v1"),paths=Object.freeze({root,origins:join(root,"origins"),blobs:join(root,"blobs"),tmp:join(root,"tmp"),snapshots:join(root,"snapshots")});
 if(resolvedStores.size>=64)resolvedStores.clear();
 resolvedStores.set(storeRoot,paths);
 return paths;
}

/** 🧱️ Resolves the immutable blob file of a content hash. */
export function blobPathV1(storeRoot:string,sha256:string):string{
 if(typeof sha256!=="string"||!hexPattern.test(sha256))throw Error(`Closed fleet sha256 required: ${String(sha256)}`);
 return join(storePathsV1(storeRoot).blobs,sha256.slice(0,2),sha256);
}

async function renameWithRetry(from:string,to:string):Promise<void>{
 for(let attempt=0;;attempt++){
  try{await rename(from,to);return;}
  catch(error){
   const name=(error as {code?:string}).code;
   if((name!=="EPERM"&&name!=="EBUSY"&&name!=="EACCES")||attempt>=4)throw error;
   await new Promise<void>(done=>{setTimeout(done,25*2**attempt);});
  }
 }
}

const seenRoots=new Set<string>(),knownOrigins=new Map<string,{signature:string;at:number;names:readonly string[]}>();

/** 👁️ Remembers that this process has seen the store root `v1` exist; from then on a missing `v1` is an unmounted share, not a fresh store. */
export function markStoreSeenV1(storeRoot:string):void{
 if(seenRoots.size>=64)seenRoots.clear();
 seenRoots.add(storePathsV1(storeRoot).root);
}

/** 📂️ Creates a directory below the store root `v1` one level at a time. A store root that this process has seen is never created again: when the share has vanished (unmounted, mount point left as an empty local directory) the write fails with ENOENT instead of rebuilding the store as a local copy. The first write into a store this process has never seen creates it. */
export async function ensureStoreDirectoryV1(storeRoot:string,directory:string):Promise<void>{
 const root=storePathsV1(storeRoot).root;
 const make=async (path:string):Promise<void>=>{
  if(path===root){if(seenRoots.has(root))await stat(root);else{await mkdir(root,{recursive:true});markStoreSeenV1(storeRoot);}return;}
  try{await mkdir(path);}
  catch(error){
   const kind=(error as {code?:string}).code;
   if(kind==="EEXIST")return;
   if(kind==="ENOENT"&&path.startsWith(root+sep)){await make(dirname(path));await make(path);return;}
   throw error;
  }
 };
 await make(directory);
}

async function replaceFile(tmp:string,originId:string,target:string,text:string):Promise<void>{
 await ensureStoreDirectoryV1(dirname(dirname(tmp)),tmp);
 const temporary=stage(tmp,originId);
 try{
  const handle=await open(temporary,"w");
  try{await handle.writeFile(text);await handle.sync();}finally{await handle.close();}
  await renameWithRetry(temporary,target);
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
}

/** 📥️ Stores a file as an immutable blob through tmp + rename after rehashing the staged copy; `false` means it was already there. */
export async function writeBlobV1(storeRoot:string,originId:string,sha256:string,source:string):Promise<boolean>{
 requireOriginIdV1(originId);
 const target=blobPathV1(storeRoot,sha256),{tmp}=storePathsV1(storeRoot);
 if(await exists(target)){const now=new Date();await utimes(target,now,now).catch(()=>{});return false;}
 await ensureStoreDirectoryV1(storeRoot,dirname(target));await ensureStoreDirectoryV1(storeRoot,tmp);
 const temporary=stage(tmp,originId);
 try{
  await copyFile(source,temporary);
  const hash=createHash("sha256");
  for await(const chunk of createReadStream(temporary))hash.update(chunk);
  if(hash.digest("hex")!==sha256)throw Error(`Fleet blob source changed while publishing: ${sha256}`);
  const handle=await open(temporary,"r+");
  try{await handle.sync();}finally{await handle.close();}
  try{await rename(temporary,target);}
  catch(error){
   if((error as {code?:string}).code!=="EEXIST"&&!await exists(target))throw error;
   await unlink(temporary).catch(()=>{});return false;
  }
  return true;
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
}

/** ✍️ Appends event lines to the origin's segments (fsync), sealing full segments, then publishes the new head through tmp + rename. */
export async function appendEventsV1(storeRoot:string,originId:string,events:readonly FleetEventV1[]):Promise<FleetHeadV1>{
 requireOriginIdV1(originId);
 const lines=events.map(event=>{
  requireEventV1(event);
  if(parseHlcV1(event.hlc).originId!==originId)throw Error(`Fleet event hlc belongs to another origin than ${originId}: ${event.hlc}`);
  return Buffer.from(`${JSON.stringify(event)}\n`,"utf8");
 });
 const prior=await readHeadV1(storeRoot,originId);
 if(lines.length===0){
  if(prior)return prior;
  throw Error("Fleet append requires at least one event");
 }
 const {origins,tmp}=storePathsV1(storeRoot),directory=join(origins,originId),segments=join(directory,"segments");
 await ensureStoreDirectoryV1(storeRoot,segments);
 let segment=prior?.segment??0,bytes=prior?.bytes??0,count=prior?.events??0,hlc=prior?.hlc??"",index=0;
 while(index<lines.length){
  if(count>=segmentEvents||bytes>=segmentBytes){segment++;bytes=0;count=0;}
  let end=index,size=bytes;
  while(end<lines.length&&count+(end-index)<segmentEvents&&(end===index||size+lines[end]!.length<=segmentBytes)){size+=lines[end]!.length;end++;}
  const data=Buffer.concat(lines.slice(index,end)),file=join(segments,segmentName(segment)),handle=await open(file,await exists(file)?"r+":"w+");
  try{
   const current=(await handle.stat()).size;
   if(current<bytes)throw Error(`Fleet segment is shorter than its head: ${file}`);
   if(current>bytes)await handle.truncate(bytes);
   let done=0;
   while(done<data.length){const {bytesWritten}=await handle.write(data,done,data.length-done,bytes+done);done+=bytesWritten;}
   await handle.sync();
  }finally{await handle.close();}
  for(let at=index;at<end;at++)if(events[at]!.hlc>hlc)hlc=events[at]!.hlc;
  bytes+=data.length;count+=end-index;index=end;
 }
 const head:FleetHeadV1={version:1,segment,bytes,events:count,hlc};
 check("Head",head,"Closed fleet head required");
 await replaceFile(tmp,originId,join(directory,"head.json"),JSON.stringify(head));
 return head;
}

/** 👀️ Reads the published head of an origin; `undefined` when the origin has none yet. */
export async function readHeadV1(storeRoot:string,originId:string):Promise<FleetHeadV1|undefined>{
 requireOriginIdV1(originId);
 const file=join(storePathsV1(storeRoot).origins,originId,"head.json"),key=`${storeRoot}\0${originId}`,known=knownHeads.get(key),now=Date.now();
 let signature:string,text:string;
 try{
  const info=await stat(file);
  signature=`${info.ino}:${info.size}:${info.mtimeMs}`;
  if(known!==undefined&&known.signature===signature&&now-known.at<headRevalidateMilliseconds)return known.head;
  text=await readFile(file,"utf8");
 }catch(error){if(missing(error))return undefined;throw error;}
 if(known!==undefined&&known.text===text){knownHeads.set(key,{signature,at:now,text,head:known.head});return known.head;}
 const value=JSON.parse(text) as unknown;
 check("Head",value,"Closed fleet head required");
 if(knownHeads.size>=4096)knownHeads.clear();
 knownHeads.set(key,{signature,at:now,text,head:value as FleetHeadV1});
 return value as FleetHeadV1;
}

/** 📇️ Lists the origin directories of a store in sorted order; a missing store has none. */
export async function listOriginsV1(storeRoot:string):Promise<readonly string[]>{
 try{
  const directory=storePathsV1(storeRoot).origins,info=await stat(directory),signature=`${info.ino}:${info.mtimeMs}:${info.nlink}`,known=knownOrigins.get(storeRoot),now=Date.now();
  if(known!==undefined&&known.signature===signature&&now-known.at<headRevalidateMilliseconds){markStoreSeenV1(storeRoot);return known.names;}
  const entries=await readdir(directory,{withFileTypes:true});
  markStoreSeenV1(storeRoot);
  const names=entries.filter(entry=>entry.isDirectory()&&originPattern.test(entry.name)).map(entry=>entry.name).sort();
  if(knownOrigins.size>=64)knownOrigins.clear();
  knownOrigins.set(storeRoot,{signature,at:now,names});
  return names;
 }catch(error){if(missing(error))return [];throw error;}
}

async function readRange(file:string,from:number,to:number):Promise<Buffer>{
 const handle=await open(file,"r");
 try{
  const length=Math.max(0,Math.min(to,(await handle.stat()).size)-from),data=Buffer.alloc(length);
  let done=0;
  while(done<length){const {bytesRead}=await handle.read(data,done,length-done,from+done);if(bytesRead===0)break;done+=bytesRead;}
  return data.subarray(0,done);
 }finally{await handle.close();}
}

/** 📏️ Like `readEventsV1` and additionally returns `ends`, the cursor just behind each event (`ends[i]` belongs to `events[i]`), so a caller can consume only a prefix of the batch and keep a byte cursor in front of the first unconsumed event. With `segments` it reads at most that many segments (from the cursor's), so a long journal can be folded in bounded memory by calling it again with the returned cursor until the cursor stops advancing. */
export async function readEventSpansV1(storeRoot:string,originId:string,after:FleetCursorV1|undefined,segments?:number):Promise<Readonly<{events:readonly FleetEventV1[];ends:readonly FleetCursorV1[];cursor:FleetCursorV1}>>{
 const start=after??{segment:0,bytes:0},head=await readHeadV1(storeRoot,originId);
 if(!head||start.segment>head.segment||(start.segment===head.segment&&start.bytes>=head.bytes))return {events:[],ends:[],cursor:start};
 const directory=join(storePathsV1(storeRoot).origins,originId,"segments"),events:FleetEventV1[]=[],ends:FleetCursorV1[]=[];
 let cursor:FleetCursorV1=start;
 const last=segments===undefined?head.segment:Math.min(head.segment,start.segment+segments-1);
 for(let segment=start.segment;segment<=last;segment++){
  const from=segment===start.segment?start.bytes:0,limit=segment===head.segment?head.bytes:Infinity;
  const data=limit<=from?Buffer.alloc(0):await readRange(join(directory,segmentName(segment)),from,limit);
  let local=0;
  for(;;){
   const newline=data.indexOf(10,local);
   if(newline<0)break;
   const line=data.toString("utf8",local,newline);
   if(line!==""){
    try{events.push(requireEventV1(JSON.parse(line)));}
    catch(error){throw Error(`Fleet segment ${segmentName(segment)} of ${originId} is corrupt at byte ${from+local}: ${(error as Error).message}`);}
    ends.push({segment,bytes:from+newline+1});
   }
   local=newline+1;
  }
  cursor={segment,bytes:from+local};
  if(segment<head.segment&&local<data.length)break;
  if(segment===last&&last<head.segment)cursor={segment:segment+1,bytes:0};
 }
 return {events,ends,cursor};
}

/** 📖️ Reads the complete event lines after a cursor across segments, trusting only `head.bytes` of the head segment; returns the advanced cursor. */
export async function readEventsV1(storeRoot:string,originId:string,after:FleetCursorV1|undefined):Promise<Readonly<{events:readonly FleetEventV1[];cursor:FleetCursorV1}>>{
 const {events,cursor}=await readEventSpansV1(storeRoot,originId,after);
 return {events,cursor};
}

const entryOf=(event:FleetEventV1):FleetManifestEntryV1=>event.operation==="put"?{hlc:event.hlc,operation:"put",sha256:event.sha256!,bytes:event.bytes!,executable:event.executable!}
 :event.operation==="link"?{hlc:event.hlc,operation:"link",target:event.target!}:{hlc:event.hlc,operation:"delete"};

/** 🎯️ What a fold needs of a manifest: lookup and store of one path; a `Map` and the compact tables of the replica both qualify. */
export type FleetManifestTargetV1=Readonly<{get(path:string):FleetManifestEntryV1|undefined;set(path:string,entry:FleetManifestEntryV1):unknown}>;

/** 🧮️ Folds events into the manifest in place and returns the events that won and still stand, ordered by hlc then path. */
export function foldEventsV1(manifest:FleetManifestTargetV1,events:readonly FleetEventV1[]):readonly FleetEventV1[]{
 const won=new Map<string,FleetEventV1>();
 for(const event of events){
  if(!winsV1(event.hlc,manifest.get(event.path)?.hlc))continue;
  manifest.set(event.path,entryOf(event));won.set(event.path,event);
 }
 return [...won.values()].sort((left,right)=>left.hlc<right.hlc?-1:left.hlc>right.hlc?1:left.path<right.path?-1:left.path>right.path?1:0);
}

/** 📸️ Writes a full manifest and the cursors it covers as `snapshots/<originId>/<hlc>.json` and returns that file; the document is the same single JSON object as ever but is streamed in sorted order through a temporary file, so the manifest never exists as one string. */
export async function writeSnapshotV1(storeRoot:string,originId:string,hlc:string,manifest:Readonly<{size:number;keys():Iterable<string>;get(path:string):FleetManifestEntryV1|undefined}>,cursors:Readonly<Record<string,FleetCursorV1>>):Promise<string>{
 requireOriginIdV1(originId);
 const sortedCursors=Object.fromEntries(Object.entries(cursors).sort(byKey));
 check("Snapshot",{version:1,hlc,manifest:{},cursors:sortedCursors},"Closed fleet snapshot required");
 const {snapshots,tmp}=storePathsV1(storeRoot),directory=join(snapshots,originId),target=join(directory,`${hlc}.json`);
 await ensureStoreDirectoryV1(storeRoot,directory);await ensureStoreDirectoryV1(storeRoot,tmp);
 const temporary=stage(tmp,originId),keys=[...manifest.keys()].sort((left,right)=>left<right?-1:left>right?1:0);
 try{
  const handle=await open(temporary,"w");
  try{
   let text=`{"version":1,"hlc":${JSON.stringify(hlc)},"manifest":{`,first=true;
   for(const path of keys){
    const entry=manifest.get(path);
    if(entry===undefined)continue;
    check("ManifestEntry",entry,"Closed fleet snapshot required");
    text+=`${first?"":","}${JSON.stringify(path)}:${JSON.stringify(entry)}`;first=false;
    if(text.length>=snapshotChunk){await handle.write(text);text="";}
   }
   await handle.write(`${text}},"cursors":${JSON.stringify(sortedCursors)}}`);
   await handle.sync();
  }finally{await handle.close();}
  await renameWithRetry(temporary,target);
 }catch(error){await unlink(temporary).catch(()=>{});throw error;}
 return target;
}

const blank=(code:number):boolean=>code===32||code===10||code===13||code===9;

/** 🌊️ Streams the members of a big JSON file without holding it: for every top-level member whose key is in `streamed` and whose value is an object, `visit(outer,inner,text)` is called once per inner member with the raw JSON text of its value; every other top-level member is delivered once as `visit(key,undefined,text)`; `opened(outer)` is called when a streamed member starts, so an empty one is still noticed; `chunk` is the read size in bytes. Any valid JSON formatting is accepted; a malformed or non-object document throws. */
export async function streamMembersV1(file:string,streamed:ReadonlySet<string>,visit:(outer:string,inner:string|undefined,text:string)=>void,opened:(outer:string)=>void=()=>{},chunk:number=1<<14):Promise<void>{
 const fail=():never=>{throw Error(`Closed JSON object required: ${file}`);};
 let started=false,done=false,inside=false,inString=false,escaped=false,keying=false,active=false,nest=0,from=0,part="",outer="",inner="",expect:"key"|"colon"|"value"|"after"="key";
 for await(const piece of createReadStream(file,{encoding:"utf8",highWaterMark:chunk})){
  const chunk=piece as string;
  if(active)from=0;
  for(let at=0;at<chunk.length;at++){
   const code=chunk.charCodeAt(at);
   if(inString){
    if(escaped)escaped=false;
    else if(code===92)escaped=true;
    else if(code===34){
     inString=false;
     if(keying){
      keying=false;active=false;
      const key=JSON.parse(part+chunk.slice(from,at+1)) as string;
      part="";
      if(inside)inner=key;else outer=key;
      expect="colon";
     }
    }
    continue;
   }
   if(done){if(!blank(code))fail();continue;}
   if(!started){if(blank(code))continue;if(code!==123)fail();started=true;expect="key";continue;}
   if(expect==="key"){
    if(blank(code))continue;
    if(code===34){inString=true;keying=true;active=true;from=at;}
    else if(code===125){if(inside){inside=false;expect="after";}else done=true;}
    else fail();
    continue;
   }
   if(expect==="colon"){if(blank(code))continue;if(code!==58)fail();expect="value";continue;}
   if(expect==="after"){
    if(blank(code))continue;
    if(code===44)expect="key";else if(code===125)done=true;else fail();
    continue;
   }
   if(!active){
    if(blank(code))continue;
    if(!inside&&code===123&&streamed.has(outer)){inside=true;expect="key";opened(outer);continue;}
    active=true;from=at;nest=0;
   }
   if(code===34){inString=true;continue;}
   if(code===123||code===91){nest++;continue;}
   if((code===125||code===93)&&nest>0){nest--;continue;}
   if(nest===0&&(code===44||code===125)){
    const text=(part+chunk.slice(from,at)).trim();
    part="";active=false;
    visit(outer,inside?inner:undefined,text);
    if(code===44)expect="key";
    else if(inside){inside=false;expect="after";}
    else done=true;
   }
  }
  if(active){part+=chunk.slice(from);}
 }
 if(!done)fail();
}

/** 🎞️ Reads the snapshot with the greatest hlc across all origins (or only the snapshots written by `onlyOrigin`), skipping unreadable ones; `undefined` when there is none. The manifest is streamed entry by entry into `create()` (a fresh `Map` by default), so a big snapshot needs no more memory than the manifest it fills. */
export async function readNewestSnapshotV1<M extends FleetManifestTargetV1=Map<string,FleetManifestEntryV1>>(storeRoot:string,onlyOrigin?:string,create:()=>M=()=>new Map<string,FleetManifestEntryV1>() as unknown as M):Promise<Readonly<{hlc:string;manifest:M;cursors:Record<string,FleetCursorV1>}>|undefined>{
 const {snapshots}=storePathsV1(storeRoot),found:{hlc:string;file:string}[]=[];
 let origins:string[];
 try{origins=await readdir(snapshots);}catch(error){if(missing(error))return undefined;throw error;}
 for(const origin of origins){
  if(!originPattern.test(origin)||(onlyOrigin!==undefined&&origin!==onlyOrigin))continue;
  let names:string[];
  try{names=await readdir(join(snapshots,origin));}catch(error){if(missing(error))continue;throw error;}
  for(const name of names)if(name.endsWith(".json")&&hlcPattern.test(name.slice(0,-5)))found.push({hlc:name.slice(0,-5),file:join(snapshots,origin,name)});
 }
 found.sort((left,right)=>left.hlc<right.hlc?1:left.hlc>right.hlc?-1:0);
 for(const {hlc,file} of found){
  try{
   const manifest=create(),top:Record<string,unknown>={};
   await streamMembersV1(file,new Set(["manifest"]),(outer,inner,text)=>{
    if(outer!=="manifest"){top[outer]=JSON.parse(text);return;}
    const entry=JSON.parse(text) as FleetManifestEntryV1;
    check("Path",inner,"Closed fleet snapshot required");
    check("ManifestEntry",entry,"Closed fleet snapshot required");
    manifest.set(inner!,entry);
   });
   check("Snapshot",{...top,manifest:{}},"Closed fleet snapshot required");
   if(top.hlc!==hlc)continue;
   return {hlc,manifest,cursors:top.cursors as Record<string,FleetCursorV1>};
  }catch{continue;}
 }
 return undefined;
}
