/** 🚶️ Contract: one pass of the module closure graph over real walk questions reaches exactly the files the command walk reaches (itself held equal to a Python reachability), an import edit moves exactly the questions whose walk reached the edited module, a body edit moves none, a declared tool stop keeps an edit inside the tool out of the questions cut there, a pass with a journal dirty set rebuilds only the touched module and equals a pass from scratch, and the stored graph answers the same digests. */
import{expect,setDefaultTimeout,test}from"bun:test";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join}from"node:path";
import{loadClosureGraphV1,saveClosureGraphV1}from"../../🟨️.mjs";

setDefaultTimeout(180000);
const TREE:Record<string,string>={
 "entry.ts":'import "./a.ts";import "./b.ts";import fs from "node:fs";export {};\n',
 "a.ts":'import {b} from "./b.ts";import "./cycle1.ts";export const a=1;\n',
 "b.ts":'import "./leaf.ts";import data from "./data.json";export const b=2;\n',
 "cycle1.ts":'import "./cycle2.ts";export const c1=1;\n',
 "cycle2.ts":'import "./cycle1.ts";export const c2=1;\n',
 "leaf.ts":'export const leaf=1;\n',
 "later.ts":'export const later=1;\n',
 "data.json":'{"k":1}\n',
 "cmd-a.ts":'export class A{run(){return 0}}\n',
 "cmd-b.ts":'import "./leaf.ts";export class B{run(){return 0}}\n',
 "router.ts":'export class Router{#c=new Map();register(n,C){this.#c.set(n,()=>C);return this}async run(argv){const load=this.#c.get(argv[0]);if(!load)return 1;return new (await load())().run(argv.slice(1));}}\nimport {A} from "./cmd-a.ts";import {B} from "./cmd-b.ts";\nnew Router().register("a",A).register("b",B);\n',
};
const write=(root:string,files:Record<string,string>)=>{for(const[name,text]of Object.entries(files)){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),text);}};

