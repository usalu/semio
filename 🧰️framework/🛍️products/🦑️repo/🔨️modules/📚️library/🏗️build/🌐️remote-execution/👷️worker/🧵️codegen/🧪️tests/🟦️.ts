import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {readRemoteExecutionPolicyV1} from "../../../🟦️.ts";
import {remoteExecutionCodegenArgumentsV1,remoteExecutionCodegenSerialRequestedV1,remoteExecutionCodegenThreadsV1} from "../🟦️.ts";

const vectors=JSON.parse(readFileSync(join(import.meta.dir,"🔣️.json"),"utf8")) as {policy:{maximumThreads:number;reserveCores:number;extraMegabytesPerThread:number};cases:readonly {id:string;cores:number;load:number;others:number;spare:number;peak:number;threads:number}[]};

test("the thread grant of every vector equals the grant of the independent Python oracle",()=>{
 for(const row of vectors.cases)expect(remoteExecutionCodegenThreadsV1({cores:row.cores,load:row.load,others:row.others,spareMegabytes:row.spare,peakMegabytes:row.peak},vectors.policy),row.id).toBe(row.threads);
 const oracle=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py")],{encoding:"utf8"});
 expect(oracle.status,oracle.stdout+oracle.stderr).toBe(0);
 expect(JSON.parse(oracle.stdout).checked).toBe(vectors.cases.length);
});

test("the authored worker policy grants a whole idle machine's cores (less the reserve, at most the cap) and one thread to a saturated one",()=>{
 const policy=readRemoteExecutionPolicyV1().worker.codegen;
 expect(remoteExecutionCodegenThreadsV1({cores:10,load:0,others:0,spareMegabytes:20000,peakMegabytes:4000},policy)).toBe(Math.min(policy.maximumThreads,10-policy.reserveCores));
 expect(remoteExecutionCodegenThreadsV1({cores:10,load:30,others:0,spareMegabytes:20000,peakMegabytes:4000},policy)).toBe(1);
});

test("one granted thread adds the serial back end once, more leave the arguments alone, and a serial request is recognised in both spellings",()=>{
 const base=["--crate-type","cdylib"];
 expect(remoteExecutionCodegenArgumentsV1(base,1)).toEqual([...base,"-Z","no-parallel-backend"]);
 expect(remoteExecutionCodegenArgumentsV1(base,4)).toBe(base);
 const asked=[...base,"-Z","no-parallel-backend"];
 expect(remoteExecutionCodegenArgumentsV1(asked,1)).toBe(asked);
 expect(remoteExecutionCodegenSerialRequestedV1(["-Zno-parallel-backend"])).toBe(true);
 expect(remoteExecutionCodegenSerialRequestedV1(base)).toBe(false);
});
