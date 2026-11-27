import type {HostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import type {IndexPoolV1} from "./🧊️pool/🟨️.mjs";
export class HostRunningErrorV1 extends Error{readonly code:"running"}
export interface PlanTaskV1{id:string;dependsOn:readonly string[];cacheable:boolean;outputs:readonly string[];stale:boolean;key?:string|null}
export interface IndexHandleV1{readonly key:string;readonly value:unknown;readonly bytes:number;release():void}
/** 📶️ Supplies a slice observation. A heartbeat or cached replay sets observed to false. */
export type HostPlanProgressV1=(milliseconds:number,detail?:{phase?:string;done?:number;total?:number;running?:unknown;decision?:unknown;reservation?:unknown;leaseWait?:unknown;sliceReport?:string},observed?:boolean)=>void|Promise<unknown>;
export interface HostPlannerPortV1{
 identify?(request:{root:string;signal?:AbortSignal}):Promise<string|undefined>;
 load(request:{root:string;workspace?:string;signal?:AbortSignal}):Promise<{value:unknown;bytes:number}>;
 plan(request:{index:Readonly<{value:unknown;key?:string}>;targets:readonly string[];revision?:string;parent?:string;dirty?:{paths:readonly string[];overflow?:boolean;cursor?:string};cursor?:string;agent?:string;root?:string;workspace?:string;arguments?:readonly string[];tool?:string;graphPolicy?:"fresh"|"current";grant?:string;signal?:AbortSignal;progress?:HostPlanProgressV1;wantsReport?:()=>boolean}):Promise<{slice?:{graph:unknown;generationPin?:string};revision:string;manifest?:string;pin?:{version:1;revision:string;holder:string;expiresMs:number};tasks:readonly PlanTaskV1[]}>;
}
export interface HostServiceOptionsV1{runtime:HostEndpointsV1;implementation:string;environment?:Record<string,string|undefined>;access?:{root?:string;platform?:string;home?:string};ports:{planner:HostPlannerPortV1;resident?:{rss?():number};workers?:{memory?():readonly Readonly<Record<string,unknown>>[];busy?():number;active?():number};settle?:()=>unknown;admission?:{enter(lane:string,signal:AbortSignal,onPosition?:(position:number)=>void):Promise<()=>void>;readonly inFlight:number;readonly queued:number};cache?:{tick(signal:AbortSignal):Promise<unknown>};reload?:{identify(signal?:AbortSignal):Promise<import("../../🔄️reload/🟨️.mjs").HostIdentityV1>|import("../../🔄️reload/🟨️.mjs").HostIdentityV1;replace(request:{implementation:string;signal?:AbortSignal}):Promise<{started:boolean;reason?:string}>}};policy?:Record<string,unknown>;log?:(event:string,detail:string)=>void;now?:()=>number;requireCapability?:boolean;onExit?:(reason:string)=>void}
export interface HostServiceV1{readonly instance:string;readonly closed:Promise<string>;readonly pool:IndexPoolV1;readonly counters:Record<string,number>;start():Promise<{instance:string;endpoint:string}>;stop(reason?:string):Promise<void>;rootKey():Buffer;status():Record<string,unknown>;sweep():void;reload?:{tick(signal?:AbortSignal):Promise<void>;state():import("../../🔄️reload/🟨️.mjs").HostReloadStateV1}}
export function createHostServiceV1(options:HostServiceOptionsV1):HostServiceV1;
export function supersededDrainDueV1(input:{supersededSinceMs:number|undefined;nowMs:number;lastProgressMs:number;connections:number;inFlight:number;graceMilliseconds:number;stallMilliseconds:number}):boolean;
export function keepaliveDueV1(options:{stage:"starting"|"attached";held:boolean|undefined;sentAt:number;now:number;intervalMilliseconds:number}):boolean;
/** 🔒️ Renders in each language why a slice load waits for a graph writer lock and who holds it. */
export function renderLeaseWaitV1(wait:Readonly<{lock:Readonly<{kind:string;number?:number}>;resource:string;position:number;queued:number;pid:number;seconds:number;state?:string;agent?:string;phase?:string;done?:number;total?:number;etaMinutes?:number}>,seconds:number,languages?:readonly string[]):Partial<Record<"en"|"de",string>>;

/** 💽 Frees spilled slices, oldest first and never the one about to be written, until the directory's device has room for the next slice body; leftover temporary files of failed writes go first. The room comes from the device's own free-space quote, so a small runtime filesystem never fails a write that evicting a stale slice would have admitted. */
export function makeRoomV1(directory: string, keep: string, bytes: number): void;
/** 🧮️ What one slice reservation asked, was granted, or waits on. */
export interface HostSliceReservationV1{readonly requested:number;readonly granted?:number;readonly wait?:Readonly<{reason:string;active:number;queued:number;spareMegabytes:number;floorMegabytes:number;minimumMegabytes:number;waitedSeconds:number}>}
/** 🪪️ Renders the gate's reason in the connection's language without naming unobserved capacity. */
export function renderSliceReservationV1(reservation: HostSliceReservationV1, seconds: number, languages?: readonly string[]): Partial<Record<"en"|"de",string>>;
