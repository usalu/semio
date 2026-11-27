import {relative} from "node:path";
import {renderCargoProfileConfigurationV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🟦️.ts";
import {offloadDeclaredThreadsV1,readFamilyPeakV1,readUnitPeakV1,readUnitPeaksV1,writeUnitPeakV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";
import {offloadUnitPeakMegabytesV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/📐️policy/🟦️.ts";
import {unitPeakKeyV1,unitProfileArgumentsV1,unitProfileKeyV1} from "../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🪪️profile/🟦️.ts";

/** 🪪️ The peak evidence keys of the crates of a component pass plan: `keyOf` is the name under which the unit cache keeps a crate's peak (`unitPeakKeyV1`), `family` the one profile key of the plan's passes when they all compile with the same profile. */
export type ComponentPeakProfilesV1=Readonly<{keyOf:(crate:string)=>string;family:string|undefined}>;
type PassV1=Readonly<{manifest:string;target:string;packages:readonly Readonly<{name:string}>[]}>;

/** 🪪️ Resolves the Cargo profile that every pass of the plan compiles with (the profile policy for its target and package composition, so the lean release compositions get their own) and keys each crate's peak evidence by the identity of the unit cache (`unitProfileKeyV1` over the rustc arguments of those settings): one identity function, no second parser.
 * @see ../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🪪️profile/🟦️.ts */
export function componentPeakProfilesV1(repoRoot:string,mode:"prototype"|"release",passes:readonly PassV1[]):ComponentPeakProfilesV1{
 const keys=new Map<string,string>(),profiles=new Set<string>();
 for(const pass of passes){
  const rendered=renderCargoProfileConfigurationV1({version:1,mode,target:pass.target,producer:"artifact",operation:"build",compositions:pass.packages.map(row=>row.name)},relative(repoRoot,pass.manifest).replaceAll("\\","/"),"repository"),profile=unitProfileKeyV1(unitProfileArgumentsV1({...rendered.plan.target,target:pass.target}));
  profiles.add(profile);
  for(const row of pass.packages){const crate=row.name.replaceAll("-","_");keys.set(crate,unitPeakKeyV1(crate,profile));}
 }
 return Object.freeze({keyOf:crate=>keys.get(crate)??unitPeakKeyV1(crate,"unresolved"),family:profiles.size===1?[...profiles][0]:undefined});
}

/** 🏋️ The peak a component pass is scheduled with: first the recorded final peaks seed the crates that have no evidence for their profile, then the median of the crates' evidence at the declared codegen threads, else the family estimate of the same profile. */
export function componentPassPeakV1(input:Readonly<{root:string|undefined;crates:readonly string[];profiles:ComponentPeakProfilesV1;seeds:Iterable<readonly[string,number]>}>):number{
 const root=input.root;if(root===undefined)return offloadUnitPeakMegabytesV1(undefined,["cdylib"],"wasm32-wasip2",undefined);
 for(const[crate,megabytes]of input.seeds){const key=input.profiles.keyOf(crate);if(Object.keys(readUnitPeaksV1(root,key)).length===0)writeUnitPeakV1(root,key,megabytes,1);}
 const measured=input.crates.flatMap(crate=>readUnitPeakV1(root,input.profiles.keyOf(crate),offloadDeclaredThreadsV1(true))??[]).sort((left,right)=>left-right);
 return measured.length>0?measured[Math.floor(measured.length/2)]!:offloadUnitPeakMegabytesV1(undefined,["cdylib"],"wasm32-wasip2",input.profiles.family===undefined?undefined:readFamilyPeakV1(root,input.profiles.family));
}
