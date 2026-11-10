import definition from "./🧬️schema/🔣️.json" with {type:"json"};
import scenarioSet from "./🎯️scenarios/🔣️.json" with {type:"json"};

export type PlanningPhaseV1="facts"|"verification"|"inference"|"planning";
export type PlanningMarkerV1=Readonly<{phase:PlanningPhaseV1;pattern:string}>;
export type PlanningCounterV1=Readonly<{name:string;pattern:string;group:number}>;
export type PlanningScenarioV1=Readonly<{id:string;kind:string;title:Readonly<{en:string;de:string}>;projects:readonly string[];dryRun?:boolean;runs:number;primes?:number;intervalMs?:number;edit?:Readonly<{file:string;comment:"//"|"#"}>;metrics:readonly string[]}>;
export type PlanningScenarioSetV1=Readonly<{version:1;markers:readonly PlanningMarkerV1[];counters:readonly PlanningCounterV1[];stores:readonly string[];scenarios:readonly PlanningScenarioV1[]}>;
export type PlanningLineV1=Readonly<{atMs:number;text:string}>;
export type PlanningSampleV1=Readonly<{run:number;exit:number|null;wallMs:number;phaseMs:Readonly<Record<PlanningPhaseV1,number>>;closures:Readonly<{verified:number|null;reused:number|null;changed:number|null}>;peakRssMb:number|null;swapMb:number|null;oomKills:number|null;minAvailableMb:number|null;storeMb:number|null}>;
export type PlanningResultV1=Readonly<{scenario:string;status:"ran"|"skipped"|"failed";reason?:string;samples:readonly PlanningSampleV1[]}>;
export type PlanningReportV1=Readonly<{version:1;started:string;host:Readonly<{cpus:number;memoryMb:number;swapMb:number|null;loadPerCore:number|null}>;results:readonly PlanningResultV1[]}>;
export type PlanningObservationV1=Readonly<{phaseMs:Readonly<Record<PlanningPhaseV1,number>>;counters:Readonly<Record<string,number>>}>;

/** 🧭️ The schema of the planning scenarios and of the report one run leaves; the same document drives the runner, the validators of every language and the oracle. */
export const planningSchemaV1=definition;
/** 🗺️ The declared planning scenarios with the log markers that split a plan into phases and the counters read from its lines. */
export const planningScenariosV1:PlanningScenarioSetV1=scenarioSet as unknown as PlanningScenarioSetV1;

const order:readonly PlanningPhaseV1[]=["facts","verification","inference","planning"];

/** ⏱️ Reads one plan's output lines (with the milliseconds since start) into time per phase and the counters; a phase starts at its first marker, a missing phase takes no time and the last one ends with the process. */
export function planningObserveV1(lines:readonly PlanningLineV1[],wallMs:number,set:PlanningScenarioSetV1=planningScenariosV1):PlanningObservationV1{
 const markers=set.markers.map(row=>[row.phase,new RegExp(row.pattern,"u")] as const),counters=set.counters.map(row=>[row,new RegExp(row.pattern,"u")] as const);
 const starts=new Map<PlanningPhaseV1,number>(),totals:Record<string,number>={};
 for(const line of lines){
  for(const [phase,pattern] of markers)if(!starts.has(phase)&&pattern.test(line.text))starts.set(phase,line.atMs);
  for(const [row,pattern] of counters){const hit=pattern.exec(line.text);if(hit)totals[row.name]=(totals[row.name]??0)+Number(hit[row.group]);}
 }
 const bounds:number[]=[wallMs];
 for(const phase of [...order].slice(1).reverse())bounds.unshift(Math.min(starts.get(phase)??bounds[0]!,bounds[0]!));
 bounds.unshift(0);
 for(let at=1;at<bounds.length;at++)bounds[at]=Math.max(bounds[at]!,bounds[at-1]!);
 const phaseMs=Object.fromEntries(order.map((phase,index)=>[phase,bounds[index+1]!-bounds[index]!])) as Record<PlanningPhaseV1,number>;
 return Object.freeze({phaseMs,counters:Object.freeze(totals)});
}

/** 🧮️ The closures a plan verified, reused and found changed; null when the plan printed no such line. */
export function planningClosuresV1(observation:PlanningObservationV1):PlanningSampleV1["closures"]{
 const pick=(name:string)=>observation.counters[name]??null;
 return Object.freeze({verified:pick("closuresVerified"),reused:pick("closuresReused"),changed:pick("closuresChanged")});
}

/** 📈️ The median of the numbers, null when there are none. */
export function planningMedianV1(values:readonly (number|null)[]):number|null{
 const sorted=values.filter((value):value is number=>value!==null).sort((a,b)=>a-b);
 if(!sorted.length)return null;
 const mid=sorted.length>>1;
 return sorted.length%2?sorted[mid]!:(sorted[mid-1]!+sorted[mid]!)/2;
}

/** 🔬️ Counts OOM kills in kernel journal text. */
export function planningOomKillsV1(journal:string):number{
 return journal.split("\n").filter(line=>/Out of memory: Killed process|oom-kill:|invoked oom-killer/u.test(line)).length;
}

const labels={scenario:{en:"Scenario",de:"Szenario"},runs:{en:"Runs",de:"Läufe"},wall:{en:"Wall ms (median)",de:"Laufzeit ms (Median)"},facts:{en:"Facts ms",de:"Fakten ms"},verification:{en:"Verify ms",de:"Prüfung ms"},inference:{en:"Infer ms",de:"Ableitung ms"},planning:{en:"Plan ms",de:"Plan ms"},verified:{en:"Closures verified/reused",de:"Hüllen geprüft/wiederverwendet"},rss:{en:"Peak RSS MB",de:"Spitzen-RSS MB"},swap:{en:"Swap MB",de:"Swap MB"},oom:{en:"OOM kills",de:"OOM-Kills"},store:{en:"Store MB",de:"Store MB"},status:{en:"Status",de:"Status"}} as const;

/** 📋️ Renders a report as one markdown table in the language asked, medians over the samples of each scenario and a dash for what was not measured. */
export function planningRenderV1(report:PlanningReportV1,language:"en"|"de"):string{
 const t=(key:keyof typeof labels)=>labels[key][language],cell=(value:number|null)=>value===null?"-":String(Math.round(value*10)/10);
 const head=[t("scenario"),t("status"),t("runs"),t("wall"),t("facts"),t("verification"),t("inference"),t("planning"),t("verified"),t("rss"),t("swap"),t("oom"),t("store")];
 const rows=report.results.map(result=>{
  const s=result.samples,med=(pick:(sample:PlanningSampleV1)=>number|null)=>cell(planningMedianV1(s.map(pick))),pair=`${med(x=>x.closures.verified)}/${med(x=>x.closures.reused)}`;
  return [result.scenario,result.reason?`${result.status}: ${result.reason}`:result.status,String(s.length),med(x=>x.wallMs),med(x=>x.phaseMs.facts),med(x=>x.phaseMs.verification),med(x=>x.phaseMs.inference),med(x=>x.phaseMs.planning),pair,med(x=>x.peakRssMb),med(x=>x.swapMb),med(x=>x.oomKills),med(x=>x.storeMb)];
 });
 return [head,head.map(()=>"---"),...rows].map(row=>`| ${row.join(" | ")} |`).join("\n")+"\n";
}
