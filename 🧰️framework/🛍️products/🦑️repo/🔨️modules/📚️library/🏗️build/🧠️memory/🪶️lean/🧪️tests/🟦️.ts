import{expect,test}from"bun:test";
import{spawn}from"node:child_process";
import Ajv from"ajv";
import schema from"../🧬️schema/🔣️.json";
import{LEAN_SLICE_MESSAGES_V1,LEAN_SLICE_POLICY_V1,leanFarmPolicyV1,leanSlicePlanV1,renderLeanSliceMessageV1}from"../🟨️.mjs";
import{PLAN_STAGE_POLICY_V1,farmReservationsV1,leanSliceV1,planFarmWorkersV1,reserveFarmV1}from"../../🪜️plan-stage/🟨️.mjs";
import{GRAPH_FARM_POLICY_V1,planGraphFarmV1}from"../../../🕸️graph/📥️inference/📦️project-nodes/🏊️project-farm/🏊️pool/🟨️.mjs";
import{HOST_POLICY_V1}from"../../../🏡️resident/🟨️.mjs";
import{observeHostTreeMegabytesV1}from"../../../🌡️host-capacity/🟦️.ts";

/** 🪶️ Contract: the lean slice read. A machine with a plan limit keeps the long-lived slice worker small and infers every stale owner in short-lived children; the plan equals an independent model, the ledger grants the first child on every declared machine and never more than the limit allows, and real child processes that infer owners in batches keep the summed tree under the plan limit with a parent whose size does not grow per owner.
 * @see ../🧬️schema/🔣️.json */
const P=LEAN_SLICE_POLICY_V1,MB=1048576,share=PLAN_STAGE_POLICY_V1.treeShare;
const env=(extra:Record<string,string>={})=>({...extra}) as Record<string,string|undefined>;

test("the policy and the messages are closed under the schema, checked by an independent implementation; both languages carry the same fields",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(schema.definitions.Plan,leanSlicePlanV1({limitMegabytes:3276,cores:2,environment:env()})),JSON.stringify(ajv.errors)).toBe(true);
 const fields=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(row=>row[1]).sort();
 for(const key of Object.keys(LEAN_SLICE_MESSAGES_V1.en))expect(fields(LEAN_SLICE_MESSAGES_V1.de[key]!),key).toEqual(fields(LEAN_SLICE_MESSAGES_V1.en[key]!));
 expect(renderLeanSliceMessageV1("lean",{host:1,parent:2,children:3,child:4,owners:5},env()).length).toBe(2);
 expect(renderLeanSliceMessageV1("lean",{host:1,parent:2,children:3,child:4,owners:5},env({SEMIO_LANGUAGE:"de"})).length).toBe(1);
 expect(P.ceilingBasisPoints).toBe(HOST_POLICY_V1.slicePlanGuard.ceilingBasisPoints);
});

test("the plan equals an independent model for a grid of limits, cores, resident sizes and modes",()=>{
 const model=(limit:number,cores:number,resident:number,mode:string)=>{
  if(mode==="off"||mode==="auto"&&!Number.isFinite(limit))return{lean:false};
  const parent=Math.max(1500,Math.ceil(resident)),fit=Number.isFinite(limit)?Math.floor((limit-parent)/1700):1e9,cap=Math.max(1,cores-1);
  return{lean:true,children:Math.min(cap,Math.max(1,fit)),parent};
 };
 let rows=0;
 for(const limit of[2048,3276,4096,6553,8192,Infinity])for(const cores of[1,2,4,8,16])for(const resident of[0,900,1500,2600,3400])for(const mode of["auto","on","off"]){
  const got=leanSlicePlanV1({limitMegabytes:limit,cores,residentMegabytes:resident,environment:env({SEMIO_SLICE_LEAN:mode})}),want=model(limit,cores,resident,mode);
  expect(got.lean,JSON.stringify({limit,cores,resident,mode})).toBe(want.lean);
  if(want.lean){expect(got.children).toBe(want.children!);expect(got.parentMegabytes).toBe(want.parent!);}
  rows++;
 }
 expect(rows).toBe(450);
 expect(()=>leanSlicePlanV1({limitMegabytes:1,environment:env({SEMIO_SLICE_LEAN:"maybe"})})).toThrow();
});

test("a lean read farms with one child on a two-core machine; the declared farm refused it (cpu), so the owners stayed in the parent",()=>{
 expect(planGraphFarmV1({projects:700,environment:{},cores:2}).workers,"control: the declared farm needs two workers").toBe(0);
 expect(planGraphFarmV1({projects:700,environment:{},cores:2,minimumWorkers:1}).workers).toBe(1);
 const plan=leanSlicePlanV1({limitMegabytes:3276,cores:2,environment:env()}),farm=leanFarmPolicyV1(GRAPH_FARM_POLICY_V1,plan);
 expect(farm.workerMegabytes).toBe(1700);expect(farm.loopsPerWorker).toBe(1);expect(farm.recycleAfterProjects).toBe(4);
 expect(leanFarmPolicyV1(GRAPH_FARM_POLICY_V1,leanSlicePlanV1({limitMegabytes:Infinity,environment:env()}))).toBe(GRAPH_FARM_POLICY_V1);
});

