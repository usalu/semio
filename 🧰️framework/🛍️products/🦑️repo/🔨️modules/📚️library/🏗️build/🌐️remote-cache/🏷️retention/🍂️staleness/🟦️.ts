import {createHash} from "node:crypto";
import {mkdir,readFile,rename,stat,writeFile} from "node:fs/promises";
import {dirname,isAbsolute,join,relative,resolve,sep} from "node:path";
import {requireSlotShapeV1,type DeclarationV1,type StalenessModeV1} from "../../../🪢️slots/🟦️.ts";

/** 🧾️ One stored entry as the staleness judge sees it: its key, when it was stored (server clock), the blobs it references and what it declared it was built from (`undefined` for entries that declared nothing). */
export type StalenessEntryV1=Readonly<{key:string;storedAt:number;refs:readonly string[];declaration:DeclarationV1|undefined}>;

/** 🥀️ Why an entry is out: a recorded source digest differs from the tree (`edited`), a recorded source is gone (`deleted`), its toolchain is not the fleet's (`toolchain`), an entry it consumed is stale or gone (`extern`), no live entry consumes it any more (`unreached`). */
export type StalenessReasonV1="edited"|"deleted"|"toolchain"|"extern"|"unreached";

/** 🚦️ The verdict on one entry: `live` (current against the tree), `grace` (stored within the grace, never retired), `stale`, `unreached` (a registry or standard library entry no live entry consumes) or `undeclared` (not judged here). */
export type StalenessVerdictV1=Readonly<{state:"live"|"grace"|"stale"|"unreached"|"undeclared";reason?:StalenessReasonV1;detail?:string}>;

/** 🗂️ The inputs of one judgement: the clock and grace, the digest of a tree file (`null` when it is gone), the toolchain identities that are the fleet's (`undefined` judges no toolchain) and the output blobs of entries retired earlier (an extern that names one is gone). */
export type StalenessInputV1=Readonly<{now:number;graceMilliseconds:number;digest:(path:string)=>Promise<string|null>;toolchains?:ReadonlySet<string>|undefined;retired?:ReadonlySet<string>|undefined;signal?:AbortSignal|undefined}>;

/** 🧮️ Judges every entry against the synchronized tree, as a function of the entries and the tree alone: an entry is stale when a recorded source digest differs from the tree or the file is gone, its toolchain is not the fleet's, or every entry that produced one of the blobs it consumed is stale (a blob nobody holds that an earlier retirement took counts as stale; one nobody ever held is unknown and ignored); entries that are no root (a Rust unit without workspace sources: registry crates, the standard library; stage outputs and build script runs are roots whatever they read) are live while a live entry consumes them, transitively, and unreached otherwise. An entry stored within the grace is never stale or unreached. Entries without a declaration are `undeclared`. */
export async function judgeStalenessV1(entries:readonly StalenessEntryV1[],input:StalenessInputV1):Promise<Map<string,StalenessVerdictV1>>{
 const digests=new Map<string,string|null>(),paths=new Set<string>();
 for(const entry of entries)for(const row of entry.declaration?.sources??[])paths.add(row.path);
 const queue=[...paths];let next=0;
 await Promise.all(Array.from({length:Math.min(32,queue.length)},async()=>{for(;;){input.signal?.throwIfAborted();const path=queue[next++];if(path===undefined)return;digests.set(path,await input.digest(path));}}));
 const owners=new Map<string,string[]>(),byKey=new Map(entries.map(entry=>[entry.key,entry]));
 for(const entry of entries)for(const hash of entry.refs){const list=owners.get(hash);if(list)list.push(entry.key);else owners.set(hash,[entry.key]);}
 const own=new Map<string,Readonly<{reason:StalenessReasonV1;detail:string}>>();
 for(const entry of entries){
  const declaration=entry.declaration;if(declaration===undefined)continue;
  if(input.toolchains!==undefined&&!input.toolchains.has(declaration.toolchain)){own.set(entry.key,{reason:"toolchain",detail:declaration.toolchain.slice(0,12)});continue;}
  for(const row of declaration.sources){
   const current=digests.get(row.path);
   if(current===null||current===undefined){own.set(entry.key,{reason:"deleted",detail:row.path});break;}
   if(current!==row.sha256){own.set(entry.key,{reason:"edited",detail:row.path});break;}
  }
 }
 const settled=new Map<string,Readonly<{reason:StalenessReasonV1;detail:string}>|false>(),visiting=new Set<string>();
 const staleOf=(key:string):Readonly<{reason:StalenessReasonV1;detail:string}>|false=>{
  const known=settled.get(key);if(known!==undefined)return known;
  const entry=byKey.get(key);if(entry?.declaration===undefined)return false;
  const direct=own.get(key);if(direct){settled.set(key,direct);return direct;}
  if(visiting.has(key))return false;
  visiting.add(key);let verdict:Readonly<{reason:StalenessReasonV1;detail:string}>|false=false;
  for(const hash of entry.declaration.externs){
   const holders=(owners.get(hash)??[]).filter(other=>other!==key);
   if(holders.length===0){if(input.retired?.has(hash)){verdict={reason:"extern",detail:hash.slice(0,12)+" gone"};break;}continue;}
   if(holders.every(other=>staleOf(other)!==false)){verdict={reason:"extern",detail:hash.slice(0,12)+" "+holders[0]!.slice(-12)};break;}
  }
  visiting.delete(key);settled.set(key,verdict);return verdict;
 };
 const rooted=(declaration:DeclarationV1):boolean=>declaration.sources.length>0||declaration.kind!=="rust-unit",verdicts=new Map<string,StalenessVerdictV1>(),young=(entry:StalenessEntryV1):boolean=>input.now-entry.storedAt<input.graceMilliseconds,reached=new Set<string>(),pending:string[]=[];
 for(const entry of entries){
  if(entry.declaration===undefined){verdicts.set(entry.key,{state:"undeclared"});continue;}
  const stale=staleOf(entry.key);
  if(stale!==false&&!young(entry)){verdicts.set(entry.key,{state:"stale",reason:stale.reason,detail:stale.detail});continue;}
  if(rooted(entry.declaration)){verdicts.set(entry.key,{state:young(entry)&&stale!==false?"grace":"live"});pending.push(entry.key);}
 }
 for(let at=0;at<pending.length;at++){
  input.signal?.throwIfAborted();
  const entry=byKey.get(pending[at]!)!;
  for(const hash of entry.declaration!.externs)for(const holder of owners.get(hash)??[]){
   if(holder===entry.key||reached.has(holder))continue;
   const other=byKey.get(holder);if(other?.declaration===undefined||rooted(other.declaration)||verdicts.has(holder))continue;
   reached.add(holder);pending.push(holder);
  }
 }
 for(const entry of entries){
  if(verdicts.has(entry.key)||entry.declaration===undefined)continue;
  if(reached.has(entry.key))verdicts.set(entry.key,{state:"live"});
  else if(young(entry))verdicts.set(entry.key,{state:"grace"});
  else verdicts.set(entry.key,{state:"unreached",reason:"unreached",detail:"no live consumer"});
 }
 return verdicts;
}

