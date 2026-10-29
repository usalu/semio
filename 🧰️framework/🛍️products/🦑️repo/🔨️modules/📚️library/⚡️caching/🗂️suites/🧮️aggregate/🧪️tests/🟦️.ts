import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

test("the current canonical project orders every closed route through a no-op aggregate",()=>{
 const project=JSON.parse(readFileSync(resolve(import.meta.dirname,"../../../📋️project.json"),"utf8"));
 expect(project.targets.test.executor).toBe("repo:noop");
 expect(project.targets["test-contracts"].options.command).toBe("repo ./📜️script.ts test contracts");
});

test("the neutral route join agrees with independent SQLite",async()=>{
 expect(new Ajv({strict:true}).compile({$ref:schema.$id+"#/$defs/Fixture",...schema})(fixture)).toBe(true);
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE routes(id TEXT,entry TEXT,args TEXT);CREATE TABLE candidates(target TEXT,entry TEXT,args TEXT)");
  for(const route of fixture.routes)for(const command of route.commands)database.query("INSERT INTO routes VALUES(?,?,?)").run(route.id,command.entry,JSON.stringify(command.arguments));
  for(const row of fixture.candidates)database.query("INSERT INTO candidates VALUES(?,?,?)").run(row.target,row.entry,JSON.stringify(row.arguments));
  const oracle=(database.query("SELECT DISTINCT target FROM candidates JOIN routes USING(entry,args) ORDER BY target").all() as {target:string}[]).map(row=>row.target);
  const {resolveCanonicalTestTargetsV1}=await import("../🟦️.ts");
  expect(resolveCanonicalTestTargetsV1(fixture.project,fixture.routes)).toEqual([...oracle,"test-contracts"].sort());
  expect(resolveCanonicalTestTargetsV1(fixture.project,fixture.routes)).toEqual(fixture.expected);
 }finally{database.close();}
});

test("route additions and legitimate target renames are derived while omissions and duplicate commands refuse",async()=>{
 const {resolveCanonicalTestTargetsV1}=await import("../🟦️.ts"),project=structuredClone(fixture.project) as any;
 project.targets["renamed-alpha"]=project.targets["test-alpha-exception"];delete project.targets["test-alpha-exception"];
 expect(resolveCanonicalTestTargetsV1(project,fixture.routes)).toEqual(["renamed-alpha","test-contracts","test-proof"]);
 project.targets["test-extra"]={executor:"repo:script",options:{command:"repo ./📜️script.ts test extra"}};
 expect(resolveCanonicalTestTargetsV1(project,[...fixture.routes,{id:"extra",commands:[{entry:"cache/📜️script.ts",arguments:["test","extra"]}]}])).toContain("test-extra");
 delete project.targets["renamed-alpha"];expect(()=>resolveCanonicalTestTargetsV1(project,fixture.routes)).toThrow();
 project.targets.alpha=fixture.project.targets["test-alpha-exception"];project.targets.duplicate=project.targets.alpha;
 expect(()=>resolveCanonicalTestTargetsV1(project,fixture.routes)).toThrow();
 expect(()=>resolveCanonicalTestTargetsV1(fixture.project,[...fixture.routes,fixture.routes[0]!])).toThrow();
 const unknown=structuredClone(fixture.project) as any;unknown.targets.unknown={executor:"repo:script",options:{command:"repo ./📜️script.ts test not-declared"}};
 expect(()=>resolveCanonicalTestTargetsV1(unknown,fixture.routes)).toThrow();
});

test("unknown cache test arguments refuse before a proof is selected",async()=>{
 const {selectCanonicalCacheTestV1}=await import("../🟦️.ts");
 expect(selectCanonicalCacheTestV1(["contracts"])).toBe("contracts");
 for(const args of [[],["not-declared"],["contracts","typo"]])expect(()=>selectCanonicalCacheTestV1(args)).toThrow();
});

test("aggregate dependency cycles and forwarded aggregate arguments refuse in the actual task planner",async()=>{
 const {resolveCanonicalTestTargetsV1}=await import("../🟦️.ts"),{neutralBuildTargetV1}=await import("../../../../🏗️build/🕸️graph/🟦️.ts"),{planBuildTasksV1}=await import("../../../../🏗️build/🧮️task-model/🟦️.ts"),project=structuredClone(fixture.project) as any;
 project.targets.test={executor:"repo:noop",dependsOn:resolveCanonicalTestTargetsV1(project,fixture.routes),metadata:{semio:{arguments:{maximum:0}}}};
 const graph:any={version:1,projects:{repo:{name:"repo",root:"cache",targets:Object.fromEntries(Object.entries(project.targets).map(([name,target])=>[name,neutralBuildTargetV1(target as any)]))},workspace:{name:"workspace",root:".",targets:{test:{execution:{kind:"noop"},dependsOn:["repo:test"]}}}},dependencies:[]};
 expect(()=>planBuildTasksV1(graph,{tasks:["repo:test"],args:["--typo"]})).toThrow();
 graph.projects.repo.targets["test-alpha-exception"].dependsOn=["workspace:test"];
 expect(()=>planBuildTasksV1(graph,{tasks:["repo:test"]})).toThrow();
 graph.projects.repo.targets["test-alpha-exception"].dependsOn=[{target:"test",projects:["repo"]}];
 expect(()=>planBuildTasksV1(graph,{tasks:["repo:test"]})).toThrow();
 graph.projects.repo.targets["test-alpha-exception"].dependsOn=[{target:"test*",projects:["repo"]}];
 expect(()=>planBuildTasksV1(graph,{tasks:["repo:test"]})).toThrow();
});

test("a stale or self-referential checked declaration refuses before project publication",async()=>{
 const {canonicalTestAggregatePolicyV1,requireCanonicalTestAggregateV1}=await import("../🟦️.ts"),project=JSON.parse(readFileSync(resolve(import.meta.dirname,"../../../📋️project.json"),"utf8"));
 requireCanonicalTestAggregateV1(project,canonicalTestAggregatePolicyV1.root);
 project.targets.test.dependsOn.push("test");
 expect(()=>requireCanonicalTestAggregateV1(project,canonicalTestAggregatePolicyV1.root)).toThrow();
});
