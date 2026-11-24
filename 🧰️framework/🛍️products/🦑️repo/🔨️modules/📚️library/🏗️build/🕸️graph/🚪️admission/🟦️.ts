import {buildOptionsV1,declaredBuildProjectSelectorsV1,literalBuildProjectSelectorsV1,selectBuildNamesV1,wholeBuildReadReasonV1} from "../../🧭️selection/📥️options/🟦️.ts";
import {buildBooleanOptionV1} from "../../🧭️selection/🟦️.ts";
import {createHash} from "node:crypto";
import {refuseBuildV1} from "../../📖️texts/🟦️.ts";
import {isOwnerFailureV1,joinOwnerFailuresV1,type OwnerFailureV1} from "../📥️load/🛟️owner-isolation/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {createDiscoveryAheadV1} from "./🔮️ahead/🟦️.ts";
import selectionSchema from "../../🧬️schema/🚪️selection.json" with {type:"json"};
import registrySchema from "../../🧬️schema/📋️owners.json" with {type:"json"};
import admissionSchema from "../../🧬️schema/🚪️admission.json" with {type:"json"};
import projectSchema from "../../🧬️schema/📋️project/🔣️.json" with {type:"json"};

/** 🎯️ Declares exact task admission or intentional complete workspace ownership; `excludeDependencies` admits exactly the owners of the named tasks, so the task resolves from its own manifest and never reads or infers a dependency. */
export type BuildGraphSelectionV1={kind:"all"}|{kind:"owners";projects:string[];exclude:string[];targets?:string[]}|{kind:"tasks";tasks:{project:string;target:string;configuration?:string}[];excludeDependencies?:true};
/** 🪪️ Resolves authored task and package names without evaluating independent owners. */
export interface BuildOwnerRegistryV1{version:1;owners:{name:string;root:string;manifest:string;packages:string[];nativeRoots?:string[]}[];catalogProviders:{ownerRoot:string;sourcePath:string;consumerRoots:string[]}[]}
/** 🔏️ Carries exact original owner observations and source-owned dependency identities; `files` admit the owners that hold them, `observedFiles` are file facts of foreign owners that are members and invalidate but admit nothing. */
export interface BuildOwnerAdmissionFactsV1{project:{name:string;targets:Record<string,{dependsOn?:unknown[]}>};files:string[];observedFiles?:string[];dependencies:string[];dependencyInputs?:Record<string,string[]>;generatedDependencies?:Record<string,string[]>}
/** 🔬️ Why a closure has the size it has: the admitted owners, the pass, the owners by the edge kind that first admitted them (`selection`, `task`, `dependency`, `file`, `input`) and the owners that admitted the most others, so a closure that is far larger than the target needs names its hub. */
export interface OwnerClosureTraceV1{owners:number;pass:number;kinds:Record<string,number>;top:{owner:string;admitted:number}[]}
/** 🚪️ Closes manifest, command-source and prerequisite ownership before source assembly. */
export interface BuildOwnerAdmissionV1{owners:string[];files:string[];observedFiles?:string[];targets:Record<string,string[]>;inputs:Record<string,string[]>;catalogProviders:{ownerRoot:string;sourcePath:string}[];generatedDependencies?:Record<string,Record<string,string[]>>}
const contains=(root:string,path:string)=>root==="."||path===root||path.startsWith(root+"/");
const sourceOwners=(registry:BuildOwnerRegistryV1)=>registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>({root,name:owner.name}))).sort((a,b)=>b.root.length-a.root.length||a.name.localeCompare(b.name));

/** ✅️ Rejects open selections and canonicalizes the finite task order. */
export function readBuildGraphSelectionV1(value:unknown):BuildGraphSelectionV1{
 if(validateJsonSchemaSubset(selectionSchema,value).length)throw Error("Closed build graph selection required");
 const selection=value as BuildGraphSelectionV1;return selection.kind==="all"?{kind:"all"}:selection.kind==="owners"?{kind:"owners",projects:selection.projects.some(value=>value.startsWith("!"))?[...selection.projects]:[...selection.projects].sort(),exclude:[...selection.exclude].sort(),...(selection.targets?{targets:[...selection.targets].sort()}:{})}:{kind:"tasks",tasks:selection.tasks.map(task=>({...task})).sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b))),...(selection.excludeDependencies?{excludeDependencies:true as const}:{})};
}

