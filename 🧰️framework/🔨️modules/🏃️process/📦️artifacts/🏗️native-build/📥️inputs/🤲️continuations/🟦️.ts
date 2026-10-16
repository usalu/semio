import type {CargoCompilerInputsViewV1} from "../🟦️.ts";
import {requireIssuedResourceControlV1,requireIssuedResourceControlRefinementV1,withIssuedResourceOperationV1} from "../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";
const maximumInputs=schema.const.maximumInputs;

/** 🤲️ Retains actual input-owner continuations until the whole driver's accepted work and consumer have settled. */
export class CargoInputContinuationTransactionV1{
 private readonly views=new Set<ResourceLeaseControl>();private accepting=true;private readonly work=new Set<Promise<void>>();private readonly releases=new Set<()=>void>();private failed=false;private failure:unknown;private closing:Promise<void>|undefined;private refusal:Promise<never>|undefined;
 constructor(private readonly control:ResourceLeaseControl){requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);}
 private refuse(error:unknown):Promise<never>{if(!this.failed){this.failed=true;this.failure=error;}this.refusal??=withIssuedResourceOperationV1(this.control,async()=>{throw error;});return this.refusal;}
 start(own:(consume:(view:CargoCompilerInputsViewV1)=>Promise<void>)=>Promise<void>,execute:(view:CargoCompilerInputsViewV1)=>Promise<void>):Promise<void>{
  if(!this.accepting||this.failed||this.work.size>=maximumInputs)return this.refuse(Error("Live finite Cargo input continuation transaction required"));let readyResolve=()=>{},readyReject:((error:unknown)=>void)=()=>{},release=()=>{},called=false,continuation:Promise<void>|undefined,duplicate=false,live=true;const ready=new Promise<void>((accept,refuse)=>{readyResolve=accept;readyReject=refuse;}),gate=new Promise<void>(done=>{release=done;});void ready.catch(()=>{});this.releases.add(release);
  const task=withIssuedResourceOperationV1(this.control,async()=>{try{await own(view=>{if(!live||called){duplicate=true;return this.refuse(Error("One live actual Cargo input owner continuation required"));}called=true;continuation=(async()=>{requireIssuedResourceControlRefinementV1(this.control,view.control);this.views.add(view.control);try{await execute(view);checkResourceLeaseControl(view.control);if(this.failed)throw this.failure;readyResolve();await gate;}finally{this.views.delete(view.control);}})();void continuation.catch(()=>{});return continuation;});if(!called)throw Error("Actual Cargo input owner continuation missing");if(continuation)await continuation;if(duplicate)throw Error("One actual Cargo input owner continuation required");}catch(error){readyReject(error);throw error;}finally{live=false;this.releases.delete(release);}});this.work.add(task);void task.then(()=>this.work.delete(task),error=>{this.work.delete(task);if(!this.failed){this.failed=true;this.failure=error;}});return ready;
 }
 check():void{checkResourceLeaseControl(this.control);for(const control of this.views)checkResourceLeaseControl(control);if(this.failed)throw this.failure;}
 close():Promise<void>{if(this.closing)return this.closing;this.accepting=false;for(const release of this.releases)release();this.closing=(async()=>{while(this.work.size)await Promise.allSettled([...this.work]);if(this.failed)throw this.failure;})();return this.closing;}
}
