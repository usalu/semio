import {expect,test} from "bun:test";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {GRAPH_CUT_POLICY_V1,graphCutAfterV1,graphCutConcurrencyV1,graphCutEnvironmentV1,graphCutLearnedV1,graphCutStartLevelV1,graphCutV1} from "../🟨️.mjs";
import {planFarmWorkersV1} from "../../🪜️plan-stage/🟨️.mjs";
import {projectConcurrencyV1} from "../../../🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs";

/** ✂️ Contract: the ladder of the graph work cut is the schema's; every level halves the concurrency and the wave down to one project and one owner; a cut load names the level it needs from how far it was over the limit; the cap reaches the project farm.
 * @see ../🧬️schema/🔣️.json */
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));

test("the policy equals the schema file and every cut matches the Cut definition",()=>{
 expect(GRAPH_CUT_POLICY_V1).toEqual(schema.definitions.Policy.const);
 const ajv=new Ajv({strict:false});ajv.addSchema(schema,"s");
 for(let level=0;level<=GRAPH_CUT_POLICY_V1.levels;level++)expect(ajv.validate({$ref:"s#/definitions/Cut"},graphCutV1(level)),JSON.stringify(ajv.errors)).toBe(true);
});

test("level 0 is the usual work; every level halves concurrency and wave down to one project and the narrowest wave; levels beyond the last stay on it",()=>{
 expect(graphCutV1(0)).toEqual({level:0,waveBasisPoints:10000});
 expect([1,2,3,4,5,6].map(level=>graphCutV1(level).concurrency)).toEqual([4,2,1,1,1,1]);
 expect([1,2,3,4,5,6].map(level=>graphCutV1(level).waveBasisPoints)).toEqual([5000,2500,1250,625,312,156]);
 expect(graphCutV1(99)).toEqual(graphCutV1(6));expect(graphCutV1(-3)).toEqual(graphCutV1(0));expect(graphCutV1(Number.NaN)).toEqual(graphCutV1(0));
 expect(graphCutV1(6).waveBasisPoints).toBeGreaterThanOrEqual(GRAPH_CUT_POLICY_V1.minimumWaveBasisPoints);
});

test("a wave of the policy's 48 owners shrinks to one owner on the last levels, never below one",()=>{
 const wave=(basisPoints:number)=>Math.max(1,Math.floor(48*basisPoints/10000));
 expect([0,1,2,3,4,5,6].map(level=>wave(graphCutV1(level).waveBasisPoints))).toEqual([48,24,12,6,3,1,1]);
});

test("the level after a refusal is one per doubling the tree was over the limit, at least one, and none at the last level",()=>{
 expect(graphCutAfterV1({level:0,treeMegabytes:9966,limitMegabytes:6553})).toBe(1);
 expect(graphCutAfterV1({level:0,treeMegabytes:13200,limitMegabytes:6553})).toBe(2);
 expect(graphCutAfterV1({level:0,treeMegabytes:30000,limitMegabytes:3276})).toBe(4);
 expect(graphCutAfterV1({level:2,treeMegabytes:6600,limitMegabytes:6553})).toBe(3);
 expect(graphCutAfterV1({level:2,treeMegabytes:100,limitMegabytes:6553})).toBe(3);
 expect(graphCutAfterV1({level:5,treeMegabytes:99999,limitMegabytes:1000})).toBe(6);
 expect(graphCutAfterV1({level:6,treeMegabytes:99999,limitMegabytes:1000})).toBeUndefined();
 expect(graphCutAfterV1({level:0,treeMegabytes:5,limitMegabytes:Infinity})).toBe(1);
});

test("the start level is the doublings between the uncut footprint and the plan limit: 16 GB Mac 1, 8 GB machine 2, no limit 0",()=>{
 expect(graphCutStartLevelV1({limitMegabytes:6553})).toBe(1);
 expect(graphCutStartLevelV1({limitMegabytes:3276})).toBe(2);
 expect(graphCutStartLevelV1({limitMegabytes:1500})).toBe(3);
 expect(graphCutStartLevelV1({limitMegabytes:10240})).toBe(0);
 expect(graphCutStartLevelV1({limitMegabytes:Infinity})).toBe(0);expect(graphCutStartLevelV1({limitMegabytes:0})).toBe(0);
 expect(graphCutStartLevelV1({limitMegabytes:1})).toBe(6);
});

