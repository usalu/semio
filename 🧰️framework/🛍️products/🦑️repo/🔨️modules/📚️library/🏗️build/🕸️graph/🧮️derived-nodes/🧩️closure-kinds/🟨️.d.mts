import type{GenerationRowV1}from"../../🕰️generations/🟨️.mjs";
export const CLOSURE_KINDS_V1:readonly["script-walk","ts-program","derived-walk"];
/** 🧮️ The limits of one closure settlement: the cutoff policy plus the environment key of the closure kinds. */
export type ClosureSettleLimitsV1=Readonly<{deriveConcurrency:number;collectEveryClosures:number;settleChunkClosures:number;manifestSaveMilliseconds:number;manifestSaveSpendRatio:number;vouchConcurrency:number;maximumSuspects:number;environmentKey:string}>;
export const CLOSURE_SETTLE_V1:ClosureSettleLimitsV1;
/** 🏷️ The node kind of one question. */
export function closureKindOfV1(path:string):"script-walk"|"ts-program"|"derived-walk";
export const CLOSURE_RULES_V1:Readonly<Record<string,(ctx:any)=>Promise<unknown>>>;
export function verifyClosureObservedV1(ref:{rows:readonly GenerationRowV1[]},options:{signal?:AbortSignal;environment:{hold(rows:readonly GenerationRowV1[],signal?:AbortSignal):Promise<boolean>}}):Promise<boolean>;
/** ❓️ The settler of derived questions: one node graph per store; `settle` asks the suspect questions again, an equal answer holds, a different answer or a failing walk changes the fact; the result lists the support rows of the answers (`answers`) only when `answers:true` is asked, so a plan holds nothing it does not read. */
export function createClosureSettlerV1(options?:{store?:unknown;workspace?:{workspaceRoot:string;cacheRoot:string;grant?:()=>unknown;blobs?:unknown;skip?:readonly string[]};limits?:ClosureSettleLimitsV1;farm?:false|{root?:string;size?:number;afterDerivations?:number;module?:string;spawn?:unknown};partIds?:false|{directory?:string}}):Readonly<{graph:unknown;store:unknown;invalidate(dirty:{paths:readonly string[];overflow?:boolean},options?:{signal?:AbortSignal}):Promise<unknown>;settle(options:{pseudoRows:ReadonlyMap<string,GenerationRowV1>;staleFacts?:readonly GenerationRowV1[];derive?:(row:GenerationRowV1,signal?:AbortSignal)=>Promise<{fact:unknown;rows?:readonly GenerationRowV1[];parts?:readonly (readonly GenerationRowV1[])[];partIds?:readonly string[]}>;hold?:(rows:readonly GenerationRowV1[],signal?:AbortSignal)=>Promise<boolean>;dirty?:{paths:readonly string[];directories?:readonly string[];overflow?:boolean;excluded?:(path:string)=>boolean};signal?:AbortSignal;answers?:boolean;progress?:(event:{phase:"verification";done:number;total:number})=>Promise<void>|void}):Promise<{held:Map<string,GenerationRowV1[]>;kept:Set<string>;changed:Map<string,GenerationRowV1>;answers:Map<string,{fact:unknown;rows:GenerationRowV1[]}>;failed:number;vouched:number;slow:number;farmed:number;reused:number;parts:Record<string,number>}>;markIndexed(options:{held?:ReadonlyMap<string,{digest?:string;length:number}>;dropped?:readonly string[]}):Promise<void>;stats():Record<string,number>}>;
/** 🎟️ Binds the capacity grant of this process to the workspace graphs. */
export function useCapacityGrantV1(grant:(()=>unknown)|undefined):void;
/** 🏭️ The settler of one workspace shared by every plan of the host service. */
export function sharedClosureSettlerV1(options:{workspaceRoot:string;cacheRoot:string;blobs?:unknown;skip?:readonly string[]}):ReturnType<typeof createClosureSettlerV1>;

export {stampOfV1} from "./🧭️row-paths/🟨️.mjs";
/** 🧩️ The rows of one closure answer: its own row followed by the rows of every part. */
export interface ClosurePartRowsV1 extends Iterable<GenerationRowV1>{readonly digest:string;readonly parts:readonly (readonly GenerationRowV1[])[];readonly ids:readonly string[];readonly fact:string;readonly length:number}
/** 📚️ The support of one held closure as the row of the question followed by the rows of its parts, read part by part: a held closure costs its list of parts, and the rows are produced where they are consumed. */
export function partRowsV1(row: GenerationRowV1, parts: readonly (readonly GenerationRowV1[])[], digest: string, ids: readonly string[], fact: string): ClosurePartRowsV1;
