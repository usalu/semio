/** 🚫️ Contract: the production refusal of the source closure holds. A nested child whose sources live under the issued `.cache` test artifact root is refused as an external implementation runtime dependency, which is why the resolution laws keep their fixtures under a declared root outside every cache tree. */
import{test,expect}from"bun:test";
import{mkdtemp,writeFile,rm}from"node:fs/promises";
import{join,relative}from"node:path";
import{fileURLToPath}from"node:url";
import{physicalSourceOriginV1}from"../../../../📥️inputs/🔏️revision/🟨️.mjs";
import{captureCompilerBuildSourceClosureV1}from"../../🏗️compiler/🟦️.ts";
import{buildSourceClosureIdentityV1,buildSourceClosureStreamLoaderV1}from"../../🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{withRepositoryTestArtifactsV1,repositoryTestArtifactRootV1}from"../../../../../../🧪️test/📦️packages/🟦️typescript/💾️storage/🟦️.ts";
import{createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{runOwnedCommand}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();

test("a nested child under the issued cache artifact root is refused as an external implementation runtime dependency",()=>withBuildTestInvocationV1(async original=>withRepositoryTestArtifactsV1(repository.root,original,createIssuedResourceControlV1(original,"test:cache-root-refusal-artifacts",0),async()=>{
 const workspace=process.cwd(),artifacts=repositoryTestArtifactRootV1(repository.root),origin=physicalSourceOriginV1(import.meta.url,(import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL);
 expect(artifacts.split(/[\\/]/u)).toContain(".cache");
 const area=await mkdtemp(join(artifacts,"cr-")),control=createIssuedResourceControlV1(original,"test:cache-root-refusal",90000),local=(name:string)=>relative(workspace,join(area,name)).replaceAll("\\","/"),specifier=(url:URL)=>{const name=relative(area,fileURLToPath(url)).replaceAll("\\","/");return name.startsWith(".")?name:"./"+name;};
 const owner=specifier(new URL("../../🟦️.ts",origin)),routing=specifier(new URL("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts",origin));
 const entry='import{captureBuildSourceClosureV1}from'+JSON.stringify(owner)+';import{receiveScriptProcessInvocation}from'+JSON.stringify(routing)+';import"./worker.ts";export async function runCapturedBuildSourceV1(args,proof){proof.recheck();await receiveScriptProcessInvocation(process.env,async original=>{try{await captureBuildSourceClosureV1(process.cwd(),args[0],original);console.log("cache-root:accepted");}catch(error){console.log("cache-root:"+error.message);}});}';
 try{
  for(const[name,text]of[["entry.ts",entry],["worker.ts","export const value=1;"]])await writeFile(join(area,name),text);
  const snapshot=await captureCompilerBuildSourceClosureV1(workspace,local("entry.ts"),original,control),output={stdout:"",stderr:""};
  await runOwnedCommand(process.execPath,["--eval",buildSourceClosureStreamLoaderV1,"--",buildSourceClosureIdentityV1(snapshot),local("worker.ts")],workspace,"cache-root-refusal",90000,{control,signal:control.signal,env:scriptResourceProcessEnvironmentV1(control,process.env),auxiliaryInput:{version:1,text:JSON.stringify(snapshot)},stdout:"ignore",output:{maximumBytes:16384,maximumLines:64},outputConsumer:(bytes,stream)=>{output[stream]+=Buffer.from(bytes).toString("utf8");}});
  const lines=output.stdout.split(/\r?\n/u).filter(line=>line.startsWith("cache-root:"));
  expect(lines,output.stderr).toEqual(["cache-root:External implementation runtime dependency refused: "+local("worker.ts")]);
 }finally{await rm(area,{recursive:true,force:true});}
})),120000);