/** 📋️ Validates only declarations and identity collisions, never independent physical inputs. */
export function readBuildOwnerRegistryV1(value:unknown):BuildOwnerRegistryV1{
 if(validateJsonSchemaSubset(registrySchema,value).length)throw Error("Canonical build owner registry required");
 const registry=value as BuildOwnerRegistryV1,names=new Set<string>(),roots=new Set<string>(),packages=new Set<string>();
 for(const owner of registry.owners){if(names.has(owner.name)||roots.has(owner.root)||!contains(owner.root,owner.manifest)||!["📋️project.json","Cargo.toml"].some(name=>owner.manifest===(owner.root==="."?name:owner.root+"/"+name)))throw Error("Canonical build owner identity collision");names.add(owner.name);for(const root of[owner.root,...owner.nativeRoots??[]]){if(roots.has(root))throw Error("Canonical native owner collision");roots.add(root);}for(const name of owner.packages){if(packages.has(name))throw Error("Canonical package owner collision");packages.add(name);}}
 return structuredClone(registry);
}

/** 🧭️ Resolves the deepest explicit physical owner, including declared native roots. */
export function buildSourceOwnerV1(registry:BuildOwnerRegistryV1,path:string):string|undefined{
 return sourceOwners(registry).find(owner=>contains(owner.root,path))?.name;
}

/** 🔗️ Keeps a relative file's workspace ancestry separate from an actual peer project dependency. */
export function buildRelativeImportProjectV1(owners:readonly {name:string;root:string}[],path:string):string|undefined{
 let selected:{name:string;root:string}|undefined;for(const owner of owners)if(owner.root!==admissionSchema.$defs.relativeImportOwnership.const.workspaceRoot&&contains(owner.root,path)&&(!selected||owner.root.length>selected.root.length||owner.root.length===selected.root.length&&owner.name.localeCompare(selected.name)<0))selected=owner;return selected?.name;
}

/** 🧭️ {@link buildSourceOwnerV1} for many paths against one registry: the owner of a path is found by looking up the path and each of its ancestors in an index of the roots (a constant number of steps per path) instead of scanning every owner root, which cost the files of the closure times the owners of the workspace; the deepest root wins, equal lengths keep the smallest name, the workspace root `.` owns what no deeper root owns. */
export function createSourceOwnerResolverV1(registry:BuildOwnerRegistryV1):(path:string)=>string|undefined{
 const ranked=new Map<string,{name:string;at:number}>();
 sourceOwners(registry).forEach((owner,at)=>{if(!ranked.has(owner.root))ranked.set(owner.root,{name:owner.name,at});});
 const workspace=ranked.get(".");
 return path=>{let best=workspace;for(let end=path.length;end>0;end=path.lastIndexOf("/",end-1)){const hit=ranked.get(end===path.length?path:path.slice(0,end));if(hit!==undefined&&(best===undefined||hit.at<best.at))best=hit;}return best?.name;};
}

/** 🗂️ {@link buildRelativeImportProjectV1} for many paths against one owner list: the deepest owner root that is the path or one of its ancestors is found by looking up the ancestors of the path (a constant number of steps per path) instead of scanning every owner; equal roots keep the smallest name, the workspace root owns nothing. */
export function createRelativeImportResolverV1(owners:readonly {name:string;root:string}[]):(path:string)=>string|undefined{
 const workspaceRoot=admissionSchema.$defs.relativeImportOwnership.const.workspaceRoot,named=new Map<string,string>();
 for(const owner of owners){if(owner.root===workspaceRoot)continue;const known=named.get(owner.root);if(known===undefined||owner.name.localeCompare(known)<0)named.set(owner.root,owner.name);}
 return path=>{for(let end=path.length;end>0;end=path.lastIndexOf("/",end-1)){const name=named.get(end===path.length?path:path.slice(0,end));if(name!==undefined)return name;}return undefined;};
}

/** 🗃️ The canonical name, package, root and relative file owner lookups, built once per registry. */
const REGISTRY_INDEXES_V1=new WeakMap<object,{byName:ReadonlyMap<string,BuildOwnerRegistryV1["owners"][number]>;byPackage:Map<string,string>;byRoot:Map<string,string>;relativeProject:(path:string)=>string|undefined}>();
export function registryOwnerIndexesV1(registry:BuildOwnerRegistryV1){
 let known=REGISTRY_INDEXES_V1.get(registry);
 if(known===undefined){const rows=registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>({name:owner.name,root})));known={byName:new Map(registry.owners.map(owner=>[owner.name,owner])),byPackage:new Map(registry.owners.flatMap(owner=>owner.packages.map(name=>[name,owner.name] as const))),byRoot:new Map(rows.map(row=>[row.root,row.name] as const)),relativeProject:createRelativeImportResolverV1(rows)};REGISTRY_INDEXES_V1.set(registry,known);}
 return known;
}

