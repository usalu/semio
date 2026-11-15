import {createHash,randomBytes} from "node:crypto";
import {lstatSync,readFileSync} from "node:fs";
import {mkdir,readFile,readdir,rename,rm,stat,utimes,writeFile} from "node:fs/promises";
import {fileURLToPath} from "node:url";
import {join} from "node:path";
import {brotliCompressSync,brotliDecompressSync,constants as zlib} from "node:zlib";
import {classifyGraphInputV1,type SharedFileV1} from "../🟦️.ts";
import {readReplicaFilesV1,type ReplicaEntryV1} from "../📥️replica/🟦️.ts";
import {compileIgnoreV1,refusePathV1,sharedV1} from "../../../../🤝️shared-worktree/🚧️boundary/🟦️.ts";
import {decideTrustV1,requireTrustShapeV1,trustPolicyV1,type TrustV1} from "../../../../🤝️shared-worktree/🔏️trust/🟦️.ts";
import {createVerdictMemoV1,verdictCodeDigestV1,verdictKeyV1} from "./🧠️verdicts/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📍️ The authored limits of the manifest cursor.
 * @see ./🧬️schema/🔣️.json */
export const GRAPH_CURSOR_POLICY_V1=schema.definitions.Policy.const;
const policy=GRAPH_CURSOR_POLICY_V1,texts=schema.definitions.TextCatalogue.const;
/** 🗣️ One message of the cursor in one language. */
export function graphCursorTextV1(key:keyof typeof texts,language:"en"|"de",fields:Readonly<Record<string,string|number>>={}):string{return texts[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(fields[name]??""));}

/** 📸️ A state of the shared files the graph walk can enter: paths in UTF-16 code unit order, the first 128 bits of the SHA-256 of each file and the identity of the whole. */
export type SnapshotV1=Readonly<{instance:string;paths:readonly string[];shas:readonly string[]}>;
/** 🧾️ One row of the dirty set, in the closed shape of the watcher journal: added, modified or removed, directory or not, and the path. */
export type CursorRowV1=readonly ["a"|"m"|"r",0|1,string];
/** 🔌️ The cursor of the journal ports. */
export type CursorV1=Readonly<{instance:string;seq:number}>;
/** 🔌️ The answer of a port to "what changed since". */
export type SinceV1={complete:true;cursor:CursorV1;rows:readonly CursorRowV1[]}|{complete:false;reason:string;detail?:string;path?:string;count?:number};
/** 🔌️ A journal port as the graph hooks use it. `watches` says how much of the tree the port sees: a `governed` port sees only shared files, so the host keeps proving the membership by a stat pass and uses the port for the facts. */
export interface JournalPortV1{readonly watches?:"all"|"governed";cursor(signal?:AbortSignal):Promise<CursorV1|undefined>;since(from:CursorV1,signal?:AbortSignal):Promise<SinceV1>;excluded():(path:string)=>boolean}

/** 🪪️ The snapshot of a list of shared files: the paths the graph walk can enter, each with its digest (a row may read the bytes of any of them). Throws on a path that the text form cannot hold. */
export function snapshotOfFilesV1(files:Iterable<SharedFileV1>):SnapshotV1{
 const rows:[string,string][]=[];
 for(const [path,sha] of files){
  if(classifyGraphInputV1(path)==="ignored")continue;
  if(path.includes("\t")||path.includes("\n")||path.includes("\r"))throw Error("unsafe path");
  rows.push([path,sha.slice(0,policy.shaLength)]);
 }
 rows.sort((a,b)=>a[0]<b[0]?-1:a[0]>b[0]?1:0);
 const paths=rows.map(row=>row[0]),shas=rows.map(row=>row[1]);
 return Object.freeze({instance:instanceOfV1(paths,shas),paths,shas});
}

function instanceOfV1(paths:readonly string[],shas:readonly string[]):string{
 const hash=createHash("sha256");
 hash.update(policy.domain+"\n");
 for(let at=0;at<paths.length;at++)hash.update(paths[at]+"\t"+shas[at]+"\n");
 return hash.digest("hex").slice(0,policy.instanceLength);
}

