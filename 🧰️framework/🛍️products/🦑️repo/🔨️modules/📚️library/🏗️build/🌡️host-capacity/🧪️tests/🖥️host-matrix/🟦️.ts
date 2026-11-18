/** 🖥️ Contract: the memory gates of a worker agree with each other on every machine class. For hosts of 8 GB / 4 cores, 16 GB / 8 cores and 32 GB / 10 cores with half and three quarters of the memory free, the REAL decision functions (capacity governor, plan limit, service limit, slice booking, farm width, owner pool limits, solo grant, per-owner share) answer for the read parent, the farm child, the pool child, `dependencies`, a build job and the service guard, and the answers obey five laws: a stage fits the host with the floor intact; the heaviest measured owner fits alone; no class books more than twice what it has measurably used; limits grow with the host; no gate asks for more free memory than half the host.
 * @see ../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts */
import {expect,test} from "bun:test";
import {writeFileSync} from "node:fs";
import {evaluateHostCapacityV1,hostFloorMegabytesV1,sizeHostRequestV1,unsatisfiableHostRequestV1,type HostGrantV1,type HostSampleV1,type HostWaiterV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import {BUILD_HOST_CAPACITY_POLICY_V1 as policy} from "../../🟦️.ts";
import {ownerGrantV1,ownerCostPolicyV1,soloGrantV1} from "../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";
import {wantedPolicyV1} from "../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import {graphServiceLimitMegabytesV1} from "../../../🧠️memory/🟨️.mjs";
import {planStageLimitBytesV1,planFarmWorkersV1,farmAloneCapV1,PLAN_STAGE_POLICY_V1} from "../../../🧠️memory/🪜️plan-stage/🟨️.mjs";
import {LEAN_SLICE_POLICY_V1} from "../../../🧠️memory/🪶️lean/🟨️.mjs";
import {sliceBookingV1,slicePlanCeilingMegabytesV1} from "../../../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs";
import {ownerPoolChildLimitsV1,ownerPoolSizingV1} from "../../../🕸️graph/📥️inference/📦️project-nodes/🌐️owner-pool/🟦️.ts";
import residentSchema from "../../../🏡️resident/🧬️schema/🔣️.json" with {type:"json"};

const now=1_800_000_000_000;
const find=(node:unknown,key:string):any=>{if(node===null||typeof node!=="object")return undefined;if(key in (node as object))return (node as any)[key];for(const child of Object.values(node as object)){const hit=find(child,key);if(hit!==undefined)return hit;}return undefined;};
const guard=find(residentSchema,"slicePlanGuard"),elasticity=find(residentSchema,"sliceHostElasticity");

/** 📏️ Footprints measured on this repository (R129 farm child 2026-11-17; `estimates.json` of the host gate for the classes; R117 parent): the evidence the bookings are held against. */
export const MEASURED={farmChildColdPeak:930,farmChildWorking:560,readParentWorking:3500,dependenciesPeak:2921,dependenciesMedian:1273,buildJobPeak:4256,buildJobMedian:1004} as const;
export const HOSTS=[{name:"8 GB / 4 cores",total:8192,cores:4},{name:"16 GB / 8 cores",total:16384,cores:8},{name:"32 GB / 10 cores",total:32768,cores:10}] as const;
export const FREE=[0.5,0.75] as const;
type Host=typeof HOSTS[number];

const sample=(host:Host,free:number):HostSampleV1=>({version:1,atMs:now,platform:"darwin",cores:host.cores,totalMegabytes:host.total,availableMegabytes:Math.floor(host.total*free),pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:null,openFilesMaximum:null,attachedImages:null}) as HostSampleV1;
const waiter=(operation:string,id:string,jobs:number,change:Record<string,unknown>={}):HostWaiterV1=>({request:{...sizeHostRequestV1(policy,{id,operation,preferredJobs:jobs,kinds:["build"]})!,...change},arrivedAtMs:now-1000,pid:4300}) as HostWaiterV1;
const parentGrant=(reserved:number,observed:number):HostGrantV1=>({id:"readparent1",operation:"show",jobs:1,cpu:2,reservedMegabytes:reserved,admittedAtMs:now-120_000,observedMegabytes:observed,pid:4200,bootAtMs:1_700_000_000_000,label:"projects"}) as HostGrantV1;
const decide=(host:Host,free:number,grants:HostGrantV1[],entry:HostWaiterV1)=>{const row=evaluateHostCapacityV1(policy,sample(host,free),grants,[entry],now).decisions.find(each=>each.id===entry.request.id)!;return {admitted:row.verdict==="admit",jobs:row.verdict==="admit"?row.jobs:0,reserved:row.reservedMegabytes,reason:row.reason};};
const footprint=(managed:number)=>managed*ownerCostPolicyV1.footprintFactor*(1+ownerCostPolicyV1.marginPercent/100);

export type Cell={host:string;free:number;cls:string;admitted:boolean;jobs:number;reserved:number;limit:number;note:string};
export function buildMatrix():{cells:Cell[];byHost:Record<string,Record<string,number>>}{
 const cells:Cell[]=[],byHost:Record<string,Record<string,number>>={};
 for(const host of HOSTS){
  const floor=hostFloorMegabytesV1(policy,sample(host,1)),lean=LEAN_SLICE_POLICY_V1,env={} as Record<string,string|undefined>;
  const read=sizeHostRequestV1(policy,{id:"readparent1",operation:"show"})!,dependencies=sizeHostRequestV1(policy,{id:"depsreq0001",operation:"dependencies"})!,farmAsk=waiter("exec","farmreq0001",host.cores-1,{perJobMegabytes:1536,fixedMegabytes:0,cpuPerJob:1,elastic:true,essential:true,label:"graph-farm",holder:"readparent1",minimumJobs:1});
  const solo=soloGrantV1({hostMegabytes:host.total},wantedPolicyV1),share=ownerGrantV1(Math.floor(host.total/2),wantedPolicyV1);
  const planLimit=planStageLimitBytesV1(env,host.total,PLAN_STAGE_POLICY_V1)/1048576,service=graphServiceLimitMegabytesV1(host.total),farmWidth=planFarmWorkersV1({workerMegabytes:1536,residentMegabytes:PLAN_STAGE_POLICY_V1.parentFootprintMegabytes,environment:env,hostMegabytes:host.total,policy:PLAN_STAGE_POLICY_V1,kind:"project-farm"}),pool=ownerPoolChildLimitsV1(host.total);
  const booking=sliceBookingV1({learnedMegabytes:0,unlearnedMegabytes:elasticity.minimumMegabytes,maximumMegabytes:elasticity.maximumMegabytes,hostMegabytes:host.total,hostCapBasisPoints:elasticity.hostCapBasisPoints,minimumMegabytes:elasticity.minimumMegabytes}),ceiling=slicePlanCeilingMegabytesV1({hostMegabytes:host.total,ceilingBasisPoints:guard.ceilingBasisPoints,planLimitMegabytes:planLimit});
  byHost[host.name]={floor,readFixed:read.fixedMegabytes,readMinimum:read.minimumMegabytes??read.fixedMegabytes,dependenciesFixed:dependencies.fixedMegabytes,dependenciesMinimum:dependencies.minimumMegabytes??dependencies.fixedMegabytes,farmPerChild:farmAsk.request.perJobMegabytes,soloManaged:solo.managedMegabytes,soloFootprint:solo.footprintMegabytes,shareManaged:share.managedMegabytes,planLimit,service,farmWidth,farmAloneCap:farmAloneCapV1(host.total),poolRecycle:pool.recycleMegabytes,poolHard:pool.hardMegabytes,sliceBooking:booking.wantedMegabytes,sliceMinimum:booking.minimumMegabytes,planCeiling:ceiling===0?Infinity:ceiling,leanParent:lean.parentMegabytes,leanChild:lean.childMegabytes};
  for(const free of FREE){
   const parent=decide(host,free,[],waiter("show","readparent1",1));cells.push({host:host.name,free,cls:"read parent",...parent,limit:parent.reserved,note:parent.reason});
   const held=parentGrant(Math.max(parent.reserved,read.minimumMegabytes??0),Math.min(parent.reserved,MEASURED.readParentWorking)),child=decide(host,free,[held],farmAsk);cells.push({host:host.name,free,cls:"farm child",...child,limit:child.reserved,note:child.reason});
   const deps=decide(host,free,[],waiter("dependencies","depsreq0001",1));cells.push({host:host.name,free,cls:"dependencies",...deps,limit:deps.reserved,note:deps.reason});
   const buildAsk=waiter("cargo","buildreq001",Math.max(1,host.cores-1)),build=decide(host,free,[],buildAsk),room=Math.floor(host.total*free)-floor,need=buildAsk.request.fixedMegabytes+buildAsk.request.perJobMegabytes;
   const bypass=!build.admitted&&unsatisfiableHostRequestV1(policy,{waitedMilliseconds:policy.wait.bypassMilliseconds,roomMegabytes:room,drainedMegabytes:room,needMegabytes:need,admitted:0,replaced:false,quiet:false,elastic:false,pause:undefined});
   cells.push({host:host.name,free,cls:"build job",admitted:build.admitted||bypass,jobs:build.admitted?build.jobs:1,reserved:build.admitted?build.reserved:Math.max(0,room),limit:build.admitted?build.reserved:Math.max(0,room),note:build.admitted?build.reason:`${build.reason}, bypass after ${policy.wait.bypassMilliseconds/1000} s with ${Math.max(0,room)} MB (asked ${need})`});
   const pooled=ownerPoolSizingV1({host:{cores:host.cores,totalMegabytes:host.total,availableMegabytes:Math.floor(host.total*free),pressure:"normal",load:1,swapOutMegabytesPerSecond:0,foreground:{cpu:0,unrealizedMegabytes:0,waiters:0}},children:0,childMegabytes:MEASURED.farmChildColdPeak});cells.push({host:host.name,free,cls:"pool child",admitted:pooled.children>=1,jobs:pooled.children,reserved:pooled.children*MEASURED.farmChildColdPeak,limit:pool.hardMegabytes,note:pooled.reason??"fits"});
   const soloHere=soloGrantV1({hostMegabytes:host.total,availableMegabytes:Math.floor(host.total*free)},wantedPolicyV1);cells.push({host:host.name,free,cls:"solo owner",admitted:soloHere.footprintMegabytes>=MEASURED.farmChildColdPeak,jobs:1,reserved:MEASURED.farmChildColdPeak,limit:soloHere.footprintMegabytes,note:`${soloHere.managedMegabytes} managed MB`});
   cells.push({host:host.name,free,cls:"service guard",admitted:service>=read.minimumMegabytes!+lean.childMegabytes&&planLimit>=lean.parentMegabytes+lean.childMegabytes,jobs:1,reserved:(read.minimumMegabytes??0)+lean.childMegabytes,limit:Math.min(service,planLimit),note:`service ${service}, plan ${planLimit}`});
  }
 }
 return {cells,byHost};
}
export function renderMatrix(matrix:ReturnType<typeof buildMatrix>):string{
 const lines=["| host | free | class | admitted | jobs | reserved MB | limit MB | note |","|---|---|---|---|---|---|---|---|"];
 for(const cell of matrix.cells)lines.push(`| ${cell.host} | ${cell.free*100} % | ${cell.cls} | ${cell.admitted?"yes":"NO"} | ${cell.jobs} | ${cell.reserved} | ${cell.limit} | ${cell.note} |`);
 lines.push("","| host | "+Object.keys(matrix.byHost[HOSTS[0].name]!).join(" | ")+" |","|---|"+Object.keys(matrix.byHost[HOSTS[0].name]!).map(()=>"---").join("|")+"|");
 for(const host of HOSTS)lines.push(`| ${host.name} | ${Object.values(matrix.byHost[host.name]!).join(" | ")} |`);
 return lines.join("\n");
}
const matrix=buildMatrix();
if(process.env.R135_PRINT)writeFileSync(process.env.R135_PRINT,renderMatrix(matrix));

test("every unit of work is admitted with half of the memory free, on every host",()=>{
 const refused=matrix.cells.filter(cell=>!cell.admitted).map(cell=>`${cell.host} @${cell.free*100}% ${cell.cls}: ${cell.note}`);
 expect(refused).toEqual([]);
});

test("a stage fits the host with the floor intact: read parent minimum, one farm child and the service guard together",()=>{
 for(const host of HOSTS){const row=matrix.byHost[host.name]!;expect(row.readMinimum!+row.farmPerChild!+row.floor!,host.name).toBeLessThanOrEqual(host.total);expect(row.leanParent!+row.leanChild!+row.floor!,host.name).toBeLessThanOrEqual(host.total);}
});

test("the heaviest measured owner fits alone: the solo footprint, the farm width, the pool kill limit and the plan limit each carry one cold Rust owner",()=>{
 for(const host of HOSTS){const row=matrix.byHost[host.name]!;
  expect(row.soloFootprint!,host.name+" solo").toBeGreaterThanOrEqual(MEASURED.farmChildColdPeak*2);
  expect(row.farmWidth!,host.name+" farm width").toBeGreaterThanOrEqual(1);
  expect(row.farmAloneCap!,host.name+" alone cap").toBeGreaterThanOrEqual(MEASURED.farmChildColdPeak*2);
  expect(row.farmAloneCap!+PLAN_STAGE_POLICY_V1.parentFootprintMegabytes,host.name+" alone cap and parent stay under the plan ceiling").toBeLessThanOrEqual(host.total*LEAN_SLICE_POLICY_V1.ceilingBasisPoints/10000);
  expect(row.poolHard!,host.name+" pool").toBeGreaterThanOrEqual(MEASURED.farmChildColdPeak);
  expect(row.planLimit!,host.name+" plan").toBeGreaterThanOrEqual(row.leanParent!+row.leanChild!);
 }
});

test("the heaviest owner that fits the host at all is never refused by the solo limit: nothing between the whole plan and the solo grant",()=>{
 for(const host of HOSTS){const row=matrix.byHost[host.name]!,whole=host.total-row.readFixed!-row.floor!;expect(row.soloFootprint!,host.name).toBeGreaterThanOrEqual(whole);}
});

test("no class books more than twice what it has measurably used, and none books less than it measurably needs",()=>{
 for(const host of HOSTS){const row=matrix.byHost[host.name]!;
  expect(row.farmPerChild!,host.name).toBeLessThanOrEqual(2*MEASURED.farmChildColdPeak);
  expect(row.dependenciesFixed!,host.name).toBeLessThanOrEqual(2*MEASURED.dependenciesPeak);
  expect(row.dependenciesMinimum!,host.name).toBeGreaterThanOrEqual(MEASURED.dependenciesMedian);
  expect(row.readMinimum!,host.name).toBeGreaterThanOrEqual(MEASURED.farmChildWorking+row.leanParent!);
 }
});

test("limits grow with the host: grants, shares, plan and service limits, farm width, pool limits and slice bookings never fall from a smaller host to a larger one",()=>{
 const keys=Object.keys(matrix.byHost[HOSTS[0].name]!).filter(key=>!["floor","readFixed","readMinimum","dependenciesFixed","dependenciesMinimum","farmPerChild","leanParent","leanChild","sliceMinimum"].includes(key));
 for(const key of keys)for(let at=1;at<HOSTS.length;at++)expect(matrix.byHost[HOSTS[at]!.name]![key]!,key).toBeGreaterThanOrEqual(matrix.byHost[HOSTS[at-1]!.name]![key]!);
});

test("no gate asks for more free memory than half the host has: minimum plus floor of every fixed request stays within half of the host",()=>{
 for(const host of HOSTS){const row=matrix.byHost[host.name]!;for(const key of ["readMinimum","dependenciesMinimum"])expect(row[key]!+row.floor!,host.name+" "+key).toBeLessThanOrEqual(host.total/2);}
});
