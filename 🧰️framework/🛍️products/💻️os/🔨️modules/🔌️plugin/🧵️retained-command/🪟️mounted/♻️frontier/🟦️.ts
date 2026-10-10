import {createCompletionFault} from "../../../📬️completion/⚠️fault/🟦️.ts";
export type PublicationLane="artifact"|"config"|"draft"|"presence"|"transient"|"windowConfig"|"windowTransient";
export type PublicationOutcomePhase="uncaptured"|"captured"|"released"|"delivered";
/** 📬️ Retains the borrowed rejection outcome until its original publication body and frame are released. */
export class PublicationOutcomeCursor{
 private phase:PublicationOutcomePhase="uncaptured";
 private report:string|null=null;
 constructor(private readonly lane:PublicationLane,private readonly reason:string|null,private readonly carrierCopyBytes:number){if(!Number.isSafeInteger(carrierCopyBytes)||carrierCopyBytes<=0)throw new RangeError("Publication report carrier needs its concrete native extent");}
 get state():PublicationOutcomePhase{return this.phase;}
 get nextCopyBytes():number{return (this.phase==="uncaptured"&&this.reason!==null&&this.lane!=="windowTransient")||(this.phase==="released"&&this.report!==null)?this.carrierCopyBytes:0;}
 capture(items:number,copyBytes:number,depth:number):boolean{if(this.phase!=="uncaptured"||items<1||copyBytes<this.nextCopyBytes||depth<1)return false;if(this.reason!==null&&this.lane!=="windowTransient")this.report=createCompletionFault({origin:"framework",code:"interactive-job.publication-rejected",severity:"error",message:this.reason,retryable:false}).report;this.phase="captured";return true;}
 releaseFrame(bodyEmpty:boolean,frameBytes:number,items:number,releaseBytes:number,depth:number):boolean{if(this.phase!=="captured"||!bodyEmpty||items<1||releaseBytes<frameBytes||depth<1)return false;this.phase="released";return true;}
 deliver(existing:string|null,items:number,copyBytes:number,depth:number):{ready:boolean;fault:string|null}{if(this.phase!=="released"||items<1||copyBytes<this.nextCopyBytes||depth<1)return {ready:false,fault:existing};this.phase="delivered";const fault=existing??this.report;this.report=null;return {ready:true,fault};}
}
