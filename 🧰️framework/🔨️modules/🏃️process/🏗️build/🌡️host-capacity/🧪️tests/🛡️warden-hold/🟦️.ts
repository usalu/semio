/** 🛡️ Contract: while the host warden holds (the host memory is tight or critical) the gate admits no new heavy work of any repo command, says why in both languages, keeps the request queued and cancellable, and admits it when the warden is calm again; light work and a missing or stale warden never wait.
 * @see ../../🛡️warden/🧪️tests/🧫️fixtures/🔣️.json (`holds`)
 * @see ../../🎫️gate/🟦️.ts */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readdirSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import {newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostSampleV1} from "../../🟦️.ts";
import type {HostWardenStateV1} from "../../🛡️warden/🟦️.ts";
import decisionFixture from "../🧫️fixtures/🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,heartbeatMilliseconds:20,sampleMaximumAgeMilliseconds:0,lockStaleMilliseconds:2000,staleMilliseconds:60000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hc-hold-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(fixedMegabytes:number)=>({id:newHostCapacityIdV1(),operation:"cargo-build",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes,perJobMegabytes:0,cpuPerJob:1});
const state=(over:Partial<HostWardenStateV1>={}):HostWardenStateV1=>({version:1,atMs:Date.now(),pid:1,mode:"enforce",code:"c",level:"guard",hold:true,trigger:"swap-used",host:{totalMegabytes:16384,availableMegabytes:2300,swapUsedMegabytes:6000,swapOutMegabytesPerSecond:0,compressedMegabytes:1200,pressure:"normal",swapFreeMegabytes:null,swapVolumeFreeMegabytes:null},...over} as HostWardenStateV1);
const options=(directory:string,fixedMegabytes:number,warden:()=>HostWardenStateV1|null,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:request(fixedMegabytes),control:control(),environment:{LANG:"en_US.UTF-8"},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),warden,...over});

test("heavy work waits while the warden holds, says why in the language of the caller, and starts once the warden is calm",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],started:number[]=[],begin=Date.now();let held=true;
 const read=()=>held?state():state({level:"calm",hold:false,trigger:null});
 const en=withHostCapacityV1(options(directory,4000,read,{write:line=>english.push(line)}),async()=>{started.push(Date.now()-begin);});
 const de=withHostCapacityV1(options(directory,4000,read,{write:line=>german.push(line),environment:{LANG:"de_DE.UTF-8"}}),async()=>{started.push(Date.now()-begin);});
 await sleep(150);
 expect(started,"nothing starts while the warden holds").toEqual([]);
 expect(readdirSync(join(directory,"queue")).length,"both requests stay queued").toBe(2);
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
 held=false;await Promise.all([en,de]);
 expect(started).toHaveLength(2);expect(Math.min(...started)).toBeGreaterThanOrEqual(140);
 expect(english.some(line=>line.startsWith("[warden] cargo-build waits: host memory is tight (swap in use is above its line")),english.join("\n")).toBe(true);
 expect(german.some(line=>line.startsWith("[Wächter] cargo-build wartet: der Hostspeicher ist knapp")),german.join("\n")).toBe(true);
 expect(english.at(-1)).toMatch(/^\[capacity\] admitted cargo-build/u);
},30000);

test("light work, a calm, absent or stale warden never hold",async()=>{
 const directory=fresh(),ran:string[]=[];
 await withHostCapacityV1(options(directory,100,()=>state()),async()=>{ran.push("light");});
 await withHostCapacityV1(options(directory,4000,()=>state({level:"calm",hold:false,trigger:null})),async()=>{ran.push("calm");});
 await withHostCapacityV1(options(directory,4000,()=>null),async()=>{ran.push("absent");});
 await withHostCapacityV1(options(directory,4000,()=>state({atMs:Date.now()-120000})),async()=>{ran.push("stale");});
 await withHostCapacityV1(options(directory,4000,()=>{throw Error("unreadable state");}),async()=>{ran.push("unreadable");});
 expect(ran).toEqual(["light","calm","absent","stale","unreadable"]);
},30000);

test("a request held by the warden stays cancellable and leaves no queue entry behind",async()=>{
 const directory=fresh(),abort=new AbortController(),lines:string[]=[];
 const waiting=withHostCapacityV1(options(directory,4000,()=>state(),{control:control(abort.signal),write:line=>lines.push(line)}),async()=>"ran");
 await sleep(80);abort.abort(Error("cancelled by the caller"));
 await expect(waiting).rejects.toThrow();
 expect(lines.some(line=>line.includes("[warden] cargo-build waits"))).toBe(true);
 expect(readdirSync(join(directory,"queue"))).toEqual([]);
 expect(readdirSync(join(directory,"grants"))).toEqual([]);
},30000);
