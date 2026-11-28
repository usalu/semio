import definition from "./🧬️schema/🔣️.json" with {type:"json"};
import budgetTable from "./🎯️budgets/🔣️.json" with {type:"json"};

export type LatencyGroupV1="door"|"graph"|"source"|"tools"|"leases"|"children"|"other"|"uncovered";
export type LatencyTripV1="graphService"|"cacheLookup"|"spawn";
export type LatencyStatusV1="ok"|"warn"|"breach"|"unmeasured";
export type LatencyTextV1=Readonly<{en:string;de:string}>;
export type LatencyCommandV1=Readonly<{id:string;args:readonly string[];title:LatencyTextV1;wallMs:number;phases:Readonly<Partial<Record<LatencyGroupV1,number>>>;trips:Readonly<Partial<Record<LatencyTripV1,number>>>}>;
export type LatencyBudgetsV1=Readonly<{version:1;reference:LatencyTextV1;targets:Readonly<{project:string;file:string}>;commands:readonly LatencyCommandV1[]}>;
export type LatencyProfileRowV1=Readonly<{phase:string;self:number}>;
export type LatencyProfileDocumentV1=Readonly<{pid?:number;total?:number;rows:readonly LatencyProfileRowV1[]}>;
export type LatencyObservationV1=Readonly<{wallMs:number;processes:number;phases:Readonly<Record<LatencyGroupV1,number>>;trips:Readonly<Record<LatencyTripV1,number|null>>}>;
export type LatencySampleV1=Readonly<{run:number;exit:number|null;observation:LatencyObservationV1}>;
export type LatencyRowV1=Readonly<{metric:string;value:number|null;budget:number;ratio:number|null;status:LatencyStatusV1}>;
export type LatencyVerdictV1=Readonly<{status:LatencyStatusV1;rows:readonly LatencyRowV1[]}>;
export type LatencyResultV1=Readonly<{command:string;status:"ran"|"skipped"|"failed";reason?:string;samples:readonly LatencySampleV1[];verdict:LatencyVerdictV1|null}>;
export type LatencyReportV1=Readonly<{version:1;started:string;host:Readonly<{cpus:number;memoryMb:number;loadPerCore:number|null;speedFactor:number}>;results:readonly LatencyResultV1[]}>;

/** ⏲️ The schema of the latency budgets, the policy numbers and the group and round trip tables; the same document drives this module, the validators of every language and the oracle.
 * @see ./🧬️schema/🔣️.json */
export const latencySchemaV1=definition;
/** 📜️ The declared policy: tolerance, breach ratio, the profile switch the runs set and the options of `repo bench latency`. */
export const LATENCY_POLICY_V1=definition.definitions.Policy.const;
/** 🗂️ The declared phase groups with the profile phase prefixes each one owns. */
export const LATENCY_GROUPS_V1=definition.definitions.Groups.const as readonly Readonly<{id:LatencyGroupV1;prefixes:readonly string[];title:LatencyTextV1}>[];
/** 🔁️ The declared round trip kinds with the profile marker each one is counted from. */
export const LATENCY_TRIPS_V1=definition.definitions.Trips.const as Readonly<Record<LatencyTripV1,Readonly<{marker:string;measured:boolean;title:LatencyTextV1}>>>;
/** 🎯️ The declared budgets of the developer commands per phase group and round trip kind.
 * @see ./🎯️budgets/🔣️.json */
export const latencyBudgetsV1:LatencyBudgetsV1=budgetTable as unknown as LatencyBudgetsV1;

const both=(english:string,german:string)=>`${english}\n${german}`,groupIds=LATENCY_GROUPS_V1.map(row=>row.id),tripIds=Object.keys(LATENCY_TRIPS_V1) as LatencyTripV1[],round=(value:number)=>Math.round(value*10)/10;

