import{repoCpuProfileArgumentsV1}from"../../../../../../../🏗️build/⏱️profile/📊️cpu/🟨️.mjs";
import{fileURLToPath}from"node:url";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve,relative} from "node:path";
import ts from "typescript";
import {withBuildTestInvocationV1,registerIsolatedBuildTestV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,withAcceptedPhysicalCleanupV1,beginAcceptedPhysicalCleanupV1,scriptInvocationEnvironment,receiveScriptProcessInvocation,SCRIPT_PROCESS_INVOCATION_ENV} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {runtimeEntrySourcePathV1,captureRuntimeEntryV1,capturedRuntimeEntryCommandV1,retainRuntimeEntryCleanupV1,capturedRuntimeEntryCleanupCommandV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🟦️.ts";
import {nativeDirectoryRuntimeEntryV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/📜️script.ts";
import {withRepositoryCompilerRuntimeEntriesV1,withRepositoryRuntimeEntriesV1} from "../../../../../../../🏗️build/📥️source/🔏️closure/🚪️command/🔌️runtime/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const validate=new Ajv({strict:false}).compile(schema.definitions.Case);

type Row=typeof corpus.cases[number];
type Operation=(root:string,artifacts:string,original:unknown,phase:Parameters<typeof withAcceptedPhysicalCleanupV1>[0])=>Promise<void>;
const transpile=(source:string)=>ts.transpileModule(source.replace(/^export /u,""),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.ESNext}}).outputText;

