import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🛠️ The two build implementations behind the neutral interface. */
export type ParityToolV1=string;
/** 🧪️ What a sample can be compared on. */
export type ParityCheckV1="graph"|"closure"|"outputs"|"results"|"inputs"|"restore";
/** 🧐️ A check or the observation itself when a tool could not be observed. */
export type ParityReportedCheckV1=ParityCheckV1|"observation";
/** ⏱️ A run on a cold cache or a run restored from the cache of the tool. */
export type ParityPhaseV1="fresh"|"restored";
/** 🌐️ A text in every language the verifier speaks. */
export type ParityTextV1=Readonly<{en:string;de:string}>;
/** 🕸️ The neutral task graph both tools print for a dry run. */
export type ParityPlanV1=Readonly<{roots:readonly string[];tasks:Readonly<Record<string,Readonly<{id:string;target:Readonly<{project:string;target:string;configuration?:string}>}>>>;dependencies:Readonly<Record<string,readonly string[]>>;continuousDependencies?:Readonly<Record<string,readonly string[]>>}>;
/** 🧾️ One captured output entry. */
export type ParityFileV1=Readonly<{kind:"file"|"directory"|"symlink";mode:number;size?:number;sha256?:string;link?:string;normalized?:Readonly<Record<string,string>>}>;
/** 📦️ The declared outputs of a run: which roots each task owns and every entry beneath them. */
export type ParityOutputsV1=Readonly<{roots:Readonly<Record<string,readonly string[]>>;files:Readonly<Record<string,ParityFileV1>>}>;
/** 🏁️ How a run ended and what each failed or test task reported. */
export type ParityRunV1=Readonly<{exit:number|null;reason:"exit"|"timeout"|"cancelled"|"output-limit"|"spawn-error";durationMs?:number;failedTasks:readonly string[];tests:Readonly<Record<string,"passed"|"failed"|"no-status">>}>;
/** 🧮️ What Nx hashes for one task. */
export type ParityNxInputsV1=Readonly<{files:readonly string[];environment:readonly string[];runtime:readonly string[];depOutputs:readonly string[];external:readonly string[]}>;
/** 🔭️ What Bazel gave the spawns of one task. */
export type ParitySpawnV1=Readonly<{inputs:readonly string[];env:readonly string[];exit:number|null}>;
/** 🧩️ Everything one tool left for one sample and phase. */
export type ParityObservationV1=Readonly<{version:1;sample:string;tool:ParityToolV1;phase:ParityPhaseV1;plan:ParityPlanV1|null;run:ParityRunV1|null;outputs:ParityOutputsV1|null;inputs:Readonly<Record<string,ParityNxInputsV1>>|null;spawns:Readonly<Record<string,ParitySpawnV1>>|null;closure:Readonly<{queried:readonly string[]}>|null;errors:readonly string[]}>;
/** 📜️ One documented normalization. */
export type ParityRuleV1=Readonly<{id:string;kind:"ignore-glob"|"text-replace"|"json-redact"|"strip-prefix"|"mode-mask"|"ambient-env"|"covered-by";surface:string;justification:ParityTextV1;evidence:string;glob?:string;pattern?:string;flags?:string;replacement?:string;pointers?:readonly string[];prefix?:string;mask?:number;names?:readonly string[];category?:"external"|"runtime";anyOf?:readonly string[]}>;
/** 🧰️ How the verifier reads task results out of tool output. */
export type ParityExtractorsV1=Readonly<{failedTasksHeading:string;failedTask:string;testTarget:string}>;
/** 📚️ The documented normalizations and extractors. */
export type ParityRulesV1=Readonly<{version:1;extractors:ParityExtractorsV1;rules:readonly ParityRuleV1[]}>;
/** 🎯️ One sample target. */
export type ParitySampleV1=Readonly<{id:string;area:string;target:string;configuration?:"prototype"|"release";skipCache?:boolean;phases?:readonly ParityPhaseV1[];checks?:readonly ParityCheckV1[];expectExit?:number;inputTasks?:readonly string[];timeoutSeconds?:number;maximumOutputBytes?:number;note:ParityTextV1}>;
/** 🗂️ The sample set with its defaults. */
export type ParitySamplesV1=Readonly<{version:1;defaults:Readonly<{phases:readonly ParityPhaseV1[];checks:readonly ParityCheckV1[];expectExit:number;timeoutSeconds:number;maximumOutputBytes:number}>;samples:readonly ParitySampleV1[]}>;
/** 🚨️ One difference between the tools. */
export type ParityDivergenceV1=Readonly<{check:ParityReportedCheckV1;code:string;subject:string;nx?:string;bazel?:string}>;
/** 📋️ The outcome of one check in one phase. */
export type ParityCheckReportV1=Readonly<{check:ParityReportedCheckV1;phase:ParityPhaseV1;status:"equal"|"diverged"|"not-applicable"|"unavailable";divergences:readonly ParityDivergenceV1[]}>;
/** 🎫️ The outcome of one sample. */
export type ParitySampleReportV1=Readonly<{sample:string;area:string;target:string;status:"equal"|"diverged"|"error";checks:readonly ParityCheckReportV1[]}>;
/** 📊️ The outcome of a whole verification. */
export type ParityReportV1=Readonly<{version:1;samples:readonly ParitySampleReportV1[];totals:Readonly<{samples:number;equal:number;diverged:number;error:number;divergences:number}>;rules:readonly Readonly<{id:string;applied:number}>[]}>;

