import{repositoryCompilerLayoutV1}from"../../../../💾️storage/🔒️enforcement/🏗️compiler/📍️layout/🟦️.ts";
import targetSchema from "../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🎯️target/🧬️schema/🔣️.json" with {type:"json"};
import {createHash,randomBytes} from "node:crypto";
import {closeSync,constants,copyFileSync,existsSync,fstatSync,fsyncSync,linkSync,lstatSync,mkdirSync,openSync,opendirSync,readFileSync,readSync,readdirSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
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
 if(parts.length!==1)return false;
 try{return resolve(lanePathsV1(root,parts[0]!).build)===build;}catch{return false;}
}

/** 🪪️ Whether a recorded compiler origin belongs to the compiler store whose lanes live under `cacheRoot`: the shared build and target directories themselves, or the build root of one of the store's lanes with a target directory in that lane's target namespace (the recipe slot of an artifact build). Pure; a lookalike directory, a path with `..` or a lane of another cache root is never owned. */
export function compilerOriginOwnedV1(input:Readonly<{cacheRoot:string;shared:Readonly<{build:string;target:string}>;origin:Readonly<{buildDirectory:string;targetDirectory:string}>}>):boolean{
 const build=resolve(input.origin.buildDirectory),target=resolve(input.origin.targetDirectory);
 if(build===resolve(input.shared.build)&&target===resolve(input.shared.target))return true;
 if(!laneBuildDirectoryOwnedV1(input.cacheRoot,build))return false;
 const id=relative(join(resolve(input.cacheRoot),layout.lanes),build);const expected=resolve(lanePathsV1(input.cacheRoot,id).target);return target===expected||insideV1(expected,target);
}

function digestV1(path:string):Readonly<{sha256:string;bytes:number}>{
 const before=lstatSync(path);if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1)throw Error("Physical receipt required");const descriptor=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0)),hash=createHash("sha256"),chunk=Buffer.allocUnsafe(rules.chunkBytes);let bytes=0;
 try{const opened=fstatSync(descriptor);if(opened.dev!==before.dev||opened.ino!==before.ino||opened.size!==before.size||opened.nlink!==1)throw Error("Changed receipt");for(let read=readSync(descriptor,chunk,0,chunk.length,null);read>0;read=readSync(descriptor,chunk,0,chunk.length,null)){hash.update(chunk.subarray(0,read));bytes+=read;}const after=fstatSync(descriptor),named=lstatSync(path);if(after.dev!==opened.dev||after.ino!==opened.ino||after.size!==opened.size||after.mtimeMs!==opened.mtimeMs||after.ctimeMs!==opened.ctimeMs||named.dev!==opened.dev||named.ino!==opened.ino||named.isSymbolicLink()||named.nlink!==1||bytes!==opened.size)throw Error("Changed receipt");}finally{closeSync(descriptor)}return{sha256:hash.digest("hex"),bytes};
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
  const identities=new Map(names.map(name=>{const stat=lstatSync(join(from,name));return[name,{dev:stat.dev,ino:stat.ino,size:stat.size,mtime:stat.mtimeMs,ctime:stat.ctimeMs}] as const;})),lane=entriesV1(from,names),present=regularNamesV1(to).filter(name=>names.includes(name)),shared=entriesV1(to,present),actions=planRetentionV1(lane,shared),digests=new Map(lane.map(entry=>[entry.name,entry.sha256]));
  mkdirSync(to,{recursive:true});
  for(const action of actions){
   if(action.action==="refuse"){refused++;continue;}
   const outcome=action.action==="same"?"same":action.action==="conflict"?"conflict":publishV1(join(from,action.name),join(to,action.name),digests.get(action.name)!);
   if(outcome==="conflict"){conflicts.push(action.name);continue;}
   if(outcome==="published"){published++;bytes+=action.bytes;}else same++;
   const path=join(from,action.name),before=identities.get(action.name)!,now=lstatSync(path),verified=digestV1(path);if(now.dev!==before.dev||now.ino!==before.ino||now.size!==before.size||now.mtimeMs!==before.mtime||now.ctimeMs!==before.ctime||verified.sha256!==digests.get(action.name))throw Error("Changed lane receipt before retirement");rmSync(path,{force:true});
  }
 }
 if(conflicts.length)(input.report??(()=>undefined))(laneMessageV1("retentionConflict",{lane:input.laneId,count:conflicts.length,names:conflicts.slice(0,4).join(", ")},input.language));
 return Object.freeze({published,same,conflicts:Object.freeze(conflicts),refused,bytes});
}

