import definition from "./🧬️schema/🔣️.json" with {type:"json"};
import scenarioSet from "./🎯️scenarios/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../..//🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🛰️ The metrics a fleet probe yields. */
export type FleetMetricV1="hitRatio"|"executedActions"|"divergentWrites"|"offloadMs"|"fallbackMs"|"offloadedUnits"|"capacityBreaches"|"reservedBytes"|"storeBytes"|"firstTaskMs"|"readyRatio"|"wallMs";
/** 📐️ The numbers one scenario run measured, by metric. */
export type FleetValuesV1=Readonly<Partial<Record<FleetMetricV1,number>>>;
/** 🎬️ One declared scenario: its id, its title in both languages and the metrics it must yield. */
export type FleetScenarioV1=Readonly<{id:string;title:Readonly<{en:string;de:string}>;metrics:readonly FleetMetricV1[];optional?:readonly FleetMetricV1[]}>;
/** 📒️ The declared scenarios of the fleet probes. */
export type FleetScenarioSetV1=Readonly<{version:1;scenarios:readonly FleetScenarioV1[]}>;
/** 📤️ What a scenario child prints: the scenario and its closed values. */
export type FleetOutcomeV1=Readonly<{version:1;scenario:string;values:FleetValuesV1}>;
/** 🧮️ The verdict of the laws a simulated host is held to: the measured number and the laws that were broken. */
export type FleetLawsV1=Readonly<{value:number;breaches:readonly string[]}>;

/** 🧭️ The schema of the fleet scenarios and of the outcome a scenario prints; the same document drives the runner, the validators of every language and the oracle. */
export const fleetSchemaV1=definition;
/** 🧱️ The fixed sizes of the probes: units per run, simulated host, bounds. */
export const fleetPolicyV1=definition.definitions.Policy.const;
/** 🗺️ The declared scenarios. */
export const fleetScenariosV1:FleetScenarioSetV1=scenarioSet as unknown as FleetScenarioSetV1;

const both=(english:string,german:string)=>`${english}\n${german}`;

/** 🔎️ Admits the declared scenarios against their schema and lists every inconsistency: a duplicate id or a metric the policy does not know. */
export function fleetScenarioIssuesV1(value:unknown=scenarioSet):string[]{
 const issues=validateJsonSchemaSubset(definition.definitions.Scenarios,value,definition).map(String);
 if(issues.length)return issues;
 const seen=new Set<string>(),found:string[]=[];
 for(const row of (value as FleetScenarioSetV1).scenarios){if(seen.has(row.id))found.push(`duplicate scenario ${row.id}`);seen.add(row.id);}
 return found;
}

/** 🌡️ Judges a warm second machine: of the units the first one published, how many the second one found (`hitRatio`) and how many it would have to compute again; units that were never published are controls and do not count. */
export function fleetWarmV1(input:Readonly<{published:number;hits:number}>):FleetValuesV1{
 if(!Number.isInteger(input.published)||input.published<1||!Number.isInteger(input.hits)||input.hits<0||input.hits>input.published)throw Error(both("A positive published count and a hit count within it are required","Eine positive Zahl veröffentlichter Einheiten und eine Trefferzahl darin werden benötigt"));
 return Object.freeze({hitRatio:input.hits/input.published,executedActions:input.published-input.hits});
}

/** 🪞️ Counts the divergent writes of one unit built twice: every output whose bytes differ between the two builds and every write the cache refused as a divergence of its own action key. */
export function fleetTwinV1(input:Readonly<{outputs:readonly Readonly<{name:string;left:string;right:string}>[];answers:readonly (boolean|"diverged"|"stale")[]}>):FleetValuesV1{
 if(!input.outputs.length)throw Error(both("A unit with outputs is required","Eine Einheit mit Ausgaben wird benötigt"));
 return Object.freeze({divergentWrites:input.outputs.filter(row=>row.left!==row.right).length+input.answers.filter(answer=>answer==="diverged").length});
}

/** 🧠️ Holds the decisions of the capacity gate on a simulated host to its laws: admitted jobs never exceed the cores, the memory they reserve stays within what is available beyond the floor, a critical pressure admits nothing, and a roomy host admits at least one request. */
export function fleetMemoryLawsV1(input:Readonly<{cores:number;floorMegabytes:number;availableMegabytes:number;pressure:"normal"|"warn"|"critical";decisions:readonly Readonly<{verdict:"admit"|"wait";jobs:number;reservedMegabytes:number}>[]}>):FleetLawsV1{
 const admitted=input.decisions.filter(row=>row.verdict==="admit"),reserved=admitted.reduce((sum,row)=>sum+row.reservedMegabytes,0),jobs=admitted.reduce((sum,row)=>sum+row.jobs,0),breaches:string[]=[];
 if(jobs>input.cores)breaches.push(`jobs ${jobs} exceed ${input.cores} cores`);
 if(reserved>Math.max(0,input.availableMegabytes-input.floorMegabytes))breaches.push(`reserved ${reserved} MB exceed available ${input.availableMegabytes} MB beyond the floor ${input.floorMegabytes} MB`);
 if(input.pressure==="critical"&&admitted.length>0)breaches.push(`${admitted.length} requests admitted under critical pressure`);
 if(input.pressure==="normal"&&input.availableMegabytes-input.floorMegabytes>0&&input.decisions.length>0&&admitted.length===0)breaches.push("no request admitted on a roomy host");
 return Object.freeze({value:reserved,breaches:Object.freeze(breaches)});
}

