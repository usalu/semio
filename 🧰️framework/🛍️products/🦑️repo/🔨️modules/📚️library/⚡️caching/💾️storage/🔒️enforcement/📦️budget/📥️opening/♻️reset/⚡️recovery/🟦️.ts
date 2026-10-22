import {requireIssuedResourceControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import type {NativeUnsupportedPublicationV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/❓️unsupported/🟦️.ts";
/** 🔎️ Records only bounded observations; the reset owner independently rechecks exact physical custody. */
export type InactiveRecoveryReasonV1=NativeUnsupportedPublicationV1|Readonly<{version:1;kind:"current-reset-intent";directory:string;receiptStamp:string;receiptSha256:string}>;
/** 🔌️ Inspects DATA before any rejecting Native open lifetime is enrolled. */
export interface InactiveRecoveryPortsV1<T>{readonly inspect:()=>Promise<InactiveRecoveryReasonV1|null>;readonly open:()=>Promise<T>;readonly reset:(reason:InactiveRecoveryReasonV1)=>Promise<void>}
/** ♻️ Resets once from handled preflight DATA, then admits exactly one ordinary opener under the same Original. */
export async function withInactivePublicationRecoveryV1<T>(directory:string,control:ResourceLeaseControl,ports:InactiveRecoveryPortsV1<T>):Promise<T>{
 const check=()=>{requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);};check();const observation=await ports.inspect();check();
 if(observation){if(observation.version!==1||observation.directory!==directory||observation.kind!=="current-reset-intent"&&observation.kind!=="unsupported-receipt-shape")throw Error("Exact canonical recovery observation required");await ports.reset(observation);check();}
 return ports.open();
}
