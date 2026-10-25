import receiverSchema from"../📥️receiver/🧬️schema/🔣️.json" with{type:"json"};
import operationSchema from "../📥️operation/🧬️schema/🔣️.json" with {type:"json"};
import commandSchema from "../🎛️command/🧬️schema/🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import continuation from "../🧬️schema/📥️continuation.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🪙️ Describes already accepted finite time; this immutable data is never an execution grant. */
export type NativeImageCleanupFundingV1=Readonly<{version:1;acceptedAtMilliseconds:number;wholeDeadlineMilliseconds:number;bodyDeadlineMilliseconds:number;reserveMilliseconds:10000}>;
/** ⚠️ Names every independently retained settlement failure role. */
export type NativeImageCleanupFailureV1="body"|"progress"|"cleanup"|"handoff"|"guard"|"admission";
/** 🗂️ Preserves actual failure values, including thrown undefined, in settlement order. */
export type NativeImageCleanupFailureRecordV1=Readonly<{role:NativeImageCleanupFailureV1;cause:unknown}>;
/** 📋️ Keeps physical and pending-work evidence without carrying a resource or command capability. */
export type NativeImageCleanupStateV1=Readonly<{received:boolean;enrolled:boolean;owned:boolean;stopped:boolean;closing:boolean;persistent:boolean;transferred:boolean;refused:boolean;acceptedWork:number;activationPending:boolean;progressPending:boolean;deactivationPending:boolean;deactivationAttempts:0|1;physical:"unknown"|"active"|"inactive";changed:boolean;expired:boolean;guardFailed:boolean;failures:readonly NativeImageCleanupFailureV1[]}>;
/** 📎️ Describes settlement obligations while leaving actual authority in the canonical issuer. */
export type NativeImageCleanupTraceV1=Readonly<{deactivationAttempts:0|1;bodyAdmissionsAfterStop:0;guardRelease:"after-physical-settlement"|"after-unresolved-handoff"|"persistent-reader-release"|"held"|"never-acquired";settlement:"physical"|"persistent"|"unresolved"|"refused";failures:readonly NativeImageCleanupFailureV1[];absoluteDeadlineRenewed:false}>;

/** 🌱️ Begins unaccepted evidence; no receipt, signal or structural control can activate it. */
export function nativeImageCleanupStateV1():NativeImageCleanupStateV1{return Object.freeze({received:false,enrolled:false,owned:false,stopped:false,closing:false,persistent:false,transferred:false,refused:false,acceptedWork:0,activationPending:false,progressPending:false,deactivationPending:false,deactivationAttempts:0,physical:"unknown",changed:false,expired:false,guardFailed:false,failures:Object.freeze([])});}

/** 🖼️ Computes preaccepted funding data without minting an Original or cleanup capability. */
export function nativeImageCleanupFundingV1(value:unknown):NativeImageCleanupFundingV1{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Finite closed image cleanup funding required");
 const row=value as {version:1;acceptedAtMilliseconds:number;wholeDeadlineMilliseconds:number;reserveMilliseconds:10000},bodyDeadlineMilliseconds=row.wholeDeadlineMilliseconds-row.reserveMilliseconds;
 if(bodyDeadlineMilliseconds<=row.acceptedAtMilliseconds)throw Error("Original image operation cannot fund its terminal reserve");
 return Object.freeze({...row,bodyDeadlineMilliseconds});
}

/** ⏱️ Seals one terminal phase within the already accepted whole-operation absolute bound. */
export function nativeImageCleanupTerminalDeadlineV1(funding:NativeImageCleanupFundingV1,closingMilliseconds:number):number{
 if(validateJsonSchemaSubset(schema.definitions.Funding,funding,schema).length||funding.bodyDeadlineMilliseconds!==funding.wholeDeadlineMilliseconds-funding.reserveMilliseconds||funding.bodyDeadlineMilliseconds<=funding.acceptedAtMilliseconds||!Number.isFinite(closingMilliseconds)||closingMilliseconds<funding.acceptedAtMilliseconds)throw Error("Actual accepted image cleanup funding facts required");
 const deadline=Math.min(funding.wholeDeadlineMilliseconds,closingMilliseconds+funding.reserveMilliseconds);if(deadline<=closingMilliseconds)throw Error("Original image cleanup absolute deadline exhausted");return deadline;
}

