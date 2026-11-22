/** 🔒️ Contract: the hold of a native shard is the exclusive lease of its namespace taken without waiting: a shard that a user holds (shared lease, as every Nx run does) answers `undefined` and stays untouched; a free shard answers a release, after which it can be held again.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireResourceLease} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {NATIVE_SHARDS_POLICY_V1,nativeShardNamespaceV1} from "../../🟦️.ts";
import {nativeShardHoldV1} from "../🟦️.ts";

test("a shard held by a user is skipped and a free one is held and released",async()=>{
 const workspace=mkdtempSync(join(tmpdir(),"nsh-")),id="3410c86e36cbdfb40e22aa01a6d35b2124eae49d",directory=join(workspace,...NATIVE_SHARDS_POLICY_V1.leaseDirectory.split("/"));
 mkdirSync(directory,{recursive:true});
 try{
  const hold=nativeShardHoldV1(workspace),user=await acquireResourceLease({directory,resource:nativeShardNamespaceV1(workspace,id).resource,mode:"shared",signal:new AbortController().signal,timeoutMs:0});
  expect(await hold(id)).toBeUndefined();
  await user.release();
  const release=await hold(id);expect(release).toBeDefined();
  expect(await hold(id)).toBeUndefined();
  release!();await new Promise(done=>setTimeout(done,100));
  const again=await hold(id);expect(again).toBeDefined();again!();await new Promise(done=>setTimeout(done,100));
 }finally{rmSync(workspace,{recursive:true,force:true});}
});
