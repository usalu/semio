import{join}from"node:path";
import{KIND_RULES_V1}from"../🧩️kinds/🟨️.mjs";
import{capacityOfGrantV1,createDerivedGraphV1}from"../🟨️.mjs";
import{createNodeStoreV1}from"../💾️store/🟨️.mjs";
import{createStatIndexV1}from"../🗂️stat-index/🟨️.mjs";
import{createDiskWorldV1}from"../🌍️world/🟨️.mjs";

/**
 * 🏭️ The derived-node graph of one workspace with every durable part wired: the disk world (remembered directories, one stat index shared with the graph), the node store (records above the limit as chunks of `blobs`, a host CAS) and the capacity of the grant the scheduler admitted. `cacheRoot` is the graph cache of the ledger (`derived-nodes` and `stat-index` below it are the declared homes `graph-derived-nodes` and `graph-stat-index`); `grant()` answers `{handle, limit}` of `withBuildCapacityV1` (or undefined when ungranted).
 * @see ../../../⚡️caching/💾️storage/🧾️ledger/🔣️policy.json
 */
export function createWorkspaceGraphV1({workspaceRoot,cacheRoot,grant=()=>undefined,blobs,skip=[],descends,rules=KIND_RULES_V1,environment,environmentKey,verifyObserved,trace=false}){
 const index=createStatIndexV1({root:join(cacheRoot,"stat-index")}),store=createNodeStoreV1({root:join(cacheRoot,"derived-nodes"),blobs});
 const world=createDiskWorldV1(workspaceRoot,{skip,descends,remember:true,index});
 const graph=createDerivedGraphV1({rules,world,store,statIndex:index,capacity:capacityOfGrantV1(grant),environment,environmentKey,verifyObserved,trace});
 return Object.freeze({graph,world,index,store});
}
