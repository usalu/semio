import {parseJsoncV1} from "../📖️formats/🟦️.ts";
import {createPackageManagerV1,type LockDriftV1,type LockMemberV1,type LockPackageV1,type LockProjectionV1,type LockReaderV1,type ManagerDescriptorV1} from "../🟦️.ts";
import descriptor from "./🔣️.json" with {type:"json"};

const linkedV1=/^(workspace|file|link|portal):/u;

/** 🍞️ Reads the JSONC lock of this manager: members come from the workspaces table, packages from the packages table. */
export const readBunLockV1:LockReaderV1=text=>{
 const lock=parseJsoncV1(text) as {lockfileVersion?:number;workspaces?:Record<string,{name?:string}>;packages?:Record<string,unknown[]>};
 if(![1,2,3].includes(lock.lockfileVersion as number))throw Error("Supported lock version required: "+String(lock.lockfileVersion));
 const members:LockMemberV1[]=Object.entries(lock.workspaces??{}).map(([path,row])=>({name:row.name??path,path:path===""?".":path})),packages:LockPackageV1[]=[];
 for(const [key,row] of Object.entries(lock.packages??{})){
  const spec=row[0];if(typeof spec!=="string")throw Error("Named lock package required: "+key);
  const at=spec.lastIndexOf("@");if(at<=0)throw Error("Versioned lock package required: "+key);
  const version=spec.slice(at+1);if(linkedV1.test(version))continue;
  packages.push({name:spec.slice(0,at),version,key,source:typeof row[1]==="string"&&row[1]!==""?row[1]:null,integrity:(row.find(item=>typeof item==="string"&&/^sha\d+-/u.test(item)) as string|undefined)??null});
 }
 return {packages,members};
};
/** 🔁️ Rewrites the workspace paths of the lock between the original tree and the staged generation, and the patch identities with them. */
export function projectBunLockV1(text:string,input:LockProjectionV1):string{
 const lock=parseJsoncV1(text) as any,links=input.reverse?new Map([...input.mapping].map(([source,target])=>[target,source])):input.mapping;
 if(lock.patchedDependencies)lock.patchedDependencies=Object.fromEntries(input.patches.map(row=>[row.name,input.reverse?row.reverse:row.forward]));
 lock.workspaces=Object.fromEntries(Object.entries(lock.workspaces??{}).filter(([path])=>input.reverse||path===""||links.has(path)).map(([path,value])=>[links.get(path)??path,value]));
 for(const [key,row] of Object.entries(lock.packages??{}) as [string,any][]){
  if(!Array.isArray(row)||typeof row[0]!=="string")continue;
  const marker=row[0].indexOf("@workspace:");if(marker<0)continue;
  const path=row[0].slice(marker+11),target=links.get(path);
  if(!target){if(input.reverse)throw Error("Locked workspace package lacks original admitted Source");delete lock.packages[key];continue;}
  row[0]=row[0].slice(0,marker+11)+target;
 }
 return JSON.stringify(lock,null,2)+"\n";
}
const dependencySectionsV1=["dependencies","devDependencies","optionalDependencies","peerDependencies"] as const;
const codeUnitsV1=(left:string,right:string):number=>left<right?-1:left>right?1:0;
const recordV1=(value:unknown):Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value)?value as Record<string,unknown>:{};
const tableV1=(value:unknown):Record<string,string>=>Object.fromEntries(Object.entries(recordV1(value)).map(([name,spec])=>[name,String(spec)]));
const namesV1=(value:unknown):Record<string,string>=>Array.isArray(value)?Object.fromEntries(value.filter((name):name is string=>typeof name==="string").map(name=>[name,""])):{};
const scalarV1=(value:unknown):string|undefined=>typeof value==="string"?value:undefined;
const edgeSectionsV1=["dependencies","optionalDependencies","peerDependencies"] as const;
/** 🕸️ Every package name some manifest of the scope declares directly (its members included) or some locked package depends on, by name over every locked version; a locked name outside this set has no user left, so Bun prunes it and refuses a frozen install. */
const reachableNamesV1=(packages:unknown,manifests:ReadonlyMap<string,unknown>):ReadonlySet<string>=>{
 const edges=new Map<string,string[]>(),reached=new Set<string>(),queue:string[]=[];
 for(const row of Object.values(recordV1(packages))){
  const spec=Array.isArray(row)?row[0]:undefined,at=typeof spec==="string"?spec.lastIndexOf("@"):-1;
  if(at<=0)continue;
  const name=(spec as string).slice(0,at),meta=recordV1((row as unknown[])[2]),targets=edges.get(name)??[];
  for(const section of edgeSectionsV1)targets.push(...Object.keys(recordV1(meta[section])));
  edges.set(name,targets);
 }
 for(const manifest of manifests.values()){const document=recordV1(manifest);if(typeof document.name==="string")reached.add(document.name);for(const section of dependencySectionsV1)queue.push(...Object.keys(recordV1(document[section])));}
 for(let name=queue.pop();name!==undefined;name=queue.pop())if(!reached.has(name)){reached.add(name);queue.push(...(edges.get(name)??[]));}
 return reached;
};
const lockedNamesV1=(packages:unknown):ReadonlySet<string>=>new Set(Object.keys(recordV1(packages)).map(key=>{const parts=key.split("/"),last=parts.pop()!;return parts.length&&parts[parts.length-1]!.startsWith("@")?parts.pop()+"/"+last:last;}));
/** 🩺️ Compares the workspace table and the root tables of a Bun lock with the manifests of its scope: the members the lock lists, each member's name and version, the four dependency tables and the root overrides, patches and trusted dependencies. A disagreement is `frozen` only where Bun is certain to refuse a frozen install (verified against Bun 1.4.2 by the differential vectors): a member the lock lacks or no manifest backs, a renamed member, a changed `overrides` table, a dependency no locked package carries, or a locked dependency that no manifest and no locked package uses any more (its last user was removed); version bumps, dependency edits that keep the resolved package set, a renamed scope root (its row carries no version), `resolutions`, patches and trusted dependencies are lock noise only. */
export function checkBunLockV1(document:unknown,manifests:ReadonlyMap<string,unknown>):readonly LockDriftV1[]{
 const lock=recordV1(document),rows=recordV1(lock.workspaces),drift:LockDriftV1[]=[],seen=new Set<string>(),locked=lockedNamesV1(lock.packages),used=reachableNamesV1(lock.packages,manifests);
 const push=(kind:LockDriftV1["kind"],manifest:string,frozen:boolean,fields:Omit<LockDriftV1,"kind"|"manifest"|"frozen">={})=>{drift.push(Object.freeze({kind,manifest,...Object.fromEntries(Object.entries(fields).filter(([,value])=>value!==undefined)),frozen}) as LockDriftV1);};
 const diff=(manifest:string,section:string,authored:Record<string,string>,table:Record<string,string>,certain:(dependency:string,kind:LockDriftV1["kind"])=>boolean)=>{
  for(const dependency of [...new Set([...Object.keys(authored),...Object.keys(table)])]){
   const manifestVersion=authored[dependency],lockVersion=table[dependency];
   if(manifestVersion===undefined)push("stale-in-lock",manifest,certain(dependency,"stale-in-lock"),{section,dependency,lockVersion});
   else if(lockVersion===undefined)push("missing-in-lock",manifest,certain(dependency,"missing-in-lock"),{section,dependency,manifestVersion});
   else if(manifestVersion!==lockVersion)push("version-mismatch",manifest,certain(dependency,"version-mismatch"),{section,dependency,manifestVersion,lockVersion});
  }
 };
 for(const [path,value] of manifests){
  const manifest=path.replaceAll("\\","/"),key=manifest==="package.json"?"":manifest.replace(/\/package\.json$/u,""),row=rows[key];
  seen.add(key);
  if(row===undefined||row===null||typeof row!=="object"){push("workspace-missing",manifest,true);continue;}
  const authored=recordV1(value),entry=recordV1(row);
  for(const field of (key===""?["name"]:["name","version"]) as readonly ("name"|"version")[])if(scalarV1(authored[field])!==scalarV1(entry[field]))push("identity-mismatch",manifest,field==="name"&&key!=="",{section:field,manifestVersion:scalarV1(authored[field]),lockVersion:scalarV1(entry[field])});
  for(const section of dependencySectionsV1)diff(manifest,section,tableV1(authored[section]),tableV1(entry[section]),(dependency,kind)=>kind==="missing-in-lock"?!locked.has(dependency):kind==="stale-in-lock"&&!used.has(dependency));
  if(key==="")for(const [section,authoredTable,lockedTable,certain] of [["overrides",{...tableV1(authored.resolutions),...tableV1(authored.overrides)},tableV1(lock.overrides),authored.resolutions===undefined],["patchedDependencies",tableV1(authored.patchedDependencies),tableV1(lock.patchedDependencies),false],["trustedDependencies",namesV1(authored.trustedDependencies),namesV1(lock.trustedDependencies),false]] as const)diff(manifest,section,authoredTable,lockedTable,()=>certain);
 }
 for(const key of Object.keys(rows))if(!seen.has(key))push("workspace-stale",key===""?"package.json":key+"/package.json",true);
 return Object.freeze(drift.sort((left,right)=>codeUnitsV1(left.manifest,right.manifest)||codeUnitsV1(left.section??"",right.section??"")||codeUnitsV1(left.dependency??"",right.dependency??"")||codeUnitsV1(left.kind,right.kind)));
}
/** 📦️ The adapter: descriptor data plus the lock reader and its capabilities. */
export const bunPackageManagerV1=createPackageManagerV1(descriptor as unknown as ManagerDescriptorV1,readBunLockV1,{document:parseJsoncV1,project:projectBunLockV1,check:checkBunLockV1});
