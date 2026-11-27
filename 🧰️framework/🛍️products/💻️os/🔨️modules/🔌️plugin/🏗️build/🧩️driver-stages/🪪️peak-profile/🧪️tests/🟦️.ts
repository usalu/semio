/** 🪪️ Contract: the component pass schedules with peak evidence of the build profile it compiles with, keyed by the unit cache's own profile identity; evidence of another profile (a stale fat-release peak) is never read.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import sourceSchema from "../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json" with {type:"json"};
import {offloadDeclaredThreadsV1,readUnitPeakV1,writeUnitPeakV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🚦️admission/🟦️.ts";
import {unitPeakKeyV1,unitProfileArgumentsV1,unitProfileKeyV1} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🚀️offload/🪪️profile/🟦️.ts";
import {componentPassPeakV1,componentPeakProfilesV1} from "../🟦️.ts";

const lean=sourceSchema.definitions.PolicyProtocol.const.leanCompositions.release[0]!,settings=sourceSchema.definitions.PolicyProtocol.const.settings,pass=(packages:readonly string[])=>({manifest:"/repo/Cargo.toml",target:"wasm32-wasip2",profile:"release",packages:packages.map(name=>({name}))}) as never;

test("the profile of a component crate is the key of the settings its pass resolves, lean compositions included",()=>{
 const profiles=componentPeakProfilesV1("/repo","release",[pass([lean]),pass(["semio-hub-other-component"])]);
 const leanKey=unitProfileKeyV1(unitProfileArgumentsV1({...settings["component-lean"],optLevel:settings["component-lean"].optLevel,target:"wasm32-wasip2"} as never)),finalKey=unitProfileKeyV1(unitProfileArgumentsV1({...settings["release-wasm"],target:"wasm32-wasip2"} as never));
 expect(profiles.keyOf(lean.replaceAll("-","_"))).toBe(unitPeakKeyV1(lean.replaceAll("-","_"),leanKey));
 expect(profiles.keyOf("semio_hub_other_component")).toBe(unitPeakKeyV1("semio_hub_other_component",finalKey));
 expect(leanKey).not.toBe(finalKey);
});

test("a stale peak of another profile is no evidence for the component pass, evidence of its own profile is",()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-peak-profile-")),crate=lean.replaceAll("-","_");
 try{
  const profiles=componentPeakProfilesV1("/repo","release",[pass([lean])]),fat=unitPeakKeyV1(crate,unitProfileKeyV1(["-C","opt-level=3","-C","lto=fat","-C","codegen-units=1"]));
  writeUnitPeakV1(root,fat,5784,1);
  const unknown=componentPassPeakV1({root,crates:[crate],profiles,seeds:[]});
  expect(unknown).toBe(componentPassPeakV1({root:undefined,crates:[crate],profiles,seeds:[]}));
  expect(readUnitPeakV1(root,profiles.keyOf(crate),offloadDeclaredThreadsV1(true))).toBeUndefined();
  componentPassPeakV1({root,crates:[crate],profiles,seeds:[[crate,1800]]});
  expect(readUnitPeakV1(root,profiles.keyOf(crate),1)).toBe(1800);
  expect(readUnitPeakV1(root,fat,1)).toBe(5784);
  writeUnitPeakV1(root,profiles.keyOf(crate),1900,offloadDeclaredThreadsV1(true));
  expect(componentPassPeakV1({root,crates:[crate],profiles,seeds:[]})).toBe(readUnitPeakV1(root,profiles.keyOf(crate),offloadDeclaredThreadsV1(true))!);
 }finally{rmSync(root,{recursive:true,force:true});}
});