/** 🔄️ Reduces accepted evidence events; consumers must separately prove private issuance before any effect. */
export function nativeImageCleanupTransitionV1(state:NativeImageCleanupStateV1,event:string):NativeImageCleanupStateV1{
 if(!continuation.definitions.Case.properties.schedule.items.enum.includes(event))throw Error("Closed image cleanup transition required");
 const next={...state,failures:[...state.failures]},failure=(role:NativeImageCleanupFailureV1)=>{if(!next.failures.includes(role))next.failures.push(role);};
 if(next.refused)throw Error("Refused image cleanup evidence cannot be renewed");
 if(event==="receive-original")next.received=true;
 else if(event==="present-copied-grant"||event==="refuse-expired-original"){next.refused=true;failure("admission");}
 else if(event==="enroll-funded-cleanup"){if(!next.received||next.enrolled)throw Error("One received cleanup enrollment required");next.enrolled=true;}
 else if(event==="create-owned-backing"){if(!next.enrolled||next.stopped||next.owned)throw Error("One accepted live image backing required before creation");next.owned=true;next.physical="inactive";}
 else if(event==="submit-activation"){if(!next.owned||next.stopped)throw Error("Accepted live image backing required before activation");next.activationPending=true;}
 else if(event==="join-activation-child")next.activationPending=false;
 else if(event==="submit-body-work"){if(!next.enrolled||next.stopped)throw Error("Live accepted image body work required");next.acceptedWork++;}
 else if(event==="join-body-work"){if(next.acceptedWork<1)throw Error("Actual accepted image work join required");next.acceptedWork--;}
 else if(event==="activation-child-fails")failure("body");
 else if(event==="observe-exact-active")next.physical="active";
 else if(event==="observe-exact-inactive")next.physical="inactive";
 else if(event==="publish-body-progress"){if(next.stopped)throw Error("Image body progress admission closed");next.progressPending=true;}
 else if(event==="join-body-progress")next.progressPending=false;
 else if(event==="reject-body-progress"){failure("progress");next.stopped=true;}
 else if(event==="cancel-body"||event==="expire-body-cutoff"){failure("body");next.stopped=true;}
 else if(event==="fail-persistent-publication"){failure("handoff");next.stopped=true;}
 else if(event==="accept-persistent-handoff"){if(!next.enrolled||next.closing||next.activationPending||next.physical!=="active"||next.changed)throw Error("Settled exact persistent handoff required");next.persistent=true;}
 else if(event==="close-first"||event==="close-second"||event==="close-after-settlement"){if(!next.enrolled)throw Error("Accepted image cleanup enrollment required");next.stopped=true;next.closing=true;}
 else if(event==="replace-backing"){next.changed=true;failure("cleanup");}
 else if(event==="fail-observation"){next.physical="unknown";failure("cleanup");}
 else if(event==="fail-deactivation"||event==="observe-still-active"){next.physical="active";failure("cleanup");}
 else if(event==="join-deactivation-child")next.deactivationPending=false;
 else if(event==="expire-cleanup-absolute-bound"){next.expired=true;if(next.progressPending)failure("progress");failure("cleanup");}
 else if(event==="accept-unresolved-handoff"){if(!next.closing||next.acceptedWork||next.activationPending||next.progressPending||next.deactivationPending||!next.failures.length)throw Error("Settled accepted unresolved ownership transfer required");next.transferred=true;}
 else if(event==="fail-guard-release"){next.guardFailed=true;failure("guard");}
 if(next.closing&&!next.persistent&&!next.transferred&&!next.acceptedWork&&!next.activationPending&&!next.progressPending&&!next.deactivationPending&&!next.changed&&!next.expired&&!next.failures.includes("cleanup")&&next.physical==="active"&&next.deactivationAttempts===0){next.deactivationAttempts=1;next.deactivationPending=true;}
 return Object.freeze({...next,failures:Object.freeze(next.failures)});
}