type Applied=Map<string,number>;
const definitions=(schema as {definitions:Record<string,unknown>}).definitions;
const closed=(name:string,value:unknown,message:string):void=>{if(validateJsonSchemaSubset(definitions[name],value,schema).length)throw Error(message);};

/** 🔐️ Admits the sample set only when it is complete, unique and every default is closed. */
export function readParitySamplesV1(value:unknown):ParitySamplesV1{
 closed("Samples",value,"Closed parity sample set required");
 const samples=value as ParitySamplesV1,ids=samples.samples.map(row=>row.id);
 if(new Set(ids).size!==ids.length)throw Error("Unique parity sample identities required");
 return samples;
}

/** 🔒️ Admits the normalizations only when each is justified in every language, unique and compilable. */
export function readParityRulesV1(value:unknown):ParityRulesV1{
 closed("Rules",value,"Closed parity rules required");
 const rules=value as ParityRulesV1,ids=rules.rules.map(row=>row.id);
 if(new Set(ids).size!==ids.length)throw Error("Unique parity rule identities required");
 for(const row of rules.rules){if(row.kind==="text-replace")new RegExp(row.pattern!,row.flags);if(row.glob!==undefined)globRegexV1(row.glob);}
 for(const pattern of Object.values(rules.extractors))new RegExp(pattern);
 return rules;
}

/** 🌿️ A sample with every default applied. */
export type ParityResolvedSampleV1=Readonly<{id:string;area:string;target:string;configuration?:"prototype"|"release";skipCache:boolean;note:ParityTextV1;phases:readonly ParityPhaseV1[];checks:readonly ParityCheckV1[];expectExit:number;inputTasks:readonly string[];timeoutSeconds:number;maximumOutputBytes:number}>;

/** 🌱️ Resolves the sample against the defaults. */
export function resolveParitySampleV1(samples:ParitySamplesV1,sample:ParitySampleV1):ParityResolvedSampleV1{
 const defaults=samples.defaults;
 return {...sample,skipCache:sample.skipCache??true,phases:sample.phases??defaults.phases,checks:sample.checks??defaults.checks,expectExit:sample.expectExit??defaults.expectExit,inputTasks:sample.inputTasks??[],timeoutSeconds:sample.timeoutSeconds??defaults.timeoutSeconds,maximumOutputBytes:sample.maximumOutputBytes??defaults.maximumOutputBytes};
}

const escape=(character:string):string=>/[\\^$.*+?()[\]{}|]/u.test(character)?"\\"+character:character;

