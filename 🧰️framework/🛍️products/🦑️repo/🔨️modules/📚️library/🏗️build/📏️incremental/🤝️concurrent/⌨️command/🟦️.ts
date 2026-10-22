import {buildBackendAlternationV1,buildBackendIdsV1} from "../../../🔌️backends/🟦️.ts";
import {rm} from "node:fs/promises";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import workloads from "../🎬️workloads/🔣️.json" with {type:"json"};
import type {IncrementalHostV1} from "../../🏃️runner/🟦️.ts";
import type {IncrementalToolV1} from "../../🟦️.ts";
import {incrementalCommandPortsV1,incrementalDefaultRootV1,incrementalLanguageV1,type IncrementalCommandPortsV1} from "../../⌨️command/🟦️.ts";
import {concurrentEvaluateV1,concurrentReportV1,concurrentSelectV1,concurrentWorkloadIssuesV1,renderConcurrentReportV1,renderConcurrentWorkloadsV1,type ConcurrentRunV1,type ConcurrentWorkloadSetV1,type ConcurrentWorkloadV1} from "../🟦️.ts";
import {concurrentCliPortsV1,concurrentPlanV1,concurrentRunWorkloadV1,type ConcurrentPlanV1} from "../🏃️runner/🟦️.ts";

const policy=schema.definitions.Policy.const,tools:readonly IncrementalToolV1[]=buildBackendIdsV1(),operations=["list","plan","run"] as const,script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📜️script.ts";

/** 🗨️ The closed arguments of `repo concurrency`. */
export type ConcurrencyArgumentsV1=Readonly<{operation:typeof operations[number];workloads:readonly string[]|"all";sessions:number|undefined;criteria:string|undefined;tools:readonly IncrementalToolV1[];root:string|undefined;file:string|undefined;json:boolean;language:"en"|"de"|undefined}>;

const both=(english:string,german:string)=>`${english}\n${german}`,valuedOptions=Object.freeze(["--workloads","--sessions","--criteria","--implementations","--root","--file","--lang"]),plainOptions=Object.freeze(["--json"]);

/** 🏳️ The option names `repo concurrency` accepts, so the operations catalogue can be checked against the parser. */
export function concurrencyOptionNamesV1():{valued:readonly string[];plain:readonly string[]}{return {valued:valuedOptions,plain:plainOptions};}

/** 🪝️ Reads `concurrency list|plan|run` with its options; every unknown or repeated option is refused in both languages. */
export function concurrencyArgumentsV1(args:readonly string[]):ConcurrencyArgumentsV1{
 const operation=args[0] as typeof operations[number];
 if(!operations.includes(operation)||args.length>1&&args[1]!.startsWith("-")===false)throw Error(both("Expected repo concurrency list|plan|run [options]","Erwartet: repo concurrency list|plan|run [Optionen]"));
 const seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),valued=new Set(valuedOptions),plain=new Set(plainOptions);
 for(let index=1;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both(`Repeated option: ${name}`,`Wiederholte Option: ${name}`));
  seen.add(name);
  if(valued.has(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both(`Option ${name} needs a value`,`Option ${name} braucht einen Wert`));values[name]=value;}
  else if(plain.has(name)&&cut<0)flags.add(name);
  else throw Error(both(`Unknown concurrency option: ${argument}`,`Unbekannte Option für concurrency: ${argument}`));
 }
 const list=(name:string)=>values[name]?.split(",").filter(Boolean),chosen=list("--implementations")??[...tools],language=values["--lang"],picked=list("--workloads");
 for(const tool of chosen)if(!tools.includes(tool as IncrementalToolV1))throw Error(both(`Unknown implementation: ${tool} (${buildBackendAlternationV1()})`,`Unbekannte Implementierung: ${tool} (${buildBackendAlternationV1()})`));
 if(language!==undefined&&language!=="en"&&language!=="de")throw Error(both(`Unknown language: ${language} (en|de)`,`Unbekannte Sprache: ${language} (en|de)`));
 const count=values["--sessions"]===undefined?undefined:Number(values["--sessions"]);
 if(count!==undefined&&(!/^[1-9][0-9]*$/u.test(values["--sessions"]!)||count>policy.maximumSessions))throw Error(both(`Sessions must be a whole number from 1 to ${policy.maximumSessions}`,`Sitzungen müssen eine ganze Zahl von 1 bis ${policy.maximumSessions} sein`));
 return Object.freeze({operation,workloads:picked===undefined||picked.includes("all")?"all":picked,sessions:count,criteria:values["--criteria"],tools:[...new Set(chosen)] as IncrementalToolV1[],root:values["--root"],file:values["--file"],json:flags.has("--json"),language:language as "en"|"de"|undefined});
}

const languageOf=(parsed:ConcurrencyArgumentsV1,ports:IncrementalCommandPortsV1):"en"|"de"=>incrementalLanguageV1(parsed.language,ports.environment);

