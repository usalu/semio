/** 🏷️ The only argument of a process that serves as a graph farm worker from the snapshot its parent runs from. */
export const GRAPH_FARM_WORKER_ROLE_V1:"--semio-role=graph-farm-worker";
/** 🪪️ The captured implementation a farm process runs from (`null` when it loaded its modules from their files) and the identity its records are written under. */
export interface GraphFarmIdentityV1{readonly implementation:string|null;readonly records:string}
/** 🧾️ The farm identity of this process for a record identity; the implementation defaults to the snapshot its loader verified. */
export function graphFarmIdentityV1(records:string,implementation?:string|null):GraphFarmIdentityV1;
/** 🔍️ The parts of an announced identity that differ from `own`, empty when the worker may serve. */
export function graphFarmIdentityDifferenceV1(own:GraphFarmIdentityV1,offered:unknown):string;
/** ⚖️ Whether a worker that announced `offered` may serve a parent of identity `own`. */
export function sameGraphFarmIdentityV1(own:GraphFarmIdentityV1,offered:unknown):boolean;
/** 👷️ Serves as a graph farm worker inside a process started from a captured snapshot. */
export function runCapturedGraphFarmWorkerV1():Promise<void>;
