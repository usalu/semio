import{AsyncLocalStorage}from"node:async_hooks";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
const policy=Object.freeze({...schema.definitions.ReadLifetimePolicyV1.const});
type ReaderFrame={owner:object;live:boolean};
const frames=new AsyncLocalStorage<readonly ReaderFrame[]>();
/** 👓️ Seals finite accepted reader promises before their actual Native namespace can close. */
export interface NxGraphReadLifetimeV1{accept<T>(run:()=>Promise<T>):Promise<T>;close():Promise<readonly unknown[]>}
/** 🛟️ Retains every accepted failure, including undefined, through idempotent drainage. */
export function createNxGraphReadLifetimeV1():NxGraphReadLifetimeV1{
 let accepting=true,closing:Promise<readonly unknown[]>|undefined;const owner={},pending=new Set<Promise<unknown>>(),failures:unknown[]=[];
 return Object.freeze({accept<T>(run:()=>Promise<T>):Promise<T>{if(!accepting||pending.size>=policy.maximumPendingOperations)throw Error("Bounded live graph reader admission required");let accept!:(value:T|PromiseLike<T>)=>void,refuse!:(error:unknown)=>void;const task=new Promise<T>((resolve,reject)=>{accept=resolve;refuse=reject;}),frame={owner,live:true};pending.add(task);void task.then(()=>{frame.live=false;pending.delete(task);},error=>{frame.live=false;accepting=false;pending.delete(task);if(!failures.includes(error))failures.push(error);});try{accept(frames.run([...(frames.getStore()??[]),frame],run));}catch(error){refuse(error);}return task;},close(){if(frames.getStore()?.some(frame=>frame.owner===owner&&frame.live))throw Error("Graph owner cannot await its own accepted reader");if(closing)return closing;accepting=false;closing=(async()=>{while(pending.size)await Promise.allSettled([...pending]);return Object.freeze([...failures]);})();return closing;}});
}
