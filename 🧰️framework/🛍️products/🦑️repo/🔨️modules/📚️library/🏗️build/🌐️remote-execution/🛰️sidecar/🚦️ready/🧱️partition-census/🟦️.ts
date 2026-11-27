import{lstatSync,mkdirSync,readdirSync,renameSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join,relative,sep}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

const P=schema.definitions.Policy.const;

/** 🧱️ The policy of the census: its file, interval and bounds. */
export const PARTITION_CENSUS_POLICY_V1=P;
/** 🧾️ The census of one holder, one partition and the whole machine, as the schema names them. */
export type PartitionHolderV1=Readonly<{path:string;files:number;bytes:number;allocatedBytes:number}>;
export type PartitionCensusRowV1=Readonly<{name:string;files:number;directories:number;bytes:number;allocatedBytes:number;partial:boolean;top:readonly PartitionHolderV1[]}>;
export type PartitionCensusVolumeV1=Readonly<{freeBytes:number|null;totalBytes:number|null;freeInodes:number|null;totalInodes:number|null}>;
export type PartitionCensusRecordV1=Readonly<{version:1;origin:string;platform:"darwin"|"linux"|"win32";atMs:number;elapsedMs:number;partitions:readonly PartitionCensusRowV1[];volume?:PartitionCensusVolumeV1}>;

const allocated=(bytes:number,cluster:number):number=>bytes===0?0:Math.ceil(bytes/cluster)*cluster;

/** 📊️ Counts the files, bytes (logical, and rounded up to the allocation cluster, which is what a Windows partition is charged), directories and the biggest holders (the entries two levels below the root of a row) of every partition below `<root>/.cache/b` and, after them, of every other folder of `<root>/.cache` (`cache/<name>`) and of the checkout (`root/<name>`: where a disk grows outside the partitions, such as `target`, `node_modules` or the replica), links never followed. Every row has its own entry budget (a huge partition never starves the next ones) and a fair share of the one time budget (what a row leaves unused goes to the next); a row cut short says `partial`. It yields to the event loop every `yieldEvery` entries and stops on the signal. A read only; a folder that cannot be read is absent from the list. `volume` reports what the volume of the root has free and in total, bytes and inodes.
 * @see ./🧪️tests/🟦️.ts */