/** 🌿️ The workspace source rows of one compilation from the dependency rows it recorded: a file below the workspace root (the working directory when the root is not known) that is a regular source, not one of the generated roots (`.cache`, `target`, …), not a lane-neutral token and not unreadable; portable path with forward slashes, sorted and without duplicates. */
export function workspaceRowsV1(input:Readonly<{dependencies:readonly Readonly<{path:string;sha256:string|null}>[];cwd:string;workspace?:string|undefined;generatedRoots:readonly string[]}>):readonly Readonly<{path:string;sha256:string}>[]{
 const root=resolve(input.workspace??input.cwd),rows=new Map<string,string>();
 for(const row of input.dependencies){
  if(row.sha256===null||row.path.includes("@@"))continue;
  const path=relative(root,isAbsolute(row.path)?row.path:resolve(input.cwd,row.path));
  if(path===""||path.startsWith("..")||isAbsolute(path))continue;
  const portable=path.split(sep).join("/");
  if(input.generatedRoots.includes(portable.split("/")[0]!))continue;
  rows.set(portable,row.sha256);
 }
 return [...rows].sort(([left],[right])=>left<right?-1:left>right?1:0).map(([path,sha256])=>({path,sha256}));
}

/** 🧬️ Builds the declaration of one published entry and checks it against the slot schema. */
export function declareV1(input:Readonly<{kind:DeclarationV1["kind"];toolchain:string;sources:DeclarationV1["sources"];externs:readonly string[]}>):DeclarationV1{
 const declaration:DeclarationV1={version:1,kind:input.kind,toolchain:/^[0-9a-f]{64}$/u.test(input.toolchain)?input.toolchain:createHash("sha256").update(input.toolchain).digest("hex"),sources:input.sources,externs:[...new Set(input.externs.filter(hash=>/^[0-9a-f]{64}$/u.test(hash)))].sort()};
 requireSlotShapeV1("Declaration",declaration,"declaration");return declaration;
}

/** 🌳️ The digests of the files of the synchronized tree: a regular file below the root is read once and remembered by its size, modification time and inode, so an unchanged file costs one `stat`; the memory is persisted beside the store. */
export interface TreeDigestsV1{digest(path:string):Promise<string|null>;flush():Promise<void>;readonly hashed:number}