test("the graph pass equals the walk, moves exactly the reaching questions, cuts at tools, is incremental through a dirty set and round-trips the store",async()=>{
 const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../../🟨️.mjs") as any,root=realpathSync(mkdtempSync(join(tmpdir(),"closure-pass-"))),stores=realpathSync(mkdtempSync(join(tmpdir(),"closure-pass-store-")));
 try{
  write(root,TREE);writeFileSync(join(root,"package.json"),'{"name":"closure-pass-fixture"}\n');
  const at=(name:string)=>join(root,name),questions=[
   {roots:[at("entry.ts")]},{roots:[at("a.ts")]},{roots:[at("cycle2.ts")]},{roots:[at("leaf.ts")]},
   {roots:[at("router.ts")],selectors:[[at("router.ts"),["a"]]]},
   {roots:[at("entry.ts")],stops:[[at("b.ts"),"tool-b"]]},
  ];
  const fresh=()=>{const cache=inference.createScriptInputCache({workspaceRoot:root});cache.pass={};cache.generatedOutput=()=>undefined;return cache;};
  const cache=fresh(),first=await scripts.closureGraphPassV1(cache,root,questions);
  expect(first.digests.every((digest:string|undefined)=>typeof digest==="string")).toBe(true);
  const key=(name:string,words:string[]|null=null)=>JSON.stringify(["{workspaceRoot}/"+name,1,0,"",words]);
  const walk=async(entries:string[],selectors=new Map<string,string[]>())=>(await scripts.relativeScriptInputs(entries.map(at),root,fresh(),false,[],selectors.size?new Map([...selectors].map(([name,words])=>[at(name),words])):new Map())) as string[];
  for(const[index,question]of questions.entries()){
   if(question.stops)continue;
   const names=question.roots.map(path=>path.slice(root.length+1)),selected=new Map((question.selectors??[]).map(([path,words])=>[(path as string).slice(root.length+1),words as string[]]));
   const redirect=new Map([...selected].flatMap(([name,words])=>[[key(name),key(name,words)],[JSON.stringify(["{workspaceRoot}/"+name,0,0,"",null]),JSON.stringify(["{workspaceRoot}/"+name,0,0,"",words])]]));
   const reached=first.graph.reach(names.map(name=>key(name,selected.get(name)??null)),{redirect}).names;
   expect([...new Set(reached)].sort(),"question "+index).toEqual(await walk(names,selected));
  }
  const cut=first.graph.reach([key("entry.ts")],{stops:new Map([[key("b.ts"),"tool-b"]])});
  expect(cut.tools).toEqual(["tool-b"]);expect(cut.names).not.toContain("{workspaceRoot}/b.ts");expect(cut.names).not.toContain("{workspaceRoot}/data.json");
  const before=first.digests,reachedBefore=await Promise.all(questions.map(async question=>question.stops?undefined:new Set(await walk(question.roots.map(path=>path.slice(root.length+1)),new Map((question.selectors??[]).map(([path,words])=>[(path as string).slice(root.length+1),words as string[]]))))));
  writeFileSync(at("leaf.ts"),"export const leaf=12345;\n");cache.pass={};
  const body=await scripts.closureGraphPassV1(cache,root,questions);expect(body.digests).toEqual(before);
  writeFileSync(at("cycle2.ts"),'import "./cycle1.ts";import "./later.ts";export const c2=1;\n');
  cache.pass={dirty:{paths:new Set(["cycle2.ts"]),directories:new Set(),subtrees:new Set()}};
  const edited=await scripts.closureGraphPassV1(cache,root,questions);
  expect(edited.built).toBeLessThanOrEqual(2);
  const moved=questions.map((_,index)=>edited.digests[index]!==before[index]),expected=reachedBefore.map((names,index)=>names===undefined?true:names.has("{workspaceRoot}/cycle2.ts"));
  expect(moved).toEqual(expected);
  const scratch=await scripts.closureGraphPassV1(fresh(),root,questions,{graph:undefined});
  expect(scratch.digests).toEqual(edited.digests);
  writeFileSync(at("b.ts"),'import "./leaf.ts";import "./later.ts";import data from "./data.json";export const b=2;\n');cache.pass={};
  const tool=await scripts.closureGraphPassV1(cache,root,questions);
  expect(tool.digests[5]).toBe(edited.digests[5]);expect(tool.digests[0]).not.toBe(edited.digests[0]);
  const store=join(stores,"graph");await saveClosureGraphV1(tool.graph,store);
  const loaded=(await loadClosureGraphV1(store))!;
  expect(loaded.closureDigest([key("entry.ts")])).toBe(tool.digests[0]);
  expect(loaded.closureDigest([key("entry.ts")],{stops:new Map([[key("b.ts"),"tool-b"]])})).toBe(tool.digests[5]);
  await Bun.sleep(2600);cache.pass={};
  const witnessed=await scripts.closureGraphPassV1(cache,root,questions,{witness:true}),adoptStore=join(stores,"adopt");await saveClosureGraphV1(witnessed.graph,adoptStore);
  const adopted=(await loadClosureGraphV1(adoptStore))!,trusted=await scripts.closureGraphWitnessedV1(adopted);
  expect(trusted.size,JSON.stringify([...adopted.records()].filter(record=>!trusted.has(record.key)).map(record=>[record.name,record.extra===null]))).toBe(adopted.stats().nodes);
  const adopter=fresh(),adopt=await scripts.closureGraphPassV1(adopter,root,questions,{graph:adopted,trusted});
  expect(adopt.digests).toEqual(tool.digests);expect(adopt.built).toBe(0);expect(adopter.walkNodes?.size??0).toBe(0);
  writeFileSync(at("cycle1.ts"),'import "./cycle2.ts";import "./leaf.ts";export const c1=1;\n');
  const again=(await loadClosureGraphV1(adoptStore))!,stillTrusted=await scripts.closureGraphWitnessedV1(again);
  expect(stillTrusted.size).toBeLessThan(again.stats().nodes);
  const partly=await scripts.closureGraphPassV1(fresh(),root,questions,{graph:again,trusted:stillTrusted}),scratchMoved=await scripts.closureGraphPassV1(fresh(),root,questions,{graph:undefined});
  expect(partly.digests).toEqual(scratchMoved.digests);expect(partly.built).toBeGreaterThan(0);expect(partly.built).toBeLessThan(scratchMoved.built);
  const pruned=await scripts.closureGraphPassV1(cache,root,[questions[3]!],{prune:true});
  expect(pruned.pruned).toBeGreaterThan(0);expect(pruned.graph.stats().nodes).toBe(1);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(stores,{recursive:true,force:true});}
});
