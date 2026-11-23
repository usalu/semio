import{withIssuedResourceOperationV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🧾️ Names every child of an aggregate failure with its message and cause chain, so the aggregate's own generic headline never hides why the consumer or the integrity recheck failed. */
export function describeAggregateChildrenV1(errors:readonly unknown[]):string{
 const chain=(error:unknown,depth:number):string=>{const text=error instanceof Error?error.message.slice(0,600):String(error);return depth<4&&error instanceof Error&&error.cause!==undefined?text+" <- "+chain(error.cause,depth+1):text;};
 return errors.map((error,index)=>"["+(index+1)+"] "+chain(error,0)).join("; ");
}

/** 🪢️ Drains bounded checks during a real control lifetime without issuing provider authority. */
export async function withWindowsCompilerLifetimeV1<T>(control:ResourceLeaseControl,verify:()=>Promise<void>,body:(recheck:()=>Promise<void>)=>Promise<T>):Promise<T>{
 checkResourceLeaseControl(control);let accepting=true,failedCheck=false,checkFailure:unknown;const pending=new Set<Promise<void>>();
 const recheck=():Promise<void>=>{if(!accepting||pending.size>=16){const error=Error("Live bounded Windows compiler custody required");if(accepting&&!failedCheck){failedCheck=true;checkFailure=error;}const refused=Promise.reject(error);void refused.catch(()=>{});return refused;}let task:Promise<void>;try{task=withIssuedResourceOperationV1(control,verify);}catch(error){if(!failedCheck){failedCheck=true;checkFailure=error;}const refused=Promise.reject(error);void refused.catch(()=>{});return refused;}pending.add(task);void task.then(()=>pending.delete(task),error=>{pending.delete(task);if(!failedCheck){failedCheck=true;checkFailure=error;}});return task;};
 await recheck();let failure:unknown,failed=false,result:T|undefined;try{result=await body(recheck);}catch(error){failed=true;failure=error;}accepting=false;await Promise.allSettled([...pending]);try{await withIssuedResourceOperationV1(control,verify);}catch(error){if(failed)throw new AggregateError([failure,error],"Windows compiler consumer and integrity recheck failed: "+describeAggregateChildrenV1([failure,error]));throw error;}if(failedCheck){if(failed&&!Object.is(failure,checkFailure))throw new AggregateError([failure,checkFailure],"Windows compiler consumer and retained check failed: "+describeAggregateChildrenV1([failure,checkFailure]));throw checkFailure;}if(failed)throw failure;return result!;
}
