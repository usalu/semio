/** 🔗️ Contract: a target's command closure follows imports through workspace package specifiers and the literal module inputs it declares, agrees with esbuild and Bun's own bundler on the same fixture, and its content digest moves exactly when an imported module changes. The scenario lives in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any,python=Bun.which("python3");
await libraryBootstrap;
const strip=(paths:readonly string[])=>paths.map(path=>path.replace("{workspaceRoot}/","")).sort();
const build=(root:string)=>{
 for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 for(const[path,target]of Object.entries(corpus.links)){mkdirSync(dirname(join(root,path)),{recursive:true});symlinkSync(target,join(root,path));}
};
const closure=async(root:string,entries:readonly string[]=[corpus.entry])=>strip(await cacheInternals.relativeScriptInputs(entries.map(entry=>join(root,entry)),root,createScriptInputCache(),true));
const digest=(root:string,files:readonly string[])=>{const hash=createHash("sha256");for(const name of files){hash.update(name).update("\0");try{hash.update("file\0").update(createHash("sha256").update(readFileSync(join(root,name))).digest("hex")).update("\0");}catch{hash.update("missing\0");}}return hash.digest("hex");};
const esbuildOracle=async(root:string,entry:string)=>{const result=await createRequire(import.meta.url)("esbuild").build({entryPoints:[join(root,entry)],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent",outfile:"out.js"});return Object.keys(result.metafile.inputs).map(path=>relative(root,resolve(root,path))).filter(path=>!path.split("/").includes("node_modules")).sort();};
const bunOracle=(root:string,entry:string)=>{const meta=join(root,"meta.json"),result=spawnSync("bun",["build","--target=bun","--outfile=/dev/null","--metafile="+meta,join(root,entry)],{cwd:root,encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return Object.keys(JSON.parse(readFileSync(meta,"utf8")).inputs).map(path=>relative(root,resolve(root,path))).filter(path=>!path.split("/").includes("node_modules")).sort();};

test("workspace closure fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);});

test("command closure follows workspace package specifiers and agrees with esbuild and Bun's bundler",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-workspace-"));
 try{
  build(root);
  const got=await closure(root);
  expect(got).toEqual([...corpus.closure].sort());
  expect(await esbuildOracle(root,corpus.entry)).toEqual(got);
  expect(bunOracle(root,corpus.entry)).toEqual(got);
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

test("closure digest moves exactly when an imported module changes",async()=>{
 for(const mutation of corpus.mutations){
  const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-mutation-"));
  try{
   build(root);
   const before=await closure(root),beforeDigest=digest(root,before);
   writeFileSync(join(root,mutation.file),mutation.content);
   const after=await closure(root),afterDigest=digest(root,after);
   expect(after,mutation.name).toEqual(mutation.closure==="same"?before:[...mutation.closure].sort());
   expect(afterDigest===beforeDigest,mutation.name).toBe(mutation.digest==="same");
  }finally{rmSync(root,{recursive:true,force:true});}
 }
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
