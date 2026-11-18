import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧮️ The numbers that decide when an interim fence of the dependency producer is paid in full.
 * @see ./🧬️schema/🔣️.json */
export const FENCE_BUDGET_POLICY_V1=schema.definitions.Policy.const;

/** 🧮️ What the producer remembers between two fences: when the last full fence ended, how long it took, how many interim fences were skipped since. */
export type FenceBudgetStateV1=Readonly<{lastEndMs:number|null;lastDurationMs:number;skipped:number}>;
/** 🧮️ What one fence site asks: the clock, and whether an irreversible step follows. */
export type FenceBudgetInputV1=Readonly<{nowMs:number;final:boolean}>;
/** 🧮️ The answer: pay in full or only check the store custody, and the state after the answer. */
export type FenceBudgetVerdictV1=Readonly<{due:boolean;state:FenceBudgetStateV1}>;

/** 🧮️ The state before the first fence: the first fence is always full. */
export const fenceBudgetStartV1=():FenceBudgetStateV1=>({lastEndMs:null,lastDurationMs:0,skipped:0});

/** 🧮️ Decides one fence site: a final fence is always full; an interim fence is full when no full fence has run yet or the last one ended at least `factor` times its duration (at most `maximumGapMilliseconds`) ago. A pure function.
 * @see ./🧪️tests/🐍️.py */
export function fenceDueV1(state:FenceBudgetStateV1,input:FenceBudgetInputV1,policy:typeof FENCE_BUDGET_POLICY_V1=FENCE_BUDGET_POLICY_V1):FenceBudgetVerdictV1{
 const due=input.final||state.lastEndMs===null||input.nowMs-state.lastEndMs>=Math.min(policy.maximumGapMilliseconds,policy.factor*state.lastDurationMs);
 return due?{due,state}:{due,state:{...state,skipped:state.skipped+1}};
}

/** 🧮️ The state after a full fence that started at `startMs` and ended at `endMs`. */
export const fenceBudgetDoneV1=(state:FenceBudgetStateV1,input:Readonly<{startMs:number;endMs:number}>):FenceBudgetStateV1=>({lastEndMs:input.endMs,lastDurationMs:Math.max(0,input.endMs-input.startMs),skipped:0});
