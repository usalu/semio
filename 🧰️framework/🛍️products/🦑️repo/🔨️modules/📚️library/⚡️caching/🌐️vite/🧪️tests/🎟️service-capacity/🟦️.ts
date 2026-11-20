/** 🎟️ Law: a Vite command service (dev listener) holds no compiler capacity for its life; only its per-transform and lazy-activation work asks for counted capacity, so any number of listeners leave the whole host to the compiles that actually run. */
import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";

test("the listener's own source holds no compiler capacity while it serves",()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),start=source.indexOf("export async function serveViteCommandV1("),end=source.indexOf("\n}\n",start);
 expect(start).toBeGreaterThan(-1);
 const body=source.slice(start,end);
 expect(body).toContain("withCompilerState({capacity:null,");
 expect(body).not.toContain("CompilerCapacityRequestV1");
});
