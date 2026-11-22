import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {recheckCargoProfileSourceV1,requireLoadedCargoProfilePolicyV1,loadedCargoProfilePolicyDriftedV1} from "../../🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {churnProtocolV1,concurrentEditOfV1,STALE_IMPLEMENTATION_CODE_V1,retryOnConcurrentEditV1} from "../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️.json" with {type:"json"};
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-compiler",maximumElapsedMilliseconds:15000});


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
    expect(concurrentEditOfV1(failure,protocol.profiles.build.rank)).toEqual({reason:row.reason,path,rank:protocol.profiles.build.rank-1});
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

test("a pinned run keeps the policy it loaded: an edit after load is drift for the next run, never a restart; unpinned it still is refused",async()=>{
 const{withImplementationPinV1}=await import("../../../../../../../../🏗️build/📥️inputs/🔏️revision/🟨️.mjs");
 const prior='{"version":1,"rows":[1]}',changed='{"version":1,"rows":[2]}',torn='{"version":',encode=(text:string)=>new TextEncoder().encode(text),digest=new Bun.CryptoHasher("sha256").update(JSON.stringify(JSON.parse(prior))).digest("hex");
 expect(loadedCargoProfilePolicyDriftedV1(encode(changed),digest)).toBe(false);
 expect(await withImplementationPinV1(async()=>[loadedCargoProfilePolicyDriftedV1(encode(prior),digest),loadedCargoProfilePolicyDriftedV1(encode(changed),digest),loadedCargoProfilePolicyDriftedV1(encode(torn),digest)])).toEqual([false,true,true]);
});