/** 📥️ Applies authored roles to matching named inputs while retaining explicit compilation entries. */
export function buildInputSourceRoleV1(selected:boolean,declared=false,observations:readonly unknown[]=[]):"compile"|"freshness"{
 const roles=admissionSchema.$defs.sourceInputObservation.const,allowed:readonly unknown[]=admissionSchema.$defs.sourceInputRole.enum;if(observations.some(value=>value!==undefined&&!allowed.includes(value)))throw Error("Closed target input observation role required");return (declared?roles.declaredEntry:selected&&(!observations.length||observations.some(value=>value!==roles.namedInputOnly))?roles.selectedTask:roles.namedInputOnly) as "compile"|"freshness";
}

/** 🚀️ Names physical runtime modules independently of matched freshness-only inputs. */
export function readBuildRuntimeSourceEntriesV1(value:unknown):readonly string[]{
 if(value===undefined)return [];if(validateJsonSchemaSubset(projectSchema.$defs.RuntimeSourceEntries,value).length)throw Error("Closed runtime Source entries required");return Object.freeze([...(value as string[])]);
}

/** 📄️ Admits exact data resources without granting their imports executable authority. */
export function readBuildSourceResourceEntriesV1(value:unknown):readonly string[]{
 if(value===undefined)return [];if(validateJsonSchemaSubset(projectSchema.$defs.SourceResourceEntries,value).length)throw Error("Closed Source resource entries required");return Object.freeze([...(value as string[])]);
}

/** 🪜️ Retains the strongest actual demand when a runtime Source later becomes a compiler input. */
export function buildSourceDemandV1(previous:unknown,requested:unknown):"runtime"|"compile"{
 const roles:readonly unknown[]=admissionSchema.$defs.sourceDemandRole.enum,priority=admissionSchema.$defs.sourceDemand.const.priority;if(!roles.includes(requested)||previous!==undefined&&!roles.includes(previous))throw Error("Closed source demand role required");const role=requested as keyof typeof priority,prior=previous as keyof typeof priority|undefined;return prior!==undefined&&priority[prior]>priority[role]?prior:role;
}

/** 🌱️ Distinguishes requested native Source facts from selected compiler preparation. */
export function buildNativeSourceObservationV1(value:unknown):"none"|"source"|"compile"{if(validateJsonSchemaSubset({$defs:admissionSchema.$defs,$ref:"#/$defs/nativeSourceObservation"},value).length)throw Error("Closed native source observation required");const row=value as Readonly<{selectedSource:boolean;selectedNative:boolean;requestedInputs:boolean;dependencyInputs:boolean;inputRole:"freshness"|"compile"}>;return row.selectedNative?"compile":row.inputRole==="compile"&&(row.requestedInputs||row.dependencyInputs)?"source":"none";}

