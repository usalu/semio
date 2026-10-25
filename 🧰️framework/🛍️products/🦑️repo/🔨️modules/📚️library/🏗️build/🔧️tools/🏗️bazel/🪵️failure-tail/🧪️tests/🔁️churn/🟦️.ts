/** 🪵️ Contract: the Bazel lane reads the persisted tails of its failed tasks through the lane-neutral churn classification: edited inputs run the batch again with a cleared failure directory, an outdated implementation restarts the outermost command, an ordinary failure stands, a persistent edit fails after the declared attempts and cancellation ends the wait. */
import {expect,test} from "bun:test";
import {existsSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import churn from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json";
import {ConcurrentEditErrorV1,churnProtocolV1,concurrentEditOfV1} from "../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {bazelFailureChurnV1,createBazelFailureTailV1,runBazelBatchWithChurnV1,writeBazelFailureTailV1} from "../../🟦️.ts";
import vectors from "./🔣️.json";

const protocol=churnProtocolV1();
const tail=(lines:string[])=>{const row=createBazelFailureTailV1();for(const line of lines)row.push(line);return row;};

test("a failed build whose tasks yielded for an outdated implementation restarts the outermost command, one whose inputs were edited runs again, anything else stands",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"churn-tails-"));
 try{
  expect(await bazelFailureChurnV1(directory)).toBeUndefined();
  await writeBazelFailureTailV1(directory,"@a/x:test",tail([vectors.plain]));
  expect(await bazelFailureChurnV1(directory)).toBeUndefined();
  await writeBazelFailureTailV1(directory,"@a/y:build",tail([vectors.exhausted]));
  const edit=await bazelFailureChurnV1(directory);
  expect(concurrentEditOfV1(edit,protocol.profiles.build.rank)).toEqual({reason:"input",path:"@a/y:build",rank:protocol.profiles.build.rank-1});
  expect(concurrentEditOfV1(edit,protocol.profiles.snapshot.rank)).toBeUndefined();
  await writeBazelFailureTailV1(directory,"@a/z:test",tail([vectors.staleEn]));
  const stale=await bazelFailureChurnV1(directory) as Error&{code?:string};
  expect(stale.code).toBe(churn.definitions.Protocol.const.staleImplementationCode);expect(stale.message).toContain("@a/z:test");
  expect(concurrentEditOfV1(stale)).toBeUndefined();
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("a Bazel batch that failed on edited inputs runs again with a cleared failure directory, one that yielded for an outdated implementation restarts the command, an ordinary failure stands, a persistent edit fails after the declared attempts, cancellation ends the wait",async()=>{
 const directory=join(mkdtempSync(join(tmpdir(),"churn-batch-")),"failures");
 const events:string[]=[],options={environment:{LANG:"en"},onRetry:(event:{text:string})=>{events.push(event.text);},sleep:async()=>undefined};
 try{
  let runs=0;
  const result=await runBazelBatchWithChurnV1(directory,async()=>{runs++;if(runs===1){await writeBazelFailureTailV1(directory,"@a/y:build",tail([vectors.exhausted]));throw new Error("bazel build failed");}expect(existsSync(directory)).toBe(false);return "green";},options);
  expect(result).toBe("green");expect(runs).toBe(2);expect(events.length).toBe(1);expect(events[0]).toContain("@a/y:build");expect(events[0]).toContain("attempt 2 of "+protocol.profiles.build.maximumAttempts);
  runs=0;
  const stale=await runBazelBatchWithChurnV1(directory,async()=>{runs++;await writeBazelFailureTailV1(directory,"@a/z:test",tail([vectors.staleDe]));throw new Error("bazel build failed");},options).catch(error=>error);
  expect(runs).toBe(1);expect(stale.code).toBe(churn.definitions.Protocol.const.staleImplementationCode);expect(existsSync(directory)).toBe(true);
  runs=0;
  const ordinary=await runBazelBatchWithChurnV1(directory,async()=>{runs++;await writeBazelFailureTailV1(directory,"@a/x:test",tail([vectors.plain]));throw new Error("compile failed");},options).catch(error=>error);
  expect(runs).toBe(1);expect(ordinary.message).toBe("compile failed");
  runs=0;events.length=0;
  const persistent=await runBazelBatchWithChurnV1(directory,async()=>{runs++;await writeBazelFailureTailV1(directory,"@a/y:build",tail([vectors.exhausted]));throw new Error("bazel build failed");},options).catch(error=>error);
  expect(persistent).toBeInstanceOf(ConcurrentEditErrorV1);expect(runs).toBe(protocol.profiles.build.maximumAttempts);expect(events.length).toBe(protocol.profiles.build.maximumAttempts-1);expect(persistent.path).toBe("@a/y:build");
  const controller=new AbortController();runs=0;
  const cancelled=await runBazelBatchWithChurnV1(directory,async()=>{runs++;await writeBazelFailureTailV1(directory,"@a/y:build",tail([vectors.exhausted]));throw new Error("bazel build failed");},{...options,signal:controller.signal,onRetry:()=>controller.abort(new Error("cancelled by the owner"))}).catch(error=>error);
  expect(cancelled.message).toBe("cancelled by the owner");expect(runs).toBe(1);
 }finally{rmSync(join(directory,".."),{recursive:true,force:true});}
});
