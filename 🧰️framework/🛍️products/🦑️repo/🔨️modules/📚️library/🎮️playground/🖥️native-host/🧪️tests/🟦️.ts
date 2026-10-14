import assert from "node:assert/strict";
import { readFileSync,lstatSync,mkdirSync,mkdtempSync,writeFileSync,renameSync,rmSync } from "node:fs";
import { resolve,dirname,join } from "node:path";
import {createRequire} from "node:module";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import {nativeHostFilesystemViewV1} from "../🟦️.ts";
import { admitPlaygroundNativeHostV1, parsePlaygroundNativeHostV1, nativeHostSourceFactsV1,nativeHostArtifactPathV1,declaredPlaygroundHostInputPathsV1,nativeHostScriptAdmissionV1 } from "../🟨️.mjs";

/** 🖥 Checks an open owner producer inventory against independent JSON and TOML oracles. */
export async function nativeHostContractLaws(repoRoot:string):Promise<void> {
  const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"./🧫️fixtures/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")));
  const facts={package:fixture.owner.package,project:fixture.owner.project,binaries:[fixture.owner.binary],targets:["native-build"],modes:{"native-build":["prototype","release"] as const}};
  for (const row of fixture.cases) {
    if(row.input===null) {assert.equal(admitPlaygroundNativeHostV1(undefined,()=>{throw new Error("unselected owner read")}),undefined);continue;}
    const invoke=()=>admitPlaygroundNativeHostV1(row.input,()=>facts);
    if(row.accepted) {assert(validate(row.input));assert.deepEqual(invoke(),row.input);}
    else assert.throws(invoke);
  }
  const source=`nativeHost = { cratePath = "${fixture.owner.cratePath}", project = "${fixture.owner.project}", package = "${fixture.owner.package}", binary = "${fixture.owner.binary}", target = "${fixture.owner.target}" }`;
  assert.deepEqual(parsePlaygroundNativeHostV1(source),(toml.parse(source) as any).nativeHost);
  for(const invalid of [source+"\n"+source,source.replace('binary = "alpha-native"','binary = "alpha-native", binary = "other"'),source.replace('binary = "alpha-native"','unknown = "alpha-native"')]) assert.throws(()=>parsePlaygroundNativeHostV1(invalid));
  for(const changed of [{...facts,modes:{"native-build":["prototype"] as const}},{...facts,binaries:[]},{...facts,project:"foreign"}]) assert.throws(()=>admitPlaygroundNativeHostV1(fixture.owner,()=>changed));
  const root=fixture.owner.cratePath, sources=new Map<string,string>([[`${root}/Cargo.toml`, '[package]\nname = "alpha-native"\n\n[[bin]]\nname = "alpha-native"\n'],[`${root}/📋️project.json`, JSON.stringify({root,name:fixture.owner.project,targets:{"native-build":{executor:"repo:script",outputs:["{projectRoot}/dist/native-{options.env.SEMIO_BUILD_MODE}"],configurations:{prototype:{env:{SEMIO_BUILD_MODE:"prototype"}},release:{env:{SEMIO_BUILD_MODE:"release"}}},options:{cwd:root,command:"bun ./📜️script.ts build"}}}})],[`${root}/📜️script.ts`, "export {}"]]);
  const directories=new Set(root.split("/").map((_:string,index:number)=>root.split("/").slice(0,index+1).join("/")));
  const view={admitScript:(path:string)=>nativeHostScriptAdmissionV1(sources.get(path)!),kind:(path:string)=>sources.has(path)?"file":directories.has(path)?"directory":null,readText:(path:string)=>sources.get(path)!};
  assert.deepEqual(nativeHostSourceFactsV1(root,view),{...facts,outputs:{"native-build":{prototype:["{projectRoot}/dist/native-prototype"],release:["{projectRoot}/dist/native-release"]}}});
  for(const bad of [`${root}/Cargo.toml`,root,"owners"]) assert.throws(()=>nativeHostSourceFactsV1(root,{...view,kind:(path:string)=>path===bad?"symlink":view.kind(path)}));
  const script=`${root}/📜️script.ts`,admissionSchema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));admissionSchema.$ref="#/$defs/NativeHostScriptAdmissionV1";const validateAdmission=new Ajv({strict:true}).compile(admissionSchema),baseline=nativeHostSourceFactsV1(root,view);
  for(const row of fixture.scriptAdmission.vectors){const text=row.character.repeat(row.count),result=nativeHostScriptAdmissionV1(text);assert(validateAdmission(result));assert.equal(result.withinLimit,Buffer.byteLength(text,"utf8")<=fixture.scriptAdmission.maximumBytes,row.id);assert.equal(result.withinLimit,row.withinLimit,row.id);if(row.withinLimit)assert.deepEqual(nativeHostSourceFactsV1(root,{...view,admitScript:()=>result}),baseline);else assert.throws(()=>nativeHostSourceFactsV1(root,{...view,admitScript:()=>result}),/oversized/);}
  for(const result of fixture.scriptAdmission.invalidResults){assert.equal(validateAdmission(result),false);assert.throws(()=>nativeHostSourceFactsV1(root,{...view,admitScript:()=>result}));}
  for(const text of [fixture.scriptAdmission.before,fixture.scriptAdmission.bodyChanged]){sources.set(script,text);assert.deepEqual(nativeHostSourceFactsV1(root,{...view,readText:(path:string)=>{assert.notEqual(path,script);return view.readText(path);}}),baseline);}
  for(const kind of [null,"symlink","directory"] as const)assert.throws(()=>nativeHostSourceFactsV1(root,{...view,kind:(path:string)=>path===script?kind:view.kind(path)}));
  assert.throws(()=>nativeHostSourceFactsV1(root,{...view,admitScript:()=>{throw Error(fixture.scriptAdmission.readError);}}),new RegExp(fixture.scriptAdmission.readError));
  const actualRoot=repoRoot, actualHost={cratePath:"✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/📦️packages/🦀️rust",project:"@semio-tech/s-gis-services-native",package:"semio-s-dev-gis-services",binary:"semio-s-gis-services-native",target:"native-build"};
  const actualView={admitScript:(path:string)=>nativeHostScriptAdmissionV1(readFileSync(resolve(actualRoot,path),"utf8")),kind:(path:string)=>{try {const stat=lstatSync(resolve(actualRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch{return null;}},readText:(path:string)=>readFileSync(resolve(actualRoot,path),"utf8")};
  assert.deepEqual(admitPlaygroundNativeHostV1(actualHost,(owner:string)=>nativeHostSourceFactsV1(owner,actualView)),actualHost);
  const actualFacts=nativeHostSourceFactsV1(actualHost.cratePath,actualView);
  const getOutputs=createRequire(import.meta.url)("nx/src/tasks-runner/utils").getOutputs,project=JSON.parse(actualView.readText(`${actualHost.cratePath}/📋️project.json`));for(const row of fixture.modes){const outputs=getOutputs({[actualHost.project]:{data:project}},{project:actualHost.project,target:actualHost.target,configuration:row.mode},{});assert.deepEqual(outputs,[`${actualHost.cratePath}/${row.directory}`]);assert.equal(nativeHostArtifactPathV1(actualHost,row.mode,actualFacts,row.platform),`${outputs[0]}/${actualHost.binary}${row.suffix}`);}
  const factsSchema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8"));factsSchema.$ref="#/$defs/NativeHostSourceFactsV1";assert(new Ajv({strict:true}).compile(factsSchema)(actualFacts));
  const actualMcpHost={...actualHost,binary:"semio-s-gis-services-mcp",target:"mcp-build"};
  assert.deepEqual(admitPlaygroundNativeHostV1(actualMcpHost,(owner:string)=>nativeHostSourceFactsV1(owner,actualView)),actualMcpHost);
  assert.equal(nativeHostArtifactPathV1(actualMcpHost,"prototype",actualFacts,"linux"),`${actualMcpHost.cratePath}/dist/mcp-prototype/${actualMcpHost.binary}`);
  assert.equal(nativeHostArtifactPathV1(actualMcpHost,"release",actualFacts,"win32"),`${actualMcpHost.cratePath}/dist/mcp-release/${actualMcpHost.binary}.exe`);
  const manifest=`[[package.metadata.semio.playground]]\n${source}\nmcpHost = ${source.slice(source.indexOf("{"))}\n`;
  const parsed=toml.parse(manifest) as any;
  for(const field of ["nativeHost","mcpHost"] as const) assert.deepEqual(parsePlaygroundNativeHostV1(manifest,field),parsed.package.metadata.semio.playground[0][field]);
  const expectedInputs=[...sources.keys()].sort((left,right)=>Buffer.from(left).compare(Buffer.from(right)));
  assert.deepEqual(declaredPlaygroundHostInputPathsV1(manifest,view),expectedInputs);
  assert.deepEqual(declaredPlaygroundHostInputPathsV1("[[package.metadata.semio.playground]]\nvariant = \"empty\"",{...view,readText:()=>{throw new Error("unselected host read");},admitScript:()=>{throw new Error("unselected script admission");}}),[]);
  assert.throws(()=>declaredPlaygroundHostInputPathsV1(manifest,{...view,kind:(path:string)=>path.endsWith("📜️script.ts")?null:view.kind(path)}));
  assert.equal(nativeHostArtifactPathV1(actualHost,"prototype",actualFacts,"linux"),`${actualHost.cratePath}/dist/native-prototype/${actualHost.binary}`);
  assert.equal(nativeHostArtifactPathV1(actualHost,"release",actualFacts,"win32"),`${actualHost.cratePath}/dist/native-release/${actualHost.binary}.exe`);
  for(const outputs of [[],["{workspaceRoot}/foreign"],["{projectRoot}/../foreign"],["{projectRoot}/dist/a","{projectRoot}/dist/b"]]) assert.throws(()=>nativeHostArtifactPathV1(actualHost,"prototype",{...actualFacts,outputs:{"native-build":{prototype:outputs,release:[]}}},"linux"));
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Native script admission laws require caller-owned ticket output");mkdirSync(output,{recursive:true});const temporary=mkdtempSync(join(output,"host-"));
  try{for(const [path,source] of sources){mkdirSync(dirname(join(temporary,path)),{recursive:true});writeFileSync(join(temporary,path),source);}const physical=nativeHostFilesystemViewV1(temporary),scriptPath=resolve(temporary,script),renamed=scriptPath+".renamed";assert.deepEqual(nativeHostSourceFactsV1(root,physical),baseline);renameSync(scriptPath,renamed);assert.throws(()=>nativeHostSourceFactsV1(root,physical),/present and no-follow/);renameSync(renamed,scriptPath);writeFileSync(scriptPath,Buffer.from(fixture.scriptAdmission.invalidUtf8));assert.throws(()=>physical.admitScript(script));writeFileSync(scriptPath,fixture.scriptAdmission.before);
   const {registerSourceAnalysisV1,observedSourceAnalysisV1,captureObservedSourceInputsV1,observationsHold}=await import("../../../🏗️build/👁️observation/🟨️.mjs"),provider=registerSourceAnalysisV1({id:"fixture:native-script-admission:v1",revision:"a".repeat(64),analyze:(_path:string,bytes:Buffer)=>nativeHostScriptAdmissionV1(bytes.toString("utf8"))}),semantic={...physical,admitScript:(path:string)=>observedSourceAnalysisV1(provider,resolve(temporary,path))};
   const captured=await captureObservedSourceInputsV1(()=>nativeHostSourceFactsV1(root,semantic));assert.deepEqual(captured.value,baseline);assert(captured.observations.some(row=>row.path===scriptPath&&row.kind==="analysis"));assert(!captured.observations.some(row=>row.path===scriptPath&&row.kind==="file"));writeFileSync(scriptPath,fixture.scriptAdmission.bodyChanged);assert(await observationsHold(captured.observations));writeFileSync(scriptPath,"a".repeat(fixture.scriptAdmission.maximumBytes+1));assert.equal(await observationsHold(captured.observations),false);assert.throws(()=>nativeHostSourceFactsV1(root,semantic),/oversized/);rmSync(scriptPath);assert.equal(await observationsHold(captured.observations),false);assert.throws(()=>nativeHostSourceFactsV1(root,semantic),/present and no-follow/);console.log("[DEBUG] Native script admission exact filesystem rename/UTF8/bound plus semantic rewrite/oversize/absence passed");
  }finally{rmSync(temporary,{recursive:true,force:true});}
  console.log(`native-host-contract: vectors=${fixture.cases.length} ajv=1 toml=1 absent-read=0 malformed=3 producer=3 source=5 actual=2 profiles=4 outputs=4 input-closure=3 passed`);
}
