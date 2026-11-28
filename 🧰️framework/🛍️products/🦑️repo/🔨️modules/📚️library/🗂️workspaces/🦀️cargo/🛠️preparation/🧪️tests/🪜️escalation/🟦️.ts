/** 🪜️ Contract: Cargo preparation plans, hashes and confirms its receipt under the shared lease, so independent runs overlap; the exclusive lease is held only to renew a receipt (update and locked fetch), a peer that renewed the same receipt meanwhile is accepted without a second update, and a shared holder never blocks the planning of another run. */
import {registerBuildTestLawV1}from"@semio-tech/process/testing/invocation";
import {expect}from"bun:test";
import {mkdirSync,mkdtempSync,readFileSync,writeFileSync}from"node:fs";
import {join}from"node:path";
import {setTimeout as delay}from"node:timers/promises";
import {createIssuedResourceControlV1,createRequestedIssuedResourceControlV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireQueuedResourceLease}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {nativeCallerPhaseV1}from"../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {cargoPreparationControlV1,repositoryCargoPreparationStorageV1}from"../../../../../⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🛠️preparation/🟦️.ts";
import {privateRepositoryWorkspaceV1,testCheckoutRootV1}from"../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {withPreparedCargoDependencyPairV1}from"../../🟦️.ts";
const repository=await privateRepositoryWorkspaceV1();await repository.prepare("leases","rust","tools");

registerBuildTestLawV1("Cargo preparation overlaps its planning under the shared lease and renews the receipt once under the exclusive lease","caller-birth",async original=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit artifact storage required");
 const base=mkdtempSync(join(output,"cargo-escalation-")),root=join(base,"workspace");mkdirSync(join(root,"kernel"),{recursive:true});
 writeFileSync(join(root,"Cargo.toml"),'[workspace]\nresolver="2"\nmembers=["kernel"]\n[workspace.metadata.semio.repository]\nschema-version=1\nexclude-patterns=[]\nowner-manifests=["*/Cargo.toml"]\nmember-manifests=["kernel/Cargo.toml"]\n[workspace.package]\nedition="2021"\n');
 const member=join(root,"kernel/Cargo.toml");writeFileSync(member,'[package]\nname="escalation-kernel"\nversion="0.1.0"\nedition.workspace=true\n[lib]\npath="🦀️.rs"\n');writeFileSync(join(root,"kernel/🦀️.rs"),"pub fn kernel() {}\n");
 const storage=repositoryCargoPreparationStorageV1(repository.root,testCheckoutRootV1()),resource=`cargo-preparation:${root}`;
 const commands:string[]=[];let active=0,peak=0,identityCalls=0;
 const runner=async(args:string[])=>{commands.push(args[0]!);await delay(250);if(args[0]==="update")writeFileSync(join(root,"Cargo.lock"),"# escalation\nversion = 4\n");};
 const identity=async()=>{identityCalls++;active++;peak=Math.max(peak,active);await delay(700);active--;return"cargo escalation 1.0";};
 let sequence=0;
 const run=()=>{const controller=new AbortController(),parent=createRequestedIssuedResourceControlV1(createIssuedResourceControlV1(original,"custody:escalation-"+sequence++,0),180000,controller.signal),phase=nativeCallerPhaseV1(original,process.cwd(),parent);return withPreparedCargoDependencyPairV1(storage,root,"Cargo.toml",phase.leaseControl,runner,identity,cargoPreparationControlV1(phase));};

 await Promise.all([run(),run()]);
 expect(peak,"two runs without a receipt plan at the same time").toBe(2);
 expect(commands.filter(command=>command==="update").length,"the follower accepts the receipt its peer renewed").toBe(1);
 expect(commands.filter(command=>command==="fetch").length).toBe(1);

 commands.length=0;await run();
 expect(commands,"a current receipt runs nothing").toEqual([]);

 writeFileSync(member,readFileSync(member,"utf8")+"# edited\n");commands.length=0;
 const before=identityCalls,holder=await acquireQueuedResourceLease({directory:storage.leaseDirectory,resource,mode:"shared",owner:"independent-shared-holder",signal:new AbortController().signal});
 let settled=false;const pending=run().then(()=>{settled=true;});
 try{
  const guard=Date.now()+60000;while(identityCalls===before&&Date.now()<guard)await delay(50);
  expect(identityCalls,"a stale receipt is planned while a peer holds the shared lease").toBe(before+1);
  await delay(1500);
  expect(settled,"the renewal waits for the exclusive lease behind the shared holder").toBe(false);expect(commands).toEqual([]);
 }finally{holder.release();}
 await pending;
 expect(commands,"the renewal ran update and locked fetch once").toEqual(["update","fetch"]);
});