/** 🔣️ Compiles a path pattern: `**` crosses directories, `*` and `?` stay inside one, everything else is literal. */
export function globRegexV1(glob:string):RegExp{
 let source="";
 for(let index=0;index<glob.length;index++){
  const character=glob[index]!;
  if(character==="*"){
   if(glob[index+1]==="*"){if(glob[index+2]==="/"){source+="(?:.*/)?";index+=2;}else{source+=".*";index+=1;}}
   else source+="[^/]*";
  }else if(character==="?")source+="[^/]";
  else source+=escape(character);
 }
 return new RegExp("^"+source+"$","su");
}

/** 🔎️ Tells whether a path pattern matches a whole path. */
export function globMatchV1(glob:string,path:string):boolean{return globRegexV1(glob).test(path);}

const compiled=new Map<string,RegExp>();
const matches=(glob:string,path:string):boolean=>{let expression=compiled.get(glob);if(!expression){expression=globRegexV1(glob);compiled.set(glob,expression);}return expression.test(path);};
const count=(applied:Applied|undefined,rule:string):void=>{if(applied)applied.set(rule,(applied.get(rule)??0)+1);};
/** 🔤️ Orders strings by Unicode code point, the order every implementation of the comparator uses. */
export function parityCompareV1(left:string,right:string):number{
 const a=[...left],b=[...right];
 for(let index=0;index<Math.min(a.length,b.length);index++){const x=a[index]!.codePointAt(0)!,y=b[index]!.codePointAt(0)!;if(x!==y)return x<y?-1:1;}
 return a.length-b.length;
}
const sorted=(values:Iterable<string>):string[]=>[...new Set(values)].sort(parityCompareV1);
const diffOf=(left:Iterable<string>,right:Iterable<string>):string[]=>{const other=new Set(right);return sorted(left).filter(value=>!other.has(value));};
const sha256=(value:string|Uint8Array):string=>createHash("sha256").update(value).digest("hex");

/** 🔑️ Names a task by project, target and configuration, whatever id each tool spells. */
export function parityTaskKeyV1(plan:ParityPlanV1,id:string):string{
 const task=plan.tasks[id];
 return task?`${task.target.project}:${task.target.target}${task.target.configuration?":"+task.target.configuration:""}`:id;
}

/** 🧬️ Canonical JSON text: keys sorted at every depth. */
export function parityCanonicalJsonV1(value:unknown):string{
 const order=(node:unknown):unknown=>Array.isArray(node)?node.map(order):node!==null&&typeof node==="object"?Object.fromEntries(Object.keys(node).sort().map(key=>[key,order((node as Record<string,unknown>)[key])])):node;
 return JSON.stringify(order(value));
}

/** 🧽️ Computes what one output file looks like after each applicable documented normalization, keyed by rule. */
export function parityNormalizedDigestsV1(path:string,content:Uint8Array,rules:ParityRulesV1):Record<string,string>{
 const result:Record<string,string>={};
 for(const rule of rules.rules){
  if(rule.surface!=="output"||rule.glob===undefined||!matches(rule.glob,path))continue;
  let text:string;try{text=new TextDecoder("utf-8",{fatal:true}).decode(content);}catch{continue;}
  if(rule.kind==="text-replace")result[rule.id]=sha256(text.replace(new RegExp(rule.pattern!,rule.flags),rule.replacement!));
  else if(rule.kind==="json-redact"){
   let document:unknown;try{document=JSON.parse(text);}catch{continue;}
   for(const pointer of rule.pointers!){
    const parts=pointer.slice(1).split("/").map(part=>part.replaceAll("~1","/").replaceAll("~0","~"));let node:unknown=document;
    for(const part of parts.slice(0,-1))node=node!==null&&typeof node==="object"?(node as Record<string,unknown>)[part]:undefined;
    if(node!==null&&typeof node==="object"&&Object.hasOwn(node,parts.at(-1)!))(node as Record<string,unknown>)[parts.at(-1)!]="<redacted>";
   }
   result[rule.id]=sha256(parityCanonicalJsonV1(document));
  }
 }
 return result;
}

