/** 🩹️ Contract: a closure leaf that a crash left zero-filled (or otherwise differing from its name) is retired by the next publication of the same content addressed bytes under writer custody, and the republished leaf hashes to its name (cross-checked against Python hashlib). */
import{test,expect}from"bun:test";
import{mkdirSync,readFileSync,rmSync,statSync,writeFileSync}from"node:fs";
import{createHash}from"node:crypto";
import{join}from"node:path";
import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import{createIssuedResourceControlV1}from"@semio-tech/process/routing";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{adoptStagedGraphLeavesV1}from"../../../🟨️.mjs";
import{inventoryRecordCorruptV1,readInventoryRecordLeanV1}from"../../../🪶️lean-inventory/🟨️.mjs";
import{withRepositoryBuildGraphPublicationFixtureV1}from"../../🟨️.mjs";
import{privateRepositoryWorkspaceV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";

const text=JSON.stringify({version:1,files:["bunfig.toml","tsconfig.json"]}),key=createHash("sha256").update(text).digest("hex");

test("a zero-filled closure leaf is detected as corrupt and replaced by the next publication",async()=>withBuildTestInvocationV1(async original=>{
 const repository=await privateRepositoryWorkspaceV1(),staging=await repository.scratch("torn-"),control=createIssuedResourceControlV1(original,"graph:torn-closure",60000);
 try{
  mkdirSync(join(staging,"build-input-closures"),{recursive:true});writeFileSync(join(staging,"build-input-closures",key+".json"),text);
  await withRepositoryBuildGraphPublicationFixtureV1(repository.root,original,control,async graph=>graph.withWriter(async()=>{
   const directory=join(graph.root,"build-input-closures"),leaf=join(directory,key+".json");
   adoptStagedGraphLeavesV1(repository.root,staging,["build-input-closures/"+key+".json"]);
   expect(readFileSync(leaf,"utf8")).toBe(text);
   writeFileSync(leaf,Buffer.alloc(Buffer.byteLength(text)));
   let caught:unknown;try{readInventoryRecordLeanV1(directory,key,4096);}catch(error){caught=error;}
   expect(inventoryRecordCorruptV1(caught)).toBe(true);
   adoptStagedGraphLeavesV1(repository.root,staging,["build-input-closures/"+key+".json"]);
   expect(readFileSync(leaf,"utf8")).toBe(text);expect(statSync(leaf).nlink).toBe(1);
   expect(readInventoryRecordLeanV1(directory,key,4096).sha256).toBe(key);
   const python=testPythonV1();
   if(python)expect(Bun.spawnSync([python,"-I","-c","import hashlib,sys\nprint(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",leaf]).stdout.toString().trim()).toBe(key);
  }));
 }finally{rmSync(staging,{recursive:true,force:true});}
}),60000);
