import {randomBytes} from "node:crypto";
import {mkdir,readFile,readdir,rename,rm,writeFile} from "node:fs/promises";
import {basename,dirname,join} from "node:path";
import {setTimeout as delay} from "node:timers/promises";
import {acquireResourceLease,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {retireArtifactDirectoriesV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import type {RepositoryArtifactRequestV1} from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/👓️reading/📋️contract/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {componentModelGenerationPathV1,componentModelLayoutV1,componentModelRequestV1,encodeComponentModelHeadV1,readComponentModelHeadV1,readComponentModelIntentV1,type ComponentModelHeadV1,type ComponentModelLayoutV1} from "../📋️contract/🟦️.ts";
const layoutPolicy=schema.definitions.Layout.const;
export type ComponentModelHeadOptionsV1=Readonly<{leaseDirectory:string;signal:AbortSignal;control?:ResourceLeaseControl}>;

/** 🧭️ Reads the head of one plugin without any lease: the head is only ever replaced by an atomic rename, so a read sees a whole old or a whole new document; an absent head is undefined. */
export async function readComponentModelHeadFileV1(head:string,signal?:AbortSignal):Promise<ComponentModelHeadV1|undefined>{
 let text:string;try{text=await readFile(head,{encoding:"utf8",signal});}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return undefined;throw error;}
 if(Buffer.byteLength(text)>layoutPolicy.maximumHeadBytes)throw Error("Finite component model head required");return readComponentModelHeadV1(JSON.parse(text));
}

/** 🔭️ Resolves the generation the head names right now into the exact reader request; the reader then holds a lease on that generation only. */
export async function resolveComponentModelRequestV1(productsRoot:string,value:unknown,signal?:AbortSignal):Promise<RepositoryArtifactRequestV1>{
 const intent=readComponentModelIntentV1(value),head=await readComponentModelHeadFileV1(componentModelLayoutV1(productsRoot,intent).head,signal);if(!head)throw Object.assign(Error("Published component model required: "+intent.pluginId),{code:"ENOENT"});return componentModelRequestV1(productsRoot,intent,head.generation);
}

/** 🔁️ Runs a read against the resolved generation and, when it fails before the consumer started because the generation vanished (a publication retired it between the head read and the lease), re-resolves the head and retries; the consumer itself never runs twice. */
export async function withResolvedComponentModelV1<T>(productsRoot:string,value:unknown,signal:AbortSignal|undefined,read:(request:RepositoryArtifactRequestV1,started:()=>void)=>Promise<T>):Promise<T>{
 const layout=componentModelLayoutV1(productsRoot,value);
 for(let attempt=0;;attempt++){
  const request=await resolveComponentModelRequestV1(productsRoot,value,signal);
  let entered=false;
  try{return await read(request,()=>{entered=true;});}catch(error){
   if(entered)throw error;
   const moved=(await readComponentModelHeadFileV1(layout.head,signal))?.generation;
   if(attempt>=3||moved===undefined||componentModelGenerationPathV1(layout,moved)===request.path)throw error;
  }
 }
}

async function replaceFileV1(source:string,destination:string,signal:AbortSignal):Promise<void>{
 for(let attempt=0;;attempt++){
  try{await rename(source,destination);return;}catch(error){
   if(!["EPERM","EACCES","EBUSY"].includes((error as NodeJS.ErrnoException).code??"")||attempt>=5)throw error;
   await delay(25*2**attempt,undefined,{signal});
  }
 }
}

/** 🔁️ Atomically replaces the head under the writers' own short exclusive lease; readers never lease the head, so no reader of any generation can delay it. `next` maps the current head to the new one and may refuse. */
export async function swapComponentModelHeadV1(layout:ComponentModelLayoutV1,next:(current:ComponentModelHeadV1|undefined)=>ComponentModelHeadV1,options:ComponentModelHeadOptionsV1):Promise<ComponentModelHeadV1>{
 const{signal,control}=options;signal.throwIfAborted();if(control)checkResourceLeaseControl(control);
 const lease=await acquireResourceLease({directory:options.leaseDirectory,resource:"artifact:"+layout.head,mode:"exclusive",signal,control,timeoutMs:layoutPolicy.writerTimeoutMilliseconds});
 try{
  const head=next(await readComponentModelHeadFileV1(layout.head,signal)),temporary=join(layout.directory,layoutPolicy.head+"."+randomBytes(6).toString("hex")+".tmp");
  await mkdir(dirname(temporary),{recursive:true});
  try{await writeFile(temporary,encodeComponentModelHeadV1(head),{flag:"wx",signal});await replaceFileV1(temporary,layout.head,signal);}catch(error){await rm(temporary,{force:true});throw error;}
  return head;
 }finally{lease.release();}
}

/** 🚀️ Makes a fully staged generation current: the head names it, the superseded generation joins the reclamation ledger the head carries, and every ledger entry no reader holds is retired without waiting. A held generation stays in the ledger and is retried by the next publication. */
export async function promoteComponentModelGenerationV1(layout:ComponentModelLayoutV1,generation:string,options:ComponentModelHeadOptionsV1):Promise<Readonly<{head:ComponentModelHeadV1;retired:readonly string[];held:readonly string[]}>>{
 componentModelGenerationPathV1(layout,generation);
 const promoted=await swapComponentModelHeadV1(layout,current=>{const owed=[...new Set([...current?.previous??[],...current&&current.generation!==generation?[current.generation]:[]])].filter(row=>row!==generation);return readComponentModelHeadV1({version:1,generation,previous:owed.slice(-layoutPolicy.maximumPrevious)});},options);
 const outcome=await retireArtifactDirectoriesV1(promoted.previous.map(row=>componentModelGenerationPathV1(layout,row)),{leaseDirectory:options.leaseDirectory,signal:options.signal,control:options.control}),present=new Set(await readdir(layout.directory)),owed=promoted.previous.filter(row=>present.has(row));
 const head=owed.length===promoted.previous.length?promoted:await swapComponentModelHeadV1(layout,current=>readComponentModelHeadV1({version:1,generation:current?.generation??generation,previous:(current?.previous??owed).filter(row=>present.has(row))}),options);
 return Object.freeze({head,retired:Object.freeze(outcome.retired.map(path=>basename(path))),held:Object.freeze(outcome.held.map(path=>basename(path)))});
}
