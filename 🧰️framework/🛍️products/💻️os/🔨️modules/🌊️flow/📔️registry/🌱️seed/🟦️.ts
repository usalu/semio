/** 🌱️ Typed node cache authority owned by the registry. */
export interface FlowNodeCachePort<Dictionary extends object>{seed(nodeHash:bigint,output:Dictionary):void}
/** 🌱️ Transfers a typed output into the owning cache. */
export function seedFlowEvalNodeCache<Dictionary extends object>(cache:FlowNodeCachePort<Dictionary>,nodeHash:bigint,output:Dictionary):void{cache.seed(nodeHash,output)}