const diverge=(check:ParityReportedCheckV1,code:string,subject:string,nx?:string,bazel?:string):ParityDivergenceV1=>({check,code,subject,...(nx===undefined?{}:{nx}),...(bazel===undefined?{}:{bazel})});
const order=(rows:ParityDivergenceV1[]):ParityDivergenceV1[]=>rows.sort((left,right)=>parityCompareV1(left.code,right.code)||parityCompareV1(left.subject,right.subject));

/** 🔀️ Compares the neutral task graphs: tasks, ordering edges, roots and edges that only need a running dependency. */
export function compareParityGraphV1(nx:ParityPlanV1|null,bazel:ParityPlanV1|null):ParityDivergenceV1[]{
 if(!nx||!bazel)return [diverge("graph","plan-unavailable",!nx&&!bazel?"nx,bazel":!nx?"nx":"bazel")];
 const keys=(plan:ParityPlanV1)=>new Map(Object.keys(plan.tasks).map(id=>[parityTaskKeyV1(plan,id),id])),left=keys(nx),right=keys(bazel),rows:ParityDivergenceV1[]=[];
 for(const key of diffOf(left.keys(),right.keys()))rows.push(diverge("graph","task-missing-bazel",key));
 for(const key of diffOf(right.keys(),left.keys()))rows.push(diverge("graph","task-missing-nx",key));
 const edges=(plan:ParityPlanV1,names:Map<string,string>,field:"dependencies"|"continuousDependencies")=>new Set([...names].flatMap(([key,id])=>(plan[field]?.[id]??[]).map(dependency=>`${key} -> ${parityTaskKeyV1(plan,dependency)}`)));
 for(const[field,onlyNx,onlyBazel]of [["dependencies","edge-missing-bazel","edge-missing-nx"],["continuousDependencies","continuous-edge-differs","continuous-edge-differs"]] as const){
  const a=edges(nx,left,field),b=edges(bazel,right,field);
  for(const edge of diffOf(a,b))rows.push(diverge("graph",onlyNx,edge,"present","absent"));
  for(const edge of diffOf(b,a))rows.push(diverge("graph",onlyBazel,edge,"absent","present"));
 }
 const roots=(plan:ParityPlanV1)=>sorted(plan.roots.map(id=>parityTaskKeyV1(plan,id)));
 if(JSON.stringify(roots(nx))!==JSON.stringify(roots(bazel)))rows.push(diverge("graph","root-differs","roots",roots(nx).join(","),roots(bazel).join(",")));
 return order(rows);
}

/** 🔗️ Compares the planner closure with what `bazel cquery` returns for the selected tasks. */
export function compareParityClosureV1(plan:ParityPlanV1|null,closure:Readonly<{queried:readonly string[]}>|null):ParityDivergenceV1[]{
 if(!plan||!closure)return [diverge("closure","closure-unavailable",!plan?"plan":"cquery")];
 const planned=Object.keys(plan.tasks).map(id=>parityTaskKeyV1(plan,id)),queried=closure.queried;
 return order([...diffOf(planned,queried).map(key=>diverge("closure","closure-task-missing-in-query",key)),...diffOf(queried,planned).map(key=>diverge("closure","closure-task-extra-in-query",key))]);
}

