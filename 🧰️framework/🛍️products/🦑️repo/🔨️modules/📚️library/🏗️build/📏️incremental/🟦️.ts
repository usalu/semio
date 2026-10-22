import {buildBackendRowV1} from "../🔌️backends/🟦️.ts";
import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🛠️ The build implementations behind the tool-neutral interface that the harness judges. */
export type IncrementalToolV1=string;
/** 🚦️ The phases of one scenario run: a warm re-run, the edited run and the reverted run. */
export type IncrementalPhaseV1="warm"|"edit"|"revert";
/** ✏️ The six kinds of edit every scenario applies. */
export type IncrementalEditKindV1="comment-only"|"body-change"|"signature-change"|"lockfile"|"unrelated-sibling"|"config";
/** 🧰️ The text mutations that realise an edit kind in a disposable copy. */
export type IncrementalEditOpV1="comment"|"private-body"|"public-api"|"whitespace";
/** 🌐️ The source languages a mutation can be rendered for. */
export type IncrementalLanguageV1="ts"|"rs"|"py"|"text";
/** 🧬️ Whether the output of a task changes for an edit kind: the basis of early cutoff. */
export type IncrementalEffectV1="changed"|"identical"|"unknown";
/** 🏷️ The tool family of a task, as the coverage classifier names it. */
export type IncrementalFamilyV1="bun-script"|"tsc"|"vitest"|"bun-test"|"vite"|"playwright"|"go"|"python"|"dotnet"|"latex"|"docker"|"wasm"|"cargo"|"aggregate";
/** 🧾️ One task of a slice: what it declares, not what it did. */
export type IncrementalSliceTaskV1=Readonly<{family:IncrementalFamilyV1;cache:boolean;outputs:boolean;dependsOn:readonly string[];consumes:readonly string[];inputs:readonly string[]}>;
/** 🍰️ The closure of a selection with the declared input files that matter to an evaluation. */
export type IncrementalSliceV1=Readonly<{version:1;selected:readonly string[];tasks:Readonly<Record<string,IncrementalSliceTaskV1>>}>;
/** ✂️ A change to files, named by its kind. */
export type IncrementalEditV1=Readonly<{kind:IncrementalEditKindV1;files:readonly string[]}>;
/** 🗂️ One row of the effect table: the output effect of an edit kind on a task family. */
export type IncrementalSensitivityRowV1=Readonly<{family:IncrementalFamilyV1;kind:IncrementalEditKindV1;effect:IncrementalEffectV1}>;
/** 🎯️ The tasks a tool must, may and must not re-execute, plus the dependents early cutoff spares. */
export type IncrementalExpectedV1=Readonly<{direct:readonly string[];must:readonly string[];may:readonly string[];cutoff:readonly string[]}>;
/** 👁️ What one run did per task, in the neutral vocabulary of both tools. */
export type IncrementalObservationV1=Readonly<{tool:IncrementalToolV1;executed:readonly string[];restored:readonly string[];cached:readonly string[];unmapped:readonly string[]}>;
/** ⚖️ The judgement of an observation against its expectation. */
export type IncrementalVerdictV1=Readonly<{status:"pass"|"stale"|"excess"|"stale-excess"|"failed"|"skipped";stale:readonly string[];excess:readonly string[];exact:boolean}>;
/** 🧪️ One authored scenario of the permanent conformance data. */
export type IncrementalScenarioV1=Readonly<{id:string;title:Readonly<{en:string;de:string}>;area:string;target:string;kind:IncrementalEditKindV1;mutation:Readonly<{op:IncrementalEditOpV1;language:IncrementalLanguageV1;file:string}>;tools?:readonly IncrementalToolV1[];effects?:readonly IncrementalSensitivityRowV1[]}>;
/** 📚️ The authored scenario data: texts, the effect table and the scenarios. */
export type IncrementalScenarioSetV1=Readonly<{version:1;labels:Readonly<Record<string,Readonly<{en:string;de:string}>>>;sensitivity:readonly IncrementalSensitivityRowV1[];scenarios:readonly IncrementalScenarioV1[]}>;
/** 📋️ One judged run of a scenario, tool and phase. */
export type IncrementalRowV1=Readonly<{scenario:string;tool:IncrementalToolV1;phase:IncrementalPhaseV1;expected:IncrementalExpectedV1;observation:IncrementalObservationV1;verdict:IncrementalVerdictV1;milliseconds:number;note?:string}>;
/** 📊️ All judged runs with their tally. */
export type IncrementalReportV1=Readonly<{version:1;rows:readonly IncrementalRowV1[];passed:number;failed:number;skipped:number}>;

