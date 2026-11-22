import {watch,type FSWatcher} from "node:fs";
import {lstat} from "node:fs/promises";
import {basename,dirname} from "node:path";
import {observePhysicalFileV1} from "../../../../../📁️filesystem/🧾️observation/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../🔒️leases/🎛️control/🟦️.ts";
import limits from "./🔣️.json" with {type:"json"};
import diagnosticSchema from "./🧬️diagnostic.json" with {type:"json"};

export type CargoAdmittedSourceV1=Readonly<{path:string;digest:string}>;
export type CargoSourceDriftV1=Readonly<{path:string;expected:string;observed:string}>;

/** 🧷️ Records only the admitted path and proved digest pair in the compiler owner's existing progress stream. */
export async function publishCargoSourceDriftV1(control:Pick<ResourceLeaseControl,"publish">,mismatch:CargoSourceDriftV1):Promise<void>{
  const record={version:1,path:mismatch.path,expected:mismatch.expected,observed:mismatch.observed};
  if(record.expected===record.observed||validateJsonSchemaSubset(diagnosticSchema,record).length)return;
  try{await control.publish({resource:"cargo:running-source-digest-mismatch:"+JSON.stringify(record),mode:"shared",elapsedMs:0});}catch{}
}

/** 🔭️ Uses file events and a rolling metadata scan as hints to compare stable bytes against the admitted plan during compilation.
 * A callback returning false declines the mismatch and keeps the watcher active.
 * @see ./🔣️.json */
export function watchCargoSourceDriftV1(inputs:Iterable<CargoAdmittedSourceV1>,admissionStartedMs:number,onDrift:(mismatch:CargoSourceDriftV1)=>boolean|void):()=>Promise<void>{
  const directories=new Map<string,Map<string,CargoAdmittedSourceV1>>();
  const paths=new Map<string,CargoAdmittedSourceV1>(),watchers=new Set<FSWatcher>(),queued=new Set<CargoAdmittedSourceV1>();
  let stopped=false,selected=0,timer:ReturnType<typeof setTimeout>|undefined,reading:Promise<void>|undefined;
  let metadataTimer:ReturnType<typeof setTimeout>|undefined,scanning:Promise<void>|undefined,offset=0;
  for(const input of inputs){
    if(selected++===limits.maximumInputs)break;
    if(paths.has(input.path))continue;
    paths.set(input.path,input);
    const directory=dirname(input.path),name=basename(input.path).normalize("NFC");
    let files=directories.get(directory);
    if(!files){
      if(directories.size===limits.maximumDirectories)continue;
      directories.set(directory,files=new Map());
    }
    if(!files.has(name))files.set(name,input);
  }
  const roster=[...paths.values()],metadataCutoff=admissionStartedMs-limits.metadataClockToleranceMs;
  function enqueue(input:CargoAdmittedSourceV1):void{
    if(stopped||queued.size>=limits.maximumQueuedInputs)return;
    queued.add(input);
    schedule();
  }
  function scheduleMetadata():void{
    if(stopped||!roster.length)return;
    metadataTimer=setTimeout(()=>{
      metadataTimer=undefined;
      scanning=scanMetadata().finally(()=>{scanning=undefined;scheduleMetadata();});
    },limits.millisecondsBetweenMetadataBatches);
    metadataTimer.unref();
  }
  async function scanMetadata():Promise<void>{
    for(let count=0;count<Math.min(limits.metadataBatchSize,roster.length)&&!stopped;count++){
      const input=roster[offset]!;
      offset=(offset+1)%roster.length;
      const stat=await lstat(input.path).catch(()=>undefined);
      if(stat?.isFile()&&Math.max(stat.mtimeMs,stat.ctimeMs)>=metadataCutoff)enqueue(input);
    }
  }
  function schedule():void{
    if(stopped||timer||reading||!queued.size)return;
    timer=setTimeout(()=>{
      timer=undefined;
      const input=queued.values().next().value!;
      queued.delete(input);
      reading=check(input).finally(()=>{reading=undefined;schedule();});
    },limits.millisecondsBetweenReads);
    timer.unref();
  }
  async function check(input:CargoAdmittedSourceV1):Promise<void>{
    const end=performance.now()+limits.millisecondsPerRead;
    const claim=await observePhysicalFileV1(input.path,{
      maxBytes:limits.maximumReadBytes,
      maxWork:Math.ceil(limits.maximumReadBytes/limits.readChunkBytes)+5,
      chunkBytes:limits.readChunkBytes,
      cancelled:()=>stopped,
      remainingMs:()=>end-performance.now(),
      onProgress:()=>{},
    }).catch(()=>undefined);
    if(!stopped&&claim&&claim.sha256!==input.digest&&onDrift({path:input.path,expected:input.digest,observed:claim.sha256})!==false)close();
  }
  function close():void{
    stopped=true;
    if(timer)clearTimeout(timer);
    if(metadataTimer)clearTimeout(metadataTimer);
    queued.clear();
    for(const watcher of watchers)watcher.close();
    watchers.clear();
  }
  for(const [directory,files] of directories){
    try{
      const watcher=watch(directory,{persistent:false},(_event,name)=>{
        if(stopped||!name)return;
        const input=files.get(name.normalize("NFC"));
        if(input)enqueue(input);
      });
      watchers.add(watcher);
      watcher.on("error",()=>{watchers.delete(watcher);watcher.close();});
    }catch{}
  }
  scheduleMetadata();
  return async()=>{close();await Promise.all([reading,scanning]);};
}
