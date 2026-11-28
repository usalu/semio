import {mkdir,writeFile} from "node:fs/promises";
import {cpus,loadavg,platform,totalmem} from "node:os";
import {dirname,resolve} from "node:path";
import {incrementalLanguageV1} from "../../../⌨️command/🟦️.ts";
import {localMachineSpeedFactorV1} from "../../../../🐢️speed/🟨️.mjs";
import {LATENCY_POLICY_V1,latencyBudgetIssuesV1,latencyBudgetsV1,latencyExceededV1,latencyRenderV1,type LatencyBudgetsV1,type LatencyReportV1} from "../🟦️.ts";
import {runLatencyV1,latencySpawnV1,type LatencyPortsV1} from "../🏃️runner/🟦️.ts";

const policy=LATENCY_POLICY_V1,both=(english:string,german:string)=>`${english}\n${german}`,script="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚪️door/📜️script.ts";

/** 🍘️ The closed arguments of `repo bench latency`. */
export type LatencyArgumentsV1=Readonly<{commands:readonly string[]|"all";runs:number;root:string|undefined;file:string|undefined;json:boolean;strict:boolean;language:"en"|"de"|undefined}>;

/** 🍙️ The option names `repo bench latency` accepts, so the operations catalogue can be checked against the parser. */
export function latencyOptionNamesV1():{valued:readonly string[];plain:readonly string[];shared:readonly string[]}{return {valued:policy.options.valued,plain:policy.options.plain,shared:policy.options.shared};}

/** 🍚️ Reads the options after `bench latency`; every unknown or repeated option is refused in both languages. */
export function latencyArgumentsV1(args:readonly string[]):LatencyArgumentsV1{
 const seen=new Set<string>(),values:Record<string,string>={},flags=new Set<string>(),valued=new Set([...policy.options.valued,...policy.options.shared.filter(name=>name!=="--json")]),plain=new Set([...policy.options.plain,"--json"]);
 for(let index=0;index<args.length;index++){
  const argument=args[index]!,cut=argument.indexOf("="),name=cut<0?argument:argument.slice(0,cut);
  if(seen.has(name))throw Error(both(`Repeated option: ${name}`,`Wiederholte Option: ${name}`));
  seen.add(name);
  if(valued.has(name)){const value=cut<0?args[++index]:argument.slice(cut+1);if(!value||value.startsWith("--"))throw Error(both(`Option ${name} needs a value`,`Option ${name} braucht einen Wert`));values[name]=value;}
  else if(plain.has(name)&&cut<0)flags.add(name);
  else throw Error(both(`Unknown bench latency option: ${argument}`,`Unbekannte Option für bench latency: ${argument}`));
 }
 const language=values["--lang"],picked=values["--commands"]?.split(",").filter(Boolean),runs=values["--runs"]===undefined?3:Number(values["--runs"]);
 if(language!==undefined&&language!=="en"&&language!=="de")throw Error(both(`Unknown language: ${language} (en|de)`,`Unbekannte Sprache: ${language} (en|de)`));
 if(values["--runs"]!==undefined&&(!/^[1-9][0-9]*$/u.test(values["--runs"])||runs>policy.maximumRuns))throw Error(both(`Runs must be a whole number from 1 to ${policy.maximumRuns}`,`Läufe müssen eine ganze Zahl von 1 bis ${policy.maximumRuns} sein`));
 return Object.freeze({commands:picked===undefined||picked.includes("all")?"all":picked,runs,root:values["--root"],file:values["--file"],json:flags.has("--json"),strict:flags.has("--strict"),language:language as "en"|"de"|undefined});
}

/** 🍛️ What `repo bench latency` needs from the host: the way to print and write files, the environment, the runs and the checkout it measures. */
export type LatencyCommandPortsV1=Readonly<{write(line:string):void;writeText(path:string,text:string):Promise<void>;environment:Readonly<Record<string,string|undefined>>;root:string;executable:string;runs:LatencyPortsV1}>;

/** 🍜️ Runs `repo bench latency`: measures the selected commands on the fixed small targets, judges every phase group and round trip count against the declared budgets, prints the report as a table or JSON, writes the JSON file and warns about every budget exceeded; like `repo doctor`, only `--strict` turns a warning into a failing exit code. */
export async function runLatencyCommandV1(args:readonly string[],ports:LatencyCommandPortsV1,set:LatencyBudgetsV1=latencyBudgetsV1):Promise<LatencyReportV1>{
 const parsed=latencyArgumentsV1(args),issues=latencyBudgetIssuesV1(set);
 if(issues.length)throw Error(both(`Closed latency budget table required: ${issues.join("; ")}`,`Geschlossene Latenzbudgettabelle erforderlich: ${issues.join("; ")}`));
 const language=incrementalLanguageV1(parsed.language,ports.environment),report=await runLatencyV1({root:parsed.root===undefined?ports.root:resolve(parsed.root),executable:ports.executable,script,commands:parsed.commands,runs:parsed.runs,environment:ports.environment},ports.runs,set);
 if(parsed.file)await ports.writeText(parsed.file,JSON.stringify(report,null,2)+"\n");
 ports.write(parsed.json?JSON.stringify(report,null,2):latencyRenderV1(report,language,set));
 const exceeded=latencyExceededV1(report);
 if(parsed.json){if(parsed.strict&&exceeded.length)throw Error(both(`Latency budget exceeded: ${exceeded.join(", ")}`,`Latenzbudget überschritten: ${exceeded.join(", ")}`));return report;}
 if(!exceeded.length)ports.write(both("Every measured metric is within its budget.","Jede gemessene Größe liegt innerhalb ihres Budgets."));
 else{ports.write(both(`Over budget (${exceeded.length}): ${exceeded.join(", ")}`,`Über Budget (${exceeded.length}): ${exceeded.join(", ")}`));if(parsed.strict)throw Error(both(`Latency budget exceeded: ${exceeded.join(", ")}`,`Latenzbudget überschritten: ${exceeded.join(", ")}`));}
 return report;
}

/** 🍲️ Binds the command to the running build CLI: its invocation for progress and cancellation, this checkout as the place the measured commands run, the local speed factor and real clocks. */
export async function latencyCommandPortsV1(root:string,invocation:import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<LatencyCommandPortsV1>{
 const routing=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts");
 return Object.freeze({
  write:(line:string)=>{process.stdout.write(line+"\n");},
  writeText:async(path:string,text:string)=>{const target=resolve(path);await mkdir(dirname(target),{recursive:true});await writeFile(target,text);},
  environment:process.env,
  root,
  executable:process.execPath,
  runs:Object.freeze({
   spawn:latencySpawnV1,
   progress:(stage:string)=>routing.advanceScriptInvocation(invocation,"bench:latency:"+stage,"running"),
   check:()=>routing.checkScriptInvocation(invocation),
   factor:localMachineSpeedFactorV1(process.env),
   clock:()=>new Date().toISOString(),
   host:()=>({cpus:Math.max(1,cpus().length),memoryMb:Math.round(totalmem()/1048576),loadPerCore:platform()==="win32"?null:Math.round((loadavg()[0]??0)/Math.max(1,cpus().length)*100)/100}),
   signal:invocation.control.signal
  })
 });
}

/** 🌝️ The entry `repo bench latency` is routed to: the arguments after `latency`. */
export async function runLatencyEntryV1(root:string,args:readonly string[],invocation:import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation):Promise<void>{
 await runLatencyCommandV1(args,await latencyCommandPortsV1(root,invocation));
}
