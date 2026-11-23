import{fileURLToPath}from"node:url";
import{WALK_FARM_V1,createWalkFarmV1,reserveWalkFarmV1,walkFarmCapacityV1,walkFarmSizeV1}from"../../../🧮️derived-nodes/🧩️closure-kinds/🏭️walk-farm/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🏭️ The declared limits of the owner discovery farm; the schema file is the single source. */
export const OWNER_FARM_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));

const POLICY=Object.freeze({...WALK_FARM_V1,...OWNER_FARM_V1}),DERIVER=fileURLToPath(new URL("./👷️deriver/🟨️.mjs",import.meta.url)),farms=new Map();

/** 🚫️ Whether the environment switched the farm off. */
export const ownerFarmOffV1=(environment=process.env)=>environment[OWNER_FARM_V1.environment]===OWNER_FARM_V1.disabledValue;

const closeFarm=root=>{const held=farms.get(root);if(held===undefined)return;farms.delete(root);clearTimeout(held.idle);held.farm.close();};
process.on("exit",()=>{for(const root of[...farms.keys()])closeFarm(root);});

/** 🧮️ The children one farm of this process may run: the granted processors minus the one the parent uses and the memory the grant of the process leaves, never fewer than two (one child beside the parent gains nothing).
 * @param {{cpus:number;availableBytes:number}} capacity */
export const ownerFarmSizeV1=capacity=>walkFarmSizeV1(capacity,POLICY);

/**
 * 🏭️ The discovery farm of one workspace, shared by every caller of this process: `discover(request, signal)` answers `{facts, rows}` (the admission facts and the rows their discovery observed) from a child or rejects, and the caller discovers the owner itself (the farm is never a reason for a read to fail or wait: `ready()` is false until a child is up and the caller does not wait for it). Children are booked as walk-farm workers of the memory plan of the process tree (`reserveWalkFarmV1`), started on the first call and closed after `idleMilliseconds` without one; undefined when the farm is off or fewer than two children are granted.
 * @param {string} workspaceRoot @param {{environment?:Record<string,string|undefined>;capacity?:{cpus:number;availableBytes:number};size?:number;spawn?:Function;module?:string}} [options]
 * @see ./🧬️schema/🔣️.json
 */
export function ownerFactsFarmV1(workspaceRoot,{environment=process.env,capacity,size,spawn,module=DERIVER}={}){
 if(ownerFarmOffV1(environment))return undefined;
 let held=farms.get(workspaceRoot);
 if(held===undefined){
  const lease=size===undefined?reserveWalkFarmV1(ownerFarmSizeV1(capacity??walkFarmCapacityV1()),{policy:POLICY,environment,yield:()=>closeFarm(workspaceRoot)}):undefined,children=size??lease.workers;
  if(children<2){lease?.release();return undefined;}
  const farm=createWalkFarmV1({root:workspaceRoot,size:children,module,policy:POLICY,...lease?{lease}:{},...spawn?{spawn}:{}});
  farm.warm();
  held={farm,idle:undefined,api:Object.freeze({size:children,ready:()=>farm.ready(),discover:(request,signal)=>farm.derive({kind:"owner-facts",path:JSON.stringify(request),fact:null},signal).then(answer=>answer.fact),stats:()=>farm.stats()})};
  farms.set(workspaceRoot,held);
 }
 clearTimeout(held.idle);held.idle=setTimeout(()=>closeFarm(workspaceRoot),OWNER_FARM_V1.idleMilliseconds);held.idle.unref?.();
 return held.api;
}

/** 🛑️ Ends the discovery farm of one workspace now (a test, the end of a read). */
export const closeOwnerFactsFarmV1=closeFarm;
