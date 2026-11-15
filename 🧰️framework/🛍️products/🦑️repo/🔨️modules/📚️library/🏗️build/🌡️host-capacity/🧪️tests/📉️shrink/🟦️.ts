/** 📉️ Contract: a request that can run smaller than it asks is admitted with what the machine can give instead of waiting for memory that a 16 GB host never has; a request that can never fit is refused at once and one that is starved (nothing of ours runs, not even its minimum is free) after the override time, both with the numbers and what to do in English and German; a request without a minimum still waits as before; a request that fits is admitted in full as soon as free memory minus the floor equals it.
 * @see ./🐍️oracle/🐍️.py */
import {afterAll,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {evaluateHostCapacityV1,newHostCapacityIdV1,readHostCapacityPolicyV1,sizeHostRequestV1,withHostCapacityV1,withUnmanagedHostV1,type HostRequestV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {reserveGraphPlanCapacityV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"shrink-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const policy=readHostCapacityPolicyV1(buildPolicy.hostCapacity);
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,load:{...buildPolicy.hostCapacity.load,overrideAfterMilliseconds:200},wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const host=(total:number,available:number):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:total,availableMegabytes:available,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const request=(fixed:number,minimum?:number):HostRequestV1=>({id:newHostCapacityIdV1(),operation:"graph-plan",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:fixed,perJobMegabytes:0,cpuPerJob:1,...minimum===undefined?{}:{minimumMegabytes:minimum},label:"graph-plan"});
const decide=(total:number,available:number,fixed:number,minimum:number|undefined,waited:number)=>{
 const item=request(fixed,minimum),sample=host(total,available),evaluation=evaluateHostCapacityV1(policy,withUnmanagedHostV1(sample,[]),[],[{request:item,arrivedAtMs:1000,pid:process.pid}],1000+waited),row=evaluation.decisions[0]!;
 return [row.verdict,row.reservedMegabytes,row.reason];
};
const cases=[
 {total:16384,available:7830,fixed:6627,minimum:2048,waited:0},{total:16384,available:8875,fixed:8192,minimum:2048,waited:0},{total:16384,available:6624,fixed:6627,minimum:2048,waited:0},
 {total:16384,available:12000,fixed:6627,minimum:2048,waited:0},{total:16384,available:3000,fixed:6627,minimum:2048,waited:0},{total:16384,available:3000,fixed:6627,minimum:2048,waited:200000},
 {total:4096,available:3000,fixed:6627,minimum:3072,waited:0},{total:16384,available:8675,fixed:6627,minimum:2048,waited:0},{total:16384,available:8674,fixed:6627,minimum:2048,waited:0},
 {total:16384,available:7830,fixed:6627,waited:0},{total:32768,available:30000,fixed:8192,minimum:2048,waited:0},{total:8192,available:5000,fixed:4096,minimum:2048,waited:0},{total:8192,available:2500,fixed:4096,minimum:2048,waited:0},
];
const oracle=()=>{
 const run=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️oracle/🐍️.py",import.meta.url))],{input:JSON.stringify({policy:{...policy.memory,overrideAfterMilliseconds:policy.load.overrideAfterMilliseconds},cases}),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as [string,number,string][];
};

test("the decision equals an independent Python oracle on 16 GB, 8 GB, 4 GB and 32 GB hosts, and grants as soon as free memory above the floor equals the request",()=>{
 const expected=oracle();
 cases.forEach((item,index)=>expect(decide(item.total,item.available,item.fixed,item.minimum,item.waited),JSON.stringify(item)).toEqual(expected[index]!));
 expect(expected[0]).toEqual(["admit",5782,"shrunk"]);expect(expected[7]).toEqual(["admit",6627,"fits"]);expect(expected[8]).toEqual(["admit",6626,"shrunk"]);
 expect(expected[6]).toEqual(["wait",0,"never"]);expect(expected[5]).toEqual(["wait",0,"starved"]);expect(expected[9]).toEqual(["wait",0,"memory"]);
});

const control=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const gate=(directory:string,item:HostRequestV1,sample:HostSampleV1,lines:string[],language="en_US.UTF-8")=>withHostCapacityV1({directory,policy:fast,request:item,control:control(),environment:{LANG:language,LC_ALL:language},write:line=>{lines.push(line);},sample:async()=>({host:sample,swappedOutMegabytes:0})},async handle=>handle.reservedMegabytes);

test("the refusal the workers met: 6627 MB asked at 7830 MB reported free on a 16 GB host waits for ever without a minimum, and is granted what is free with one",async()=>{
 const directory=fresh(),lines:string[]=[],waiting=gate(directory,request(6627),host(16384,7830),lines).then(()=>"granted",()=>"refused"),first=await Promise.race([waiting,new Promise<string>(done=>setTimeout(()=>done("still waiting"),600))]);
 expect(first).toBe("still waiting");expect(lines.some(line=>line.includes("waiting")&&line.includes("need 6627")),lines.join("\n")).toBe(true);
 const granted=await gate(fresh(),request(6627,2048),host(16384,7830),lines);
 expect(granted).toBe(5782);expect(lines.some(line=>line.includes("5782 of 6627")&&line.includes("2048")),lines.join("\n")).toBe(true);
},60000);

test("a request whose minimum can never fit is refused at once in English and German with the numbers and the way out",async()=>{
 const started=Date.now(),english=await gate(fresh(),request(6627,3072),host(4096,3000),[]).then(()=>"",error=>String(error.message)),german=await gate(fresh(),request(6627,3072),host(4096,3000),[],"de_DE.UTF-8").then(()=>"",error=>String(error.message));
 expect(Date.now()-started).toBeLessThan(2000);
 for(const text of [english,german])for(const number of ["3072","4096","1536","2560","4608"])expect(text,text).toContain(number);
 expect(english).toContain("fleet cache");expect(english).toContain("more memory");expect(german).toContain("Speicher");expect(german).toContain("Flottencache");
},60000);

test("a request that nothing of ours can free memory for is refused after the override time with the free numbers and what to close, in both languages",async()=>{
 const started=Date.now(),english=await gate(fresh(),request(6627,2048),host(16384,3000),[]).then(()=>"",error=>String(error.message)),german=await gate(fresh(),request(6627,2048),host(16384,3000),[],"de_DE.UTF-8").then(()=>"",error=>String(error.message));
 expect(Date.now()-started).toBeGreaterThanOrEqual(200);expect(Date.now()-started).toBeLessThan(5000);
 for(const text of [english,german]){expect(text).toContain("2048");expect(text).toContain("952");expect(text).toContain("2048");expect(text).toContain("16384");}
 expect(english).toContain("repo service host stop");expect(german).toContain("Anwendungen schließen");
},60000);

test("the plan reservation of the graph step carries the minimum and answers the megabytes it was granted",async()=>{
 const directory=fresh(),lines:string[]=[],reservation=await reserveGraphPlanCapacityV1({megabytes:8192,minimumMegabytes:2048,environment:{LANG:"en_US.UTF-8"},directory,policy:fast,write:line=>{lines.push(line);},sample:async()=>({host:host(16384,7830),swappedOutMegabytes:0})});
 expect(reservation.megabytes).toBe(5782);reservation.release();
});

test("the doors that read or install the graph (graph, show, dependencies) carry a minimum and are admitted with what a 16 GB host has free instead of waiting for their fixed block",()=>{
 const room=(name:string)=>{const item=sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:name})!,evaluation=evaluateHostCapacityV1(policy,withUnmanagedHostV1(host(16384,7000),[]),[],[{request:item,arrivedAtMs:1000,pid:process.pid}],1000);return{item,row:evaluation.decisions[0]!};};
 for(const name of ["graph","show","dependencies"]){const{item,row}=room(name);expect(item.minimumMegabytes,name).toBe(3072);expect(item.fixedMegabytes,name).toBeGreaterThan(4952);expect([row.verdict,row.reason,row.reservedMegabytes],name).toEqual(["admit","shrunk",4952]);}
 expect(sizeHostRequestV1(policy,{id:newHostCapacityIdV1(),operation:"command"})!.minimumMegabytes).toBeUndefined();
});
