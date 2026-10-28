/** 🎛️ Original edit controls shared by canonical generators and publication barriers. */
import {hostContinuations} from '@semio-tech/framework-async/continuation';
export interface IfcEditProgress {phase:"validateBase"|"applyEntities"|"validateResult"|"admitInput"|"copyOwned"|"compare"|"validateTemplate";completed:number;total:number}
export interface IfcApplyOptions {signal?:AbortSignal;onProgress?:(progress:IfcEditProgress)=>void;maxEntities?:number;maxValues?:number;maxDepth?:number;yieldEvery?:number}
export type IfcEditSteps<T>=Generator<IfcEditProgress,T>;
/** 🛂️ Refusals leave graph ownership and publication unchanged. */
export class IfcEditError extends Error {constructor(readonly code:string,message:string,readonly target:readonly(string|number)[]=[]){super(`${code}: ${message}`);this.name="IfcEditError";}}
interface Scope {failed:boolean;failure:unknown;closing:number}
const scopes=new WeakMap<IfcApplyOptions,Scope>();
const remember=(scope:Scope,error:unknown):void=>{if(!scope.failed){scope.failed=true;scope.failure=error;}};
/** 📌️ Capture each caller property once; only private records are reused within one operation. */
export function captureIfcEditOptions(options:IfcApplyOptions={}):IfcApplyOptions{
 if(scopes.has(options))return options;
 const signal=options.signal,observer=options.onProgress,maxEntities=options.maxEntities,maxValues=options.maxValues,maxDepth=options.maxDepth,yieldEvery=options.yieldEvery,scope:Scope={failed:false,failure:undefined,closing:0};
 const captured:IfcApplyOptions=Object.freeze({signal,maxEntities,maxValues,maxDepth,yieldEvery,onProgress:observer===undefined?undefined:(progress:IfcEditProgress):void=>{if(scope.closing)return;if(scope.failed)throw scope.failure;try{observer(progress);}catch(error){remember(scope,error);throw scope.failure;}}});
 scopes.set(captured,scope);return captured;
}
/** ⛔️ Original callback failure and cancellation remain authoritative after nested continuations. */
export function checkIfcCancellation(options:IfcApplyOptions):void{const scope=scopes.get(options);if(scope?.failed)throw scope.failure;if(options.signal?.aborted)throw new IfcEditError("cancelled","IFC edit was cancelled");}
/** 🧷️ Downstream error mapping cannot replace the original observer's thrown value. */
export function throwIfcEditFailure(options:IfcApplyOptions,error:unknown):never{const scope=scopes.get(options);if(scope?.failed)throw scope.failure;throw error;}
const cadence=(options:IfcApplyOptions):number=>{const value=options.yieldEvery??256;if(!Number.isSafeInteger(value)||value<=0)throw new IfcEditError("invalidLimit","yieldEvery must be positive");return value;};
/** ▶️ Drives bounded ordinary work and drains finite entered generator cleanup without observers. */
export function runIfcEdit<T>(steps:IfcEditSteps<T>,options:IfcApplyOptions={}):T{
 let captured:IfcApplyOptions|undefined,scope:Scope|undefined,value:T|undefined,failed=false,failure:unknown;
 const retain=(error:unknown):void=>{if(scope){remember(scope,error);failure=scope.failure;}else if(!failed)failure=error;failed=true;};
 try{captured=captureIfcEditOptions(options);scope=scopes.get(captured)!;for(;;){checkIfcCancellation(captured);const step=steps.next();checkIfcCancellation(captured);if(step.done){value=step.value;break;}captured.onProgress?.(step.value);checkIfcCancellation(captured);}}catch(error){retain(error);}
 if(scope)scope.closing++;try{let step=steps.return(undefined as T);while(!step.done)step=steps.next();}catch(error){retain(error);}finally{if(scope)scope.closing--;}
 if(failed)throw failure;if(captured)checkIfcCancellation(captured);return value as T;
}
/** ⏳️ Cleanup scheduling uses the same admitted cadence and never the cancelled ordinary observer. */
export async function runIfcEditAsync<T>(steps:IfcEditSteps<T>,options:IfcApplyOptions={}):Promise<T>{
 let captured:IfcApplyOptions|undefined,scope:Scope|undefined,value:T|undefined,failed=false,failure:unknown,every=256,count=0;
 const retain=(error:unknown):void=>{if(scope){remember(scope,error);failure=scope.failure;}else if(!failed)failure=error;failed=true;};
 try{captured=captureIfcEditOptions(options);scope=scopes.get(captured)!;every=cadence(captured);for(;;){checkIfcCancellation(captured);const step=steps.next();checkIfcCancellation(captured);if(step.done){value=step.value;break;}captured.onProgress?.(step.value);checkIfcCancellation(captured);if(++count%every===0)await hostContinuations.yieldContinuation();}}catch(error){retain(error);}
 if(scope)scope.closing++;try{let step=steps.return(undefined as T);while(!step.done){if(++count%every===0)await hostContinuations.yieldContinuation();step=steps.next();}}catch(error){retain(error);}finally{if(scope)scope.closing--;}
 if(failed)throw failure;if(captured)checkIfcCancellation(captured);return value as T;
}