/** 🗾️ Renders the plan of workloads: per workload and tool the sessions with their groups, the closure of every target, the tasks that always execute and every problem found. */
export function renderConcurrentPlanV1(set:ConcurrentWorkloadSetV1,rows:readonly Readonly<{workload:ConcurrentWorkloadV1;tool:IncrementalToolV1;plan:ConcurrentPlanV1}>[],language:"en"|"de"):string{
 const t=(key:string)=>set.labels[key]![language],out:string[]=[];
 for(const row of rows){
  out.push(`${row.workload.id} / ${row.tool}: ${t("sessions")} ${row.workload.sessions.map(session=>`${session.id}[${session.group}]`).join(" ")}`);
  for(const target of row.plan.targets)out.push(`  ${t("target")} ${target.target}: ${target.tasks} tasks, ${t("uncached")} ${target.uncached.length}`);
  out.push(`  ${row.plan.issues.length||row.plan.missing.length?t("refused"):t("independent")}`);
  for(const file of row.plan.missing)out.push(`    missing ${file}`);
  for(const issue of row.plan.issues)out.push(`    ${issue}`);
 }
 return out.join("\n");
}

/** 🛫️ Runs `repo concurrency`: lists the workloads, plans them in the prepared checkout copy, or runs them with every implementation, measures waits per lock class, cache hits and correctness against a serial reference and judges the declared pass criteria. Progress is published per stage and every stage checks for cancellation. */
export async function runConcurrencyCommandV1(live:string,args:readonly string[],ports:IncrementalCommandPortsV1):Promise<void>{
 const parsed=concurrencyArgumentsV1(args.slice(args[0]==="concurrency"?1:0)),set=workloads as unknown as ConcurrentWorkloadSetV1,issues=concurrentWorkloadIssuesV1(set);
 if(issues.length)throw Error(`Closed concurrent workloads required: ${issues.join("; ")}`);
 const lang=languageOf(parsed,ports),root=parsed.root??incrementalDefaultRootV1(ports.platform,ports.tmpdir(),ports.home());
 if(parsed.operation==="list"){ports.write(parsed.json?JSON.stringify(set.workloads.map(row=>({id:row.id,criteria:row.criteria,sessions:row.sessions.map(session=>({id:session.id,group:session.group}))})),null,2):renderConcurrentWorkloadsV1(set,lang));return;}
 if(!await ports.exists(join(root,script)))throw Error(both(`Prepare the disposable root first: repo incremental prepare --root=${root}`,`Bereite zuerst das Wegwerf-Verzeichnis vor: repo incremental prepare --root=${root}`));
 const selected=concurrentSelectV1(set,parsed.workloads,parsed.sessions),override=parsed.criteria;
 if(override!==undefined&&!set.criteria.some(row=>row.id===override))throw Error(both(`Unknown criteria: ${override} (${set.criteria.map(row=>row.id).join("|")})`,`Unbekannte Kriterien: ${override} (${set.criteria.map(row=>row.id).join("|")})`));
 const scratch=await ports.scratch(),files=await ports.files(live,ports.signal);
 const host:IncrementalHostV1=({live,root,script,executable:process.execPath,scratch,signal:ports.signal,environment:ports.environment,run:ports.run,nxRecord:ports.nxRecord,adapter:ports.adapter,inventory:async()=>files,now:ports.now,check:ports.check,progress:ports.progress});
 try{
  const real=concurrentCliPortsV1(host,ports.signal),pairs=selected.flatMap(workload=>parsed.tools.filter(tool=>!workload.tools||workload.tools.includes(tool)).map(tool=>({workload,tool})));
  if(parsed.operation==="plan"){
   const rows=[];
   for(const{workload,tool}of pairs){ports.check();await ports.progress(`${workload.id}:${tool}:plan`);rows.push({workload,tool,plan:await concurrentPlanV1(workload,tool,real)});}
   ports.write(parsed.json?JSON.stringify(rows.map(row=>({workload:row.workload.id,tool:row.tool,targets:row.plan.targets,uncached:row.plan.uncached,missing:row.plan.missing,issues:row.plan.issues})),null,2):renderConcurrentPlanV1(set,rows,lang));
   return;
  }
  const runs:ConcurrentRunV1[]=[];
  for(const{workload,tool}of pairs){
   const criteriaId=override??workload.criteria,criteria=set.criteria.find(row=>row.id===criteriaId)!,measurement=await concurrentRunWorkloadV1(set,workload,tool,real);
   runs.push(Object.freeze({workload:workload.id,tool,criteria:criteriaId,measurement,rules:concurrentEvaluateV1(criteria,measurement)}));
  }
  const report=concurrentReportV1(runs);
  if(parsed.file)await ports.report(parsed.file,report);
  ports.write(parsed.json?JSON.stringify(report,null,2):renderConcurrentReportV1(report,set,lang));
  if(report.failed)throw Error(both(`Concurrency conformance failed in ${report.failed} of ${report.passed+report.failed+report.skipped} criteria`,`Konformität der Gleichzeitigkeit verletzt in ${report.failed} von ${report.passed+report.failed+report.skipped} Kriterien`));
 }finally{await rm(scratch,{recursive:true,force:true});}
}

/** 🧱️ Binds the command to the running build CLI: the incremental harness's ports with progress published under `concurrency:`. */
export async function concurrencyCommandPortsV1(root:string,invocation:import("../../../../../../../..//🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<IncrementalCommandPortsV1>{
 const base=await incrementalCommandPortsV1(root,invocation),routing=await import("../../../../../../../..//🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 return Object.freeze({...base,progress:stage=>routing.advanceScriptInvocation(invocation,"concurrency:"+stage,"running")});
}

/** 🔔️ The entry the build CLI calls for `repo concurrency`. */
export async function runConcurrencyV1(root:string,args:readonly string[],invocation:import("../../../../../../../..//🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void>{
 await runConcurrencyCommandV1(root,args,await concurrencyCommandPortsV1(root,invocation));
}
