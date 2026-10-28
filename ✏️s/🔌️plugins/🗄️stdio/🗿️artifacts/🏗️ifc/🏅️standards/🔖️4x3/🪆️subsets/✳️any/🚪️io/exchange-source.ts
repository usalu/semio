/** 🫳️ IFC projections share the central owned graph admission and original operation controls. */
import {captureIfcGraphSteps,runIfcEditAsync,IfcEditError,type IfcApplyOptions} from '../🧬️schema/🔺️diff/🟦️.ts';
import type {IfcSnapshot} from '../🧬️schema/📸️snapshot/🟦️.ts';
import type {IfcExchangeOutcome} from './geometry.ts';
import {captureIfcIoControlScope,IfcStepError,type IfcIoControl} from './control.ts';

/** 🛂️ Admit only captured own data; inputs must stay unchanged during capture for exact start-state semantics. */
export async function captureIfcExchangeGraph(source:IfcSnapshot,control:IfcIoControl):Promise<IfcExchangeOutcome<IfcSnapshot>> {
 const scope=captureIfcIoControlScope(control);control=scope.control;
 const options:IfcApplyOptions={signal:control.signal,maxEntities:control.maxEntities,maxValues:control.maxValues,maxDepth:control.maxDepth,yieldEvery:control.batchSize,onProgress:progress=>control.progress?.({...progress,phase:`capture-${progress.phase}`})};
 try{return{artifact:await runIfcEditAsync(captureIfcGraphSteps(source,'validateBase',options),options),fidelity:'lossless',diagnostics:[]};}
 catch(error){
  scope.rethrowCallerFailure();
  if(!(error instanceof IfcEditError))throw error;
  if(error.code==='cancelled')throw new IfcStepError('io.cancelled',error.message);
  return{fidelity:'unsupported',diagnostics:[{code:'exchange.graph',severity:'error',message:error.message,entityIds:[]}]};
 }
}