/** 🔬️ Compares the declared outputs byte for byte; only documented normalizations may explain a difference. `restore` compares a tool's cold outputs with its own cache-restored outputs instead of one tool with the other. */
export function compareParityOutputsV1(nx:ParityOutputsV1|null,bazel:ParityOutputsV1|null,rules:ParityRulesV1,applied?:Applied,mode:"tools"|"restore"="tools"):ParityDivergenceV1[]{
 const check:ParityCheckV1=mode==="tools"?"outputs":"restore",names=mode==="tools"?{missingLeft:"output-missing-nx",missingRight:"output-missing-bazel",kind:"output-kind-differs",link:"output-link-differs",mode:"output-mode-differs",bytes:"output-bytes-differ"}:{missingLeft:"restore-missing",missingRight:"restore-missing",kind:"restore-kind-differs",link:"restore-kind-differs",mode:"restore-mode-differs",bytes:"restore-bytes-differ"};
 if(!nx||!bazel)return [];
 const ignore=rules.rules.filter(rule=>rule.kind==="ignore-glob"&&rule.surface==="output"),mask=rules.rules.find(rule=>rule.kind==="mode-mask"),rows:ParityDivergenceV1[]=[];
 const ignored=(path:string):boolean=>{const rule=ignore.find(row=>matches(row.glob!,path));if(rule){count(applied,rule.id);return true;}return false;};
 for(const path of sorted([...Object.keys(nx.files),...Object.keys(bazel.files)])){
  if(ignored(path))continue;
  const a=nx.files[path],b=bazel.files[path];
  if(!a){rows.push(diverge(check,names.missingLeft,path,"absent",b!.kind));continue;}
  if(!b){rows.push(diverge(check,names.missingRight,path,a.kind,"absent"));continue;}
  if(a.kind!==b.kind){rows.push(diverge(check,names.kind,path,a.kind,b.kind));continue;}
  if(a.kind==="symlink"){if(a.link!==b.link)rows.push(diverge(check,names.link,path,a.link,b.link));continue;}
  if(a.kind!=="file")continue;
  const bits=mask?.mask??0o777;
  if((a.mode&bits)!==(b.mode&bits))rows.push(diverge(check,names.mode,path,(a.mode&bits).toString(8),(b.mode&bits).toString(8)));
  else if(mask&&a.mode!==b.mode)count(applied,mask.id);
  if(a.sha256===b.sha256)continue;
  const explained=Object.keys(a.normalized??{}).find(rule=>a.normalized![rule]!==undefined&&a.normalized![rule]===b.normalized?.[rule]);
  if(explained)count(applied,explained);
  else rows.push(diverge(check,names.bytes,path,`${a.size??"?"} ${a.sha256?.slice(0,12)}`,`${b.size??"?"} ${b.sha256?.slice(0,12)}`));
 }
 if(mode==="tools"){
  for(const task of sorted([...Object.keys(nx.roots),...Object.keys(bazel.roots)])){
   const a=sorted(nx.roots[task]??[]),b=sorted(bazel.roots[task]??[]);
   if(JSON.stringify(a)!==JSON.stringify(b))rows.push(diverge("outputs","output-root-differs",task,a.join(","),b.join(",")));
  }
 }
 return order(rows);
}

/** 🚥️ Compares exit codes, termination reasons, failed tasks and the status of every test task. */
export function compareParityResultsV1(nx:ParityRunV1|null,bazel:ParityRunV1|null,expectExit:number):ParityDivergenceV1[]{
 if(!nx||!bazel)return [diverge("results","run-unavailable",!nx&&!bazel?"nx,bazel":!nx?"nx":"bazel")];
 const rows:ParityDivergenceV1[]=[];
 if(nx.exit!==bazel.exit)rows.push(diverge("results","exit-differs","exit",String(nx.exit),String(bazel.exit)));
 if(nx.exit!==expectExit)rows.push(diverge("results","exit-unexpected",`nx expected ${expectExit}`,String(nx.exit)));
 if(bazel.exit!==expectExit)rows.push(diverge("results","exit-unexpected",`bazel expected ${expectExit}`,undefined,String(bazel.exit)));
 if(nx.reason!==bazel.reason)rows.push(diverge("results","reason-differs","reason",nx.reason,bazel.reason));
 const failed=[diffOf(nx.failedTasks,bazel.failedTasks),diffOf(bazel.failedTasks,nx.failedTasks)];
 if(failed[0]!.length||failed[1]!.length)rows.push(diverge("results","failed-tasks-differ","failedTasks",sorted(nx.failedTasks).join(","),sorted(bazel.failedTasks).join(",")));
 for(const task of sorted([...Object.keys(nx.tests),...Object.keys(bazel.tests)])){
  const a=nx.tests[task]??"no-status",b=bazel.tests[task]??"no-status";
  if(a!==b)rows.push(diverge("results","test-status-differs",task,a,b));
 }
 return order(rows);
}