test("the environment of a cut load carries the concurrency cap and nothing at level 0; the reader takes only whole numbers of at least one",()=>{
 expect(graphCutEnvironmentV1(graphCutV1(0))).toEqual({});
 expect(graphCutEnvironmentV1(graphCutV1(2))).toEqual({SEMIO_GRAPH_CUT_CONCURRENCY:"2"});
 expect(graphCutConcurrencyV1({SEMIO_GRAPH_CUT_CONCURRENCY:"2"})).toBe(2);
 for(const text of [undefined,"","0","-1","1.5","many"])expect(graphCutConcurrencyV1({SEMIO_GRAPH_CUT_CONCURRENCY:text})).toBe(Infinity);
});

test("the cap reaches the project farm: a cut load starts at most that many workers, also where the plan has no limit; uncut behaviour is unchanged",()=>{
 expect(planFarmWorkersV1({workerMegabytes:1600,residentMegabytes:1000,hostMegabytes:30000,environment:{}})).toBe(Infinity);
 expect(planFarmWorkersV1({workerMegabytes:1600,residentMegabytes:1000,hostMegabytes:30000,environment:{SEMIO_GRAPH_CUT_CONCURRENCY:"2"}})).toBe(2);
 expect(planFarmWorkersV1({workerMegabytes:1600,residentMegabytes:1000,hostMegabytes:16384,environment:{SEMIO_GRAPH_CUT_CONCURRENCY:"1"}})).toBe(1);
 expect(planFarmWorkersV1({workerMegabytes:1600,residentMegabytes:1000,hostMegabytes:16384,environment:{SEMIO_GRAPH_CUT_CONCURRENCY:"64"}})).toBe(3);
 expect(planFarmWorkersV1({workerMegabytes:1600,residentMegabytes:1000,hostMegabytes:16384,environment:{}})).toBe(3);
});

test("the cap reaches the in-process inference: projects at once are the lower of the usual count and the cap, never below one",()=>{
 expect(projectConcurrencyV1({projectConcurrency:8},{})).toBe(8);
 expect(projectConcurrencyV1({projectConcurrency:8},{SEMIO_GRAPH_CUT_CONCURRENCY:"4"})).toBe(4);
 expect(projectConcurrencyV1({projectConcurrency:8},{SEMIO_GRAPH_CUT_CONCURRENCY:"1"})).toBe(1);
 expect(projectConcurrencyV1({projectConcurrency:2},{SEMIO_GRAPH_CUT_CONCURRENCY:"4"})).toBe(2);
 expect(projectConcurrencyV1({},{SEMIO_GRAPH_CUT_CONCURRENCY:"2"})).toBeLessThanOrEqual(2);
});

test("the schema text names the same uncut footprint as the constant, in both languages",()=>{
 expect(String(schema.definitions.Policy.description)).toContain(String(GRAPH_CUT_POLICY_V1.unboundedMegabytes)+" MB");
 expect(String(schema.definitions.Policy.description)).not.toMatch(/\((?:9966 MB, )/u);
});

test("a finished load keeps its level while its peak was near the limit and relaxes one level when the peak was under half of it; an uncut level stays uncut",()=>{
 expect(graphCutLearnedV1({level:2,peakMegabytes:5000,limitMegabytes:6553})).toBe(2);
 expect(graphCutLearnedV1({level:2,peakMegabytes:3277,limitMegabytes:6553})).toBe(2);
 expect(graphCutLearnedV1({level:2,peakMegabytes:3200,limitMegabytes:6553})).toBe(1);
 expect(graphCutLearnedV1({level:1,peakMegabytes:1000,limitMegabytes:Infinity})).toBe(0);
 expect(graphCutLearnedV1({level:0,peakMegabytes:1,limitMegabytes:6553})).toBe(0);
});
