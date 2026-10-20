import {requireIssuedResourceControlV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import type {NativeUnsupportedPublicationV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📤️publication/❓️unsupported/🟦️.ts";
/** 🔎️ Records only bounded observations; the reset owner independently rechecks exact physical custody. */
export type InactiveNxRecoveryReasonV1=NativeUnsupportedPublicationV1|Readonly<{version:1;kind:"current-reset-intent";directory:string;receiptStamp:string;receiptSha256:string}>;
/** 🔌️ Inspects DATA before any rejecting Native open lifetime is enrolled. */
export interface InactiveNxRecoveryPortsV1<T>{readonly inspect:()=>Promise<InactiveNxRecoveryReasonV1|null>;readonly open:()=>Promise<T>;readonly reset:(reason:InactiveNxRecoveryReasonV1)=>Promise<void>}
/** ♻️ Resets once from handled preflight DATA, then admits exactly one ordinary opener under the same Original. */
export async function withInactiveNxPublicationRecoveryV1<T>(directory:string,control:ResourceLeaseControl,ports:InactiveNxRecoveryPortsV1<T>):Promise<T>{
 const check=()=>{requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);};check();const observation=await ports.inspect();check();
 if(observation){if(observation.version!==1||observation.directory!==directory||observation.kind!=="current-reset-intent"&&observation.kind!=="unsupported-receipt-shape")throw Error("Exact canonical Nx recovery observation required");await ports.reset(observation);check();}
 return ports.open();
}