/** 🌳️ Opens the digests of a tree. A path that leaves the root, a directory or an unreadable file has no digest (`null`). */
export async function openTreeDigestsV1(root:string,memoPath?:string):Promise<TreeDigestsV1>{
 const memo=new Map<string,Readonly<{size:number;mtimeMs:number;ino:number;sha256:string}>>();let dirty=false,hashed=0;
 if(memoPath!==undefined){try{for(const line of (await readFile(memoPath,"utf8")).split("\n")){if(line==="")continue;const [path,size,mtimeMs,ino,sha256]=JSON.parse(line) as [string,number,number,number,string];memo.set(path,{size,mtimeMs,ino,sha256});}}catch{}}
 const absolute=resolve(root),running=new Map<string,Promise<string|null>>();
 const compute=async(path:string):Promise<string|null>=>{
  const full=resolve(absolute,...path.split("/"));if(relative(absolute,full).startsWith("..")||isAbsolute(path))return null;
  let info;try{info=await stat(full);}catch{return null;}
  if(!info.isFile())return null;
  const known=memo.get(path);if(known&&known.size===info.size&&known.mtimeMs===info.mtimeMs&&known.ino===info.ino)return known.sha256;
  let bytes;try{bytes=await readFile(full);}catch{return null;}
  const sha256=createHash("sha256").update(bytes).digest("hex");hashed++;
  memo.set(path,{size:info.size,mtimeMs:info.mtimeMs,ino:info.ino,sha256});dirty=true;return sha256;
 };
 return {
  get hashed(){return hashed;},
  digest:path=>{let job=running.get(path);if(!job){job=compute(path).finally(()=>{running.delete(path);});running.set(path,job);}return job;},
  async flush(){
   if(memoPath===undefined||!dirty)return;dirty=false;
   const temporary=memoPath+"."+process.pid+".tmp";await mkdir(dirname(memoPath),{recursive:true});
   await writeFile(temporary,[...memo].map(([path,row])=>JSON.stringify([path,row.size,row.mtimeMs,row.ino,row.sha256])).join("\n")+"\n");await rename(temporary,memoPath);
  }
 };
}

/** 📋️ What one staleness sweep found, ready for the status endpoint and the log (`StalenessReport` of the slot schema): counts per state and per reason, how many entries a run in enforce mode retired, the refusals of the publish gate and the first examples. */
export function stalenessReportV1(input:Readonly<{mode:StalenessModeV1;now:number;verdicts:ReadonlyMap<string,StalenessVerdictV1>;retired:number;refused:Readonly<Record<string,number>>;examples:number}>):Readonly<{version:1;mode:StalenessModeV1;sweptAt:number;entries:number;declared:number;live:number;grace:number;stale:Record<string,number>;unreached:number;retired:number;refused:Record<string,number>;examples:string[]}>{
 const stale:Record<string,number>={},examples:string[]=[];let declared=0,live=0,grace=0,unreached=0;
 for(const [key,verdict] of input.verdicts){
  if(verdict.state!=="undeclared")declared++;
  if(verdict.state==="live")live++;else if(verdict.state==="grace")grace++;
  else if(verdict.state==="unreached"){unreached++;if(examples.length<input.examples)examples.push(`unreached ${key} (${verdict.detail})`);}
  else if(verdict.state==="stale"){stale[verdict.reason!]=(stale[verdict.reason!]??0)+1;if(examples.length<input.examples)examples.push(`${verdict.reason} ${key} (${verdict.detail})`);}
 }
 const report={version:1 as const,mode:input.mode,sweptAt:input.now,entries:input.verdicts.size,declared,live,grace,stale,unreached,retired:input.retired,refused:{...input.refused},examples};
 requireSlotShapeV1("StalenessReport",report,"staleness report");return report;
}

/** 🚪️ The publish gate: whether a declaration still matches the tree. Answers the first source that differs (`edited` or `deleted`) or `undefined` when every digest matches or the file is one the tree cannot judge. A difference is looked at again after `recheckMilliseconds` (the replica of the tree may lag by about a second) before it counts. */
export async function stalenessOnArrivalV1(declaration:DeclarationV1,input:Readonly<{digest:(path:string)=>Promise<string|null>;recheckMilliseconds:number;toolchains?:ReadonlySet<string>|undefined;wait?:(milliseconds:number)=>Promise<void>}>):Promise<Readonly<{reason:StalenessReasonV1;detail:string}>|undefined>{
 if(input.toolchains!==undefined&&!input.toolchains.has(declaration.toolchain))return {reason:"toolchain",detail:declaration.toolchain.slice(0,12)};
 const differing=async():Promise<Readonly<{reason:StalenessReasonV1;detail:string}>|undefined>=>{
  for(const row of declaration.sources){const current=await input.digest(row.path);if(current===null)return {reason:"deleted",detail:row.path};if(current!==row.sha256)return {reason:"edited",detail:row.path};}
  return undefined;
 };
 const first=await differing();if(first===undefined)return undefined;
 await (input.wait??(milliseconds=>new Promise<void>(done=>{setTimeout(done,milliseconds);})))(input.recheckMilliseconds);
 return differing();
}
