/** 🧠️ The declared guard of the reach sets. */
export const REACH_SETS_POLICY_V1:Readonly<{version:1;guardBytes:number}>;
/** 🧠️ A reach set: a dense bitset of 32 bit words over module numbers, shared by equal sets. */
export type ReachSetV1=Uint32Array;
/** 🧠️ The store of reach sets. */
export type ReachSetsV1=Readonly<{of(numbers:Iterable<number>):ReachSetV1;union(sets:Iterable<ReachSetV1>):ReachSetV1;has(set:ReachSetV1,number:number):boolean;members(set:ReachSetV1):IterableIterator<number>;retain(live:Iterable<ReachSetV1>):void;stats():{sets:number;bytes:number;guardBytes:number}}>;
/** 🧠️ Creates the store. */
export function createReachSetsV1(guardBytes?:number):ReachSetsV1;
