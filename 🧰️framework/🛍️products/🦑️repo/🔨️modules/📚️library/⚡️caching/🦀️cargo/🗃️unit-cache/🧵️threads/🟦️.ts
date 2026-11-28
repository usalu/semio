import {UNIT_CACHE_POLICY_V1} from "../🔏️key/🟦️.ts";

const policy=UNIT_CACHE_POLICY_V1.threads;
export type UnitThreadsHostV1=Readonly<{level:"calm"|"guard"|"danger";availableMegabytes:number;guardMegabytes:number}>;
export type UnitThreadsInputV1=Readonly<{args:readonly string[];budgetMegabytes:number;peakMegabytes?:number;host?:UnitThreadsHostV1}>;
export type UnitThreadsVerdictV1=Readonly<{threads:number;reason:"peak"|"host-memory"}>;

/** 🧵️ How many threads the parallel front end of one compile unit gets: the constrained count when its measured peak is above the library budget or the host is below its memory guard, else `undefined` (the repository's rustflags decide). Only an actual compile unit on a toolchain that already takes unstable flags is decided; the unit key never sees it because the flag is appended after the key is taken, and the outputs do not depend on it ([`🧪️tests/🧵️threads`](../🧪️tests/🧵️threads/🟦️.ts)). */
export function unitThreadsVerdictV1(input:UnitThreadsInputV1):UnitThreadsVerdictV1|undefined{
 const compile=input.args.includes("--crate-name")&&input.args.includes("--out-dir")&&input.args.some(argument=>argument==="-Z"||argument.startsWith("-Z"));
 if(!compile)return undefined;
 if(input.peakMegabytes!==undefined&&input.peakMegabytes>input.budgetMegabytes)return Object.freeze({threads:policy.constrained,reason:"peak"});
 const host=input.host;
 if(host!==undefined&&(host.level!=="calm"||host.availableMegabytes<host.guardMegabytes))return Object.freeze({threads:policy.constrained,reason:"host-memory"});
 return undefined;
}

/** 🚩️ The compiler arguments of a verdict: the later `-Z threads` wins over the one of the rustflags. */
export function unitThreadsArgumentsV1(verdict:UnitThreadsVerdictV1|undefined):readonly string[]{return verdict===undefined?[]:["-Z","threads="+verdict.threads];}