async function runOwner(row:Row,original:Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0],operation:Operation):Promise<void>{
 const ports={createIssuedResourceControlV1,withRepositoryCompilerRuntimeEntriesV1},names=Object.keys(ports),values=Object.values(ports),root=resolve(".");
 if(row.owner==="bootstrap"){
  const source=readFileSync(new URL("../../../../../../🚀️bootstrap/📜️script.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("bootstrap.ts",source,ts.ScriptTarget.Latest,true),class_=ast.statements.find(node=>ts.isClassDeclaration(node)&&node.name?.text==="NxScript");
  if(!class_||!ts.isClassDeclaration(class_))throw Error("Actual bootstrap class required");
  const method=class_.members.find(node=>ts.isMethodDeclaration(node)&&node.name.getText(ast)==="run");
  if(!method)throw Error("Actual initial bootstrap method required");
  const Owner=new Function(...names,transpile("class Owner{"+method.getText(ast)+"}")+";return Owner;")(...values),owner=new Owner();
  Object.assign(owner,{root,invocation:original,runIssued:async(args:string[],phase:Parameters<typeof withAcceptedPhysicalCleanupV1>[0])=>{expect(args).toEqual(["--version"]);await operation(root,"unused-artifact-port",original,phase);}});
  await owner.run(["--version"]);return;
 }
 const url=new URL(row.owner==="service"?"../../../🟦️.ts":"../📥️runtime/🟦️.ts",import.meta.url),name=row.owner==="service"?"withNxDaemonInitialRuntimeV1":"proveNxDaemonOutputObservationV1",source=readFileSync(url,"utf8"),ast=ts.createSourceFile("owner.ts",source,ts.ScriptTarget.Latest,true),declaration=ast.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name);
 if(!declaration)throw Error("Actual initial owner required");
 const run=new Function(...names,"proveNxDaemonOutputObservationInScopeV1",transpile(declaration.getText(ast))+";return "+name+";")(...values,operation);
 if(row.owner==="service")await run(root,original,(phase:Parameters<typeof withAcceptedPhysicalCleanupV1>[0])=>operation(root,"unused-artifact-port",original,phase));
 else await run(root,"unused-artifact-port",original);
}

async function proveOwner(row:Row,checkpoint?:()=>void):Promise<void>{
 return withBuildTestInvocationV1(async original=>{
  expect(validate(row)).toBe(true);const events:string[]=[],failure=Error(row.name),db=new Database(":memory:");
  db.exec("create table phases(position integer primary key,name text)");row.expected.forEach((name,index)=>db.query("insert into phases values(?,?)").run(index,name));
  const operation:Operation=async(root,artifacts,received,phase)=>{
   expect(root).toBe(resolve("."));expect(artifacts).toBe("unused-artifact-port");expect(received).toBe(original);
   await withAcceptedPhysicalCleanupV1(phase,{version:1,kind:"native-image",resource:"test:daemon-provider-cleanup",reserveMilliseconds:1000},async(cleanup,body)=>{
    const entry=await captureRuntimeEntryV1(nativeDirectoryRuntimeEntryV1(),body),command=capturedRuntimeEntryCommandV1(entry,body),retained=retainRuntimeEntryCleanupV1(entry,cleanup,body);
    expect(command.implementation.length).toBe(64);const image=JSON.parse(command.input) as {entry:string;files:{path:string}[]};expect(image.entry).toBe(relative(resolve("."),runtimeEntrySourcePathV1(nativeDirectoryRuntimeEntryV1())).replaceAll("\\","/"));expect(image.files.some(file=>file.path.includes("/🧪️tests/"))).toBe(false);events.push("ordinary");
    try{if(row.fail)throw failure;}finally{
     await beginAcceptedPhysicalCleanupV1(cleanup);await Promise.resolve();
     expect(capturedRuntimeEntryCleanupCommandV1(retained,cleanup)).toBe(command);events.push("settlement");
    }
   });
  };
  try{
   let observed:unknown;try{await runOwner(row,original,operation);}catch(error){observed=error;}
   expect(observed).toBe(row.fail?failure:undefined);
   expect(events).toEqual(row.expected);
   expect(events).toEqual((db.query("select name from phases order by position").all() as {name:string}[]).map(item=>item.name));
   checkpoint?.();
  }finally{db.close();}
 });
}

/** 🧊️ Every initial owner retains a genuine immutable token through its accepted success or failure cleanup. */
for(const row of corpus.cases){
 if(row.fail)registerIsolatedBuildTestV1(fileURLToPath(import.meta.url),row.name,{bodyMilliseconds:90000,scope:{kind:"body"}},checkpoint=>proveOwner(row,checkpoint),new RegExp(row.name,"u"));
 else test(row.name,()=>proveOwner(row));
}

/** 🚫️ The admitted consumer keeps its real Source-map refusal in its own genuinely issued negative process. */
registerIsolatedBuildTestV1(fileURLToPath(import.meta.url),"admitted consumer refuses missing Source context",{bodyMilliseconds:90000,scope:{kind:"body"}},checkpoint=>withBuildTestInvocationV1(async original=>{
 let entered=false;const phase=createIssuedResourceControlV1(original,"test:admitted-provider-refusal",30000);
 await expect(withRepositoryRuntimeEntriesV1(resolve("."),original,phase,async()=>{entered=true;})).rejects.toThrow("Actual admitted Source resolution map required");
 expect(entered).toBe(false);checkpoint();
}),/Actual admitted Source resolution map required/u);

/** 🪢️ The infrastructure owner encloses its complete Native operation and settlement. */
test("Source owner wraps the complete Native operation in its runtime provider",()=>{const source=readFileSync(new URL("../📥️runtime/🟦️.ts",import.meta.url),"utf8");expect(source.includes("withRepositoryCompilerRuntimeEntriesV1(root,original,phase")).toBe(true);});

/** 🥾️ Raw initial script and all existing service epochs admit cleanup before their first store. */
test("raw bootstrap and service entry epochs select explicit initial compiler context",()=>{
 const bootstrap=readFileSync(new URL("../../../../../../🚀️bootstrap/📜️script.ts",import.meta.url),"utf8"),service=readFileSync(new URL("../../../🟦️.ts",import.meta.url),"utf8");
 expect(bootstrap.includes("withRepositoryCompilerRuntimeEntriesV1(this.root,this.invocation,phase")).toBe(true);
 expect(service.includes("parent=>withNxDaemonInitialRuntimeV1(birth.repositoryRoot,parent,")).toBe(true);
 expect(service.includes("original=>withNxDaemonInitialRuntimeV1(birth.repositoryRoot,original,")).toBe(true);
 expect(service.includes("original=>withNxDaemonInitialRuntimeV1(deployment.repositoryRoot,original,")).toBe(true);
});

/** 📨️ The real fresh issuer forwards the same envelope before the actual client observes its environment. */
test("the client environment forwards the same issued Original before queued requests",()=>withBuildTestInvocationV1(async original=>{
 const source=readFileSync(new URL("../📥️runtime/🟦️.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("owner.ts",source,ts.ScriptTarget.Latest,true),declaration=ast.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="reflectNxDaemonClientEnvironmentV1");
 expect(declaration).toBeDefined();if(!declaration)throw Error("Actual client environment projection required");
 const reflect=new Function("scriptInvocationEnvironment",transpile(declaration.getText(ast))+";return reflectNxDaemonClientEnvironmentV1;")(scriptInvocationEnvironment),prior={...process.env},expected=scriptInvocationEnvironment(original,{})[SCRIPT_PROCESS_INVOCATION_ENV];
 expect(new Ajv({strict:false}).compile(schema.definitions.ClientProjection)(corpus.clientProjection)).toBe(true);const input=Object.fromEntries(Object.entries(corpus.clientProjection.environment).map(([key,value])=>[key,value??undefined]));
 try{reflect(original,{...input,[SCRIPT_PROCESS_INVOCATION_ENV]:"invalid-ambient-envelope"});expect(process.env.EXPLICIT_CLIENT).toBe(corpus.clientProjection.expected.EXPLICIT_CLIENT);expect(process.env.REMOVE_ME).toBeUndefined();expect(process.env[SCRIPT_PROCESS_INVOCATION_ENV]).toBe(expected);const db=new Database(":memory:");try{db.run("create table envelope(wire text)");db.run("insert into envelope values(?)",[process.env[SCRIPT_PROCESS_INVOCATION_ENV]!]);expect(db.query("select wire from envelope").get()).toEqual({wire:expected});}finally{db.close();}await Promise.resolve();await receiveScriptProcessInvocation(process.env,async received=>{expect(scriptInvocationEnvironment(received,{})[SCRIPT_PROCESS_INVOCATION_ENV]).toBe(expected);});}
 finally{for(const key of Object.keys(process.env))if(!(key in prior))delete process.env[key];Object.assign(process.env,prior);}
 expect(source.indexOf("reflectNxDaemonClientEnvironmentV1(original,environment)")).toBeLessThan(source.indexOf("client=new("));
}));

/** 📊️ The actual qualified daemon driver gets only the closed profiler prefix before its immutable entry. */
test("the actual qualified Node birth preserves the Original and profiles its own entry",()=>withBuildTestInvocationV1(async original=>{
 const source=readFileSync(new URL("../📥️runtime/🟦️.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("owner.ts",source,ts.ScriptTarget.Latest,true),declaration=ast.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="startNxDaemonObservationServiceV1");expect(declaration).toBeDefined();if(!declaration)throw Error("Actual qualified Node birth required");
 for(const enabled of [false,true]){const environment=enabled?corpus.nodeProfile.environment:{},driver={command:"qualified-node",prefix:corpus.nodeProfile.prefix,suffix:corpus.nodeProfile.suffix},calls:{command:string;args:string[];options:{cwd:string;env:Record<string,string>;stdio:string[]}}[]=[],child=Object.freeze({identity:"actual-child-port"});
 const flags=repoCpuProfileArgumentsV1(environment,{platform:"linux",pid:123,sequence:1}),prepare=(value:unknown)=>{expect(value).toBe(environment);return flags;},spawn=(command:string,args:string[],options:{cwd:string;env:Record<string,string>;stdio:string[]})=>{calls.push({command,args,options});return child;};
 const run=new Function("spawn","prepareRepoCpuProfileArgumentsV1","scriptInvocationEnvironment",transpile(declaration.getText(ast))+";return startNxDaemonObservationServiceV1;")(spawn,prepare,scriptInvocationEnvironment);expect(run(driver,corpus.nodeProfile.entry,"/workspace",environment,original)).toBe(child);expect(calls).toHaveLength(1);const call=calls[0]!;expect(call.command).toBe(driver.command);expect(call.args).toEqual([...driver.prefix,...flags,corpus.nodeProfile.entry,...driver.suffix]);expect(call.options.cwd).toBe("/workspace");expect(call.options.stdio).toEqual(["ignore","pipe","pipe","ipc"]);expect<unknown>(call.options.env[SCRIPT_PROCESS_INVOCATION_ENV]).toBe(scriptInvocationEnvironment(original,{})[SCRIPT_PROCESS_INVOCATION_ENV]);
 }
}));
