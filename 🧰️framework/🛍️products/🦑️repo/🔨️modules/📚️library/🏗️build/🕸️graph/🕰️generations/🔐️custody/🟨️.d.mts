import type {ClaimFilePortV1,ClaimLeasePortV1} from "../🧩️claims/🟨️.mjs";
import type {DoorAskV1,DoorHolderV1,DoorProgressV1} from "../🪧️door-board/🟨️.mjs";
/** 🔐️ The declared constants of the slice custody. */
export interface CustodyPolicyV1{readonly version:1;readonly directory:string;readonly pollMilliseconds:number;readonly noticeMilliseconds:number;readonly maximumClaims:number;readonly maximumOwners:number;readonly sampleOwners:number;readonly widenRounds:number;readonly attachPollMilliseconds:number;readonly attachRecheckMilliseconds:number;readonly attachSpendRatio:number;readonly attachMaximumPollMilliseconds:number;readonly stallMilliseconds:number;readonly askGraceMilliseconds:number;readonly progressWriteMilliseconds:number;readonly maximumHandovers:number;readonly commandCharacters:number}
/** 📐️ The declared constants of the slice custody. */
export const CUSTODY_POLICY_V1:CustodyPolicyV1;
/** 🔑️ The keys of the bilingual custody texts. */
export type CustodyMessageKeyV1="blocked"|"blockedAll"|"attached"|"attachedWave"|"attachedServed"|"closureRepaired"|"holder"|"holderSuffix"|"askedStalled"|"askedStale"|"deposed"|"handoverStalled"|"handoverStale";
/** 🗨️ The bilingual texts of the slice custody. */
export const CUSTODY_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<CustodyMessageKeyV1,string>>>>;
/** 🖨️ Renders one custody message in the language the caller resolved. */
export function renderCustodyMessageV1(key:CustodyMessageKeyV1,fields:Record<string,string|number>,language:"en"|"de"):string;
/** 🪪️ A closed custody claim document. */
export interface CustodyClaimV1{readonly version:1;readonly token:string;readonly ticket:number;readonly owners:readonly string[];readonly structural:boolean;readonly state:"waiting"|"held";readonly queuedMs:number;readonly holder?:DoorHolderV1;readonly progress?:DoorProgressV1;readonly ask?:DoorAskV1}
/** 🛂️ Admits a closed custody claim or refuses it. */
export function readCustodyClaimV1(value:unknown):CustodyClaimV1;
/** ⚔️ Whether two claims may not build at the same time. */
export function claimsConflictV1(a:Pick<CustodyClaimV1,"owners"|"structural">,b:Pick<CustodyClaimV1,"owners"|"structural">):boolean;
/** 🚦️ Decides one admission against the other live claims. */
export function admissionOfV1<C extends Pick<CustodyClaimV1,"token"|"ticket"|"owners"|"structural"|"state">>(mine:C,others:readonly C[]):{admit:boolean;blockers:C[]};
/** 🧭️ What a builder asks the custody for: the owners it writes, structural when they cannot be named. */
export interface CustodyRequestV1{readonly dirty:readonly string[];readonly structural:boolean}
/** 🎫️ A held claim. */
export interface SliceClaimV1{readonly token:string;readonly owners:readonly string[];readonly structural:boolean;extend(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal}):Promise<{held:boolean;request:CustodyRequestV1}>;alone(options?:{signal?:AbortSignal}):Promise<boolean>;release():Promise<void>;settle(finished:readonly string[]):Promise<boolean>;progress(event:{phase?:string;done?:number;total?:number}):Promise<void>;handover():Promise<{reason:"stalled"|"stale-code";current?:string;ask?:DoorAskV1}|undefined>;handoverError(reason:{reason:"stalled"|"stale-code";current?:string},counts?:{done?:number;total?:number}):Error&{code:"SEMIO_STALE_IMPLEMENTATION";handover:{reason:string;done:number;total:number}}}
/** 🏛️ The slice custody of one generation store. */
export interface SliceCustodyV1{hold(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal;say?:(key:CustodyMessageKeyV1,fields:Record<string,string|number>)=>void;language?:"en"|"de";holder?:DoorHolderV1;currentDigest?:()=>Promise<string|undefined>|string|undefined}):Promise<SliceClaimV1>;idle(options:{owners?:readonly string[];structural?:boolean;signal?:AbortSignal}):Promise<boolean>;list(signal?:AbortSignal):Promise<CustodyClaimV1[]>;readonly resource:{claim(token:string):string;admission:string}}
/** 🧲️ What an attached reader sees of the custody: whether a live claim holds it against the reader and the newest generation. */
export interface CustodyLookV1{readonly held:boolean;readonly generation:number|undefined}
/** 🧲️ Attaches a reader to the live builder of its stale projects instead of preparing them beside it; answers what `recheck` served, or undefined when nothing holds the custody against the reader (any more). */
export function attachToBuilderV1<T>(options:{look:()=>Promise<CustodyLookV1>;recheck:()=>Promise<T|undefined>;say?:(key:CustodyMessageKeyV1,fields:Record<string,string|number>)=>void;signal?:AbortSignal;now?:()=>number;sleep?:(milliseconds:number,signal?:AbortSignal)=>Promise<void>;policy?:Pick<CustodyPolicyV1,"attachPollMilliseconds"|"attachRecheckMilliseconds"|"noticeMilliseconds"|"attachSpendRatio"|"attachMaximumPollMilliseconds">}):Promise<T|undefined>;
/** 🏗️ Creates the slice custody of the generation documents under `root`. */
export function createSliceCustodyV1(options:{root:string;fs:Omit<ClaimFilePortV1,never>;lease:ClaimLeasePortV1;now?:()=>number;token?:()=>string}):SliceCustodyV1;
