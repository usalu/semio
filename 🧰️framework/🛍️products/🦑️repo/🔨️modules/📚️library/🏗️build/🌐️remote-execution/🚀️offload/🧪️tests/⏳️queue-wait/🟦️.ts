import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {OFFLOAD_POLICY_V1,offloadQueueWaitMillisecondsV1} from "../../📐️policy/🟦️.ts";

type Vectors={policy:Record<string,number>;cases:{measured:{p90Milliseconds:number;samples:number}|null;expected:number}[]};
const vectors=JSON.parse(readFileSync(fileURLToPath(new URL("./🔣️.json",import.meta.url)),"utf8")) as Vectors;

test("the vectors carry the shipped queue-wait policy",()=>{
 for(const [name,value] of Object.entries(vectors.policy))expect(OFFLOAD_POLICY_V1.timeouts[name as keyof typeof OFFLOAD_POLICY_V1.timeouts],name).toBe(value);
});

test("a queued unit waits ten times the measured 90th percentile of the queue wait between the floor and the cap, and the fixed wait while the scheduler has too few samples",()=>{
 for(const row of vectors.cases)expect(offloadQueueWaitMillisecondsV1(row.measured??undefined),JSON.stringify(row)).toBe(row.expected);
 expect(offloadQueueWaitMillisecondsV1({p90Milliseconds:Number.NaN,samples:99})).toBe(OFFLOAD_POLICY_V1.timeouts.queueSaturatedMilliseconds);
 expect(offloadQueueWaitMillisecondsV1({p90Milliseconds:-1,samples:99})).toBe(OFFLOAD_POLICY_V1.timeouts.queueSaturatedMilliseconds);
});

test("an independent implementation computes the same waits",()=>{
 const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{encoding:"utf8",input:JSON.stringify(vectors)});
 expect(oracle.stdout.trim(),oracle.stderr).toBe("ok");
});
