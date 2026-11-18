import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {readFile} from "node:fs/promises";
import {join} from "node:path";
import {CompactTableV1,ReplicaBookV1,localPathsV1} from "../../../../🤝️shared-worktree/🔁️replica/📇️book/🟦️.ts";
import {decideTrustV1,trustPolicyV1} from "../../../../🤝️shared-worktree/🔏️trust/🟦️.ts";
import {StateCorruptV1,checkpointPartsV1,decodeRecordsV1,partWidthV1,readCheckpointV1,readWalV1} from "../../../../🤝️shared-worktree/🔁️replica/💾️state/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {GRAPH_STATE_POLICY_V1 as policy,type SharedFileV1} from "../🟦️.ts";

const pageBytes=1<<20,typed={u8:Uint8Array,u16:Uint16Array,u32:Uint32Array,i32:Int32Array,f64:Float64Array,text:Uint8Array} as const;

/** 📥️ The files of the replica of one worktree: what the daemon of this machine holds as applied state, read without touching the worktree. */
export type ReplicaFilesV1=Readonly<{generation:number;shared:number;manifest:string;files:()=>Iterable<SharedFileV1>;entries:()=>Iterable<ReplicaEntryV1>;get(path:string):string|undefined}>;
/** 🧾️ One regular file of the replica with what the daemon saw when it hashed it: path, SHA-256, size, modification time and whether the stamp is too young to prove the bytes (the daemon hashed it within the racy window of the change). */
export type ReplicaEntryV1=readonly [path:string,sha256:string,bytes:number,mtimeMs:number,racy:boolean];

/** 🩺️ Whether the replica daemon of the worktree is healthy now: it wrote its health file within the allowed age, its store is reachable, its process lives and its `trust` statement grants the tier `trusted` (`decideTrustV1`: a sweep found the manifest equal to the tree after the last event that can make it wrong). The manifest may stand in for a file walk only then; lifetime failure counters play no part. */
export async function replicaHealthyV1(worktree:string,now:number=Date.now(),alive:(pid:number)=>boolean=pid=>{try{process.kill(pid,0);return true;}catch(error){return (error as NodeJS.ErrnoException).code==="EPERM";}}):Promise<boolean>{
 try{
  const value=JSON.parse(await readFile(machinePathV1(worktree,".cache","f","health.json"),"utf8")) as {wallMs:number;pid:number;store:string;trust:Parameters<typeof decideTrustV1>[0]["trust"]};
  if(validateJsonSchemaSubset({$ref:"#/definitions/Health"},value,schema).length)return false;
  return decideTrustV1(value,now,alive,{healthMaximumAgeMilliseconds:policy.healthMaximumAgeMilliseconds,verifiedMaximumAgeMilliseconds:trustPolicyV1.verifiedMaximumAgeMilliseconds})==="trusted";
 }catch{return false;}
}

/** 📖️ Reads the checkpoint and the delta log of the replica of `worktree` into the same columnar tables the daemon uses. `undefined` when there is no checkpoint (no replica, or a machine that never joined); a corrupt or torn state throws `StateCorruptV1`. Only `put` entries become files: links are skipped like the build graph walk skips them. */
export async function readReplicaFilesV1(worktree:string,attempts:number=3):Promise<ReplicaFilesV1|undefined>{
 const paths=localPathsV1(worktree),arrays=new Map<string,ArrayBufferView>(),rows=new Map<string,number>();
 let pages:Buffer|undefined,pageUsed=0;
 const header=await readCheckpointV1(paths.checkpoint,(name,length)=>{
  const kind=checkpointPartsV1[name];
  if(kind===undefined||name.startsWith("queue.")||name.startsWith("folds.")||name.startsWith("dirs.")||name==="files.targets")return undefined;
  const width=partWidthV1[kind]!;
  if(length%width!==0)throw new StateCorruptV1(`part ${name}`);
  if(name==="book.pages"){pages=Buffer.allocUnsafe(Math.ceil(length/pageBytes)*pageBytes);pageUsed=length;return pages.subarray(0,length);}
  const stride=name.endsWith(".sha")?32:1,count=length/width/stride,capacity=count*stride+(name==="book.starts"?1:0),array=new typed[kind](capacity);
  arrays.set(name,array);rows.set(name,count);
  return new Uint8Array(array.buffer,0,length);
 });
 if(header===undefined)return undefined;
 const get=<T extends ArrayBufferView>(name:string):T=>{const value=arrays.get(name);if(value===undefined)throw new StateCorruptV1(`missing part ${name}`);return value as unknown as T;};
 const meta=get<Float64Array>("meta");
 if(meta.length<3)throw new StateCorruptV1("meta");
 for(const name of ["files.flags","files.ms","files.counter","files.origin","files.bytes","files.sha","files.mtime","files.seen"])if(rows.get(name)!==meta[2])throw new StateCorruptV1(`rows of ${name}`);
 const list:Buffer[]=[];
 if(pages!==undefined)for(let offset=0;offset<pages.length;offset+=pageBytes)list.push(pages.subarray(offset,offset+pageBytes));
 const origins=Buffer.from(get<Uint8Array>("origins")).toString("utf8"),book=new ReplicaBookV1(),files=new CompactTableV1(book,true);
 book.adopt({starts:get("book.starts"),lengths:get("book.lengths"),hashes:get("book.hashes"),folded:get("book.folded"),pages:list,used:list.length===0?pageBytes:pageUsed-(list.length-1)*pageBytes},header.paths,origins===""?[]:origins.split("\n"));
 files.adopt({flags:get("files.flags"),ms:get("files.ms"),counter:get("files.counter"),origin:get("files.origin"),bytes:get("files.bytes"),sha:get("files.sha"),mtime:get("files.mtime"),seen:get("files.seen")},get("fold"),meta[0]!);
 const found=await readWalV1(paths.wal);
 if(found!==undefined){
  const {scan}=found;
  if(scan.verdict==="corrupt")throw new StateCorruptV1("log");
  if(scan.generation!==undefined&&scan.generation>header.generation){if(attempts>1)return readReplicaFilesV1(worktree,attempts-1);throw new StateCorruptV1("log generation");}
  if(scan.generation===header.generation)for(const payload of scan.payloads)for(const record of decodeRecordsV1(payload)){
   if(record.kind!=="files")continue;
   if(record.entry===null){const id=book.find(record.path);if(id!==undefined)files.clearId(id);}
   else files.write(book.intern(record.path),record.entry);
  }
 }
 return Object.freeze({
  generation:header.generation,shared:files.shared,manifest:files.manifestHex(),
  *files(){for(const id of files.ids())if(files.operationAt(id)==="put")yield [book.name(id),files.shaAt(id)] as SharedFileV1;},
  *entries(){for(const id of files.ids())if(files.operationAt(id)==="put")yield [book.name(id),files.shaAt(id),files.bytes[id]!,files.mtime[id]!,files.racyAt(id)] as ReplicaEntryV1;},
  get(path:string){const id=book.find(path);return id!==undefined&&files.operationAt(id)==="put"?files.shaAt(id):undefined;},
 });
}