/** 📦️ The stored form of a snapshot: brotli of the header line and one line per file. */
export function encodeSnapshotV1(snapshot:SnapshotV1):Uint8Array{
 const lines=[policy.header];
 for(let at=0;at<snapshot.paths.length;at++)lines.push(snapshot.paths[at]+"\t"+snapshot.shas[at]);
 const text=Buffer.from(lines.join("\n")+"\n","utf8");
 return brotliCompressSync(text,{params:{[zlib.BROTLI_PARAM_QUALITY]:policy.brotliQuality,[zlib.BROTLI_PARAM_LGWIN]:policy.brotliWindow,[zlib.BROTLI_PARAM_SIZE_HINT]:text.length}});
}

/** 📖️ Reads a stored snapshot; the instance is recomputed from the content and a text that does not carry the expected one is refused. */
export function decodeSnapshotV1(bytes:Uint8Array,expected?:string):SnapshotV1{
 const text=brotliDecompressSync(bytes,{maxOutputLength:policy.maximumBytes}).toString("utf8"),lines=text.split("\n");
 if(lines.at(-1)==="")lines.pop();
 if(lines[0]!==policy.header||lines.length-1>policy.maximumFiles)throw Error("graph snapshot required");
 const paths:string[]=[],shas:string[]=[];
 for(let at=1;at<lines.length;at++){const line=lines[at]!,cut=line.lastIndexOf("\t");if(cut<0)throw Error("graph snapshot line required");paths.push(line.slice(0,cut));shas.push(line.slice(cut+1));}
 const instance=instanceOfV1(paths,shas);
 if(expected!==undefined&&instance!==expected)throw Error("graph snapshot differs from its name");
 return Object.freeze({instance,paths,shas});
}

/** 📁️ The directories that hold the files of a snapshot, every ancestor included, without the workspace root. */
export function snapshotDirectoriesV1(paths:readonly string[]):Set<string>{
 const directories=new Set<string>();
 for(const path of paths){
  for(let cut=path.lastIndexOf("/");cut>0;cut=path.lastIndexOf("/",cut-1)){const directory=path.slice(0,cut);if(directories.has(directory))break;directories.add(directory);}
 }
 return directories;
}

/** ➗️ The rows that turn `before` into `after`: a path only in `after` is added, only in `before` removed, in both with another digest modified; a directory that appears or vanishes with its files is added or removed too, so a listing of its parent is checked again. */
export function snapshotRowsV1(before:SnapshotV1,after:SnapshotV1):CursorRowV1[]{
 const rows:CursorRowV1[]=[];
 let left=0,right=0;const lefts=before.paths.length,rights=after.paths.length;
 while(left<lefts||right<rights){
  const a=left<lefts?before.paths[left]!:undefined,b=right<rights?after.paths[right]!:undefined;
  if(b===undefined||a!==undefined&&a<b){rows.push(["r",0,a!]);left++;}
  else if(a===undefined||b<a){rows.push(["a",0,b]);right++;}
  else{if(before.shas[left]!==after.shas[right])rows.push(["m",0,a]);left++;right++;}
 }
 if(rows.length===0)return rows;
 const was=snapshotDirectoriesV1(before.paths),now=snapshotDirectoriesV1(after.paths);
 for(const directory of now)if(!was.has(directory))rows.push(["a",1,directory]);
 for(const directory of was)if(!now.has(directory))rows.push(["r",1,directory]);
 return rows;
}

/** 🗄️ The snapshots of this machine, one file per instance below the cache of the replica; atomic writes, the newest kept. */
export function snapshotStoreV1(root:string,directory:string=join(root,...policy.directory.split("/")),maximum:number=policy.maximumSnapshots){
 const path=(instance:string)=>join(directory,instance+policy.extension);
 return Object.freeze({
  directory,
  async has(instance:string):Promise<boolean>{try{await stat(path(instance));return true;}catch{return false;}},
  async bytes(instance:string):Promise<Uint8Array|undefined>{try{return await readFile(path(instance));}catch{return undefined;}},
  async get(instance:string):Promise<SnapshotV1|undefined>{const bytes=await this.bytes(instance);if(bytes===undefined)return undefined;try{return decodeSnapshotV1(bytes,instance);}catch{return undefined;}},
  async putBytes(instance:string,bytes:Uint8Array):Promise<void>{
   decodeSnapshotV1(bytes,instance);
   await mkdir(directory,{recursive:true});
   const temporary=path(instance)+"."+randomBytes(6).toString("hex")+".tmp";
   try{await writeFile(temporary,bytes);await rename(temporary,path(instance));}finally{await rm(temporary,{force:true});}
  },
  async put(snapshot:SnapshotV1):Promise<void>{
   if(await this.has(snapshot.instance)){const now=new Date();await utimes(path(snapshot.instance),now,now).catch(()=>undefined);return;}
   await this.putBytes(snapshot.instance,encodeSnapshotV1(snapshot));
   await this.prune(snapshot.instance);
  },
  async prune(keep:string):Promise<void>{
   try{
    const rows=await Promise.all((await readdir(directory)).filter(name=>name.endsWith(policy.extension)).map(async name=>({name,at:(await stat(join(directory,name)).catch(()=>undefined))?.mtimeMs??0})));
    rows.sort((a,b)=>b.at-a.at);
    for(const row of rows.slice(maximum))if(row.name!==keep+policy.extension)await rm(join(directory,row.name),{force:true});
   }catch{}
  },
 });
}
/** 🗄️ The store of snapshots. */
export type SnapshotStoreV1=ReturnType<typeof snapshotStoreV1>;

