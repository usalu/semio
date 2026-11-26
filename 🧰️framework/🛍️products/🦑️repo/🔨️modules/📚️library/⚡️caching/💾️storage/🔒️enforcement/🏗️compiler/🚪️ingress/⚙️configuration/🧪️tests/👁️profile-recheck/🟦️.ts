import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {AsyncLocalStorage} from "node:async_hooks";
import Ajv from "ajv";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {captureCargoProfilePolicySourcesV1,recheckCargoProfileSourceV1,requireLoadedCargoProfilePolicyV1,loadedCargoProfilePolicyDriftedV1} from "../../🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1,withReceivedImplementationPinV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {churnProtocolV1,concurrentEditOfV1,STALE_IMPLEMENTATION_CODE_V1,retryOnConcurrentEditV1} from "../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️.json" with {type:"json"};
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-compiler",maximumElapsedMilliseconds:15000});
/** 🔓️ Runs a body as an unpinned reader: the test process is one process-wide pinned invocation and its receiver runs every body inside a pin, so a law about unpinned behaviour leaves that pin context and suspends the process-wide count for its duration.
 * @see ../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs */
async function unpinnedV1<T>(body:()=>Promise<T>):Promise<T>{const holder=globalThis as Record<symbol,unknown>,count=holder[Symbol.for("semio.implementationPin.count.v1")]as{open:number;processWide:boolean}|undefined,context=holder[Symbol.for("semio.implementationPin.context.v1")]as AsyncLocalStorage<unknown>|undefined;if(count===undefined||context===undefined)throw Error("Actual implementation pin state required");const prior=count.processWide;count.processWide=false;try{return await context.exit(body);}finally{count.processWide=prior;}}


test("captured Cargo profile changes request a new build without admitting different source bytes",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Explicit test artifact directory required");
 const root=mkdtempSync(join(artifacts,"p")),source={path:"profile.json",bytes:Buffer.byteLength(fixture.original),digest:new Bun.CryptoHasher("sha256").update(fixture.original).digest("hex")};
 const path=join(root,source.path),oracle=new Database(":memory:");
 try{
  const classify=oracle.query<{reason:string|null},[string|null,string]>("SELECT CASE WHEN ?1 IS NULL THEN 'vanished' WHEN length(cast(?1 AS blob)) != length(cast(?2 AS blob)) THEN 'length' WHEN ?1 != ?2 THEN 'bytes' END AS reason");
  const protocol=churnProtocolV1();
  await withBuildTestInvocationV1(async invocation=>{
   const control=createIssuedResourceControlV1(invocation,"cargo:profile-source-recheck",0);
   for(const row of fixture.observations){
    expect(classify.get(row.value,fixture.original)).toEqual({reason:row.reason});
    if(row.value===null)rmSync(path);else writeFileSync(path,row.value);
    const verify=()=>recheckCargoProfileSourceV1(root,source,control);
    if(row.reason===null){await expect(verify()).resolves.toBeUndefined();continue;}
    let failure:unknown;
    try{await verify();}catch(error){failure=error;}
    expect(concurrentEditOfV1(failure,protocol.profiles.snapshot.rank)).toBeUndefined();
    expect<unknown>(concurrentEditOfV1(failure,protocol.profiles.build.rank)).toEqual({reason:row.reason,path,rank:protocol.profiles.build.rank-1});
   }
  });
 }finally{oracle.close();rmSync(root,{recursive:true,force:true});}
});


test("changed loaded Cargo profile requests a fresh process without snapshot retries",async()=>{
 const prior='{"version":1,"rows":[1]}', equivalent='{ \"version\": 1, \"rows\": [1] }', changed='{"version":1,"rows":[2]}';
 const digest=new Bun.CryptoHasher("sha256").update(JSON.stringify(JSON.parse(prior))).digest("hex");
 const oracle=new Database(":memory:");try{const same=oracle.query<{equal:number},[string,string]>("select json(?1)=json(?2) as equal");expect(same.get(prior,equivalent)?.equal).toBe(1);expect(same.get(prior,changed)?.equal).toBe(0);}finally{oracle.close();}
 expect(requireLoadedCargoProfilePolicyV1("profile.json",new TextEncoder().encode(equivalent),digest)).toBeUndefined();
 let attempts=0;const failure=await retryOnConcurrentEditV1("snapshot",async()=>{attempts++;requireLoadedCargoProfilePolicyV1("profile.json",new TextEncoder().encode(changed),digest);}).then(()=>null,error=>error);
 expect(attempts).toBe(1);expect(failure.code).toBe(STALE_IMPLEMENTATION_CODE_V1);expect(String(failure)).toContain("profile.json");
});

test("a pinned run keeps the policy it loaded: an edit after load is drift for the next run, never a restart; unpinned it still is refused",()=>unpinnedV1(async()=>{
 const{withImplementationPinV1}=await import("../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs");
 const prior='{"version":1,"rows":[1]}',changed='{"version":1,"rows":[2]}',torn='{"version":',encode=(text:string)=>new TextEncoder().encode(text),digest=new Bun.CryptoHasher("sha256").update(JSON.stringify(JSON.parse(prior))).digest("hex");
 expect(loadedCargoProfilePolicyDriftedV1(encode(changed),digest)).toBe(false);
 expect(await withImplementationPinV1(async()=>[loadedCargoProfilePolicyDriftedV1(encode(prior),digest),loadedCargoProfilePolicyDriftedV1(encode(changed),digest),loadedCargoProfilePolicyDriftedV1(encode(torn),digest)])).toEqual([false,true,true]);
}));

for (const row of fixture.loadedPolicy.cases) test(row.id, async () => {
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Explicit test artifact directory required");
 const root=mkdtempSync(join(artifacts,"p")),relative="profile.json",path=join(root,relative);
 const {initial,replacement}=fixture.loadedPolicy,loaded=row.loaded==="initial"?initial:replacement;
 const oracle=new Database(":memory:");
 try{
  const parse=oracle.query<{canonical:string},[string]>("select json(?1) as canonical");
  const digest=new Bun.CryptoHasher("sha256").update(parse.get(loaded)!.canonical).digest("hex");
  const result=await withBuildTestInvocationV1(async invocation=>{
   const control=createIssuedResourceControlV1(invocation,"cargo:captured-profile",0);
   const producer=row.producer==="descriptor-emitter"?"descriptor-emitter":"artifact";
   const run=async()=>{
    writeFileSync(path,row.edit==="before"?replacement:initial);
    const held=await captureCargoProfilePolicySourcesV1(root,{
     request:{version:1,mode:"release",target:"aarch64-apple-darwin",producer,operation:"build"},
     sources:[{owner:"profile-schema",path:relative,digest}],
    },control);
    if(row.result==="accepted")expect(held.sourceInputs).toEqual([{path:relative,digest,bytes:Buffer.byteLength(loaded)}]);
    if(row.edit==="after")writeFileSync(path,replacement);
    await held.recheck(control);
   };
   return (row.pinned?withReceivedImplementationPinV1(run):unpinnedV1(run)).then(
    ()=>"accepted",
    (error:unknown)=>{
     if((error as {code?:unknown})?.code===STALE_IMPLEMENTATION_CODE_V1)return "stale";
     const drift=concurrentEditOfV1(error,churnProtocolV1().profiles.build.rank);
     if(drift?.path===path&&row.edit==="after")return "drift";
     throw error;
    },
   );
  });
  expect(result).toBe(row.result);
 }finally{oracle.close();rmSync(root,{recursive:true,force:true});}
});