test("the ledger grants the first project child on every declared machine from 8 GB up, whatever the parent holds, and more children only where the limit has room after the parent",()=>{
 const input=(host:number,resident:number)=>({hostMegabytes:host,residentMegabytes:resident,environment:env()});
 for(const resident of[900,1500,2300,3000]){
  const lease=reserveFarmV1({...input(8192,resident),kind:"project-farm",workerMegabytes:1536,wanted:4});
  expect(lease.workers,"8 GB, parent "+resident).toBe(1);
  expect(farmReservationsV1()).toEqual([{kind:"project-farm",megabytes:1700}]);lease.release();
 }
 const sixteen=reserveFarmV1({...input(16384,1500),kind:"project-farm",workerMegabytes:1536,wanted:8});
 expect(sixteen.workers).toBe(Math.floor((16384*share-1500)/1700));sixteen.release();
 const none=reserveFarmV1({...input(8192,4800),kind:"project-farm",workerMegabytes:1536,wanted:2});
 expect(none.workers,"parent plus one child past the ceiling: no child, the caller says so").toBe(0);
 const serve=reserveFarmV1({...input(65536,1500),kind:"project-farm",workerMegabytes:1536,wanted:5});expect(serve.workers).toBe(5);
 const off=reserveFarmV1({hostMegabytes:8192,residentMegabytes:900,environment:env({SEMIO_SLICE_LEAN:"off"}),kind:"project-farm",workerMegabytes:1536,wanted:4});
 expect(off.workers,"lean off keeps the old law: the parent floor of 3600 leaves nothing").toBe(0);
 expect(leanSliceV1({hostMegabytes:8192,environment:env()}).lean).toBe(true);expect(leanSliceV1({hostMegabytes:65536,environment:env()}).lean).toBe(false);
 expect(farmReservationsV1()).toEqual([]);
});

test("the walk farm gives its children back to the first project child; random law: the tree of parent and children stays within the ceiling at every grant",()=>{
 let yielded=0;
 const walk=reserveFarmV1({hostMegabytes:8192,residentMegabytes:1500,environment:env(),kind:"walk-farm",workerMegabytes:768,wanted:2,yield:()=>{yielded++;}});
 expect(walk.workers).toBe(2);
 const project=reserveFarmV1({hostMegabytes:8192,residentMegabytes:1500,environment:env(),kind:"project-farm",workerMegabytes:1536,wanted:2});
 expect(project.workers).toBe(1);expect(yielded).toBe(1);project.release();walk.release();
 let seed=65;const next=(n:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%n;};
 for(const host of[8192,12288,16384]){
  const open:{release():void}[]=[];
  for(let step=0;step<300;step++){
   if(open.length&&next(3)===0){open.splice(next(open.length),1)[0].release();continue;}
   const resident=700+next(2600),kind=next(2)?"project-farm":"walk-farm",lease=reserveFarmV1({hostMegabytes:host,residentMegabytes:resident,environment:env(),kind,workerMegabytes:kind==="project-farm"?1536:768,wanted:1+next(4)});
   open.push(lease);
   const held=farmReservationsV1().reduce((sum,row)=>sum+row.megabytes,0);
   if(lease.workers>0)expect(held+Math.max(resident,1500)).toBeLessThanOrEqual(Math.max(host*share,Math.max(resident,1500)+1700)+1700);
   if(lease.workers>0&&kind==="project-farm"&&lease.workers>1)expect(held+Math.max(resident,1500)).toBeLessThanOrEqual(host*share+1700);
  }
  for(const lease of open)lease.release();
  expect(farmReservationsV1()).toEqual([]);
 }
});

test("real processes under a declared small machine: owners inferred in children that exit after a batch keep the summed tree under the plan limit and the parent does not grow per owner; owners inferred in the parent do",async()=>{
 const plan=leanSlicePlanV1({limitMegabytes:3276,cores:2,environment:env()}),owners=12,perOwner=60,ballast=(count:number)=>`const b=[];for(let i=0;i<${count};i++){const x=Buffer.alloc(${perOwner}*1048576);for(let k=0;k<x.length;k+=4096)x[k]=1;b.push(x);}setTimeout(()=>process.exit(0),700);`;
 const sample=async(work:()=>Promise<void>)=>{let peak=0,stop=false;const loop=(async()=>{while(!stop){peak=Math.max(peak,await observeHostTreeMegabytesV1(process.pid));await new Promise(done=>setTimeout(done,100));}})();await work();stop=true;await loop;return peak;};
 const before=process.memoryUsage().rss/MB;
 const treePeak=await sample(async()=>{for(let at=0;at<owners;at+=plan.ownersPerChild){const child=spawn(process.execPath,["-e",ballast(Math.min(plan.ownersPerChild,owners-at))],{stdio:"ignore"});await new Promise(done=>child.on("exit",done));}});
 const lean=process.memoryUsage().rss/MB-before;
 const held:Buffer[]=[];
 await sample(async()=>{for(let at=0;at<owners;at++){const x=Buffer.alloc(perOwner*MB);for(let k=0;k<x.length;k+=4096)x[k]=1;held.push(x);}});
 const inProcess=process.memoryUsage().rss/MB-before;held.length=0;
 expect(treePeak,"summed tree RSS of the test process and the children").toBeLessThanOrEqual(3276);
 expect(lean,"parent growth over "+owners+" owners inferred by children").toBeLessThan(80);
 expect(inProcess,"control: the same owners inferred in the parent").toBeGreaterThan(owners*perOwner*0.8);
},120000);
