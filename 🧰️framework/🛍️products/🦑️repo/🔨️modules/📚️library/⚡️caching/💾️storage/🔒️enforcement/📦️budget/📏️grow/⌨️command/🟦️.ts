import {existsSync,readFileSync} from "node:fs";
import {dirname,join} from "node:path";
import budgetPolicy from "../../🔣️policy.json" with {type:"json"};
import cachePolicy from "../../../../../🔣️policy.json" with {type:"json"};
import {GROW_PARTITIONS_V1,GrowRefusalV1,growPartitionV1,growTargetBytesV1,growTextV1,readGrowJournalV1,realGrowToolsV1,type GrowLanguageV1,type GrowPortsV1} from "../🟦️.ts";

/** 📝️ The closed arguments of `repo cache grow`: the partition, an optional exact target and whether only to plan. */
export type CacheGrowArgumentsV1=Readonly<{partition:string;bytes:number|undefined;dryRun:boolean;stopCollectors:boolean}>;
/** 🧷️ Reads `grow --partition=<id> [--bytes=<n>] [--dry-run]`; every unknown, repeated or malformed option is refused in both languages. */
export function cacheGrowArgumentsV1(args:readonly string[]):CacheGrowArgumentsV1{
 const both=(key:string,variables:Readonly<Record<string,string|number>>={})=>growTextV1("en",key,variables)+"\n"+growTextV1("de",key,variables);
 let partition:string|undefined,bytes:number|undefined,dryRun=false;const stopCollectors=true;const seen=new Set<string>();
 for(const argument of args.slice(1)){
  const cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut),value=cut<0?undefined:argument.slice(cut+1);
  if(seen.has(name))throw Error(both("refused.unsupported",{partition:name,supported:"once"}));seen.add(name);
  if(name==="--partition"&&value)partition=value;
  else if(name==="--bytes"&&value&&/^[1-9][0-9]*$/u.test(value))bytes=Number(value);
  else if(name==="--dry-run"&&value===undefined)dryRun=true;
  else if(name!=="--lang"&&name!=="--json")throw Error(both("refused.unsupported",{partition:argument,supported:Object.keys(GROW_PARTITIONS_V1).join(", ")}));
 }
 if(!partition)throw Error(both("refused.unsupported",{partition:"",supported:Object.keys(GROW_PARTITIONS_V1).join(", ")}));
 return Object.freeze({partition,bytes,dryRun,stopCollectors});
}

/** 🔌️ What the command needs from the running CLI: the workspace, the output, the free bytes of the host volume and the exclusive lease of an image. */
export type CacheGrowContextV1=Readonly<{workspace:string;write(line:string):void;freeBytes(path:string):number|undefined;lease(directory:string,report:(line:string)=>void):Promise<Readonly<{release():void}>>;tools?:GrowPortsV1["tools"];signal?:AbortSignal;checkpoint?:GrowPortsV1["checkpoint"]}>;
const gib=(bytes:number)=>(bytes/2**30).toFixed(1)+" GiB";

/** 🌱️ Runs `repo cache grow`: bounds the target by the policy-declared size, the elastic rule maximum and the host free-space reserve, then grows the image offline under the exclusive image lease (see {@link growPartitionV1}). A dry run only states the plan. */
export async function runCacheGrowV1(args:readonly string[],context:CacheGrowContextV1,language:GrowLanguageV1):Promise<number>{
 const parsed=cacheGrowArgumentsV1(args),supported=GROW_PARTITIONS_V1[parsed.partition],write=(line:string)=>context.write(line);
 try{
  if(!supported)throw new GrowRefusalV1("unsupported",{partition:parsed.partition,supported:Object.keys(GROW_PARTITIONS_V1).join(", ")});
  const budget=(budgetPolicy.partitions as Record<string,{directory:string;maximumLogicalBytes:number}>)[parsed.partition]!,rule=(budgetPolicy.elastic as Record<string,{maximumBytes:number;hostFreeReservePercent:number}>)[supported.elastic]!,strict=(cachePolicy.storage as unknown as Record<string,{filesystemBytes:number}>)[supported.strict]!;
  const directory=join(context.workspace,budget.directory),receiptPath=join(directory,"image.json");
  if(!existsSync(receiptPath)||!existsSync(join(directory,"fixed.dmg")))throw new GrowRefusalV1("absent",{partition:parsed.partition,directory});
  const journal=readGrowJournalV1(directory),current=(JSON.parse(readFileSync(receiptPath,"utf8")) as {filesystemBytes:number}).filesystemBytes,declared=Math.min(strict.filesystemBytes,budget.maximumLogicalBytes),free=context.freeBytes(dirname(directory))??0;
  const tools=context.tools??realGrowToolsV1(context.signal),target=journal?{targetBytes:journal.toBytes,allowedBytes:0}:growTargetBytesV1({currentBytes:current,declaredBytes:declared,ruleMaximumBytes:rule.maximumBytes,hostFreeBytes:free,hostFreeReservePercent:rule.hostFreeReservePercent,...parsed.bytes===undefined?{}:{requestedBytes:parsed.bytes}});
  write(growTextV1(language,"plan",{partition:parsed.partition,from:gib(current),to:gib(target.targetBytes),free:gib(free),declared:gib(declared),maximum:gib(rule.maximumBytes)}));
  if(parsed.dryRun){write(growTextV1(language,"plan.dry",{partition:parsed.partition}));return 0;}
  const outcome=await growPartitionV1({partition:parsed.partition,directory,targetBytes:target.targetBytes,language,stopCollectors:parsed.stopCollectors},{lease:()=>context.lease(directory,write),tools,report:write,...context.signal?{signal:context.signal}:{},...context.checkpoint?{checkpoint:context.checkpoint}:{}});
  return outcome.toBytes===target.targetBytes?0:1;
 }catch(error){
  if(error instanceof GrowRefusalV1){write(error.render(parsed.partition));return 1;}
  throw error;
 }
}
