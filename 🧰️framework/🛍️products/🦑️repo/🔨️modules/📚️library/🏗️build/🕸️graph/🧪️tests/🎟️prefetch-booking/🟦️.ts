/** 🎟️ Contract: a prefetching run obtains its replacement reservation before it enters graph custody. The claimed computation only marks the phase, and the reservation is released back to planning when prefetch ends. */
import {graphSourceV1} from "../📖️source/🟦️.ts";
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";

const source=graphSourceV1();
const prefetch=source.indexOf("async prefetch<T>(request:GenerationPrefetchRequestV1,");
const method=source.slice(prefetch,source.indexOf("\n    },",prefetch));
const prefetcher=source.slice(source.indexOf("async function prefetchStaleGenerationV1("),source.indexOf("export function edgeRefreshOwnersV1("));

test("prefetch obtains capacity before custody and marks the phase at the claimed computation",()=>{
 expect(source,"the upgrade before the custody (rev3-g/upgrade-before-custody) lands first").toMatch(/import ?\{markBuildCapacityPhaseV1,upgradeBuildCapacityBeforeRebuildV1\} ?from/u);
 expect(method).toContain("await prepareRebuild(published);");
 expect(method).toContain('book:()=>markBuildCapacityPhaseV1("rebuild")');
 expect(method.indexOf("prepareRebuild")).toBeLessThan(method.indexOf("prefetchStaleGenerationV1"));
 expect(method).toContain('finally{try{releaseCapacity?.();}finally{await markBuildCapacityPhaseV1("plan");}}');
});

test("an exact peer publication can end the capacity wait and restore without starting a rebuild",()=>{
 expect(source).toContain("async()=>available=await published.available()");expect(source).toContain("const restored=await published.restore()");
 expect(source).toContain("if(prepared.restored!==undefined)return prepared.restored;");
 const loader=readFileSync(new URL("../../🕰️generations/🔌️load/🟨️.mjs",import.meta.url),"utf8");
 expect(loader).toContain('attempt(claim??true,store,control,expectedHead),{dirty:[],structural:true},undefined,"restore"');
});

test("the booking happens inside the computation of a claimed chunk, once, before the first project is inferred",()=>{
 const claim=prefetcher.indexOf("await prepareStaleProjectsV1("),run=prefetcher.indexOf("run:async owners=>",claim),book=prefetcher.indexOf("await input.book()",run),infer=prefetcher.indexOf("await prefetchBuildGraphProjectsV1(",run);
 expect(claim).toBeGreaterThan(0);expect(run).toBeGreaterThan(claim);expect(book).toBeGreaterThan(run);expect(infer).toBeGreaterThan(book);
 expect(prefetcher).toContain("if(!booked){booked=true;await input.book();}");
 expect(prefetcher.match(/input\.book\(/gu)?.length).toBe(1);
 expect(prefetcher.slice(0,claim)).not.toContain("input.book(");
});
