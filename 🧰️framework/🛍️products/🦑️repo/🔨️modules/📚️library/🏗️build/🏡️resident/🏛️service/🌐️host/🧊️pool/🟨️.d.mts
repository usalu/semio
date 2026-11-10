export interface PooledIndexV1<V=unknown>{readonly key:string;readonly value:V;readonly bytes:number;readonly hit:boolean;release():void}
export interface IndexPoolV1{acquire<V>(key:string,load:(signal:AbortSignal)=>Promise<{value:V;bytes:number}>,signal?:AbortSignal):Promise<PooledIndexV1<V>>;stats():{indexes:number;bytes:number;evictions:number;referenced:number};keys():string[];clear():void}
export function createIndexPoolV1(options:{maximumBytes:number;maximumEntries:number;now?:()=>number;onEvict?:(key:string,value:unknown)=>void}):IndexPoolV1;
