import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {listOriginsV1,readFlatHeadsV1} from "../../📒️journal/🟦️.ts";
import {readAttestationsV1} from "../../🩺️attest/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {blamesCheckV1,blamesStartV1,blamesWatchdogV1,canaryCandidatesV1,canaryLimitsV1,canaryReasonV1,createCanaryTrackerV1,decideCanaryV1,publishRefusalV1,readRefusalsOfV1,readRefusalsV1,refusalsPathV1,requireCanaryShapeV1,type CanaryInputV1,type CanaryPeerV1,type CanaryVerdictV1} from "../🟦️.ts";

setDefaultTimeout(60_000);
const ajv=new Ajv({strict:true,allowUnionTypes:true}),scratch:string[]=[],python=testPythonV1();
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`canary-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const D="d".repeat(64),E="e".repeat(64);

test("the fixture satisfies the closed schema and the Python oracle agrees with every vector",()=>{
 const validate=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Fixture"});
 expect(validate(fixture),JSON.stringify(validate.errors?.[0])).toBe(true);
 requireCanaryShapeV1("Fixture",fixture,"fixture");
 if(python===undefined)return;
 const result=Bun.spawnSync([python,"-I",join(import.meta.dirname,"🐍️.py")],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stdout.toString()+result.stderr.toString()).toBe(0);
});

test("every decision of the language-agnostic fixture is reproduced",()=>{
 expect(fixture.decisions.length).toBeGreaterThan(30);
 for(const item of fixture.decisions)expect(decideCanaryV1(item.input as CanaryInputV1),item.name).toEqual(item.expected as CanaryVerdictV1);
});

test("every tracker sequence of the language-agnostic fixture is reproduced",()=>{
 for(const item of fixture.trackers){
  const tracker=createCanaryTrackerV1(item.freshMs);
  for(const [index,step] of item.steps.entries())expect(tracker.observe(item.digest,item.originId,step.peers as CanaryPeerV1[],step.now),`${item.name} #${index}`).toEqual(step.expected);
 }
});

test("a tracker starts again when the digest changes",()=>{
 const tracker=createCanaryTrackerV1(),peer=(code:string,at:number):CanaryPeerV1=>({originId:"g1-server",groupId:"g1",code,wallMs:at,sync:true,fallback:false});
 expect(tracker.observe(D,"g1-m02",[peer(D,1000)],1000)).toEqual({"g1-server":0});
 expect(tracker.observe(D,"g1-m02",[peer(D,6000)],6000)).toEqual({"g1-server":5000});
 expect(tracker.observe(E,"g1-m02",[peer(E,7000)],7000)).toEqual({"g1-server":0});
 expect(tracker.observe(D,"g1-m02",[peer(D,8000)],8000)).toEqual({"g1-server":0});
});

test("the deadlock cannot occur: whatever the closure, a fleet that reads the same store lets exactly one origin go first, then the next one a turn later",()=>{
 const origins=["g1-server","g1-m01","g1-m02","g1-m03","g1-m04","g1-m06","g1-m07"],now=5_000_000_000,peers:CanaryPeerV1[]=origins.map(originId=>({originId,groupId:"g1",code:"1".repeat(64),wallMs:now-2000,sync:true,fallback:false}));
 const ask=(originId:string,waitedMs:number):CanaryVerdictV1=>decideCanaryV1({digest:D,running:"1".repeat(64),originId,groupId:"g1",now,waitedMs,authored:false,peers,retired:[],snapshots:[],refusals:[],healthy:{},limits:canaryLimitsV1});
 for(const waited of [0,60_000,119_999])expect(origins.filter(origin=>ask(origin,waited).ok),`waited ${waited}`).toEqual([]);
 expect(origins.filter(origin=>ask(origin,120_000).ok)).toEqual(["g1-server"]);
 expect(origins.filter(origin=>ask(origin,419_999).ok)).toEqual(["g1-server"]);
 expect(origins.filter(origin=>ask(origin,420_000).ok)).toEqual(["g1-server","g1-m01"]);
 expect(origins.filter(origin=>ask(origin,120_000+6*300_000).ok)).toEqual(origins);
 const order=canaryCandidatesV1({originId:"g1-m02",groupId:"g1",now,peers,retired:[],limits:canaryLimitsV1});
 expect(order).toEqual(["g1-server","g1-m01","g1-m02","g1-m03","g1-m04","g1-m06","g1-m07"]);
});