/** 🧾️ Projects every failure and guard obligation without releasing physical custody. */
export function nativeImageCleanupResultV1(state:NativeImageCleanupStateV1):NativeImageCleanupTraceV1{
 const pending=state.acceptedWork>0||state.activationPending||state.progressPending||state.deactivationPending,settlement=state.refused?"refused":state.guardFailed?"unresolved":state.persistent&&state.closing?"persistent":state.closing&&!pending&&!state.changed&&state.physical==="inactive"?"physical":"unresolved",guardRelease=state.refused?"never-acquired":state.guardFailed||pending?"held":settlement==="persistent"?"persistent-reader-release":settlement==="physical"?"after-physical-settlement":state.transferred?"after-unresolved-handoff":"held";
 return Object.freeze({deactivationAttempts:state.deactivationAttempts,bodyAdmissionsAfterStop:0,guardRelease,settlement,failures:state.failures,absoluteDeadlineRenewed:false});
}

/** 📚️ Projects a complete language-neutral schedule; serialized evidence never becomes authority. */
export function nativeImageCleanupTraceV1(schedule:readonly string[]):NativeImageCleanupTraceV1{return nativeImageCleanupResultV1(schedule.reduce(nativeImageCleanupTransitionV1,nativeImageCleanupStateV1()));}

/** 🧯️ Reports retained failures only after the actual physical owner has settled or transferred. */
export function throwNativeImageCleanupFailuresV1(failures:readonly NativeImageCleanupFailureRecordV1[]):void{if(failures.length===1)throw failures[0]!.cause;if(failures.length>1)throw new AggregateError(failures.map(row=>row.cause),"Accepted image body and settlement failures: "+failures.map(row=>row.role).join(", "));}

/** 🧭️ Names the next bounded evidence action; serialized facts never issue command custody. */
export function nativeImageCleanupActionV1(value:unknown):string{
 if(validateJsonSchemaSubset(commandSchema.definitions.Evidence,value).length)throw Error("Closed cleanup command evidence required");const row=value as {platform:string;phase:string;creator:string;mapping:string;pending:number;deactivations:number;observations:number;persistent:boolean};if(row.platform!=="darwin")return "refuse-platform";if(row.pending)return "join-accepted";if(row.phase==="body")return "stop-body";if(row.creator==="none")return "settled-no-effects";if(row.creator!=="exact")return "retain-unresolved";if(row.persistent)return row.mapping==="exact-active"&&row.observations>0&&row.deactivations===0?"persistent-transfer":"retain-unresolved";if(row.observations===0)return "observe";if(row.deactivations===1&&row.observations===1)return "verify-inactive";if(row.mapping==="inactive")return "remove-created";if(row.mapping==="exact-active"&&row.deactivations===0&&row.observations===1)return "deactivate";return "retain-unresolved";
}

/** 🧮️ Projects finite Native operation admission data without granting a physical owner. */
export function nativeImageOperationAdmissionV1(value:unknown):Readonly<{accepts:boolean;mustDrain:boolean}>{if(validateJsonSchemaSubset(operationSchema.definitions.Admission,value).length)throw Error("Closed finite Native operation data required");const row=value as {pending:number;topOpen:boolean;nestedLive:boolean};return Object.freeze({accepts:(row.topOpen||row.nestedLive)&&row.pending<operationSchema.definitions.Protocol.const.maximumPendingOperations,mustDrain:row.pending>0});}

