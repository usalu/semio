import { validateJsonSchemaSubset } from "../../../../../🧬️schema/✅️validator/🟦️.ts";
import { runMutationInventoryProcess,readMutationInventoryCapture,readMutationInventoryExecutionPolicy,MutationInventoryProcessWorkspace,admitMutationInventoryProducerArguments } from "../🟦️.ts";
import type { MutationInventoryExecutionPolicy,MutationInventoryProcessOperation,MutationInventoryProcessResult } from "../🟦️.ts";
import requestSchema from "./🧬️schema/🔣️.json" with {type:"json"};
export type MutationInventoryCargoProducerRequest = Readonly<{manifestPath:string;binary:string;cwd:string}>;
/** 🏭️ Runs only the selected Rust producer with the exact authored collector and storage policy. */
export async function runMutationInventoryCargoProducer(request:MutationInventoryCargoProducerRequest,argv:readonly string[],environment:Readonly<Record<string,string|undefined>>,policy:MutationInventoryExecutionPolicy,operation:MutationInventoryProcessOperation):Promise<MutationInventoryProcessResult>{
  const errors=validateJsonSchemaSubset(requestSchema,request);
  if(errors.length)throw Error("Invalid mutation inventory Cargo producer: "+errors.join("; "));
  const args=admitMutationInventoryProducerArguments(argv,policy);
  if(environment.CARGO_TARGET_DIR!==policy.compilerStorage.targetDirectory||environment.CARGO_BUILD_BUILD_DIR!==policy.compilerStorage.buildDirectory)throw Error("Cargo producer environment differs from its authored compiler storage");
  return runMutationInventoryProcess({command:"cargo",argv:["run","--quiet","--offline","--locked","--manifest-path",request.manifestPath,"--bin",request.binary,"--features","mutation-inventory","--",...args],cwd:request.cwd,environment,captureDirectory:policy.compilerStorage.captureDirectory,budgetMs:policy.budgetMs,maximumCaptureBytes:policy.maximumCaptureBytes,compiler:{buildDirectory:policy.compilerStorage.buildDirectory,leaseDirectory:policy.compilerStorage.leaseDirectory}},operation);
}
/** 🖥️ Owns the CLI signal and deadline through compiler execution, capture reads and backpressured output. */
export async function runMutationInventoryCargoProducerCommand(request:MutationInventoryCargoProducerRequest):Promise<void>{
  const controller=new AbortController(),cancel=()=>controller.abort();
  process.once("SIGINT",cancel);process.once("SIGTERM",cancel);
  let timer:ReturnType<typeof setTimeout>|undefined;
  const write=async(stream:{write(text:string,callback:(error?:Error|null)=>void):boolean},text:string)=>{
    controller.signal.throwIfAborted();
    await new Promise<void>((accept,reject)=>{const abort=()=>reject(controller.signal.reason);controller.signal.addEventListener("abort",abort,{once:true});stream.write(text,error=>{controller.signal.removeEventListener("abort",abort);error?reject(error):accept();});if(controller.signal.aborted)abort();});
  };
  try{
    const policy=readMutationInventoryExecutionPolicy(process.env),workspace=new MutationInventoryProcessWorkspace();
    timer=setTimeout(cancel,policy.budgetMs);
    let last=0;
    const operation={signal:controller.signal,maximumUnits:policy.maximumUnits,maximumOwnedBytes:policy.maximumOwnedBytes,workspace,onProgress:async(progress:{stage:string;completed:number;capturedBytes:number})=>{if(Date.now()-last>=1000){last=Date.now();await write(process.stderr,"[mutation-inventory] "+progress.stage+" units="+progress.completed+" capturedBytes="+progress.capturedBytes+"\n");}},yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))};
    const result=await runMutationInventoryCargoProducer(request,process.argv.slice(2),process.env,policy,operation);
    if(result.reason!=="exit"||result.code!==0){process.exitCode=result.reason==="exit"&&result.code&&result.code>0?result.code:1;throw Error("Mutation inventory producer refused: "+result.reason+" code="+result.code+"; capture="+result.stderr.path);}
    await write(process.stdout,await readMutationInventoryCapture(result.stdout,operation));
  }catch(error){process.exitCode??=1;await write(process.stderr,String(error)+"\n").catch(()=>{});}
  finally{if(timer)clearTimeout(timer);process.off("SIGINT",cancel);process.off("SIGTERM",cancel);}
}

