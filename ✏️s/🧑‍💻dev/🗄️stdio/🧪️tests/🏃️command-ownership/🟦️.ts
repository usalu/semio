import{createIssuedResourceControlV1}from"@semio-tech/process/routing";
import { describe, test, expect, spyOn } from "bun:test";
import type { RepositoryCacheStoreV1 } from "@semio-tech/repo-lib/storage";
import type { acquireQueuedResourceLease } from "@semio-tech/process/leases";
import { verifyStdioCommandOwnership } from "../📦️artifact-package-graph/🟦️.ts";
import fixture from "../🧫️fixtures/🏃️command-ownership/🔣️.json";
import {stdioCargoMetadata} from "../../🏘️composition/🕸️artifact-graph/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,withScriptInvocationSignalV1,acknowledgeScriptInvocationSignalStopV1} from "@semio-tech/process/routing";
import * as nativePhases from "@semio-tech/repo-lib/artifacts";
import {captureOwnedCommandResultV1} from "@semio-tech/process/owned-execution";
import {Database} from "bun:sqlite";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import Ajv from "ajv";
import { ownedExecutionCaptureSchema as captureSchema } from "@semio-tech/process/owned-execution/capture/schema";
import { ownedExecutionOutputSchema as outputSchema } from "@semio-tech/process/owned-execution/schema";

describe("stdio command ownership", () => {
  test("binds semantic owners, direct routers, source data and registered inputs", async () => {
    await verifyStdioCommandOwnership();
  }, 30_000);
});

async function withStdioTestInvocationV1<T>(childMilliseconds:number,body:(original:any)=>Promise<T>,maximumElapsedMilliseconds=30000):Promise<T>{
 const root=process.cwd(),{createNativeBuildCallerCapabilitiesV1,readBuildAuthoringCapabilitiesV1}=await import("@semio-tech/repo-lib/build"),args=["show","projects"],declared=createNativeBuildCallerCapabilitiesV1(root,args,"nx",process.env),capabilities=readBuildAuthoringCapabilitiesV1(root,args,"nx",{...declared,native:{...declared.native,child:{maximumElapsedMilliseconds:childMilliseconds}}});
 return withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"stdio:original-control-fixture",maximumElapsedMilliseconds},capabilities,Date.now()),body);
}

test("Stdio metadata retains actual Original refusal and bounded owned capture",async()=>{
 const root=process.cwd(),row=fixture.cargoControl,database=new Database(":memory:");
 try{
  await withStdioTestInvocationV1(30000,async original=>{const controller=new AbortController();await withScriptInvocationSignalV1(original,controller.signal,async cancelled=>{controller.abort(Error("Original Stdio cancelled"));await expect(stdioCargoMetadata(root,cancelled)).rejects.toThrow("Original Stdio cancelled");acknowledgeScriptInvocationSignalStopV1(cancelled);});expect(original.control.signal.aborted).toBe(false);});
  await expect(withStdioTestInvocationV1(30000,async original=>{await Bun.sleep(5);await stdioCargoMetadata(root,original);},1)).rejects.toThrow(/deadline/u);
  let published=false,completed=false;const phaseOwner=nativePhases.nativeCallerPhaseV1,refusal=spyOn(nativePhases,"nativeCallerPhaseV1").mockImplementation((...args)=>{const phase=phaseOwner(...args);return{...phase,advance:async command=>{await phase.advance(command);published=true;await Bun.sleep(row.progressDelayMilliseconds);completed=true;throw Error("Original Stdio progress continuation refused");}};});
  try{await expect(withStdioTestInvocationV1(30000,original=>stdioCargoMetadata(root,original))).rejects.toThrow("Original Stdio progress continuation refused");expect(published).toBe(true);expect(completed).toBe(true);}finally{refusal.mockRestore();}
  await withStdioTestInvocationV1(30000,async original=>{
   const control=scriptCompilerAdmissionControlV1(original,"stdio:bounded-capture",30000),output={maximumBytes:row.maximumBytes,maximumLines:row.maximumLines},ajv=new Ajv({strict:true}),validate=ajv.compile(captureSchema);expect(ajv.validate(outputSchema,output)).toBe(true);
   for(const value of row.outputs){const run=()=>captureOwnedCommandResultV1(process.execPath,["-e","process.stdout.write('x'.repeat(Number(process.argv[1])))",String(value.bytes)],root,{control,env:{...process.env},output});if(!value.accepted){await expect(run()).rejects.toThrow("output limit exceeded");continue;}const captured=await run();expect(validate(captured)).toBe(true);expect(captured.status).toBe(0);expect(captured.signal).toBe(null);const oracle=database.query("select printf('%.*c',?,'x') expected").get(value.bytes)as{expected:string};expect(captured.stdout).toBe(oracle.expected);}
  });
 }finally{database.close();}
},30000);

