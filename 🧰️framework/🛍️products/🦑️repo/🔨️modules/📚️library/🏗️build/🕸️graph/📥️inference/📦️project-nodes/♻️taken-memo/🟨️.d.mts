/** ♻️ The owners one read took in this process, kept from one pass over the owners to the next. */
export interface TakenOwnerV1{readonly name:string;readonly project:unknown;readonly rows:readonly unknown[]}
export interface TakenMemoStatsV1{readonly entries:number;readonly remembered:number;readonly reused:number;readonly evictedNamed:number;readonly evictedChanged:number;readonly evictedKey:number;readonly passes:number;readonly trusted:number}
export interface TakenMemoV1{
 remember(name:string,entry:{key:string;project:unknown;rows:readonly unknown[]}):boolean;
 evictReferencing(paths:readonly string[]):number;
 replay(wanted:Iterable<{name:string;key:string}>,options?:{signal?:AbortSignal}):Promise<TakenOwnerV1[]>;
 trustSurvivors():number;
 distrust():number;
 forget(name:string):boolean;
 clear():void;
 readonly size:number;
 stats():TakenMemoStatsV1;
}
export function createPathRowMatcherV1(paths:readonly string[]):(row:{kind:string;path:string})=>boolean;
export function createTakenMemoV1(ports:{holds:(rows:readonly unknown[])=>boolean;retain:(rows:readonly unknown[])=>void}):TakenMemoV1;
export function keepTakenThenReleaseV1(input:{memo:TakenMemoV1|undefined;name:string;key:string|undefined;project:unknown;read:()=>readonly unknown[]|undefined;release?:()=>void}):void;
