import type {DirtyIndexV1,FrontierAdvanceV1,FrontierChangeV1,FrontierPortsV1,FrontierRowsV1,JournalCursorV1,JournalRowV1} from "../🟨️.mjs";
export type JournalAnswerV1={complete:true;cursor:JournalCursorV1;rows:readonly JournalRowV1[]}|{complete:false;reason:string;detail?:string;path?:string;count?:number};
export interface JournalPortV1{cursor(signal?:AbortSignal):Promise<JournalCursorV1|undefined>;since(from:JournalCursorV1,signal?:AbortSignal):Promise<JournalAnswerV1>;excluded():(path:string)=>boolean}
export interface DirtyCarryV1{exact:string[];subtrees:string[]}
export interface StoredCursorV1 extends JournalCursorV1{carry?:DirtyCarryV1;install?:string}
export interface CursorStoreV1{get(key:string):StoredCursorV1|undefined;set(key:string,cursor:JournalCursorV1&{install?:string},carry?:DirtyCarryV1):void}
export interface JournalNoticeV1{reason:string;detail?:string;path?:string;count?:number;generation?:number}
export interface JournalSeamsOptionsV1{root:string;journal:JournalPortV1;store:CursorStoreV1;ports:FrontierPortsV1;digest:(members:[string,number][])=>string;maximumCarry?:number;install?:()=>Promise<string|undefined>;membership:(previous:any,signal?:AbortSignal)=>Promise<any>;holds:(rows:readonly {kind:string;path:string;fact:any}[],signal?:AbortSignal)=>Promise<boolean>;classify?:(changes:FrontierChangeV1[])=>Promise<FrontierChangeV1[]>;adopt?:(files:string[])=>void;notify?:(notice:JournalNoticeV1)=>void;now?:()=>number}
export function resolveRowDoubtfulV1(root:string,row:{path:string;fact?:unknown},index:{has(path:string):boolean},excluded:(path:string)=>boolean):boolean;
export function installCoveredV1(relative:string):boolean;
export function relativeFactPathV1(root:string,path:string):string|undefined;
export function journalGraphSeamsV1(options:JournalSeamsOptionsV1):{membership(previous:any,signal?:AbortSignal):Promise<any>;holds(rows:readonly {kind?:string;path:string;fact?:unknown}[],signal?:AbortSignal):Promise<boolean>;commit(record:{generation:number;owners:readonly string[];covered?:ReadonlySet<string>}):void;scope(full:boolean):void;factsVerified():void;stats:{advanced:number;verified:number;checked:number;trusted:number;doubtful:number;resumed:number}};
export type {DirtyIndexV1,FrontierAdvanceV1,FrontierRowsV1};
