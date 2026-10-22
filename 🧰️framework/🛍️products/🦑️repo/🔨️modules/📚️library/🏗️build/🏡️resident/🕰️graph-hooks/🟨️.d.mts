export interface FrontierRowsV1{version:1;capturedAt:number;directories:[string,number,number][];members:[string,number][]}
export interface JournalChangeV1{path:string;kind:"added"|"removed"|"changed";directory:boolean;mode?:number;stamp?:[number,number]}
export interface JournalReplyV1{complete:boolean;seq?:number;changes:JournalChangeV1[];cursor?:{instance:string;seq:number};reason?:string}
export interface MembershipV1{digest:string;frontier:FrontierRowsV1;changes:{path:string;kind:"added"|"removed"|"changed";directory?:boolean;config?:boolean}[];cursor?:{instance:string;seq:number}}
export function membershipFromJournalV1(previous:{manifest:{membership:string};frontier:FrontierRowsV1}|undefined,reply:JournalReplyV1|undefined,options?:{now?:number;isConfig?:(path:string)=>boolean;digest?:(members:[string,number][])=>string}):MembershipV1|undefined;
export function dirtyPathsFromJournalV1(reply:JournalReplyV1|undefined):{has(path:string):boolean;size:number}|undefined;
export function createResidentHoldsV1(options:{dirty:()=>Promise<{has(path:string):boolean}|undefined>;fallback:(rows:readonly {path:string}[],signal?:AbortSignal)=>Promise<boolean>}):(rows:readonly {path:string}[],signal?:AbortSignal)=>Promise<boolean>;