/** 🧾️ The files whose code decides the verdict of a path that no snapshot names: the input classification and the roster behind it, the ignore rules and the path refusals of the boundary. */
const VERDICT_CODE_FILES_V1:readonly string[]=["../🟦️.ts","../🧬️schema/🔣️.json","../../🕸️graph/🧭️roster/🟨️.mjs","../../../../🤝️shared-worktree/🚧️boundary/🟦️.ts","../../../../🤝️shared-worktree/🚧️boundary/🔤️fold/🔣️.json"].map(path=>fileURLToPath(new URL(path,import.meta.url)));

const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}};

/** 🩺️ What the health file of the replica daemon of `root` allows: `unusable` (no fresh health, store unreachable or process dead: the manifest cannot be used), `verify` (the daemon does not state that its manifest reflects the tree: it has no verification yet, lost trust through an event that can make the manifest wrong for shared source paths, or its last verification is older than ten minutes; every file the manifest names is then checked against its size and modification time before it is believed) or `trusted` (a sweep of the daemon found the manifest equal to the tree after its last loss). The lifetime failure counters of the health file play no part; the rule is `decideTrustV1` of the shared worktree trust member, the statement is the `trust` object of the health schema. */
export async function manifestTrustV1(root:string,now:number=Date.now(),isAlive:(pid:number)=>boolean=alive):Promise<"unusable"|"verify"|"trusted">{
 try{
  const value=JSON.parse(await readFile(join(root,".cache","f","health.json"),"utf8")) as {wallMs:number;pid:number;store:string;trust?:unknown};
  if(typeof value.wallMs!=="number"||typeof value.pid!=="number")return "unusable";
  let trust:TrustV1|undefined;
  try{requireTrustShapeV1("Trust",value.trust,"trust");trust=value.trust as TrustV1;}catch{}
  return decideTrustV1({wallMs:value.wallMs,pid:value.pid,store:value.store,trust},now,isAlive,{healthMaximumAgeMilliseconds:policy.healthMaximumAgeMilliseconds,verifiedMaximumAgeMilliseconds:trustPolicyV1.verifiedMaximumAgeMilliseconds});
 }catch{return "unusable";}
}

/** ⏱️ Waits until the delta log of the replica has been quiet for the settle time, at most `settleMaximumMilliseconds`: a change made just before the command is applied to the manifest within about a second (a daemon that reports failures is not believed and checked against the tree instead, which needs no waiting). */
export async function settleReplicaV1(root:string,now:()=>number=Date.now):Promise<void>{
 const started=now();
 for(;;){
  const at=(await stat(join(root,".cache","f","state.wal")).catch(()=>undefined))?.mtimeMs;
  if(at===undefined)return;
  const wait=policy.settleMilliseconds-(now()-at);
  if(wait<=0||now()-started>=policy.settleMaximumMilliseconds)return;
  await new Promise(resolve=>setTimeout(resolve,Math.min(wait,250)));
 }
}

