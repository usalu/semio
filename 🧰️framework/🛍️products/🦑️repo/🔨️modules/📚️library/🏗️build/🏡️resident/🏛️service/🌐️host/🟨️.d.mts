import type {HostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
export class HostRunningErrorV1 extends Error{readonly code:"running"}
export interface PlanTaskV1{id:string;dependsOn:readonly string[];cacheable:boolean;outputs:readonly string[];stale:boolean}
export interface IndexHandleV1{readonly key:string;readonly value:unknown;readonly bytes:number;release():void}
export interface HostPlannerPortV1{
 identify?(request:{root:string;signal?:AbortSignal}):Promise<string|undefined>;
 load(request:{root:string;workspace?:string;signal?:AbortSignal}):Promise<{value:unknown;bytes:number}>;
 plan(request:{index:IndexHandleV1;targets:readonly string[];revision?:string;parent?:string;dirty?:{paths:readonly string[];overflow?:boolean;cursor?:string};cursor?:string;agent?:string;root?:string;workspace?:string;arguments?:readonly string[];tool?:string;graphPolicy?:"fresh"|"current";signal?:AbortSignal}):Promise<{slice?:{graph:unknown;generationPin?:string};revision:string;manifest?:string;pin?:{version:1;revision:string;holder:string;expiresMs:number};tasks:readonly PlanTaskV1[]}>;
}
export interface HostServiceOptionsV1{runtime:HostEndpointsV1;implementation:string;environment?:Record<string,string|undefined>;ports:{planner:HostPlannerPortV1;admission?:{enter(lane:string,signal:AbortSignal,onPosition?:(position:number)=>void):Promise<()=>void>;readonly inFlight:number;readonly queued:number};cache?:{tick(signal:AbortSignal):Promise<unknown>};reload?:{identify(signal?:AbortSignal):Promise<import("../../🔄️reload/🟨️.mjs").HostIdentityV1>|import("../../🔄️reload/🟨️.mjs").HostIdentityV1;replace(request:{implementation:string;signal?:AbortSignal}):Promise<{started:boolean;reason?:string}>}};policy?:Record<string,unknown>;log?:(event:string,detail:string)=>void;now?:()=>number;requireCapability?:boolean;onExit?:(reason:string)=>void}
export interface HostServiceV1{readonly instance:string;readonly closed:Promise<string>;readonly counters:Record<string,number>;start():Promise<{instance:string;endpoint:string}>;stop(reason?:string):Promise<void>;rootKey():Buffer;status():Record<string,unknown>;sweep():void;reload?:{tick(signal?:AbortSignal):Promise<void>;state():import("../../🔄️reload/🟨️.mjs").HostReloadStateV1}}
export function createHostServiceV1(options:HostServiceOptionsV1):HostServiceV1;
export function keepaliveDueV1(options:{stage:"starting"|"attached";held:boolean|undefined;sentAt:number;now:number;intervalMilliseconds:number}):boolean;
