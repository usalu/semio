import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import {requireIssuedResourceControlV1,withIssuedResourceOperationV1} from "../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import sourceSchema from "../../../📤️publication/📥️sources/🧬️schema/🔣️.json";
const maximumOperations=sourceSchema.properties.maximumPendingOperations.const;
/** 🪢️ Retains every genuine graph/input operation until local planning closes births and joins their actual settlement. */
export class CargoGraphOperationsV1{
 private accepting=true;private readonly pending=new Set<Promise<unknown>>();private failed=false;private failure:unknown;private closing:Promise<void>|undefined;private refusal:Promise<never>|undefined;
 constructor(private readonly control:ResourceLeaseControl){requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);}
 check():void{checkResourceLeaseControl(this.control);if(!this.accepting)throw Error("Native graph planning admission closed");}
 private observe<T>(task:Promise<T>):Promise<T>{this.pending.add(task);void task.then(()=>this.pending.delete(task),error=>{this.pending.delete(task);if(!this.failed){this.failed=true;this.failure=error;}});return task;}
 start<T>(body:()=>Promise<T>):Promise<T>{if(!this.accepting||this.pending.size>=maximumOperations||typeof body!=="function"){this.refusal??=this.observe(withIssuedResourceOperationV1(this.control,async()=>{throw Error("Live finite Native graph planning required");}));return this.refusal;}return this.observe(withIssuedResourceOperationV1(this.control,async()=>{this.check();return body();}));}
 stop():void{this.accepting=false;}
 close():Promise<void>{if(this.closing)return this.closing;this.stop();this.closing=(async()=>{while(this.pending.size)await Promise.allSettled([...this.pending]);if(this.failed)throw this.failure;})();return this.closing;}
}