/** ⚖️ Compares what Nx hashes with what Bazel's spawns received, task by task, with the documented classes separated. */
export function compareParityInputsV1(nx:Readonly<Record<string,ParityNxInputsV1>>|null,bazel:Readonly<Record<string,ParitySpawnV1>>|null,rules:ParityRulesV1,applied?:Applied):ParityDivergenceV1[]{
 if(!nx||!bazel)return [diverge("inputs","inputs-unavailable",!nx&&!bazel?"nx,bazel":!nx?"nx":"bazel")];
 const rows:ParityDivergenceV1[]=[],strip=rules.rules.filter(rule=>rule.kind==="strip-prefix"),ignoreNx=rules.rules.filter(rule=>rule.kind==="ignore-glob"&&rule.surface==="input-nx"),ignoreBazel=rules.rules.filter(rule=>rule.kind==="ignore-glob"&&rule.surface==="input-bazel"),ambient=rules.rules.filter(rule=>rule.kind==="ambient-env"),covered=rules.rules.filter(rule=>rule.kind==="covered-by");
 const ignored=(list:readonly ParityRuleV1[],path:string):boolean=>{const rule=list.find(row=>matches(row.glob!,path));if(rule){count(applied,rule.id);return true;}return false;};
 for(const task of sorted([...Object.keys(nx),...Object.keys(bazel)])){
  const a=nx[task],b=bazel[task];
  if(!a||!b){rows.push(diverge("inputs","inputs-unavailable",task,a?"present":"absent",b?"present":"absent"));continue;}
  const mapped=new Set<string>();
  for(const path of b.inputs){
   if(ignored(ignoreBazel,path))continue;
   const rule=strip.find(row=>path.startsWith(row.prefix!));
   if(rule)count(applied,rule.id);
   const stripped=rule?path.slice(rule.prefix!.length):path;
   if(!ignored(ignoreBazel,stripped))mapped.add(stripped);
  }
  const files=new Set(a.files.filter(path=>!ignored(ignoreNx,path)));
  for(const path of diffOf(files,mapped))rows.push(diverge("inputs","input-file-only-nx",`${task} ${path}`));
  for(const path of diffOf(mapped,files))rows.push(diverge("inputs","input-file-only-bazel",`${task} ${path}`));
  for(const name of diffOf(a.environment,b.env))rows.push(diverge("inputs","env-hashed-not-visible",`${task} ${name}`));
  for(const name of diffOf(b.env,a.environment)){
   const rule=ambient.find(row=>row.names!.some(pattern=>matches(pattern,name)));
   if(rule)count(applied,rule.id);else rows.push(diverge("inputs","env-visible-not-hashed",`${task} ${name}`));
  }
  for(const[category,values]of [["external",a.external],["runtime",a.runtime]] as const){
   if(!values.length)continue;
   const rule=covered.find(row=>row.category===category&&b.inputs.some(path=>row.anyOf!.some(pattern=>matches(pattern,path))));
   if(rule)count(applied,rule.id);else rows.push(diverge("inputs",category==="external"?"external-uncovered":"runtime-uncovered",task,sorted(values).join(",")));
  }
 }
 return order(rows);
}

const status=(rows:readonly ParityDivergenceV1[],unavailable:readonly string[]):ParityCheckReportV1["status"]=>rows.some(row=>unavailable.includes(row.code))?"unavailable":rows.length?"diverged":"equal";