/** 📬️ Describes closed receiver transitions without issuing a physical owner or guard. */
export function nativeImageReceiverDecisionV1(value:unknown):string{if(validateJsonSchemaSubset(receiverSchema.definitions.Evidence,value,receiverSchema).length)throw Error("Closed Native receiver evidence required");const row=value as{role:string;guard:string;mapping:string;receipt:string;creator:string;pending:number;bodyLive:boolean;openingLive:boolean;receiverOwnsGuard:boolean;acknowledged:boolean;activationAccepted:boolean;verified:boolean};if(row.pending)return"join-accepted";if(!row.openingLive)return"retain-unresolved";if(row.role==="mounted")return row.guard==="shared"&&row.receipt==="existing"&&row.mapping==="active"&&row.receiverOwnsGuard&&row.verified?"ready":"refuse";if(row.acknowledged){if(!row.receiverOwnsGuard||row.creator!=="exact"||!["created","existing"].includes(row.receipt)||row.mapping!=="active")return"retain-unresolved";return row.guard==="exclusive"?"transfer-to-shared":row.guard==="none"?"acquire-shared":row.verified?"ready":"verify-receiver";}if(!row.bodyLive)return"cleanup-before-ack";if(row.guard!=="exclusive"||row.receipt==="changed"||row.role==="fresh"&&row.receipt==="existing"||row.role==="inactive"&&(row.receipt!=="existing"||row.creator!=="exact"))return"refuse";if(row.creator==="unknown"||row.mapping==="ambiguous")return"retain-unresolved";if(row.role==="fresh"&&row.creator==="none")return row.receipt==="absent"&&row.mapping==="unknown"?"create":"refuse";if(row.activationAccepted&&row.mapping==="unknown")return"observe-after-effect";if(row.mapping==="unknown")return"observe-before-effect";if(row.mapping==="inactive"&&!row.activationAccepted)return"activate";if(row.mapping==="active"&&row.activationAccepted)return row.receiverOwnsGuard&&["created","existing"].includes(row.receipt)?"acknowledge":row.receipt==="created"?"refuse":"construct-receiver";return"refuse";}

const preparationSchema=structuredClone(schema.definitions.Preparation);
/** 🧲️ Selects a finite preparation obligation; only real provider custody can execute it. */
export function nativeImagePreparationDecisionV1(value:unknown):"drain"|"settle-failure"|"handoff"|"retain"{if(validateJsonSchemaSubset(preparationSchema,value).length)throw Error("Closed finite Native preparation evidence required");const row=value as{failed:boolean;pendingCommands:number;pendingOperations:number;acknowledged:boolean;bodyOpen:boolean};return row.pendingCommands||row.pendingOperations?"drain":row.failed?"settle-failure":row.acknowledged&&!row.bodyOpen?"handoff":"retain";}

const successfulChildSchema=structuredClone(schema.definitions.SuccessfulChildRetirement);
/** ♻️ Describes complete terminal obligations; neutral evidence issues no physical retirement authority. */
export function nativeSuccessfulChildRetirementDecisionV1(value:unknown):boolean{if(validateJsonSchemaSubset(successfulChildSchema,value).length)throw Error("Closed successful child retirement evidence required");const row=value as Record<string,unknown>;return row.phase==="cleanup"&&successfulChildSchema.required.every(key=>key==="version"||key==="phase"||row[key]===true);}

/** 💻️ Joins terminal close after body settlement and preserves the first thrown value, including undefined. */
export async function withNativeImageTerminalCloseV1<T>(body:()=>Promise<T>,close:()=>Promise<void>):Promise<T>{if(typeof body!=="function"||typeof close!=="function")throw Error("Actual body and terminal close required");let value:T|undefined,failed=false,failure:unknown;try{value=await body();}catch(error){failed=true;failure=error;}try{await close();}catch(error){if(!failed){failed=true;failure=error;}}if(failed)throw failure;return value as T;}