test("a canary that fails is refused for everybody: after the refusal nobody follows and nobody leads, and a follower goes only after the canary stayed healthy",()=>{
 const now=9_000_000_000,peers:CanaryPeerV1[]=[{originId:"g1-server",groupId:"g1",code:D,wallMs:now,sync:true,fallback:false},{originId:"g1-m02",groupId:"g1",code:"1".repeat(64),wallMs:now,sync:true,fallback:false}];
 const base:CanaryInputV1={digest:D,running:"1".repeat(64),originId:"g1-m02",groupId:"g1",now,waitedMs:900_000,authored:false,peers,retired:[],snapshots:[],refusals:[],healthy:{"g1-server":30_000},limits:canaryLimitsV1};
 expect(decideCanaryV1(base)).toEqual({ok:false,why:"canary",origin:"g1-server",seconds:30});
 expect(decideCanaryV1({...base,healthy:{"g1-server":60_000}})).toEqual({ok:true,role:"follower"});
 const refused={...base,healthy:{"g1-server":600_000},refusals:[{digest:D,originId:"g1-server",reason:"watchdog",wallMs:now}]};
 expect(decideCanaryV1(refused)).toEqual({ok:false,why:"refused",origin:"g1-server",seconds:0});
 expect(decideCanaryV1({...refused,peers:[peers[1]!]})).toEqual({ok:false,why:"refused",origin:"g1-server",seconds:0});
});

test("a stale server does not hold the fleet: the lowest fresh id leads, and a server that comes back is first again",()=>{
 const now=7_000_000_000,mk=(originId:string,age:number):CanaryPeerV1=>({originId,groupId:"g1",code:"1".repeat(64),wallMs:now-age,sync:true,fallback:false});
 const input=(originId:string,peers:readonly CanaryPeerV1[]):CanaryInputV1=>({digest:D,running:"1".repeat(64),originId,groupId:"g1",now,waitedMs:120_000,authored:false,peers,retired:[],snapshots:[],refusals:[],healthy:{},limits:canaryLimitsV1});
 const stale=[mk("g1-server",120_000),mk("g1-m01",1000),mk("g1-m02",1000)];
 expect(decideCanaryV1(input("g1-m01",stale)).ok).toBe(true);
 expect(decideCanaryV1(input("g1-m02",stale)).ok).toBe(false);
 const back=[mk("g1-server",1000),mk("g1-m01",1000),mk("g1-m02",1000)];
 expect(decideCanaryV1(input("g1-m01",back)).ok).toBe(false);
 expect(decideCanaryV1(input("g1-server",back)).ok).toBe(true);
});

test("the waiting texts are English and German, name the origin and the seconds, and keep the phrase the boot tests and operators know",()=>{
 const mixed=canaryReasonV1({ok:false,why:"mixed",origin:"g1-server",seconds:90}),turn=canaryReasonV1({ok:false,why:"turn",origin:"g1-m01",seconds:300}),canary=canaryReasonV1({ok:false,why:"canary",origin:"g1-server",seconds:12}),refused=canaryReasonV1({ok:false,why:"refused",origin:"g1-m07",seconds:0});
 expect(mixed).toContain("no origin runs this code yet");
 for(const text of [mixed,turn,canary,refused]){expect(text.split(" / ").length).toBeGreaterThanOrEqual(2);expect(text).toMatch(/g1-/u);}
 expect(mixed).toContain("90 s");expect(canary).toContain("12 s");expect(canary).toContain("gesund");expect(refused).toContain("abgelehnt");
});

