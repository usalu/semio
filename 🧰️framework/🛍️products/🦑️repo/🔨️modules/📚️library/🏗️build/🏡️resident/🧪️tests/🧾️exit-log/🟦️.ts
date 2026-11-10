/** 🧾️ Contract: the host service keeps a bounded exit log under its runtime directory, and a client that loses the service says why instead of "lost": the reason of an exit row, or a kill without a record with the oom count of the service's cgroup, or that nothing is known. */
import {expect,test} from "bun:test";
import {mkdtempSync,readFileSync,rmSync,statSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {appendServiceExitRowV1,explainServiceLossV1,readServiceExitRowsV1} from "../../🧾️exit-log/🟨️.mjs";

const directory=()=>mkdtempSync(join(tmpdir(),"exit-log-"));

test("the log keeps the newest rows within its byte bound and survives a damaged line",()=>{
 const root=directory();
 try{
  for(let at=0;at<200;at++)appendServiceExitRowV1(root,{event:"exit",at,reason:"stop:SIGTERM",pad:"x".repeat(200)},{maximumBytes:4096,maximumRows:10});
  expect(statSync(join(root,"exit.log")).size).toBeLessThanOrEqual(4096+300);
  const rows=readServiceExitRowsV1(root);expect(rows.length).toBeLessThanOrEqual(10);expect(rows.at(-1)!.at).toBe(199);
  appendServiceExitRowV1(root,{event:"start",at:1000,pid:1});
  expect(readFileSync(join(root,"exit.log"),"utf8").includes("{damaged")).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a loss is explained by the last exit row, by a kill without a record and its cgroup's oom kills, or as unknown",()=>{
 const root=directory(),dead=()=>false;
 try{
  expect(explainServiceLossV1(root)).toBe("the service left no exit record");
  appendServiceExitRowV1(root,{event:"start",at:1,pid:4242,cgroup:"/sys/fs/cgroup/semio-service-graph-x",limitMegabytes:4096});
  expect(explainServiceLossV1(root,{alive:dead,events:()=>({oom_kill:2,high:9})})).toBe("the service was killed without an exit record: its cgroup recorded 2 oom kills (memory limit 4096 MB)");
  expect(explainServiceLossV1(root,{alive:dead,events:()=>({oom_kill:0})})).toContain("no oom kill in its cgroup");
  expect(explainServiceLossV1(root,{alive:dead,events:()=>undefined})).toContain("its cgroup is gone");
  expect(explainServiceLossV1(root,{alive:()=>true})).toBe("the service is still recorded as running");
  appendServiceExitRowV1(root,{event:"exit",at:2,reason:"memory",code:75,rssMegabytes:3300});
  expect(explainServiceLossV1(root)).toBe("the service exited: memory (code 75), 3300 MB resident");
  appendServiceExitRowV1(root,{event:"exit",at:4,reason:"stop:SIGTERM",rssMegabytes:2980,workers:[{index:1,heapMb:900.4,extraMb:300.2,objectCount:1,objectTypes:[],ageMilliseconds:50}]});
  expect(explainServiceLossV1(root)).toContain("2980 MB resident, slice workers held 1201 MB");
  appendServiceExitRowV1(root,{event:"exit",at:3,reason:"crash",error:"Error: boom\n    at x"});
  expect(explainServiceLossV1(root)).toBe("the service exited: crash: Error: boom");
 }finally{rmSync(root,{recursive:true,force:true});}
});
