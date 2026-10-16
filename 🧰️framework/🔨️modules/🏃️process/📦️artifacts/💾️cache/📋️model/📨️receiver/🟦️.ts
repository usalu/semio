import{AsyncLocalStorage}from"node:async_hooks";
import{nativeArtifactCacheModelV1,type NativeArtifactCacheReceiverV1}from"../🟦️.ts";
import type{NativeArtifactCachePolicyV1}from"../../⚙️policy/🟦️.ts";
/** 📨️ Coordinates finite accepted work only; no resource, invocation or physical authority is created here. */
export class NativeArtifactCacheReceiverLedgerV1{
 private context=new AsyncLocalStorage<boolean>();
 private transition:ReturnType<typeof nativeArtifactCacheModelV1>["artifactCacheReceiverEventV1"];
 constructor(policy:NativeArtifactCachePolicyV1){this.transition=nativeArtifactCacheModelV1(policy).artifactCacheReceiverEventV1;}
 private state:NativeArtifactCacheReceiverV1={accepting:true,pending:0,accepted:0,replacing:false};
 private pending=new Set<Promise<unknown>>();
 private outcomes:{failed:boolean;value:unknown}[]=[];
 private closing:Promise<void>|undefined;
 run<T>(kind:"read"|"replace",body:()=>Promise<T>):Promise<T>{this.state=this.transition(this.state,kind);const outcome={failed:false,value:undefined as unknown};this.outcomes.push(outcome);let resolve!:(value:T|PromiseLike<T>)=>void,reject!:(error:unknown)=>void;const work=new Promise<T>((yes,no)=>{resolve=yes;reject=no;});this.pending.add(work);void work.then(()=>{this.pending.delete(work);this.state=this.transition(this.state,kind==="replace"?"settle-replace":"settle-read");},error=>{outcome.failed=true;outcome.value=error;this.state=this.transition(this.state,"close");this.pending.delete(work);this.state=this.transition(this.state,kind==="replace"?"settle-replace":"settle-read");});try{Promise.resolve(this.context.run(true,body)).then(resolve,reject);}catch(error){reject(error);}return work;}
 close():Promise<void>{if(this.context.getStore())throw Error("Cannot close cache receiver inside its accepted operation");if(this.closing)return this.closing;this.state=this.transition(this.state,"close");this.closing=(async()=>{while(this.pending.size)await Promise.allSettled([...this.pending]);const failures=this.outcomes.filter(row=>row.failed).map(row=>row.value);if(failures.length)throw failures.length===1?failures[0]:new AggregateError(failures,"Accepted compiler state receiver work failed");})();this.closing.catch(()=>{});return this.closing;}
}