/** 🏗️ Selects contract terminals through actual normalized own input references without reading Source. */
export function buildSourceContractGroupsV1(project:{name:string;targets?:Record<string,{inputs?:readonly unknown[]}>;namedInputs?:Record<string,readonly unknown[]>;metadata?:{sourceInputs?:Record<string,unknown>}},defaults:Record<string,readonly unknown[]>,requested:readonly unknown[]=[],groups?:ReadonlySet<string>):string[]{
 const policy=admissionSchema.$defs.sourceContractScope.const,contracts=project.metadata?.sourceInputs??{},named={...defaults,...project.namedInputs},selected=new Set<string>(),active=new Set<string>(),complete=new Set<string>();let references=0,literals=0;
 if(Object.keys(contracts).some(name=>(policy.reservedGroups as readonly string[]).includes(name)))throw Error("Reserved source input contract group");
 const visit=(values:readonly unknown[],depth=0)=>{if(!Array.isArray(values)||depth>policy.maximumDepth)throw Error("Finite selected source input closure required");for(const value of values){if(typeof value==="string"&&(value.includes("{workspaceRoot}")||value.includes("{projectRoot}")||value.startsWith("^"))){if(++literals>policy.maximumLiterals)throw Error("Finite selected source input literals required");continue;}if(++references>policy.maximumReferences)throw Error("Finite selected source input closure required");if(typeof value!=="string"){
  if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Closed selected source input required");const row=value as Record<string,unknown>,keys=Object.keys(row);
  if(typeof row.input==="string"&&row.input&&keys.every(key=>["input","projects","dependencies"].includes(key))){if(row.projects!==undefined){if(!Array.isArray(row.projects)||row.projects.some(name=>typeof name!=="string"||!name)||row.dependencies!==undefined)throw Error("Canonical named input projects required");if(row.projects.includes(project.name))visit([row.input],depth+1);}else if(row.dependencies===undefined||row.dependencies===false)visit([row.input],depth+1);else if(row.dependencies!==true)throw Error("Closed named input dependency required");}
  else if(keys.length===1&&typeof row.fileset==="string"){if(groups!==undefined)visit([row.fileset],depth+1);}
  else if(keys.length===1&&typeof(row.env??row.runtime)==="string"||keys.length===1&&Array.isArray(row.externalDependencies)&&row.externalDependencies.every(name=>typeof name==="string")||typeof row.dependentTasksOutputFiles==="string"&&keys.every(key=>["dependentTasksOutputFiles","transitive"].includes(key))&&(row.transitive===undefined||typeof row.transitive==="boolean"))continue;
  else throw Error("Unsupported selected source input object");continue;
 }if(!value)throw Error("Named source input required");if(value.startsWith("^")){if(value.length===1)throw Error("Named dependency input required");continue;}if(value.includes("{workspaceRoot}")||value.includes("{projectRoot}"))continue;const contract=Object.hasOwn(contracts,value);if(groups?.has(value)??contract)selected.add(value);if(complete.has(value))continue;if(active.has(value))throw Error("Selected named source input cycle: "+value);const inputs=Object.hasOwn(named,value)?named[value]:undefined;if(inputs===undefined){if(contract)continue;throw refuseBuildV1("invalid","graph.unknown-source-input",{owner:project.name,input:value});}active.add(value);visit(inputs,depth+1);active.delete(value);complete.add(value);}};
 for(const target of Object.values(project.targets??{}))visit(target.inputs??policy.defaultInputs);visit(requested);return[...selected].sort();
}

/** 🔑️ Names the complete immutable selection independently of whole graph state and of the owner registry: what a read asked of the registry travels in the records it derived, so an edit of one registry row never gives a scope a new identity.
 * @see ../🧾️registry-reads/🟨️.mjs */
export function buildGraphScopeKeyV1(selection:BuildGraphSelectionV1):string{
 return createHash("sha256").update(JSON.stringify(readBuildGraphSelectionV1(selection))).digest("hex");
}

/** 🚪️ Admits only a closed owner admission. */
export function readBuildOwnerAdmissionV1(value:unknown):BuildOwnerAdmissionV1{
 if(validateJsonSchemaSubset(admissionSchema,value).length)throw Error("Closed selected build owner admission required");
 return value as BuildOwnerAdmissionV1;
}

/** 📬️ Orders dirty owners by their eligible pass and the same lexical order as the admission roster. */
function ownerAdmissionFrontierV1(){
 type Entry={name:string;pass:number};
 const rows:Entry[]=[],queued=new Set<string>();
 const precedes=(a:Entry,b:Entry)=>a.pass<b.pass||a.pass===b.pass&&a.name<b.name;
 return {
  add(name:string,pass:number){
   if(queued.has(name))return;
   queued.add(name);
   const row={name,pass};
   let index=rows.length;
   while(index>0){
    const parent=(index-1)>>1;
    if(!precedes(row,rows[parent]!))break;
    rows[index]=rows[parent]!;index=parent;
   }
   rows[index]=row;
  },
  take(pass:number):string|undefined{
   const first=rows[0];
   if(first?.pass!==pass)return undefined;
   const last=rows.pop()!;
   queued.delete(first.name);
   if(rows.length){
    let index=0,left=1;
    while(left<rows.length){
     const right=left+1,child=right<rows.length&&precedes(rows[right]!,rows[left]!)?right:left;
     if(!precedes(rows[child]!,last))break;
     rows[index]=rows[child]!;index=child;left=2*index+1;
    }
    rows[index]=last;
   }
   return first.name;
  },
  peek(pass:number,limit:number):string[]{return rows.filter(row=>row.pass===pass).map(row=>row.name).sort((a,b)=>a<b?-1:a>b?1:0).slice(0,limit);},
 };
}

