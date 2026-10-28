/** 📐️ Tests for testing workers learning. */
import{expect,test}from"bun:test";
import{existsSync,mkdtempSync,readFileSync,rmSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../🏖️sandbox/🟦️.ts";
import{readHostCapacityPolicyV1}from"../../../../🏗️build/🌡️host-capacity/📏️decision/🟦️.ts";
import{estimateHostRatioV1,readHostEstimateStoreV1}from"../../../../🏗️build/🌡️host-capacity/📐️estimates/🟦️.ts";
import hostFixture from"../../../../🏗️build/🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json";
import estimateFixture from"../../../../🏗️build/🌡️host-capacity/🧪️tests/📐️estimates/🧫️fixtures/🔣️.json";
import{planTestWorkersV1,readTestWorkerPolicyV1,type TestWorkerFactsV1}from"../../🟦️.ts";
import{learnedTestWorkerRatioV1,observeTestWorkerPeakV1,recordTestWorkersV1,testWorkerLineageV1,withTestWorkerLearningV1,type TestWorkerLearningV1}from"../../📐️learning/🟦️.ts";
import corpus from"../🧫️fixtures/🔣️.json";

const host=readHostCapacityPolicyV1({...hostFixture.policy,estimates:estimateFixture.policy}),policy=readTestWorkerPolicyV1(corpus.policy);
const share:TestWorkerFactsV1={source:"grant",cores:8,hostCores:10,memoryMegabytes:1300};
const fresh=()=>{const directory=mkdtempSync(join(testScratchRootV1("workers"),"learning-"));let clock=1_000_000;return{directory,learning:{policy:host,directory,now:()=>clock+=1000} satisfies TestWorkerLearningV1,remove:()=>rmSync(directory,{recursive:true,force:true})};};

test("The classes of a runner run from the runner to what the caller names, with white space and length bounded",()=>{
 expect(testWorkerLineageV1("vitest")).toEqual(["test-worker-vitest"]);
 expect(testWorkerLineageV1("vitest","course test")).toEqual(["test-worker-vitest","test-worker-vitest:course_test"]);
 expect(testWorkerLineageV1("vitest","x".repeat(300))[1]!.length).toBe("test-worker-vitest:".length+120);
 expect(testWorkerLineageV1("bun-test","")).toHaveLength(1);
});

test("Law: what a runner held teaches its class, the next plan fits more workers into the same share, a sibling class borrows, and a run that died early cannot shrink the estimate",async()=>{
 const sandbox=fresh();
 try{
  const before=planTestWorkersV1(policy,"vitest",share,{ratioBasisPoints:learnedTestWorkerRatioV1(sandbox.learning,"vitest","course")});
  expect(before.ratioBasisPoints).toBe(10000);
  for(let run=0;run<5;run++)expect(await recordTestWorkersV1(sandbox.learning,before,"course",{status:"ok",peakMegabytes:Math.round(before.declaredMegabytes*0.5),peakAtMilliseconds:4000,activeMilliseconds:9000})).toBe(true);
  const ratio=learnedTestWorkerRatioV1(sandbox.learning,"vitest","course");
  expect(ratio).toBeLessThan(7000);expect(ratio).toBeGreaterThanOrEqual(5000);
  const after=planTestWorkersV1(policy,"vitest",share,{ratioBasisPoints:ratio});
  expect(after.workers).toBeGreaterThan(before.workers);expect(after.workerMegabytes).toBeLessThan(before.workerMegabytes);
  const sibling=learnedTestWorkerRatioV1(sandbox.learning,"vitest","ui-react"),stranger=learnedTestWorkerRatioV1(sandbox.learning,"cargo-test","course");
  expect(sibling).toBeLessThan(10000);expect(stranger).toBe(10000);
  const stored=readHostEstimateStoreV1(sandbox.directory);
  expect(Object.keys(stored.classes).sort()).toEqual(["test-worker-vitest","test-worker-vitest:course"]);
  expect(await recordTestWorkersV1(sandbox.learning,after,"course",{status:"failed",peakMegabytes:Math.round(after.declaredMegabytes*0.1),peakAtMilliseconds:10,activeMilliseconds:20})).toBe(false);
  expect(learnedTestWorkerRatioV1(sandbox.learning,"vitest","course")).toBe(ratio);
  expect(await recordTestWorkersV1(sandbox.learning,after,"course",{status:"failed",peakMegabytes:after.declaredMegabytes*3,peakAtMilliseconds:10,activeMilliseconds:20})).toBe(true);
  expect(learnedTestWorkerRatioV1(sandbox.learning,"vitest","course")).toBeGreaterThan(ratio);
  expect(await recordTestWorkersV1(sandbox.learning,after,"course",{status:"ok",peakMegabytes:0,peakAtMilliseconds:0,activeMilliseconds:20})).toBe(false);
  expect(estimateHostRatioV1(host,stored,testWorkerLineageV1("vitest","course"),2_000_000).ratioBasisPoints).toBe(ratio);
 }finally{sandbox.remove();}
});

test("The observer keeps the highest tree it saw and the time it first came within 5 % of it, and a platform that cannot observe answers zero",async()=>{
 let clock=0;const rows=[100,300,290,200],seen:number[]=[];
 const observer=observeTestWorkerPeakV1(1234,10,async pid=>{seen.push(pid);return rows[Math.min(seen.length-1,rows.length-1)]!;},()=>clock+=50);
 await Bun.sleep(80);
 const peak=await observer.stop();
 expect(seen.every(pid=>pid===1234)).toBe(true);expect(peak.peakMegabytes).toBe(300);expect(peak.peakAtMilliseconds).toBeGreaterThan(0);
 const blind=observeTestWorkerPeakV1(1,10,async()=>null);
 expect(await blind.stop()).toEqual({peakMegabytes:0,peakAtMilliseconds:0});
});

test("A runner under learning teaches the store once, rethrows its failure, and without learning simply runs",async()=>{
 const sandbox=fresh(),plan=planTestWorkersV1(policy,"bun-test",share,{});
 try{
  expect(await withTestWorkerLearningV1(undefined,plan,"suite",async()=>7)).toBe(7);
  expect(existsSync(join(sandbox.directory,"estimates.json"))).toBe(false);
  expect(await withTestWorkerLearningV1(sandbox.learning,plan,"suite",async()=>{await Bun.sleep(1200);return 9;})).toBe(9);
  const classes=JSON.parse(readFileSync(join(sandbox.directory,"estimates.json"),"utf8")).classes as Record<string,{samples:{ratioBasisPoints:number}[]}>;
  expect(classes["test-worker-bun-test:suite"]!.samples).toHaveLength(1);expect(classes["test-worker-bun-test:suite"]!.samples[0]!.ratioBasisPoints).toBeGreaterThan(0);
  await expect(withTestWorkerLearningV1(sandbox.learning,plan,"suite",async()=>{throw new Error("boom");})).rejects.toThrow("boom");
 }finally{sandbox.remove();}
},30000);
