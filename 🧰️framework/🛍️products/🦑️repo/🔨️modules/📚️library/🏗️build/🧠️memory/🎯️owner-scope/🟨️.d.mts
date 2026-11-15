/** ⛔️ What a stopped owner had taken and the limit it passed, in bytes. */
export interface GraphOwnerStopV1{readonly name:string;readonly takenBytes:number;readonly limitBytes:number}
export class GraphOwnerTooHeavyErrorV1 extends Error{readonly code:"BUILD_GRAPH_OWNER_TOO_HEAVY";readonly detail:GraphOwnerStopV1;constructor(detail:GraphOwnerStopV1)}
export interface GraphOwnerScopeV1{readonly name:string;exceed(detail:Omit<GraphOwnerStopV1,"name">):void;readonly exceeded:Omit<GraphOwnerStopV1,"name">|undefined;credit(bytes:number):void;readonly credited:number}
export function createOwnerScopeV1(name:string):GraphOwnerScopeV1;
export function runInOwnerScopeV1<T>(scope:GraphOwnerScopeV1,work:()=>T):T;
export function throwIfOwnerExceededV1():void;
export function ownerTooHeavyOfV1(error:unknown,depth?:number):GraphOwnerStopV1|undefined;
export function creditOwnerSharedV1(bytes:number):void;
