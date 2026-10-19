import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import selectionSchema from "../../🧬️schema/🚪️selection.json" with {type:"json"};
import registrySchema from "../../🧬️schema/📋️owners.json" with {type:"json"};
import admissionSchema from "../../🧬️schema/🚪️admission.json" with {type:"json"};

/** 🎯️ Declares exact task admission or intentional complete workspace ownership. */
export type BuildGraphSelectionV1={kind:"all"}|{kind:"tasks";tasks:{project:string;target:string;configuration?:string}[]};
/** 🪪️ Resolves authored task and package names without evaluating independent owners. */
export interface BuildOwnerRegistryV1{version:1;owners:{name:string;root:string;manifest:string;packages:string[];nativeRoots?:string[]}[];catalogProviders:{ownerRoot:string;sourcePath:string;consumerRoots:string[]}[]}
/** 📥️ Carries exact original owner observations and source-owned dependency identities. */
export interface BuildOwnerAdmissionFactsV1{project:{name:string;targets:Record<string,{dependsOn?:unknown[]}>};files:string[];dependencies:string[];dependencyInputs?:Record<string,string[]>;generatedDependencies?:Record<string,string[]>}
/** 🚪️ Closes manifest, command-source and prerequisite ownership before source assembly. */
export interface BuildOwnerAdmissionV1{owners:string[];files:string[];targets:Record<string,string[]>;inputs:Record<string,string[]>;catalogProviders:{ownerRoot:string;sourcePath:string}[];generatedDependencies?:Record<string,Record<string,string[]>>}
const contains=(root:string,path:string)=>root==="."||path===root||path.startsWith(root+"/");
const sourceOwners=(registry:BuildOwnerRegistryV1)=>registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>({root,name:owner.name}))).sort((a,b)=>b.root.length-a.root.length||a.name.localeCompare(b.name));

