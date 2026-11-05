import {runtimeWhichV1} from "../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {join,posix,win32} from "node:path";
import {fileURLToPath} from "node:url";
import {spawnSync} from "node:child_process";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import schema from "../../🧬️schema/📥️install.json" with {type:"json"};
import vectors from "./🔣️.json" with {type:"json"};

test("Source directory pairs match neutral topology and independent Python and SQLite",()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("pairs.ts",source,ts.ScriptTarget.Latest,true),names=["javascriptDependencySourcePairPlanV1","javascriptDependencySourcePairPhysicalV1"],nodes=names.map(name=>{const node=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!node)throw Error(name);return node;}),code=ts.transpileModule(nodes.map(node=>node.getText(tree)).join("\n")+";return {plan:javascriptDependencySourcePairPlanV1,physical:javascriptDependencySourcePairPhysicalV1};",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,oracle=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(vectors),encoding:"utf8",windowsHide:true,timeout:10000,maxBuffer:32768});
 expect(oracle.status,oracle.stderr).toBe(0);const answers=JSON.parse(oracle.stdout)as{base:string|null;shared:boolean}[],validate=new Ajv({strict:true}).compile(schema.$defs.SourceDirectoryPairCase),db=new Database(":memory:");
 try{for(const[index,row]of vectors.entries()){
  expect(validate(row),row.id).toBe(true);const grammar=row.style==="win32"?win32:posix,primary=Object.freeze({cancel:row.id});let checks=0;
  const api=new Function("posix","win32","join","lstatSync",code)(posix,win32,grammar.join,(path:string)=>({dev:row.device==="split"&&path.includes("cache")?2n:1n,isDirectory:()=>true,isSymbolicLink:()=>row.device==="linked"&&path.includes("cache")}));
  const plan=api.plan(row.target,row.parent,row.style);expect(plan?.base??null,row.id).toBe(row.base);expect(plan?.base??null,row.id).toBe(answers[index]!.base);
  let shared=false,error:unknown;try{shared=plan!==undefined&&api.physical(plan,()=>{if(row.canceled&&++checks===2)throw primary;});}catch(failure){error=failure;}
  expect(shared,row.id).toBe(row.shared);expect(shared,row.id).toBe(answers[index]!.shared);if(row.canceled)expect(error).toBe(primary);else expect(error).toBeUndefined();
  expect(Boolean((db.query("SELECT ?1 IS NOT NULL AND ?2='same' AND NOT ?3 valid").get(row.base,row.device,row.canceled?1:0)as{valid:number}).valid),row.id).toBe(shared);
 }}finally{db.close();}
});

import * as fs from "node:fs";
import {resolve,dirname,sep} from "node:path";
import {withBuildTestInvocationV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import * as issued from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {checkResourceLeaseControl} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {readCachingTestTableV1} from "../../../../../🗂️suites/🟦️.ts";
import * as descriptor from "@semio-tech/framework-filesystem/observation/descriptor";
import nativeCases from "./📁️.json" with {type:"json"};
const suite=readCachingTestTableV1().suites.find(row=>row.id==="bootstrap-dependency-store-producer");if(!suite)throw Error("Authored Source pair suite required");
declareLibraryBuildTestEnvelopeV1({suite:suite.id,maximumElapsedMilliseconds:suite.budgetMs});

test("actual Source pairs retain simultaneous native custody through mutation and races",()=>withBuildTestInvocationV1(async original=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("native-pair.ts",source,ts.ScriptTarget.Latest,true),names=["javascriptDependencySourcePairPlanV1","javascriptDependencySourcePairPhysicalV1","withJavascriptDependencySourceDirectoryPairV1","withJavascriptDependencyProducedDirectoryV1"],nodes=names.map(name=>{const node=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===name);if(!node)throw Error(name);return node;}),api=new Function("posix","win32","join","process","lstatSync","installSchema","withArtifactRelativeDirectoryV1",ts.transpileModule(nodes.map(node=>node.getText(tree)).join("\n")+";return withJavascriptDependencySourceDirectoryPairV1;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText)(posix,win32,join,process,fs.lstatSync,schema,descriptor.withArtifactRelativeDirectoryV1),keys=["dev","ino","mode","nlink","size","mtimeNs","ctimeNs"]as const,stamp=(facts:Pick<fs.BigIntStats,typeof keys[number]>)=>JSON.stringify(keys.map(key=>String(facts[key]))),base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Owned native pair artifacts required");expect(descriptor.artifactDescriptorProtocolV1().maximumOwners).toBe(32);
 for(const row of nativeCases){
  expect(new Ajv({strict:true}).validate(schema.$defs.SourceDirectoryPairNativeCase,row)).toBe(true);
  const root=fs.mkdtempSync(join(base,"dp-")),target=join(root,row.mode==="parent-ancestor"?"p/t":"t"),parent=join(root,row.mode==="target-ancestor"?"t/p":row.mode==="same"?"t":"p"),probe=join(parent,"proof"),stop=new AbortController(),primary=Object.freeze({cancel:row.mode}),control=issued.createIssuedResourceControlV1(original,"Source:pair:"+row.mode,0,stop.signal),before=descriptor.artifactDescriptorCustodyV1();let peak=before.active,accepted=false,error:unknown,attempted=false;
  for(const path of[target,parent]){if(path.length>256)throw Error("Native pair path capacity exceeded");fs.mkdirSync(path,{recursive:true});}
  const parentBefore=fs.lstatSync(parent,{bigint:true}),check=()=>{issued.requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);peak=Math.max(peak,descriptor.artifactDescriptorCustodyV1().active);};
  try{
   await api(target,parent,check,async(targetHandle:descriptor.ArtifactRelativeDirectoryV1,ownParent:<T>(check:()=>void,body:(directory:descriptor.ArtifactRelativeDirectoryV1)=>Promise<T>)=>Promise<T>)=>{
    const expected=stamp(targetHandle.stat()),targetCheck=()=>{check();expect(stamp(fs.lstatSync(target,{bigint:true}))).toBe(expected);expect(stamp(targetHandle.stat())).toBe(expected);};
    targetCheck();
    if(row.mode==="parent-race"){attempted=true;fs.renameSync(parent,join(root,"retired-parent"));fs.mkdirSync(parent);}
    return ownParent(targetCheck,async parentHandle=>{
     if(String(parentHandle.stat().ino)!==String(parentBefore.ino))throw Error("Exact captured Source parent changed");
     if(row.mode==="target-race"){attempted=true;fs.renameSync(target,join(root,"retired-target"));fs.mkdirSync(target);}
     if(row.mode==="cancel"){attempted=true;stop.abort(primary);}
     targetCheck();
     if(row.mutate)await parentHandle.withNewFile("proof",async file=>{const bytes=Buffer.from("proof");expect(file.write(bytes,0,bytes.length,0)).toBe(bytes.length);file.sync();});
     targetCheck();
    });
   });
   accepted=true;
  }catch(failure){error=failure;}
  try{expect(accepted,row.mode+" "+String(error)).toBe(row.accepted);expect(peak).toBeLessThanOrEqual(32);expect(descriptor.artifactDescriptorCustodyV1()).toEqual(before);if(row.accepted&&row.mutate)expect(fs.readFileSync(probe,"utf8")).toBe("proof");if(!row.accepted){expect(attempted).toBe(true);expect(fs.existsSync(probe)).toBe(false);}if(row.mode==="cancel")expect(error).toBe(primary);}finally{stop.abort();if(!resolve(root).startsWith(resolve(base)+sep))throw Error("Owned native pair cleanup required");fs.rmSync(root,{recursive:true,force:true});}
 }
}),suite.budgetMs);