/** 🔮️ One question of the closure to the observation port: an owner with the targets and inputs asked of it. */
export type OwnerObservationRequestV1={readonly owner:BuildOwnerRegistryV1["owners"][number];readonly targets:readonly string[];readonly inputs:readonly string[]};

/** 🔮️ How many owners behind the one in work the closure announces to the observation port (`lookahead`), which may start them early. */
const OWNER_LOOKAHEAD_V1=16;

/** 🧩️ Converges exact task prerequisites and source owners through a caller-owned observation port; an owner reached only through `file` edges is admitted ownership-only (its manifest and the imported files are members, it is never observed) until a task, input or dependency edge needs it; the named failure of an owner (see `observeOwnerWithRetryV1`) does not end the walk: every other owner is still observed and kept, the owner is left out when none of its tasks is in the task closure of the selection (a mere project dependency, or a `^` task of a target it does not declare as `declares` answers) and `report` names it, and a needed owner ends the closure with one named failure listing every needed owner.
 * @see ../📥️load/🛟️owner-isolation/🟦️.ts */
export async function closeBuildOwnerAdmissionV1(selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,inspect:(owner:BuildOwnerRegistryV1["owners"][number],targets:readonly string[],inputs:readonly string[])=>Promise<BuildOwnerAdmissionFactsV1>,report?:(failure:{owner:string;error:OwnerFailureV1;needed:boolean})=>void,declares:(owner:string,target:string)=>boolean|undefined=()=>true,trace?:(summary:OwnerClosureTraceV1)=>void,width?:()=>number):Promise<BuildOwnerAdmissionV1>{
 selection=readBuildGraphSelectionV1(selection);registry=readBuildOwnerRegistryV1(registry);
 const direct=selection.kind==="tasks"&&selection.excludeDependencies===true,byName=new Map(registry.owners.map(owner=>[owner.name,owner])),sourceOwner=createSourceOwnerResolverV1(registry),admitted=new Map<string,number>(),targets=new Map<string,Set<string>>(),inputs=new Map<string,Set<string>>(),optionalTasks=new Set<string>(),files=new Set<string>(),admitting=new Set<string>(),observed=new Set<string>(),providers=new Map<string,{ownerRoot:string;sourcePath:string}>(),revisions=new Map<string,string>(),facts=new Map<string,BuildOwnerAdmissionFactsV1>(),failures=new Map<string,OwnerFailureV1>(),named=new Set<string>();
 const attempt=async<T>(name:string,revision:string|undefined,run:()=>Promise<T>):Promise<T|undefined>=>{try{const value=await run();failures.delete(name);return value;}catch(error){if(!isOwnerFailureV1(error))throw error;failures.set(name,error);if(revision!==undefined)revisions.set(name,revision);return undefined;}};
 const flights=createDiscoveryAheadV1<OwnerObservationRequestV1,BuildOwnerAdmissionFactsV1>({key:request=>[request.owner.name,request.targets.join("\u0001"),request.inputs.join("\u0001")].join("\0"),width:()=>width?.()??0,start:async({owner,targets,inputs})=>{const value=await inspect(owner,targets,inputs);if(validateJsonSchemaSubset({$defs:admissionSchema.$defs,$ref:"#/$defs/ownerFacts"},value).length)throw Error("Closed build owner observations required");return value;}});
 const observe=(owner:BuildOwnerRegistryV1["owners"][number],targets:readonly string[],inputs:readonly string[])=>flights.take({owner,targets,inputs});
 const frontier=ownerAdmissionFrontierV1();
 const requestOf=(next:string,known:BuildOwnerAdmissionFactsV1|undefined):OwnerObservationRequestV1|undefined=>{
  if(ownership.has(next))return undefined;
  const chosen=targets.get(next),open=(target:string)=>optionalTasks.has(next+"\0"+target),owner=byName.get(next)!;
  if(known===undefined&&[...chosen??[]].some(open))return{owner,targets:[],inputs:[]};
  const wanted=[...chosen??[]].filter(target=>!open(target)||known?.project.targets[target]).sort(),requested=[...inputs.get(next)??[]].sort();
  return revisions.get(next)===JSON.stringify([wanted,requested])||!(wanted.length||requested.length||known===undefined)?undefined:{owner,targets:wanted,inputs:requested};
 };
 const follow=(request:OwnerObservationRequestV1,known:BuildOwnerAdmissionFactsV1):OwnerObservationRequestV1[]=>{
  if(request.targets.length||request.inputs.length||facts.has(request.owner.name))return[];
  const next=requestOf(request.owner.name,known);
  return next!==undefined&&(next.targets.length||next.inputs.length)?[next]:[];
 };
 let pass=-1,cursor:string|undefined;
 const dirty=(name:string)=>frontier.add(name,cursor!==undefined&&admitted.get(name)!<=pass&&cursor<name?pass:pass+1);
 const origin=new Map<string,{by:string;kind:string}>(),ownership=new Set<string>();let nextTrace=250;
 const summary=():OwnerClosureTraceV1=>{const kinds:Record<string,number>={},by=new Map<string,number>();for(const row of origin.values()){kinds[row.kind]=(kinds[row.kind]??0)+1;by.set(row.by,(by.get(row.by)??0)+1);}return {owners:admitted.size,pass,kinds,top:[...by].sort((a,b)=>b[1]-a[1]||a[0].localeCompare(b[0])).slice(0,8).map(([owner,admitted])=>({owner,admitted}))};};
 const admit=(name:string,kind="selection")=>{
  if(!byName.has(name))throw Error("Unknown canonical build owner: "+name);
  if(admitted.has(name)){if(kind!=="file"&&ownership.delete(name))dirty(name);return;}
  admitted.set(name,pass+1);origin.set(name,{by:cursor??"selection",kind});if(kind==="file")ownership.add(name);frontier.add(name,pass+1);
 };
 const task=(name:string,target:string,optional=false)=>{
  admit(name,cursor===undefined?"selection":optional?"caret":"task");
  const selected=targets.get(name)??new Set<string>(),key=name+"\0"+target,had=selected.has(target);
  if(optional&&!had)optionalTasks.add(key);
  const required=!optional&&optionalTasks.delete(key);
  selected.add(target);targets.set(name,selected);
  if(!had||required)dirty(name);
 };
 const requireInputs=(name:string,groups:readonly string[])=>{
  admit(name,"input");
  const selected=inputs.get(name)??new Set<string>(),previous=selected.size;
  for(const input of groups)selected.add(input);
  inputs.set(name,selected);
  if(selected.size!==previous)dirty(name);
 };
 if(selection.kind==="all")for(const owner of registry.owners)admit(owner.name);else if(selection.kind==="owners")for(const name of selectBuildNamesV1(registry.owners,selection.projects,selection.exclude)){admit(name);named.add(name);for(const target of selection.targets??[])task(name,target,true);}else for(const value of selection.tasks)task(value.project,value.target);
 for(pass=0;pass<=registry.owners.length+65536;pass++){
  let changed=false;
  for(let name=frontier.take(pass);name!==undefined;name=frontier.take(pass)){
   cursor=name;
   const owner=byName.get(name)!,selected=targets.get(name),optional=(target:string)=>optionalTasks.has(name+"\0"+target);
   if((width?.()??0)>0)flights.ahead(frontier.peek(pass,OWNER_LOOKAHEAD_V1).map(next=>requestOf(next,facts.get(next))).filter((request):request is OwnerObservationRequestV1=>request!==undefined),follow);
   if(ownership.has(name)){files.add(owner.manifest);continue;}
   let prior=facts.get(name);
   if(!prior&&[...selected??[]].some(optional)){
    prior=await attempt(name,undefined,()=>observe(owner,[],[]));
    if(prior===undefined)continue;
    if(prior.project.name!==name)throw Error("Admitted canonical build owner differs: "+owner.manifest);
   }
   for(const target of selected??[])if(optional(target)&&!prior?.project.targets[target])selected!.delete(target);
   const active=[...selected??[]].sort(),requestedInputs=[...inputs.get(name)??[]].sort(),revision=JSON.stringify([active,requestedInputs]);
   if(revisions.get(name)===revision)continue;
   const value=await attempt(name,revision,async()=>active.length||requestedInputs.length||prior===undefined?await observe(owner,active,requestedInputs):prior);
   if(value===undefined)continue;
   if(value.project.name!==name)throw Error("Admitted canonical build owner differs: "+owner.manifest);
   revisions.set(name,revision);facts.set(name,value);changed=true;files.add(owner.manifest);
   for(const file of value.files){files.add(file);admitting.add(file);const dependency=sourceOwner(file);if(dependency&&!direct)admit(dependency,"file");}for(const file of value.observedFiles??[]){files.add(file);observed.add(file);}if(!direct)for(const dependency of value.dependencies)admit(dependency,"dependency");
   for(const[dependency,groups]of direct?[]:Object.entries(value.dependencyInputs??{}))requireInputs(dependency,groups);
   for(const target of active){const operation=value.project.targets[target];if(!operation)throw Error("Unknown admitted build target: "+name+":"+target);
    if(!direct)for(const dependency of [...operation.dependsOn??[],...value.generatedDependencies?.[target]??[]]){
     if(typeof dependency==="string"){if(dependency.startsWith("^")){for(const owner of value.dependencies)task(owner,dependency.slice(1),true);}else if(dependency.includes(":")){const separator=dependency.lastIndexOf(":"),project=dependency.slice(0,separator);task(project,dependency.slice(separator+1));}else task(name,dependency);}
     else if(dependency&&typeof dependency==="object"&&"target" in dependency){const entry=dependency as {target:string;projects?:string|string[];dependencies?:boolean};if(typeof entry.target!=="string")throw Error("Canonical build target dependency required");const projects=entry.dependencies?value.dependencies:entry.projects===undefined?[name]:entry.projects==="*"?[...byName.keys()]:typeof entry.projects==="string"?[entry.projects]:entry.projects;for(const project of projects)task(project,entry.target,entry.dependencies===true);}
     else throw Error("Canonical build target dependency required");
    }
   }
  }
  cursor=undefined;
  if(trace!==undefined&&admitted.size>=nextTrace){trace(summary());nextTrace=admitted.size+250;}
  for(const provider of registry.catalogProviders)if(selection.kind==="all"||[...targets].some(([name,selected])=>[...selected].some(target=>facts.get(name)?.project.targets[target])&&provider.consumerRoots.some(root=>contains(root,byName.get(name)!.root)))||[...inputs.values()].some(selected=>selected.has("{workspaceRoot}/"+provider.sourcePath))){
   if(!providers.has(provider.sourcePath)){providers.set(provider.sourcePath,{ownerRoot:provider.ownerRoot,sourcePath:provider.sourcePath});files.add(provider.sourcePath);const owner=sourceOwner(provider.sourcePath);if(!owner)throw Error("Canonical catalog provider owner required");requireInputs(owner,["{workspaceRoot}/"+provider.sourcePath]);changed=true;}
  }
  if(!changed){
   const needs=(name:string)=>selection.kind==="all"||named.has(name)||(inputs.get(name)?.size??0)>0||[...targets.get(name)??[]].some(target=>!optionalTasks.has(name+"\0"+target)||declares(name,target)!==false),verdicts=[...failures].map(([owner,error])=>({owner,error,needed:needs(owner)}));
   trace?.(summary());for(const verdict of verdicts)report?.(verdict);
   const required=verdicts.filter(verdict=>verdict.needed);if(required.length)throw joinOwnerFailuresV1(required.map(verdict=>verdict.error),sourceOwner);
   for(const verdict of verdicts){admitted.delete(verdict.owner);targets.delete(verdict.owner);inputs.delete(verdict.owner);}
   const foreign=[...observed].filter(file=>!admitting.has(file)).sort(),sorted=(values:Map<string,Set<string>>)=>Object.fromEntries([...values].filter(([,values])=>values.size).sort(([a],[b])=>a.localeCompare(b)).map(([name,values])=>[name,[...values].sort()])),result={owners:[...admitted.keys()].sort(),files:[...files].sort(),...(foreign.length?{observedFiles:foreign}:{}),targets:sorted(targets),inputs:sorted(inputs),catalogProviders:[...providers.values()].sort((a,b)=>a.sourcePath.localeCompare(b.sourcePath)),...(!direct&&[...facts.values()].some(value=>Object.keys(value.generatedDependencies??{}).length)?{generatedDependencies:Object.fromEntries([...facts].filter(([,value])=>Object.keys(value.generatedDependencies??{}).length).sort(([a],[b])=>a.localeCompare(b)).map(([name,value])=>[name,Object.fromEntries(Object.entries(value.generatedDependencies!).sort(([a],[b])=>a.localeCompare(b)).map(([target,producers])=>[target,[...producers].sort()]))]))}:{})};readBuildOwnerAdmissionV1(result);return result;}
 }
 throw Error("Finite canonical owner closure exceeded");
}

