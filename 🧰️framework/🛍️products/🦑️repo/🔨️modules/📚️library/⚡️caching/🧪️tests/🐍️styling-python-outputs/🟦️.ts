import assert from"node:assert/strict";
import{readFileSync}from"node:fs";
import{createRequire}from"node:module";
import{join}from"node:path";
import{parse}from"@iarna/toml";
import{stylingPythonOutputV1,STYLING_PYTHON_POLICY_V1 as policy}from"@semio-tech/ui-styling/builder/python/contract";
/** 🐍️ Verifies the actual Python Source/output contract without fabricating a Native execution owner. */
export async function testStylingPythonSourceContractV1(workspace:string):Promise<void>{
 const require=createRequire(import.meta.url),fixture=JSON.parse(readFileSync(join(import.meta.dir,"../🧫️fixtures/🎨️styling-outputs/🐍️python/🔣️.json"),"utf8")),packagePath=join(fixture.owner,fixture.package),project=JSON.parse(readFileSync(join(workspace,packagePath,"📋️project.json"),"utf8")),manifestSource=readFileSync(join(workspace,packagePath,"pyproject.toml"),"utf8"),manifest=Bun.TOML.parse(manifestSource);
 assert.equal(project.targets.build.cache,true);assert.deepEqual(project.targets.build.outputs,fixture.outputs);assert.equal(project.targets.deps.cache,false);assert.ok(project.targets.build.dependsOn.includes("deps"));assert.ok(project.targets.build.dependsOn.includes("@semio-tech/ui-styling-tokens:generate"));assert.ok(project.targets.build.inputs.some((input:unknown)=>typeof input==="object"&&input!==null&&"dependentTasksOutputFiles"in input&&input.dependentTasksOutputFiles==="**/*"));assert.deepEqual(manifest,parse(manifestSource));
 const build=manifest["build-system"],groups=manifest["dependency-groups"];if(!build||typeof build!=="object"||!("requires"in build)||!groups||typeof groups!=="object"||!("build"in groups))throw Error("Actual Python backend contract absent");assert.deepEqual(build.requires,[fixture.backend]);assert.deepEqual(groups.build,[fixture.backend]);
 for(const mode of["prototype","release"]as const)assert.equal(stylingPythonOutputV1(join(workspace,".cache/b/p/m"),mode),join(workspace,".cache/b/p/m",policy.outputNamespace,mode));
 const compiler=join(workspace,fixture.owner,"🏗️builder/🐍️python/📜️script.ts"),bundle=await require("esbuild").build({entryPoints:[compiler],absWorkingDir:workspace,bundle:true,packages:"external",platform:"node",format:"esm",write:false,metafile:true});assert.ok(!Object.keys(bundle.metafile.inputs).some(path=>path.endsWith("🎨️styling/📦️packages/🐍️python/📜️script.ts")),"The actual wheel producer must not import its test consumer");
 assert.deepEqual(fixture.values,["#ff344f","#123456"]);assert.deepEqual(fixture.runtimeCases.map((row:{operation:string})=>row.operation),["cold","warm","restore","source-change","backend-failure"]);
}