/** 🧭️ The group a profile phase belongs to: the first declared group owning a prefix of the name, `other` when none does; a round trip marker belongs to no group (it takes no time). */
export function latencyGroupOfV1(phase:string):LatencyGroupV1|null{
 if(phase.startsWith(LATENCY_POLICY_V1.tripPrefix))return null;
 for(const group of LATENCY_GROUPS_V1)if(group.prefixes.some(prefix=>phase.startsWith(prefix)))return group.id;
 return "other";
}

/** 📖️ Reads every profile document (`SEMIO_REPO_PROFILE=json` prints one JSON line per process) out of a process's error output; other lines are ignored. */
export function latencyParseProfileV1(text:string):readonly LatencyProfileDocumentV1[]{
 const found:LatencyProfileDocumentV1[]=[];
 for(const line of text.split("\n")){
  const trimmed=line.trim();if(!trimmed.startsWith("{")||!trimmed.endsWith("}"))continue;
  try{const value=JSON.parse(trimmed) as {version?:unknown;rows?:unknown};if(value.version===1&&Array.isArray(value.rows))found.push(value as unknown as LatencyProfileDocumentV1);}catch{}
 }
 return found;
}

/** ⏱️ Reduces the profile documents of one command to time per phase group (the self time of every row, so nested phases are never counted twice), the time no phase covers (outer wall minus the groups, never below zero) and the round trips counted from the markers; a kind nobody counts yet is `null`, not zero. */
export function latencyObserveV1(documents:readonly LatencyProfileDocumentV1[],wallMs:number):LatencyObservationV1{
 const phases=Object.fromEntries(groupIds.map(id=>[id,0])) as Record<LatencyGroupV1,number>,counts=Object.fromEntries(tripIds.map(id=>[id,0])) as Record<LatencyTripV1,number>;
 for(const document of documents)for(const row of document.rows){
  const group=latencyGroupOfV1(row.phase);
  if(group!==null)phases[group]+=Number(row.self)||0;
  for(const id of tripIds){const marker=LATENCY_TRIPS_V1[id].marker;if(marker.endsWith(".")?row.phase.startsWith(marker):row.phase===marker)counts[id]++;}
 }
 const covered=groupIds.filter(id=>id!=="uncovered").reduce((sum,id)=>sum+phases[id],0);
 phases.uncovered=Math.max(0,wallMs-covered);
 for(const id of groupIds)phases[id]=round(phases[id]);
 const trips=Object.fromEntries(tripIds.map(id=>[id,LATENCY_TRIPS_V1[id].measured?counts[id]:null])) as Record<LatencyTripV1,number|null>;
 return Object.freeze({wallMs:round(wallMs),processes:documents.length,phases:Object.freeze(phases),trips:Object.freeze(trips)});
}

/** 📈️ The median of the numbers, null when there are none. */
export function latencyMedianV1(values:readonly (number|null)[]):number|null{
 const sorted=values.filter((value):value is number=>value!==null).sort((a,b)=>a-b);
 if(!sorted.length)return null;
 const mid=sorted.length>>1;
 return sorted.length%2?sorted[mid]!:(sorted[mid-1]!+sorted[mid]!)/2;
}

/** 🧮️ The median observation of several runs, metric by metric. */
export function latencyMedianObservationV1(observations:readonly LatencyObservationV1[]):LatencyObservationV1{
 const pick=(read:(row:LatencyObservationV1)=>number|null)=>latencyMedianV1(observations.map(read));
 return Object.freeze({wallMs:pick(row=>row.wallMs)??0,processes:pick(row=>row.processes)??0,phases:Object.freeze(Object.fromEntries(groupIds.map(id=>[id,pick(row=>row.phases[id])??0])) as Record<LatencyGroupV1,number>),trips:Object.freeze(Object.fromEntries(tripIds.map(id=>[id,pick(row=>row.trips[id])])) as Record<LatencyTripV1,number|null>)});
}