test("a refusal is published once to the own record, read back from every origin, capped at 64, and foreign, torn, open or AppleDouble records are ignored",async()=>{
 const store=temp("store");
 expect(await publishRefusalV1(store,"g1-m01",{digest:D,reason:"self-check failed"},1000)).toBe(true);
 expect(await publishRefusalV1(store,"g1-m01",{digest:D,reason:"again"},2000)).toBe(false);
 expect(await publishRefusalV1(store,"g1-m02",{digest:E,reason:"x".repeat(900)},3000)).toBe(true);
 expect(await readRefusalsOfV1(store,"g1-m01")).toEqual([{digest:D,reason:"self-check failed",wallMs:1000}]);
 const all=await readRefusalsV1(store);
 expect(all.map(item=>[item.originId,item.digest]).sort()).toEqual([["g1-m01",D],["g1-m02",E]]);
 expect(all.find(item=>item.originId==="g1-m02")!.reason.length).toBe(512);
 writeFileSync(join(store,"v1","origins","g1-m01","._refused.json"),"AppleDouble");
 mkdirSync(join(store,"v1","origins","g1-m03"),{recursive:true});writeFileSync(refusalsPathV1(store,"g1-m03"),"{torn");
 mkdirSync(join(store,"v1","origins","g1-m04"),{recursive:true});writeFileSync(refusalsPathV1(store,"g1-m04"),JSON.stringify({version:1,originId:"g1-m04",refused:[{digest:D,reason:"r",wallMs:1,extra:true}]}));
 mkdirSync(join(store,"v1","origins","g1-m05"),{recursive:true});writeFileSync(refusalsPathV1(store,"g1-m05"),JSON.stringify({version:1,originId:"g1-m01",refused:[{digest:D,reason:"r",wallMs:1}]}));
 expect((await readRefusalsV1(store)).map(item=>item.originId).sort()).toEqual(["g1-m01","g1-m02"]);
 for(let n=0;n<70;n++)await publishRefusalV1(store,"g1-m06",{digest:n.toString(16).padStart(64,"0"),reason:"r",wallMs:0},n);
 const kept=await readRefusalsOfV1(store,"g1-m06");
 expect(kept!.length).toBe(64);expect(kept![0]!.wallMs).toBe(6);expect(kept!.at(-1)!.wallMs).toBe(69);
});

test("the store readers ignore the AppleDouble files that macOS NFS clients leave next to every file",async()=>{
 const store=temp("store"),origins=join(store,"v1","origins"),heads=join(store,"v1","heads");
 mkdirSync(join(origins,"g1-m01"),{recursive:true});mkdirSync(heads,{recursive:true});
 writeFileSync(join(origins,"._g1-m01"),"x");writeFileSync(join(origins,"g1-m01","._attest.json"),"x");writeFileSync(join(heads,"._g1-m01_1_2_3_1794638206411-000000-g1-m01"),"");
 expect(await listOriginsV1(store)).toEqual(["g1-m01"]);
 expect(await readAttestationsV1(store)).toEqual({});
 expect([...(await readFlatHeadsV1(store)).keys()]).toEqual([]);
});

test("a failed start and an end by the watchdog blame the new code, a revert and an old code do not",()=>{
 expect(blamesStartV1("the worktree code does not start: exit 1")).toBe(true);
 expect(blamesCheckV1("self-check exited with 1: SyntaxError")).toBe(true);
 for(const reason of ["self-check timed out after 180000 ms","self-check cancelled","self-check could not start: ENOENT"])expect(blamesCheckV1(reason)).toBe(false);
 expect(blamesStartV1("the code on disk ran before and was superseded")).toBe(false);
 const supervisor=readFileSync(join(import.meta.dirname,"..","..","🔁️replica","🛟️supervisor","🟦️.ts"),"utf8");
 expect(supervisor).toContain("the worktree code does not start");
 expect(blamesWatchdogV1({firstRunMs:1000,endedMs:1000+60_000,windowMs:1_800_000})).toBe(true);
 expect(blamesWatchdogV1({firstRunMs:1000,endedMs:1000+1_800_000,windowMs:1_800_000})).toBe(true);
 expect(blamesWatchdogV1({firstRunMs:1000,endedMs:1001+1_800_000,windowMs:1_800_000})).toBe(false);
 expect(blamesWatchdogV1({firstRunMs:5000,endedMs:4000,windowMs:1_800_000})).toBe(false);
 expect(blamesWatchdogV1({firstRunMs:undefined,endedMs:4000,windowMs:1_800_000})).toBe(false);
 expect(blamesWatchdogV1({firstRunMs:5000,endedMs:undefined,windowMs:1_800_000})).toBe(false);
});
