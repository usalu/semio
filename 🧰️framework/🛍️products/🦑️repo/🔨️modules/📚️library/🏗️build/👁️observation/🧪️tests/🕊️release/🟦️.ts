/** 🕊️ Contract: a process that built the graph and goes on to run for hours drops every cache the construction filled: the interned observation rows are released and collected, and the generation load calls that release once the graph is assembled and before it returns. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";

const module=new URL("../../🟨️.mjs",import.meta.url).href,child=fileURLToPath(new URL("./🟨️.mjs",import.meta.url));

test("releasing the inference caches empties every cache a construction filled",()=>{
 const result=Bun.spawnSync([process.execPath,child,module,"20000"],{stdout:"pipe",stderr:"pipe"});
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const{held,released}=JSON.parse(result.stdout.toString());
 expect(held.interned).toBeGreaterThanOrEqual(20000);expect(held.registryFiles).toBeGreaterThanOrEqual(0);
 expect(released).toEqual({interned:0,verifiedBlockRows:0,settledReads:0,projectImages:0,publications:0,registryFiles:0});
});

test("the generation load releases once after assembling, on the verified path and on the built path",()=>{
 const text=readFileSync(fileURLToPath(new URL("../../../🕸️graph/🕰️generations/🔌️load/🟨️.mjs",import.meta.url)),"utf8");
 expect(text).toContain("if(!first.needsBuild){await host.release?.();return first.graph;}");
 expect(text.indexOf("const built=await host.withBuilder")).toBeLessThan(text.indexOf("await host.release?.();\n return built.graph;"));
 const host=readFileSync(fileURLToPath(new URL("../../../🕸️graph/🟦️.ts",import.meta.url)),"utf8");
 expect(host).toContain("release:async()=>{observationV1.releaseInferenceMemoryV1();");
});
