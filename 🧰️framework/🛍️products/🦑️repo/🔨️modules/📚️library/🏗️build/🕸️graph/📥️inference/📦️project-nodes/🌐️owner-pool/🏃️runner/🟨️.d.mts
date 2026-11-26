/** 🧮️ What the pool asks of the capacity port: how many children and how much memory each. */
export type OwnerPoolCapacityAskV1=Readonly<{children:number;childMegabytes:number}>;
/** 🏊️ The ports of one owner pool run; the running host fills every default. */
export type OwnerPoolRunOptionsV1=Readonly<{
 root:string;environment:Readonly<Record<string,string|undefined>>;totalMegabytes?:number;signal?:AbortSignal;say?:(line:string)=>void;
 capacity?:(ask:OwnerPoolCapacityAskV1)=>Promise<unknown>;siblings?:number;fleetRoot?:string;work?:unknown;status?:unknown;
 pollMilliseconds?:number;targetPollMilliseconds?:number;once?:boolean;driver?:unknown;childEntry?:string;spawn?:unknown;now?:()=>number;
 priority?:number;observe?:unknown;revision?:unknown;starved?:unknown;ramp?:boolean;
}>;
/** 💬️ Words one owner pool message in English and German. */
export function ownerPoolTextV1(key:string,fields?:Readonly<Record<string,string|number>>):string;
/** 🧮️ The default capacity port: sizes the children from the host. */
export function defaultOwnerPoolCapacityV1(ask:OwnerPoolCapacityAskV1):Promise<unknown>;
/** 🏊️ Runs the owner pool of this machine until it is cancelled (or once). */
export function runOwnerPoolV1(options:OwnerPoolRunOptionsV1):Promise<unknown>;
