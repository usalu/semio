export type StaleReexecOutcomeV1=Readonly<{status:"completed"}|{status:"yielded"|"exhausted"|"reexecuted";code:number}>;
export function launchReplacementProcessV1(options:{execPath:string;args:readonly string[];environment:Readonly<Record<string,string|undefined>>}):Promise<number>;
export function runWithStaleReexecV1(run:()=>Promise<void>,options?:{environment?:Readonly<Record<string,string|undefined>>;outermost?:boolean;argv?:readonly string[];execPath?:string;execArgv?:readonly string[];launch?:typeof launchReplacementProcessV1;write?:(text:string)=>void;ranMilliseconds?:number;sleep?:(milliseconds:number)=>Promise<void>;now?:()=>number;yielded?:(environment:Readonly<Record<string,string|undefined>>)=>Promise<Readonly<Record<string,string>>|undefined>}):Promise<StaleReexecOutcomeV1>;

/** 🧾️ The stale implementation a run yielded for. */
export type StaleImplementationV1=NonNullable<ReturnType<typeof import("../🟨️.mjs").staleImplementationOfV1>>;
/** 🧬️ Names the live values of the stamps that moved: a restart that finds other live values than the one before followed a tree that kept moving under it (a peer edit), a restart that finds the same ones again did not converge. */
export function movedSignatureV1(stale: StaleImplementationV1): string;
/** 🧭️ Names what asked for a restart: the path and reason of a concurrent edit, the declared stamps that moved, or the half-edited module; empty when a nested command only passed on its exit code. */
export function restartCauseV1(stale: StaleImplementationV1): string;
