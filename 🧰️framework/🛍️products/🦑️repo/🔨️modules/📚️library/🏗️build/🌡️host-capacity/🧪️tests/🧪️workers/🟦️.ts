/** 🧪️ Tests for build host-capacity workers. */
import{expect,test}from"bun:test";
import{existsSync,mkdtempSync,readFileSync,rmSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import{plannedTestWorkersV1,testWorkerLearningV1,withPlannedTestWorkersV1}from"../../🧪️workers/🟦️.ts";
import{repositoryCargoTestPolicyV1}from"../../../../🏃️process/🦀️cargo/🔣️policy/🟦️.ts";
import buildPolicy from"../../../🔣️policy.json";

const job={SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1,id:"k".repeat(22),jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:2048})};
const sandbox=()=>{const directory=mkdtempSync(join(testScratchRootV1("workers"),"library-"));return{directory,environment:{SEMIO_HOST_CAPACITY_DIRECTORY:directory,LANG:"en_US.UTF-8"},remove:()=>rmSync(directory,{recursive:true,force:true})};};

test("The runners learn in the one store of the build: its estimate policy and the directory of its gate",()=>{
 const rooms=sandbox();
 try{
  const learning=testWorkerLearningV1(rooms.environment);
  expect(learning.directory).toBe(rooms.directory);expect(JSON.stringify(learning.policy.estimates)).toBe(JSON.stringify((buildPolicy as unknown as{hostCapacity:{estimates:object}}).hostCapacity.estimates));
  expect(plannedTestWorkersV1("vitest",rooms.environment,"spec").ratioBasisPoints).toBe(10000);
 }finally{rooms.remove();}
});

test("Law: a planned runner under observation teaches the store, the next plan is smaller per worker, a reduced count is worded once, and the override variables of the policy reach the plan",async()=>{
 const rooms=sandbox();
 try{
  const lines:string[]=[];
  for(let run=0;run<4;run++)await withPlannedTestWorkersV1("vitest",{...rooms.environment,...job},"spec",async()=>{await Bun.sleep(1100);},undefined,line=>lines.push(line));
  const stored=JSON.parse(readFileSync(join(rooms.directory,"estimates.json"),"utf8")).classes as Record<string,{samples:unknown[]}>;
  expect(stored["test-worker-vitest:spec"]!.samples).toHaveLength(4);
  const learned=plannedTestWorkersV1("vitest",{...rooms.environment,...job},"spec",undefined,()=>{});
  expect(learned.ratioBasisPoints).toBeLessThan(10000);expect(learned.workerMegabytes).toBeLessThan(plannedTestWorkersV1("vitest",{...job,SEMIO_HOST_CAPACITY_DIRECTORY:join(rooms.directory,"none")},"spec",undefined,()=>{}).workerMegabytes);
  const announced:string[]=[];
  plannedTestWorkersV1("e2e-stacks",{...rooms.environment,...job},"architecture-course",2,line=>announced.push(line));
  expect(announced).toHaveLength(1);expect(announced[0]).toContain("end-to-end stack: 1 of 2 worker(s)");
  expect(plannedTestWorkersV1("playwright",{...rooms.environment,SEMIO_PLAYWRIGHT_WORKERS:"3"},"architecture-course",undefined,()=>{}).workers).toBe(3);
  expect(()=>plannedTestWorkersV1("playwright",{...rooms.environment,SEMIO_PLAYWRIGHT_WORKERS:"0"})).toThrow("SEMIO_PLAYWRIGHT_WORKERS");
 }finally{rooms.remove();}
},60000);

test("The Cargo test policy of an owner plans its assertion threads from the same policy: the host share without a grant, one job under a grant, the override variable when set",()=>{
 const manifest="🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust/Cargo.toml",rooms=sandbox();
 try{
  const base={...process.env,SEMIO_TEST_ARTIFACT_DIR:rooms.directory,SEMIO_BUILD_MODE:"prototype",SEMIO_HOST_CAPACITY_DIRECTORY:rooms.directory};
  delete (base as Record<string,string|undefined>).SEMIO_HOST_CAPACITY_PLAN;
  const open=repositoryCargoTestPolicyV1(manifest,process.cwd(),base).assertionThreads,granted=repositoryCargoTestPolicyV1(manifest,process.cwd(),{...base,...job}).assertionThreads;
  expect(open).toBeGreaterThanOrEqual(1);expect(granted).toBeLessThanOrEqual(4);expect(granted).toBeLessThanOrEqual(open);
  expect(repositoryCargoTestPolicyV1(manifest,process.cwd(),{...base,SEMIO_CARGO_TEST_THREADS:"3"}).assertionThreads).toBe(3);
  expect(()=>repositoryCargoTestPolicyV1(manifest,process.cwd(),{...base,SEMIO_CARGO_TEST_THREADS:"0"})).toThrow("SEMIO_CARGO_TEST_THREADS");
  expect(existsSync(join(rooms.directory,"estimates.json"))).toBe(false);
 }finally{rooms.remove();}
});
