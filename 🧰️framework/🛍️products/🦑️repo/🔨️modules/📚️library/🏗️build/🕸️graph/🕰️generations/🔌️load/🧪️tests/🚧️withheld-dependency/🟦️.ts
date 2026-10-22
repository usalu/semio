/** 🚧️ Contract: a test project whose native sources name a project the graph snapshot does not hold is withheld alone, with a bilingual diagnostic naming its declaring file, the missing projects and the fix; the graph is still built, every other project stays, and the next run that holds the project brings the test project back. The model is the fixture, derived independently in Python by `🐍️oracle.py`. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import generationSchema from "../../../🧬️schema/🔣️.json";
import graphSchema from "../../../../../🧬️schema/🔣️.json";
import leanSchema from "../../../../../🧬️schema/🪶️lean.json";
import {danglingTestDependenciesV1} from "../../../../../../../🧪️test/🟨️.mjs";
import {announceWithheldProjectsV1,describeUnavailableProjectV1,loadGenerationGraphV1,withholdDanglingTestProjectsV1} from "../../🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";

const source=readFileSync(new URL("../../../../🟦️.ts",import.meta.url),"utf8");
const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️oracle.py"),fixturePath=join(here,"🔣️.json"),schemaPath=join(here,"..","..","..","🧬️schema","🔣️.json"),sourcePath=fileURLToPath(new URL("../../../../🟦️.ts",import.meta.url));
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-withheld-"));
const clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value));
const graphRow=new Ajv2020({strict:false}).addSchema(graphSchema,"graph").compile({$ref:"graph#/$defs/UnavailableProject"});
const leanRows=new Ajv2020({strict:false}).compile({...leanSchema,$id:"lean"});
const present=(row:Readonly<Record<string,string|undefined>>):Record<string,string>=>Object.fromEntries(Object.entries(row).flatMap(([key,value])=>value===undefined?[]:[[key,value]]));
const run=(project:Record<string,unknown>,rawConfigs:Readonly<Record<string,string|undefined>>,rawDigests:Readonly<Record<string,string|undefined>>)=>{
 const configs=present(rawConfigs),digests=present(rawDigests),projects=clone(project) as Record<string,any>,dangling=danglingTestDependenciesV1(projects as never),unavailable=new Map<string,any>();
 const withheld=withholdDanglingTestProjectsV1({projects,configs,unavailable,dangling,digest:file=>digests[file]??fixture.emptyDigest});
 return{dangling,withheld,remaining:Object.keys(projects).sort(),unavailable:[...unavailable.values()]};
};

test("the fixture is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);});

for(const row of fixture.cases)test("withholding: "+row.id,()=>{
 const actual=run(row.projects,row.configs,row.digests);
 expect(actual.dangling).toEqual(row.expected.dangling);
 expect(actual.withheld).toEqual(row.expected.withheld);
 expect(actual.remaining).toEqual(row.expected.remaining);
 expect(actual.unavailable).toEqual(row.expected.unavailable);
 for(const[index,unavailable]of actual.unavailable.entries()){
  expect(graphRow(unavailable),JSON.stringify(graphRow.errors)).toBe(true);
  expect(describeUnavailableProjectV1(unavailable,"en")).toBe(row.expected.text[index]!.en);
  expect(describeUnavailableProjectV1(unavailable,"de")).toBe(row.expected.text[index]!.de);
 }
});

for(const law of fixture.laws)test("assembly order: "+law.name,()=>{
 const start=source.indexOf(law.start),end=source.indexOf(law.end,start+law.start.length);
 expect(start,"start").toBeGreaterThan(-1);expect(source.indexOf(law.start,start+1),"start unique").toBe(-1);expect(end,"end").toBeGreaterThan(start);
 const text=source.slice(start,end),before=text.indexOf(law.before),after=text.indexOf(law.after,before+1);
 expect(before,"before").toBeGreaterThan(-1);expect(after,"after").toBeGreaterThan(before);
});

test("an independent Python model derives the same dangling projects, rows and words",()=>{
 expect(python,"python3 is part of the test toolchain").toBeDefined();
 const result=Bun.spawnSync([python!,"-I",oracle,fixturePath,schemaPath,sourcePath]);
 expect(result.exitCode,result.stderr.toString()).toBe(0);
 const model=JSON.parse(result.stdout.toString());
 for(const row of fixture.cases)expect(model[row.id],row.id).toEqual(row.expected);
 expect(model.laws).toEqual(Object.fromEntries(fixture.laws.map(row=>[row.name,true])));
});

test("the lean listing admits a withheld test project with its missing names",()=>{
 const row=fixture.cases.find(entry=>entry.id==="project-created-after-the-snapshot")!.expected.unavailable[0]!;
 expect(leanRows({version:1,lean:true,projects:{},dependencies:[],unavailable:[row]}),JSON.stringify(leanRows.errors)).toBe(true);
});

test("a withheld source that does not parse keeps its own words in both languages",()=>{
 const row={root:"a",project:"p",source:"a/📋️project.json",sha256:"ab".repeat(32),reason:"x"};
 expect(describeUnavailableProjectV1(row,"en")).toBe("graph: p is withheld: a/📋️project.json does not parse (sha256 "+"ab".repeat(6)+"); fix that file and run the command again");
 expect(describeUnavailableProjectV1(row,"de")).toBe("Graph: p wird zurückgehalten: a/📋️project.json lässt sich nicht lesen (sha256 "+"ab".repeat(6)+"); die Datei korrigieren und den Befehl erneut ausführen");
 expect(Object.keys(generationSchema.definitions.Messages.const.en)).toContain("withheldDependency");
});

test("a graph build completes with the withheld test project, says why in the language of the caller, and the next run brings the project back",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(5));
  const mock=createMockHost(dir,{language:"de"});
  const unavailable=new Map<string,any>();
  withholdDanglingTestProjectsV1({projects:{"test-p3":{}},configs:{"test-p3":"pkg/p3/tests/case/spec.feature"},unavailable,dangling:[{source:"test-p3",root:"pkg/p3/tests",missing:["ghost-crate"]}],digest:()=>fixture.emptyDigest});
  const row=[...unavailable.values()][0];
  let mode:"withheld"|"project"|"dropped"="withheld";
  const test={name:"test-p3",root:row.root,config:row.source,tag:"tests",nodeRows:[]};
  const host={...mock.host,async discoverPart(...args:Parameters<typeof mock.host.discoverPart>){
   const part=await mock.host.discoverPart(...args);
   const extra=mode==="withheld"?[{...test,unavailable:row}]:mode==="project"?[{...test,node:{name:"test-p3",root:row.root,targets:{test:{}}}}]:[];
   return{...part,owners:[...part.owners,...extra],replaceTags:["tests"]};
  }};
  const announced:string[]=[];
  const loud={...host,say:(key:never,fields:Record<string,string|number>)=>{announced.push(String(key)+":"+Object.values(fields).join("|"));host.say(key,fields);}};
  const cold=await loadGenerationGraphV1(loud as never) as any;
  expect(Object.keys(cold.projects).sort()).toEqual(["p0","p1","p2","p3","p4"]);
  expect(cold.unavailable).toEqual([row]);
  expect(announced.some(line=>line.startsWith("withheldDependency:test-p3|pkg/p3/tests/case/spec.feature|ghost-crate"))).toBe(true);
  expect(mock.said.some(entry=>entry.key==="withheldDependency"&&entry.text.startsWith("Graph: test-p3 wird zurückgehalten")&&entry.text.includes("ghost-crate"))).toBe(true);
  const warm=await loadGenerationGraphV1(loud as never) as any;
  expect(warm.unavailable).toEqual([row]);
  mode="project";
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=2;\n'}]);
  const repaired=await loadGenerationGraphV1(loud as never) as any;
  expect(Object.keys(repaired.projects).sort()).toEqual(["p0","p1","p2","p3","p4","test-p3"]);
  expect(repaired.unavailable).toBeUndefined();
  mode="withheld";
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=3;\n'}]);
  const withheldAgain=await loadGenerationGraphV1(loud as never) as any;
  expect(Object.keys(withheldAgain.projects).sort()).toEqual(["p0","p1","p2","p3","p4"]);
  expect(withheldAgain.unavailable).toEqual([row]);
  mode="dropped";
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=4;\n'}]);
  const dropped=await loadGenerationGraphV1(loud as never) as any;
  expect(Object.keys(dropped.projects).sort()).toEqual(["p0","p1","p2","p3","p4"]);
  expect(dropped.unavailable).toBeUndefined();
  const said:string[]=[];
  announceWithheldProjectsV1({say:(key,fields)=>{said.push(key+":"+String(fields.project));}},{owners:[{name:"a",root:"a"},{name:"b",root:"b",unavailable:{...row}},{name:"c",root:"c",unavailable:{root:"c",project:"c",source:"c/x",sha256:"0".repeat(64),reason:"x"}}]});
  expect(said).toEqual(["withheldDependency:test-p3","withheldSource:c"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},120000);
