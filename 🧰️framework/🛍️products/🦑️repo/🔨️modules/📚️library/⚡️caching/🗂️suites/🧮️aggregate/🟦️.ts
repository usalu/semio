import {posix} from "node:path";
import {isDeepStrictEqual} from "node:util";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {literalCommandTokensV1} from "../../../🏗️build/📤️arguments/🟨️.mjs";
import {readCachingTestTableV1} from "../🟦️.ts";
import {readBuildTestTableV1} from "../../../🏗️build/🗂️suites/🟦️.ts";
import {refuseBuildV1} from "../../../🏗️build/📖️texts/🟦️.ts";
import {refuseProcessV1} from "../../../../../../../🔨️modules/🏃️process/📖️texts/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🧭️ One canonical route can retain distinct declared script issuers. */
export interface CanonicalTestRouteV1 {readonly id:string;readonly commands:readonly {readonly entry:string;readonly arguments:readonly string[]}[];}
/** 🏷️ The authored project declarations needed to resolve exact test command ownership. */
export interface CanonicalTestProjectV1 {readonly name:string;readonly root:string;readonly targets:Readonly<Record<string,any>>;}
/** 📜️ The schema-owned aggregate identity and route entries. */
export const canonicalTestAggregatePolicyV1=schema.$defs.Policy.const;

/** 🗂️ Derives executable route identities from both closed tables, including deliberate function proofs. */
export function canonicalTestRoutesV1(cache=readCachingTestTableV1(),build=readBuildTestTableV1()):readonly CanonicalTestRouteV1[]{
 const policy=canonicalTestAggregatePolicyV1,command=(entry:string,arguments_:readonly string[])=>({entry,arguments:arguments_});
 return [...cache.suites.map(row=>({id:row.id,commands:[command(policy.cacheEntry,["test",row.id])]})),...build.suites.map(row=>({id:row.id,commands:[command(policy.buildEntry,["test",row.id])]})),...cache.codeSuites.map(id=>({id,commands:[command(policy.cacheEntry,["test",id]),command(policy.publicEntry,["test","cache",id])]}))];
}

/** 🔗️ Resolves each canonical route to exactly one existing target by literal command and workspace coordinates. */
export function resolveCanonicalTestTargetsV1(project:CanonicalTestProjectV1,routes:readonly CanonicalTestRouteV1[]=canonicalTestRoutesV1(),aggregate:string="test",contracts:string="test-contracts"):readonly string[]{
 const identities=new Map<string,string>(),prefixes=new Map<string,{entry:string;arguments:readonly string[]}>(),ids=new Set<string>(),matches=new Map<string,string[]>(),signature=(entry:string,args:readonly string[])=>JSON.stringify([entry,args]);
 const refuse=(reason:string):never=>{throw refuseBuildV1("invalid","test.aggregate-refused",{owner:project.name,reason});};
 for(const route of routes){
  if(validateJsonSchemaSubset({$defs:schema.$defs,$ref:"#/$defs/Route"},route).length||ids.has(route.id)||route.id==="contracts")refuse("route-invalid:"+route.id);
  ids.add(route.id);
  for(const command of route.commands){if(command.arguments.at(-1)!==route.id)refuse("route-identity:"+route.id);const key=signature(command.entry,command.arguments);if(identities.has(key))refuse("route-ambiguous:"+route.id);identities.set(key,route.id);const prefix={entry:command.entry,arguments:command.arguments.slice(0,-1)};prefixes.set(signature(prefix.entry,prefix.arguments),prefix);}
 }
 for(const[name,target]of Object.entries(project.targets)){
  if(name===aggregate||name===contracts||target.executor!=="repo:script"||typeof target.options?.command!=="string")continue;
  const tokens=literalCommandTokensV1(target.options.command);if(tokens[0]!=="repo"||tokens.length<3)continue;
  const cwd=(target.options.cwd??project.root).replaceAll("{projectRoot}",project.root).replaceAll("{workspaceRoot}",".").replaceAll("\\","/");
  if(cwd.includes("{")||posix.isAbsolute(cwd))refuse("coordinate-relative:"+name);
  const entry=posix.relative("/",posix.resolve("/",cwd,tokens[1]!.replaceAll("\\","/"))),id=identities.get(signature(entry,tokens.slice(2)));
  if(id!==undefined)matches.set(id,[...(matches.get(id)??[]),name]);
  else if([...prefixes.values()].some(prefix=>prefix.entry===entry&&tokens.length-2===prefix.arguments.length+1&&prefix.arguments.every((value,index)=>tokens[index+2]===value)))refuse("route-undeclared:"+name);
 }
 const selected:string[]=[];
 for(const id of ids){const names=matches.get(id)??[];if(names.length!==1)refuse("route-targets:"+id+":"+names.length);selected.push(names[0]!);}
 if(!project.targets[contracts]||project.targets[contracts].executor!=="repo:script")refuse("contracts-missing:"+contracts);
 if(new Set(selected).size!==selected.length||selected.includes(aggregate)||selected.includes(contracts))refuse("target-duplicate");
 return [...selected,contracts].sort();
}

/** 🧮️ Derives the no-op declaration without changing any independently issued test leaf. */
export function deriveCanonicalTestAggregateV1(project:CanonicalTestProjectV1):Record<string,any>{
 const policy=canonicalTestAggregatePolicyV1,inputs=[policy.manifest,policy.cacheTable,policy.buildTable,policy.implementation+"/**/*"].map(path=>"{workspaceRoot}/"+path);
 return {executor:"repo:noop",cache:false,dependsOn:resolveCanonicalTestTargetsV1(project),inputs,outputs:[],metadata:{semio:{cache:{version:1,class:"aggregate",reason:"orders-prerequisites"},arguments:{maximum:0}}}};
}

/** 🛡️ Refuses stale canonical dependencies before graph publication. */
export function requireCanonicalTestAggregateV1(project:Record<string,any>,root:string):void{
 const policy=canonicalTestAggregatePolicyV1;if(root!==policy.root)return;
 if(project.name!==policy.owner)throw refuseBuildV1("invalid","test.aggregate-refused",{owner:project.name,reason:"owner"});
 const expected=deriveCanonicalTestAggregateV1({...project,root} as CanonicalTestProjectV1),actual=project.targets?.[policy.target];
 if(!isDeepStrictEqual(actual,expected))throw refuseBuildV1("invalid","test.aggregate-stale",{owner:project.name});
}

/** 🚦️ Selects one declared process suite, function proof, or explicit contracts leaf. */
export function selectCanonicalCacheTestV1(args:readonly string[],cache=readCachingTestTableV1()):"suite"|"code"|"contracts"{
 if(args.length===1&&args[0]==="contracts")return "contracts";
 if(args.length===1&&cache.suites.some(row=>row.id===args[0]))return "suite";
 if(args.length>0&&cache.codeSuites.includes(args[0]!))return "code";
 throw refuseProcessV1("invalid","script.suite-unknown",{suite:args.join(" "),known:[...cache.suites.map(row=>row.id),...cache.codeSuites,"contracts"].join(", ")});
}
