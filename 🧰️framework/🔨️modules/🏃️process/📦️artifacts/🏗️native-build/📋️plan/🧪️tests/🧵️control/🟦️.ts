import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import _ from "lodash";
import {declareBuildTestEnvelopeV1,withBuildTestInvocationV1} from "../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {readCargoCompilationGraphV1} from "../../../🕸️units/🟦️.ts";
import {admitCargoSourceBoundariesV1} from "../../../📥️source/🎟️admission/🟦️.ts";
import {captureCargoArtifactPlanFactsV1,cargoArtifactPlanIdentityV1} from "../../🟦️.ts";
import vectors from "../../../🕸️units/🧪️tests/🧫️fixtures/🔣️.json";

declareBuildTestEnvelopeV1({owner:"test:cargo-artifact-plan-control",maximumElapsedMilliseconds:30000,capabilities:{artifacts:true}});
const canonical=(value:unknown):unknown=>Array.isArray(value)?value.map(canonical):value&&typeof value==="object"?Object.fromEntries(_.sortBy(Object.entries(value),row=>row[0]).map(([key,row])=>[key,canonical(row)])):value;
const hash=(value:unknown)=>createHash("sha256").update(JSON.stringify(canonical(value))).digest("hex");

/** 🧪️ Real issued control and actual admission algorithms operate on neutral compiler/source ports, never Native authority. */
test("controlled facts bind actual admission output and stream the same graph as the independent oracle",async()=>withBuildTestInvocationV1(async original=>{
 const control=createIssuedResourceControlV1(original,"test:admitted-plan-facts",10000),row=vectors.cases[0]!,graph=readCargoCompilationGraphV1({version:1,roots:row.roots,units:row.units},vectors.metadata,"/repo"),recipe={version:1 as const,cwd:"/repo",manifest:"/repo/Cargo.toml",command:"cargo",args:["build"],buildDirectory:"/repo/target",compilerIdentity:"cargo fixture",toolchainIdentity:"rustc fixture",inputs:[],environment:[]};
 let reads=0,cfg=0;const bytes=new TextEncoder().encode("pub fn value() -> u8 { 1 }");
 const port={async cfg(){cfg++;return [];},async read(){reads++;return bytes;},async recheck(){}};
 await admitCargoSourceBoundariesV1(graph,"/repo",{version:1,maxSourceBytes:4096,maxSourceFiles:8,maxActiveLines:100},control,port);
 expect(reads).toBe(2);expect(cfg).toBe(2);
 const first=await captureCargoArtifactPlanFactsV1(graph,recipe,"prototype","linux",control,1048576);
 expect(first.source).toBe(hash(graph));expect(first.configuration).toBe(hash(recipe));expect(first.product).toBe(hash([recipe.manifest,recipe.command,recipe.args]));
 expect(graph.units.every(unit=>unit.sourceInputs.length===1)).toBe(true);
 await admitCargoSourceBoundariesV1(graph,"/repo",{version:1,maxSourceBytes:4096,maxSourceFiles:8,maxActiveLines:100},control,{...port,async read(){return new TextEncoder().encode("pub fn value() -> u8 { 2 }");}});
 const second=await captureCargoArtifactPlanFactsV1(graph,recipe,"prototype","linux",control,1048576);expect(cargoArtifactPlanIdentityV1(second)).not.toBe(cargoArtifactPlanIdentityV1(first));
 await expect(captureCargoArtifactPlanFactsV1(graph,recipe,"prototype","linux",control,256)).rejects.toThrow("Artifact plan observation capacity exceeded");
}));

test("controlled fact streaming preserves actual additional cancellation reason",async()=>withBuildTestInvocationV1(async original=>{
 const row=vectors.cases[0]!,graph=readCargoCompilationGraphV1({version:1,roots:row.roots,units:row.units},vectors.metadata,"/repo"),active=createIssuedResourceControlV1(original,"test:plan-cancel-prepare",10000);
 await admitCargoSourceBoundariesV1(graph,"/repo",{version:1,maxSourceBytes:4096,maxSourceFiles:8,maxActiveLines:100},active,{async cfg(){return [];},async read(){return new TextEncoder().encode("pub fn value() {}");},async recheck(){}});
 const cancel=new AbortController(),reason=Error("owned-plan-cancel"),control=createIssuedResourceControlV1(original,"test:plan-cancel",10000,cancel.signal),recipe={version:1 as const,cwd:"/repo",manifest:"/repo/Cargo.toml",command:"cargo",args:["build"],buildDirectory:"/repo/target",compilerIdentity:"cargo fixture",toolchainIdentity:"rustc fixture",inputs:[],environment:[]};
 queueMicrotask(()=>cancel.abort(reason));await expect(captureCargoArtifactPlanFactsV1(graph,recipe,"prototype","linux",control,1048576)).rejects.toBe(reason);
}));