/** ⚔️ The executed tasks of both implementations for one scenario and phase, side by side. */
export type IncrementalComparisonV1=Readonly<{scenario:string;phase:IncrementalPhaseV1;same:boolean;onlyNx:readonly string[];onlyBazel:readonly string[]}>;

const policy=schema.definitions.Policy.const,order=(left:string,right:string)=>left<right?-1:left>right?1:0,sorted=(values:Iterable<string>)=>[...new Set(values)].sort(order);
const closed=(name:keyof typeof schema.definitions,value:unknown,message:string):void=>{if(validateJsonSchemaSubset(schema.definitions[name],value,schema).length)throw Error(message);};

/** 🔗️ Names a task or a file with an ASCII Bazel label: tasks use the adapter's own `t_<16 hex of sha256(id)>` scheme. */
export function incrementalLabelV1(kind:"t"|"f",value:string):string{return `${kind}_${createHash("sha256").update(value).digest("hex").slice(0,16)}`;}

/** 🛡️ Admits a slice only when it is closed, references only its own tasks, stays within the bound and has no dependency cycle. */
export function incrementalSliceV1(value:unknown):IncrementalSliceV1{
 closed("Slice",value,"Closed incremental slice required");
 const slice=value as IncrementalSliceV1,ids=Object.keys(slice.tasks);
 if(ids.length>policy.maximumTasks)throw Error("Bounded incremental slice required");
 for(const id of ids){
  if(!new RegExp(policy.taskId,"u").test(id))throw Error(`Closed incremental task id required: ${id}`);
  const task=slice.tasks[id]!;
  for(const other of [...task.dependsOn,...task.consumes])if(!slice.tasks[other])throw Error(`Unknown incremental dependency ${other} of ${id}`);
  for(const other of task.consumes)if(!task.dependsOn.includes(other)&&!reaches(slice,id,other))throw Error(`Consumed task ${other} is not a dependency of ${id}`);
 }
 for(const id of slice.selected)if(!slice.tasks[id])throw Error(`Unknown selected incremental task ${id}`);
 const state=new Map<string,1|2>();
 const visit=(id:string):void=>{if(state.get(id)===2)return;if(state.get(id)===1)throw Error(`Incremental dependency cycle at ${id}`);state.set(id,1);for(const other of slice.tasks[id]!.dependsOn)visit(other);state.set(id,2);};
 for(const id of ids)visit(id);
 return slice;
}

function reaches(slice:IncrementalSliceV1,from:string,target:string):boolean{
 const seen=new Set<string>(),pending=[from];
 while(pending.length){const id=pending.pop()!;for(const other of slice.tasks[id]!.dependsOn){if(other===target)return true;if(!seen.has(other)){seen.add(other);pending.push(other);}}}
 return false;
}

/** 🔬️ Looks up the output effect of an edit kind on a task family: authored rows win in order, anything unlisted is unknown. */
export function incrementalEffectV1(rows:readonly IncrementalSensitivityRowV1[],family:IncrementalFamilyV1,kind:IncrementalEditKindV1,outputs:boolean):IncrementalEffectV1{
 if(!outputs)return "identical";
 return rows.find(row=>row.family===family&&row.kind===kind)?.effect??"unknown";
}

