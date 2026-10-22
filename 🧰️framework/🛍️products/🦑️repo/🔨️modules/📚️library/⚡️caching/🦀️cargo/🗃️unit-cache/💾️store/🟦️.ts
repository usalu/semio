import {randomBytes} from "node:crypto";
import {appendFileSync,chmodSync,constants,copyFileSync,existsSync,lstatSync,mkdirSync,readFileSync,readdirSync,readlinkSync,renameSync,rmSync,statSync,utimesSync,writeFileSync} from "node:fs";
import {basename,dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {laneNeutralV1,laneRestoreV1,parseDepInfoV1,sha256V1,UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=UNIT_CACHE_POLICY_V1,layout=policy.layout;
type Definitions=typeof schema.definitions;
/** 📦️ One cached compilation: the files it read, the environment it observed, the files it wrote and the compiler's own messages. */
export type UnitEntryV1={version:1;key:string;crate:string;dependencies:{path:string;sha256:string|null}[];environment:{name:string;value:string|null}[];outputs:{name:string;sha256:string;bytes:number;executable:boolean;template?:boolean}[];stderr:string|null;stdout:string|null;createdMs:number;provenance?:{peakRssBytes:number;userMs:number;sysMs:number;wallMs:number;rmetaOffsetMs:number|null}|null};
/** 🧾️ One measured cache decision (`Event` in the schema). */
export type UnitEventV1={version:1;atMs:number;lane:string;pid:number;kind:Definitions["Event"]["properties"]["kind"]["enum"][number];crate:string;key:string|null;milliseconds:number;waitedMs:number;reason?:string};

/** 🗄️ Where one cache root keeps each kind of state. */
export function storePathsV1(root:string){return Object.freeze({root,objects:join(root,layout.objects),entries:join(root,layout.entries),locks:join(root,layout.locks),events:join(root,layout.events),temporary:join(root,layout.temporary)});}

function ensure(path:string):string{mkdirSync(path,{recursive:true});return path;}
const objectPath=(root:string,sha:string)=>join(root,layout.objects,sha.slice(0,2),sha);

/** 🕳️ Whether a recorded entry lost an object it replays: an output or, for a build script run, one of its two streams. */
const lostObjectV1=(root:string,entry:Readonly<{outputs:readonly{sha256:string}[];stdout?:string|null;stderr?:string|null}>):boolean=>[...entry.outputs.map(row=>row.sha256),entry.stdout,entry.stderr].some(sha=>typeof sha==="string"&&!existsSync(objectPath(root,sha)));
const entryDirectory=(root:string,key:string)=>join(root,layout.entries,key.slice(0,2),key);

/** 🔏️ Content digest of a file, memoised per lane by (size, mtime, inode) so unchanged dependency artifacts are hashed once. */
export function digestFileV1(path:string,stampDirectory:string|undefined):string|null{
 let stat;try{stat=statSync(path,{bigint:true});}catch{return null;}
 if(!stat.isFile())return null;
 const stamp=stampDirectory?join(stampDirectory,sha256V1(path).slice(0,2),sha256V1(path).slice(2,34)):undefined,mark=`${stat.size}:${stat.mtimeNs}:${stat.ino}`;
 if(stamp){try{const line=readFileSync(stamp,"utf8"),at=line.indexOf(" ");if(at>0&&line.slice(0,at)===mark)return line.slice(at+1);}catch{}}
 const digest=sha256V1(readFileSync(path));
 if(stamp){try{ensure(dirname(stamp));const temporary=stamp+"."+randomBytes(3).toString("hex");writeFileSync(temporary,mark+" "+digest);rmSync(stamp,{force:true});renameSync(temporary,stamp);}catch{}}
 return digest;
}

/** 📤️ Appends one decision to the lane's event log; one small append is atomic across processes. */
export function emitUnitEventV1(root:string,event:Omit<UnitEventV1,"version"|"atMs"|"pid">):void{
 try{const directory=ensure(join(root,layout.events)),file=join(directory,event.lane+".jsonl");appendFileSync(file,JSON.stringify({version:1,atMs:Date.now(),pid:process.pid,...event})+"\n");}catch{}
}

function putObject(root:string,bytes:Uint8Array|string):string{
 const sha=sha256V1(bytes),target=objectPath(root,sha);
 if(existsSync(target)){try{const now=new Date();utimesSync(target,now,now);}catch{}return sha;}
 const temporary=join(ensure(join(root,layout.temporary)),randomBytes(8).toString("hex"));
 writeFileSync(temporary,bytes);ensure(dirname(target));try{renameSync(temporary,target);}catch(error){rmSync(temporary,{force:true});if(!existsSync(target))throw error;}return sha;
}

function readObject(root:string,sha:string):Buffer|undefined{try{return readFileSync(objectPath(root,sha));}catch{return undefined;}}

/** 🔎️ Returns the newest entry of `key` whose recorded files and environment still hold, `undefined` on a miss. A missing object or an unreadable entry is a miss, never an error. */
export function lookupUnitV1(root:string,key:string,context:Readonly<{cwd:string;laneRoot:string|undefined;environment:Readonly<Record<string,string|undefined>>;stampDirectory:string|undefined}>):UnitEntryV1|undefined{
 const directory=entryDirectory(root,key);let names:string[];
 try{names=readdirSync(directory).filter(name=>name.endsWith(".json")).sort().reverse();}catch{return undefined;}
 for(const name of names.slice(0,policy.key.maximumVariants)){
  let entry:UnitEntryV1;try{entry=JSON.parse(readFileSync(join(directory,name),"utf8"));}catch{continue;}
  if(entry.version!==1||entry.key!==key)continue;
  if(entry.dependencies.some(row=>digestFileV1(resolveStored(context.cwd,laneRestoreV1(row.path,context.laneRoot)),context.stampDirectory)!==row.sha256))continue;
  if(entry.environment.some(row=>{const value=context.environment[row.name];return(value===undefined?null:laneNeutralV1(value,context.laneRoot))!==row.value;}))continue;
  if(lostObjectV1(root,entry))continue;
  return entry;
 }
 return undefined;
}

function resolveStored(cwd:string,path:string):string{return isAbsolute(path)?path:resolve(cwd,path);}

/** ♻️ Writes a cached compilation into `outDir` as the compiler would have: output files by reflink where the filesystem supports it (a copy elsewhere, never a shared inode), the dependency file rewritten for this lane, and the compiler's messages returned for replay. */
export function restoreUnitV1(root:string,entry:UnitEntryV1,context:Readonly<{outDir:string;laneRoot:string|undefined;stampDirectory:string|undefined}>):{stdout:string;stderr:string}{
 const render=(text:string)=>laneRestoreV1(text.split(policy.key.outputToken).join(context.outDir),context.laneRoot);
 ensure(context.outDir);
 for(const row of entry.outputs){
  const destination=join(context.outDir,row.name);rmSync(destination,{force:true});
  const source=objectPath(root,row.sha256);
  if(row.template){writeFileSync(destination,render(readFileSync(source,"utf8")));}
  else{copyFileSync(source,destination,constants.COPYFILE_FICLONE);if(row.executable)chmodSync(destination,0o755);}
  try{const now=new Date();utimesSync(source,now,now);}catch{}
  if(!row.template&&context.stampDirectory)stampFile(destination,row.sha256,context.stampDirectory);
 }
 const text=(sha:string|null)=>sha===null?"":render(readObject(root,sha)?.toString("utf8")??"");
 return{stdout:text(entry.stdout),stderr:text(entry.stderr)};
}

function stampFile(path:string,sha:string,stampDirectory:string):void{
 try{const stat=statSync(path,{bigint:true}),hash=sha256V1(path),stamp=join(stampDirectory,hash.slice(0,2),hash.slice(2,34));ensure(dirname(stamp));const temporary=stamp+"."+randomBytes(3).toString("hex");writeFileSync(temporary,`${stat.size}:${stat.mtimeNs}:${stat.ino} ${sha}`);rmSync(stamp,{force:true});renameSync(temporary,stamp);}catch{}
}

/** 📥️ Publishes one successful compilation. Objects are content-addressed and the entry is renamed into place last, so a reader sees all of it or none of it. Returns the digests, `undefined` when the dependency file is missing or the outputs exceed the bound. */
export function publishUnitV1(root:string,input:Readonly<{key:string;crate:string;cwd:string;outDir:string;laneRoot:string|undefined;outputs:readonly string[];stdout:string;stderr:string;stampDirectory:string|undefined;provenance?:UnitEntryV1["provenance"]}>):UnitEntryV1|undefined{
 const depInfoPath=input.outputs.find(path=>path.endsWith(".d"));if(!depInfoPath||!existsSync(depInfoPath))return undefined;
 const info=parseDepInfoV1(readFileSync(depInfoPath,"utf8"));
 if(info.dependencies.length>policy.key.maximumDependencyFiles)return undefined;
 const neutral=(text:string)=>laneNeutralV1(text.split(input.outDir).join(policy.key.outputToken),input.laneRoot);
 let total=0;const outputs:UnitEntryV1["outputs"]=[];
 for(const path of input.outputs){
  const stat=statSync(path);total+=stat.size;if(total>policy.key.maximumOutputBytes)return undefined;
  const template=path===depInfoPath,bytes=template?Buffer.from(neutral(readFileSync(path,"utf8"))):readFileSync(path),sha=putObject(root,bytes);
  outputs.push({name:basename(path),sha256:sha,bytes:bytes.length,executable:(stat.mode&0o111)!==0&&!template,...(template?{template:true}:{})});
  if(!template&&input.stampDirectory)stampFile(path,sha,input.stampDirectory);
 }
 const dependencies=info.dependencies.map(path=>({path:laneNeutralV1(path,input.laneRoot),sha256:digestFileV1(resolveStored(input.cwd,path),input.stampDirectory)}));
 const entry:UnitEntryV1={version:1,key:input.key,crate:input.crate,dependencies,environment:info.environment.map(row=>({name:row.name,value:row.value===null?null:laneNeutralV1(row.value,input.laneRoot)})),outputs,stderr:input.stderr?putObject(root,neutral(input.stderr)):null,stdout:input.stdout?putObject(root,neutral(input.stdout)):null,createdMs:Date.now(),provenance:input.provenance??null};
 const directory=ensure(entryDirectory(root,input.key)),temporary=join(ensure(join(root,layout.temporary)),randomBytes(8).toString("hex")),name=String(entry.createdMs).padStart(15,"0")+"-"+randomBytes(3).toString("hex")+".json";
 writeFileSync(temporary,JSON.stringify(entry));renameSync(temporary,join(directory,name));
 try{const names=readdirSync(directory).filter(file=>file.endsWith(".json")).sort();for(const old of names.slice(0,Math.max(0,names.length-policy.key.maximumVariants)))rmSync(join(directory,old),{force:true});}catch{}
 return entry;
}

/** 🔭️ Lists the cached objects with their sizes and last use, for garbage collection. */
export function listObjectsV1(root:string):{digest:string;bytes:number;lastUsedMs:number}[]{
 const rows:{digest:string;bytes:number;lastUsedMs:number}[]=[],base=join(root,layout.objects);
 let shards:string[];try{shards=readdirSync(base);}catch{return rows;}
 for(const shard of shards)for(const name of readdirSync(join(base,shard))){try{const stat=statSync(join(base,shard,name));rows.push({digest:name,bytes:stat.size,lastUsedMs:Math.floor(stat.mtimeMs)});}catch{}}
 return rows;
}

/** 🗑️ Removes objects by digest; entries that lose an object become misses and are dropped lazily. */
export function removeObjectsV1(root:string,digests:readonly string[]):number{let removed=0;for(const digest of digests){if(!/^[a-f0-9]{64}$/u.test(digest))continue;try{const path=objectPath(root,digest);if(existsSync(path)){rmSync(path,{force:true});removed++;}}catch{}}return removed;}

/** 🧽️ Drops entries that lost an object, empty entry directories, and temporary files of writers that died an hour ago. */
export function pruneEntriesV1(root:string,nowMs:number):{entries:number;temporary:number}{
 let entries=0,temporary=0;
 for(const kind of[layout.entries,layout.runs]){
  const base=join(root,kind);let shards:string[];try{shards=readdirSync(base);}catch{shards=[];}
  for(const shard of shards)for(const key of readdirSync(join(base,shard))){
   const directory=join(base,shard,key);let files:string[];try{files=readdirSync(directory);}catch{continue;}
   for(const file of files){try{const entry=JSON.parse(readFileSync(join(directory,file),"utf8")) as {outputs:{sha256:string}[];stdout?:string|null;stderr?:string|null};if(lostObjectV1(root,entry)){rmSync(join(directory,file),{force:true});entries++;}}catch{rmSync(join(directory,file),{force:true});entries++;}}
   try{if(readdirSync(directory).length===0)rmSync(directory,{recursive:true,force:true});}catch{}
  }
 }
 const work=join(root,layout.temporary);let names:string[];try{names=readdirSync(work);}catch{names=[];}
 for(const name of names){try{if(nowMs-statSync(join(work,name)).mtimeMs>3600000){rmSync(join(work,name),{force:true});temporary++;}}catch{}}
 return{entries,temporary};
}

/** 🌳️ Digest of a directory tree for build-script facts: sorted relative paths with file digests (links by target text). `undefined` beyond the file bound. */
export function treeDigestV1(root:string,options:Readonly<{ignoredDirectories:readonly string[];ignoredExtensions:readonly string[];maximumFiles:number;stampDirectory:string|undefined}>):string|undefined{
 const rows:string[][]=[];let overflow=false;
 const walk=(directory:string):void=>{
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  for(const entry of entries.sort((left,right)=>left.name<right.name?-1:1)){
   if(overflow)return;
   const path=join(directory,entry.name),name=relative(root,path).split(sep).join("/");
   if(entry.isDirectory()){if(!options.ignoredDirectories.includes(entry.name))walk(path);continue;}
   if(options.ignoredExtensions.some(extension=>entry.name.endsWith(extension)))continue;
   if(rows.length>=options.maximumFiles){overflow=true;return;}
   if(entry.isSymbolicLink()){try{rows.push([name,"link:"+readlinkSync(path)]);}catch{rows.push([name,"link:"]);}continue;}
   if(entry.isFile())rows.push([name,digestFileV1(path,options.stampDirectory)??"unreadable"]);
  }
 };
 walk(root);
 return overflow?undefined:sha256V1(JSON.stringify(rows));
}

/** 📋️ The facts a build script run depends on, recomputed from the disk: every declared path (a missing one means Cargo reruns always, so nothing is cacheable), the package tree, and the declared environment variables. */
export function buildFactsV1(input:Readonly<{cwd:string;directives:Readonly<{files:readonly string[];variables:readonly string[]}>;laneRoot:string|undefined;environment:Readonly<Record<string,string|undefined>>;stampDirectory:string|undefined}>):{facts:{path:string;kind:"file"|"tree"|"package";sha256:string}[];environment:{name:string;value:string|null}[]}|undefined{
 const facts:{path:string;kind:"file"|"tree"|"package";sha256:string}[]=[],build=policy.buildScript,declared=input.directives.files.length+input.directives.variables.length>0;
 for(const path of [...new Set(input.directives.files)].sort()){
  const absolute=resolve(input.cwd,path);let stat;try{stat=lstatSync(absolute);}catch{return undefined;}
  if(stat.isDirectory()){const sha=treeDigestV1(absolute,{ignoredDirectories:[],ignoredExtensions:[],maximumFiles:build.maximumPackageFiles,stampDirectory:input.stampDirectory});if(sha===undefined)return undefined;facts.push({path:laneNeutralV1(path,input.laneRoot),kind:"tree",sha256:sha});}
  else{const sha=digestFileV1(absolute,input.stampDirectory);if(sha===null)return undefined;facts.push({path:laneNeutralV1(path,input.laneRoot),kind:"file",sha256:sha});}
 }
 const root=input.environment.CARGO_MANIFEST_DIR??input.cwd,sha=treeDigestV1(root,{ignoredDirectories:build.ignoredDirectories,ignoredExtensions:declared?build.undeclaredIgnoredExtensions:[],maximumFiles:build.maximumPackageFiles,stampDirectory:input.stampDirectory});
 if(sha===undefined)return undefined;
 facts.push({path:".",kind:"package",sha256:sha});
 const environment=[...new Set(input.directives.variables)].sort().map(name=>({name,value:input.environment[name]===undefined?null:laneNeutralV1(input.environment[name]!,input.laneRoot)}));
 return{facts,environment};
}

/** 📦️ One cached build script run: the facts it depends on, its OUT_DIR tree, and the two output streams (lane-neutral). */
export type BuildEntryV1={version:1;key:string;package:string;facts:{path:string;kind:"file"|"tree"|"package";sha256:string}[];environment:{name:string;value:string|null}[];outputs:{path:string;sha256:string;bytes:number;executable:boolean;template:boolean}[];stdout:string|null;stderr:string|null;createdMs:number};
const runDirectory=(root:string,key:string)=>join(root,layout.runs,key.slice(0,2),key);

/** 🔎️ Returns the newest recorded run of `key` whose facts and environment still hold. */
export function lookupBuildRunV1(root:string,key:string,context:Readonly<{cwd:string;laneRoot:string|undefined;environment:Readonly<Record<string,string|undefined>>;stampDirectory:string|undefined}>):BuildEntryV1|undefined{
 const directory=runDirectory(root,key);let names:string[];
 try{names=readdirSync(directory).filter(name=>name.endsWith(".json")).sort().reverse();}catch{return undefined;}
 for(const name of names.slice(0,policy.key.maximumVariants)){
  let entry:BuildEntryV1;try{entry=JSON.parse(readFileSync(join(directory,name),"utf8"));}catch{continue;}
  if(entry.version!==1||entry.key!==key)continue;
  if(lostObjectV1(root,entry))continue;
  const directives={files:entry.facts.filter(row=>row.kind!=="package").map(row=>laneRestoreV1(row.path,context.laneRoot)),variables:entry.environment.map(row=>row.name)};
  const now=buildFactsV1({cwd:context.cwd,directives,laneRoot:context.laneRoot,environment:context.environment,stampDirectory:context.stampDirectory});
  if(!now)continue;
  const packageFact=entry.facts.find(row=>row.kind==="package"),nowPackage=now.facts.find(row=>row.kind==="package");
  if(!packageFact||packageFact.sha256!==nowPackage?.sha256)continue;
  const same=(left:{path:string;sha256:string},right:{path:string;sha256:string})=>left.path===right.path&&left.sha256===right.sha256;
  const recorded=entry.facts.filter(row=>row.kind!=="package"),current=now.facts.filter(row=>row.kind!=="package");
  if(recorded.length!==current.length||!recorded.every((row,index)=>same(row,current[index]!)&&row.kind===current[index]!.kind))continue;
  if(entry.environment.length!==now.environment.length||!entry.environment.every((row,index)=>row.name===now.environment[index]!.name&&row.value===now.environment[index]!.value))continue;
  return entry;
 }
 return undefined;
}

function isText(bytes:Buffer):boolean{if(bytes.includes(0))return false;try{new TextDecoder("utf-8",{fatal:true}).decode(bytes);return true;}catch{return false;}}

/** 📥️ Publishes one successful build script run. `undefined` when the run is not cacheable: a declared path is missing, the OUT_DIR holds a link, is too large, or a binary file embeds the lane path. */
export function publishBuildRunV1(root:string,input:Readonly<{key:string;package:string;cwd:string;laneRoot:string|undefined;directives:Readonly<{files:readonly string[];variables:readonly string[]}>;environment:Readonly<Record<string,string|undefined>>;outDir:string;stdout:string;stderr:string;stampDirectory:string|undefined}>):BuildEntryV1|undefined{
 const known=buildFactsV1({cwd:input.cwd,directives:input.directives,laneRoot:input.laneRoot,environment:input.environment,stampDirectory:input.stampDirectory});if(!known)return undefined;
 const outputs:BuildEntryV1["outputs"]=[];let total=0,refused=false;
 const walk=(directory:string):void=>{
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  for(const entry of entries.sort((left,right)=>left.name<right.name?-1:1)){
   const path=join(directory,entry.name),name=relative(input.outDir,path).split(sep).join("/");
   if(entry.isDirectory()){walk(path);continue;}
   if(!entry.isFile()){refused=true;return;}
   const stat=statSync(path);total+=stat.size;if(total>policy.buildScript.maximumOutputBytes){refused=true;return;}
   const bytes=readFileSync(path),text=isText(bytes),neutral=text?laneNeutralV1(bytes.toString("utf8"),input.laneRoot):undefined;
   if(!text&&input.laneRoot&&bytes.includes(Buffer.from(input.laneRoot))){refused=true;return;}
   const template=neutral!==undefined&&neutral!==bytes.toString("utf8"),stored=template?Buffer.from(neutral!):bytes;
   outputs.push({path:name,sha256:putObject(root,stored),bytes:stored.length,executable:(stat.mode&0o111)!==0,template});
  }
 };
 walk(input.outDir);
 if(refused)return undefined;
 const neutral=(text:string)=>laneNeutralV1(text,input.laneRoot);
 const entry:BuildEntryV1={version:1,key:input.key,package:input.package,facts:known.facts,environment:known.environment,outputs,stdout:input.stdout?putObject(root,neutral(input.stdout)):null,stderr:input.stderr?putObject(root,neutral(input.stderr)):null,createdMs:Date.now()};
 const directory=ensure(runDirectory(root,input.key)),temporary=join(ensure(join(root,layout.temporary)),randomBytes(8).toString("hex")),name=String(entry.createdMs).padStart(15,"0")+"-"+randomBytes(3).toString("hex")+".json";
 writeFileSync(temporary,JSON.stringify(entry));renameSync(temporary,join(directory,name));
 try{const names=readdirSync(directory).filter(file=>file.endsWith(".json")).sort();for(const old of names.slice(0,Math.max(0,names.length-policy.key.maximumVariants)))rmSync(join(directory,old),{force:true});}catch{}
 return entry;
}

/** ♻️ Writes a cached run's OUT_DIR tree, and returns the streams to replay, as the script would have produced them. */
export function restoreBuildRunV1(root:string,entry:BuildEntryV1,context:Readonly<{outDir:string;laneRoot:string|undefined}>):{stdout:string;stderr:string}{
 ensure(context.outDir);
 for(const row of entry.outputs){
  const destination=join(context.outDir,row.path);ensure(dirname(destination));rmSync(destination,{force:true});
  const source=objectPath(root,row.sha256);
  if(row.template)writeFileSync(destination,laneRestoreV1(readFileSync(source,"utf8"),context.laneRoot));else copyFileSync(source,destination,constants.COPYFILE_FICLONE);
  if(row.executable)chmodSync(destination,0o755);
  try{const now=new Date();utimesSync(source,now,now);}catch{}
 }
 const text=(sha:string|null)=>sha===null?"":laneRestoreV1(readObject(root,sha)?.toString("utf8")??"",context.laneRoot);
 return{stdout:text(entry.stdout),stderr:text(entry.stderr)};
}
