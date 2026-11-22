import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** ♻️ The declared policy of the slice worker recycling: the schema file is the single source.
 * @see ./🧬️schema/🔣️.json */
export const SLICE_RECYCLE_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));

const MESSAGES_V1=Object.freeze(structuredClone(schema.definitions.Messages.const)),count=value=>Number.isSafeInteger(value)&&value>=0?value:0;

/** 🌱 The environment of a worker: the heap threshold, and the counters of the read when it continues a recycled one. */
export function sliceRecycleEnvironmentV1(state,policy=SLICE_RECYCLE_POLICY_V1){
 return Object.freeze({[policy.heapEnvironment]:String(policy.heapMegabytes),...state===undefined?{}:{[policy.stateEnvironment]:JSON.stringify(state)}});
}

/** 📦 The counters a worker continues from; zeros for a read that was never recycled or whose state is unreadable. */
export function sliceRecycleCarriedV1(environment=process.env,policy=SLICE_RECYCLE_POLICY_V1){
 let value;try{value=JSON.parse(environment[policy.stateEnvironment]??"null");}catch{value=null;}
 const state=value!==null&&typeof value==="object"?value:{};
 return Object.freeze({recycles:count(state.recycles),round:count(state.round),passes:count(state.passes),owners:count(state.owners),heapMegabytes:count(state.heapMegabytes)});
}

/** ⚖️ Whether a worker with `heapBytes` of live heap hands the read over now: never without a threshold in its environment, never after the maximum number of recycles. */
export function sliceRecycleDecisionV1({environment=process.env,heapBytes,carried=sliceRecycleCarriedV1(environment)},policy=SLICE_RECYCLE_POLICY_V1){
 const limit=Number(environment[policy.heapEnvironment]);
 if(!Number.isFinite(limit)||limit<=0||!Number.isFinite(heapBytes)||carried.recycles>=policy.maximumRecycles)return undefined;
 const heap=Math.round(heapBytes/1048576);
 return heap>limit?Object.freeze({heapMegabytes:heap,limitMegabytes:limit,recycles:carried.recycles+1}):undefined;
}

/** 🤝 The error that ends a worker at a pass boundary: both texts, the counters the next worker continues from. */
export function sliceRecycleErrorV1({decision,round,passes,owners,carried=sliceRecycleCarriedV1()},policy=SLICE_RECYCLE_POLICY_V1){
 const fields={heap:decision.heapMegabytes,limit:decision.limitMegabytes,round,recycles:decision.recycles,maximum:policy.maximumRecycles},render=template=>template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]??""));
 const state=Object.freeze({recycles:decision.recycles,round,passes:carried.passes+passes,owners,heapMegabytes:decision.heapMegabytes});
 return Object.assign(Error(render(MESSAGES_V1.en)+"\n"+render(MESSAGES_V1.de)),{code:"SEMIO_SLICE_RECYCLE",recycle:state});
}

/** 🔎 The counters of a recycle handover that `error` is or wraps (a cause up to four deep), nothing for every other failure. */
export function sliceRecycleOfV1(error){
 for(let depth=0,value=error;value!==null&&typeof value==="object"&&depth<=4;depth++,value=value.cause)if(value.code==="SEMIO_SLICE_RECYCLE"&&value.recycle!==undefined)return value.recycle;
 return undefined;
}
