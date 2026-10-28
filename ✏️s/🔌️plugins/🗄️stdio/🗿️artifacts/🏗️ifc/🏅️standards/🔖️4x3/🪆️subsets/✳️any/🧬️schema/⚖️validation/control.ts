import {hostContinuations} from '@semio-tech/framework-async/continuation';
import type {ExpressControl} from './ast.ts';

/** 🛑 Budget exhaustion and unsupported evaluation are explicit failures. */
export class ExpressEvaluationError extends Error {constructor(readonly code:string,message:string,readonly offset?:number){super(message);this.name='ExpressEvaluationError';}}

/** 🎟️ One operation retains its original grants and exact observer refusal. */
export function captureExpressControlScope(source:ExpressControl){
 const signal=source.signal,progress=source.progress,batchSize=source.batchSize,maximumWork=source.maximumWork,maximumDepth=source.maximumDepth,maximumIterations=source.maximumIterations;
 for(const [name,value] of Object.entries({batchSize,maximumWork,maximumDepth,maximumIterations}))if(value!==undefined&&(!Number.isSafeInteger(value)||value<0||name==='batchSize'&&(value===0||value>4096)||name==='maximumDepth'&&value>256))throw new ExpressEvaluationError('control',`Invalid EXPRESS ${name}`);
 let failed=false,failure:unknown;
 const rethrowCallerFailure=():void=>{if(failed)throw failure;};
 const check=(offset?:number):void=>{rethrowCallerFailure();if(signal?.aborted)throw new ExpressEvaluationError('cancelled','EXPRESS operation cancelled',offset);};
 const observe:NonNullable<ExpressControl['progress']>=row=>{check();try{progress?.(row);}catch(error){failed=true;failure=error;throw error;}check();};
 const control:ExpressControl=Object.freeze({signal,batchSize,maximumWork,maximumDepth,maximumIterations,progress:observe});
 const checkpoint=async(row:Parameters<NonNullable<ExpressControl['progress']>>[0],offset?:number):Promise<void>=>{check(offset);observe(row);await hostContinuations.yieldContinuation();check(offset);};
 return{control,check,checkpoint,rethrowCallerFailure};
}
