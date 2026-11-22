import {expect,test} from "bun:test";
import * as fs from "node:fs";
import {dirname,join,posix} from "node:path";
import picomatch from "picomatch";
import {loadProductFunctionsSyncV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import cachePolicy from "../../../../../../⚡️caching/🔣️policy.json";
import {createBuildInputPathMatcherV1} from "../../../../../📥️inputs/🟨️.mjs";
import {INFERENCE_LIMITS_V1} from "../../../🧬️schema/🟨️.mjs";

const generated=cachePolicy.generatedDirectories as string[],project="people/proof",patterns=[project+"/🧘️lean/**/*",project+"/**/*",project+"/🧘️lean/*.lean"];

function populate(root:string,generatedFiles:number){
 const authored=["🧘️lean/Main.lean","🧘️lean/lakefile.toml","🧘️lean/Sub/Deep.lean","📜️script.ts","📋️project.json"];
 for(const path of authored){fs.mkdirSync(join(root,project,dirname(path)),{recursive:true});fs.writeFileSync(join(root,project,path),"");}
 for(let index=0;index<generatedFiles;index++){
  const directory=join(root,project,"🧘️lean",".lake","packages","mathlib","Mathlib","Part"+index%40,"Nest"+index%7);
  fs.mkdirSync(directory,{recursive:true});fs.writeFileSync(join(directory,"File"+index+".lean"),"");
 }
}

async function expand(root:string){
 let directoryReads=0;
 const execute=loadProductFunctionsSyncV1(new URL("../../🟨️.mjs",import.meta.url),["readBuildSourceInputMatchesV1"],{
  INFERENCE_LIMITS_V1,join,dirname,buildPath:(path:string)=>path.replaceAll("\\","/"),POLICY:{generatedDirectories:generated},
  isOpaqueSourcePathV1:()=>false,EXAMPLE_COLLECTIONS:[],WORKSPACE_OWNERSHIP:{collections:{moduleMember:"member"}},
  lstatSync:fs.lstatSync,existsSync:fs.existsSync,readdirSync:(path:string,options:object)=>{directoryReads++;return fs.readdirSync(path,options as never);},
  yieldInferenceV1:async()=>{},loadedProjectInputs:{createBuildInputPathMatcherV1},
 }).readBuildSourceInputMatchesV1;
 const result=await execute([{target:"build-lean",compile:true,patterns}],root,{});
 return {matched:[...result.matched].sort(),directoryReads};
}

test("a generated directory of any size changes neither the expanded inputs nor the number of directories read",async()=>{
 expect(generated).toContain(".lake");
 const scratch=testScratchRootV1("generated-trees"),plain=fs.mkdtempSync(join(scratch,"plain-")),heavy=fs.mkdtempSync(join(scratch,"heavy-"));
 try{
  populate(plain,0);populate(heavy,4000);
  const without=await expand(plain),withGenerated=await expand(heavy);
  expect(withGenerated.matched).toEqual(without.matched);
  expect(withGenerated.directoryReads).toBe(without.directoryReads);
  const listed=(fs.readdirSync(join(heavy,project),{recursive:true,withFileTypes:true}) as fs.Dirent[]).filter(entry=>entry.isFile()).map(entry=>posix.join(entry.parentPath.slice(heavy.length+1).split("\\").join("/"),entry.name)).filter(path=>!path.split("/").some(part=>generated.includes(part)));
  const oracle=listed.filter(path=>patterns.some(pattern=>picomatch.isMatch(path,pattern))).sort();
  expect(withGenerated.matched).toEqual(oracle);
  expect(withGenerated.matched.some(path=>path.includes("/.lake/"))).toBe(false);
 }finally{fs.rmSync(plain,{recursive:true,force:true});fs.rmSync(heavy,{recursive:true,force:true});}
});
