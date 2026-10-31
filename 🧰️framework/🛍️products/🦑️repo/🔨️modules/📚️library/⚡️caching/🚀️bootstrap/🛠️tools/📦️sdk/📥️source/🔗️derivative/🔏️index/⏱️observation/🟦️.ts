import{AsyncLocalStorage}from"node:async_hooks";
/** ⏱️ Fixed aggregate categories retain real worker time without creating per-file profile rows. */
export function createNxSdkVerificationObservationV1(now:()=>number=()=>performance.now()){
 const totals:Record<string,[number,number]>=Object.fromEntries(["context","recheck","parent","lstat","publish","metadata","index","identity","hash"].map(name=>[name,[0,0]]));
 const snapshot=()=>Object.freeze(Object.fromEntries(Object.entries(totals).map(([name,[count,ms]])=>[name,Object.freeze([count,Math.round(ms*1000)/1000])])));
 const begin=(name:string)=>{const row=totals[name];if(!row)throw Error("Closed SDK verification metric required");const started=now();return()=>{row[0]++;row[1]+=now()-started;};};
 return Object.freeze({snapshot,measure<T>(name:string,body:()=>T):T{const finish=begin(name);try{return body();}finally{finish();}},async measureAsync<T>(name:string,body:()=>T|PromiseLike<T>):Promise<T>{const finish=begin(name);try{return await body();}finally{finish();}}});
}
const observations=new AsyncLocalStorage<ReturnType<typeof createNxSdkVerificationObservationV1>>();
/** 🪢️ Scopes only timing DATA while the original verification context remains unchanged. */
export function withNxSdkVerificationObservationV1<T>(observation:ReturnType<typeof createNxSdkVerificationObservationV1>,body:()=>T):T{return observations.run(observation,body);}
/** ⏱️ Times an actual synchronous context call without projecting or replacing its receiver. */
export function measureNxSdkVerificationV1<T>(name:string,body:()=>T):T{const observation=observations.getStore();return observation?observation.measure(name,body):body();}
/** ⏳️ Times actual asynchronous verification work and retains its exact resolved value or rejection. */
export async function measureNxSdkVerificationAsyncV1<T>(name:string,body:()=>T|PromiseLike<T>):Promise<T>{const observation=observations.getStore();return observation?await observation.measureAsync(name,body):await body();}