/** 🧮️ Evaluates the tasks a tool re-executes after an edit, as a pure function of the slice: declared inputs decide which tasks run, outputs decide who follows. Bazel follows every dependency whose output changed (early cutoff on output digests); Nx follows only the tasks that consume outputs. Uncached tasks always run. */
export function incrementalExpectedV1(sliceValue:unknown,tool:IncrementalToolV1,edit:IncrementalEditV1,sensitivity:readonly IncrementalSensitivityRowV1[]):IncrementalExpectedV1{
 const slice=incrementalSliceV1(sliceValue);closed("Edit",edit,"Closed incremental edit required");
 const ids=sorted(Object.keys(slice.tasks)),files=new Set(edit.files),dependents=new Map<string,string[]>();
 for(const id of ids)for(const other of buildBackendRowV1(tool).dependencyModel==="all"?slice.tasks[id]!.dependsOn:slice.tasks[id]!.consumes)dependents.set(other,[...dependents.get(other)??[],id]);
 const direct=ids.filter(id=>slice.tasks[id]!.inputs.some(file=>files.has(file))),always=ids.filter(id=>!slice.tasks[id]!.cache);
 const effect=(id:string)=>incrementalEffectV1(sensitivity,slice.tasks[id]!.family,edit.kind,slice.tasks[id]!.outputs);
 const closure=(follows:(id:string)=>boolean):Set<string>=>{
  const ran=new Set([...direct,...always]),changed=new Set(direct),pending=[...direct];
  while(pending.length){const id=pending.pop()!;if(!follows(id))continue;for(const other of dependents.get(id)??[])if(!changed.has(other)){changed.add(other);ran.add(other);pending.push(other);}}
  return ran;
 };
 const must=closure(id=>effect(id)==="changed"),may=closure(id=>effect(id)!=="identical");
 const cutoff=ids.filter(id=>!may.has(id)&&(buildBackendRowV1(tool).dependencyModel==="all"?slice.tasks[id]!.dependsOn:slice.tasks[id]!.consumes).some(other=>may.has(other)));
 return Object.freeze({direct:sorted(direct),must:sorted(must),may:sorted(may),cutoff});
}

/** 🌫️ The expectation of a run that follows no edit: only uncached tasks execute. */
export function incrementalQuiescentV1(sliceValue:unknown):IncrementalExpectedV1{
 const slice=incrementalSliceV1(sliceValue),always=sorted(Object.keys(slice.tasks).filter(id=>!slice.tasks[id]!.cache));
 return Object.freeze({direct:[],must:always,may:always,cutoff:[]});
}

/** 🔦️ Judges the executed tasks of a run: a task that must run but did not is stale, a task that ran beyond every allowed task is excess. */
export function incrementalVerdictV1(expected:IncrementalExpectedV1,executed:readonly string[]):IncrementalVerdictV1{
 closed("Expected",expected,"Closed incremental expectation required");
 const ran=new Set(executed),allowed=new Set(expected.may),stale=expected.must.filter(id=>!ran.has(id)),excess=sorted(executed.filter(id=>!allowed.has(id)));
 const status=stale.length&&excess.length?"stale-excess":stale.length?"stale":excess.length?"excess":"pass";
 return Object.freeze({status,stale:sorted(stale),excess,exact:!stale.length&&!excess.length&&expected.must.length===expected.may.length});
}

const number=(nonce:string)=>Number.parseInt(nonce.slice(0,6),36)%1000003;
const templates:Readonly<Record<IncrementalLanguageV1,Readonly<Partial<Record<IncrementalEditOpV1,(nonce:string)=>string>>>>>=Object.freeze({
 ts:{comment:nonce=>`// [incremental] ${nonce}\n`,"private-body":nonce=>`void ${number(nonce)};\n`,"public-api":nonce=>`export const __incremental${nonce}=${number(nonce)} as const;\n`},
 rs:{comment:nonce=>`// [incremental] ${nonce}\n`,"private-body":nonce=>`const _: u32 = ${number(nonce)};\n`,"public-api":nonce=>`/// [incremental] public probe ${nonce}\npub const __INCREMENTAL_${nonce.toUpperCase()}: u32 = ${number(nonce)};\n`},
 py:{comment:nonce=>`# [incremental] ${nonce}\n`,"private-body":nonce=>`_incremental_${nonce} = ${number(nonce)}\n`,"public-api":nonce=>`INCREMENTAL_${nonce.toUpperCase()} = ${number(nonce)}\n`},
 text:{comment:nonce=>`# [incremental] ${nonce}\n`}
});

/** 🖋️ Renders one mutation of a source text: appended at the end, so no existing line moves; `whitespace` adds one newline for configuration and lock files. The same input always gives the same output and the result always differs from the input. */
export function incrementalMutateV1(text:string,op:IncrementalEditOpV1,language:IncrementalLanguageV1,nonce:string):string{
 if(!new RegExp(policy.nonce,"u").test(nonce))throw Error("Closed incremental nonce required");
 if(op==="whitespace")return text+"\n";
 const render=templates[language]?.[op];
 if(!render)throw Error(`Incremental mutation ${op} is unsupported for ${language}`);
 return (text.endsWith("\n")||text===""?text:text+"\n")+render(nonce);
}

