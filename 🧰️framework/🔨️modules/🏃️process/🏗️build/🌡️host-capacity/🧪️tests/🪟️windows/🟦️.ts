/** 🧪️ Tests for build host-capacity windows. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,requireHostCapacityV1,withHostCapacityV1,type HostSampleV1} from "../../🟦️.ts";
import {cgroupV1CpuFilesV1,observeTreeMegabytesV1,parseProcessTableV1,readHostCoresV1,readHostCountersV1,reduceHostSampleV1,treeMegabytesV1,WINDOWS_PROCESS_TABLE_COMMAND,type HostCommandRunnerV1} from "../../📡️sampling/🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

/** 🪟️ Contract: Windows and cgroup v1 hosts are read through injected ports, from fixtures, on any machine. A Windows host observes its process trees through one PowerShell process table (so the gate learns peaks there like on macOS and Linux), and a host whose probe fails is never silent: the line exists in every language, the sample says `counters: "partial"`, and unknown pressure counts as raised. The tree sum equals an independent Python implementation. */
const here=fileURLToPath(new URL(".",import.meta.url)),policy=readHostCapacityPolicyV1(decisionFixture.policy);

test("the Windows process table is parsed through the shared parser and equals the independent Python tree sum", async () => {
 const rows=parseProcessTableV1(fixture.processTable);expect(rows.length).toBe(5);
 const python=testPythonV1();
 for(const [pid,megabytes] of Object.entries(fixture.trees)){
  expect(treeMegabytesV1(rows,Number(pid)),pid).toBe(megabytes);
  if(python)expect(JSON.parse(spawnSync(python,["-I",join(here,"🧫️fixtures","🐍️tree.py"),fixture.processTable,pid],{encoding:"utf8"}).stdout),"oracle "+pid).toBe(megabytes);
  const calls:string[][]=[];
  const run:HostCommandRunnerV1=async(command,args)=>{calls.push([command,...args]);return fixture.processTable;};
  expect(await observeTreeMegabytesV1(Number(pid),run,"win32",undefined,()=>{throw Error("no notice expected");},"resident")).toBe(megabytes);
  expect(calls).toEqual([["powershell.exe","-NoProfile","-NonInteractive","-Command",WINDOWS_PROCESS_TABLE_COMMAND]]);
 }
 expect(WINDOWS_PROCESS_TABLE_COMMAND).toContain("Win32_Process");expect(WINDOWS_PROCESS_TABLE_COMMAND).toContain("ParentProcessId");expect(WINDOWS_PROCESS_TABLE_COMMAND).toContain("WorkingSetSize");
});

test("a Windows host that cannot list its processes says so once in every language and observes nothing", async () => {
 const lines:string[]=[];
 expect(await observeTreeMegabytesV1(100,async()=>{throw Error("powershell.exe timed out");},"win32",undefined,line=>{lines.push(line);},"resident")).toBeNull();
 expect(lines.length).toBe(2);expect(lines[0]).toContain("cannot be observed on win32");expect(lines[0]).toContain("powershell.exe timed out");expect(lines[1]).toContain("kann unter win32 nicht beobachtet werden");
 const written:string[]=[],original=process.stderr.write.bind(process.stderr);
 (process.stderr as {write:unknown}).write=(chunk:string)=>{written.push(String(chunk));return true;};
 try{for(let round=0;round<3;round++)await observeTreeMegabytesV1(100,async()=>{throw Error("again");},"win32",undefined,undefined,"resident");}finally{(process.stderr as {write:unknown}).write=original;}
 expect(written.length,"announced once per process, not per heartbeat").toBe(2);
});