/** 🔎️ The shared files of the replica checked against the tree, the way a version control index is: an entry whose size and modification time equal what the daemon saw keeps its digest, every other one (changed, young, unreadable) is hashed from the file again and a file that is gone is dropped. A file that cannot be read gets a digest nothing else has, so every row that reads it is checked again. `stat` and `read` are the ports on the tree. */
export async function verifiedReplicaFilesV1(root:string,entries:Iterable<ReplicaEntryV1>,ports:Readonly<{lstat?:(path:string)=>Promise<{isFile():boolean;size:number;mtimeMs:number}|undefined>;read?:(path:string)=>Promise<Uint8Array|undefined>}>={}):Promise<SharedFileV1[]>{
 const gone=(error:unknown)=>["ENOENT","ENOTDIR"].includes((error as NodeJS.ErrnoException).code??"");
 const visible=[...entries].filter(entry=>classifyGraphInputV1(entry[0])!=="ignored"),out:SharedFileV1[]=[];
 const digest=(path:string,data:Uint8Array):SharedFileV1=>[path,createHash("sha256").update(data).digest("hex")];
 if(ports.lstat===undefined&&ports.read===undefined){
  for(let at=0;at<visible.length;at++){
   const[path,sha,bytes,mtimeMs,racy]=visible[at]!,full=root+"/"+path;
   try{
    const status=lstatSync(full);
    if(!status.isFile())continue;
    out.push(!racy&&status.size===bytes&&status.mtimeMs===mtimeMs?[path,sha]:digest(path,readFileSync(full)));
   }catch(error){if(!gone(error))out.push([path,randomBytes(32).toString("hex")]);}
   if(at%policy.verifyBatch===policy.verifyBatch-1)await new Promise(resolve=>setImmediate(resolve));
  }
  return out;
 }
 const statOf=ports.lstat??(async path=>{try{return lstatSync(path);}catch(error){if(gone(error))return undefined;throw error;}});
 const readOf=ports.read??(async path=>{try{return readFileSync(path);}catch(error){if(gone(error))return undefined;throw error;}});
 for(const[path,sha,bytes,mtimeMs,racy]of visible){
  const full=root+"/"+path;
  try{
   const status=await statOf(full);
   if(!status||!status.isFile())continue;
   if(!racy&&status.size===bytes&&status.mtimeMs===mtimeMs){out.push([path,sha]);continue;}
   const data=await readOf(full);
   if(data!==undefined)out.push(digest(path,data));
  }catch{out.push([path,randomBytes(32).toString("hex")]);}
 }
 return out;
}

/** 📥️ The shared files of the replica of `root`, believed or checked as the health of the daemon allows, or the key of the reason text when the manifest cannot be used. */
export async function replicaSnapshotSourceV1(root:string,now:number=Date.now()):Promise<{files:Iterable<SharedFileV1>}|{reason:"manifestDaemon"|"manifestUnread"}>{
 const trust=await manifestTrustV1(root,now);if(trust==="unusable")return{reason:"manifestDaemon"};
 if(trust==="trusted")await settleReplicaV1(root);
 try{
  const replica=await readReplicaFilesV1(root);if(replica===undefined)return{reason:"manifestUnread"};
  return{files:trust==="trusted"?replica.files():await verifiedReplicaFilesV1(root,replica.entries())};
 }catch{return{reason:"manifestUnread"};}
}

type Source=()=>Promise<{files:Iterable<SharedFileV1>}|{reason:"manifestDaemon"|"manifestUnread"}>;
/** 🔌️ A manifest journal port with the instance of its snapshots. */
export interface ManifestJournalPortV1 extends JournalPortV1{readonly watches:"governed";owns(instance:string):Promise<boolean>;readonly store:SnapshotStoreV1;lastReason():string|undefined;flushVerdicts():void}

