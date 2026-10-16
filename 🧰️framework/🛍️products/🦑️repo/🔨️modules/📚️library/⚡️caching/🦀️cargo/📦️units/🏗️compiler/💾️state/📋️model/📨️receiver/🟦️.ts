import{compilerStateReceiverEventV1,type RustCompilerStateReceiverV1}from"../🟦️.ts";
/** 📨️ Coordinates finite accepted work only; no resource, invocation or physical authority is created here. */
export class RustCompilerStateReceiverLedgerV1{
 private state:RustCompilerStateReceiverV1={accepting:true,pending:0,accepted:0,replacing:false};
 private pending=new Set<Promise<unknown>>();
 private outcomes:{failed:boolean;value:unknown}[]=[];
 private closing:Promise<void>|undefined;
 run<T>(kind:"read"|"replace",body:()=>Promise<T>):Promise<T>{this.state=compilerStateReceiverEventV1(this.state,kind);const outcome={failed:false,value:undefined as unknown};this.outcomes.push(outcome);let work:Promise<T>;try{work=Promise.resolve(body());}catch(error){work=Promise.reject(error);}this.pending.add(work);void work.then(()=>{this.pending.delete(work);this.state=compilerStateReceiverEventV1(this.state,kind==="replace"?"settle-replace":"settle-read");},error=>{outcome.failed=true;outcome.value=error;this.pending.delete(work);this.state=compilerStateReceiverEventV1(this.state,kind==="replace"?"settle-replace":"settle-read");});return work;}
 close():Promise<void>{if(this.closing)return this.closing;this.state=compilerStateReceiverEventV1(this.state,"close");this.closing=(async()=>{while(this.pending.size)await Promise.allSettled([...this.pending]);const failures=this.outcomes.filter(row=>row.failed).map(row=>row.value);if(failures.length)throw failures.length===1?failures[0]:new AggregateError(failures,"Accepted compiler state receiver work failed");})();this.closing.catch(()=>{});return this.closing;}
}
