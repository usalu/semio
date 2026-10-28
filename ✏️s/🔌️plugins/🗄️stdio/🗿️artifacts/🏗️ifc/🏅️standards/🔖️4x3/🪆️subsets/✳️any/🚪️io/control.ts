/** 🚦️ Cooperative STEP and artifact exchange controls share original cancellation. */
import {hostContinuations} from '@semio-tech/framework-async/continuation';
export interface IfcIoControl {
 signal?:AbortSignal;
 batchSize?:number;
 progress?:(progress:{phase:string;completed:number;total:number})=>void;
 maxEntities?:number;
 maxValues?:number;
 maxDepth?:number;
 maxNumericCodeUnits?:number;
}

/** 🛑️ A refusal retains a stable diagnostic code and original input offset. */
export class IfcStepError extends Error {
 constructor(readonly code:string,message:string,readonly offset=0){super(`${code} at ${offset}: ${message}`);this.name="IfcStepError";}
}

/** 📏️ Admit explicit finite work limits before any exchange work begins. */
export function ifcIoLimit(value:number|undefined,fallback:number,name:string):number {
 if(value===undefined)return fallback;if(!Number.isSafeInteger(value)||value<1)throw new IfcStepError("io.control",`${name} requires a positive safe integer`);return value;
}

/** 🫳️ Capture the original signal, callback and finite limits before invoking caller code. */
export function captureIfcIoControl(source:IfcIoControl):Readonly<IfcIoControl> {
 const {signal,progress,batchSize:batch,maxDepth:depth,maxNumericCodeUnits:numeric,maxEntities:entities,maxValues:values}=source;
 const batchSize=Math.min(4096,ifcIoLimit(batch,256,"batchSize")),maxDepth=ifcIoLimit(depth,256,"maxDepth"),maxNumericCodeUnits=ifcIoLimit(numeric,4096,"maxNumericCodeUnits");
 if(maxDepth>256||maxNumericCodeUnits>16384)throw new IfcStepError("io.control","STEP supports depth at most 256 and numeric tokens at most 16384 code units");
 if(progress!==undefined&&typeof progress!=="function")throw new IfcStepError("io.control","progress requires a callback");
 const observe:IfcIoControl["progress"]=progress?event=>progress(event):undefined;
 return Object.freeze({signal,progress:observe,batchSize,maxDepth,maxNumericCodeUnits,maxEntities:ifcIoLimit(entities,1_000_000,"maxEntities"),maxValues:ifcIoLimit(values,10_000_000,"maxValues")});
}

/** 🧷️ Captured operation controls keep caller refusal identity separate from domain diagnostics. */
export interface IfcIoControlScope {readonly control:Readonly<IfcIoControl>;rethrowCallerFailure():void;run<T>(work:()=>Promise<T>):Promise<T>}
/** 🧯️ Preserve exact caller failures when domain errors are converted into diagnostic outcomes. */
export function captureIfcIoControlScope(source:IfcIoControl):IfcIoControlScope{
 const captured=captureIfcIoControl(source),progress=captured.progress;let failed=false,failure:unknown;
 const observe:IfcIoControl["progress"]=progress?event=>{if(failed)throw failure;try{progress(event);}catch(error){failed=true;failure=error;throw error;}}:undefined;
 const control=progress?Object.freeze({...captured,progress:observe}):captured;
 const rethrowCallerFailure=()=>{if(failed)throw failure;};
 return Object.freeze({control,rethrowCallerFailure,async run<T>(work:()=>Promise<T>){try{const result=await work();rethrowCallerFailure();return result;}catch(error){rethrowCallerFailure();throw error;}}});
}

/** ⏱️ Check cancellation before and after callbacks and the cooperative host yield. */
export async function ifcCheckpoint(control:IfcIoControl,phase:string,completed:number,total:number):Promise<void>{
 if(control.signal?.aborted)throw new IfcStepError("io.cancelled","Exchange cancelled");control.progress?.({phase,completed,total});if(control.signal?.aborted)throw new IfcStepError("io.cancelled","Exchange cancelled");await hostContinuations.yieldContinuation();if(control.signal?.aborted)throw new IfcStepError("io.cancelled","Exchange cancelled");
}

/** 🧵️ Shared evaluators expose one bounded continuation per graph edge or source member. */
export type IfcIoSteps<T>=Generator<void,T,void>;
/** 🏃️ Reference callers drain the same evaluator without scheduling host work. */
export function drainIfcIoSteps<T>(steps:IfcIoSteps<T>):T{for(;;){const step=steps.next();if(step.done)return step.value;}}
/** ⏳️ Original controls govern publication; finite entered cleanup completes without replacing prior refusals. */
export async function runIfcIoSteps<T>(steps:IfcIoSteps<T>,source:IfcIoControl,phase:string):Promise<T>{
 const scope=captureIfcIoControlScope(source),control=scope.control;return scope.run(async()=>{let completed=0,failed=false;
 try{await ifcCheckpoint(control,phase,0,0);for(;;){const step=steps.next();if(step.done){await ifcCheckpoint(control,phase,completed,completed);return step.value;}if(++completed%(control.batchSize!)===0)await ifcCheckpoint(control,phase,completed,0);}}
 catch(error){failed=true;throw error;}
 finally{try{let closing=steps.return(undefined as T),turns=0;while(!closing.done){if(++turns%(control.batchSize!)===0)try{await hostContinuations.yieldContinuation();}catch(error){if(!failed)throw error;}closing=steps.next();}}catch(error){if(!failed)throw error;}}
 });
}