const worst=(rows:readonly LatencyRowV1[]):LatencyStatusV1=>rows.some(row=>row.status==="breach")?"breach":rows.some(row=>row.status==="warn")?"warn":rows.some(row=>row.status==="ok")?"ok":"unmeasured";

/** ⚖️ Judges one observation against the budget of a command on a machine with the given speed factor: a time limit is lengthened by the factor and takes the tolerance on top, a count limit is exact; `ok` up to the limit, `warn` up to the breach ratio times the limit, `breach` beyond, `unmeasured` for a round trip kind nobody counts yet. */
export function latencyJudgeV1(observation:LatencyObservationV1,command:LatencyCommandV1,factor:number):LatencyVerdictV1{
 const scale=Math.max(1,factor),tolerance=LATENCY_POLICY_V1.toleranceMilliseconds,ratio=LATENCY_POLICY_V1.breachRatio;
 const time=(metric:string,value:number,budget:number):LatencyRowV1=>{
  const limit=Math.ceil(budget*scale),status:LatencyStatusV1=value<=limit+tolerance?"ok":value<=limit*ratio+tolerance?"warn":"breach";
  return Object.freeze({metric,value,budget:limit,ratio:limit===0?null:Math.round(value/limit*100)/100,status});
 };
 const count=(metric:string,value:number|null,budget:number):LatencyRowV1=>{
  const status:LatencyStatusV1=value===null?"unmeasured":value<=budget?"ok":value<=Math.max(budget+1,Math.ceil(budget*ratio))?"warn":"breach";
  return Object.freeze({metric,value,budget,ratio:value===null||budget===0?null:Math.round(value/budget*100)/100,status});
 };
 const rows:LatencyRowV1[]=[time("wall",observation.wallMs,command.wallMs)];
 for(const id of groupIds)if(command.phases[id]!==undefined)rows.push(time("phase:"+id,observation.phases[id],command.phases[id]!));
 for(const id of tripIds)if(command.trips[id]!==undefined)rows.push(count("trips:"+id,observation.trips[id],command.trips[id]!));
 return Object.freeze({status:worst(rows),rows:Object.freeze(rows)});
}

/** 🔎️ Names every inconsistency of a budget table; empty when it is closed: unique command ids, known placeholders and groups, no phase budget above the wall budget. */
export function latencyBudgetIssuesV1(set:LatencyBudgetsV1=latencyBudgetsV1):readonly string[]{
 const found:string[]=[],ids=new Set<string>();
 for(const command of set.commands){
  if(ids.has(command.id))found.push(`${command.id}: repeated id`);ids.add(command.id);
  for(const part of command.args)for(const hit of part.matchAll(/\{([a-z]+)\}/gu))if(!Object.hasOwn(set.targets,hit[1]!))found.push(`${command.id}: unknown placeholder {${hit[1]}}`);
  for(const [group,budget] of Object.entries(command.phases)){if(!groupIds.includes(group as LatencyGroupV1))found.push(`${command.id}: unknown group ${group}`);else if(budget!>command.wallMs)found.push(`${command.id}: ${group} budget exceeds the wall budget`);}
  for(const trip of Object.keys(command.trips))if(!tripIds.includes(trip as LatencyTripV1))found.push(`${command.id}: unknown round trip ${trip}`);
 }
 return found;
}

/** 🔤️ The arguments of a command with `{project}` and `{file}` replaced by the fixed small targets of the table. */
export function latencyArgumentsV1(command:LatencyCommandV1,targets:LatencyBudgetsV1["targets"]=latencyBudgetsV1.targets):readonly string[]{
 return command.args.map(part=>part.replace(/\{([a-z]+)\}/gu,(_,name:string)=>{const value=(targets as Record<string,string>)[name];if(value===undefined)throw Error(both(`Unknown placeholder {${name}}`,`Unbekannter Platzhalter {${name}}`));return value;}));
}

