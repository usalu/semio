/** 📥️ Unmounted Source borrowed machine metadata stays independent of context prose. */
import {PackError,type ValueRefusalKind,type PagedRefusalMetadata,type PagedAllocationMetadata} from "../../../⚠️error/🟦️.ts";

export type SourceCause=Readonly<{type:"value";kind:ValueRefusalKind;reason:string}>|Readonly<{type:"paged";error:PagedRefusalMetadata}>|Readonly<{type:"allocation";error:PagedAllocationMetadata}>;
export type SourceFault=Readonly<{kind:ValueRefusalKind;reason:string;allocatedBytes:number|null}>;

/** 🪪️ Captures the exact provider kind and optional actual allocation witness. */
export function sourceFault(cause:SourceCause):SourceFault{
 switch(cause.type){
  case "value":return{kind:cause.kind,reason:cause.reason,allocatedBytes:null};
  case "paged":return{kind:cause.error.kind,reason:cause.error.reason,allocatedBytes:null};
  case "allocation":return{kind:cause.error.kind,reason:cause.error.reason,allocatedBytes:cause.error.allocatedBytes};
 }
}

/** 📤️ Preserves a zero-byte allocated witness as an allocation fault with its actual kind. */
export function projectSourceFault(fault:SourceFault,what:string,offset:bigint):PackError{
 return fault.allocatedBytes===null?new PackError({kind:"RetainedMalformed",refusalKind:fault.kind,what,offset,detail:fault.reason}):new PackError({kind:"RetainedAllocation",refusalKind:fault.kind,allocatedBytes:fault.allocatedBytes,what,offset,detail:fault.reason});
}

export type SegmentAdmission=Readonly<{kind:"fault";error:PackError}>|Readonly<{kind:"closed"|"complete"|"pending"|"inflaterBackpressure"|"ready"}>;

/** 🚦️ Borrows an existing fault before checking the scheduling state. */
export function segmentAdmission(state:Readonly<{fault:PackError|null;closed:boolean;complete:boolean;pending:boolean;inflaterBackpressure:boolean}>):SegmentAdmission{
 if(state.fault)return{kind:"fault",error:state.fault};
 if(state.closed)return{kind:"closed"};
 if(state.complete)return{kind:"complete"};
 if(state.pending)return{kind:"pending"};
 if(state.inflaterBackpressure)return{kind:"inflaterBackpressure"};
 return{kind:"ready"};
}