/** 🏭️ Orders exact generated task prerequisites and hashes their Source inputs without adopting generated bytes. */
export function applyBuildGeneratedTaskDependenciesV1<T extends {name:string;targets:Record<string,any>}>(project:T,value:unknown):T{
 if(validateJsonSchemaSubset({$defs:admissionSchema.$defs,$ref:"#/$defs/generatedTaskDependencies"},value).length)throw Error("Closed generated task dependencies required");
 const targets={...project.targets};for(const[name,producers]of Object.entries(value as Record<string,string[]>)){if(!targets[name])throw Error("Generated consumer task absent");const inputs=[...(targets[name].inputs??["default","^default"])];for(const producer of producers){if(producer===project.name+":"+name)throw Error("Generated task cannot consume its own output: "+JSON.stringify({project:project.name.slice(0,256),target:name.slice(0,256),producer:producer.slice(0,256)}));const owner=producer.slice(0,producer.lastIndexOf(":")),input=owner===project.name?"default":{input:"default",projects:[owner]};if(!inputs.some(value=>JSON.stringify(value)===JSON.stringify(input)))inputs.push(input);}targets[name]={...targets[name],dependsOn:[...new Set([...(targets[name].dependsOn??[]),...producers])],inputs};}return {...project,targets};
}

/** 🌐️ The bilingual progress line of a `run-many` whose `-p` selectors cannot resolve from declared names, so its graph read stays complete; undefined when the read is scoped.
 * @see ../../🧭️selection/📥️options/🟦️.ts */
