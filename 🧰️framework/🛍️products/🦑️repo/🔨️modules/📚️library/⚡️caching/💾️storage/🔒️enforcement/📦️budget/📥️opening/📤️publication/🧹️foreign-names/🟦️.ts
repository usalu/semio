import{lstatSync,mkdirSync,readdirSync,renameSync,rmSync}from"node:fs";
import{join}from"node:path";

/** 🧹️ The declared names of the `j` folder of a dependency image beside generations and capsules (`SelectedPlanProtocol.foreignNames` of the install schema): `ignorable` OS metadata patterns, the `tombstone` pattern of a retirement in flight, the `quarantine` folder and the bounds of the quarantine. */
export type DependencyForeignPolicyV1=Readonly<{ignorable:readonly string[];tombstone:string;generation:string;quarantine:string;graceMilliseconds:number;maximum:number;retentionMilliseconds:number}>;

export type DependencyNameKindV1="generation"|"tombstone"|"quarantine"|"ignorable"|"unknown";

/** 🏷️ What a name of the generation folder is: a generation or capsule, the tombstone of a retirement the collector finishes, the quarantine folder, an ignorable OS metadata file, or unknown. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function dependencyNameKindV1(name:string,policy:DependencyForeignPolicyV1):DependencyNameKindV1{
 if(new RegExp(policy.generation,"u").test(name))return"generation";
 if(new RegExp(policy.tombstone,"u").test(name))return"tombstone";
 if(name===policy.quarantine)return"quarantine";
 return policy.ignorable.some(pattern=>new RegExp(pattern,"u").test(name))?"ignorable":"unknown";
}

/** 🧮️ The generation and capsule names among the names of the generation folder: tombstones, the quarantine folder, OS metadata and unknown names are not generations, so a roster comparison made after the plan was captured must look at these names only. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function dependencyGenerationNamesV1(names:readonly string[],policy:DependencyForeignPolicyV1):readonly string[]{return names.filter(name=>dependencyNameKindV1(name,policy)==="generation");}

/** 🧭️ How the generation folder differs from its plan: `roster` when the generation and capsule names differ (what was added, what is gone), `identity` when it is another directory (device, inode or mode of the 7 field stamp), undefined when it holds the planned generations; a changed size or time of the folder alone (tombstones, the quarantine folder and OS metadata come and go) is no change. A pure function.
 * @see ./🧪️tests/🟦️.ts */
export function dependencyContainerChangeV1(names:readonly string[],stamp:string,plan:Readonly<{names:readonly string[];stamp:string}>,policy:DependencyForeignPolicyV1):Readonly<{kind:"roster";added:readonly string[];gone:readonly string[]}>|Readonly<{kind:"identity"}>|undefined{
 const current=dependencyGenerationNamesV1(names,policy);
 if(JSON.stringify(current)!==JSON.stringify(plan.names))return Object.freeze({kind:"roster" as const,added:Object.freeze(current.filter(name=>!plan.names.includes(name)).slice(0,8)),gone:Object.freeze(plan.names.filter(name=>!current.includes(name)).slice(0,8))});
 const identity=(value:string)=>JSON.stringify((JSON.parse(value) as string[]).slice(0,3));
 return identity(stamp)===identity(plan.stamp)?undefined:Object.freeze({kind:"identity" as const});
}

/** 🧹️ Makes the roster of the generation folder exact without a person: ignorable OS metadata is removed, any unknown name that stayed longer than `graceMilliseconds` (a younger one may be a writer's work in flight) is moved into the quarantine folder of the same folder as `<ms>-<name>` (the entry itself is moved, a link is never followed), and the quarantine is bounded to `maximum` entries and `retentionMilliseconds`, oldest collected first. Returns what it did, so the caller can tell it.
 * @see ./🧪️tests/🟦️.ts */
export function settleDependencyForeignNamesV1(input:Readonly<{directory:string;nowMs:number;policy:DependencyForeignPolicyV1;check?:()=>void}>):Readonly<{removed:readonly string[];quarantined:readonly string[];collected:number}>{
 const{directory,nowMs,policy}=input,check=input.check??(()=>undefined),removed:string[]=[],quarantined:string[]=[];
 let collected=0;
 for(const name of readdirSync(directory)){
  check();
  const kind=dependencyNameKindV1(name,policy);
  if(kind==="ignorable"){try{rmSync(join(directory,name),{recursive:true,force:true});removed.push(name);}catch{}}
  else if(kind==="unknown"){
   try{
    const facts=lstatSync(join(directory,name));
    if(nowMs-facts.mtimeMs<policy.graceMilliseconds)continue;
    mkdirSync(join(directory,policy.quarantine),{recursive:true,mode:0o700});
    renameSync(join(directory,name),join(directory,policy.quarantine,`${Math.floor(nowMs)}-${name.slice(0,64)}`));quarantined.push(name);
   }catch{}
  }
 }
 try{
  const root=join(directory,policy.quarantine),entries=readdirSync(root).sort((left,right)=>Number.parseInt(left,10)-Number.parseInt(right,10)||(left<right?-1:1)),excess=Math.max(0,entries.length-policy.maximum);
  entries.forEach((entry,index)=>{
   check();
   const stamp=Number.parseInt(entry,10);
   if(index<excess||Number.isFinite(stamp)&&nowMs-stamp>=policy.retentionMilliseconds){try{rmSync(join(root,entry),{recursive:true,force:true});collected++;}catch{}}
  });
 }catch{}
 return Object.freeze({removed:Object.freeze(removed),quarantined:Object.freeze(quarantined),collected});
}
