/** 🫳️ External domain models share central own-data capture before any semantic source use. */
import {copyIfcOwnedSteps,runIfcEditAsync,IfcEditError,type IfcApplyOptions} from "../🧬️schema/🔺️diff/🟦️.ts";
import type {IfcExchangeOutcome} from "./geometry.ts";
import {captureIfcIoControlScope,ifcCheckpoint,IfcStepError,type IfcIoControl} from "./control.ts";

/** 🛂️ Capture may observe caller changes; completed source and optional domain-record indexing are private. */
export async function captureIfcExternalSource<T>(source:T,control:IfcIoControl,records?:(owned:T)=>Iterable<unknown>):Promise<IfcExchangeOutcome<T>> {
 const scope=captureIfcIoControlScope(control);control=scope.control;
 const batch=control.batchSize!,maximum=control.maxEntities!;
 const options:IfcApplyOptions={signal:control.signal,maxValues:control.maxValues,maxDepth:control.maxDepth,yieldEvery:control.batchSize,onProgress:progress=>control.progress?.({...progress,phase:"capture-external"})};
 try{
  const artifact=await runIfcEditAsync(copyIfcOwnedSteps(source,options,false,{maxProperties:control.maxValues,maxContainerDepth:control.maxDepth,plainDataOnly:true}),options);
  if(records){
   let count=0;
   for(const _record of records(artifact)){
    if(count>=maximum)throw new IfcEditError("entityLimit","External domain record count exceeds admission");
    if(count%batch===0)await ifcCheckpoint(control,"index-external",count,count);
    count++;
   }
   await ifcCheckpoint(control,"index-external",count,count);
  }
  return{artifact,fidelity:"lossless",diagnostics:[]};
 }catch(error){
  scope.rethrowCallerFailure();
  if(error instanceof IfcStepError&&error.code==="io.cancelled")throw error;
  if(!(error instanceof IfcEditError))throw error;
  if(error.code==="cancelled")throw new IfcStepError("io.cancelled",error.message);
  return{fidelity:"unsupported",diagnostics:[{code:"exchange.source",severity:"error",message:error.message,entityIds:[]}]};
 }
}