const kindOps:Readonly<Record<IncrementalEditKindV1,readonly IncrementalEditOpV1[]>>=Object.freeze({"comment-only":["comment"],"body-change":["private-body"],"signature-change":["public-api"],lockfile:["whitespace"],config:["whitespace"],"unrelated-sibling":["comment","whitespace"]});

/** 🔍️ Admits the authored scenario data and lists every inconsistency: duplicate ids, an op that does not realise its kind, an op without a template, an over-long path, a duplicate tool. */
export function incrementalScenarioIssuesV1(value:unknown):string[]{
 const issues=validateJsonSchemaSubset(schema.definitions.ScenarioSet,value,schema).map(String);
 if(issues.length)return issues;
 const set=value as IncrementalScenarioSetV1,seen=new Set<string>(),found:string[]=[];
 for(const scenario of set.scenarios){
  if(seen.has(scenario.id))found.push(`duplicate scenario ${scenario.id}`);
  seen.add(scenario.id);
  if(!kindOps[scenario.kind].includes(scenario.mutation.op))found.push(`${scenario.id}: ${scenario.mutation.op} does not realise ${scenario.kind}`);
  if(scenario.mutation.op!=="whitespace"&&!templates[scenario.mutation.language]?.[scenario.mutation.op])found.push(`${scenario.id}: ${scenario.mutation.op} is unsupported for ${scenario.mutation.language}`);
  if(scenario.mutation.file.length>policy.maximumPathLength)found.push(`${scenario.id}: path exceeds ${policy.maximumPathLength}`);
 }
 return found;
}

/** 🔢️ Tallies judged rows: a pass is a pass, a skip is a skip, everything else is a failure. */
export function incrementalReportV1(rows:readonly IncrementalRowV1[]):IncrementalReportV1{
 const passed=rows.filter(row=>row.verdict.status==="pass").length,skipped=rows.filter(row=>row.verdict.status==="skipped").length;
 const report={version:1 as const,rows,passed,failed:rows.length-passed-skipped,skipped};
 closed("Report",report,"Closed incremental report required");
 return Object.freeze(report);
}

/** 🤝️ Pairs the rows of Nx and Bazel for the same scenario and phase and lists the tasks only one of them executed; rows that failed or were skipped carry no observation and are left out. */
export function incrementalComparisonsV1(rows:readonly IncrementalRowV1[]):IncrementalComparisonV1[]{
 const usable=(row:IncrementalRowV1|undefined)=>row!==undefined&&row.verdict.status!=="failed"&&row.verdict.status!=="skipped",found:IncrementalComparisonV1[]=[];
 for(const nx of rows.filter(row=>row.tool==="nx"&&usable(row))){
  const bazel=rows.find(row=>row.tool==="bazel"&&row.scenario===nx.scenario&&row.phase===nx.phase&&usable(row));
  if(!bazel)continue;
  const onlyNx=nx.observation.executed.filter(id=>!bazel.observation.executed.includes(id)),onlyBazel=bazel.observation.executed.filter(id=>!nx.observation.executed.includes(id));
  found.push(Object.freeze({scenario:nx.scenario,phase:nx.phase,same:!onlyNx.length&&!onlyBazel.length,onlyNx:sorted(onlyNx),onlyBazel:sorted(onlyBazel)}));
 }
 return found;
}

/** 🕸️ Writes the generated Bazel package that answers the dependents question with Bazel's own `rdeps`: a filegroup per task naming its dependencies and the edited files it declares. */
export function incrementalRdepsPackageV1(sliceValue:unknown,tool:IncrementalToolV1,files:readonly string[]):string{
 const slice=incrementalSliceV1(sliceValue),quote=(value:string)=>JSON.stringify(value),edited=new Set(files),lines:string[]=[];
 for(const id of sorted(Object.keys(slice.tasks))){
  const task=slice.tasks[id]!,sources=sorted([...(buildBackendRowV1(tool).dependencyModel==="all"?task.dependsOn:task.consumes).map(other=>":"+incrementalLabelV1("t",other)),...task.inputs.filter(file=>edited.has(file)).map(file=>":"+incrementalLabelV1("f",file))]);
  lines.push(`filegroup(name = ${quote(incrementalLabelV1("t",id))}, srcs = [${sources.map(quote).join(", ")}])`);
 }
 for(const file of sorted(edited))lines.push(`filegroup(name = ${quote(incrementalLabelV1("f",file))})`);
 return lines.join("\n")+"\n";
}

