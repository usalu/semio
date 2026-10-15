import type {ResourceLeaseControl} from "../../../../🔒️leases/🟦️.ts";

declare const cleanupBrand:unique symbol;
/** 🛟️ Identifies one privately accepted, nonrenewable physical operation in the canonical process owner. */
export type AcceptedPhysicalCleanupV1=Readonly<{[cleanupBrand]:true}>;
/** 📥️ Requests a finite reserve without conferring image or guard custody. */
export type PhysicalCleanupRequestV1=Readonly<{version:1;kind:"native-image";resource:string;reserveMilliseconds:number}>;
/** ⏱️ Captures immutable whole/body deadlines and the issuer-owned cleanup cancellation signal. */
export type AcceptedPhysicalCleanupFactsV1=Readonly<{version:1;kind:"native-image";resource:string;acceptedAtEpochMilliseconds:number;deadlineEpochMilliseconds:number;bodyDeadlineEpochMilliseconds:number;reserveMilliseconds:number;cleanupSignal:AbortSignal}>;
/** 🧷️ Keeps the Native operation and its physical settlement inside the accepted process scope. */
export type AcceptedPhysicalCleanupBodyV1<T>=(enrollment:AcceptedPhysicalCleanupV1,bodyControl:ResourceLeaseControl)=>Promise<T>;
