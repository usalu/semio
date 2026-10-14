import {test,expect} from "bun:test";
import {mkdirSync,writeFileSync,readFileSync,rmSync} from "node:fs";
import {resolve,join,dirname} from "node:path";
import {randomUUID} from "node:crypto";
import Ajv from "ajv";
import {build} from "esbuild";
import {parse} from "@iarna/toml";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {bindBuildCommandV1,literalCommandTokensV1} from "../🟨️.mjs";
import {cacheInternals,createScriptInputCache} from "../../🕸️graph/📥️inference/🟨️.mjs";

const oracle=new Ajv({strict:true}),boundShape=oracle.compile({$defs:schema.$defs,$ref:"#/$defs/BoundBuildCommandV1"});
for(const row of fixture.commands)test("portable outer command binding: "+row.name,()=>{const root=resolve("command-owner-root"),bound=bindBuildCommandV1(row.command,root,row.cwd,"owner",row.arguments);expect(boundShape(bound)).toBe(true);expect(bound.script).toBe(row.entry===null?null:resolve(root,row.entry));expect(literalCommandTokensV1(bound.command)).toEqual([bound.program,...bound.arguments]);});
for(const row of fixture.refusals)test("portable outer command refusal: "+row.name,()=>{expect(()=>bindBuildCommandV1(row.command,resolve("command-owner-root"),row.cwd,"owner",row.arguments)).toThrow();});
for(const row of fixture.sourceCases)test("declared child Source closure: "+row.name,async()=>{
 const parent=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!parent)throw Error("Original source-only artifact directory required");const root=join(resolve(parent),"cmd-"+randomUUID());mkdirSync(root,{recursive:true});
 try{
  for(const[path,source]of Object.entries({...fixture.source.files,"owner/execution.json":JSON.stringify(fixture.source.declaration)})){if(path===row.missing)continue;const target=join(root,path);if(target.length>256)throw Error("Fixture Source path exceeds portable capacity");mkdirSync(dirname(target),{recursive:true});writeFileSync(target,source);}
  const target={options:{cwd:fixture.source.cwd,command:fixture.source.command,forwardAllArgs:false}},collect=()=>cacheInternals.targetScriptSources(target,root,createScriptInputCache(),"owner","owner/Cargo.toml"),reference=async()=>{const manifest=parse(readFileSync(join(root,"owner/Cargo.toml"),"utf8")),reference=(manifest.package as any).metadata.semio["cargo-execution"];JSON.parse(readFileSync(resolve(root,"owner",reference),"utf8"));return build({absWorkingDir:root,entryPoints:["wrapper/📜️script.ts","owner/📜️script.ts"],outdir:join(root,"output"),bundle:true,write:false,metafile:true,platform:"neutral",logLevel:"silent"});};
  if(!row.accepted){await expect(collect()).rejects.toThrow();await expect(reference()).rejects.toThrow();return;}
  const actual=await collect();if(!actual.closure)throw Error("Declared original command Source closure required");expect(actual.fallback).toBe(false);expect(actual.closure).toEqual(fixture.source.required.map(path=>"{workspaceRoot}/"+path).sort());expect(actual.closure).not.toContain("{workspaceRoot}/📜️script.ts");
  const independent=await reference(),expected=Object.keys(independent.metafile!.inputs).map(path=>path.replaceAll("\\","/")).sort(),compiled=actual.closure.map((path:string)=>path.slice("{workspaceRoot}/".length)).filter((path:string)=>path.endsWith(".ts")).sort();expect(compiled).toEqual(expected);
 }finally{rmSync(root,{recursive:true,force:true});}
});
