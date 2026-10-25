import {createHash,randomBytes} from "node:crypto";
import {lstat,readFile,rename,rm,writeFile} from "node:fs/promises";
import {join} from "node:path";

/** 📌️ Format of the persisted generation receipt; see `./🧬️schema/🔣️.json`. */
export const NX_GENERATION_FORMAT=1;
/** 📄️ File name of the receipt inside one Nx workspace data namespace. */
export const NX_GENERATION_FILE="semio-nx-generation.json";
/** 🗂️ The graph files one finished construction leaves in the namespace; all three must exist for a reusable generation. */
export const NX_GENERATION_GRAPH_FILES=["project-graph.json","file-map.json","source-maps.json"] as const;
/** 🧭️ Files whose bytes decide what an Nx project graph contains besides the pinned portable graph, in digest order. */
export const NX_GENERATION_IDENTITY_FILES=["node_modules/nx/package.json","node_modules/@nx/js/package.json","node_modules/@nxlv/python/package.json","nx.json","package.json"] as const;

type Stamp=readonly [dev:number,ino:number,size:number,mtimeMs:number,ctimeMs:number];

/** 🎫️ What a generation was built from: the sealed portable graph it projects, the plugin implementation and the Nx tooling. */
export interface NxGenerationClaimV1 {readonly graphDigest:string;readonly implementation:string;readonly tooling:string;readonly identity:string}

/** 🧾️ The persisted proof that the graph files of a namespace belong to exactly one claim. */
export interface NxGenerationReceiptV1 {readonly format:typeof NX_GENERATION_FORMAT;readonly claim:NxGenerationClaimV1;readonly graph:Readonly<Record<(typeof NX_GENERATION_GRAPH_FILES)[number],Stamp>>}

const sha256=(value:string|Uint8Array)=>createHash("sha256").update(value).digest("hex");
const stampOf=(value:{dev:number;ino:number;size:number;mtimeMs:number;ctimeMs:number}):Stamp=>[value.dev,value.ino,value.size,value.mtimeMs,value.ctimeMs];
const same=(left:Stamp,right:Stamp)=>left.every((value,index)=>value===right[index]);
const hex=/^[a-f0-9]{64}$/u;

/** 🧮️ Digests the identity files of a workspace: each file as `<byte length>:<bytes>`, a missing file as `!<path>`. */
export async function nxGenerationIdentityV1(root:string):Promise<string>{
 const parts=await Promise.all(NX_GENERATION_IDENTITY_FILES.map(async path=>{try{return await readFile(join(root,path));}catch{return Buffer.from(`!${path}`);}}));
 return sha256(Buffer.concat(parts.flatMap(part=>[Buffer.from(`${part.byteLength}:`),part])));
}

/** 🔧️ Builds the claim of one invocation from its pin, implementation and tooling identities. */
export async function nxGenerationClaimV1(root:string,environment:Readonly<Record<string,string|undefined>>):Promise<NxGenerationClaimV1|undefined>{
 const graphDigest=environment.SEMIO_BUILD_GRAPH_DIGEST,implementation=environment.SEMIO_NX_IMPLEMENTATION_IDENTITY,tooling=environment.SEMIO_NX_TOOLING_IDENTITY;
 if(!graphDigest||!implementation||!tooling||![graphDigest,implementation,tooling].every(value=>hex.test(value)))return undefined;
 return Object.freeze({graphDigest,implementation,tooling,identity:await nxGenerationIdentityV1(root)});
}

async function graphStamps(directory:string):Promise<Record<string,Stamp>|undefined>{
 const result:Record<string,Stamp>={};
 for(const name of NX_GENERATION_GRAPH_FILES){try{const status=await lstat(join(directory,name));if(!status.isFile()||status.isSymbolicLink()||status.nlink!==1)return undefined;result[name]=stampOf(status);}catch{return undefined;}}
 return result;
}

/** ✍️ Records that the graph files now in `directory` were constructed for `claim`; false (and no receipt) while a graph file is missing, shared or replaced during the write, or the identity moved since the claim was made. `between` runs after the first stamps and before the receipt is sealed (a seam for the race laws). */
export async function writeNxGenerationReceiptV1(root:string,directory:string,claim:NxGenerationClaimV1,signal?:AbortSignal,between?:()=>Promise<void>):Promise<boolean>{
 const target=join(directory,NX_GENERATION_FILE);
 await rm(target,{force:true});
 if(await nxGenerationIdentityV1(root)!==claim.identity)return false;
 const before=await graphStamps(directory);
 if(!before)return false;
 signal?.throwIfAborted();
 await between?.();
 const receipt={format:NX_GENERATION_FORMAT,claim,graph:before},temporary=`${target}.${process.pid}.${randomBytes(6).toString("hex")}.tmp`;
 try{
  await writeFile(temporary,JSON.stringify(receipt)+"\n",{flag:"wx"});
  const after=await graphStamps(directory);
  if(!after||NX_GENERATION_GRAPH_FILES.some(name=>!same(before[name]!,after[name]!))||await nxGenerationIdentityV1(root)!==claim.identity)return false;
  signal?.throwIfAborted();
  await rename(temporary,target);
  return true;
 }finally{await rm(temporary,{force:true});}
}

/** ✅️ True when the namespace holds a receipt for exactly `claim` and every graph file still carries the stamp the receipt names; costs three stats and a few small reads, never the graph bytes. */
export async function nxGenerationHoldsV1(root:string,directory:string,claim:NxGenerationClaimV1,signal?:AbortSignal):Promise<boolean>{
 let receipt:NxGenerationReceiptV1;
 try{receipt=JSON.parse(await readFile(join(directory,NX_GENERATION_FILE),"utf8")) as NxGenerationReceiptV1;}catch{return false;}
 if(receipt.format!==NX_GENERATION_FORMAT||receipt.claim?.graphDigest!==claim.graphDigest||receipt.claim.implementation!==claim.implementation||receipt.claim.tooling!==claim.tooling||receipt.claim.identity!==claim.identity)return false;
 signal?.throwIfAborted();
 if(await nxGenerationIdentityV1(root)!==claim.identity)return false;
 const current=await graphStamps(directory);
 return current!==undefined&&NX_GENERATION_GRAPH_FILES.every(name=>Array.isArray(receipt.graph?.[name])&&same(receipt.graph[name],current[name]!));
}
