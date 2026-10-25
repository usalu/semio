import{otherLeaseHoldersV1}from"../../../../🔒️leases/🟦️.ts";
const held=new Map<string,number>();
/** 🧷️ Registers a child image this process created for its own parent and has not retired yet; the returned release is idempotent. */
export function holdNativeLiveChildV1(directory:string):()=>void{held.set(directory,(held.get(directory)??0)+1);let released=false;return()=>{if(released)return;released=true;const left=(held.get(directory)??1)-1;if(left>0)held.set(directory,left);else held.delete(directory);};}
/** 🫀️ Tells whether a child directory has a live user: this process registered it, or another live process holds its image lease (a leaked mount has neither). */
export function nativeLiveChildV1(directory:string,leaseDirectory:string):boolean{return held.has(directory)||otherLeaseHoldersV1(leaseDirectory,"native-fixed-image:"+directory).length>0;}
