/** 🔗️ Contract: when the environment of a generation moved, every untagged owner of a real project root that is not inferred again gets its edges again, withheld owners included (their fragment names the environment it was published for, so a generation with one stale owner is refused as incomplete); nothing is refreshed on a cold start or while the environment stays; when the consumers of the edit are known, a kept project that is no consumer keeps its edges (carried). The independent oracle is a Python model of the same rule over the same worlds. */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{edgeRefreshOwnersV1}from"../../🟦️.ts";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
const call=(w:(typeof fixture)["worlds"][number]["world"])=>edgeRefreshOwnersV1({cold:w.cold,environment:w.environment,previousEnvironment:w.previousEnvironment,roots:new Set(w.roots),stubs:w.stubs as never,inferred:new Set(w.inferred),...("consumers" in w?{consumers:new Set(w.consumers as string[])}:{})});

for(const row of fixture.worlds)test("edge refresh: "+row.id,()=>{
 const own=call(row.world),run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify(row.world))});
 expect(run.exitCode).toBe(0);
 const model=JSON.parse(run.stdout.toString()) as {projects:string[];withheld:string[];carried:string[]};
 expect({projects:[...own.projects].sort(),withheld:[...own.withheld].sort(),carried:[...own.carried].sort()}).toEqual(model);
});

test("the worlds exercise a moved environment with withheld owners, and the quiet cases",()=>{
 const moved=call(fixture.worlds[0]!.world);
 expect(moved.withheld).toEqual(["withheld"]);expect(moved.projects).toEqual(["keep"]);expect(moved.carried).toEqual([]);
 expect(call(fixture.worlds[1]!.world)).toEqual({projects:[],withheld:[],carried:[]});
 expect(call(fixture.worlds[2]!.world)).toEqual({projects:[],withheld:[],carried:[]});
 const known=call(fixture.worlds[6]!.world);expect(known.projects).toEqual(["keep"]);expect(known.carried).toEqual(["kept-too"]);expect(known.withheld).toEqual(["withheld"]);
 const none=call(fixture.worlds[7]!.world);expect(none.projects).toEqual([]);expect(none.carried).toEqual(["keep"]);expect(none.withheld).toEqual(["withheld"]);
});