/** 💽️ Holds the images born on a host with the given free bytes to their laws: every image is a whole number of mebibytes and together they stay within the share of the free bytes the policy allows. */
export function fleetDiskLawsV1(input:Readonly<{freeBytes:number;born:readonly number[];sharePercent:number;mebibyte:number}>):FleetLawsV1{
 const total=input.born.reduce((sum,bytes)=>sum+bytes,0),breaches:string[]=[];
 for(const bytes of input.born)if(bytes%input.mebibyte!==0)breaches.push(`image of ${bytes} bytes is not a whole number of mebibytes`);
 if(total>input.freeBytes*input.sharePercent/100)breaches.push(`images of ${total} bytes exceed ${input.sharePercent} % of ${input.freeBytes} free bytes`);
 return Object.freeze({value:total,breaches:Object.freeze(breaches)});
}

/** 🖥️ Condenses the memory laws of every simulated moment and the disk laws of the simulated disk to the metrics of the host scenario: the number of broken laws, the most memory ever reserved and the disk the images take. */
export function fleetHostV1(input:Readonly<{memory:readonly FleetLawsV1[];disk:FleetLawsV1;mebibyte:number}>):FleetValuesV1{
 return Object.freeze({capacityBreaches:input.memory.reduce((sum,row)=>sum+row.breaches.length,0)+input.disk.breaches.length,reservedBytes:Math.max(0,...input.memory.map(row=>row.value))*input.mebibyte,storeBytes:input.disk.value});
}

/** 📦️ Closes the values of one scenario: exactly the declared metrics, each a finite number that is not negative, as the closed outcome schema wants them. */
export function fleetOutcomeV1(scenario:string,values:FleetValuesV1,set:FleetScenarioSetV1=fleetScenariosV1):FleetOutcomeV1{
 const row=set.scenarios.find(candidate=>candidate.id===scenario);
 if(!row)throw Error(both(`Unknown fleet scenario: ${scenario}`,`Unbekanntes Flottenszenario: ${scenario}`));
 const keys=Object.keys(values).sort(),wanted=[...row.metrics].sort(),optional=new Set(row.optional??[]);
 if(keys.some(key=>!wanted.includes(key as FleetMetricV1))||wanted.some(key=>!optional.has(key)&&!keys.includes(key)))throw Error(both(`The scenario ${scenario} must yield exactly ${wanted.join(", ")} (got ${keys.join(", ")})`,`Das Szenario ${scenario} muss genau ${wanted.join(", ")} liefern (erhalten: ${keys.join(", ")})`));
 const outcome={version:1 as const,scenario,values};
 if(validateJsonSchemaSubset(definition.definitions.Outcome,outcome,definition).length)throw Error(both(`The outcome of ${scenario} is not a closed outcome`,`Das Ergebnis von ${scenario} ist kein geschlossenes Ergebnis`));
 return Object.freeze(outcome);
}

/** 🚦️ One readiness receipt of a worker as the fleet store keeps it (`origins/<origin>/ready.json`): whether the worker is Ready, when its supervisor started, when it became Ready and when the receipt was last written. */
export type FleetReceiptV1=Readonly<{ready:boolean;startedMs:number;readyMs:number|null;updatedMs:number}>;

/** 🩺️ Judges the readiness of the fleet from the receipts written within `maximumAgeMs` before `nowMs` (older receipts belong to workers that are gone): the share of them that is Ready and, when some are, the median milliseconds from the start of their supervisor to the moment they were Ready (a receipt whose Ready time lies before its start belongs to an earlier life and counts as Ready without a time). */
export function fleetReadyV1(receipts:readonly FleetReceiptV1[],nowMs:number,maximumAgeMs:number):FleetValuesV1{
 const live=receipts.filter(row=>nowMs-row.updatedMs<=maximumAgeMs);
 if(!live.length)throw Error(both("No worker wrote a receipt recently","Kein Worker hat kürzlich einen Beleg geschrieben"));
 const ready=live.filter(row=>row.ready),times=ready.flatMap(row=>row.readyMs!==null&&row.readyMs>=row.startedMs?[row.readyMs-row.startedMs]:[]).sort((left,right)=>left-right),middle=(times.length-1)/2;
 return Object.freeze({readyRatio:ready.length/live.length,...times.length?{wallMs:(times[Math.floor(middle)]!+times[Math.ceil(middle)]!)/2}:{}});
}

/** ⏱️ Reads the output of one command with the milliseconds at which each line arrived: the time to the first task line (`> nx run ...`) and the time the command took in all. */
export function fleetFirstTaskV1(lines:readonly Readonly<{atMs:number;text:string}>[],wallMs:number):FleetValuesV1{
 const first=lines.find(row=>/^> nx run /u.test(row.text));
 if(!first)throw Error(both("The command printed no task line","Der Befehl gab keine Task-Zeile aus"));
 return Object.freeze({firstTaskMs:first.atMs,wallMs});
}
