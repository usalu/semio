import{planCompileUnitsV1,type CompileUnitsHostV1,type CompilerCapacityRequestV1}from"@semio-tech/process/compiler-capacity";
import{runPrintWorkersV1}from"../../../../../📦️artifacts/🟦️.ts";
/** ⚖️ The counted compiler capacity one measurement run asks for: the live host share bounded by memory and pressure and by the item count, started as soon as one core is free; the single job the machine admission grants a lone task is deliberately not a limit, because the lease below is what bounds the cores. */
export function printGalleryCapacityRequestV1(host:CompileUnitsHostV1,items:number):CompilerCapacityRequestV1{
  const decision=planCompileUnitsV1(Object.freeze({...host,grantedJobs:null,grantedMegabytes:null}),{needed:Math.max(1,items)});
  return Object.freeze({kind:"available" as const,maximumParallelism:Math.max(1,Math.min(decision.preferred,Math.max(1,items))),minimumParallelism:1});
}
/** 📈️ One finished item: how many are done of the total, which one, and the elapsed time since the first started. */
export type PrintScheduledProgressV1=Readonly<{done:number;total:number;index:number;elapsedMs:number}>;
/** 🧮️ Measures every item on at most `workers` concurrent workers and returns the results in item order, never in completion order; cancellation reaches every worker through `signal`.
 * @see ../../../../../📦️artifacts/🟦️.ts */
export async function measurePrintItemsInOrderV1<T,R>(items:readonly T[],signal:AbortSignal,workers:number,measure:(item:T,signal:AbortSignal,index:number)=>Promise<R>,completed?:(progress:PrintScheduledProgressV1)=>void|Promise<void>):Promise<readonly R[]>{
  const rows=new Array<R>(items.length),started=performance.now();let done=0;
  await runPrintWorkersV1(items,signal,async(item,actual,index)=>{rows[index]=await measure(item,actual,index);await completed?.(Object.freeze({done:++done,total:items.length,index,elapsedMs:performance.now()-started}));},workers);
  return Object.freeze(rows);
}
