import type {GenerationOwnerRowV1} from "../🟨️.mjs";
type OwnerTableV1=Readonly<Record<string,Pick<GenerationOwnerRowV1,"edgesEnv"|"tag">>>;
/** 🪡️ The owners whose edges are of another environment, untagged ones in `edges`, tagged ones in `tagged`. */
export function staleEdgeOwnersV1(owners:OwnerTableV1,environment:string,pending?:Iterable<string>):{edges:string[];tagged:string[]};
/** 🪡️ The `edgeStale` field of a compute spec: stale untagged owners the build does not infer anyway. */
export function edgeStaleSpecV1(owners:OwnerTableV1,environment:string|null|undefined,inferred:Iterable<string>):{edgeStale?:string[]};
/** 🩹️ The owners a build gives their edges again (`edges`) or infers (`infer`) after its compute left stale ones. */
export function edgeHealPlanV1(input:{owners:OwnerTableV1;environment:string;cold:boolean;healed:number;tried?:ReadonlyMap<string,string>|readonly string[]|ReadonlySet<string>;inferred?:ReadonlyMap<string,string>|readonly string[]|ReadonlySet<string>;pending?:Iterable<string>;limit?:number}):{edges:string[];infer:string[]};
/** 🚧️ Refuses a publication whose owner table holds edges of another environment. */
export function requireEdgeEnvironmentV1(input:{owners:OwnerTableV1;environment:string;fail:(message:string,code?:string)=>Error;wave:readonly string[];returned:ReadonlySet<string>;dropped:ReadonlySet<string>;carried?:readonly string[];cold:boolean;repair:boolean;healed:number;resumed:boolean;before:Readonly<Record<string,{edgesEnv?:string}>>;previous:string|null|undefined;pending?:Iterable<string>;spec?:{stubs?:Record<string,{kind?:string;tag?:string}>;edgeStale?:readonly string[];owners?:readonly string[];discover?:readonly string[]}}):void;
/** 🧩️ Restamps the owners whose edges provably hold under `environment` (no component they read moved) instead of computing them again. */
export function restampComponentValidEdgesV1(input:{owners:Readonly<Record<string,any>>;environment:string;component:{holds(name:string,row:any):boolean};pending?:Iterable<string>;say?:(key:string,fields:Record<string,number>)=>void}):{owners:Record<string,any>;restamped:string[]};
/** 🪡️ Stamps the kept edges of a build (carried owners, and with a judge the owners whose edges provably hold) with the environment. */
export function carryEdgeStampsV1(input:{owners:Record<string,any>;carried?:readonly string[];returned:ReadonlySet<string>;environment:string;component?:{stamp(name:string,row:any):object;holds(name:string,row:any):boolean};cold:boolean;pending?:Iterable<string>;say?:(key:string,fields:Record<string,number>)=>void}):void;
export {componentEdgesV1} from "../🧩️environment-components/🟨️.mjs";
