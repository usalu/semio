import{AsyncLocalStorage}from"node:async_hooks";

const storage=new AsyncLocalStorage();

/** 🛑️ The error that stops the inference of ONE project: it grew past what this machine can give one project. It carries which owner, how far it had grown and the limit, so the caller can mark the owner too heavy here and ask the fleet. */
export class GraphOwnerTooHeavyErrorV1 extends Error{
 constructor(detail){
  super("Graph owner too heavy for this machine: "+detail.name+" ("+Math.round(detail.takenBytes/1048576)+" MB of "+Math.round(detail.limitBytes/1048576)+" MB)");
  this.name="GraphOwnerTooHeavyErrorV1";this.code="BUILD_GRAPH_OWNER_TOO_HEAVY";this.detail=Object.freeze({...detail});
 }
}

/** 🎯️ The scope of one owner's inference: the governor marks it exceeded, the inference notices at its next yield point; `credit(bytes)` records memory the owner built for every owner after it (a shared warm-up), which the governor never charges to this owner. */
export function createOwnerScopeV1(name){
 let exceeded,credited=0;
 return Object.freeze({name,exceed(detail){exceeded??=Object.freeze({...detail});},credit(bytes){if(bytes>0)credited+=bytes;},get credited(){return credited;},get exceeded(){return exceeded;}});
}

/** 🤝️ Credits `bytes` that the running owner built for the owners after it (see {@link createOwnerScopeV1}); silent outside any scope. */
export function creditOwnerSharedV1(bytes){storage.getStore()?.credit(bytes);}

/** 🏃️ Runs `work` (and everything it awaits) inside the scope of one owner. */
export const runInOwnerScopeV1=(scope,work)=>storage.run(scope,work);

/** 🚧️ Throws {@link GraphOwnerTooHeavyErrorV1} when the owner this code runs for was stopped; a constant-time check for the yield points of the inference, silent outside any scope. */
export function throwIfOwnerExceededV1(){
 const scope=storage.getStore();
 if(scope!==undefined&&scope.exceeded!==undefined)throw new GraphOwnerTooHeavyErrorV1({...scope.exceeded,name:scope.name});
}

/** 🔎️ The detail of an owner stop in `error` or its causes (a failure tag wraps errors), else undefined. */
export function ownerTooHeavyOfV1(error,depth=6){
 if(typeof error!=="object"||error===null||depth<0)return undefined;
 if(error instanceof GraphOwnerTooHeavyErrorV1||error.code==="BUILD_GRAPH_OWNER_TOO_HEAVY")return error.detail;
 return ownerTooHeavyOfV1(error.cause,depth-1);
}
