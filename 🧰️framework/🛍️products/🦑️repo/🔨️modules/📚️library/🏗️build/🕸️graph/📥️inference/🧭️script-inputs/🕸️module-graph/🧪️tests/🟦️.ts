/** 🕸️ Contract: the command walk over module graph nodes reaches exactly the files an independent Python reachability over the same tree reaches, through cycles, builtin and bare imports, resources, router selectors and unresolved optional imports, and follows edits of the tree between two passes of one cache: a body edit changes nothing, an added import adds the files behind it; walks asked at the same time share the nodes of the module closure graph. The closure digests themselves are held to an independent oracle in `../../🌳️closure-graph/🧪️tests`. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";

setDefaultTimeout(120000);

const oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),python=process.env.SEMIO_PYTHON??"python3";
const DOT=".";
const ROUTER=`export class Router{#c=new Map();register(n,C){this.#c.set(n,()=>C);return this}registerLazy(n,l){this.#c.set(n,l);return this}async run(argv){const load=this.#c.get(argv[0]);if(!load)return 1;return new (await load())().run(argv.slice(1));}}
import {A} from "${DOT}/cmd-a.ts";import {B} from "${DOT}/cmd-b.ts";
new Router().register("a",A).register("b",B);
`;
const TREE:Record<string,string>={
 "entry.ts":'import "./a.ts";import "./b.ts";import fs from "node:fs";import "left-pad";import "./optional-missing.ts";export {};\n',
 "a.ts":'import {b} from "./b.ts";import "./cycle1.ts";export const a=1;\n',
 "b.ts":'import "./leaf.ts";import data from "./data.json";export const b=2;\n',
 "cycle1.ts":'import "./cycle2.ts";export const c1=1;\n',
 "cycle2.ts":'import "./cycle1.ts";import "./leaf.ts";export const c2=1;\n',
 "leaf.ts":'export const leaf=1;\n',
 "later.ts":'export const later=1;\n',
 "uses-gen.ts":'import "@gen/out";export const g=1;\n',
 "src-gen.ts":'import "./leaf.ts";import "./later.ts";export const sg=1;\n',
 "data.json":'{"k":1}\n',
 "cmd-a.ts":'export class A{run(){return 0}}\n',
 "cmd-b.ts":'import "./leaf.ts";export class B{run(){return 0}}\n',
 "router.ts":ROUTER,
};
const write=(root:string,files:Record<string,string>)=>{for(const[name,text]of Object.entries(files)){mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),text);}};
const GENERATED={"@gen/out":["src-gen.ts"]};
const expected=(root:string,entries:string[],kept:Record<string,string[]>={})=>JSON.parse(spawnSync(python,["-I",oracle],{input:JSON.stringify({root,entries,kept,generated:GENERATED}),encoding:"utf8"}).stdout) as string[];

test("the walk equals the Python reachability on a tree with cycles, builtins, resources and selectors, and follows edits between passes of one cache",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../🟨️.mjs") as any,root=realpathSync(mkdtempSync(join(tmpdir(),"walk-graph-")));
 try{
  write(root,TREE);writeFileSync(join(root,"package.json"),'{"name":"walk-fixture"}\n');
  const cache=inference.createScriptInputCache({workspaceRoot:root});
  const names=async(entries:string[],selectors=new Map<string,string[]>())=>(await scripts.relativeScriptInputs(entries.map(entry=>join(root,entry)),root,cache,false,[],selectors)).map((path:string)=>path.replace("{workspaceRoot}/","")).filter((path:string)=>path!=="package.json");
  cache.pass={};cache.generatedOutput=(specifier:string)=>specifier==="@gen/out"?{target:"p:gen",output:"gen/out.ts",sources:["src-gen.ts"]}:undefined;
  expect(await names(["entry.ts"])).toEqual(expected(root,["entry.ts"]));
  const generated=await names(["uses-gen.ts"]);expect(generated).toContain("src-gen.ts");expect(generated).toContain("later.ts");expect(generated).toEqual(expected(root,["uses-gen.ts"]));
  expect(await names(["a.ts","b.ts"])).toEqual(expected(root,["a.ts","b.ts"]));
  expect(await names(["cycle2.ts"])).toEqual(expected(root,["cycle2.ts"]));
  const selected=await names(["router.ts"],new Map([[join(root,"router.ts"),["a"]]]));
  expect(selected).toContain("cmd-a.ts");expect(selected).not.toContain("cmd-b.ts");expect(selected).not.toContain("leaf.ts");
  expect(selected).toEqual(expected(root,["router.ts"],{"router.ts":["./cmd-a.ts"]}).filter(path=>path!=="cmd-b.ts"&&path!=="leaf.ts"));
  const before=await names(["entry.ts"]);
  cache.pass={};writeFileSync(join(root,"leaf.ts"),"export const leaf=12345; // a body edit that adds no import\n");
  expect(await names(["entry.ts"])).toEqual(before);
  cache.pass={};writeFileSync(join(root,"cycle2.ts"),'import "./cycle1.ts";import "./leaf.ts";import "./later.ts";export const c2=1;\n');
  const after=await names(["entry.ts"]);
  expect(after).toContain("later.ts");expect(after).toEqual(expected(root,["entry.ts"]));
  cache.pass={};writeFileSync(join(root,"cycle2.ts"),'import "./cycle1.ts";import "./leaf.ts";export const c2=1;\n');
  expect(await names(["entry.ts"])).toEqual(before);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("two walks asked at the same time share the graph nodes they both need, however their builds interleave, and answer what one walk after the other answers",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../🟨️.mjs") as any,root=realpathSync(mkdtempSync(join(tmpdir(),"walk-concurrent-")));
 try{
  write(root,TREE);writeFileSync(join(root,"package.json"),'{"name":"walk-fixture"}\n');
  const entries=["entry.ts","a.ts","b.ts","cycle1.ts","uses-gen.ts"],fresh=()=>{const cache=inference.createScriptInputCache({workspaceRoot:root});cache.pass={};cache.generatedOutput=(specifier:string)=>specifier==="@gen/out"?{target:"p:gen",output:"gen/out.ts",sources:["src-gen.ts"]}:undefined;return cache;};
  const together=fresh(),answers=await Promise.all(entries.map(entry=>scripts.closureGraphAnswerV1(together,root,{roots:[join(root,entry)]})));
  const alone=fresh(),sequential=[];for(const entry of entries)sequential.push(await scripts.closureGraphAnswerV1(alone,root,{roots:[join(root,entry)]}));
  expect(answers).toEqual(sequential);
  expect(answers.every((answer:any)=>/^[a-f0-9]{64}$/u.test(answer.digest)&&/^[a-f0-9]{64}$/u.test(answer.answer))).toBe(true);
  expect(together.walkNodes.size,"each node is built once although the walks interleave").toBeLessThanOrEqual(Object.keys(TREE).length+2);expect(alone.closureGraph,"the caches of one workspace and designation share the module graph of the process").toBe(together.closureGraph);
 }finally{rmSync(root,{recursive:true,force:true});}
});
