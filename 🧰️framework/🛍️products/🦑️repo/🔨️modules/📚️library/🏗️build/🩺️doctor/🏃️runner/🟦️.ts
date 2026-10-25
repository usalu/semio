import {fileURLToPath} from "node:url";
import {DOCTOR_POLICY_V1,DOCTOR_PROTOCOL_V1,DOCTOR_RULES_V1,type DoctorAreaV1,type DoctorFactsV1,type DoctorPolicyV1,type DoctorProbeRowV1,type DoctorReportV1,type DoctorRepairResultV1,type DoctorRuleV1} from "../🟦️.ts";
import {analyzeDoctorV1,buildDoctorReportV1} from "../🧮️analysis/🟦️.ts";
import type {DoctorPortsV1} from "../🔌️ports/🟦️.ts";
import {DOCTOR_PROBES_V1,DOCTOR_REPAIRS_V1} from "../🔎️probes/🟦️.ts";
import type {DoctorProbeContextV1,DoctorProbeV1,DoctorRepairV1,DoctorSectionsV1} from "../🔎️probes/📋️contract/🟦️.ts";

/** 🎛️ What one doctor run is asked to do. */
export type DoctorRunOptionsV1=Readonly<{
 only?:ReadonlySet<DoctorAreaV1>;fix?:boolean;dryRun?:boolean;
 policy?:DoctorPolicyV1;rules?:Readonly<Record<string,DoctorRuleV1>>;probes?:readonly DoctorProbeV1[];repairs?:ReadonlyMap<string,DoctorRepairV1>;libraryRoot?:string;
}>;
/** 📦️ The facts a run read and the report derived from them. */
export type DoctorRunV1=Readonly<{facts:DoctorFactsV1;report:DoctorReportV1}>;

const SUPPORT:Readonly<Record<string,readonly DoctorAreaV1[]>>={processes:["leases","capacity"],host:["capacity"]};
const LIBRARY_ROOT=fileURLToPath(new URL("../../../",import.meta.url));
const firstLine=(error:unknown):string=>(error instanceof Error?error.message:String(error)).split("\n")[0]!.slice(0,300);

/** ✋️ The error of a doctor run that was cancelled; nothing it read is reported. */
export class DoctorCancelledV1 extends Error{constructor(){super("doctor cancelled");this.name="DoctorCancelledV1";}}

/** 🔎️ Runs one probe with its own time budget: a probe that overruns is cut off and reported, a probe that throws is reported and never stops the others. */
async function runProbe(probe:DoctorProbeV1,ports:DoctorPortsV1,context:DoctorProbeContextV1,policy:DoctorPolicyV1):Promise<Readonly<{row:DoctorProbeRowV1;sections?:DoctorSectionsV1}>>{
 const budget=policy.probe.timeoutOverrides[probe.area]??policy.probe.timeoutMilliseconds,started=ports.now(),controller=new AbortController(),forward=():void=>controller.abort(ports.signal.reason);
 if(ports.signal.aborted)throw new DoctorCancelledV1();
 ports.signal.addEventListener("abort",forward,{once:true});
 let timer:ReturnType<typeof setTimeout>|undefined;
 try{
  const timeout=new Promise<"timeout">(done=>{timer=setTimeout(()=>done("timeout"),budget);});
  const cancelled=new Promise<never>((_,fail)=>{if(ports.signal.aborted)fail(new DoctorCancelledV1());ports.signal.addEventListener("abort",()=>fail(new DoctorCancelledV1()),{once:true});});
  const outcome=await Promise.race([probe.read({...ports,signal:controller.signal},{...context,deadlineMs:started+Math.floor(budget*0.8)}),timeout,cancelled]);
  const durationMs=Math.max(0,ports.now()-started);
  if(outcome==="timeout"){controller.abort();return {row:{area:probe.area,status:"timeout",durationMs}};}
  if("unavailable" in outcome)return {row:{area:probe.area,status:"unavailable",durationMs,reason:outcome.unavailable.slice(0,300)}};
  return {row:{area:probe.area,status:"ok",durationMs},sections:outcome.sections};
 }catch(error){
  if(ports.signal.aborted)throw new DoctorCancelledV1();
  return {row:{area:probe.area,status:"failed",durationMs:Math.max(0,ports.now()-started),reason:firstLine(error)}};
 }finally{
  if(timer!==undefined)clearTimeout(timer);
  ports.signal.removeEventListener("abort",forward);
 }
}

/** 🧭️ Picks the probes a request needs: the named areas plus the probes whose facts they depend on. */
export function selectDoctorProbesV1(probes:readonly DoctorProbeV1[],only:ReadonlySet<DoctorAreaV1>|undefined):readonly DoctorProbeV1[]{
 if(only===undefined)return probes;
 const wanted=new Set<DoctorAreaV1>();
 for(const area of only){wanted.add(area);for(const support of SUPPORT[area]??[])wanted.add(support);}
 return probes.filter(probe=>wanted.has(probe.area));
}

