/** 🧮️ Contract: a target input that names a module the entry closure already compiled adds nothing, one outside it (or one the closure only hashed as a resource) joins with its own imports, and the closure equals the one derived the old way (every declared module as an extra entry) and contains what Bun's bundler reaches from each of them. The cases live in `./🔣️.json` as language-agnostic data. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const inference=await import("../../🟨️.mjs"),{cacheInternals,createScriptInputCache,libraryBootstrap}=inference as any;
await libraryBootstrap;
const strip=(paths:readonly string[])=>paths.map(path=>path.replace("{workspaceRoot}/","")).sort();
const build=(root:string)=>{for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}};
const bunOracle=(root:string,entry:string)=>{const meta=join(root,"meta.json"),result=spawnSync("bun",["build","--target=bun","--outfile=/dev/null","--metafile="+meta,join(root,entry)],{cwd:root,encoding:"utf8"});expect(result.status,result.stderr).toBe(0);return Object.keys(JSON.parse(readFileSync(meta,"utf8")).inputs).map(path=>relative(root,resolve(root,path))).filter(path=>!path.split("/").includes("node_modules"));};
const declaredPaths=(root:string,projectRoot:string,inputs:readonly string[])=>inputs.filter(input=>/^\{(?:projectRoot|workspaceRoot)\}\/[^*?{}\[\]!]*\.[cm]?[jt]sx?$/u.test(input)).map(input=>resolve(root,input.replace("{projectRoot}",projectRoot).replace("{workspaceRoot}","."))).filter(path=>{try{readFileSync(path);return true;}catch{return false;}});

test("covered input fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);});

test("declared module inputs the closure already compiled change nothing, the others join, exactly as when every declared module is an extra entry",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-covered-"));
 try{
  build(root);
  const script=join(root,"tools/📜️script.ts"),baseline=strip(await cacheInternals.relativeScriptInputs([script],root,createScriptInputCache(),true));
  expect(baseline).toEqual([...corpus.base].sort());
  for(const row of corpus.cases){
   const sources=await cacheInternals.targetScriptSources({inputs:row.inputs,options:{command:corpus.command}},root,createScriptInputCache(),row.projectRoot),got=strip(sources.closure);
   expect(got,row.id).toEqual([...row.closure].sort());
   const reference=strip(await cacheInternals.relativeScriptInputs([script,...declaredPaths(root,row.projectRoot,row.inputs)],root,createScriptInputCache(),true));
   expect(got,row.id+" (reference)").toEqual(reference);
   for(const path of declaredPaths(root,row.projectRoot,row.inputs))expect(bunOracle(root,relative(root,path)).every(reached=>got.includes(reached)||reached===relative(root,path)),row.id+" (bundler)").toBe(true);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("one script input cache serves every case with the same answers",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"closure-covered-shared-"));
 try{
  build(root);const cache=createScriptInputCache();
  for(const pass of [0,1])for(const row of corpus.cases){
   const sources=await cacheInternals.targetScriptSources({inputs:row.inputs,options:{command:corpus.command}},root,cache,row.projectRoot);
   expect(strip(sources.closure),`${row.id} pass ${pass}`).toEqual([...row.closure].sort());
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});
