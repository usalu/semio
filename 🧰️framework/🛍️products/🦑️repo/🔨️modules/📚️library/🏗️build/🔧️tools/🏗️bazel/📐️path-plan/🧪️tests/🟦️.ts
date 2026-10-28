/** 📐️ Contract: the Bazel path plan is closed, the worst-case path law of every declared target agrees with an independent Python implementation and the language-agnostic corpus on every host (UTF-16 units counted like iconv-lite), the short output root is stable per checkout, the hermetic runner builds its private root in the declared short place from aliased inputs, and the manifest that carries the aliases is closed. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import iconv from "iconv-lite";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,readlinkSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import component from "../../🧬️schema/🔣️.json";
import execSchema from "../../🏃️execution/🧬️schema/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {BAZEL_PATH_PLAN_V1,bazelHermeticRootV1,bazelOutputRootV1,bazelPathLawV1,bazelPathLimitV1,bazelPathUnitsV1,linkBazelOutputRootV1} from "../🟦️.ts";
import {BAZEL_HERMETIC_RUNNER,bazelHermeticManifestV1} from "../../🏃️execution/🟦️.ts";

const api=await import("../../📜️script.ts"),python=Bun.which("python3"),units=(text:string)=>iconv.encode(text,"utf16-le").length/2;
const run=(program:string,input:unknown)=>Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(input)),stdout:"pipe",stderr:"pipe"});
const hostOf=(row:any)=>({platform:row.platform,arch:row.arch,longPaths:row.longPaths,...(row.outputBase===undefined?{}:{outputBase:row.outputBase}),...(row.checkout===undefined?{}:{checkout:row.checkout})});

test("the path plan and the manifest fields that carry it are closed schemas",()=>{
 const ajv=new Ajv({strict:true}).addSchema(schema,"plan").addSchema(execSchema,"execution");
 expect(ajv.compile({$ref:"plan#/definitions/PolicyShape"})(BAZEL_PATH_PLAN_V1)).toBe(true);
 const shape=ajv.compile({$ref:"plan#/definitions/PolicyShape"});
 expect(shape({...BAZEL_PATH_PLAN_V1,unit:"utf8"})).toBe(false);expect(shape({...BAZEL_PATH_PLAN_V1,extra:1})).toBe(false);
 expect(shape({...BAZEL_PATH_PLAN_V1,outputRoot:{...BAZEL_PATH_PLAN_V1.outputRoot,win32:{template:"x",checkoutDigits:2}}})).toBe(false);
 expect(shape({...BAZEL_PATH_PLAN_V1,hermeticRoot:{...BAZEL_PATH_PLAN_V1.hermeticRoot,win32:{...BAZEL_PATH_PLAN_V1.hermeticRoot.win32,separator:"|"}}})).toBe(false);
 const target=ajv.compile({$ref:"plan#/definitions/Target"});expect(target({id:"t",kind:"hermetic",sources:["a"]})).toBe(true);expect(target({id:"t",kind:"other",sources:[]})).toBe(false);
 const manifest=ajv.compile({$ref:"execution#/definitions/Manifest"}),base={version:1,id:"p:t",project:"p",target:"t",root:"r",args:[],commands:["x"],outputs:[]};
 const entry=bazelHermeticManifestV1({id:"p:t",project:"p",target:"t",label:"t_0000000000000000",dependencies:[],args:[],root:"r",definition:{execution:{kind:"command",command:"x"},outputs:[]}} as any,[],{inputs:["r/a.ts"],aliases:{r:"a/0123456789"}});
 expect(manifest(entry),JSON.stringify(manifest.errors)).toBe(true);expect(entry.scratch).toEqual(BAZEL_PATH_PLAN_V1.hermeticRoot);expect(entry.aliases).toEqual({r:"a/0123456789"});
 expect(manifest({...base,aliases:{r:"s/a"}})).toBe(false);expect(manifest({...base,scratch:{darwin:{}}})).toBe(false);
 expect(component.definitions.PathBudget.const.repositoryUnits).toBe(256);expect(BAZEL_PATH_PLAN_V1.fillers.stash).toBe(execSchema.definitions.Contract.const.toolchain.mnemonic);
});

test("the worst-case law of every declared target agrees with the corpus, an independent Python implementation and iconv-lite's UTF-16 counts on every host",()=>{
 const alias=(path:string)=>api.bazelSourceAliasLabelV1(path);
 for(const row of fixture.cases.law){
  const actual=bazelPathLawV1(hostOf(row.host) as never,row.targets as never,alias);
  expect({limit:actual.limit,fits:actual.fits,over:actual.over},row.name).toEqual({limit:row.expected.limit,fits:row.expected.fits,over:row.expected.over});
  expect(actual.rows.map(entry=>({id:entry.id,families:entry.families,worst:entry.worst,slack:entry.slack,fits:entry.fits})),row.name).toEqual(row.expected.rows as never);
  for(const target of row.targets)for(const path of target.sources)expect(bazelPathUnitsV1(path),path).toBe(units(path));
 }
 if(python){
  const result=run(fixture.oracle,{schema,component,cases:{law:fixture.cases.law.map(({name,host,targets})=>({name,host,targets})),roots:fixture.cases.roots.map(({platform,checkout})=>({platform,checkout}))}});
  expect(result.exitCode,result.stderr.toString()).toBe(0);
  const expected=JSON.parse(result.stdout.toString());expect(expected.law).toEqual(fixture.cases.law.map(row=>row.expected));expect(expected.roots).toEqual(fixture.cases.roots.map(row=>row.expected));
 }
 const windows=fixture.cases.law.filter(row=>row.host.platform==="win32"&&row.host.outputBase==="C:\\b\\0123abcd\\s0");
 for(const row of windows){const found=Object.fromEntries(row.expected.rows.map(entry=>[entry.id,entry]));expect(found["hermetic-deep"]!.fits,"the aliased and short-rooted deep plugin path fits Windows MAX_PATH").toBe(true);expect(found["hermetic-240"]!.fits).toBe(true);expect(found["hermetic-241"]!.fits).toBe(false);}
 expect(bazelPathLimitV1({platform:"win32",longPaths:"enabled"})).toBe(256);expect(bazelPathLimitV1({platform:"win32",longPaths:"disabled"})).toBe(256);expect(bazelPathLimitV1({platform:"darwin",longPaths:"unprobed"})).toBe(256);
});

test("the short output root is ASCII, short, stable per checkout, distinct across checkouts and only declared where the host needs it",()=>{
 for(const row of fixture.cases.roots){
  const actual=bazelOutputRootV1(row.platform,row.checkout);expect(actual?.root??null,row.checkout).toBe(row.expected);
  if(actual){expect(actual.root).toMatch(/^[A-Z]:\\b\\[a-f0-9]{8}$/u);expect(units(actual.base("s0"))).toBeLessThanOrEqual(16);expect(actual.base("s3")).toBe(actual.root+"\\s3");}
 }
 const [first,second,third]=fixture.cases.roots.slice(0,3).map(row=>bazelOutputRootV1(row.platform,row.checkout)!.root);expect(first).toBe(second);expect(first).not.toBe(third);
 expect(bazelOutputRootV1("darwin","/x")).toBeUndefined();expect(bazelOutputRootV1("linux","/x")).toBeUndefined();
 const custom={...BAZEL_PATH_PLAN_V1,outputRoot:{...BAZEL_PATH_PLAN_V1.outputRoot,darwin:{template:"/tmp/b/{checkout}",checkoutDigits:6}}};expect(bazelOutputRootV1("darwin","/Users/x/semio",custom as never)!.root).toMatch(/^\/tmp\/b\/[a-f0-9]{6}$/u);
 const root=bazelHermeticRootV1("win32",undefined)!;expect(root.units).toBe(units("C:\\b\\w\\000000\\r\\"));expect(bazelHermeticRootV1("darwin",undefined)!.units).toBe(units("/tmp/000000000000/r/"));
});

test.skipIf(process.platform==="win32")("the short root is a link to the real output base directory, reconciled idempotently and never over a real directory",()=>{
 const directory=mkdtempSync(join(tmpdir(),"path-root-")),real=join(directory,"real"),other=join(directory,"other"),root=join(directory,"x/b/abcd1234");
 try{
  mkdirSync(real);mkdirSync(other);linkBazelOutputRootV1(root,real);expect(readlinkSync(root)).toBe(real);const before=lstatSync(root,{bigint:true}).ino;
  linkBazelOutputRootV1(root,real);expect(lstatSync(root,{bigint:true}).ino).toBe(before);
  linkBazelOutputRootV1(root,other);expect(readlinkSync(root)).toBe(other);
  rmSync(root);mkdirSync(root);expect(()=>linkBazelOutputRootV1(root,real)).toThrow("Short Bazel output root is a real directory");
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the hermetic runner reads aliased inputs into a short private root, refuses an undeclared directory and removes the root",()=>{
 const directory=mkdtempSync(join(tmpdir(),"path-runner-")),inputs=["lib/src/a.txt","lib/gen.ts","top.txt"],deep="lib/"+"d".repeat(120)+"/"+"e".repeat(100)+"/x.txt";
 const files:Record<string,string>={"lib/src/a.txt":"alpha\n","lib/gen.ts":'import{mkdirSync,readFileSync,writeFileSync}from"node:fs";const root=process.env.SEMIO_WORKSPACE_ROOT;mkdirSync(root+"/lib/gen",{recursive:true});writeFileSync(root+"/lib/gen/out.txt",readFileSync(root+"/lib/src/a.txt","utf8").toUpperCase()+"|"+root);',"top.txt":"t\n",[deep]:"deep\n"};
 const aliasOf=(path:string)=>{const cut=path.lastIndexOf("/");return "a/"+api.bazelSourceAliasV1(cut<0?"":path.slice(0,cut));};
 const prepare=(declared:string[],extra:Record<string,unknown>={})=>{
  const root=mkdtempSync(join(directory,"exec-"));
  for(const path of Object.keys(files)){const cut=path.lastIndexOf("/");mkdirSync(join(root,aliasOf(path)),{recursive:true});writeFileSync(join(root,aliasOf(path),path.slice(cut+1)),files[path]!);}
  mkdirSync(join(root,"s"),{recursive:true});writeFileSync(join(root,"hermetic-runner.mjs"),BAZEL_HERMETIC_RUNNER);
  const aliases=Object.fromEntries([...new Set(declared.map(path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):""))].map(dir=>[dir,aliasOf(dir+"/x")]));
  writeFileSync(join(root,"m.json"),JSON.stringify({version:1,id:"f:g",project:"f",target:"g",root:"lib",args:[],commands:["bun {projectRoot}/gen.ts"],outputs:["{projectRoot}/gen"],inputs:declared,aliases,scratch:BAZEL_PATH_PLAN_V1.hermeticRoot,...extra}));
  return root;
 };
 const exec=(root:string)=>Bun.spawnSync([process.execPath,"hermetic-runner.mjs","m.json","out"],{cwd:root,env:{},stdout:"pipe",stderr:"pipe"});
 try{
  const root=prepare([...inputs,deep]),result=exec(root);expect(result.exitCode,result.stderr.toString()).toBe(0);
  const snapshot=JSON.parse(readFileSync(join(root,"out/snapshot.json"),"utf8")),entry=snapshot.entries.find((row:any)=>row.path==="lib/gen/out.txt"),text=readFileSync(join(root,"out/files",entry.file),"utf8"),[content,workspace]=text.split("|");
  expect(content).toBe("ALPHA\n");expect(workspace).toMatch(/^\/(?:private\/)?tmp\/[0-9a-f]{12}\/r$/u);expect(units(workspace!+"/"+deep)).toBeLessThanOrEqual(256);expect(existsSync(workspace!.replace(/\/r$/u,"")),"the private root is removed after a successful run").toBe(false);
  const some=prepare(inputs.slice(0,2)),partial=exec(some);expect(partial.exitCode,partial.stderr.toString()).toBe(0);
  const bad=prepare(["lib/gen.ts","top.txt"]),manifest=JSON.parse(readFileSync(join(bad,"m.json"),"utf8"));delete manifest.aliases[""];writeFileSync(join(bad,"m.json"),JSON.stringify(manifest));
  const refused=exec(bad);expect(refused.exitCode).not.toBe(0);expect(refused.stderr.toString()).toMatch(/an input directory without a declared alias: top\.txt/u);
  const reader=prepare(["lib/gen.ts"]);const read=exec(reader);expect(read.exitCode,"the generator needs lib/src/a.txt, which is staged but undeclared").not.toBe(0);expect(read.stderr.toString()).toMatch(/a\.txt/u);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
