import {test,expect} from "bun:test";
import {resolve} from "node:path";
import {loadGenerationGraphV1} from "../../🟨️.mjs";

const caching=resolve(import.meta.dirname,"../../../../../../⚡️caching");

test("a refusing preflight stops the graph load before any identity or verification work",async()=>{
 let reached=false;
 const host={preflight:async()=>{throw new Error("stale aggregate");},identity:async()=>{reached=true;return {};}};
 await expect(loadGenerationGraphV1(host as never)).rejects.toThrow("stale aggregate");
 expect(reached).toBe(false);
});

test("the real caching project declares a target for every cache suite, as the independent oracle computes",async()=>{
 const {requireCanonicalTestAggregateV1,canonicalTestAggregatePolicyV1:policy}=await import(resolve(caching,"🗂️suites/🧮️aggregate/🟦️.ts"));
 const project=await Bun.file(resolve(caching,"📋️project.json")).json();
 expect(()=>requireCanonicalTestAggregateV1(project,policy.root)).not.toThrow();
 const run=Bun.spawnSync(["python3","-I",resolve(import.meta.dirname,"🐍️.py"),caching]);
 expect(JSON.parse(run.stdout.toString()).missing).toEqual([]);
});