/** 📸️ Reads the facts of a machine: phase 1 probes run with bounded parallelism, then the process probe sees the pids they found. */
export async function collectDoctorFactsV1(ports:DoctorPortsV1,options:DoctorRunOptionsV1={}):Promise<DoctorFactsV1>{
 const policy=options.policy??DOCTOR_POLICY_V1,probes=selectDoctorProbesV1(options.probes??DOCTOR_PROBES_V1,options.only),sections:Record<string,unknown>={},rows:DoctorProbeRowV1[]=[];
 const base:DoctorProbeContextV1={policy,known:new Set(),libraryRoot:options.libraryRoot??LIBRARY_ROOT,deadlineMs:0};
 let started=0;
 const phase=async(group:readonly DoctorProbeV1[],context:DoctorProbeContextV1):Promise<void>=>{
  let next=0;
  const total=probes.length,worker=async():Promise<void>=>{
   for(;;){
    const probe=group[next++];if(probe===undefined)return;
    ports.progress(`probe:${probe.area}:${started++}/${total}`);
    const result=await runProbe(probe,ports,context,policy);
    rows.push(result.row);
    if(result.sections)Object.assign(sections,result.sections);
   }
  };
  await Promise.all(Array.from({length:Math.min(policy.probe.parallelism,group.length)},worker));
 };
 await phase(probes.filter(probe=>probe.phase===1),base);
 const known=new Set<number>();
 const leases=sections.leases as DoctorFactsV1["leases"],capacity=sections.capacity as DoctorFactsV1["capacity"];
 for(const row of leases?.holders??[])known.add(row.pid);
 for(const row of leases?.waiters??[]){known.add(row.pid);for(const pid of row.blockers)known.add(pid);}
 for(const row of capacity?.grants??[])known.add(row.pid);
 for(const row of capacity?.waiters??[])known.add(row.pid);
 await phase(probes.filter(probe=>probe.phase===2),{...base,known});
 const order=new Map<DoctorAreaV1,number>(DOCTOR_PROTOCOL_V1.areas.map((area,index)=>[area,index]));
 rows.sort((left,right)=>order.get(left.area)!-order.get(right.area)!);
 return Object.freeze({version:1,atMs:ports.now(),platform:ports.platform,probes:rows,...sections});
}

/** 🩺️ Reads the machine, evaluates it and, when asked, applies the safe repairs the findings point at and reads again; a dry run reports what the repairs would do. */
export async function runDoctorV1(ports:DoctorPortsV1,options:DoctorRunOptionsV1={}):Promise<DoctorRunV1>{
 const analyze=(facts:DoctorFactsV1)=>{
  const analysis=analyzeDoctorV1(facts,{...options.policy?{policy:options.policy}:{},...options.rules?{rules:options.rules}:{}}),only=options.only;
  return only===undefined?analysis:{findings:analysis.findings.filter(row=>only.has(row.area)||row.area==="doctor"),waits:analysis.waits.filter(row=>only.has(row.kind==="lease"?"leases":"capacity")),processes:only.has("processes")?analysis.processes:[]};
 };
 let facts=await collectDoctorFactsV1(ports,options),analysis=analyze(facts);
 const results:DoctorRepairResultV1[]=[];
 if(options.fix){
  const dryRun=options.dryRun??false,registry=options.repairs??DOCTOR_REPAIRS_V1,policy=options.policy??DOCTOR_POLICY_V1;
  const ids=[...new Set(analysis.findings.filter(row=>row.fix.kind==="automatic"&&row.fix.repair!==undefined).map(row=>row.fix.repair!))];
  const context:DoctorProbeContextV1={policy,known:new Set(),libraryRoot:options.libraryRoot??LIBRARY_ROOT,deadlineMs:0};
  for(const id of ids){
   const repair=registry.get(id);if(!repair)continue;
   ports.progress(`repair:${id}`);
   try{
    const done=await repair.run(ports,context,dryRun);
    results.push({id,area:repair.area,dryRun,status:done.status??"done",removed:done.removed,bytes:done.bytes,...done.reason===undefined?{}:{reason:done.reason}});
   }catch(error){
    if(ports.signal.aborted){results.push({id,area:repair.area,dryRun,status:"cancelled",removed:0,bytes:0});break;}
    results.push({id,area:repair.area,dryRun,status:"failed",removed:0,bytes:0,reason:firstLine(error)});
   }
  }
  if(!dryRun&&results.some(row=>row.status==="done")){facts=await collectDoctorFactsV1(ports,options);analysis=analyze(facts);}
 }
 return {facts,report:buildDoctorReportV1(facts,analysis,results)};
}
