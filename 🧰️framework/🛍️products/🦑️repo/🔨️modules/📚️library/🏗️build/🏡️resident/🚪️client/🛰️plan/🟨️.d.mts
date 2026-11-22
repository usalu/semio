import type {HostEndpointsV1} from "../../📨️protocol/📍️endpoint/🏠️host/🟨️.mjs";
import type {PlanTaskV1} from "../../🏛️service/🌐️host/🟨️.mjs";
type Environment=Readonly<Record<string,string|undefined>>;
export interface HostProgressV1{phase:string;done:number;total:number;message:Record<string,string>}
export type HostStartupTimeoutV1=Readonly<{mode:"live-ceiling"|"log-silence";elapsedMilliseconds:number;limitMilliseconds:number;pid:number|null;seen:number;candidates:readonly Readonly<{tag:string;log:string}>[]}>;
export class HostUnavailableErrorV1 extends Error{constructor(reason:string,message?:string,startup?:HostStartupTimeoutV1);readonly reason:string;readonly missing?:boolean;readonly startup?:HostStartupTimeoutV1;notice(language:string):string;}
export const PLAN_KEY_WAIT_V1:Readonly<{maximum:number;milliseconds:number}>;
export function planKeyRetryV1(error:unknown,waited:number):boolean;
export class PlanRefusedErrorV1 extends Error{readonly code:string;readonly retry:string;readonly message_:Readonly<Record<string,string>>}
export function connectHostV1(options:{implementation:string;identify?:()=>string;environment?:Environment;platform?:string;spawnService?:(request:{runtime:HostEndpointsV1;implementation:string;environment:Environment})=>unknown;signal?:AbortSignal;onProgress?:(frame:HostProgressV1)=>void;languages?:readonly string[];policy?:Readonly<Record<string,number>>;ignorePids?:readonly number[]}):Promise<{socket:import("node:net").Socket;runtime:HostEndpointsV1;started:boolean;implementation?:string}>;
export interface PlanResultV1{reply:{slice?:{hash:string;size:number};revision:string;manifest?:string;pin?:{version:1;revision:string;holder:string;expiresMs:number};plan:{tasks:readonly PlanTaskV1[]}|{ref:{hash:string;size:number}};progress:readonly {phase:string;ms:number}[];cacheHits:number};welcome:{service:{pid:number}};runtime:HostEndpointsV1;plan:{tasks:readonly PlanTaskV1[]}}
export function requestPlanV1(options:{root:string;implementation:string;identify?:()=>string;targets:readonly string[];agent:string;priority?:number;revision?:string;parent?:string;dirty?:{paths:readonly string[];overflow?:boolean;cursor?:string};arguments?:readonly string[];tool?:string;graphPolicy?:"fresh"|"current";language:"en"|"de";signal?:AbortSignal;onProgress?:(frame:HostProgressV1)=>void;environment?:Environment;platform?:string;spawnService?:Parameters<typeof connectHostV1>[0]["spawnService"];ignorePids?:readonly number[];attempts?:number;deadlineMs?:number;silenceMilliseconds?:number}):Promise<PlanResultV1>;
export function resolvePlanV1(reply:PlanResultV1["reply"],runtime:HostEndpointsV1):{tasks:readonly PlanTaskV1[]};
export function requestHostStatusV1(options:{implementation:string;environment?:Environment;platform?:string;signal?:AbortSignal;timeoutMilliseconds?:number}):Promise<{pid:number;rssMb:number;indexBytes:number;indexes:number;workspaces:readonly {id:string;root:string;indexKey:string;lastUsedAt:number;plans:number}[];clients:number;queue:number;inFlight:number}|undefined>;

export function holdHostV1(options:{implementation:string;environment?:Environment;platform?:string;agent?:string}):Promise<()=>Promise<void>>;
export function retireHostServiceV1(options:{implementation:string;environment?:Environment;platform?:string;signal?:AbortSignal;timeoutMilliseconds?:number;only?:number;status?:(options:Parameters<typeof requestHostStatusV1>[0])=>Promise<{pid:number}|undefined>;kill?:(pid:number,signal:string)=>unknown;pause?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>}):Promise<{pid:number|undefined;retired:boolean}>;
export function withRestartOwnerV1<T>(options:{implementation:string;environment?:Environment;platform?:string;signal?:AbortSignal;policy?:Readonly<Record<string,number>>},owned:()=>Promise<T>,joined:()=>Promise<T>):Promise<T>;
export function lastAccessDenialV1(options:{implementation:string;environment?:Environment;platform?:string;since?:number}):{event:"exit";reason:"access-denied";pid?:number;route?:"launchd"|"direct"|"foreground";root?:string;place?:string;folder?:string;at:number}|undefined;

/** 🌐 A text of the start wait in the requested languages. */
export function waitMessagesV1(key:"moved"|"ceiling"|"live"|"silence"|"candidates",fields:Readonly<Record<string,string|number>>,languages:readonly string[]):Record<string,string>;