test("the gate learns the peak of a Windows tree through the process table, and a blind host keeps the declared sizes after saying so", async () => {
 const directory=mkdtempSync(join(tmpdir(),"hcw-")),tree=(pid:number)=>`${pid} 4 51200\n${pid+1} ${pid} 25600\n`;
 const sample=async()=>({host:{version:1 as const,atMs:Date.now(),platform:"win32" as const,cores:8,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal" as const,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:null,openFilesMaximum:null,attachedImages:null} satisfies HostSampleV1,swappedOutMegabytes:0});
 const control={signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined};
 const fast=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20}});
 const run=async(runner:HostCommandRunnerV1,lines:string[])=>{const learned:{peakMegabytes:number}[]=[],notify=(line:string)=>{lines.push(line);};
  await withHostCapacityV1({directory,policy:fast,request:{id:newHostCapacityIdV1(),operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:100,perJobMegabytes:0,cpuPerJob:1},control,environment:{},write:()=>undefined,sample,observe:pid=>observeTreeMegabytesV1(pid,runner,"win32",undefined,notify),learn:row=>{learned.push(row);}},async()=>{await new Promise(done=>setTimeout(done,150));});
  return learned;};
 try{
  const seen:string[]=[],ok=await run(async()=>tree(process.pid),seen);
  expect(ok.length).toBe(1);expect(ok[0]!.peakMegabytes).toBe(75);expect(seen).toEqual([]);
  const blind:string[]=[],none=await run(async()=>{throw Error("no powershell");},blind);
  expect(none,"nothing is learned from a run nobody could observe").toEqual([]);expect(blind.length,"the footprint fallback and the blind process table, each once in every language").toBe(4);expect(blind[0]).toContain("footprint");expect(blind[2]).toContain("cannot be observed");
 }finally{rmSync(directory,{recursive:true,force:true});}
},20000);

test("a failed Windows counter probe marks the sample partial, raises the pressure and stays valid against the closed schema", async () => {
 const lines:string[]=[];
 const counters=await readHostCountersV1({platform:"win32",run:async()=>{throw Error("powershell.exe is not recognised");},read:async()=>null,notify:line=>{lines.push(line);}});
 expect(counters.partial).toBe(true);expect(lines.length).toBe(2);expect(lines[0]).toContain("could not be read");expect(lines[1]).toContain("konnten nicht gelesen werden");
 const facts={platform:"win32" as const,cores:8,totalMegabytes:16384,atMs:5,load:null},healthy={...counters,partial:undefined,availableBytes:16384*1048576};
 const partial=reduceHostSampleV1(policy,facts,{...counters,availableBytes:16384*1048576}),whole=reduceHostSampleV1(policy,facts,healthy as never);
 expect(partial.counters).toBe("partial");expect(partial.pressure,"unknown pressure is never normal").toBe("warn");
 expect("counters" in whole,"a complete sample keeps its old shape").toBe(false);expect(whole.pressure).toBe("normal");
 requireHostCapacityV1("Host",partial);requireHostCapacityV1("Host",whole);
 expect(()=>requireHostCapacityV1("Host",{...partial,counters:"full"})).toThrow();
});

test("a cgroup v1 CPU quota bounds the cores like the v2 quota, from the same kind of fixture", async () => {
 for(const [name,row] of Object.entries(fixture.cgroupV1)){
  const files=new Map(Object.entries(row.files));
  const cores=await readHostCoresV1({platform:"linux",run:async()=>{throw Error("no command expected");},read:async path=>path==="/proc/self/cgroup"?row.membership:files.get(path)??null},row.logical);
  expect(cores,name).toBe(row.cores);
 }
 expect(cgroupV1CpuFilesV1("4:cpu,cpuacct:/a/b\n").map(file=>file.quota)).toEqual(["/sys/fs/cgroup/cpu,cpuacct/a/b/cpu.cfs_quota_us","/sys/fs/cgroup/cpu,cpuacct/a/cpu.cfs_quota_us","/sys/fs/cgroup/cpu,cpuacct/cpu.cfs_quota_us"]);
 expect(cgroupV1CpuFilesV1("0::/x\n3:memory:/y\n")).toEqual([]);expect(cgroupV1CpuFilesV1(null)).toEqual([]);
});

test("the fixture file is the one the tests read", () => {
 expect(JSON.parse(readFileSync(join(here,"🧫️fixtures","🔣️.json"),"utf8")).trees["100"]).toBe(35);
});