/** 🪞️ Compares everything one sample's observations carry, check by check, and reports what was applied. */
export function compareParitySampleV1(sample:ParityResolvedSampleV1,observations:Readonly<{nx:Partial<Record<ParityPhaseV1,ParityObservationV1>>;bazel:Partial<Record<ParityPhaseV1,ParityObservationV1>>}>,rules:ParityRulesV1,applied?:Applied):ParitySampleReportV1{
 const checks:ParityCheckReportV1[]=[],errors:ParityCheckReportV1[]=[];
 for(const phase of sample.phases){
  const nx=observations.nx[phase],bazel=observations.bazel[phase];
  for(const tool of ["nx","bazel"] as const){
   const observation=observations[tool][phase];
   if(!observation||observation.errors.length)errors.push({check:"observation",phase,status:"unavailable",divergences:(observation?.errors??["observation missing"]).map(message=>diverge("observation","observation-error",tool,message))});
  }
  const add=(check:ParityCheckV1,compare:()=>ParityDivergenceV1[],unavailable:string[]=[])=>{if(!sample.checks.includes(check))return;const rows=compare();checks.push({check,phase,status:status(rows,unavailable),divergences:rows});};
  if(phase==="fresh"){
   add("graph",()=>compareParityGraphV1(nx?.plan??null,bazel?.plan??null),["plan-unavailable"]);
   add("closure",()=>compareParityClosureV1(bazel?.plan??null,bazel?.closure??null),["closure-unavailable"]);
  }
  const declared=Boolean(nx?.outputs&&Object.keys(nx.outputs.roots).length||bazel?.outputs&&Object.keys(bazel.outputs.roots).length);
  if(sample.checks.includes("outputs")){
   const rows=declared?compareParityOutputsV1(nx?.outputs??null,bazel?.outputs??null,rules,applied):[];
   checks.push({check:"outputs",phase,status:declared?status(rows,[]):"not-applicable",divergences:rows});
  }
  add("results",()=>compareParityResultsV1(nx?.run??null,bazel?.run??null,sample.expectExit),["run-unavailable"]);
  if(phase==="fresh")add("inputs",()=>compareParityInputsV1(nx?.inputs??null,bazel?.spawns??null,rules,applied),["inputs-unavailable"]);
 }
 if(sample.checks.includes("restore")&&sample.phases.includes("fresh")&&sample.phases.includes("restored"))
  for(const tool of ["nx","bazel"] as const){
   const fresh=observations[tool].fresh?.outputs??null,restored=observations[tool].restored?.outputs??null,rows=fresh&&restored?compareParityOutputsV1(fresh,restored,rules,applied,"restore").map(row=>({...row,subject:`${tool} ${row.subject}`})):[];
   checks.push({check:"restore",phase:"restored",status:fresh&&restored?status(rows,[]):"unavailable",divergences:rows});
  }
 const all=[...errors,...checks],failed=errors.length>0,diverged=checks.some(row=>row.status==="diverged"||row.status==="unavailable");
 return {sample:sample.id,area:sample.area,target:sample.target,status:failed?"error":diverged?"diverged":"equal",checks:all};
}

/** 📈️ Summarizes sample reports and the documented rules each one used. */
export function summarizeParityV1(samples:readonly ParitySampleReportV1[],rules:ParityRulesV1,applied:Applied):ParityReportV1{
 const divergences=samples.reduce((sum,row)=>sum+row.checks.reduce((inner,check)=>inner+check.divergences.length,0),0),total=(state:string)=>samples.filter(row=>row.status===state).length;
 const report:ParityReportV1={version:1,samples,totals:{samples:samples.length,equal:total("equal"),diverged:total("diverged"),error:total("error"),divergences},rules:rules.rules.map(rule=>({id:rule.id,applied:applied.get(rule.id)??0}))};
 closed("Report",report,"Closed parity report required");
 return report;
}

