/** 🔬️ Contract: the supervisor names what the slice worker of a silent read does. It samples once the report of this attempt is `silentMilliseconds` old (not the final report, not an earlier attempt, a valid pid); the summary of a real macOS `sample` report (main thread, own time by frame, end of the heaviest path) equals an independent model in CPython; a live process blocked in synchronous `lstat` calls is named as such; no sampler, a failing sampler or a report without a call graph answer nothing.
 * @see ../🟦️.ts */
import {expect,test} from "bun:test";
import {spawn,spawnSync} from "node:child_process";
import {readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {READY_SAMPLE_POLICY_V1 as P,readySampleFrameV1,readySampleLinesV1,readySampleSummaryV1,readySliceSampleDueV1,readySliceSampleV1} from "../🟦️.ts";

const here=import.meta.dir,fixture=join(here,"🧫️fixtures","📄️lstat-loop.txt"),oracle=join(here,"🐍️.py");
const model=(file:string):unknown=>{const run=spawnSync("python3",["-I",oracle,file],{input:JSON.stringify(P),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const tree=["Call graph:","    9 Thread_1: JavaScriptCore libpas scavenger","    + 9 __psynch_cvwait  (in libsystem_kernel.dylib) + 8  [0x1]","    10 Thread_2   DispatchQueue_1: com.apple.main-thread  (serial)","    + 10 start  (in dyld) + 6652  [0x18a810d7c]","    +   6 ???  (in bun)  load address 0x104d0c000 + 0x9bdd0c  [0x1056c9d0c]","    +   ! 4 lstat  (in libsystem_kernel.dylib) + 8  [0x2]","    +   ! 2 ???  (in <unknown binary>)  [0x122e4c0a4]","    +   4 read  (in libsystem_kernel.dylib) + 8  [0x3]","    9 Thread_3: Heap Helper Thread","    + 9 __psynch_cvwait  (in libsystem_kernel.dylib) + 8  [0x1]","","Total number in stack:"].join("\n");

test("a reading stage is sampled once its own report of this attempt is silent for the window, as the policy declares",()=>{
 const now=10_000_000_000,started=now-6_000_000,due=(report:unknown)=>readySliceSampleDueV1(report,now,started);
 expect(due({atMs:now-P.silentMilliseconds,reason:"interval",pid:42})).toBe(true);
 expect(due({atMs:now-P.silentMilliseconds+1,reason:"interval",pid:42})).toBe(false);
 expect(due({atMs:now-P.silentMilliseconds,reason:"end",pid:42})).toBe(false);
 expect(due({atMs:started-1,reason:"interval",pid:42})).toBe(false);
 expect(due({atMs:now-P.silentMilliseconds,reason:"interval",pid:0})).toBe(false);
 expect(due({atMs:now-P.silentMilliseconds,reason:"interval"})).toBe(false);
 expect(due(null)).toBe(false);
});

test("frames lose their address, JIT code and unsymbolized binaries are named, as the independent model says",()=>{
 expect(readySampleFrameV1("lstat  (in libsystem_kernel.dylib) + 8  [0x18a9c1234]")).toBe("lstat (in libsystem_kernel.dylib)");
 expect(readySampleFrameV1("???  (in <unknown binary>)  [0x122e4c0a4]")).toBe("JIT");
 expect(readySampleFrameV1("???  (in bun)  load address 0x104d0c000 + 0x111ec14  [0x105e2ac14]")).toBe("bun+0x111ec14");
});

test("the summary of the main thread equals the independent model, for a crafted call graph and for a real sample of a bun process",()=>{
 const crafted=join(process.env.TMPDIR??"/tmp","ready-sample-"+process.pid+".txt");writeFileSync(crafted,tree);
 try{
  const summary=readySampleSummaryV1(tree)!;
  expect(summary).toEqual(model(crafted) as never);
  expect(summary).toEqual({total:10,self:[{frame:"lstat (in libsystem_kernel.dylib)",count:4},{frame:"read (in libsystem_kernel.dylib)",count:4},{frame:"JIT",count:2}],path:[{frame:"start (in dyld)",count:10},{frame:"bun+0x9bdd0c",count:6},{frame:"lstat (in libsystem_kernel.dylib)",count:4}]});
 }finally{rmSync(crafted,{force:true});}
 const real=readySampleSummaryV1(readFileSync(fixture,"utf8"))!;
 expect(real).toEqual(model(fixture) as never);
 expect(real.total).toBe(2066);
 expect(real.self.some(row=>row.frame.startsWith("lstat"))).toBe(true);
 expect(readySampleSummaryV1("no call graph here")).toBeUndefined();
});

test("the lines name the process, the window and every frame in English and German without an unreplaced placeholder",()=>{
 const lines=readySampleLinesV1(readySampleSummaryV1(tree)!,{pid:4242,silentSeconds:181,seconds:P.seconds});
 expect(lines[0]).toBe("slice worker 4242 silent for 181 s, sampled for "+P.seconds+" s: 10 samples of its main thread");
 expect(lines.some(line=>line.startsWith("Slice-Worker 4242 seit 181 s still"))).toBe(true);
 expect(lines.filter(line=>line.includes("lstat")).length).toBe(4);
 expect(lines.some(line=>/\{\w+\}/u.test(line))).toBe(false);
});

test("no sampler, a failing sampler and a report without a call graph answer nothing; the sampler is asked with the declared bounds",async()=>{
 expect(await readySliceSampleV1({pid:42,silentSeconds:200,platform:"linux",run:async()=>tree})).toEqual([]);
 expect(await readySliceSampleV1({pid:42,silentSeconds:200,platform:"darwin",run:async()=>{throw Error("denied");}})).toEqual([]);
 expect(await readySliceSampleV1({pid:42,silentSeconds:200,platform:"darwin",run:async()=>"nothing"})).toEqual([]);
 let asked:unknown;
 const lines=await readySliceSampleV1({pid:42,silentSeconds:200,platform:"darwin",run:async(program,args,options)=>{asked={program,args,options};return tree;}});
 expect(asked).toEqual({program:P.program,args:["42",String(P.seconds)],options:{timeout:P.timeoutMilliseconds,maxBuffer:P.outputBytes}});
 expect(lines[0]).toContain("slice worker 42 silent for 200 s");
});

test.skipIf(process.platform!=="darwin")("a live process whose event loop is blocked in synchronous lstat calls is named by the real sampler",async()=>{
 const child=spawn(process.execPath,["-e","const{lstatSync}=require('node:fs');const end=Date.now()+20000;while(Date.now()<end)lstatSync('/');"],{stdio:"ignore"});
 try{
  await new Promise(resolve=>setTimeout(resolve,1000));
  const lines=await readySliceSampleV1({pid:child.pid!,silentSeconds:180});
  expect(lines.length).toBeGreaterThan(4);
  const own=lines.slice(2,lines.indexOf(lines.find(line=>line.startsWith("end of the heaviest"))!));
  expect(own.some(line=>line.includes("lstat"))).toBe(true);
 }finally{child.kill("SIGKILL");}
},60_000);
