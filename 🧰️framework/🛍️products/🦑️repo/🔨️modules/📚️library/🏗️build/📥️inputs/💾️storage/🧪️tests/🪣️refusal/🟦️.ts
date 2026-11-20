import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import{mkdirSync,existsSync,writeFileSync,truncateSync,rmSync}from"node:fs";
import{join}from"node:path";
import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import{createIssuedResourceControlV1}from"@semio-tech/process/routing";
import{nativeStatfsSyncV1}from"@semio-tech/process/artifacts/native-build/storage/platform/contract";
import{adoptStagedGraphLeavesV1}from"../../../🟨️.mjs";
import{withRepositoryBuildGraphPublicationFixtureV1}from"../../🟨️.mjs";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import component from"../../🧬️schema/🔣️.json"with{type:"json"};
import cases from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

test("staged graph leaf admission reports the writer’s original refusal",async()=>withBuildTestInvocationV1(async original=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.validate(schema,cases)).toBe(true);
 using oracle=new Database(":memory:");
 const decide=oracle.query(`SELECT CASE WHEN $bytes>$free THEN 'disk-headroom'
  WHEN $bytes+$held>1048576 THEN 'retained-quota'
  WHEN $busy=1 THEN 'ledger-contention' ELSE 'published' END AS result`);
 const repository=await privateRepositoryWorkspaceV1(),staging=await repository.scratch("adopt-");
 const control=createIssuedResourceControlV1(original,"graph:publication-refusal",60000);
 try{
  mkdirSync(join(staging,"inference","np"),{recursive:true});
  await withRepositoryBuildGraphPublicationFixtureV1(repository.root,original,control,async graph=>graph.withWriter(async()=>{
   let held=0;
   for(const [index,row] of cases.entries()){
    const name=String(index+1).padStart(24,"0")+".json",leaf=join(staging,"inference","np",name),space=nativeStatfsSyncV1(graph.root),free=space.bavail*space.bsize,bytes=row.bytes==="physical"?free+1:row.bytes;
    writeFileSync(leaf,"");
    truncateSync(leaf,bytes);
    using gate=new Database(join(graph.root,component.definitions.OptionalLedger.definitions.Protocol.const.gateFilename),{create:false});
    if(row.gate)gate.exec("begin exclusive");
    try{
     const verdict=decide.get({$bytes:bytes,$free:free,$held:held,$busy:Number(row.gate)}) as {result:string};
     const adopted=adoptStagedGraphLeavesV1(repository.root,staging,["inference/np/"+name]);
     expect(adopted.declined?adopted.reason:"published",row.case).toBe(verdict.result);
     if(adopted.declined)expect(ajv.validate(component.definitions.OptionalPublicationRefusal,adopted.reason)).toBe(true);
     expect(adopted.published,row.case).toBe(Number(verdict.result==="published"));
     expect(existsSync(join(graph.root,"inference","np",name)),row.case).toBe(verdict.result==="published");
     if(!adopted.declined)held+=bytes;
    }finally{if(row.gate)gate.exec("rollback");}
   }
   expect(graph.optionalPublications()).toEqual({attempts:cases.length,declined:cases.length-1});
  }));
 }finally{rmSync(staging,{recursive:true,force:true});}
}),60000);
