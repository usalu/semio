/** 🔗️ Command inputs retain workspace package resolution manifests alongside source modules. Independent Node resolution, esbuild, Bun and Python validate the neutral closure and digest cases in ./🔣️.json. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve,sep} from "node:path";
import corpus from "./🔣️.json";

const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any,python=testPythonV1();
await libraryBootstrap;
const strip=(paths:readonly string[])=>paths.map(path=>path.replace("{workspaceRoot}/","")).sort();
const relativeName=(root:string,file:string)=>relative(root,file).split(sep).join("/");
const build=(root:string)=>{
 for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 for(const[path,target]of Object.entries(corpus.links)){mkdirSync(dirname(join(root,path)),{recursive:true});const link=join(root,path);symlinkSync(process.platform==="win32"?resolve(dirname(link),target):target,link,process.platform==="win32"?"junction":"dir");}
};
const closure=async(root:string,entries:readonly string[]=[corpus.entry])=>strip(await cacheInternals.relativeScriptInputs(entries.map(entry=>join(root,entry)),root,createScriptInputCache(),true));
const digest=(root:string,files:readonly string[])=>{const hash=createHash("sha256");for(const name of files){hash.update(name).update("\0");try{hash.update("file\0").update(createHash("sha256").update(readFileSync(join(root,name))).digest("hex")).update("\0");}catch{hash.update("missing\0");}}return hash.digest("hex");};
const esbuildOracle=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map(path=>relativeName(root,resolve(root,path))).filter(path=>!path.split("/").includes("node_modules")).sort();};
const bunOracle=(root:string,entry:string)=>{const meta=join(root,"meta.json"),result=spawnSync("bun",["build","--target=bun","--outfile="+join(root,"bun-bundle.js"),"--metafile="+meta,join(root,entry)],{cwd:root,encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return Object.keys(JSON.parse(readFileSync(meta,"utf8")).inputs).map(path=>relativeName(root,resolve(root,path))).filter(path=>!path.split("/").includes("node_modules")).sort();};

const nodeResolutionOracle=(root:string,mode="import")=>{if(mode!=="import"&&mode!=="require")throw Error("Known runtime resolution mode required");const code=mode==="import"?"import{fileURLToPath}from'node:url';try{process.stdout.write(fileURLToPath(import.meta.resolve(process.argv[1])));}catch(error){process.stdout.write('!'+error.code);}":"import{createRequire}from'node:module';import{pathToFileURL}from'node:url';try{process.stdout.write(createRequire(pathToFileURL(process.cwd()+'/oracle.mjs')).resolve(process.argv[1]));}catch(error){process.stdout.write('!'+error.code);}";const result=spawnSync("node",["--input-type=module","-e",code,corpus.packageResolution.specifier],{cwd:dirname(join(root,corpus.entry)),encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return result.stdout.startsWith("!")?result.stdout:relativeName(root,result.stdout);};
const codeInputs=(paths:readonly string[])=>paths.filter(path=>path!==corpus.packageResolution.manifest).sort();

test("workspace package resolution follows its real manifest through the independent Node resolver",()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-resolver-"));try{build(root);expect(nodeResolutionOracle(root)).toBe(corpus.packageResolution.target);expect(corpus.closure).toContain(corpus.packageResolution.manifest);}finally{rmSync(root,{recursive:true,force:true});}});

test("command closure follows workspace package specifiers and agrees with esbuild and Bun's bundler",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-workspace-"));
 try{
  build(root);
  const got=await closure(root);
  expect(got).toEqual([...corpus.closure].sort());
  expect(await esbuildOracle(root,corpus.entry)).toEqual(codeInputs(got));
  expect(bunOracle(root,corpus.entry)).toEqual(codeInputs(got));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("literal module inputs a target declares contribute their own import closure",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-declared-"));
 try{
  build(root);
  const{declared}=corpus,target={inputs:declared.inputs,options:{command:declared.command}};
  const sources=await cacheInternals.targetScriptSources(target,root,createScriptInputCache(),declared.projectRoot);
  expect(strip(sources.closure)).toEqual([...declared.closure].sort());
  const bare=await cacheInternals.targetScriptSources({inputs:["default","^default"],options:{command:declared.command}},root,createScriptInputCache(),declared.projectRoot);
  expect(strip(bare.closure)).toEqual(strip(await cacheInternals.relativeScriptInputs([join(root,corpus.entry)],root,createScriptInputCache(),true)));
  expect(bunOracle(root,"tools/check.test.ts").every(path=>declared.closure.includes(path))).toBe(true);
  expect((await esbuildOracle(root,"tools/check.test.ts")).every(path=>declared.closure.includes(path))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("literal new URL references to existing files join the closure with the imports of the modules they name",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-url-"));
 try{
  build(root);
  const got=await closure(root,[corpus.url.entry]);
  expect(got).toEqual([...corpus.url.closure].sort());
  const bun=bunOracle(root,corpus.url.entry);
  expect(bun.filter(path=>/\.[cm]?[jt]sx?$/u.test(path)).every(path=>got.includes(path))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

for(const mutation of corpus.mutations)test("closure digest follows package resolution metadata: "+mutation.name,async()=>{
  const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-mutation-"));
  try{
   build(root);
   const before=await closure(root),beforeDigest=digest(root,before);
   writeFileSync(join(root,mutation.file),mutation.content);
   if("refused" in mutation&&mutation.refused){expect(nodeResolutionOracle(root)).toBe(mutation.resolverTarget);await expect(closure(root)).rejects.toThrow();return;}
   const after=await closure(root),afterDigest=digest(root,after);
   expect(after,mutation.name).toEqual(mutation.closure==="same"?before:[...mutation.closure].sort());
   expect(afterDigest===beforeDigest,mutation.name).toBe(mutation.digest==="same");
   expect(nodeResolutionOracle(root),mutation.name).toBe("resolverTarget" in mutation?mutation.resolverTarget:corpus.packageResolution.target);
   expect(await esbuildOracle(root,corpus.entry),mutation.name).toEqual(codeInputs(after));
   expect(bunOracle(root,corpus.entry),mutation.name).toEqual(codeInputs(after));
  }finally{rmSync(root,{recursive:true,force:true});}
});

test.skipIf(!python)("closure digest matches an independent Python hashlib computation of the same inventory",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-python-"));
 try{
  build(root);
  const files=await closure(root),result=Bun.spawnSync([python!,"-I","-c","import hashlib,json,os,sys\nd=json.loads(sys.stdin.read())\nh=hashlib.sha256()\nfor n in d['files']:\n    h.update(n.encode()+b'\\0')\n    p=os.path.join(d['root'],n)\n    if os.path.isfile(p):\n        h.update(b'file\\0'+hashlib.sha256(open(p,'rb').read()).hexdigest().encode()+b'\\0')\n    else:\n        h.update(b'missing\\0')\nprint(h.hexdigest())"],{stdin:new TextEncoder().encode(JSON.stringify({root,files})),stdout:"pipe",stderr:"pipe"});
  expect(result.exitCode,result.stderr.toString()).toBe(0);
  expect(result.stdout.toString().trim()).toBe(digest(root,files));
 }finally{rmSync(root,{recursive:true,force:true});}
});

test.skipIf(!process.env.SEMIO_SCRIPT_PROCESS_INVOCATION)("the library's own Nx closure digest moves with an imported module and stays with an unimported one",async()=>{
 const{receiveScriptProcessInvocation,createIssuedResourceControlV1}=await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts") as any,{sourceClosureDigest}=await import("../../../../📥️inputs/🧮️digest/🟨️.mjs") as any;
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-nx-"));
 try{
  build(root);
  await receiveScriptProcessInvocation(process.env,async(invocation:unknown)=>{
   const control=createIssuedResourceControlV1(invocation,"inputs-closure:digest",0),measure=async()=>sourceClosureDigest(root,{version:1,files:await closure(root)},control);
   const before=await measure();
   writeFileSync(join(root,"pkg/src/unused.ts"),"export const unused = 30;\n");
   expect(await measure()).toBe(before);
   writeFileSync(join(root,"pkg/src/deep.ts"),"export const deep = 20;\n");
   expect(await measure()).not.toBe(before);
  });
 }finally{rmSync(root,{recursive:true,force:true});}
});

for(const row of corpus.resolutionCases)test("conditional package closure agrees with actual runtime syntax: "+row.name,async()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-conditions-"));try{build(root);const manifest=corpus.mutations.find(row=>row.name==="conditional export follows import instead of require or erased types");if(!manifest)throw Error("Conditional manifest fixture required");writeFileSync(join(root,corpus.packageResolution.manifest),manifest.content);writeFileSync(join(root,corpus.entry),row.source);const got=await closure(root);expect(got).toEqual([...row.closure].sort());for(const[mode,target]of Object.entries(row.resolverTargets))expect(nodeResolutionOracle(root,mode)).toBe(target);expect(await esbuildOracle(root,corpus.entry)).toEqual("esbuildClosure" in row?row.esbuildClosure:codeInputs(got));expect(bunOracle(root,corpus.entry)).toEqual("bunClosure" in row?row.bunClosure:codeInputs(got));if("executionValue" in row)for(const runtime of ["node","bun"]){const result=spawnSync(runtime,["-e","import('node:url').then(({pathToFileURL})=>import(pathToFileURL(process.env.SEMIO_ORACLE_ENTRY).href)).then(module=>process.stdout.write(String(module.value)));"],{cwd:root,encoding:"utf8",timeout:10000,env:{...process.env,SEMIO_ORACLE_ENTRY:join(root,corpus.entry)}});expect(result.status,result.stderr).toBe(0);expect(result.stdout).toBe(String(row.executionValue));}}finally{rmSync(root,{recursive:true,force:true});}});

import compiled from "./compiled.json";
for(const row of compiled.cases)test("compiled public package retains its authenticated producer: "+row.name,async()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-compiled-"));try{build(root);const owner=join(root,"compiled");mkdirSync(owner);writeFileSync(join(owner,"package.json"),JSON.stringify({name:compiled.packageName,type:"module",exports:row.exports}));writeFileSync(join(owner,"📋️project.json"),JSON.stringify({name:"compiled-owner",metadata:{semio:{artifactPackage:"missingProducer" in row?undefined:compiled.producer}},targets:"missingTarget" in row?{}:{build:{}}}));writeFileSync(join(owner,"source.ts"),"export const value = 42;\n");if("missingSource" in row)rmSync(join(owner,"source.ts"));if("warm" in row){mkdirSync(join(owner,"dist"));writeFileSync(join(owner,"dist/🟦️.js"),"throw Error('Generated bytes must never join physical Source');");}mkdirSync(join(root,"node_modules/@fixture"),{recursive:true});symlinkSync(owner,join(root,"node_modules/@fixture/compiled"),process.platform==="win32"?"junction":"dir");const entry="self" in row?join(owner,"case.ts"):join(root,corpus.entry);writeFileSync(entry,row.source);const cache=createScriptInputCache();cache.generatedOutput=(file:string)=>cacheInternals.readBuildProducedCompileOutputV1(file,root,{}, {owners:"unknownOwner" in row?[]:[{name:"compiled-owner",root:"compiled",manifest:"compiled/📋️project.json"}]});const run=()=>cacheInternals.relativeScriptInputs([entry],root,cache,true);if(!row.accepted){await expect(run()).rejects.toThrow();return;}const got=strip(await run());expect(got).toContain("compiled/source.ts");expect(got).toContain("compiled/📋️project.json");expect(got).toContain("compiled/package.json");expect(got.some((name:string)=>name.includes("/dist/"))).toBe(false);const facts=[...cache.generatedImports.values()].flat() as any[];expect(facts.some(fact=>fact.project==="compiled-owner"&&fact.target==="compiled-owner:build")).toBe(true);const resolveExports=createRequire(import.meta.url)("resolve.exports");expect(resolveExports.resolve(JSON.parse(readFileSync(join(owner,"package.json"),"utf8")),compiled.packageName,{conditions:["node","require" in row?"require":"import"],require:"require" in row})).toEqual(["./dist/🟦️.js"]);}finally{rmSync(root,{recursive:true,force:true});}});

import bindingSchema from "../../../../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🧬️schema/🔣️.json";
import {readTypeScriptPackageProducerV1} from "../../../../../⚡️caching/📦️artifacts/🟦️typescript/🔗️binding/🏭️producer/🟦️.ts";
test("artifact producer declaration agrees with the independent schema validator",()=>{const Ajv=createRequire(import.meta.url)("ajv"),valid=new Ajv({strict:true}).compile(bindingSchema.$defs.Producer);for(const row of compiled.producers){expect(valid(row.value),row.name).toBe(row.accepted);const read=(file:string)=>file.endsWith("📋️project.json")?JSON.stringify({name:"fixture",metadata:{semio:{artifactPackage:row.value}},targets:{build:{}}}):"export const value=42;";if(row.accepted)expect(readTypeScriptPackageProducerV1("/fixture/pkg","/fixture",read)?.target).toBe("fixture:build");else expect(()=>readTypeScriptPackageProducerV1("/fixture/pkg","/fixture",read)).toThrow();}});

test("artifact Source containment is established before reading the authoring body",()=>{const reads:string[]=[],producer={...compiled.producer,source:"../../outside.ts"},read=(path:string)=>{reads.push(path);return path.endsWith("📋️project.json")?JSON.stringify({name:"fixture",metadata:{semio:{artifactPackage:producer}},targets:{build:{}}}):"export const value=42;";};expect(new URL(producer.source,"file:///fixture/pkg/").href).toBe("file:///outside.ts");expect(()=>readTypeScriptPackageProducerV1("/fixture/pkg","/fixture",read)).toThrow("workspace TypeScript artifact Source");expect(reads.every(path=>path.endsWith("📋️project.json"))).toBe(true);});
