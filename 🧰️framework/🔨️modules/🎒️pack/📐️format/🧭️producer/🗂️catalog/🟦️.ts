/** 🗂️ Unmounted retained Catalog source metadata stays independent of contextual code. */
import {PackError,type ValueRefusalKind,type PagedRefusalMetadata,type PagedAllocationMetadata} from "../../../⚠️error/🟦️.ts";

export type CatalogCause=Readonly<{type:"value";kind:ValueRefusalKind;reason:string}>|Readonly<{type:"paged";error:PagedRefusalMetadata}>|Readonly<{type:"allocation";error:PagedAllocationMetadata}>;
export type CatalogFault=Readonly<{cause:CatalogCause;code:string;offset:bigint}>;

/** 📋️ Lower kind, borrowed reason and actual allocation witness project through their direct factories. */
export function projectCatalogFault(fault:CatalogFault,what:string):PackError{
 switch(fault.cause.type){
  case "value":return new PackError({kind:"RetainedMalformed",refusalKind:fault.cause.kind,what,offset:fault.offset,detail:fault.cause.reason});
  case "paged":return PackError.fromPagedRefusal(fault.cause.error,what,fault.offset);
  case "allocation":return PackError.fromPagedAllocation(fault.cause.error,what,fault.offset);
 }
}