/** 📍️ The journal port of the replica manifest. `cursor` is the identity of the snapshot of the shared files now (stored beside the others, so a later `since` can find it); `since(cursor)` is the set difference between that snapshot and the one now; `excluded` names every path the manifest cannot vouch for: ignored classes and any path that is in neither snapshot, and any directory that holds none of their files. */
export function manifestJournalPortV1(options:Readonly<{root:string;source?:Source;store?:SnapshotStoreV1;maximumRows?:number}>):ManifestJournalPortV1{
 const store=options.store??snapshotStoreV1(options.root),source:Source=options.source??(()=>replicaSnapshotSourceV1(options.root)),maximumRows=options.maximumRows??policy.maximumRows;
 let reason:string|undefined,shared:((path:string)=>boolean)|undefined,seen:{files:Set<string>;directories:Set<string>}|undefined,verdicts:ReturnType<typeof createVerdictMemoV1>|undefined;
 const codeDigest=verdictCodeDigestV1(VERDICT_CODE_FILES_V1),now=async():Promise<SnapshotV1|undefined>=>{
  const found=await source();
  if("reason" in found){reason=found.reason;return undefined;}
  let current:SnapshotV1;
  try{current=snapshotOfFilesV1(found.files);}catch{reason="manifestUnsafe";return undefined;}
  await store.put(current).catch(()=>undefined);
  return current;
 };
 const boundary=async()=>{
  const text=await readFile(join(options.root,".gitignore"),"utf8").catch(()=>""),ignore=compileIgnoreV1(text);
  shared=path=>refusePathV1(path)===undefined&&sharedV1(path,true,ignore);
  verdicts=createVerdictMemoV1({directory:join(store.directory,"..","verdicts"),key:codeDigest===undefined?undefined:verdictKeyV1({ignoreText:text,code:codeDigest})});
 };
 return Object.freeze({
  watches:"governed" as const,store,
  lastReason:()=>reason,
  owns:(instance:string)=>store.has(instance),
  async cursor(){const snapshot=await now();return snapshot?{instance:snapshot.instance,seq:0}:undefined;},
  async since(from:CursorV1):Promise<SinceV1>{
   const after=await now();
   if(!after)return{complete:false,reason:"unrecorded",detail:reason};
   const before=from.instance===after.instance?after:await store.get(from.instance);
   if(!before){reason="manifestSnapshot";return{complete:false,reason:"unrecorded",detail:reason};}
   const rows=before===after?[]:snapshotRowsV1(before,after);
   if(rows.length>maximumRows){reason="manifestOverflow";return{complete:false,reason:"unrecorded",detail:reason,count:maximumRows};}
   await boundary();
   seen={files:new Set(after.paths),directories:snapshotDirectoriesV1(after.paths)};
   return{complete:true,cursor:{instance:after.instance,seq:0},rows};
  },
  flushVerdicts(){verdicts?.save();},
  excluded(){
   const inside=shared,known=seen,remembered=verdicts,memo=new Map<string,boolean>();
   return(path:string)=>{
    if(path===""||known?.files.has(path)||known?.directories.has(path))return false;
    let verdict=memo.get(path);
    if(verdict===undefined){
     verdict=inside===undefined?undefined:remembered?.get(path);
     if(verdict===undefined){verdict=classifyGraphInputV1(path)==="ignored"||inside===undefined||!inside(path);if(inside!==undefined)remembered?.set(path,verdict);}
     memo.set(path,verdict);
    }
    return verdict;
   };
  },
 });
}

/** 🧩️ Joins journal ports in order of preference: the first that can give a cursor serves `cursor`; `since` goes to the port that holds the state of the cursor it is asked from (the manifest port when it stores that snapshot, else the next), and `excluded` is the predicate of the port that answered last. `say` receives the key of the reason when a port was passed over. */
export function composeJournalPortsV1(ports:readonly JournalPortV1[],manifest?:ManifestJournalPortV1,say:(reason:string)=>void=()=>undefined):JournalPortV1{
 let last:JournalPortV1=ports[0]!,told=false;
 const tell=(reason:string|undefined)=>{if(reason&&!told){told=true;say(reason);}};
 return {
  get watches(){return last.watches??"all";},
  async cursor(signal?:AbortSignal){
   for(const port of ports){const cursor=await port.cursor(signal);if(cursor){last=port;return cursor;}if(port===manifest)tell(manifest.lastReason());}
   return undefined;
  },
  async since(from:CursorV1,signal?:AbortSignal){
   const port=manifest&&await manifest.owns(from.instance)?manifest:ports.find(other=>other!==manifest)??ports[0]!;
   last=port;const answer=await port.since(from,signal);
   if(!answer.complete&&port===manifest)tell(manifest.lastReason());
   return answer;
  },
  excluded:()=>last.excluded(),
 };
}

/** 🧭️ The ports for one workspace: the replica manifest first, the watcher journal as the fallback (`watcher` builds it only when it is asked). */
export function graphJournalPortV1(root:string,watcher:()=>JournalPortV1,say:(reason:string)=>void=()=>undefined,options:Readonly<{source?:Source;store?:SnapshotStoreV1}>={}):JournalPortV1&{readonly manifest:ManifestJournalPortV1}{
 const manifest=manifestJournalPortV1({root,...options});
 let fallback:JournalPortV1|undefined;
 const lazy:JournalPortV1={
  get watches(){return fallback?.watches??"all";},
  cursor:signal=>(fallback??=watcher()).cursor(signal),
  since:(from,signal)=>(fallback??=watcher()).since(from,signal),
  excluded:()=>(fallback??=watcher()).excluded(),
 };
 return Object.create(composeJournalPortsV1([manifest,lazy],manifest,say),{manifest:{value:manifest,enumerable:true}}) as JournalPortV1&{readonly manifest:ManifestJournalPortV1};
}
