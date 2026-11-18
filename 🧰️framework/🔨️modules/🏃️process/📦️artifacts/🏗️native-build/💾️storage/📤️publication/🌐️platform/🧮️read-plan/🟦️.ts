import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import type {ArtifactRelativeDirectoryV1,ArtifactRelativeEntryV1} from "@semio-tech/framework-filesystem/observation/descriptor";

/** 🧮️ How much of a staged input group a read proves by hashing; see the schema description. */
export type ProtectedArtifactInputHashModeV1="all"|"root"|"changed";

/** ⏳️ The racy window in nanoseconds: a file whose newest time is this close to the recording pass is hashed again. */
export const PROTECTED_ARTIFACT_INPUT_SETTLE_NS_V1=BigInt(schema.definitions.Policy.const.settleNanoseconds);

/** 🔬️ Whether a read hashes the bytes of one staged input file or vouches for it by its stamp. See https://git-scm.com/docs/racy-git */
export function protectedArtifactInputHashedV1(mode:ProtectedArtifactInputHashModeV1,file:Readonly<{root:number}>,rootIndex:number|undefined,times:Readonly<{mtimeNs:bigint;ctimeNs:bigint}>,recordedAtNs:bigint|undefined):boolean{
 if(mode==="all"||(mode==="root"&&file.root===rootIndex)||recordedAtNs===undefined)return true;
 const newest=times.mtimeNs>times.ctimeNs?times.mtimeNs:times.ctimeNs;
 return newest+PROTECTED_ARTIFACT_INPUT_SETTLE_NS_V1>recordedAtNs;
}

/** 🌳️ Calls `visit` once for each requested directory path, descending from each parent instead of from the root for every path; every step re-reads the entry and refuses anything but a same-device directory. */
export async function visitProtectedArtifactInputDirectoriesV1(root:ArtifactRelativeDirectoryV1,paths:Iterable<string>,visit:(path:string,directory:ArtifactRelativeDirectoryV1)=>Promise<void>):Promise<void>{
 type Node={children:Map<string,Node>;wanted:boolean};
 const top:Node={children:new Map(),wanted:false},device=root.stat().dev;
 for(const path of paths){let node=top;for(const name of path.split("/")){let next=node.children.get(name);if(!next){next={children:new Map(),wanted:false};node.children.set(name,next);}node=next;}node.wanted=true;}
 const walk=async(directory:ArtifactRelativeDirectoryV1,node:Node,prefix:string):Promise<void>=>{
  for(const[name,child]of node.children){
   const identity=await directory.entry(name) as ArtifactRelativeEntryV1|null;
   if(!identity||identity.kind!=="directory"||identity.dev!==device)throw Error("Actual same-device immutable input directory required");
   const path=prefix===""?name:prefix+"/"+name;
   await directory.withDirectoryPath([Object.freeze({name,identity})],async opened=>{if(child.wanted)await visit(path,opened);await walk(opened,child,path);});
  }
 };
 await walk(root,top,"");
}

/** 📐️ The number of checkpoints a staging or a read of one input group may pass before it is refused as runaway: the operations of the group (a base, each root with its path segments, each file with its path segments and its chunks) times `checkpointsPerOperation`, so the allowance is derived from the size of the work and not from a fixed step count that one platform exceeds with correct work.
 * @see ./🧪️tests/🟦️.ts */
export function protectedArtifactInputWorkAllowanceV1(shape:Readonly<{roots:readonly string[];files:readonly Readonly<{path:string;bytes:number}>[]}>,chunkBytes:number):number{
 const policy=schema.definitions.WorkAllowance.const,segments=(path:string):number=>path.split("/").length;
 let operations=policy.operationsBase;
 for(const root of shape.roots)operations+=policy.operationsPerRoot+segments(root);
 for(const file of shape.files)operations+=policy.operationsPerFile+segments(file.path)+policy.operationsPerChunk*Math.max(1,Math.ceil(Math.ceil(file.bytes/3)*4/chunkBytes));
 return operations*policy.checkpointsPerOperation;
}

/** 🪣️ The step of staging and reading back the bytes of a batch: the largest power of two of base64 characters between the declared bounds that is at most a quarter of the batch and at most a 128th of the free memory (`decoded` is the byte count of `encoded` characters).
 * @see ./🧬️schema/🔣️.json */
export function protectedArtifactInputChunkV1(totalBytes:number,freeMemoryBytes:number):Readonly<{encoded:number;decoded:number}>{
 const plan=schema.definitions.ChunkPlan.const,want=Math.max(1,Math.min(Math.floor(totalBytes/plan.batchDivisor),Math.floor(freeMemoryBytes/plan.memoryDivisor)));
 let exponent=plan.minimumEncodedExponent;
 while(exponent<plan.maximumEncodedExponent&&2**(exponent+1)/4*3<=want)exponent++;
 return Object.freeze({encoded:2**exponent,decoded:2**exponent/4*3});
}

/** ✍️ Writes the bytes of one base64 file in steps of one chunk: one decode and one call of the file per step, one check before each call and one progress step after it; returns the bytes written. */
export async function copyProtectedArtifactInputV1(file:Readonly<{write(buffer:Uint8Array,offset:number,length:number,position:number):number}>,data:string,chunk:Readonly<{encoded:number}>,ports:Readonly<{check():void;step():Promise<void>}>):Promise<number>{
 let position=0;
 for(let offset=0;offset<data.length;offset+=chunk.encoded){
  const bytes=Buffer.from(data.slice(offset,offset+chunk.encoded),"base64");
  for(let consumed=0;consumed<bytes.length;){ports.check();const count=file.write(bytes,consumed,bytes.length-consumed,position);if(!count)throw Error("Immutable input write made no progress");consumed+=count;position+=count;}
  await ports.step();
 }
 return position;
}

/** 📖️ Feeds the bytes of one staged file to `update` in steps of one chunk: one call of the file per step, one check before it and one progress step after it; returns the bytes read. */
export async function readProtectedArtifactInputV1(file:Readonly<{read(buffer:Uint8Array,offset:number,length:number,position:number):number}>,bytes:number,chunk:Readonly<{decoded:number}>,update:(buffer:Uint8Array)=>void,ports:Readonly<{check():void;step():Promise<void>}>):Promise<number>{
 const buffer=Buffer.alloc(Math.max(1,Math.min(chunk.decoded,bytes)));
 let offset=0;
 while(offset<bytes){ports.check();const count=file.read(buffer,0,Math.min(buffer.length,bytes-offset),offset);if(!count)throw Error("Complete staged input read extent required");offset+=count;update(buffer.subarray(0,count));await ports.step();}
 return offset;
}
