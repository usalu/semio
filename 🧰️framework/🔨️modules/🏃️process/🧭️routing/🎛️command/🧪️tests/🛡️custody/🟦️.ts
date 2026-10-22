/** 🛡️ Contract: the custody check of one command execution compares a structured copy with the live values, so the null-prototype policy and capabilities the process reader issues never read as a changed custody; Bun 1.4.2's strict deep equality tells a null prototype from an object prototype. */
import{expect,test}from"bun:test";
import{mkdirSync,mkdtempSync}from"node:fs";
import{join}from"node:path";
import{createScriptProcessEnvelope,readScriptPolicy,withScriptProcessEnvelope}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{executeCommandV1}from"../../🟦️.ts";

test("an execution inside a genuine process birth keeps its custody",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Command custody law requires explicit SEMIO_TEST_ARTIFACT_DIR");mkdirSync(output,{recursive:true});
 const root=mkdtempSync(join(output,"custody-law-")),policy={version:1 as const,context:{version:1 as const,cwd:root,cacheRoot:join(root,"cache"),leaseDirectory:join(root,"leases")},budgetMs:20000,maximumOutputBytes:1048576,artifactDirectory:join(root,"captures"),retainArtifacts:false,cargoPolicies:[],vitestPolicy:null,cargoArtifactPolicy:null};
 const request={version:1 as const,cwd:root,command:process.execPath,args:["-e","process.stdout.write('custody-kept')"],manifests:[],preparation:"none" as const},phases:string[]=[];
 await withScriptProcessEnvelope(createScriptProcessEnvelope(readScriptPolicy({version:1,owner:"command:custody-law",maximumElapsedMilliseconds:60000}),{},Date.now()),async invocation=>{
  expect(Object.getPrototypeOf(invocation.policy),"the reader issues null-prototype policy data").toBeNull();
  const result=await executeCommandV1(request,policy,{environment:process.env,invocation,onProgress:event=>{phases.push(event.phase);}});
  expect(result.stdout).toBe("custody-kept");expect(result.status).toBe(0);
 });
 expect(phases).toEqual(["started","completed"]);
});