export async function partitionCensusV1(root:string,options:Readonly<{origin:string;platform?:"darwin"|"linux"|"win32";signal?:AbortSignal;now?:()=>number;entryBudget?:number;timeBudgetMilliseconds?:number;yieldEvery?:number;topHolders?:number;holderKeyLimit?:number;clusterBytes?:number;extraRows?:number;volume?:()=>PartitionCensusVolumeV1|null}>):Promise<PartitionCensusRecordV1>{
 const now=options.now??Date.now,started=now(),limit=started+(options.timeBudgetMilliseconds??P.timeBudgetMilliseconds),every=options.yieldEvery??P.yieldEvery,cluster=options.clusterBytes??P.clusterBytes,keys=options.holderKeyLimit??P.holderKeyLimit,top=options.topHolders??P.topHolders,perRow=options.entryBudget??P.entryBudget,extra=options.extraRows??P.extraRows;
 let counted=0;
 const base=join(root,...P.directory),cache=join(root,P.directory[0]!),list=(path:string)=>{try{return readdirSync(path,{withFileTypes:true});}catch{return[];}};
 const folders=(path:string,skip:(name:string)=>boolean)=>list(path).filter(entry=>entry.isDirectory()&&!entry.isSymbolicLink()&&!skip(entry.name)).sort((first,second)=>first.name<second.name?-1:1);
 const targets=[...folders(base,()=>false).map(entry=>({name:entry.name,start:join(base,entry.name)})),...folders(cache,name=>name===P.directory[1]).slice(0,extra).map(entry=>({name:`cache/${entry.name}`,start:join(cache,entry.name)})),...folders(root,name=>name===P.directory[0]).slice(0,extra).map(entry=>({name:`root/${entry.name}`,start:join(root,entry.name)}))];
 const rows:PartitionCensusRowV1[]=[];
 for(const [index,target] of targets.entries()){
  const {start}=target,holders=new Map<string,{files:number;bytes:number;allocatedBytes:number}>(),share=Math.max(0,limit-now())/(targets.length-index),rowLimit=now()+share;
  let budget=perRow,files=0,directories=0,bytes=0,allocatedBytes=0,partial=false;
  const stack=[start];
  while(stack.length>0){
   if(budget<=0||now()>=rowLimit||options.signal?.aborted===true){partial=true;break;}
   const path=stack.pop()!;
   for(const child of list(path)){
    if(budget<=0){partial=true;break;}
    budget--;
    if(++counted%every===0)await new Promise<void>(done=>setImmediate(done));
    const full=join(path,child.name);
    if(child.isDirectory()&&!child.isSymbolicLink()){directories++;stack.push(full);continue;}
    let size=0;try{size=child.isSymbolicLink()?0:lstatSync(full).size;}catch{continue;}
    const rounded=allocated(size,cluster);
    files++;bytes+=size;allocatedBytes+=rounded;
    const parts=relative(start,full).split(sep),wanted=parts.slice(0,2).join("/"),key=holders.has(wanted)||holders.size<keys?wanted:parts[0]!,holder=holders.get(key)??{files:0,bytes:0,allocatedBytes:0};
    holder.files++;holder.bytes+=size;holder.allocatedBytes+=rounded;holders.set(key,holder);
   }
  }
  rows.push(Object.freeze({name:target.name.slice(0,100),files,directories,bytes,allocatedBytes,partial,top:Object.freeze([...holders].map(([path,row])=>Object.freeze({path:path.slice(0,300),...row})).sort((first,second)=>second.allocatedBytes-first.allocatedBytes||second.files-first.files||(first.path<second.path?-1:1)).slice(0,top))}));
 }
 const volume=options.volume?.()??null;
 return Object.freeze({version:1 as const,origin:options.origin,platform:options.platform??(process.platform as "darwin"|"linux"|"win32"),atMs:now(),elapsedMs:now()-started,partitions:Object.freeze(rows),...volume===null?{}:{volume}});
}

/** 📮️ Publishes the census of the machine into its store record, at most one at a time, the first one `firstDelayMilliseconds` after the start and then every `intervalMilliseconds`; a record that does not validate or cannot be written is dropped and the next interval tries again. A machine without a store says nothing.
 * @see ./🧪️tests/🟦️.ts */
export function createPartitionCensusMirrorV1(input:Readonly<{root:string;store:string|undefined;origin:string;now?:()=>number;census?:(root:string,options:{origin:string})=>Promise<PartitionCensusRecordV1>;volume?:()=>PartitionCensusVolumeV1|null;write?:(file:string,text:string)=>void}>):Readonly<{tick():Promise<boolean>}>{
 const now=input.now??Date.now,census=input.census??((root,options)=>partitionCensusV1(root,{...options,...input.volume===undefined?{}:{volume:input.volume}})),startedAt=now(),write=input.write??((file:string,text:string)=>{mkdirSync(dirname(file),{recursive:true});const temporary=`${file}.${process.pid}.tmp`;try{writeFileSync(temporary,text);renameSync(temporary,file);}finally{rmSync(temporary,{force:true});}});
 let running=false,lastAt:number|undefined;
 return Object.freeze({async tick():Promise<boolean>{
  if(input.store===undefined||input.store===""||running)return false;
  const at=now();
  if(lastAt===undefined?at-startedAt<P.firstDelayMilliseconds:at-lastAt<P.intervalMilliseconds)return false;
  running=true;
  try{
   const record=await census(input.root,{origin:input.origin});
   if(validateJsonSchemaSubset(schema.definitions.Record,record,schema).length)return false;
   write(join(input.store,"v1","origins",input.origin,P.fileName),JSON.stringify(record));
   lastAt=now();return true;
  }catch{lastAt=now();return false;}finally{running=false;}
 }});
}
