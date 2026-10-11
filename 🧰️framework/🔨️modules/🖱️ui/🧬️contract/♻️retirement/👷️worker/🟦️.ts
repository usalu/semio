/** 👷️ Fixed UI worker retirement policy, independent of owner quotes. */
export const UI_WORKER_RETIREMENT_POLICY=Object.freeze({maximum_items:1,maximum_copy_bytes:65536,maximum_capacity_bytes:65536,maximum_release_bytes:262144,maximum_depth:64});
export function uiWorkerRetirementPermits(demand:{copy_bytes:number;capacity_bytes:number;release_bytes:number;depth:number}):boolean{
 return Object.values(demand).every(value=>Number.isSafeInteger(value)&&value>=0)&&demand.copy_bytes<=UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes&&demand.capacity_bytes<=UI_WORKER_RETIREMENT_POLICY.maximum_capacity_bytes&&demand.release_bytes<=UI_WORKER_RETIREMENT_POLICY.maximum_release_bytes&&demand.depth<=UI_WORKER_RETIREMENT_POLICY.maximum_depth;
}

/** 🚥️ Structural refusal keeps the fixed caller policy and original owner. */
export function uiWorkerRetirementRefusal(demand:{copy_bytes:number;capacity_bytes:number;release_bytes:number;depth:number}):string|null{
 if(!Object.values(demand).every(value=>Number.isSafeInteger(value)&&value>=0))return "invalidValue";
 if(demand.depth>UI_WORKER_RETIREMENT_POLICY.maximum_depth)return "depthLimit";
 if(demand.capacity_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_capacity_bytes||demand.release_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_release_bytes)return "ownershipLimit";
 return demand.copy_bytes>UI_WORKER_RETIREMENT_POLICY.maximum_copy_bytes?"workLimit":null;
}
