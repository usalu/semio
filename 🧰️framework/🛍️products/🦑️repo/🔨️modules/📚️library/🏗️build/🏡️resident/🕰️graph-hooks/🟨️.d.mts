export interface FrontierRowsV1{version:1;capturedAt:number;directories:[string,number,number][];members:[string,number][]}
export interface JournalCursorV1{instance:string;seq:number}
export type JournalRowV1=readonly ["a"|"m"|"r"|"n",0|1,string];
export interface DirtySetV1{cursor:JournalCursorV1;complete:boolean;exact:string[];subtrees:string[];overflow:boolean}
export interface DirtyIndexV1{size:number;has(path:string):boolean}
export interface FrontierChangeV1{path:string;kind:"added"|"removed"|"changed";directory:boolean;config?:boolean;pending?:boolean}
export interface FrontierPortsV1{list(directory:string):Promise<{name:string;kind:"directory"|"file"|"other"}[]|undefined>;stat(path:string):Promise<{directory:boolean;file:boolean;mtimeMs:number;ino:number;mode:number}|undefined>;descend(name:string,path:string,parent:string):boolean;member(path:string):boolean}
export type FrontierAdvanceV1={unchanged:true;checked:number}|{directories:[string,number,number][];members:[string,number][];changes:FrontierChangeV1[];checked:number;capturedAt:number};
export const GRAPH_HOOKS_POLICY_V1:Readonly<{version:1;maximumCursors:number;cursorFile:string;texts:Record<string,Record<string,string>>}>;
export function dirtySetFromRowsV1(rows:readonly JournalRowV1[],cursor:JournalCursorV1,options?:{overflow?:boolean}):DirtySetV1;
export function dirtyIndexV1(set:Pick<DirtySetV1,"exact"|"subtrees">):DirtyIndexV1;
export function advanceFrontierByDirtySetV1(options:FrontierPortsV1&{frontier:FrontierRowsV1;set:DirtySetV1;signal?:AbortSignal;now?:number;concurrency?:number}):Promise<FrontierAdvanceV1|undefined>;
export function classifyChangesV1<T extends {path:string;directory?:boolean}>(changes:readonly T[],isConfig?:(path:string)=>boolean):(T&{config?:true})[];
export function renderGraphHooksNoticeV1(notice:string,language:string,fields?:Record<string,unknown>):string;
export function derivedDirtySetV1(set:DirtySetV1,revision?:string):{revision?:string;paths:string[];overflow:boolean};
