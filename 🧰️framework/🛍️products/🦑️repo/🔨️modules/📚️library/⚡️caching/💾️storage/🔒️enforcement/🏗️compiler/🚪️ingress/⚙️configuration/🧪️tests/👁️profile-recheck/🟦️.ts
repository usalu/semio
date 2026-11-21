import "../📮️order.ts";
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {recheckCargoProfileSourceV1} from "../../🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {churnProtocolV1,concurrentEditOfV1} from "../../../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️.json" with {type:"json"};


test("captured Cargo profile changes request a new build without admitting different source bytes",async()=>{
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
 if(!artifacts)throw Error("Explicit test artifact directory required");
 const root=mkdtempSync(join(artifacts,"profile-")),source={path:"profile.json",bytes:Buffer.byteLength(fixture.original),digest:new Bun.CryptoHasher("sha256").update(fixture.original).digest("hex")};
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