/** ⚖️ Judges the runs of one command: the median observation of the runs that exited 0 against the budget; no such run leaves no verdict. */
export function latencyResultV1(command:LatencyCommandV1,samples:readonly LatencySampleV1[],factor:number,state:Readonly<{status?:"ran"|"skipped"|"failed";reason?:string}>={}):LatencyResultV1{
 const good=samples.filter(sample=>sample.exit===0),status=state.status??(good.length?"ran":"failed");
 return Object.freeze({command:command.id,status,...(state.reason===undefined?{}:{reason:state.reason}),samples,verdict:good.length?latencyJudgeV1(latencyMedianObservationV1(good.map(sample=>sample.observation)),command,factor):null});
}

const labels={command:{en:"Command",de:"Befehl"},metric:{en:"Metric",de:"Messgröße"},value:{en:"Median",de:"Median"},budget:{en:"Budget",de:"Budget"},ratio:{en:"Ratio",de:"Verhältnis"},status:{en:"Status",de:"Status"},unmeasured:{en:"unmeasured",de:"nicht gemessen"},ok:{en:"ok",de:"ok"},warn:{en:"WARN over budget",de:"WARNUNG über Budget"},breach:{en:"BREACH over twice the budget",de:"VERLETZT über dem Doppelten des Budgets"},wall:{en:"wall ms",de:"Laufzeit ms"},phase:{en:"phase ms",de:"Phase ms"},trips:{en:"round trips",de:"Rundreisen"},failed:{en:"failed",de:"fehlgeschlagen"},skipped:{en:"skipped",de:"übersprungen"},speed:{en:"Speed factor",de:"Geschwindigkeitsfaktor"}} as const;

/** 📋️ Renders a report as one markdown table per command in the language asked: the median of every metric, its (scaled) budget, the ratio and the status; a command that did not run says why. */
export function latencyRenderV1(report:LatencyReportV1,language:"en"|"de",set:LatencyBudgetsV1=latencyBudgetsV1):string{
 const t=(key:keyof typeof labels)=>labels[key][language],cell=(value:number|null)=>value===null?"-":String(Math.round(value*10)/10),out:string[]=[];
 out.push(`${t("speed")}: ${report.host.speedFactor} (${report.host.cpus} CPU, ${report.host.memoryMb} MB)`,"");
 for(const result of report.results){
  const command=set.commands.find(row=>row.id===result.command),title=command?.title[language]??result.command;
  out.push(`### ${title}`);
  if(!result.verdict){out.push(`${t(result.status==="skipped"?"skipped":"failed")}${result.reason?`: ${result.reason}`:""}`,"");continue;}
  out.push(`| ${t("metric")} | ${t("value")} | ${t("budget")} | ${t("ratio")} | ${t("status")} |`,"| --- | --- | --- | --- | --- |");
  for(const row of result.verdict.rows){
   const [kind,name]=row.metric.split(":"),label=kind==="wall"?t("wall"):kind==="phase"?`${t("phase")} ${LATENCY_GROUPS_V1.find(group=>group.id===name)?.title[language]??name}`:`${t("trips")} ${LATENCY_TRIPS_V1[name as LatencyTripV1]?.title[language]??name}`;
   out.push(`| ${label} | ${cell(row.value)} | ${row.budget} | ${row.ratio===null?"-":row.ratio} | ${t(row.status)} |`);
  }
  out.push("");
 }
 return out.join("\n");
}

/** 🚨️ Every `<command> <metric>` over its budget (`warn` and `breach`); with `breachesOnly` only those beyond the breach ratio. Empty when nothing is over budget. */
export function latencyExceededV1(report:LatencyReportV1,breachesOnly=false):readonly string[]{
 return report.results.flatMap(result=>(result.verdict?.rows??[]).filter(row=>row.status==="breach"||!breachesOnly&&row.status==="warn").map(row=>`${result.command} ${row.metric}`));
}
