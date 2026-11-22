import {createHash} from "node:crypto";
import {existsSync,readdirSync,statSync,statfsSync,unlinkSync} from "node:fs";
import {join,resolve} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧱️ The authored policy of the native Nx shards: where they are mounted, which of their files Nx regenerates, how much room a rewrite needs and from which fill the shard is relieved. */
export const NATIVE_SHARDS_POLICY_V1=Object.freeze(schema.definitions.Policy.const);
/** 📋️ What is known about one mounted native shard: its mount, size, free bytes and top-level files. */
export type NativeShardFactV1=Readonly<{id:string;path:string;totalBytes:number;freeBytes:number;files:readonly Readonly<{name:string;bytes:number}>[]}>;
/** 🧾️ One file the relief removes. */
export type NativeShardRemovalV1=Readonly<{id:string;path:string;name:string;bytes:number}>;
/** 🔎️ The shard that cannot rewrite its file map, named for the refusal that replaces a bare ENOSPC. */
export type NativeShardConsumerV1=Readonly<{id:string;path:string;usedBytes:number;totalBytes:number;freeBytes:number;fileMapBytes:number;file:string}>;

/** 🗣️ One bilingual line of the shards, placeholders filled; `both` joins English and German. */
export function nativeShardTextV1(key:keyof typeof schema.definitions.Messages.const.en,language:"en"|"de"|"both",values:Readonly<Record<string,string|number>>):string{
 const fill=(template:string)=>template.replace(/\{([A-Za-z]+)\}/gu,(_,name:string)=>String(values[name]??"?")),messages=schema.definitions.Messages.const;
 return language==="both"?fill(messages.en[key])+" / "+fill(messages.de[key]):fill(messages[language][key]);
}

function fileMapOf(fact:NativeShardFactV1,policy=NATIVE_SHARDS_POLICY_V1):Readonly<{bytes:number;files:readonly Readonly<{name:string;bytes:number}>[]}>{
 const files=fact.files.filter(file=>policy.regenerableFiles.includes(file.name));
 return{bytes:files.reduce((sum,file)=>sum+file.bytes,0),files};
}

/** 🧮️ Whether a shard is short of room: it holds a regenerable file map that Nx rewrites next to the old copy, and either its free bytes do not cover that copy with the margin or it is at least `nearFullPercent` full. Pure. */
export function nativeShardShortV1(fact:NativeShardFactV1,policy=NATIVE_SHARDS_POLICY_V1):boolean{
 const map=fileMapOf(fact,policy).bytes;
 if(map===0)return false;
 const nearFull=fact.totalBytes>0&&(fact.totalBytes-fact.freeBytes)*100>=fact.totalBytes*policy.nearFullPercent;
 return fact.freeBytes<map+policy.rewriteMarginBytes||nearFull;
}

/** 🪚️ The files to remove so that every short shard can rewrite its file map; shards with room and files Nx does not regenerate are never listed. Pure and deterministic (shard id order). */
export function planNativeShardReliefV1(facts:readonly NativeShardFactV1[],policy=NATIVE_SHARDS_POLICY_V1):readonly NativeShardRemovalV1[]{
 return[...facts].sort((first,second)=>first.id<second.id?-1:first.id>second.id?1:0).filter(fact=>nativeShardShortV1(fact,policy)).flatMap(fact=>fileMapOf(fact,policy).files.map(file=>({id:fact.id,path:fact.path,name:file.name,bytes:file.bytes})));
}

/** 🔎️ The fullest short shard as the named consumer of a "no space left" that came from Nx, or `undefined` when every shard has room. */
export function nativeShardConsumerV1(facts:readonly NativeShardFactV1[],policy=NATIVE_SHARDS_POLICY_V1):NativeShardConsumerV1|undefined{
 const short=facts.filter(fact=>nativeShardShortV1(fact,policy)).sort((first,second)=>(second.totalBytes-second.freeBytes)-(first.totalBytes-first.freeBytes)||(first.id<second.id?-1:1))[0];
 if(short===undefined)return undefined;
 const map=fileMapOf(short,policy);
 return{id:short.id,path:short.path,usedBytes:short.totalBytes-short.freeBytes,totalBytes:short.totalBytes,freeBytes:short.freeBytes,fileMapBytes:map.bytes,file:map.files[0]!.name};
}

/** 📖️ Reads every mounted native shard of the workspace: its own volume (a plain directory of the nx partition is no shard), size, free bytes and top-level files. A machine without the nx partition has none. */
export function readNativeShardsV1(workspace:string,policy=NATIVE_SHARDS_POLICY_V1):readonly NativeShardFactV1[]{
 const root=join(workspace,...policy.directory.split("/"));
 if(!existsSync(root))return[];
 const pattern=new RegExp(policy.shardPattern,"u"),parent=statSync(root).dev,facts:NativeShardFactV1[]=[];
 for(const id of readdirSync(root).filter(name=>pattern.test(name)).sort()){
  const path=join(root,id,...policy.child.split("/"));
  try{
   if(statSync(path).dev===parent)continue;
   const space=statfsSync(path),files=readdirSync(path).flatMap(name=>{try{const status=statSync(join(path,name));return status.isFile()?[{name,bytes:status.size}]:[];}catch{return[];}});
   facts.push({id,path,totalBytes:Number(space.blocks)*Number(space.bsize),freeBytes:Number(space.bavail)*Number(space.bsize),files});
  }catch{continue;}
 }
 return facts;
}

/** 🔒️ The resource name and directory under which a namespace's users hold its shard, derived from the shard id exactly as the Nx graph owner derives them (a test compares both). */
export function nativeShardNamespaceV1(workspace:string,id:string,policy=NATIVE_SHARDS_POLICY_V1):Readonly<{directory:string;resource:string}>{
 const directory=join(workspace,...policy.directory.split("/"),...policy.namespaceGraphs.split("/"),id.slice(0,policy.namespaceKeyCharacters),id.slice(policy.namespaceKeyCharacters));
 return{directory,resource:policy.namespaceResourcePrefix+createHash("sha256").update(resolve(directory).replaceAll("\\","/")).digest("hex")};
}

/** ♻️ Removes the planned files of the shards that nobody holds: `hold` answers the release of the exclusive namespace lease, or `undefined` while a user has it (that shard is skipped and reported). Answers the bytes freed inside the images, the lines for the log and the shards left alone. */
export async function relieveNativeShardsV1(input:Readonly<{facts:readonly NativeShardFactV1[];hold(id:string):Promise<(()=>void)|undefined>;language?:"en"|"de"|"both";dryRun?:boolean;remove?(path:string):void;signal?:AbortSignal}>):Promise<Readonly<{freedBytes:number;lines:readonly string[];held:readonly string[]}>>{
 const language=input.language??"both",lines:string[]=[],held:string[]=[];let freedBytes=0;
 const plan=planNativeShardReliefV1(input.facts),remove=input.remove??unlinkSync;
 for(const id of[...new Set(plan.map(row=>row.id))]){
  input.signal?.throwIfAborted();
  const release=input.dryRun===true?()=>undefined:await input.hold(id);
  if(release===undefined){held.push(id);lines.push(nativeShardTextV1("held",language,{id:id.slice(0,12)}));continue;}
  try{
   for(const row of plan.filter(item=>item.id===id)){
    if(input.dryRun!==true){try{remove(join(row.path,row.name));}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")continue;throw error;}}
    freedBytes+=row.bytes;lines.push(nativeShardTextV1("freed",language,{id:id.slice(0,12),file:row.name,map:Math.round(row.bytes/1048576)}));
   }
  }finally{release();}
 }
 return{freedBytes,lines,held};
}
