/** 🕊️ Contract: a process that built the graph and goes on to run for hours drops every cache the construction filled: the interned observation rows are released and collected, and the generation load calls that release once the graph is assembled and before it returns. */
import {graphSourceV1} from "../../../🕸️graph/🧪️tests/📖️source/🟦️.ts";
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";

const module=new URL("../../🟨️.mjs",import.meta.url).href,child=fileURLToPath(new URL("./🟨️.mjs",import.meta.url));

test("releasing the inference caches empties every cache a construction filled",()=>{
 const result=Bun.spawnSync([process.execPath,child,module,"20000"],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const{held,released}=JSON.parse(result.stdout.toString());
 expect(held.interned).toBeGreaterThanOrEqual(20000);expect(held.registryFiles).toBeGreaterThanOrEqual(0);
 const{rowStore,...counts}=released;expect(counts).toEqual({interned:0,analysisImages:0,analysisBytes:0,settledReadBytes:0,fileRecordImageBytes:0,verifiedBlockRows:0,settledReads:0,projectImages:0,publications:0,registryFiles:0});expect({rows:rowStore.rows,strings:rowStore.strings,characters:rowStore.characters}).toEqual({rows:0,strings:0,characters:0});expect(held.rowStore.bytes).toBeGreaterThan(rowStore.bytes);
});

test("the generation load releases once after assembling, on the verified path and on the built path",()=>{
 const text=readFileSync(fileURLToPath(new URL("../../../🕸️graph/🕰️generations/🔌️load/🟨️.mjs",import.meta.url)),"utf8");
 expect(text).toContain("if(!first.needsBuild){await host.release?.();");expect(text.indexOf("await host.release?.();",text.indexOf("if(!first.needsBuild){"))).toBeLessThan(text.indexOf("return{graph:first.graph,stats:first.stats};"));
 const builderAt=text.indexOf("built=await host.withBuilder"),graphAt=text.indexOf("return{graph:built.graph,stats:built.stats};");expect(builderAt).toBeGreaterThan(-1);expect(text.indexOf("await host.release?.();",builderAt)).toBeGreaterThan(builderAt);expect(text.indexOf("await host.release?.();",builderAt)).toBeLessThan(graphAt);
 const host=graphSourceV1();
 expect(host).toContain("release:async(options?:{more?:boolean})=>{observationV1.releaseInferenceMemoryV1(");
});
