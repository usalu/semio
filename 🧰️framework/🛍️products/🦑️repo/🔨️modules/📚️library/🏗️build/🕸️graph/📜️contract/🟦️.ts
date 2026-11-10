import type {BuildGraphInvalidationInventoryV1,BuildGraphV1,BuildTargetV1} from "../📋️model/🟦️.ts";
import {readBuildGraphSelectionV1} from "../🚪️admission/🟦️.ts";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {repositoryCachePartitionRootV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import {graphDeclarationV1} from "../🔐️authority/🟦️.ts";


/** ✅️ Admits the language-neutral graph contract and cross-project identities. */
export function readBuildGraphV1(value:unknown):BuildGraphV1 {
  const errors=validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/🔣️.json"),value);
  if(errors.length)throw Error(`Invalid build graph: ${JSON.stringify(errors.slice(0,4))}`);
  const graph=value as BuildGraphV1;
  if(graph.admission)readBuildGraphSelectionV1(graph.admission.selection);
  const roots=new Map<string,string>();
  for(const[name,project]of Object.entries(graph.projects)){if(name!==project.name)throw Error(`Build project identity differs: ${name}`);const previous=roots.get(project.root);if(previous)throw Error(`Duplicate build project root ${project.root}: ${previous}, ${name}`);roots.set(project.root,name);}
  for(const edge of graph.dependencies)if(!graph.projects[edge.source]&&!graph.externalNodes?.[edge.source]||!graph.projects[edge.target]&&!graph.externalNodes?.[edge.target])throw Error(`Unknown build dependency ${edge.source} -> ${edge.target}`);
  return graph;
}


/** 🔄️ Removes implementation-specific executor names from an inferred operation. */
export function neutralBuildTargetV1(value:Record<string,any>,identity?:string):BuildTargetV1 {
  const {executor,command,options:declaredOptions={},configurations,...rest}=value;
  if(command!==undefined&&(typeof command!=="string"||declaredOptions.command!==undefined||declaredOptions.commands!==undefined))throw Error("Build command shorthand requires one command form");
  const options=command===undefined?declaredOptions:{...declaredOptions,command};
  requireBuildArgumentsV1(options,identity);
  if(executor!==undefined&&!["repo:script","repo:noop"].includes(executor))throw Error(`Unsupported project executor${identity?` for ${identity}`:""}: ${executor}`);
  const normalize=(configuration:Record<string,any>)=>{requireBuildConfigurationV1(options,configuration,identity);return {execution:configuration};};
  return {...rest,execution:{kind:executor==="repo:noop"?"noop":"command",...options},...(configurations?{configurations:Object.fromEntries(Object.entries(configurations).map(([name,configuration])=>[name,normalize(configuration as Record<string,any>)]))}:{})};
}



/** 🔐️ Keeps configured execution options inside the same task and command identity. */
export function requireBuildConfigurationV1(base:Record<string,unknown>,configuration:Record<string,unknown>,identity?:string):void {
  if(!configuration||typeof configuration!=="object"||Array.isArray(configuration))throw Error("Build configuration execution options required");
  if(configuration.env!==undefined&&(!configuration.env||typeof configuration.env!=="object"||Array.isArray(configuration.env)||Object.values(configuration.env).some(value=>typeof value!=="string")))throw Error("Build configuration environment strings required");
  requireBuildArgumentsV1(configuration,identity);
  const fixed=["kind","program","ownerArguments","nativeOwnerCommand","executor","inputs","outputs","dependsOn","cache","continuous","parallelism","metadata","defaultConfiguration","configurations","options"];
  if(Object.keys(configuration).some(key=>fixed.includes(key)))throw Error("Build configuration cannot change task identity");
  if(base.program!==undefined&&(configuration.command!==undefined||configuration.commands!==undefined)||configuration.command!==undefined&&(configuration.commands!==undefined||base.commands!==undefined)||configuration.commands!==undefined&&base.command!==undefined)throw Error("Build configuration cannot change command form");
}


/** 🔡️ Keeps authored argument boundaries literal through every backend's execution protocol. */
function requireBuildArgumentsV1(options:Record<string,unknown>,identity?:string):void {
  if(options.program!==undefined&&(typeof options.program!=="string"||!options.program||options.command!==undefined||options.commands!==undefined))throw Error("Literal build program requires one exclusive nonempty executable");
  if(options.args!==undefined&&(!Array.isArray(options.args)||options.args.some(value=>typeof value!=="string")))throw Error(`Build execution arguments must be literal string arrays${identity?` for ${identity}`:""}`);
}


/** 🗃️ Locates the sole published portable graph. */
export function buildGraphPathV1(root:string):string { const publication=graphDeclarationV1("../🧬️schema/📣️publication.json").properties;return join(repositoryCachePartitionRootV1(root,publication.partition.const),publication.graph.const); }


/** 🗑️ Selects only owned current publications and their bounded staging files. */
export function planBuildGraphInvalidationV1(inventory:BuildGraphInvalidationInventoryV1):string[] {
  const schema=graphDeclarationV1("../🧬️schema/🗑️reset.json");if(validateJsonSchemaSubset(schema,inventory).length)throw Error("Build graph reset inventory required");
  const owned=new RegExp(schema.$defs.OwnedName.pattern,"u"),selected=inventory.entries.filter(entry=>owned.test(entry.name));
  for(const entry of selected)if(validateJsonSchemaSubset(schema.$defs.OwnedEntry,entry,schema).length||entry.device!==inventory.device)throw Error("Owned regular bounded graph publication required");
  return selected.map(entry=>entry.name).sort();
}


/** 🪶️ Admits the lean graph of `show projects|targets`: project and target names, unavailable projects, no inputs. */
export function readBuildGraphLeanV1(value:unknown):BuildGraphV1 {
  const errors=validateJsonSchemaSubset(graphDeclarationV1("../🧬️schema/🪶️lean.json"),value);
  if(errors.length)throw Error(`Invalid lean build graph: ${JSON.stringify(errors.slice(0,4))}`);
  return value as BuildGraphV1;
}