export function buildWholeReadReasonV1(args:readonly string[]):string|undefined{
 const head=args.slice(0,args.includes("--")?args.indexOf("--"):undefined);if(args[0]!=="run-many"||head.some(argument=>argument==="--all"||argument.startsWith("--all=")))return undefined;
 const list=(names:string[])=>buildOptionsV1(args,names).flatMap(value=>value.split(",").filter(Boolean));return wholeBuildReadReasonV1(list(["projects","p"]),list(["exclude"]));
}

/** 🏁️ Makes complete workspace admission explicit for roster-dependent commands. */
export function buildInvocationGraphSelectionV1(args:readonly string[]):BuildGraphSelectionV1{
 const head=args.slice(0,args.includes("--")?args.indexOf("--"):undefined);
 if(args[0]==="run-many"&&!head.some(argument=>argument==="--all"||argument.startsWith("--all="))){const projects=buildOptionsV1(args,["projects","p"]).flatMap(value=>value.split(",").filter(Boolean)),exclude=buildOptionsV1(args,["exclude"]).flatMap(value=>value.split(",").filter(Boolean)),targets=[...new Set(buildOptionsV1(args,["targets","target","t"]).flatMap(value=>value.split(",").filter(Boolean)))];if(declaredBuildProjectSelectorsV1(projects,exclude)&&targets.length&&targets.length<=64&&targets.every(target=>!/[*?\[\]{}()\\:]/u.test(target)))return readBuildGraphSelectionV1({kind:"owners",projects:[...new Set(projects)],exclude:[...new Set(exclude)],targets});}
 if(args[0]==="exec"){const projects=buildOptionsV1(args,["projects","p"]).flatMap(value=>value.split(",").filter(Boolean)),exclude=buildOptionsV1(args,["exclude"]).flatMap(value=>value.split(",").filter(Boolean));if(literalBuildProjectSelectorsV1(projects,exclude))return readBuildGraphSelectionV1({kind:"owners",projects:[...new Set(projects)],exclude:[...new Set(exclude)]});}
 if(args[0]==="show"&&args[1]==="project"&&args[2]&&literalBuildProjectSelectorsV1([args[2]],[]))return readBuildGraphSelectionV1({kind:"owners",projects:[args[2]],exclude:[]});
 if(args[0]!=="run")return {kind:"all"};
 const excludeDependencies=buildBooleanOptionV1(head,["exclude-dependencies","exclude-task-dependencies"])===true,identity=args[1]?.split(":");if(!identity||identity.length<2||identity.length>3||identity.some(part=>!part))throw Error("Canonical build task identity required");
 let configuration=identity[2];for(let index=2;index<args.length&&args[index]!=="--";index++){const value=args[index]!;if(value==="--configuration"||value==="-c"||value.startsWith("--configuration=")||value.startsWith("-c=")){const selected=value.includes("=")?value.slice(value.indexOf("=")+1):args[++index];if(!selected||selected.startsWith("-")||configuration&&configuration!==selected)throw Error("Conflicting task configurations");configuration=selected;}}
 return readBuildGraphSelectionV1({kind:"tasks",tasks:[{project:identity[0],target:identity[1],...(configuration?{configuration}:{})}],...(excludeDependencies?{excludeDependencies:true}:{})});
}