/** 📝️ Records beside a lane which shared ledger receives its receipts, so whoever removes the lane while its holder is gone knows where they belong. A failed record prevents the body from starting. */
function recordDestinationV1(base:string,shared:string):void{
 const path=join(base,layout.destination),temporary=path+"."+randomBytes(3).toString("hex");try{writeFileSync(temporary,JSON.stringify({version:1,targetDirectory:canonicalPhysicalV1(base),sharedBuild:shared})+"\n",{flag:"wx"});renameSync(temporary,path);}finally{rmSync(temporary,{force:true});}
}

type TargetEvidenceV1=Readonly<{path:string;target:string;build:string;digest:string;dev:number;ino:number;size:number;mtime:number;ctime:number}>;
const canonicalPhysicalV1=(value:string):string=>realpathSync(value).replaceAll("\\","/").normalize("NFC");
/** 🔍️ Discovers bounded physical target bindings and rejects unresolved or foreign custody. */
function discoverLaneEvidenceV1(base:string):Readonly<{bindings:readonly TargetEvidenceV1[];ledgers:readonly string[]}>{const root=canonicalPhysicalV1(base),pending=[base],bindings:TargetEvidenceV1[]=[],ledgers:string[]=[];let directories=0,entries=0;while(pending.length){const dir=pending.pop()!;if(++directories>rules.maximumFiles||dir.length>256)throw Error("Bounded lane evidence required");const handle=opendirSync(dir);try{for(let entry=handle.readSync();entry!==null;entry=handle.readSync()){if(++entries>rules.scanMaximumEntries)throw Error("Bounded lane entry scan required");const child=join(dir,entry.name);if(entry.name===rules.ledger){if(!entry.isDirectory()||entry.isSymbolicLink())throw Error("Physical lane ledger required");ledgers.push(dir);}if(entry.name===targetSchema.definitions.Protocol.const.receipt){if(!entry.isFile()||entry.isSymbolicLink())throw Error("Physical target binding required");const before=lstatSync(child);if(before.nlink!==1||before.size>targetSchema.definitions.Protocol.const.maximumReceiptBytes)throw Error("Bounded target binding required");const fd=openSync(child,constants.O_RDONLY|(constants.O_NOFOLLOW??0));let text:string;try{const opened=fstatSync(fd);if(opened.dev!==before.dev||opened.ino!==before.ino||opened.size!==before.size||opened.nlink!==1)throw Error("Changed target binding");text=readFileSync(fd,"utf8");const after=fstatSync(fd);if(after.dev!==opened.dev||after.ino!==opened.ino||after.size!==opened.size||after.mtimeMs!==opened.mtimeMs||after.ctimeMs!==opened.ctimeMs)throw Error("Changed target binding");}finally{closeSync(fd)}const row=JSON.parse(text);if(!row||row.version!==1||typeof row.buildDirectory!=="string"||typeof row.targetDirectory!=="string"||text!==JSON.stringify({version:1,buildDirectory:row.buildDirectory,targetDirectory:row.targetDirectory})+"\n")throw Error("Canonical target binding required");const build=canonicalPhysicalV1(row.buildDirectory),target=canonicalPhysicalV1(dir);if(build!==row.buildDirectory||target!==row.targetDirectory||!(build===root||insideV1(root,build))||!insideV1(root,target))throw Error("In-lane physical target binding required");bindings.push({path:child,target,build,digest:createHash("sha256").update(text).digest("hex"),dev:before.dev,ino:before.ino,size:before.size,mtime:before.mtimeMs,ctime:before.ctimeMs});}if(entry.isDirectory()&&!entry.isSymbolicLink())pending.push(child);}}finally{handle.closeSync()}}return{bindings,ledgers};}
/** 🛡️ Checks every matching ledger entry before publishing so unresolved custody cannot be trimmed. */
function requireLedgerV1(build:string):boolean{const dir=join(build,rules.ledger);if(!existsSync(dir))return false;const stat=lstatSync(dir);if(!stat.isDirectory()||stat.isSymbolicLink())throw Error("Physical lane ledger required");const rows=readdirSync(dir,{withFileTypes:true}).filter(row=>pattern.test(row.name));if(rows.length>rules.maximumFiles)throw Error("Bounded lane ledger required");for(const row of rows){const stat=lstatSync(join(dir,row.name));if(!row.isFile()||row.isSymbolicLink()||stat.nlink!==1||stat.size>rules.maximumFileBytes)throw Error("Unresolved lane receipt required");}return rows.length>0;}
/** 🧭️ Reads the exact current target destination under its physical compiler store. */
function readTargetMarkerV1(target:string):string{const marker=join(target,layout.destination),stat=lstatSync(marker);if(!stat.isFile()||stat.isSymbolicLink()||stat.nlink!==1||stat.size>targetSchema.definitions.Protocol.const.maximumReceiptBytes)throw Error("Physical target destination required");const fd=openSync(marker,constants.O_RDONLY|(constants.O_NOFOLLOW??0));let destination:string;try{const current=fstatSync(fd);if(current.dev!==stat.dev||current.ino!==stat.ino||current.size!==stat.size||current.nlink!==1)throw Error("Changed target destination");destination=readFileSync(fd,"utf8");const after=fstatSync(fd);if(after.dev!==current.dev||after.ino!==current.ino||after.size!==current.size||after.mtimeMs!==current.mtimeMs||after.ctimeMs!==current.ctimeMs)throw Error("Changed target destination");const named=lstatSync(marker);if(!named.isFile()||named.isSymbolicLink()||named.nlink!==1||named.dev!==current.dev||named.ino!==current.ino||named.size!==current.size||named.mtimeMs!==current.mtimeMs||named.ctimeMs!==current.ctimeMs)throw Error("Changed target destination");}finally{closeSync(fd)}const row=JSON.parse(destination);if(!row||row.version!==1||typeof row.targetDirectory!=="string"||typeof row.sharedBuild!=="string"||row.targetDirectory!==canonicalPhysicalV1(target)||destination!==JSON.stringify({version:1,targetDirectory:row.targetDirectory,sharedBuild:row.sharedBuild})+"\n")throw Error("Canonical target destination required");return row.sharedBuild;}
function validateTargetDestinationV1(base:string,destination:string):string{if(!isAbsolute(destination))throw Error("Absolute target destination required");const store=canonicalPhysicalV1(dirname(dirname(dirname(base)))),permitted=["prototype","release"].map(mode=>resolve(repositoryCompilerLayoutV1(store,mode).buildDirectory));if(!permitted.includes(resolve(destination)))throw Error("Canonical shared compiler destination required");mkdirSync(destination,{recursive:true});const shared=canonicalPhysicalV1(destination);if(!permitted.includes(resolve(shared)))throw Error("Physical shared compiler destination required");return shared;}
function targetDestinationV1(base:string,target:string):string{return validateTargetDestinationV1(base,readTargetMarkerV1(target));}
/** 🧹️ Publishes authenticated target bindings and each exactly attributed ledger before a lane can be removed. */
function captureTargetDestinationV1(base:string,target:string){const original=readTargetMarkerV1(target),shared=validateTargetDestinationV1(base,original),marker=join(target,layout.destination),stat=lstatSync(marker),digest=createHash("sha256").update(JSON.stringify({version:1,targetDirectory:canonicalPhysicalV1(target),sharedBuild:original})+"\n").digest("hex");if(digestV1(marker).sha256!==digest||targetDestinationV1(base,target)!==shared)throw Error("Changed target destination");return{target,shared,marker,stat,digest};}
function verifyTargetDestinationV1(base:string,evidence:ReturnType<typeof captureTargetDestinationV1>):void{const stat=lstatSync(evidence.marker);if(stat.dev!==evidence.stat.dev||stat.ino!==evidence.stat.ino||stat.size!==evidence.stat.size||stat.mtimeMs!==evidence.stat.mtimeMs||stat.ctimeMs!==evidence.stat.ctimeMs||digestV1(evidence.marker).sha256!==evidence.digest||targetDestinationV1(base,evidence.target)!==evidence.shared)throw Error("Changed target destination");}
export function retainRemovedLaneV1(paths:Readonly<{id:string;base:string;build:string}>):readonly string[]{try{const evidence=discoverLaneEvidenceV1(paths.base),builds=new Map<string,Set<string>>(),markers=new Map<string,ReturnType<typeof captureTargetDestinationV1>>();for(const binding of evidence.bindings){const marker=captureTargetDestinationV1(paths.base,binding.target);markers.set(binding.target,marker);const known=builds.get(binding.build)??new Set<string>();known.add(marker.shared);builds.set(binding.build,known);}for(const dir of evidence.ledgers)if(!builds.has(canonicalPhysicalV1(dir)))throw Error("Unbound lane ledger");for(const [build,destinations]of builds)if(destinations.size!==1&&requireLedgerV1(build))throw Error("Ambiguous lane ledger destination");for(const binding of evidence.bindings){const marker=markers.get(binding.target)!;verifyTargetDestinationV1(paths.base,marker);const ledger=join(marker.shared,rules.ledger);mkdirSync(ledger,{recursive:true});const outcome=publishV1(binding.path,join(ledger,rules.bindingPrefix+binding.digest.slice(0,32)+".json"),binding.digest);if(outcome==="conflict")throw Error("Target binding evidence conflict");const markerOutcome=publishV1(marker.marker,join(ledger,rules.destinationPrefix+marker.digest.slice(0,32)+".json"),marker.digest);if(markerOutcome==="conflict")throw Error("Target destination evidence conflict");verifyTargetDestinationV1(paths.base,marker);const now=lstatSync(binding.path),verified=digestV1(binding.path);if(now.dev!==binding.dev||now.ino!==binding.ino||now.size!==binding.size||now.mtimeMs!==binding.mtime||now.ctimeMs!==binding.ctime||verified.sha256!==binding.digest)throw Error("Changed target binding before retirement");}const conflicts:string[]=[];for(const [build,destinations]of builds){for(const marker of markers.values())verifyTargetDestinationV1(paths.base,marker);if(!requireLedgerV1(build))continue;const result=retainLaneOutputsV1({lane:build,shared:[...destinations][0]!,laneId:paths.id,language:"en"});conflicts.push(...result.conflicts);if(result.conflicts.length)return conflicts;if(result.refused)throw Error("Lane receipt publication refused");if(requireLedgerV1(build))throw Error("Lane receipt publication incomplete");for(const marker of markers.values())verifyTargetDestinationV1(paths.base,marker);}return conflicts;}catch{return["unresolved lane custody"]}}

