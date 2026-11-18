/** 🪜️ The declared limits and stages of a graph plan. */
export interface PlanStagePolicyV1{readonly version:1;readonly hostShare:number;readonly treeShare:number;readonly minimumMegabytes:number;readonly maximumMegabytes:number;readonly serveHostMegabytes:number;readonly limitEnvironment:string;readonly off:string;readonly owner:string;readonly stages:readonly string[];readonly stageMegabytes:Readonly<Record<string,number>>;readonly sampleMilliseconds:number;readonly recentSamples:number;readonly farmPriority:Readonly<Record<string,number>>;readonly parentMegabytes:number;readonly parentFootprintMegabytes:number;readonly farmWorkerPeakMegabytes:Readonly<Record<string,number>>;readonly farmChildFootprintMegabytes:Readonly<Record<string,{readonly typicalMegabytes:number;readonly capMegabytes:number;readonly aloneCapMegabytes:number}>>}
export declare const PLAN_STAGE_POLICY_V1:PlanStagePolicyV1;
export type PlanStageMessageKeyV1="started"|"finished"|"stopped"|"detail";
export declare const PLAN_STAGE_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<PlanStageMessageKeyV1,string>>>>;
/** 📡️ The message a slice worker sends to its service when a plan stage starts. */
export interface PlanStageEventV1{readonly type:"stage";readonly stage:string;readonly operation:string;readonly managedMegabytes:number;readonly residentMegabytes:number}
export function onPlanStageV1(listener:(event:PlanStageEventV1)=>void):()=>boolean;
export function renderPlanStageMessageV1(key:PlanStageMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment:Readonly<Record<string,string|undefined>>):string[];
export function planStageLimitBytesV1(environment?:Readonly<Record<string,string|undefined>>,hostMegabytes?:number,policy?:PlanStagePolicyV1):number;
/** 📐️ How many children whose footprint is bounded fit in the hard share of the tree. */
export function farmWidthV1(input:{hostMegabytes:number;reservedMegabytes?:number;policy?:PlanStagePolicyV1;child:{readonly typicalMegabytes:number;readonly capMegabytes:number};hardPercent?:number}):number;
/** 🏊️ How many worker processes of `workerMegabytes` a plan that holds `residentMegabytes` already may start so that its whole process tree stays within the share of the host; Infinity where the plan has no limit. */
export function planFarmWorkersV1(input:{workerMegabytes:number;residentMegabytes?:number;environment?:Readonly<Record<string,string|undefined>>;hostMegabytes?:number;policy?:PlanStagePolicyV1;kind?:string;counted?:string}):number;
/** 🪶️ The lean plan of the slice read on this machine. */
export function leanSliceV1(input?:{environment?:Readonly<Record<string,string|undefined>>;hostMegabytes?:number;cores?:number;residentMegabytes?:number;policy?:PlanStagePolicyV1}):import("../🪶️lean/🟨️.mjs").LeanSlicePlanV1;
/** 🧮️ Reserves workers of one kind of farm in the one memory plan of the process tree. */
export function reserveFarmV1(input:{kind:string;workerMegabytes:number;wanted?:number;residentMegabytes?:number;environment?:Readonly<Record<string,string|undefined>>;hostMegabytes?:number;policy?:PlanStagePolicyV1;yield?:()=>void}):Readonly<{workers:number;release():void}>;
export function farmReservationsV1():{kind:string;megabytes:number}[];
export interface PlanStageGuardV1{mark(stage:string):void;operation(name:string):void;run<T>(work:()=>Promise<T>|T):Promise<T>}
export function createPlanStageGuardV1(options?:{limitBytes?:number;ports?:{sample:()=>number;collect:()=>void};environment?:Readonly<Record<string,string|undefined>>;report?:(line:string)=>void;now?:()=>number;resident?:()=>number;usage?:()=>Readonly<Record<string,number>>;heap?:()=>Readonly<Record<string,any>>|undefined;resources?:()=>readonly string[]|undefined;stageMegabytes?:Readonly<Record<string,number>>;refuse?:(detail:{stage:string;operation:string;megabytes:number;limit:number})=>Error}):PlanStageGuardV1;

/** 🛑️ Registers a listener that is told when the plan guard of this process stops a plan; returns the function that removes it. */
export function onPlanStopV1(listener:(event:{stage:string;operation:string;usedMegabytes:number;budgetMegabytes:number})=>void):()=>boolean;
