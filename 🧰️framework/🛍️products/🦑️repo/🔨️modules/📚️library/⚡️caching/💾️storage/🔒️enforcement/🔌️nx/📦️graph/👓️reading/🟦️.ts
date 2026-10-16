import schema from"../🧬️schema/🔣️.json"with{type:"json"};
const policy=Object.freeze({...schema.definitions.ReadLifetimePolicyV1.const});
/** 👓️ Seals finite accepted reader promises before their actual Native namespace can close. */
export interface NxGraphReadLifetimeV1{accept<T>(run:()=>Promise<T>):Promise<T>;close():Promise<readonly unknown[]>}
/** 🛟️ Retains every accepted failure, including undefined, through idempotent drainage. */
export function createNxGraphReadLifetimeV1():NxGraphReadLifetimeV1{
 let accepting=true,closing:Promise<readonly unknown[]>|undefined;const pending=new Set<Promise<unknown>>(),failures:unknown[]=[];
 return Object.freeze({accept<T>(run:()=>Promise<T>):Promise<T>{if(!accepting||pending.size>=policy.maximumPendingOperations)throw Error("Bounded live graph reader admission required");let accept!:(value:T|PromiseLike<T>)=>void,refuse!:(error:unknown)=>void;const task=new Promise<T>((resolve,reject)=>{accept=resolve;refuse=reject;});pending.add(task);void task.then(()=>pending.delete(task),error=>{accepting=false;pending.delete(task);if(!failures.includes(error))failures.push(error);});try{accept(run());}catch(error){refuse(error);}return task;},close(){if(closing)return closing;accepting=false;closing=(async()=>{while(pending.size)await Promise.allSettled([...pending]);return Object.freeze([...failures]);})();return closing;}});
}
