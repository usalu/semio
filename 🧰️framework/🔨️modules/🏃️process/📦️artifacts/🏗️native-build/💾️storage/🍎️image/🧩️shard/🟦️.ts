import{createHash}from"node:crypto";
import{appendFileSync,closeSync,fsyncSync,lstatSync,mkdirSync,openSync,readFileSync,rmSync,rmdirSync,statfsSync,writeFileSync}from"node:fs";
import{basename,dirname,join,posix,win32}from"node:path";
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import{validateJsonSchemaSubset}from"../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{kickNativeImageV1,nativeImageKickDueV1,type NativeImageKickHooksV1,type NativeImageKickOutcomeV1,type NativeImageKickPortV1}from"../👟️kick/🟦️.ts";
import type{NativeImageReclaimRuleV1}from"../♻️reclaim/🟦️.ts";
import{nativeImageVolumeUsedBytesSyncV1}from"../⚖️availability/🟦️.ts";
const protocol=schema.definitions.Protocol.const,idPattern=new RegExp(protocol.idPattern,"u");
type CommandRunnerV1=(program:string,args:readonly string[])=>Promise<string>;
/** 🧩️ The authored protocol of image shards: per-platform maximum image size, layout names, lifecycle commands and messages.
 * @see ./🧬️schema/🔣️.json */
