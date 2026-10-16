import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import maintenance from"../🧬️schema/🔣️.json"with{type:"json"};
export const metadataBatchSizeV1=schema.definitions.Protocol.const.maximumAcceptedMetadata;
/** 📚️ Drains every accepted reader before ordered reduction or the first input-ordered rejection. */
export async function forEachMetadataBatchV1<T,R>(items:readonly T[],read:(item:T,index:number)=>Promise<R>,reduce:(item:T,value:R,index:number)=>void|Promise<void>):Promise<void>{
 for(let offset=0;offset<items.length;offset+=metadataBatchSizeV1){const batch=items.slice(offset,offset+metadataBatchSizeV1),results=await Promise.allSettled(batch.map((item,index)=>Promise.resolve().then(()=>read(item,offset+index))));for(const result of results)if(result.status==="rejected")throw result.reason;for(let index=0;index<batch.length;index++){const result=results[index]!;if(result.status!=="fulfilled")throw Error("Settled metadata result required");const reduced=reduce(batch[index]!,result.value,offset+index);if(reduced)await reduced;}}
}
/** 🚦️ Serializes the unchanged continuation interval before any later metadata admission. */
export class MetadataContinuationBarrierV1{
 private operations=0;
 private pending:Promise<void>|undefined;
 private failed=false;
 private failure:unknown;
 checkpoint(run:()=>Promise<void>):Promise<void>|undefined{
  if(this.failed)throw this.failure;if(this.pending)return this.pending.then(()=>this.checkpoint(run));if(++this.operations%maintenance.definitions.Policy.const.maximumOperationsPerContinuation!==0)return;
  const accepted=Promise.resolve().then(run),pending=accepted.then(()=>{if(this.pending===pending)this.pending=undefined;},error=>{this.failed=true;this.failure=error;if(this.pending===pending)this.pending=undefined;throw error;});this.pending=pending;return pending;
 }
}
