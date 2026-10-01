import assert from "node:assert/strict";
import { readFileSync,lstatSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";
import * as toml from "@iarna/toml";
import { admitPlaygroundNativeHostV1, parsePlaygroundNativeHostV1, nativeHostSourceFactsV1,nativeHostArtifactPathV1,declaredPlaygroundHostInputPathsV1 } from "../🟨️.mjs";

/** 🖥 Checks an open owner producer inventory against independent JSON and TOML oracles. */
export function nativeHostContractLaws(repoRoot:string):void {
  const fixture=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8"));
  const validate=new Ajv({strict:true}).compile(JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")));
  const facts={package:fixture.owner.package,project:fixture.owner.project,binaries:[fixture.owner.binary],targets:["native-build","native-build-release"]};
  for (const row of fixture.cases) {
    if(row.input===null) {assert.equal(admitPlaygroundNativeHostV1(undefined,()=>{throw new Error("unselected owner read")}),undefined);continue;}
    const invoke=()=>admitPlaygroundNativeHostV1(row.input,()=>facts);
    if(row.accepted) {assert(validate(row.input));assert.deepEqual(invoke(),row.input);}
    else assert.throws(invoke);
  }
  const source=`nativeHost = { cratePath = "${fixture.owner.cratePath}", project = "${fixture.owner.project}", package = "${fixture.owner.package}", binary = "${fixture.owner.binary}", target = "${fixture.owner.target}" }`;
  assert.deepEqual(parsePlaygroundNativeHostV1(source),(toml.parse(source) as any).nativeHost);
  for(const invalid of [source+"\n"+source,source.replace('binary = "alpha-native"','binary = "alpha-native", binary = "other"'),source.replace('binary = "alpha-native"','unknown = "alpha-native"')]) assert.throws(()=>parsePlaygroundNativeHostV1(invalid));
  for(const changed of [{...facts,targets:["native-build"]},{...facts,binaries:[]},{...facts,project:"foreign"}]) assert.throws(()=>admitPlaygroundNativeHostV1(fixture.owner,()=>changed));
  const root=fixture.owner.cratePath, sources=new Map<string,string>([[`${root}/Cargo.toml`, '[package]\nname = "alpha-native"\n\n[[bin]]\nname = "alpha-native"\n'],[`${root}/📋️project.json`, JSON.stringify({root,name:fixture.owner.project,targets:{"native-build":{executor:"nx:run-commands",options:{cwd:root,command:"bun ./📜️script.ts build dev"}},"native-build-release":{executor:"nx:run-commands",options:{cwd:root,command:"bun ./📜️script.ts build release"}}}})],[`${root}/📜️script.ts`, "export {}"]]);
  const directories=new Set(root.split("/").map((_:string,index:number)=>root.split("/").slice(0,index+1).join("/")));
  const view={kind:(path:string)=>sources.has(path)?"file":directories.has(path)?"directory":null,readText:(path:string)=>sources.get(path)!};
  assert.deepEqual(nativeHostSourceFactsV1(root,view),{...facts,outputs:{"native-build":undefined,"native-build-release":undefined}});
  for(const bad of [`${root}/Cargo.toml`,root,"owners"]) assert.throws(()=>nativeHostSourceFactsV1(root,{...view,kind:(path:string)=>path===bad?"symlink":view.kind(path)}));
  assert.throws(()=>nativeHostSourceFactsV1(root,{...view,readText:(path:string)=>path.endsWith("📜️script.ts")?" ".repeat(1024*1024+1):view.readText(path)}));
  const actualRoot=repoRoot, actualHost={cratePath:"✏️s/🧑‍💻dev/💡️services/📦️packages/🦀️rust",project:"@semio-tech/s-services-native",package:"semio-s-dev-services",binary:"semio-s-services-native",target:"native-build"};
  const actualView={kind:(path:string)=>{try {const stat=lstatSync(resolve(actualRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch{return null;}},readText:(path:string)=>readFileSync(resolve(actualRoot,path),"utf8")};
  assert.deepEqual(admitPlaygroundNativeHostV1(actualHost,(owner:string)=>nativeHostSourceFactsV1(owner,actualView)),actualHost);
  const actualFacts=nativeHostSourceFactsV1(actualHost.cratePath,actualView);
  const actualMcpHost={...actualHost,binary:"semio-s-services-mcp",target:"mcp-build"};
  assert.deepEqual(admitPlaygroundNativeHostV1(actualMcpHost,(owner:string)=>nativeHostSourceFactsV1(owner,actualView)),actualMcpHost);
  assert.equal(nativeHostArtifactPathV1(actualMcpHost,"dev",actualFacts,"linux"),`${actualMcpHost.cratePath}/dist/mcp-dev/${actualMcpHost.binary}`);
  assert.equal(nativeHostArtifactPathV1(actualMcpHost,"release",actualFacts,"win32"),`${actualMcpHost.cratePath}/dist/mcp-release/${actualMcpHost.binary}.exe`);
  const manifest=`[[package.metadata.semio.playground]]\n${source}\nmcpHost = ${source.slice(source.indexOf("{"))}\n`;
  const parsed=toml.parse(manifest) as any;
  for(const field of ["nativeHost","mcpHost"] as const) assert.deepEqual(parsePlaygroundNativeHostV1(manifest,field),parsed.package.metadata.semio.playground[0][field]);
  const expectedInputs=[...sources.keys()].sort((left,right)=>Buffer.from(left).compare(Buffer.from(right)));
  assert.deepEqual(declaredPlaygroundHostInputPathsV1(manifest,view),expectedInputs);
  assert.deepEqual(declaredPlaygroundHostInputPathsV1("[[package.metadata.semio.playground]]\nvariant = \"empty\"",{...view,readText:()=>{throw new Error("unselected host read");}}),[]);
  assert.throws(()=>declaredPlaygroundHostInputPathsV1(manifest,{...view,kind:(path:string)=>path.endsWith("📜️script.ts")?null:view.kind(path)}));
  assert.equal(nativeHostArtifactPathV1(actualHost,"dev",actualFacts,"linux"),`${actualHost.cratePath}/dist/native-dev/${actualHost.binary}`);
  assert.equal(nativeHostArtifactPathV1(actualHost,"release",actualFacts,"win32"),`${actualHost.cratePath}/dist/native-release/${actualHost.binary}.exe`);
  for(const outputs of [[],["{workspaceRoot}/foreign"],["{projectRoot}/../foreign"],["{projectRoot}/dist/a","{projectRoot}/dist/b"]]) assert.throws(()=>nativeHostArtifactPathV1(actualHost,"dev",{...actualFacts,outputs:{"native-build":outputs}},"linux"));
  console.log(`native-host-contract: vectors=${fixture.cases.length} ajv=1 toml=1 absent-read=0 malformed=3 producer=3 source=5 actual=2 profiles=4 outputs=4 input-closure=3 passed`);
}
