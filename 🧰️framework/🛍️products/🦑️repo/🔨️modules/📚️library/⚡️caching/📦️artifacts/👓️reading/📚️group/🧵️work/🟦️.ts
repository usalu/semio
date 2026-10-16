import schema from "../🧬️schema/🔣️.json" with {type:"json"};
const maximum=schema.definitions.Protocol.const.maximumAcceptedTasks;
/** 🧵️ Owns finite Promise settlement only; it issues no reader, control, lease or store authority. */
export interface ArtifactGroupWorkV1{accept<T>(run:()=>Promise<T>):Promise<T>;close():Promise<readonly unknown[]>}
/** 💧️ Enrolls before callback effects and joins detached failures, including undefined, before closure. */
export function createArtifactGroupWorkV1():ArtifactGroupWorkV1{let accepting=true,accepted=0;const pending=new Set<Promise<unknown>>(),failures:unknown[]=[];return Object.freeze({accept<T>(run:()=>Promise<T>):Promise<T>{if(!accepting||accepted>=maximum||typeof run!=="function")throw Error("Artifact group task admission closed or capacity exceeded");accepted++;const work=Promise.resolve().then(run);pending.add(work);void work.then(()=>{pending.delete(work);},error=>{accepting=false;if(!failures.includes(error))failures.push(error);pending.delete(work);});return work;},async close():Promise<readonly unknown[]>{accepting=false;while(pending.size)await Promise.allSettled([...pending]);return Object.freeze([...failures]);}});}