/** 🔎️ The one `rdeps` expression over the generated package that returns every task reachable from the edited files. */
export function incrementalRdepsExpressionV1(files:readonly string[]):string{
 if(!files.length)throw Error("Edited files required");
 return `rdeps(//:all, set(${sorted(files).map(file=>"//:"+incrementalLabelV1("f",file)).join(" ")}))`;
}

/** 📥️ Reads the `rdeps` output back into sorted task ids, ignoring the file nodes. */
export function incrementalRdepsTasksV1(sliceValue:unknown,output:string):string[]{
 const slice=incrementalSliceV1(sliceValue),names=new Map(Object.keys(slice.tasks).map(id=>["//:"+incrementalLabelV1("t",id),id]));
 return sorted(output.split("\n").map(line=>line.trim()).filter(Boolean).flatMap(line=>names.has(line)?[names.get(line)!]:[]));
}

const localized=(set:IncrementalScenarioSetV1,key:string,language:"en"|"de"):string=>{const row=set.labels[key];if(!row)throw Error(`Unknown incremental label ${key}`);return row[language];};

/** 🖨️ Renders a report as an aligned table in the chosen language, followed by the stale and excess tasks of every failed row. */
export function renderIncrementalReportV1(report:IncrementalReportV1,set:IncrementalScenarioSetV1,language:"en"|"de"):string{
 const text=(key:string)=>localized(set,key,language),status=(row:IncrementalRowV1)=>row.verdict.status==="stale-excess"?`${text("stale")}+${text("excess")}`:text(row.verdict.status);
 const head=["scenario","tool","phase","must","may","executed","restored","cutoff","verdict","milliseconds"].map(text),
  body=report.rows.map(row=>[row.scenario,row.tool,text(row.phase),String(row.expected.must.length),String(row.expected.may.length),String(row.observation.executed.length),String(row.observation.restored.length),String(row.expected.cutoff.length),status(row),String(row.milliseconds)]);
 const widths=head.map((cell,column)=>Math.max(cell.length,...body.map(row=>row[column]!.length))),line=(cells:readonly string[])=>cells.map((cell,column)=>column===0||column===1||column===2||column===8?cell.padEnd(widths[column]!):cell.padStart(widths[column]!)).join("  ").trimEnd();
 const out=[text("title"),"",line(head),...body.map(line),"",`${text("summary")}: ${text("pass")} ${report.passed}, ${text("failed")} ${report.failed}, ${text("skipped")} ${report.skipped}`];
 for(const row of report.rows)if(row.verdict.status!=="pass"&&row.verdict.status!=="skipped"){
  out.push("",`${row.scenario} / ${row.tool} / ${text(row.phase)}`);
  if(row.verdict.stale.length)out.push(`  ${text("stale")}: ${row.verdict.stale.join(", ")}`);
  if(row.verdict.excess.length)out.push(`  ${text("excess")}: ${row.verdict.excess.join(", ")}`);
  if(row.note)out.push(`  ${row.note}`);
 }
 const compared=incrementalComparisonsV1(report.rows);
 if(compared.length){
  out.push("",`${text("comparison")}: nx / bazel`);
  for(const row of compared){
   out.push(`  ${row.scenario} / ${text(row.phase)}: ${row.same?text("agree"):text("differ")}`);
   if(row.onlyNx.length)out.push(`    ${text("only")} nx: ${row.onlyNx.join(", ")}`);
   if(row.onlyBazel.length)out.push(`    ${text("only")} bazel: ${row.onlyBazel.join(", ")}`);
  }
 }
 return out.join("\n");
}

/** 📑️ Renders the scenario list as an aligned table in the chosen language. */
export function renderIncrementalScenariosV1(set:IncrementalScenarioSetV1,language:"en"|"de"):string{
 const text=(key:string)=>localized(set,key,language),head=[text("scenario"),text("area"),text("kind"),text("target"),text("title")],body=set.scenarios.map(row=>[row.id,row.area,row.kind,row.target,row.title[language]]);
 const widths=head.map((cell,column)=>Math.max(cell.length,...body.map(row=>row[column]!.length)));
 return [head,...body].map(cells=>cells.map((cell,column)=>cell.padEnd(widths[column]!)).join("  ").trimEnd()).join("\n");
}
