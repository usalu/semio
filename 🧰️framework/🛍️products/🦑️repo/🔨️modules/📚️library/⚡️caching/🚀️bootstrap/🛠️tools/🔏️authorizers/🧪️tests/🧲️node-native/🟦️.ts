import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,rmSync} from "node:fs";
import {resolve,relative,join,dirname} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import {captureNxAuthorizerSourcesV1} from "../../🟦️.ts";
import {snapshotNxAuthorizerSourcesV1,compileNxAuthorizerSnapshotV1} from "../../📦️snapshot/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,scriptInvocationEnvironment} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandResultV1} from "../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {nodeNativeLinkerEnvironmentV1,planNodeNativeAddonV1,nodeNativeCompilerArgumentsV1} from "../../../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts";
import nativeSchema from "../../../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json";
import {toolForCommandV1,resolveRegisteredToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const native="🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native",owner=native+"/🏗️addon/🟦️.ts",resource=native+"/🦀️.rs",hash=(bytes:Uint8Array|string)=>createHash("sha256").update(bytes).digest("hex");

/** 📦️ Runs the actual owned Node bundle with cold imports, both dependency branches and exact physical Rust resource identity. */
test("the actual Node bundle retains built-in dependencies without import effects and binds its Rust resource",()=>withBuildTestInvocationV1(async original=>{
 const root=resolve("."),entry=relative(root,fileURLToPath(new URL("./📥️entry/🟦️.ts",import.meta.url))).replaceAll("\\","/"),phase=createIssuedResourceControlV1(original,"test:node-native-bundle",90000),captured=await captureNxAuthorizerSourcesV1(root,entry,phase.signal,{installed:true});
 for(const path of [owner,native+"/🧮️system/🟦️.ts",resource])expect(captured.manifest.sources.some(row=>row.path===path)).toBe(true);
 const expected=captured.manifest.sources.find(row=>row.path===resource)!;
 const compiled=await compileNxAuthorizerSnapshotV1(root,snapshotNxAuthorizerSourcesV1(captured),phase,"node","module"),area=mkdtempSync(join(testScratchRootV1("node-native-bundle"),"bundle-")),file=join(area,"bundle.mjs");writeFileSync(file,compiled.bytes);
 const program=`const fs=require("node:fs"),cp=require("node:child_process"),module_=require("node:module"),url=require("node:url"),saved=[],calls=[];for(const[object,names]of [[fs,["readFileSync","existsSync","mkdirSync","statSync","writeFileSync","renameSync","rmSync"]],[cp,["spawnSync","execFileSync"]],[process,["dlopen"]]])for(const name of names){const original=object[name];saved.push(()=>object[name]=original);object[name]=()=>{calls.push(name);throw Error("Eager effect "+name);};}module_.syncBuiltinESMExports();(async()=>{let loaded;try{loaded=await import(url.pathToFileURL(process.argv[1]).href);}finally{for(const restore of saved)restore();module_.syncBuiltinESMExports();}if(calls.length)throw Error("Import effects "+calls.join(","));console.log(JSON.stringify({effects:calls,...loaded.observeNativeSourceV1(),pointer:loaded.nativePointerV1()}));})().catch(error=>{console.error(error);process.exitCode=1;});`;
 try{for(const row of corpus.cases){
  expect(new Ajv({strict:false}).compile(schema.definitions.Case)(row)).toBe(true);
  const cc=toolForCommandV1("cc"),registered=cc?resolveRegisteredToolV1(cc,root,{env:process.env}):undefined,binding=registered?nodeNativeLinkerEnvironmentV1(process.env,{...registered,suffix:registered.suffix??[]}):process.env;
  const environment=scriptInvocationEnvironment(original,{...binding,SEMIO_LANGUAGE:"en",SEMIO_RUNTIME_NATIVE_ADDON:fileURLToPath(new URL(row.provider?"./🧫️absent.cjs":"./🧫️core.cjs",import.meta.url)),SEMIO_RUNTIME_NATIVE_PROVIDER:row.provider?fileURLToPath(new URL("./🧫️provider.cjs",import.meta.url)):""});
  const result=await captureOwnedCommandResultV1("node",["-e",program,file],root,{control:phase,env:environment,output:{maximumBytes:1048576,maximumLines:64}});
  expect(result.status,result.stderr).toBe(0);const observed=JSON.parse(result.stdout);expect(observed.effects).toEqual([]);expect(observed.source).toBe(resolve(resource));expect(observed.sha256).toBe(expected.sha256);expect(observed.pointer).toBe(row.pointer);expect(observed.linker).toEqual(binding.SEMIO_RUNTIME_NATIVE_LINKER?JSON.parse(binding.SEMIO_RUNTIME_NATIVE_LINKER):null);
 }}finally{rmSync(area,{recursive:true,force:true});}
}));

/** 🧬️ Declared Rust DATA is pinned without parsing it as JavaScript; mutation changes identity and missing bytes refuse. */
test("the declared opaque resource participates in captured identity and refuses missing bytes",async()=>{
 const root=mkdtempSync(join(testScratchRootV1("node-native-resource"),"source-")),entry=join(root,owner),data=join(root,resource);mkdirSync(dirname(entry),{recursive:true});writeFileSync(entry,"export const fixture=1;\n");writeFileSync(data,corpus.resource.first);
 try{
  const first=await captureNxAuthorizerSourcesV1(root,owner,new AbortController().signal),row=first.manifest.sources.find(row=>row.path===resource);expect(row?.sha256).toBe(hash(corpus.resource.first));
  writeFileSync(data,corpus.resource.second);const second=await captureNxAuthorizerSourcesV1(root,owner,new AbortController().signal);expect(second.digest).not.toBe(first.digest);
  const db=new Database(":memory:");try{db.run("create table resources(path text primary key,sha text)");for(const item of second.manifest.sources)db.run("insert into resources values(?,?)",[item.path,item.sha256]);expect(db.query("select sha from resources where path=?").get(resource)).toEqual({sha:hash(corpus.resource.second)});}finally{db.close();}
  rmSync(data);await expect(captureNxAuthorizerSourcesV1(root,owner,new AbortController().signal)).rejects.toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🔗️ Projects only the registered SDK driver with exact argument boundaries and isolates its cache identity. */
test("registered linker DATA preserves native choices and the exact Rust linker call",()=>{
 const command=resolve("sdk cc"),source=new Uint8Array([1,2,3]),base={platform:"linux",arch:"x64",home:resolve("."),source:resolve("core.rs")};
 for(const row of corpus.linkers){
  const environment=row.environment as Record<string,string>,resolution={source:row.source,command:"missing" in row?null:command,prefix:row.prefix,suffix:row.suffix};
  if("refuse" in row){expect(()=>nodeNativeLinkerEnvironmentV1(environment,resolution)).toThrow();continue;}
  const projected=nodeNativeLinkerEnvironmentV1(environment,resolution);
  if(!("project" in row)||!row.project){expect(projected).toBe(environment);continue;}
  const linker=JSON.parse(projected.SEMIO_RUNTIME_NATIVE_LINKER!);expect(new Ajv({strict:false}).compile(nativeSchema.definitions.Linker)(linker)).toBe(true);expect(linker).toEqual({version:1,command,arguments:row.suffix});
  const plan=planNodeNativeAddonV1({...base,env:projected},source),plain=planNodeNativeAddonV1({...base,env:environment},source);expect(plan.key).not.toBe(plain.key);
  const args=nodeNativeCompilerArgumentsV1(plan,base.source,resolve("out.node")),expected=["-C","linker="+command,...row.suffix.flatMap(value=>["-C","link-arg="+value])];expect(args.slice(-expected.length-5,-5)).toEqual(expected);
  const db=new Database(":memory:");try{db.run("create table args(position integer primary key,value text)");expected.forEach((value,index)=>db.run("insert into args values(?,?)",[index,value]));expect(db.query("select value from args order by position").all().map(value=>(value as {value:string}).value)).toEqual(expected);}finally{db.close();}
 }
 expect(()=>planNodeNativeAddonV1({...base,env:{SEMIO_RUNTIME_NATIVE_LINKER:JSON.stringify({version:1,command:"relative",arguments:[]})}},source)).toThrow();
});
