import{test,expect}from"bun:test";
import{writeFileSync,existsSync}from"node:fs";
import{join}from"node:path";
import{privateRepositoryWorkspaceV1}from"../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import{openRepositoryCachePartitionV1}from"../../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts";
import{repositoryCacheBudgetV1}from"../../../../../../💾️storage/🔒️enforcement/📦️budget/🟦️.ts";
import{repositoryLeaseDirectoryV1}from"../../../../../../💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts";
import{concurrentLeaseResourceV1}from"../../../../🚦️concurrent/🟦️.ts";
import{createIssuedResourceControlV1,type ScriptInvocation}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{acquireResourceLease,requireResourceLeaseReadCustodyV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import{readJavascriptDependencyPreservationPresenceV1}from"../../../🟦️.ts";

test("empty preservation census shares actual binding custody with a distinct live reader and opens no dependency image",async()=>{
 const workspace=await privateRepositoryWorkspaceV1("store"),root=workspace.root;
 writeFileSync(join(root,"package.json"),JSON.stringify({name:"presence-private",private:true,workspaces:[],semio:{workspace:{schemaVersion:1,members:["packages/**"],owners:[]}}}));
 const dependencyRoot=repositoryCacheBudgetV1(root).partitions.dependencies.directory,before=existsSync(dependencyRoot);expect(before).toBe(false);
 let ready!:(reader:ScriptInvocation)=>void,finish!:()=>void;
 const waiting=new Promise<ScriptInvocation>(accept=>{ready=accept;}),finished=new Promise<void>(accept=>{finish=accept;});
 const held=workspace.invoke(async reader=>{
  const control=createIssuedResourceControlV1(reader,"test:preservation-reader",0),image=await openRepositoryCachePartitionV1(root,"leases",control);
  try{const directory=repositoryLeaseDirectoryV1(root),resource=concurrentLeaseResourceV1("bindings",root),lease=await acquireResourceLease({directory,resource,mode:"shared",signal:control.signal,control,timeoutMs:0});
   try{ready(reader);await finished;requireResourceLeaseReadCustodyV1(lease,directory,resource,control);expect(existsSync(dependencyRoot)).toBe(before);}
   finally{lease.release();}
  }finally{await image.close();}
 },{owner:"preservation-reader",maximumElapsedMilliseconds:90000});
 let failed=false,failure:unknown;
 try{const reader=await Promise.race([waiting,held.then(()=>{throw Error("Reader custody ended before observation");})]);
  await workspace.invoke(async observer=>{
   expect(observer).not.toBe(reader);const observed=await readJavascriptDependencyPreservationPresenceV1(root,observer,createIssuedResourceControlV1(observer,"test:preservation-observer",0));expect(observed.state).toBe("empty");expect(observed.parents).toBe(1);expect(Object.isFrozen(observed)).toBe(true);
  },{owner:"preservation-observer",maximumElapsedMilliseconds:60000});
 }catch(error){failed=true;failure=error;}
 finally{finish();try{await held;}catch(error){if(!failed){failed=true;failure=error;}}}
 if(failed)throw failure;expect(existsSync(dependencyRoot)).toBe(false);
},120000);
