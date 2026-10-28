import {createHash,randomBytes} from "node:crypto";
import {closeSync,constants,copyFileSync,fsyncSync,linkSync,lstatSync,mkdirSync,openSync,readFileSync,readSync,readdirSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {basename,dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {UNIT_CACHE_POLICY_V1} from "../../🔏️key/🟦️.ts";
import {laneMessageV1} from "../../🚪️launch/🟦️.ts";
import {lanePathsV1} from "../🟦️.ts";
import {refuseCachingV1} from "../../../../📖️texts/🟦️.ts";

const rules=UNIT_CACHE_POLICY_V1.retention,layout=UNIT_CACHE_POLICY_V1.layout,pattern=new RegExp(rules.ledgerPattern,"u");
/** 🧾️ One retained file as the plan sees it: its name, the SHA-256 of its bytes and their count. */
export type RetainedEntryV1=Readonly<{name:string;sha256:string;bytes:number}>;
/** 🎬️ What retention does with one lane file: copy it to the shared ledger (`publish`), drop it because the shared ledger holds the same bytes (`same`), keep it and report it because the name exists there with other bytes (`conflict`), or leave it alone because it is too large (`refuse`). */
export type RetentionActionV1=Readonly<{action:"publish"|"same"|"conflict"|"refuse";name:string;bytes:number}>;
/** 📊️ What one retention pass did. */
export type RetentionSummaryV1=Readonly<{published:number;same:number;conflicts:readonly string[];refused:number;bytes:number}>;

const byName=(left:{name:string},right:{name:string}):number=>left.name<right.name?-1:left.name>right.name?1:0;

/** 🧭️ Plans the retention of a lane's provenance ledger. Pure and deterministic: lane files named like a receipt, in name order, at most `maximumFiles` of them; a name the shared ledger lacks is published, one it holds with the same digest is dropped, one it holds with another digest is a conflict that is never overwritten, one over `maximumFileBytes` is refused. Applying the result and planning again publishes nothing. */
export function planRetentionV1(lane:readonly RetainedEntryV1[],shared:readonly RetainedEntryV1[]):RetentionActionV1[]{
 const known=new Map(shared.map(entry=>[entry.name,entry]));
 return lane.filter(entry=>pattern.test(entry.name)).sort(byName).slice(0,rules.maximumFiles).map(entry=>{
  if(entry.bytes>rules.maximumFileBytes)return{action:"refuse",name:entry.name,bytes:entry.bytes};
  const old=known.get(entry.name);
  return{action:old===undefined?"publish":old.sha256===entry.sha256?"same":"conflict",name:entry.name,bytes:entry.bytes};
 });
}

const insideV1=(parent:string,path:string):boolean=>{const part=relative(parent,path);return part!==""&&!isAbsolute(part)&&part.split(sep)[0]!=="..";};

/** 🛤️ Whether a build directory is the build directory of a lane of the pool under `cacheRoot` (`<cacheRoot>/<lanes>/<id>/<build>` with a valid lane identifier). Pure. */
export function laneBuildDirectoryOwnedV1(cacheRoot:string,buildDirectory:string):boolean{
 const root=resolve(cacheRoot),build=resolve(buildDirectory),parts=relative(join(root,layout.lanes),build).split(sep);
 if(parts.length!==2||parts[1]!==layout.build)return false;
 try{return resolve(lanePathsV1(root,parts[0]!).build)===build;}catch{return false;}
}

/** 🪪️ Whether a recorded compiler origin belongs to the compiler store whose lanes live under `cacheRoot`: the shared build and target directories themselves, or the build directory of one of the store's lanes with a target directory that is the lane's own or lies inside its build directory (the recipe slot of an artifact build). Pure; a lookalike directory, a path with `..` or a lane of another cache root is never owned. */
export function compilerOriginOwnedV1(input:Readonly<{cacheRoot:string;shared:Readonly<{build:string;target:string}>;origin:Readonly<{buildDirectory:string;targetDirectory:string}>}>):boolean{
 const build=resolve(input.origin.buildDirectory),target=resolve(input.origin.targetDirectory);
 if(build===resolve(input.shared.build)&&target===resolve(input.shared.target))return true;
 if(!laneBuildDirectoryOwnedV1(input.cacheRoot,build))return false;
 return target===resolve(join(dirname(build),layout.target))||insideV1(build,target);
}

function digestV1(path:string):Readonly<{sha256:string;bytes:number}>{
 const hash=createHash("sha256"),chunk=Buffer.allocUnsafe(rules.chunkBytes),descriptor=openSync(path,"r");let bytes=0;
 try{for(let read=readSync(descriptor,chunk,0,chunk.length,null);read>0;read=readSync(descriptor,chunk,0,chunk.length,null)){hash.update(chunk.subarray(0,read));bytes+=read;}}finally{closeSync(descriptor);}
 return{sha256:hash.digest("hex"),bytes};
}

function regularNamesV1(directory:string):string[]{
 try{return readdirSync(directory,{withFileTypes:true}).filter(entry=>entry.isFile()&&pattern.test(entry.name)).map(entry=>entry.name).sort();}catch{return[];}
}

function entriesV1(directory:string,names:readonly string[]):RetainedEntryV1[]{
 const rows:RetainedEntryV1[]=[];
 for(const name of names){
  const path=join(directory,name);let size:number;
  try{const facts=lstatSync(path);if(!facts.isFile())continue;size=facts.size;}catch{continue;}
  rows.push(size>rules.maximumFileBytes?{name,sha256:"",bytes:size}:{name,...digestV1(path)});
 }
 return rows;
}

/** 🔎️ What a name that is already taken means: the same receipt (`same`) or anything else, a file with other bytes or an entry that cannot be read (`conflict`). */
function takenV1(destination:string,sha256:string):"same"|"conflict"{
 try{return digestV1(destination).sha256===sha256?"same":"conflict";}catch{return"conflict";}
}

/** 🪢️ The publication on a file system without hard links: the destination name is created exclusively from the verified temporary, so a name taken by a peer at any moment (whatever its bytes, even a link that leads nowhere) is never replaced, and the copy is read back; a copy that does not read back as the receipt is removed again, so no reader keeps a damaged receipt under a receipt name. */
export function publishWithoutLinkV1(temporary:string,destination:string,sha256:string):"published"|"same"|"conflict"{
 try{copyFileSync(temporary,destination,constants.COPYFILE_EXCL);}
 catch(error){if((error as NodeJS.ErrnoException).code==="EEXIST")return takenV1(destination,sha256);throw error;}
 if(digestV1(destination).sha256===sha256)return"published";
 rmSync(destination,{force:true});throw refuseCachingV1("stale","lane.provenance-changed",{file:basename(destination)});
}

function publishV1(source:string,destination:string,sha256:string):"published"|"same"|"conflict"{
 const temporary=join(dirname(destination),"."+basename(destination)+"."+randomBytes(4).toString("hex")+".tmp");
 try{
  copyFileSync(source,temporary,constants.COPYFILE_EXCL);
  if(digestV1(temporary).sha256!==sha256)throw refuseCachingV1("stale","lane.provenance-changed",{file:basename(source)});
  const descriptor=openSync(temporary,"r+");try{fsyncSync(descriptor);}finally{closeSync(descriptor);}
  try{linkSync(temporary,destination);return"published";}
  catch(error){
   const code=(error as NodeJS.ErrnoException).code;
   if(code==="EEXIST")return takenV1(destination,sha256);
   if(!["EPERM","ENOTSUP","EXDEV","ENOSYS","EACCES"].includes(code??""))throw error;
   return publishWithoutLinkV1(temporary,destination,sha256);
  }
 }finally{rmSync(temporary,{force:true});}
}

/** 📦️ The retention rule: before a lane is released (and so becomes evictable) its provenance ledger copies are published, by content, into the shared build directory of the store, which is where the readers of the ledger look. A file is copied beside its destination, verified against its digest, linked into place without ever replacing an existing name, and only then removed from the lane, so an interrupted pass loses nothing and the next release of the same lane finishes it. A name that exists with other bytes is reported (EN/DE) and stays in the lane. Diagnostics logs are not retained: they stay in the lane, where the message that names them points.
 * @see ../../../../../💾️storage/🔒️enforcement/🏗️compiler/🛤️lane/🟦️.ts */
export function retainLaneOutputsV1(input:Readonly<{lane:string;shared:string;laneId:string;language:"en"|"de";report?:(line:string)=>void}>):RetentionSummaryV1{
 let published=0,same=0,refused=0,bytes=0;const conflicts:string[]=[];
 const from=join(input.lane,rules.ledger),to=join(input.shared,rules.ledger),names=regularNamesV1(from);
 if(names.length){
  const lane=entriesV1(from,names),present=regularNamesV1(to).filter(name=>names.includes(name)),shared=entriesV1(to,present),actions=planRetentionV1(lane,shared),digests=new Map(lane.map(entry=>[entry.name,entry.sha256]));
  mkdirSync(to,{recursive:true});
  for(const action of actions){
   if(action.action==="refuse"){refused++;continue;}
   const outcome=action.action==="same"?"same":action.action==="conflict"?"conflict":publishV1(join(from,action.name),join(to,action.name),digests.get(action.name)!);
   if(outcome==="conflict"){conflicts.push(action.name);continue;}
   if(outcome==="published"){published++;bytes+=action.bytes;}else same++;
   rmSync(join(from,action.name),{force:true});
  }
 }
 if(conflicts.length)(input.report??(()=>undefined))(laneMessageV1("retentionConflict",{lane:input.laneId,count:conflicts.length,names:conflicts.slice(0,4).join(", ")},input.language));
 return Object.freeze({published,same,conflicts:Object.freeze(conflicts),refused,bytes});
}

/** 📝️ Records beside a lane which shared ledger receives its receipts, so whoever removes the lane while its holder is gone knows where they belong. Best effort: a lane without the record retains nothing when it is removed. */
function recordDestinationV1(base:string,shared:string):void{
 try{const path=join(base,layout.destination),temporary=path+"."+randomBytes(3).toString("hex");writeFileSync(temporary,shared);renameSync(temporary,path);}catch{}
}

/** 🧹️ The retention of a lane that a trimmer removes while its holder is gone (killed between its body and its own retention): the receipts the lane still holds are published into the shared ledger the holder recorded before the lane goes, so the removal cannot lose the provenance of a build that finished. The caller holds the lane's claim and has seen it idle. A lane without a record, a record that is not an absolute path and a failing retention retain nothing and never block the removal (the pool bound is the hard limit; a killed build is rebuilt by its next run); the names that conflict with the shared ledger are returned.
 * @see ../🏊️pool/🟦️.ts */
export function retainRemovedLaneV1(paths:Readonly<{id:string;base:string;build:string}>):readonly string[]{
 let shared:string;try{shared=readFileSync(join(paths.base,layout.destination),"utf8");}catch{return[];}
 if(!isAbsolute(shared))return[];
 try{return retainLaneOutputsV1({lane:paths.build,shared,laneId:paths.id,language:"en"}).conflicts;}catch{return[];}
}

/** 🔗️ Runs `body` in a held lane, then publishes the lane's provenance (whether `body` succeeded or failed) and only then releases the lane, so no pass can trim a lane whose provenance is not yet in the shared ledger. A failing `body` keeps its own error (a failing retention joins it); after a successful `body` a failing retention is the error, because the caller would otherwise lose the provenance of a build that looked successful.
 * @see ./🟦️.ts */
export async function withLaneRetentionV1<T>(input:Readonly<{lane:Readonly<{paths:Readonly<{id:string;build:string}>;release():void}>;shared:string;language:"en"|"de";report?:(line:string)=>void}>,body:()=>Promise<T>):Promise<T> {
 let value!:T,bodyFailed=false,bodyFailure:unknown,retentionFailed=false,retentionFailure:unknown;
 recordDestinationV1(dirname(input.lane.paths.build),input.shared);
 try{value=await body();}catch(error){bodyFailed=true;bodyFailure=error;}
 try{retainLaneOutputsV1({lane:input.lane.paths.build,shared:input.shared,laneId:input.lane.paths.id,language:input.language,...input.report?{report:input.report}:{}});}
 catch(error){retentionFailed=true;retentionFailure=error;}
 finally{input.lane.release();}
 if(bodyFailed&&retentionFailed)throw new AggregateError([bodyFailure,retentionFailure],"Lane body and provenance retention failed");
 if(bodyFailed)throw bodyFailure;
 if(retentionFailed)throw retentionFailure;
 return value;
}
