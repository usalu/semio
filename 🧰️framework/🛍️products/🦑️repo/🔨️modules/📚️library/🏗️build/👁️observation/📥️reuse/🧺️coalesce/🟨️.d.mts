export interface PresenceCoalescerStatsV1{readonly asked:number;readonly remembered:number;readonly batches:number;readonly retried:number;readonly names:number;}
export interface PresenceCoalescerV1{
 /** 🪣️ Whether every name is present: from the memory of `round` when it was found before, else through the next batch. */
 ask(names:readonly string[],round?:object,signal?:AbortSignal):Promise<boolean>;
 stats():PresenceCoalescerStatsV1;
}
/** 🧺️ The verdict of a presence check shared by every look that asks at the same time; `run(names)` answers the set of missing names and throws for a refusal. */
export function createPresenceCoalescerV1(options:{run(names:readonly string[],signal?:AbortSignal):Promise<ReadonlySet<string>>;maximumNames?:number;pause?():Promise<unknown>}):PresenceCoalescerV1;
