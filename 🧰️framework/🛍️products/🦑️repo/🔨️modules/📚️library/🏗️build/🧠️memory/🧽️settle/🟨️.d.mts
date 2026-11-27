/** 🧽️ The declared numbers of the memory settling. */
export interface SettlePolicyV1{readonly version:1;readonly censusCaches:number}
export declare const SETTLE_POLICY_V1:SettlePolicyV1;
export type SettleMessageKeyV1="answered"|"retained";
export declare const SETTLE_MESSAGES_V1:Readonly<Record<"en"|"de",Readonly<Record<SettleMessageKeyV1,string>>>>;
export function renderSettleMessageV1(key:SettleMessageKeyV1,fields:Readonly<Record<string,string|number>>,environment?:Readonly<Record<string,string|undefined>>):string[];
/** 🧽️ What a settling measured, in megabytes. */
export interface SettleResultV1{readonly managedBeforeMegabytes:number;readonly managedMegabytes:number;readonly residentBeforeMegabytes:number;readonly residentMegabytes:number;readonly releasedMegabytes:number}
export function settleGraphMemoryV1(options?:{release?:()=>unknown;ports?:{sample:()=>number;collect:()=>void};resident?:()=>number;shrink?:()=>void;environment?:Readonly<Record<string,string|undefined>>;report?:(line:string)=>void;limitBytes?:number;force?:boolean}):Promise<SettleResultV1|undefined>;
/** 🔌️ Registers the collections of the whole read with the report of retained memory (once per process). */
export function registerRetainedCollectionsV1():Promise<void>;
export function reportRetainedGraphMemoryV1(options?:{signal?:AbortSignal;ports?:{sample:()=>number;collect:()=>void};resident?:()=>number;environment?:Readonly<Record<string,string|undefined>>;report?:(line:string)=>void;limitBytes?:number;stats?:()=>readonly {name:string;entries:number;bytes:number}[]}):Promise<void>;
