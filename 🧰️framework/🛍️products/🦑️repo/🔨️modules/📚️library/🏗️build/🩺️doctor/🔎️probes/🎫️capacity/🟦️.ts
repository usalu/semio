import{machinePathV1}from"../../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import {hostCapacityDirectoryV1,readHostCapacityStatusV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/📋️status/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1} from "../../../🌡️host-capacity/🎛️control/🟦️.ts";
import {join} from "node:path";
import type {DoctorPortsV1} from "../../🔌️ports/🟦️.ts";
import type {DoctorProbeV1,DoctorRepairRunV1} from "../📋️contract/🟦️.ts";

type GrantRow=Readonly<{id:string;label?:string;operation:string;pid:number;jobs:number;reservedMegabytes:number;observedMegabytes:number|null;admittedAtMs:number;lentSinceMs?:number}>;
type WaiterRow=Readonly<{request:Readonly<{id:string;operation:string;fixedMegabytes:number;minimumJobs:number;perJobMegabytes:number}>;pid:number;arrivedAtMs:number}>;

/** 💿️ Detaches the attached images of dead test sandboxes (identity by pid and start time, nested first) and removes the dead sandbox directories of this checkout; images of the checkout, of live sandboxes and foreign images are never touched. */
async function repairLeakedImages(ports:DoctorPortsV1,dryRun:boolean):Promise<DoctorRepairRunV1>{
 const parents=[machinePathV1(ports.root,".cache","test-sandbox")];
 const [{attachedDiskImagesV1},{classifyAttachedImagesV1,sweepLeakedSandboxImagesV1}]=await Promise.all([import("../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🖼️images/🟦️.ts"),import("../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🧹️image-sweep/🟦️.ts")]);
 if(dryRun){const leaked=classifyAttachedImagesV1(attachedDiskImagesV1()).filter(row=>row.kind==="leaked").length;return leaked===0?{removed:0,bytes:0,status:"nothing"}:{removed:leaked,bytes:0};}
 const report=sweepLeakedSandboxImagesV1({parents});
 return report.detached+report.sandboxesRemoved===0?{removed:0,bytes:0,status:"nothing"}:{removed:report.detached+report.sandboxesRemoved,bytes:0};
}

/** 🎫️ The machine-wide admission state and the host sample next to it: grants, waiters and why each waits, free memory against the floor, load, swap, open files and attached images. */
export const capacityProbeV1:DoctorProbeV1=Object.freeze({
 area:"capacity",phase:1,
 repairs:[Object.freeze({id:"host-leaked-images",area:"host" as const,run:(ports:DoctorPortsV1,_context:unknown,dryRun:boolean)=>repairLeakedImages(ports,dryRun)})],
 async read(ports){
  const control={signal:ports.signal,remainingMilliseconds:()=>null,publish:()=>undefined,yieldContinuation:()=>new Promise<void>(done=>setImmediate(done))};
  const now=ports.now(),status=await readHostCapacityStatusV1({directory:hostCapacityDirectoryV1(ports.environment),policy:BUILD_HOST_CAPACITY_POLICY_V1,control:control as never,now:ports.now});
  const holders=(status.grants as readonly GrantRow[]).filter(grant=>grant.lentSinceMs===undefined).map(grant=>({grant,megabytes:Math.max(grant.reservedMegabytes,grant.observedMegabytes??0)})).sort((left,right)=>right.megabytes-left.megabytes||left.grant.pid-right.grant.pid);
  const blockerOf=(id:string)=>{const largest=holders[0];return largest===undefined?{blocker:"-",holder:"none" as const}:{blocker:`${largest.grant.label??largest.grant.operation} (pid ${largest.grant.pid}, ${largest.megabytes} MB)`,holder:largest.grant.id===id?"self" as const:"other" as const};};
  const reasons=new Map<string,string>(status.evaluation.decisions.map((row:{id:string;reason:string})=>[row.id,row.reason])),host=status.host;
  return {sections:{
   host:{cores:host.cores,totalMegabytes:host.totalMegabytes,availableMegabytes:host.availableMegabytes,floorMegabytes:status.evaluation.floorMegabytes,pressure:host.pressure,swapUsedMegabytes:host.swapUsedMegabytes,swapOutMegabytesPerSecond:host.swapOutMegabytesPerSecond,load:host.load,openFiles:host.openFiles,openFilesMaximum:host.openFilesMaximum,attachedImages:host.attachedImages,governor:ports.environment.SEMIO_HOST_CAPACITY==="off"?"off" as const:"on" as const},
   capacity:{
    headroomMegabytes:status.evaluation.headroomMegabytes,
    grants:(status.grants as readonly GrantRow[]).map(grant=>({operation:grant.operation,pid:grant.pid,jobs:grant.jobs,reservedMegabytes:grant.reservedMegabytes,observedMegabytes:grant.observedMegabytes,heldSeconds:Math.max(0,Math.floor((now-grant.admittedAtMs)/1000))})),
    waiters:(status.waiters as readonly WaiterRow[]).map(waiter=>({operation:waiter.request.operation,pid:waiter.pid,needMegabytes:waiter.request.fixedMegabytes+waiter.request.minimumJobs*waiter.request.perJobMegabytes,waitedSeconds:Math.max(0,Math.floor((now-waiter.arrivedAtMs)/1000)),reason:reasons.get(waiter.request.id)??"queue-order",...blockerOf(waiter.request.id)})),
   },
  }};
 },
});