test("neutral native link size equals public target UTF8 bytes",()=>withBuildTestInvocationV1(async original=>{
 const rootBase=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!rootBase)throw Error("Owned native size artifacts required");
 const root=fs.mkdtempSync(join(rootBase,"ls-")),stop=new AbortController(),control=issued.createIssuedResourceControlV1(original,"Source:link-size",0,stop.signal),before=descriptor.artifactDescriptorCustodyV1(),source=new URL(import.meta.resolve("@semio-tech/framework-filesystem/observation/descriptor")),fixture=new URL("./🧪️tests/🔗️size/📁️.json",source),contract=JSON.parse(fs.readFileSync(new URL("./🧪️tests/🔗️size/🔣️.json",source),"utf8")),rows=JSON.parse(fs.readFileSync(fixture,"utf8"));
 expect(new Ajv({strict:false}).validate(contract,rows)).toBe(true);
 const check=()=>{issued.requireIssuedResourceControlV1(control);checkResourceLeaseControl(control);};
 try{for(const row of rows){expect(Buffer.byteLength(row.suffix)).toBe(row.utf8Bytes);const target=join(root,row.suffix);fs.mkdirSync(target);fs.symlinkSync(target,join(root,"link"),process.platform==="win32"?"junction":"dir");
 await descriptor.withArtifactRelativeDirectoryV1(root,check,async directory=>{const entry=await directory.entry("link");expect(entry?.kind).toBe("link");const publicTarget=fs.readlinkSync(join(root,"link")),python=spawnSync(requireTestToolV1("python"),["-I","-c","import json,sys;print(len(json.loads(sys.stdin.buffer.read().decode('utf-8')).encode('utf-8')))"],{input:JSON.stringify(publicTarget),encoding:"utf8",windowsHide:true,timeout:10000});expect(python.status).toBe(0);expect(entry?.size).toBe(BigInt(python.stdout.trim()));expect(entry?.size).toBe(fs.lstatSync(join(root,"link"),{bigint:true}).size);expect(entry?.size).toBe(BigInt(Buffer.byteLength(publicTarget)));const node=runtimeWhichV1("node");if(!node)throw Error("Node comparison tool required");const observed=spawnSync(node,["-e","const fs=require('fs');console.log(fs.lstatSync(process.argv[1],{bigint:true}).size.toString())",join(root,"link")],{encoding:"utf8",windowsHide:true,timeout:10000});expect(observed.status).toBe(0);expect(entry?.size).toBe(BigInt(observed.stdout.trim()));});
 fs.unlinkSync(join(root,"link"));}
 expect(descriptor.artifactDescriptorCustodyV1()).toEqual(before);
 }finally{stop.abort();if(!resolve(root).startsWith(resolve(rootBase)+sep))throw Error("Owned native size cleanup required");fs.rmSync(root,{recursive:true,force:true});}
}),suite.budgetMs);