/** 🔗️ Runs `body` in a held lane, then publishes the lane's provenance (whether `body` succeeded or failed) and only then releases the lane, so no pass can trim a lane whose provenance is not yet in the shared ledger. A failing `body` keeps its own error (a failing retention joins it); after a successful `body` a failing retention is the error, because the caller would otherwise lose the provenance of a build that looked successful.
 * @see ./🟦️.ts */
export async function withLaneRetentionV1<T>(input:Readonly<{lane:Readonly<{paths:Readonly<{id:string;base:string;build:string;target:string}>;release():void}>;shared:string;language:"en"|"de";report?:(line:string)=>void}>,body:()=>Promise<T>):Promise<T> {
 let value!:T,bodyFailed=false,bodyFailure:unknown,retentionFailed=false,retentionFailure:unknown,admitted=false;
 try{const marker=join(input.lane.paths.target,layout.destination);if(existsSync(marker)){const previous=readTargetMarkerV1(input.lane.paths.target);if(previous!==input.shared){const unresolved=retainRemovedLaneV1(input.lane.paths);if(unresolved.length)throw Error("Previous lane custody is unresolved");}}else if(requireLedgerV1(input.lane.paths.build))throw Error("Lane ledger lacks its original target destination");recordDestinationV1(input.lane.paths.target,input.shared);targetDestinationV1(input.lane.paths.base,input.lane.paths.target);admitted=true;value=await body();}catch(error){bodyFailed=true;bodyFailure=error;}
 try{if(admitted){const result=retainLaneOutputsV1({lane:input.lane.paths.build,shared:input.shared,laneId:input.lane.paths.id,language:input.language,...input.report?{report:input.report}:{}});if(result.conflicts.length||result.refused)throw Error("Lane receipt custody remains unresolved");}} 
 catch(error){retentionFailed=true;retentionFailure=error;}
 finally{input.lane.release();}
 if(bodyFailed&&retentionFailed)throw new AggregateError([bodyFailure,retentionFailure],"Lane body and provenance retention failed");
 if(bodyFailed)throw bodyFailure;
 if(retentionFailed)throw retentionFailure;
 return value;
}