/** 🗺️ Resolves the declared output patterns of one task to workspace-relative paths; unresolved placeholders are refused. */
export function resolveParityOutputPatternsV1(patterns:readonly string[],task:Readonly<{project:string;root:string;execution:Readonly<Record<string,unknown>>}>,environment:Readonly<Record<string,string|undefined>>):string[]{
 const root=task.root==="."?"":task.root.replace(/\/+$/u,""),resolved:string[]=[];
 for(const pattern of patterns){
  const text=pattern.replaceAll("{workspaceRoot}/","").replaceAll("{workspaceRoot}","").replaceAll("{projectRoot}",root).replaceAll("{projectName}",task.project).replace(/\{options\.([^}]+)\}/gu,(_all,key:string)=>{
   let node:unknown=task.execution;for(const part of key.split("."))node=node!==null&&typeof node==="object"?(node as Record<string,unknown>)[part]:undefined;
   if(node===undefined&&key.startsWith("env."))node=environment[key.slice(4)];
   if(typeof node!=="string"&&typeof node!=="number"&&typeof node!=="boolean")throw Error(`Parity output requires option ${key}: ${task.project}`);
   return String(node);
  });
  const parts:string[]=[];
  for(const part of text.replace(/^\.\//u,"").replace(/^\/+/u,"").split("/")){if(part===".."){if(!parts.length)throw Error(`Parity output leaves the workspace: ${pattern}`);parts.pop();}else if(part!==""&&part!==".")parts.push(part);}
  if(!parts.length)throw Error(`Parity output names the workspace itself: ${pattern}`);
  resolved.push(parts.join("/"));
 }
 return sorted(resolved).filter((path,_index,all)=>!all.some(parent=>parent!==path&&path.startsWith(parent+"/")));
}

/** 🔌️ What a verification needs from the outside world; the live implementation runs the tools, tests substitute recorded answers. */
export interface ParityRunnerV1{
 plan(sample:ParityResolvedSampleV1,tool:ParityToolV1):Promise<Readonly<{plan:ParityPlanV1|null;errors:readonly string[]}>>;
 roots(sample:ParityResolvedSampleV1,plans:readonly ParityPlanV1[]):Promise<Readonly<{roots:Readonly<Record<string,readonly string[]>>;errors:readonly string[]}>>;
 observe(sample:ParityResolvedSampleV1,tool:ParityToolV1,phase:ParityPhaseV1,plan:ParityPlanV1|null,roots:Readonly<Record<string,readonly string[]>>):Promise<ParityObservationV1>;
}

/** 🎼️ Verifies every sample: plans first, then each phase of each tool, then the comparison. Stops between steps when the caller cancels. */
export async function verifyParityV1(runner:ParityRunnerV1,samples:ParitySamplesV1,rules:ParityRulesV1,selected:readonly ParitySampleV1[],hooks:Readonly<{progress?:(label:string)=>Promise<void>;check?:()=>void;plansOnly?:boolean}>={}):Promise<ParityReportV1>{
 const applied:Applied=new Map(),reports:ParitySampleReportV1[]=[],progress=hooks.progress??(async()=>{}),check=hooks.check??(()=>{});
 for(const row of selected){
  check();
  const sample=resolveParitySampleV1(samples,row),observations:{nx:Partial<Record<ParityPhaseV1,ParityObservationV1>>;bazel:Partial<Record<ParityPhaseV1,ParityObservationV1>>}={nx:{},bazel:{}},plans:Partial<Record<ParityToolV1,ParityPlanV1|null>>={},errors:Partial<Record<ParityToolV1,readonly string[]>>={};
  for(const tool of ["nx","bazel"] as const){await progress(`${sample.id}:${tool}:plan`);check();const planned=await runner.plan(sample,tool);plans[tool]=planned.plan;errors[tool]=planned.errors;}
  const known=[plans.nx,plans.bazel].filter((plan):plan is ParityPlanV1=>plan!==null&&plan!==undefined);
  if(hooks.plansOnly){
   for(const tool of ["nx","bazel"] as const)observations[tool].fresh={version:1,sample:sample.id,tool,phase:"fresh",plan:plans[tool]??null,run:null,outputs:null,inputs:null,spawns:null,closure:null,errors:errors[tool]??[]};
   reports.push(compareParitySampleV1({...sample,phases:["fresh"],checks:["graph"]},observations,rules,applied));
   continue;
  }
  const located=known.length?await runner.roots(sample,known):{roots:{},errors:["no plan to locate outputs"]};
  for(const phase of sample.phases)for(const tool of ["nx","bazel"] as const){
   await progress(`${sample.id}:${tool}:${phase}`);check();
   const observation=await runner.observe(sample,tool,phase,plans[tool]??null,located.roots);
   observations[tool][phase]={...observation,errors:[...(phase==="fresh"?errors[tool]??[]:[]),...(phase==="fresh"?located.errors:[]),...observation.errors]};
  }
  reports.push(compareParitySampleV1(sample,observations,rules,applied));
 }
 return summarizeParityV1(reports,rules,applied);
}
