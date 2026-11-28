/** 🔗️ Contract: when the environment of a generation moved, every untagged owner of a real project root that is not inferred again gets its edges again, withheld owners included (their fragment names the environment it was published for, so a generation with one stale owner is refused as incomplete); nothing is refreshed on a cold start or while the environment stays, except the owners named stale (their stored edges are of another environment), which are refreshed without inference, never carried; the heal plan after a compute that left stale owners gives the untagged ones their edges first and infers the rest in a second round; when the consumers of the edit are known, a kept project that is no consumer keeps its edges (carried). The independent oracle is a Python model of the same rule over the same worlds. */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{edgeRefreshOwnersV1}from"../../🟦️.ts";
import{edgeHealPlanV1,requireEdgeEnvironmentV1}from"../../🕰️generations/🪡️edge-heal/🟨️.mjs";
import fixture from"./🔣️.json";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️oracle/🐍️.py");
type World=(typeof fixture)["worlds"][number]["world"];
const heals=fixture.worlds.filter(row=>"heal" in row.world),refreshes=fixture.worlds.filter(row=>!("heal" in row.world));
const call=(w:any)=>edgeRefreshOwnersV1({cold:w.cold,environment:w.environment,previousEnvironment:w.previousEnvironment,roots:new Set<string>(w.roots),stubs:w.stubs as never,inferred:new Set<string>(w.inferred),...("consumers" in w?{consumers:new Set<string>(w.consumers as string[])}:{}),...("stale" in w?{stale:new Set<string>(w.stale as string[])}:{})});
const model=(world:World)=>{const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify(world))});expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString());};

for(const row of refreshes)test("edge refresh: "+row.id,()=>{
 const own=call(row.world);
 expect({projects:[...own.projects].sort(),withheld:[...own.withheld].sort(),carried:[...own.carried].sort()}).toEqual(model(row.world));
});

for(const row of heals)test("edge heal: "+row.id,()=>{
 const{heal}=row.world as unknown as {heal:{owners:Record<string,{edgesEnv:string;tag?:string}>;environment:string;cold:boolean;healed:number;previous?:number;tried?:string[];inferred?:string[];pending?:string[]}},own=edgeHealPlanV1(heal);
 expect(own).toEqual(model(row.world));
});

test("the worlds exercise a moved environment with withheld owners, and the quiet cases",()=>{
 const moved=call(fixture.worlds[0]!.world);
 expect([...moved.withheld].sort()).toEqual(["dirty-withheld","withheld"]);expect([...moved.projects].sort()).toEqual(["dirty-one","keep"]);expect(moved.carried).toEqual([]);
 expect(call(fixture.worlds[1]!.world)).toEqual({projects:[],withheld:[],carried:[]});
 expect(call(fixture.worlds[2]!.world)).toEqual({projects:[],withheld:[],carried:[]});
 const known=call(fixture.worlds[6]!.world);expect(known.projects).toEqual(["keep"]);expect([...known.carried].sort()).toEqual(["dirty-one","kept-too"]);expect([...known.withheld].sort()).toEqual(["dirty-withheld","withheld"]);
 const none=call(fixture.worlds[7]!.world);expect(none.projects).toEqual([]);expect([...none.carried].sort()).toEqual(["dirty-one","keep"]);expect([...none.withheld].sort()).toEqual(["dirty-withheld","withheld"]);
});

test("stale owners are refreshed while the environment stays, and the heal rounds follow the contract",()=>{
 const stayed=call(fixture.worlds[9]!.world);expect(stayed.projects).toEqual(["keep"]);expect(stayed.withheld).toEqual(["withheld"]);expect(stayed.carried).toEqual([]);
 const known=call(fixture.worlds[10]!.world);expect(known.projects.sort()).toEqual(["dirty-one","keep"]);expect(known.carried).toEqual([]);
 expect(call(fixture.worlds[12]!.world)).toEqual({projects:[],withheld:[],carried:[]});
 const scanned=call(fixture.worlds[13]!.world);expect(scanned.projects).toEqual(["dirty-one"]);expect(scanned.withheld).toEqual(["dirty-withheld"]);expect(scanned.carried).toEqual([]);
 const first=edgeHealPlanV1((fixture.worlds[14]!.world as any).heal);expect(first).toEqual({edges:["b","c"],infer:["t"]});
 expect(edgeHealPlanV1((fixture.worlds[15]!.world as any).heal)).toEqual({edges:[],infer:["b","c","t"]});
});

test("an owner is given its edges, then inferred once, then nothing is left; the limit only bounds a runaway",()=>{
 const heal=(i:number)=>(fixture.worlds[i]!.world as any).heal;
 expect(edgeHealPlanV1(heal(19))).toEqual({edges:[],infer:["c","t"]});
 expect(edgeHealPlanV1(heal(20))).toEqual({edges:[],infer:[]});
 expect(edgeHealPlanV1(heal(21))).toEqual({edges:["c"],infer:["b"]});
 expect(edgeHealPlanV1(heal(16))).toEqual({edges:[],infer:[]});
 const env=(name:string)=>new Map([[name,"e2"]]),old=new Map([["b","e1"]]);
 expect(edgeHealPlanV1({...heal(19),tried:env("b"),inferred:new Map()})).toEqual({edges:["c"],infer:["b","t"]});
 expect(edgeHealPlanV1({...heal(19),tried:old,inferred:old})).toEqual({edges:["b","c"],infer:["t"]});
});

test("a stale owner is given its edges before it is ever inferred",()=>{
 expect(edgeHealPlanV1((fixture.worlds[22]!.world as any).heal)).toEqual({edges:["c"],infer:["b","t"]});
});

test("an owner that waits for inference in a later wave is neither healed nor refused (read 3 and 5, @leutwiler/realparts-of-powers-z-n)",()=>{
 const heal=(i:number)=>(fixture.worlds[i]!.world as any).heal;
 expect(edgeHealPlanV1(heal(23))).toEqual({edges:[],infer:[]});
 expect(edgeHealPlanV1(heal(24))).toEqual({edges:[],infer:[]});
 const refuse=(pending:string[])=>requireEdgeEnvironmentV1({owners:heal(24).owners,environment:"e2",fail:(message:string)=>new Error(message),wave:[],returned:new Set<string>(),dropped:new Set<string>(),cold:false,repair:true,healed:4,resumed:false,before:{},previous:"e2",pending});
 expect(()=>refuse(["b"])).toThrow("Every owner needs edges of the published environment: c");
 expect(()=>refuse(["b","c"])).not.toThrow();
});