test("Stdio metadata bounds actual queued tooling custody by its entry native grant",async()=>{
 const root=process.cwd(),prefix="../../../../../🧰️framework/",row=fixture.cargoControl;
 const {withRepositoryToolingStoreV1}=await import(prefix+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts"),{withRepositoryCacheStoreV1}=await import(prefix+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts"),{repositoryLeaseDirectoryV1,repositoryCachePartitionRootV1}=await import(prefix+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts"),leases=await import(prefix+"🔨️modules/🏃️process/🔒️leases/🟦️.ts"),rust=await import(prefix+"🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🦀️rustup/📥️installation/🟦️.ts"),resource="repository-cache-charge:"+repositoryCachePartitionRootV1(root,"tools"),acquire=leases.acquireQueuedResourceLease,write=process.stderr.write.bind(process.stderr);let reached=false,published=false,continued=false,entered=false;
 let ready!:()=>void,release!:()=>void;const acquired=new Promise<void>(accept=>{ready=accept;}),released=new Promise<void>(accept=>{release=accept;});
 const holder=withStdioTestInvocationV1(30000,async original=>withRepositoryCacheStoreV1(root,"leases",original,createIssuedResourceControlV1(original,"cache:store-entry",0),async (store: RepositoryCacheStoreV1)=>{const held=await acquire({directory:repositoryLeaseDirectoryV1(root),resource,mode:"exclusive",owner:crypto.randomUUID(),signal:store.control.signal,control:store.control});try{ready();await released;}finally{held.release();}}));
 const observer=spyOn(leases,"acquireQueuedResourceLease").mockImplementation((options: Parameters<typeof acquireQueuedResourceLease>[0])=>{if(options.resource===resource&&options.mode==="exclusive")reached=true;return acquire(options);}),progress=spyOn(process.stderr,"write").mockImplementation(((chunk:any,...args:any[])=>{if(String(chunk).includes('"command":"cache:tools-store"'))published=true;return(write as any)(chunk,...args);})as any),generation=spyOn(rust,"withRepositoryRustToolchainV1").mockImplementation((...args:any[])=>{continued=true;throw Error("Stdio queue reached Rust custody after expired native grant");});
 try{await Promise.race([acquired,holder.then(()=>{throw Error("Original charge holder ended before queue");})]);const started=performance.now();let failure:unknown;try{await withStdioTestInvocationV1(row.queueChildMilliseconds,original=>stdioCargoMetadata(root,original));}catch(error){failure=error;}expect(failure).toBeInstanceOf(Error);const causes:string[]=[];for(let current:any=failure;current;current=current.cause)causes.push(String(current.message));expect(causes).toContain("Original native child deadline exhausted");expect(performance.now()-started).toBeGreaterThanOrEqual(row.queueChildMilliseconds-20);expect(performance.now()-started).toBeLessThan(row.queueChildMilliseconds+5000);expect(reached).toBe(true);expect(published).toBe(true);expect(continued).toBe(false);}finally{generation.mockRestore();progress.mockRestore();observer.mockRestore();release();await holder;}
 await withStdioTestInvocationV1(30000,original=>withRepositoryToolingStoreV1(root,original,createIssuedResourceControlV1(original,"cache:entry",0),async()=>{entered=true;}));expect(entered).toBe(true);
},30000);
