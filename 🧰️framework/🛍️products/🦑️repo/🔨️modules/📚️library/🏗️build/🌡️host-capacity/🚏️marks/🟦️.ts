import type {ResourceLeaseControl} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🚏️ Records which phase is running, for phase sizing and learning at the owning grant. */
export async function markBuildCapacityPhaseV1(phase:"assembly"|"plan"|"rebuild"|"execution",environment:Readonly<Record<string,string|undefined>>=process.env):Promise<boolean>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined)return false;
 if(environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return false;
 return(await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts")).markHostCapacityPhaseV1(environment,phase);
}

/** ⏫️ Obtains the fixed rebuild reservation before a builder or prefetch can hold graph custody. */
export async function upgradeBuildCapacityBeforeRebuildV1(environment:Readonly<Record<string,string|undefined>>=process.env,control?:ResourceLeaseControl,reuse?:()=>Promise<boolean>):Promise<(()=>void)|undefined>{
 if(environment["SEMIO_HOST_CAPACITY_GRANT"]===undefined)return undefined;
 const {createCapacityControlV1,upgradeBuildCapacityV1}=await import("../🟦️.ts"),waiting=control===undefined?createCapacityControlV1():undefined,wait=control??waiting!.control;
 try{
  const outcome=await upgradeBuildCapacityV1(wait,environment,reuse);
  if(outcome==="absent"||outcome==="reused")return undefined;
  if(environment["SEMIO_HOST_CAPACITY_PHASE_MARKS"]==="off")return undefined;
  const capacity=await import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts"),release=await capacity.requestHostRebuildCapacityV1(environment,wait,reuse);
  if(release===undefined)return undefined;
  const granted=capacity.readHostRebuildGrantMegabytesV1(environment),name="SEMIO_HOST_CAPACITY_REBUILD_MEGABYTES",mutable=environment as Record<string,string|undefined>;
  if(granted!==undefined)mutable[name]=String(granted);
  return()=>{delete mutable[name];release();};
 }finally{waiting?.dispose();}
}
