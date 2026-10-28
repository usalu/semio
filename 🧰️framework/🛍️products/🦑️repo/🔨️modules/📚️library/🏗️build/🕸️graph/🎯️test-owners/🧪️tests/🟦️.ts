/** 🎯️ Contract: the selection of test owners to discover again equals the independent Python model on seeded random workspaces, an owner is always discovered with all of its features, and the owner by owner discovery answers what the discovery of all of them answers, with the facts of the shared state followed by the facts of the owner itself. */
import {expect,test} from "bun:test";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import {SEPARATE_TEST_OWNERS_V1,compareTestOwnersV1,discoverTestOwnersV1,selectTestFeaturesV1} from "../🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle","🐍️.py");
const groupsOf=(row:{features:string[];all:boolean;roots:string[];known:string[]})=>selectTestFeaturesV1(row.features,{all:row.all,roots:new Set(row.roots),known:new Set(row.known)}),select=(row:Parameters<typeof groupsOf>[0])=>[...groupsOf(row)].map(([owner,features])=>[owner,features]);

test("the fixture is closed under its schema",()=>{expect(new Ajv({strict:true}).validate(fixtureSchema,fixture)).toBe(true);});

for(const row of fixture.worlds)test("selection of "+row.name+" equals the model",()=>{
 expect(select(row)).toEqual(row.groups);
 for(const[owner,features]of groupsOf(row))expect(features).toEqual(row.features.filter(feature=>feature.startsWith(owner+"/")&&feature.slice(owner.length+1).split("/").length===2));
});

test("the fixture is reproduced by the Python model",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",oracle,"check",join(here,"🔣️.json")]);
 expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});

test("nothing is selected when nothing is stale, everything when everything is",()=>{
 const features=["a/🧪️tests/x/f.feature","b/🧪️tests/y/f.feature"];
 expect(select({features,all:false,roots:[],known:["a/🧪️tests","b/🧪️tests"]})).toEqual([]);
 expect(select({features,all:true,roots:[],known:[]}).map(([owner])=>owner)).toEqual(["a/🧪️tests","b/🧪️tests"]);
 expect(select({features,all:false,roots:["a/🧪️tests"],known:["a/🧪️tests","b/🧪️tests"]}).map(([owner])=>owner)).toEqual(["a/🧪️tests"]);
 expect(select({features,all:false,roots:["a/🧪️tests/x"],known:["a/🧪️tests","b/🧪️tests"]}).map(([owner])=>owner)).toEqual(["a/🧪️tests"]);
 expect(select({features,all:false,roots:["b"],known:["a/🧪️tests"]}).map(([owner])=>owner)).toEqual(["b/🧪️tests"]);
 expect(select({features,all:false,roots:["a"],known:["a/🧪️tests"]})).toEqual([]);
});

test("owners are discovered in the order of their bytes",()=>{
 expect(["b","a","\u{1F600}","～"].sort(compareTestOwnersV1)).toEqual(["a","b","～","\u{1F600}"]);
});

const fake=(log:string[])=>({
 discoverTestSharedV1:async()=>{log.push("shared");return{shared:true};},
 discoverTestBuildProjectsV1:async(features:readonly string[],_options:object,context:{testShared?:unknown})=>{
  log.push("owner:"+features[0]);expect(context.testShared).toEqual({shared:true});
  const owner=features[0]!.split("/").slice(0,-2).join("/");
  return[[features[0]!,{projects:{["test-"+owner]:{root:owner,cases:features.length}}}]] as [string,{projects:Record<string,unknown>}][];
 },
});
const capture=(log:string[])=>async(compute:()=>Promise<unknown>)=>{const value=await compute();return{value,observations:[log.at(-1)!]};};

test("owners are discovered one by one after one shared computation, each project with the shared facts then its own",async()=>{
 const log:string[]=[],groups=new Map([["m/🧪️tests",["m/🧪️tests/a/f.feature","m/🧪️tests/b/f.feature"]],["a/🧪️tests",["a/🧪️tests/z/f.feature"]]]);
 const result=await discoverTestOwnersV1({tests:fake(log),capture:capture(log),context:{workspaceRoot:"/w"},groups});
 expect(log).toEqual(["shared","owner:a/🧪️tests/z/f.feature","owner:m/🧪️tests/a/f.feature"]);
 expect(result.contributions.map(([file])=>file)).toEqual(["a/🧪️tests/z/f.feature","m/🧪️tests/a/f.feature"]);
 expect(result.rows.get("test-a/🧪️tests")).toEqual(["shared","owner:a/🧪️tests/z/f.feature"]);
 expect(result.rows.get("test-m/🧪️tests")).toEqual(["shared","owner:m/🧪️tests/a/f.feature"]);
});

test("no owner means no shared computation",async()=>{
 const log:string[]=[],result=await discoverTestOwnersV1({tests:fake(log),capture:capture(log),context:{},groups:new Map()});
 expect(log).toEqual([]);expect(result.contributions).toEqual([]);expect(result.rows.size).toBe(0);
});

test("a selection beyond the separate limit is discovered in one call whose facts every project carries",async()=>{
 const log:string[]=[],many=new Map(Array.from({length:SEPARATE_TEST_OWNERS_V1+1},(_,at)=>["m"+String(at).padStart(3,"0")+"/🧪️tests",["m"+String(at).padStart(3,"0")+"/🧪️tests/a/f.feature"]] as [string,string[]]));
 const tests={
  discoverTestSharedV1:async()=>{log.push("shared");return{shared:true};},
  discoverTestBuildProjectsV1:async(features:readonly string[])=>{log.push("call:"+features.length);return features.map(feature=>[feature,{projects:{["test-"+feature]:{}}}]) as [string,{projects:Record<string,unknown>}][];},
 };
 const result=await discoverTestOwnersV1({tests,capture:async(compute:()=>Promise<unknown>)=>({value:await compute(),observations:["call"+log.length]}),context:{},groups:many});
 expect(log).toEqual(["shared","call:"+many.size]);expect(result.contributions.length).toBe(many.size);
 expect(result.rows.get("test-m000/🧪️tests/a/f.feature")).toEqual(["call1","call2"]);
 const few=await discoverTestOwnersV1({tests,capture:async(compute:()=>Promise<unknown>)=>({value:await compute(),observations:["x"]}),context:{},groups:many,separate:many.size});
 expect(few.contributions.length).toBe(many.size);
});
