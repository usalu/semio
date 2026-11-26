import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {OBSERVATION_LIMITS_V1 as limits,settleObservedComputationV1} from "../../🟨️.mjs";

/** ⏳️ Contract: a computation that failed with `BUILD_SOURCE_CONFLICT` (a source read in two states while the replica changed it) is repeated after pauses of `conflictPauseMilliseconds` times four per repetition, at most `conflictRounds` times, before the unchanged failure counts as genuine; the pause schedule equals an independent Python computation.
 * @see ../../🟨️.mjs */
const conflict=()=>Object.assign(Error("Conflicting inference observations: global:[\"closure-graph\"]"),{code:"BUILD_SOURCE_CONFLICT"});
const schedule=(rounds:number,base:number)=>{const run=spawnSync("python3",["-I","-c","import sys;b=int(sys.argv[1]);print(sum(b*4**i for i in range(int(sys.argv[2]))))",String(base),String(rounds)],{encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return Number(run.stdout);};

test("a conflict that heals after two pauses is retried and the computation completes",async()=>{
 let computations=0;const started=performance.now();
 const value=await settleObservedComputationV1(()=>{if(++computations<3)throw conflict();return "settled";});
 expect(value).toBe("settled");expect(computations).toBe(3);
 expect(performance.now()-started).toBeGreaterThanOrEqual(schedule(2,limits.conflictPauseMilliseconds)-5);
});

test("a conflict that persists fails after the bounded rounds and pauses",async()=>{
 let computations=0;const started=performance.now();
 await expect(settleObservedComputationV1(()=>{computations++;throw conflict();})).rejects.toThrow("Conflicting inference observations");
 expect(computations).toBe(limits.conflictRounds+1);
 expect(performance.now()-started).toBeGreaterThanOrEqual(schedule(limits.conflictRounds,limits.conflictPauseMilliseconds)-10);
},30000);

test("a cancelled conflict pause ends with the original reason",async()=>{
 const controller=new AbortController(),reason=Error("stop");let computations=0;
 const pending=settleObservedComputationV1(()=>{computations++;queueMicrotask(()=>controller.abort(reason));throw conflict();},{signal:controller.signal});
 await expect(pending).rejects.toBe(reason);expect(computations).toBe(1);
});