export const NATIVE_IMAGE_SHARD_V1=protocol;
/** 📐️ One root of a sharded partition: the logical directory whose matching children are the keys, and the working set of one key. */
export type NativeImageShardRootV1=Readonly<{prefix:string;pattern:string;needBytes:number}>;
/** 📋️ The shard rule of a partition as its policy declares it; `enabled: false` keeps the partition one image. */
export type NativeImageShardRuleV1=Readonly<{enabled:boolean;maximumImageBytes?:number;headroomPercent:number;hostFreeReservePercent:number;roots:readonly NativeImageShardRootV1[]}>;
/** 🧾️ One live shard of the ledger: its id, key, root index, logical mount path below the home mount (kept in the log, so a later change of the rule never orphans a shard) and capacity. */
export type NativeImageShardRecordV1=Readonly<{id:string;key:string;root:number;path:string;capacityBytes:number}>;
/** 🗂️ The live shards by id: the fold of the event log. */
export type NativeImageShardStateV1=ReadonlyMap<string,NativeImageShardRecordV1>;
/** 📜️ One event of the shard log. */
export type NativeImageShardEventV1=Readonly<{type:"born";id:string;key:string;root:number;path:string;capacityBytes:number;atMs:number}|{type:"retired";id:string;atMs:number}>;
const platformKey=(platform:string):"darwin"|"linux"|"windows"|undefined=>platform==="win32"?"windows":platform==="darwin"||platform==="linux"?platform:undefined;
const alignUp=(value:number,unit:number)=>Math.ceil(value/unit)*unit,alignDown=(value:number,unit:number)=>Math.floor(value/unit)*unit;
const segmentsOf=(path:string):string[]|undefined=>{const parts=path===""?[]:path.split("/");return parts.some(part=>part===""||part==="."||part===".."||/[\\\0]/u.test(part))?undefined:parts;};
/** 🧭️ The most bytes one image of a partition may have on a platform: the platform maximum, lowered by the partition's own `maximumImageBytes`; 0 for a platform without images. */
export function nativeImageShardCapBytesV1(platform:string,rule?:Pick<NativeImageShardRuleV1,"maximumImageBytes">):number{const key=platformKey(platform);return key===undefined?0:Math.min(protocol.maximumImageBytes[key],rule?.maximumImageBytes??Infinity);}
/** 🧭️ Whether the platform can create, mount and retire shards (`hdiutil` and `diskutil` on macOS); elsewhere the partition stays one image. */
export function nativeImageShardSupportedV1(platform:string):boolean{const key=platformKey(platform);return key!==undefined&&protocol.lifecycle[key].kind!=="unsupported";}
/** 🛑️ Refuses a rule whose roots overlap (a mount point could be claimed twice), whose patterns are not anchored regular expressions or that does not match its schema. */
export function requireNativeImageShardRuleV1(value:unknown):NativeImageShardRuleV1{
 if(validateJsonSchemaSubset(schema.definitions.Rule,value,schema).length)throw Error("Closed image shard rule required");
 const rule=value as NativeImageShardRuleV1,prefixes=rule.roots.map(root=>root.prefix===""?[]:root.prefix.split("/"));
 for(const[index,root]of rule.roots.entries()){
  if(!root.pattern.startsWith("^")||!root.pattern.endsWith("$"))throw Error("Anchored shard key pattern required: "+root.pattern);
  new RegExp(root.pattern,"u");
  for(const[other,theirs]of prefixes.entries())if(other!==index&&prefixes[index]!.length<=theirs.length&&prefixes[index]!.every((part,at)=>theirs[at]===part))throw Error("Disjoint shard roots required: "+root.prefix+" and "+rule.roots[other]!.prefix);
 }
 return rule;
}
/** 🪪️ The image id of a key: the key itself when it is a short id, else `x` and the first characters of its SHA-256, so every key has a short, stable directory name. */
export function nativeImageShardIdV1(key:string):string{return idPattern.test(key)?key:protocol.hashedIdPrefix+createHash("sha256").update(key).digest("hex").slice(0,protocol.hashedIdCharacters);}
/** 🔑️ The shard key of a logical path below the mount of the home image: the first root whose prefix leads the path and whose pattern matches the next segment; `null` keeps the path in the home image. Pure and language-agnostic. */
export function nativeImageShardKeyV1(rule:Pick<NativeImageShardRuleV1,"roots">,path:string):Readonly<{root:number;key:string;id:string}>|null{
 const parts=segmentsOf(path);if(parts===undefined)return null;
 for(const[root,declared]of rule.roots.entries()){
  const prefix=declared.prefix===""?[]:declared.prefix.split("/");
  if(parts.length>prefix.length&&prefix.every((part,at)=>parts[at]===part)&&new RegExp(declared.pattern,"u").test(parts[prefix.length]!))return Object.freeze({root,key:parts[prefix.length]!,id:nativeImageShardIdV1(parts[prefix.length]!)});
 }
 return null;
}
/** 📏️ The capacity a shard is born with: as large as the cap and the share of free host space allow (the backing is sparse, so the declared size costs nothing until written), never below the key's working set plus headroom. `refused` names why nothing can be born. Pure and language-agnostic. */
export function nativeImageShardCapacityV1(platform:string,rule:Pick<NativeImageShardRuleV1,"maximumImageBytes"|"headroomPercent"|"hostFreeReservePercent">,facts:Readonly<{needBytes:number;hostFreeBytes:number}>):Readonly<{capacityBytes:number}|{refused:"cap"|"host-free"}>{
 const cap=alignDown(nativeImageShardCapBytesV1(platform,rule),protocol.alignBytes),want=Math.max(protocol.minimumImageBytes,alignUp(Math.floor(facts.needBytes*(100+rule.headroomPercent)/100),protocol.alignBytes));
 if(want>cap)return{refused:"cap"};
 const room=alignDown(Math.floor(facts.hostFreeBytes*rule.hostFreeReservePercent/100),protocol.alignBytes);
 return room<want?{refused:"host-free"}:{capacityBytes:Math.min(cap,room)};
}
/** 🎯️ The decision for one logical path: the home image, an existing shard, a shard to birth at a capacity, or a refusal (`id-collision`, `shard-limit`, `cap`, `host-free`). */
export type NativeImageShardDecisionV1=Readonly<{kind:"home"}|{kind:"existing";id:string;key:string;root:number;capacityBytes:number}|{kind:"birth";id:string;key:string;root:number;capacityBytes:number}|{kind:"refused";id:string;key:string;root:number;reason:"id-collision"|"shard-limit"|"cap"|"host-free"}>;
/** 🎯️ Decides which image holds a logical path. Disabled rules and unsupported platforms always answer home, so the layout before the switchover is the layout of a disabled rule. Pure and language-agnostic. */
export function nativeImageShardSelectV1(input:Readonly<{rule:NativeImageShardRuleV1;platform:string;live:NativeImageShardStateV1;path:string;hostFreeBytes:number}>):NativeImageShardDecisionV1{
 if(!input.rule.enabled||!nativeImageShardSupportedV1(input.platform))return{kind:"home"};
 const found=nativeImageShardKeyV1(input.rule,input.path);if(found===null)return{kind:"home"};
 const shard={id:found.id,key:found.key,root:found.root},record=input.live.get(found.id);
 if(record!==undefined)return record.key===found.key?{kind:"existing",...shard,capacityBytes:record.capacityBytes}:{kind:"refused",...shard,reason:"id-collision"};
 if(input.live.size>=protocol.maximumShards)return{kind:"refused",...shard,reason:"shard-limit"};
 const made=nativeImageShardCapacityV1(input.platform,input.rule,{needBytes:input.rule.roots[found.root]!.needBytes,hostFreeBytes:input.hostFreeBytes});
 return"refused"in made?{kind:"refused",...shard,reason:made.refused}:{kind:"birth",...shard,capacityBytes:made.capacityBytes};
}
/** 🗂️ Where one image lives: its directory (`<partition>/s/<id>`), backing, receipt, the logical mount point inside the home mount and the lease resource that guards it alone. The home image is the partition directory itself, mounted at `<directory>/m`. */
export type NativeImageShardLayoutV1=Readonly<{id:string;key:string|null;directory:string;image:string;receipt:string;mountPoint:string;leaseResource:string}>;
type FlavorV1="posix"|"win32";
const flavorOf=(flavor?:FlavorV1)=>(flavor??(process.platform==="win32"?"win32":"posix"))==="win32"?win32:posix;
/** 🏠️ The layout of the home image of a partition directory. */
export function nativeImageShardHomeLayoutV1(directory:string,flavor?:FlavorV1):NativeImageShardLayoutV1{const path=flavorOf(flavor);return Object.freeze({id:"home",key:null,directory,image:path.join(directory,protocol.imageName),receipt:path.join(directory,"image.json"),mountPoint:path.join(directory,protocol.mountDirectory),leaseResource:protocol.leasePrefix+directory});}
/** 🧩️ The layout of one shard: its mount point is the logical path of its key, so every path below it keeps its length and meaning. */
export function nativeImageShardLayoutV1(directory:string,record:Pick<NativeImageShardRecordV1,"id"|"key"|"path">,flavor?:FlavorV1):NativeImageShardLayoutV1{
 const path=flavorOf(flavor),own=path.join(directory,protocol.directory,record.id);
 return Object.freeze({id:record.id,key:record.key,directory:own,image:path.join(own,protocol.imageName),receipt:path.join(own,protocol.receiptName),mountPoint:path.join(directory,protocol.mountDirectory,...record.path.split("/")),leaseResource:protocol.leasePrefix+own});
}
/** 🧭️ The logical mount path of a key below the home mount: the prefix of its root and the key. */
export function nativeImageShardPathV1(rule:Pick<NativeImageShardRuleV1,"roots">,root:number,key:string):string{const declared=rule.roots[root];if(declared===undefined)throw Error("Declared shard root required");return declared.prefix===""?key:declared.prefix+"/"+key;}
/** 🗂️ Every image of a partition, home first, then the live shards by id. */
export function nativeImageShardLayoutsV1(directory:string,live:NativeImageShardStateV1,flavor?:FlavorV1):readonly NativeImageShardLayoutV1[]{return Object.freeze([nativeImageShardHomeLayoutV1(directory,flavor),...[...live.values()].sort((a,b)=>a.id<b.id?-1:1).map(record=>nativeImageShardLayoutV1(directory,record,flavor))]);}
/** 📏️ The 256 character rule for the files a shard adds below the workspace: the longest of its backing, receipt and the ledger against the budget, and how long the partition directory may be at most. The files below a mount point need no new rule: the mount point is the logical path itself. Pure and language-agnostic. */
export function nativeImageShardPathBudgetV1(directory:string,id:string,flavor?:FlavorV1):Readonly<{longest:number;maximumDirectoryCharacters:number;fits:boolean}>{
 const path=flavorOf(flavor),owned=[path.join(directory,protocol.directory,id,protocol.imageName),path.join(directory,protocol.directory,id,protocol.receiptName),path.join(directory,protocol.ledgerFile)],longest=Math.max(...owned.map(value=>value.length)),suffix=Math.max(...owned.slice(0,2).map(value=>path.join(protocol.directory,id,path.basename(value)).length),protocol.ledgerFile.length)+1;
 return Object.freeze({longest,maximumDirectoryCharacters:protocol.pathBudgetCharacters-suffix,fits:longest<=protocol.pathBudgetCharacters});
}
/** 🧮️ Folds the event log into the live shards. A repeated birth of the same key is idempotent (two birth attempts), a different key under a live id and a retirement of an unknown id are errors. */
export function nativeImageShardFoldV1(events:readonly NativeImageShardEventV1[]):NativeImageShardStateV1{
 const live=new Map<string,NativeImageShardRecordV1>();
 for(const event of events){
  if(validateJsonSchemaSubset(schema.definitions.Event,event,schema).length)throw Error("Closed image shard event required");
  if(event.type==="born"){const before=live.get(event.id);if(before!==undefined&&before.key!==event.key)throw Error("Image shard id "+event.id+" belongs to "+before.key);if(before===undefined)live.set(event.id,Object.freeze({id:event.id,key:event.key,root:event.root,path:event.path,capacityBytes:event.capacityBytes}));}
  else if(!live.delete(event.id))throw Error("Live image shard required to retire "+event.id);
 }
 return live;
}
/** ✍️ Validates a birth against the live shards and the rule and answers the event: the id is the one of the key, the key matches its root, the capacity is aligned and lies within the minimum and the cap, the shard limit holds. */
export function nativeImageShardBirthEventV1(input:Readonly<{rule:NativeImageShardRuleV1;platform:string;live:NativeImageShardStateV1;key:string;root:number;capacityBytes:number;atMs:number}>):NativeImageShardEventV1{
 const root=input.rule.roots[input.root],id=nativeImageShardIdV1(input.key);
 if(root===undefined||!new RegExp(root.pattern,"u").test(input.key))throw Error("Shard key of a declared root required: "+input.key);
 if(input.live.has(id))throw Error("Image shard "+id+" is live already");
 if(input.live.size>=protocol.maximumShards)throw Error("Image shard limit reached");
 if(input.capacityBytes%protocol.alignBytes!==0||input.capacityBytes<protocol.minimumImageBytes||input.capacityBytes>nativeImageShardCapBytesV1(input.platform,input.rule))throw Error("Image shard capacity within the cap required: "+input.capacityBytes);
 return Object.freeze({type:"born",id,key:input.key,root:input.root,path:nativeImageShardPathV1(input.rule,input.root,input.key),capacityBytes:input.capacityBytes,atMs:input.atMs});
}
/** 📖️ Reads the event log of a partition; a torn last line (a crash during an append) is ignored, anything else invalid is an error. */
export function readNativeImageShardLedgerV1(directory:string):readonly NativeImageShardEventV1[]{
 let text:string;try{text=readFileSync(join(directory,protocol.ledgerFile),"utf8");}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return[];throw error;}
 const complete=text.split("\n").slice(0,-1).filter(line=>line!=="");
 if(complete.length>protocol.maximumLedgerEvents)throw Error("Image shard ledger exceeds its bound");
 return Object.freeze(complete.map(line=>JSON.parse(line)as NativeImageShardEventV1));
}
/** ✍️ Appends one event as one line with one write and syncs it. */
export function appendNativeImageShardEventV1(directory:string,event:NativeImageShardEventV1):void{
 const line=JSON.stringify(event)+"\n";if(Buffer.byteLength(line)>protocol.maximumLedgerLineBytes)throw Error("Image shard event exceeds its line bound");
 mkdirSync(directory,{recursive:true});const handle=openSync(join(directory,protocol.ledgerFile),"a");try{appendFileSync(handle,line);fsyncSync(handle);}finally{closeSync(handle);}
}
/** 📊️ What one image of the partition holds right now. `volume` names the host volume that holds its backing, so images on one volume share its free bytes. */
export type NativeImageShardMeasureV1=Readonly<{id:string;volume:string;capacityBytes:number;usedBytes:number;allocatedBytes:number;hostFreeBytes:number}>;
/** 📊️ The partition seen as one: see `nativeImageShardAggregateV1`. */
export type NativeImageShardAggregateV1=Readonly<{shards:number;capacityBytes:number;usedBytes:number;allocatedBytes:number;slackBytes:number;freeBytes:number;hostFreeBytes:number;rows:readonly Readonly<{id:string;freeBytes:number}>[]}>;
/** 📊️ Adds the images of a partition up: capacity, used and allocated bytes add, the honest free bytes of the images on one host volume are their free bytes capped once by that volume's free bytes (a sum of per-image caps would count the same host space twice). Pure and language-agnostic. */
export function nativeImageShardAggregateV1(rows:readonly NativeImageShardMeasureV1[]):NativeImageShardAggregateV1{
 const volumes=new Map<string,{free:number;host:number}>();
 for(const row of rows){const inside=Math.max(0,row.capacityBytes-row.usedBytes),before=volumes.get(row.volume);volumes.set(row.volume,{free:(before?.free??0)+inside,host:Math.min(before?.host??Infinity,row.hostFreeBytes)});}
 let freeBytes=0,hostFreeBytes=0;for(const volume of volumes.values()){freeBytes+=Math.min(volume.free,volume.host);hostFreeBytes+=volume.host;}
 return Object.freeze({shards:rows.length,capacityBytes:rows.reduce((sum,row)=>sum+row.capacityBytes,0),usedBytes:rows.reduce((sum,row)=>sum+row.usedBytes,0),allocatedBytes:rows.reduce((sum,row)=>sum+row.allocatedBytes,0),slackBytes:rows.reduce((sum,row)=>sum+Math.max(0,row.allocatedBytes-row.usedBytes),0),freeBytes,hostFreeBytes,rows:Object.freeze(rows.map(row=>Object.freeze({id:row.id,freeBytes:Math.min(Math.max(0,row.capacityBytes-row.usedBytes),volumes.get(row.volume)!.host)})))});
}
/** 🚦️ One image's share of an upfront plan: the bytes its writers are expected to add, the slack it already holds, and whether it is still to be born (its birth adds `birthAllocationBytes`). */
export type NativeImageShardNeedV1=Readonly<{id:string;volume:string;needBytes:number;slackBytes:number;hostFreeBytes:number;newShard:boolean}>;
/** 🚦️ The upfront admission over all images: per host volume the expected growth (need less the credited share of the slack, plus the birth allocation of a new shard) plus the reserve against the volume's free bytes; the shortfall adds up over volumes. Pure and language-agnostic. */
export function nativeImageShardAdmissionV1(rows:readonly NativeImageShardNeedV1[],reserveBytes:number,slackCreditBasisPoints:number):Readonly<{volumes:Readonly<Record<string,Readonly<{needBytes:number;hostFreeBytes:number;shortfallBytes:number}>>>;shortfallBytes:number}>{
 const volumes=new Map<string,{need:number;host:number}>();
 for(const row of rows){const growth=Math.max(0,row.needBytes-Math.floor(row.slackBytes*slackCreditBasisPoints/10000))+(row.newShard?protocol.birthAllocationBytes:0),before=volumes.get(row.volume);volumes.set(row.volume,{need:(before?.need??0)+growth,host:Math.min(before?.host??Infinity,row.hostFreeBytes)});}
 const out:Record<string,{needBytes:number;hostFreeBytes:number;shortfallBytes:number}>={};let shortfallBytes=0;
 for(const[volume,row]of[...volumes].sort((a,b)=>a[0]<b[0]?-1:1)){const need=row.need+reserveBytes,shortfall=Math.max(0,need-row.host);out[volume]={needBytes:need,hostFreeBytes:row.host,shortfallBytes:shortfall};shortfallBytes+=shortfall;}
 return Object.freeze({volumes:Object.freeze(out),shortfallBytes});
}
/** 💬️ The bilingual line of a key that cannot get an image. */
export function nativeImageShardRefusedTextV1(language:"en"|"de",values:Readonly<{key:string;partition:string;free:string;need:string}>):string{return protocol.messages.refused[language].replace(/\{(\w+)\}/gu,(_,name:string)=>values[name as keyof typeof values]??"");}
/** 💬️ The bilingual line of a reclaim pass in which some images failed. */
export function nativeImageShardPartialTextV1(language:"en"|"de",values:Readonly<{failed:string;total:string;partition:string;ids:string}>):string{return protocol.messages.partial[language].replace(/\{(\w+)\}/gu,(_,name:string)=>values[name as keyof typeof values]??"");}
/** 🧮️ Which images a reclaim pass kicks, by the same due rule as the single image. Pure and language-agnostic. */
export function nativeImageShardReclaimDueV1(rule:NativeImageReclaimRuleV1,facts:readonly Readonly<{allocatedBytes:number;usedBytes:number;hostFreeBytes:number;sinceLastKickMilliseconds:number}>[],minimumIntervalMilliseconds?:number):readonly boolean[]{return facts.map(row=>nativeImageKickDueV1(rule,row,minimumIntervalMilliseconds));}
/** 🔌️ What the per-image reclaim touches: the measurements of one image, its nonwaiting shared lease (`undefined` when an exclusive holder has it) and its kick port (`null` where the platform has none). */
export type NativeImageShardReclaimPortsV1=Readonly<{measure(layout:NativeImageShardLayoutV1):Readonly<{allocatedBytes:number;usedBytes:number;hostFreeBytes:number;sinceLastKickMilliseconds:number}>|undefined;lease(layout:NativeImageShardLayoutV1):Promise<(()=>void)|undefined>;port(layout:NativeImageShardLayoutV1):NativeImageKickPortV1|null;kicked?(layout:NativeImageShardLayoutV1):void}>;
/** 📋️ What happened to one image in a reclaim pass. */
export type NativeImageShardReclaimRowV1=Readonly<{id:string;status:"kicked"|"not-due"|"lease-held"|"unmeasured"|"no-port"|"failed";outcome?:NativeImageKickOutcomeV1;error?:unknown}>;
/** 👟️ Kicks every due image of a partition one after the other, each under its own lease: an image whose lease is held, whose kick fails or that cannot be measured never stops the others, and the totals add up over the images that were kicked. Cancellation propagates at once. `effective` holds when at least one kicked image gave blocks back. */
export async function reclaimNativeImageShardsV1(input:Readonly<{layouts:readonly NativeImageShardLayoutV1[];rule:NativeImageReclaimRuleV1;ports:NativeImageShardReclaimPortsV1;hooks:NativeImageKickHooksV1;minimumIntervalMilliseconds?:number}>):Promise<Readonly<{rows:readonly NativeImageShardReclaimRowV1[];beforeBytes:number;afterBytes:number;settled:boolean;effective:boolean}>>{
 const rows:NativeImageShardReclaimRowV1[]=[],{hooks,ports}=input;let beforeBytes=0,afterBytes=0,settled=true,effective=false;
 for(const[index,layout]of input.layouts.entries()){
  hooks.check();
  const facts=ports.measure(layout);if(facts===undefined){rows.push({id:layout.id,status:"unmeasured"});continue;}
  if(!nativeImageKickDueV1(input.rule,facts,input.minimumIntervalMilliseconds)){rows.push({id:layout.id,status:"not-due"});continue;}
  const port=ports.port(layout);if(port===null){rows.push({id:layout.id,status:"no-port"});continue;}
  const release=await ports.lease(layout);if(release===undefined){rows.push({id:layout.id,status:"lease-held"});continue;}
  try{
   const outcome=await kickNativeImageV1(port,{check:hooks.check,...hooks.sleep===undefined?{}:{sleep:hooks.sleep},...hooks.progress===undefined?{}:{progress:(done,total)=>hooks.progress!(Math.round((index+done/total)/input.layouts.length*100),100)}});
   beforeBytes+=outcome.beforeBytes;afterBytes+=outcome.afterBytes;settled&&=outcome.settled;effective||=outcome.effective;ports.kicked?.(layout);rows.push({id:layout.id,status:"kicked",outcome});
  }catch(error){hooks.check();rows.push({id:layout.id,status:"failed",error});}
  finally{release();}
 }
 return Object.freeze({rows:Object.freeze(rows),beforeBytes,afterBytes,settled,effective});
}
/** 🪟️ The facts of a mounted image read from the host: capacity of its filesystem and the bytes its volume really uses (`diskutil`, because `statfs` free bytes are clamped by the host volume), the host blocks of its backing, the free bytes of the host volume that holds the backing and that volume's id. */
export function measureNativeImageShardV1(layout:NativeImageShardLayoutV1):NativeImageShardMeasureV1|undefined{
 try{
  const mount=statfsSync(layout.mountPoint),backing=lstatSync(layout.image),host=statfsSync(dirname(layout.image));
  return Object.freeze({id:layout.id,volume:String(backing.dev),capacityBytes:mount.blocks*mount.bsize,usedBytes:nativeImageVolumeUsedBytesSyncV1(layout.mountPoint,mount)??(mount.blocks-mount.bfree)*mount.bsize,allocatedBytes:backing.blocks*512,hostFreeBytes:Number(host.bavail)*host.bsize});
 }catch{return undefined;}
}
/** 🪟️ The cheap facts of a mounted image for pressure decisions: its capacity and free bytes by `statfs` (already capped by the host volume) without asking `diskutil`. */
export function measureNativeImageShardFreeV1(layout:NativeImageShardLayoutV1):NativeImageShardMeasureV1|undefined{
 try{const mount=statfsSync(layout.mountPoint),backing=lstatSync(layout.image),host=statfsSync(dirname(layout.image)),capacityBytes=mount.blocks*mount.bsize;return Object.freeze({id:layout.id,volume:String(backing.dev),capacityBytes,usedBytes:Math.max(0,capacityBytes-Number(mount.bavail)*mount.bsize),allocatedBytes:backing.blocks*512,hostFreeBytes:Number(host.bavail)*host.bsize});}catch{return undefined;}
}
/** 🧭️ The sharded partition a path lives in: the first ancestor named `m` whose parent holds a shard log; `undefined` for a path that is not inside a sharded partition. */
export function nativeImageShardPartitionOfV1(path:string):string|undefined{
 for(let at=path,parent=dirname(at);parent!==at;at=parent,parent=dirname(at))try{if(basename(at)===protocol.mountDirectory&&lstatSync(join(parent,protocol.ledgerFile)).isFile())return parent;}catch{}
 return undefined;
}
/** 📊️ The partition seen as one for a path inside it: the aggregate of the home image and every live shard, by the cheap measure; `undefined` when the path is not in a sharded partition or no shard is live. */
export function measureNativeImageShardRootV1(path:string):NativeImageShardAggregateV1|undefined{
 const directory=nativeImageShardPartitionOfV1(path);if(directory===undefined)return undefined;
 try{
  const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory));if(live.size===0)return undefined;
  const rows=nativeImageShardLayoutsV1(directory,live).map(measureNativeImageShardFreeV1);
  return rows.every(row=>row!==undefined)?nativeImageShardAggregateV1(rows as NativeImageShardMeasureV1[]):undefined;
 }catch{return undefined;}
}
/** 📏️ The volume facts the cache collector decides on for a sharded partition: the volume is the budget (the sum of the images' declared sizes is far larger, because every sparse image declares its cap), the used bytes are those of all images, the free bytes are what the budget leaves, never more than the honest free bytes of the aggregate. Pure and language-agnostic. */
export function nativeImageShardVolumeFactsV1(aggregate:Pick<NativeImageShardAggregateV1,"capacityBytes"|"usedBytes"|"freeBytes">,budgetBytes:number):Readonly<{volumeBytes:number;freeBytes:number}>{
 const volumeBytes=Math.max(1,budgetBytes>1?Math.min(aggregate.capacityBytes,budgetBytes):aggregate.capacityBytes);
 return Object.freeze({volumeBytes,freeBytes:Math.max(0,Math.min(aggregate.freeBytes,volumeBytes-aggregate.usedBytes))});
}
/** 🔍️ Whether a directory is the mount point of another filesystem than its parent. */
export function nativeImageShardMountedV1(mountPoint:string):boolean{try{return lstatSync(mountPoint).dev!==lstatSync(dirname(mountPoint)).dev;}catch{return false;}}
const fill=(template:readonly string[],values:Readonly<Record<string,string>>):string[]=>template.map(part=>part.replace(/<([A-Za-z]+)>/gu,(_,name)=>values[name]??_));
/** 🍎️ The effects of one shard's life on macOS: `hdiutil create` makes the case-sensitive APFS image like the home image, `diskutil image attach` mounts it through the UNMAP-capable stack at its logical mount point inside the home mount, `hdiutil detach` ends it, retiring removes its files. Every method is idempotent, so a crashed birth or retirement is finished by repeating it. See https://ss64.com/mac/hdiutil.html */
export function createNativeImageShardDarwinLifecycleV1(given:Readonly<{run:CommandRunnerV1}>){
 const mechanism=protocol.lifecycle.darwin;
 const detach=async(layout:NativeImageShardLayoutV1):Promise<void>=>{
  if(!nativeImageShardMountedV1(layout.mountPoint))return;
  try{await given.run(mechanism.detachProgram,fill(mechanism.detachArguments,{mountPoint:layout.mountPoint}));}catch(error){await given.run(mechanism.detachProgram,fill(mechanism.forceDetachArguments,{mountPoint:layout.mountPoint})).catch(()=>{throw error;});}
  if(nativeImageShardMountedV1(layout.mountPoint))throw Error("Shard "+layout.id+" is still mounted at "+layout.mountPoint);
 };
 return Object.freeze({
  async create(layout:NativeImageShardLayoutV1,capacityBytes:number):Promise<void>{
   if(capacityBytes%1048576!==0)throw Error("Whole mebibytes required");
   const seed=join(layout.directory,protocol.seedDirectory);rmSync(layout.directory,{recursive:true,force:true});mkdirSync(seed,{recursive:true});
   try{await given.run(mechanism.createProgram,fill(mechanism.createArguments,{megabytes:String(capacityBytes/1048576),volumeName:protocol.volumeName,seed,image:layout.image}));}finally{rmSync(seed,{recursive:true,force:true});}
   writeFileSync(layout.receipt,JSON.stringify({version:1,id:layout.id,key:layout.key,capacityBytes})+"\n");
  },
  async attach(layout:NativeImageShardLayoutV1,home:NativeImageShardLayoutV1):Promise<void>{
   if(!nativeImageShardMountedV1(home.mountPoint))throw Error("Mounted home image required before a shard is mounted: "+home.mountPoint);
   mkdirSync(layout.mountPoint,{recursive:true});
   if(nativeImageShardMountedV1(layout.mountPoint))return;
   await given.run(mechanism.attachProgram,fill(mechanism.attachArguments,{mountPoint:layout.mountPoint,image:layout.image}));
   if(!nativeImageShardMountedV1(layout.mountPoint))throw Error("Shard "+layout.id+" is not mounted at "+layout.mountPoint);
  },
  detach,
  async retire(layout:NativeImageShardLayoutV1):Promise<void>{
   await detach(layout);rmSync(layout.directory,{recursive:true,force:true});
   try{rmdirSync(layout.mountPoint);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
  }
 });
}
/** 🔌️ The shard lifecycle of one platform. */
export type NativeImageShardLifecycleV1=ReturnType<typeof createNativeImageShardDarwinLifecycleV1>;
type LockV1=<T>(body:()=>Promise<T>)=>Promise<T>;
/** 🌱️ Makes the image of one logical path exist and be mounted, born lazily: `home` for a path no root claims, the existing shard (mounted again when it is not), or a new one created at its capacity, mounted and logged, all inside `lock` so two commands never birth the same key. The order (create, mount, log) leaves at most an unlogged image that the next birth replaces; `repairNativeImageShardsV1` retires a logged shard whose files are gone. */
export async function ensureNativeImageShardV1(given:Readonly<{directory:string;rule:NativeImageShardRuleV1;platform:string;path:string;hostFreeBytes:()=>number;now:()=>number;lifecycle:NativeImageShardLifecycleV1;lock:LockV1;check?:()=>void}>):Promise<Readonly<{kind:"home"}|{kind:"existing"|"birth";layout:NativeImageShardLayoutV1;capacityBytes:number}|{kind:"refused";reason:string;key:string}>>{
 const check=given.check??(()=>{}),home=nativeImageShardHomeLayoutV1(given.directory);
 const choose=()=>nativeImageShardSelectV1({rule:given.rule,platform:given.platform,live:nativeImageShardFoldV1(readNativeImageShardLedgerV1(given.directory)),path:given.path,hostFreeBytes:given.hostFreeBytes()});
 const first=choose();check();
 if(first.kind==="home")return{kind:"home"};
 if(first.kind==="refused")return{kind:"refused",reason:first.reason,key:first.key};
 if(first.kind==="existing"){const layout=nativeImageShardLayoutV1(given.directory,{...first,path:nativeImageShardPathV1(given.rule,first.root,first.key)});await given.lifecycle.attach(layout,home);return{kind:"existing",layout,capacityBytes:first.capacityBytes};}
 return given.lock(async()=>{
  const again=choose();check();
  if(again.kind==="home")return{kind:"home"}as const;
  if(again.kind==="refused")return{kind:"refused",reason:again.reason,key:again.key}as const;
  const layout=nativeImageShardLayoutV1(given.directory,{...again,path:nativeImageShardPathV1(given.rule,again.root,again.key)});
  if(again.kind==="existing"){await given.lifecycle.attach(layout,home);return{kind:"existing",layout,capacityBytes:again.capacityBytes}as const;}
  const live=nativeImageShardFoldV1(readNativeImageShardLedgerV1(given.directory)),event=nativeImageShardBirthEventV1({rule:given.rule,platform:given.platform,live,key:again.key,root:again.root,capacityBytes:again.capacityBytes,atMs:given.now()});
  await given.lifecycle.detach(layout);await given.lifecycle.create(layout,again.capacityBytes);check();await given.lifecycle.attach(layout,home);check();
  appendNativeImageShardEventV1(given.directory,event);
  return{kind:"birth",layout,capacityBytes:again.capacityBytes}as const;
 });
}
/** 🗑️ Retires one shard whole: detaches it, removes its files and logs the retirement. All blocks of the key return to the host at once, with no trim. Refuses while the image is busy (the detach fails) and is repeatable after a crash. */
export async function retireNativeImageShardV1(given:Readonly<{directory:string;id:string;now:()=>number;lifecycle:NativeImageShardLifecycleV1;lock:LockV1}>):Promise<boolean>{
 return given.lock(async()=>{
  const record=nativeImageShardFoldV1(readNativeImageShardLedgerV1(given.directory)).get(given.id);if(record===undefined)return false;
  await given.lifecycle.retire(nativeImageShardLayoutV1(given.directory,record));appendNativeImageShardEventV1(given.directory,{type:"retired",id:given.id,atMs:given.now()});return true;
 });
}
/** 🩹️ After a restart or a reboot: mounts every live shard again and retires the ones whose files are gone (recomputable cache), answering what it did per id. */
export async function repairNativeImageShardsV1(given:Readonly<{directory:string;now:()=>number;lifecycle:NativeImageShardLifecycleV1;lock:LockV1;check?:()=>void}>):Promise<Readonly<Record<string,"mounted"|"retired">>>{
 const home=nativeImageShardHomeLayoutV1(given.directory),done:Record<string,"mounted"|"retired">={};
 return given.lock(async()=>{
  for(const record of nativeImageShardFoldV1(readNativeImageShardLedgerV1(given.directory)).values()){
   given.check?.();const layout=nativeImageShardLayoutV1(given.directory,record);
   let present=true;try{lstatSync(layout.image);}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;present=false;}
   if(present){await given.lifecycle.attach(layout,home);done[record.id]="mounted";}
   else{await given.lifecycle.retire(layout);appendNativeImageShardEventV1(given.directory,{type:"retired",id:record.id,atMs:given.now()});done[record.id]="retired";}
  }
  return Object.freeze(done);
 });
}
