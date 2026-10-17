import{AsyncLocalStorage}from"node:async_hooks";
import{availableParallelism}from"node:os";
import{resolve}from"node:path";
import{acquireCompilerCapacityLeaseV1,type CompilerCapacityLeaseV1,type CompilerAdmissionControl,type CompilerCapacityRequestV1,readCompilerCapacityRequestV1}from"../🟦️.ts";
import{checkResourceLeaseControl}from"../../../🔒️leases/🟦️.ts";
import{requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withIssuedResourceOperationV1,withRequestedIssuedResourceControlV1}from"../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🧮️ Projects a bounded child allocation from its actual retained parent, never from environment hints. */
export function compilerCapacitySubdivisionV1(parent:number,requested:CompilerCapacityRequestV1):number{if(validateJsonSchemaSubset(schema.definitions.Request,{parent,requested}).length||requested.kind==="exact"&&requested.parallelism>parent)throw Error("Compiler capacity subdivision exceeds actual parent");return requested.kind==="whole-host"?parent:requested.kind==="exact"?requested.parallelism:Math.min(parent,requested.maximumParallelism);}
/** 🎛️ Exposes immutable worker DATA while the private capacity owner remains retained. */
export type CompilerCapacityFactsV1=Readonly<{parallelism:number}>;
/** 🎟️ Requests actual capacity beneath one genuinely issued existing control. */
export type CompilerCapacityScopeOptionsV1=Readonly<{directory:string;control:CompilerAdmissionControl;environment?:Readonly<Record<string,string|undefined>>;request:CompilerCapacityRequestV1}>;
type Waiter={count:number;available:boolean;control:CompilerAdmissionControl;grant:(count:number)=>void;reject:(error:unknown)=>void;stop:()=>void};
type Frame={parent?:Frame;directory:string;control:CompilerAdmissionControl;parallelism:number;depth:number;live:boolean;accepting:boolean;failed:boolean;failure:unknown;used:number;queue:Waiter[];pending:Set<Promise<unknown>>};
const context=new AsyncLocalStorage<Frame>(),policy=Object.freeze({...schema.definitions.Policy.const});
function requireFrame(frame:Frame,control:CompilerAdmissionControl):void{for(let row:Frame|undefined=frame;row;row=row.parent)if(!row.live||!row.accepting||row.failed)throw Error("Compiler capacity scope admission closed");requireIssuedResourceControlRefinementV1(frame.control,control);}
function fail(frame:Frame,error:unknown):void{for(let row:Frame|undefined=frame;row;row=row.parent){row.accepting=false;if(!row.failed){row.failed=true;row.failure=error;}for(const waiter of row.queue.splice(0)){waiter.stop();waiter.reject(error);}}}
function pump(frame:Frame):void{while(frame.queue.length){const waiter=frame.queue[0],free=frame.parallelism-frame.used;if(!waiter||free<1||!waiter.available&&waiter.count>free)return;frame.queue.shift();waiter.stop();try{checkResourceLeaseControl(waiter.control);const count=waiter.available?Math.min(waiter.count,free):waiter.count;frame.used+=count;waiter.grant(count);}catch(error){waiter.reject(error);}}}
function reserve(frame:Frame,count:number,available:boolean,control:CompilerAdmissionControl):Promise<number>{return new Promise((grant,reject)=>{const abort=()=>{const index=frame.queue.indexOf(waiter);if(index<0)return;frame.queue.splice(index,1);waiter.stop();reject(control.signal.reason);pump(frame);},waiter:Waiter={count,available,control,grant,reject,stop:()=>control.signal.removeEventListener("abort",abort)};frame.queue.push(waiter);control.signal.addEventListener("abort",abort,{once:true});if(control.signal.aborted)abort();else pump(frame);});}

async function runFrame<T>(parent:Frame|undefined,directory:string,parallelism:number,control:CompilerAdmissionControl,body:(facts:CompilerCapacityFactsV1,control:CompilerAdmissionControl)=>Promise<T>):Promise<T>{const frame:Frame={...(parent?{parent}:{}),directory,control,parallelism,depth:(parent?.depth??0)+1,live:true,accepting:true,failed:false,failure:undefined,used:0,queue:[],pending:new Set()};let value:T|undefined,failed=false,failure:unknown;try{value=await context.run(frame,()=>body(Object.freeze({parallelism}),control));}catch(error){failed=true;failure=error;fail(frame,error);}frame.accepting=false;try{while(frame.pending.size)await Promise.allSettled([...frame.pending]);if(failed)throw failure;if(frame.failed)throw frame.failure;checkResourceLeaseControl(control);return value as T;}finally{frame.live=false;}}
/** 🪆️ Owns real kernel capacity and drains every accepted private borrower before releasing its exact credits or lease. */
export function withCompilerCapacityScopeV1<T>(options:CompilerCapacityScopeOptionsV1,body:(facts:CompilerCapacityFactsV1,control:CompilerAdmissionControl)=>Promise<T>):Promise<T>{
 const control=options.control;requireIssuedResourceControlV1(control);const parent=context.getStore();if(parent)requireFrame(parent,control);
 const directory=resolve(options.directory),requested=readCompilerCapacityRequestV1(options.request),environment=Object.freeze({...options.environment??process.env});if(typeof body!=="function")throw Error("Actual compiler capacity body required");
 if(parent){if(directory!==parent.directory)throw Error("Exact retained compiler capacity namespace required");if(parent.depth>=policy.maximumDepth||parent.pending.size>=policy.maximumAcceptedChildren||parent.queue.length>=policy.maximumQueuedChildren)throw Error("Compiler capacity scope bound exceeded");const maximum=compilerCapacitySubdivisionV1(parent.parallelism,requested);if(requested.kind==="whole-host"&&parent.parallelism!==availableParallelism())throw Error("Whole-host compiler requires the complete retained host");let parallelism=0;
  let reserved=false;const release=()=>{if(reserved){reserved=false;parent.used-=parallelism;pump(parent);}};
  const accepted=withRequestedIssuedResourceControlV1(control,0,async actual=>{if(parent.failed)throw parent.failure;await actual.publish({resource:"compiler:capacity-subdivision",mode:"shared",elapsedMs:0});if(parent.failed)throw parent.failure;parallelism=await reserve(parent,maximum,requested.kind==="available",actual);reserved=true;return runFrame(parent,directory,parallelism,actual,body);});
  const work=accepted.then(value=>{release();return value;},error=>{fail(parent,error);release();throw error;});parent.pending.add(work);void work.then(()=>{parent.pending.delete(work);},error=>{fail(parent,error);parent.pending.delete(work);});return work;
 }
 compilerCapacitySubdivisionV1(availableParallelism(),requested);
 let lease:CompilerCapacityLeaseV1|undefined;
 const accepted=withRequestedIssuedResourceControlV1(control,0,async actual=>{lease=await acquireCompilerCapacityLeaseV1({directory,control:actual,environment,request:requested});return runFrame(undefined,directory,lease.parallelism,actual,body);});
 return withIssuedResourceOperationV1(control,async()=>{let value:T|undefined,failed=false,failure:unknown;try{value=await accepted;}catch(error){failed=true;failure=error;}try{lease?.release();}catch(error){if(failed&&!Object.is(error,failure))throw new AggregateError([failure,error],"Compiler capacity work and release failed");throw error;}if(failed)throw failure;return value as T;});
}