/** ✅️ Rejects open selections and canonicalizes the finite task order. */
export function readBuildGraphSelectionV1(value:unknown):BuildGraphSelectionV1{
 if(validateJsonSchemaSubset(selectionSchema,value).length)throw Error("Closed build graph selection required");
 const selection=value as BuildGraphSelectionV1;return selection.kind==="all"?{kind:"all"}:{kind:"tasks",tasks:selection.tasks.map(task=>({...task})).sort((a,b)=>JSON.stringify(a).localeCompare(JSON.stringify(b)))};
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

/** 📥️ Distinguishes byte freshness observation from a task or explicitly declared compilation entry. */
export function buildInputSourceRoleV1(selected:boolean,declared=false):"compile"|"freshness"{
 const roles=admissionSchema.$defs.sourceInputObservation.const;return (declared?roles.declaredEntry:selected?roles.selectedTask:roles.namedInputOnly) as "compile"|"freshness";
}

/** 🔑️ Names the complete immutable selection and registry declaration independently of whole graph state. */
export function buildGraphScopeKeyV1(selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1):string{
 return createHash("sha256").update(JSON.stringify([readBuildGraphSelectionV1(selection),readBuildOwnerRegistryV1(registry)])).digest("hex");
}

/** 🧩️ Converges exact task prerequisites and source owners through a caller-owned observation port. */
export async function closeBuildOwnerAdmissionV1(selection:BuildGraphSelectionV1,registry:BuildOwnerRegistryV1,inspect:(owner:BuildOwnerRegistryV1["owners"][number],targets:readonly string[],inputs:readonly string[])=>Promise<BuildOwnerAdmissionFactsV1>):Promise<BuildOwnerAdmissionV1>{
 selection=readBuildGraphSelectionV1(selection);registry=readBuildOwnerRegistryV1(registry);
 const byName=new Map(registry.owners.map(owner=>[owner.name,owner])),ownerIndex=sourceOwners(registry),sourceOwner=(path:string)=>ownerIndex.find(owner=>contains(owner.root,path))?.name,admitted=new Set<string>(),targets=new Map<string,Set<string>>(),inputs=new Map<string,Set<string>>(),optionalTasks=new Set<string>(),files=new Set<string>(),providers=new Map<string,{ownerRoot:string;sourcePath:string}>(),revisions=new Map<string,string>(),facts=new Map<string,BuildOwnerAdmissionFactsV1>();
 const observe=async(owner:BuildOwnerRegistryV1["owners"][number],targets:readonly string[],inputs:readonly string[])=>{const value=await inspect(owner,targets,inputs);if(validateJsonSchemaSubset({$defs:admissionSchema.$defs,$ref:"#/$defs/ownerFacts"},value).length)throw Error("Closed build owner observations required");return value;};
 const admit=(name:string)=>{if(!byName.has(name))throw Error("Unknown canonical build owner: "+name);admitted.add(name);},task=(name:string,target:string,optional=false)=>{admit(name);const selected=targets.get(name)??new Set<string>(),key=name+"\0"+target;if(optional&&!selected.has(target))optionalTasks.add(key);else if(!optional)optionalTasks.delete(key);selected.add(target);targets.set(name,selected);};
 if(selection.kind==="all")for(const owner of registry.owners)admit(owner.name);else for(const value of selection.tasks)task(value.project,value.target);
 for(let pass=0;pass<=registry.owners.length+65536;pass++){
  let changed=false;
  for(const name of [...admitted].sort()){
   const owner=byName.get(name)!;if(!facts.has(name)){const base=await observe(owner,[],[]);if(base.project.name!==name)throw Error("Admitted canonical build owner differs: "+owner.manifest);facts.set(name,base);}
   for(const target of targets.get(name)??[])if(optionalTasks.has(name+"\0"+target)&&!facts.get(name)!.project.targets[target])targets.get(name)!.delete(target);
   const active=[...targets.get(name)??[]].sort(),requestedInputs=[...inputs.get(name)??[]].sort(),revision=JSON.stringify([active,requestedInputs]);if(revisions.get(name)===revision)continue;
   const value=active.length||requestedInputs.length?await observe(owner,active,requestedInputs):facts.get(name)!;if(value.project.name!==name)throw Error("Admitted canonical build owner differs: "+owner.manifest);revisions.set(name,revision);facts.set(name,value);changed=true;files.add(owner.manifest);
   for(const file of value.files){files.add(file);const dependency=sourceOwner(file);if(dependency)admit(dependency);}for(const dependency of value.dependencies)admit(dependency);
   for(const[dependency,groups]of Object.entries(value.dependencyInputs??{})){admit(dependency);const selected=inputs.get(dependency)??new Set<string>();for(const input of groups)selected.add(input);inputs.set(dependency,selected);}
   for(const target of active){const operation=value.project.targets[target];if(!operation)throw Error("Unknown admitted build target: "+name+":"+target);
    for(const dependency of [...operation.dependsOn??[],...value.generatedDependencies?.[target]??[]]){
     if(typeof dependency==="string"){if(dependency.startsWith("^")){for(const owner of value.dependencies)task(owner,dependency.slice(1),true);}else if(dependency.includes(":")){const separator=dependency.lastIndexOf(":"),project=dependency.slice(0,separator);task(project,dependency.slice(separator+1));}else task(name,dependency);}
     else if(dependency&&typeof dependency==="object"&&"target" in dependency){const entry=dependency as {target:string;projects?:string|string[];dependencies?:boolean};if(typeof entry.target!=="string")throw Error("Canonical build target dependency required");const projects=entry.dependencies?value.dependencies:entry.projects===undefined?[name]:entry.projects==="*"?[...byName.keys()]:typeof entry.projects==="string"?[entry.projects]:entry.projects;for(const project of projects)task(project,entry.target,entry.dependencies===true);}
     else throw Error("Canonical build target dependency required");
    }
   }
  }
  for(const provider of registry.catalogProviders)if(selection.kind==="all"||[...targets].some(([name,selected])=>[...selected].some(target=>facts.get(name)?.project.targets[target])&&provider.consumerRoots.some(root=>contains(root,byName.get(name)!.root)))||[...inputs.values()].some(selected=>selected.has("{workspaceRoot}/"+provider.sourcePath))){
   if(!providers.has(provider.sourcePath)){providers.set(provider.sourcePath,{ownerRoot:provider.ownerRoot,sourcePath:provider.sourcePath});files.add(provider.sourcePath);const owner=sourceOwner(provider.sourcePath);if(!owner)throw Error("Canonical catalog provider owner required");admit(owner);const selected=inputs.get(owner)??new Set<string>();selected.add("{workspaceRoot}/"+provider.sourcePath);inputs.set(owner,selected);changed=true;}
  }
  if(!changed){const sorted=(values:Map<string,Set<string>>)=>Object.fromEntries([...values].filter(([,values])=>values.size).sort(([a],[b])=>a.localeCompare(b)).map(([name,values])=>[name,[...values].sort()])),result={owners:[...admitted].sort(),files:[...files].sort(),targets:sorted(targets),inputs:sorted(inputs),catalogProviders:[...providers.values()].sort((a,b)=>a.sourcePath.localeCompare(b.sourcePath)),...([...facts.values()].some(value=>Object.keys(value.generatedDependencies??{}).length)?{generatedDependencies:Object.fromEntries([...facts].filter(([,value])=>Object.keys(value.generatedDependencies??{}).length).sort(([a],[b])=>a.localeCompare(b)).map(([name,value])=>[name,Object.fromEntries(Object.entries(value.generatedDependencies!).sort(([a],[b])=>a.localeCompare(b)).map(([target,producers])=>[target,[...producers].sort()]))]))}:{})};if(validateJsonSchemaSubset(admissionSchema,result).length)throw Error("Closed selected build owner admission required");return result;}
 }
 throw Error("Finite canonical owner closure exceeded");
}

/** 🏭️ Orders exact generated task prerequisites and hashes their Source inputs without adopting generated bytes. */
export function applyBuildGeneratedTaskDependenciesV1<T extends {name:string;targets:Record<string,any>}>(project:T,value:unknown):T{
 if(validateJsonSchemaSubset({$defs:admissionSchema.$defs,$ref:"#/$defs/generatedTaskDependencies"},value).length)throw Error("Closed generated task dependencies required");
 const targets={...project.targets};for(const[name,producers]of Object.entries(value as Record<string,string[]>)){if(!targets[name])throw Error("Generated consumer task absent");const inputs=[...(targets[name].inputs??["default","^default"])];for(const producer of producers){if(producer===project.name+":"+name)throw Error("Generated task cannot consume its own output");const owner=producer.slice(0,producer.lastIndexOf(":")),input=owner===project.name?"default":{input:"default",projects:[owner]};if(!inputs.some(value=>JSON.stringify(value)===JSON.stringify(input)))inputs.push(input);}targets[name]={...targets[name],dependsOn:[...new Set([...(targets[name].dependsOn??[]),...producers])],inputs};}return {...project,targets};
}

/** 🏁️ Makes complete workspace admission explicit for roster-dependent commands. */
export function buildInvocationGraphSelectionV1(args:readonly string[]):BuildGraphSelectionV1{
 if(args[0]!=="run")return {kind:"all"};
 const identity=args[1]?.split(":");if(!identity||identity.length<2||identity.length>3||identity.some(part=>!part))throw Error("Canonical build task identity required");
 let configuration=identity[2];for(let index=2;index<args.length&&args[index]!=="--";index++){const value=args[index]!;if(value==="--configuration"||value==="-c"||value.startsWith("--configuration=")||value.startsWith("-c=")){const selected=value.includes("=")?value.slice(value.indexOf("=")+1):args[++index];if(!selected||selected.startsWith("-")||configuration&&configuration!==selected)throw Error("Conflicting task configurations");configuration=selected;}}
 return readBuildGraphSelectionV1({kind:"tasks",tasks:[{project:identity[0],target:identity[1],...(configuration?{configuration}:{})}]});
}
