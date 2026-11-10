/** 🕸️ Contract: the command walk over module graph nodes reaches exactly the files an independent Python reachability over the same tree reaches, through cycles, builtin and bare imports, resources, router selectors and unresolved optional imports, and follows edits of the tree between two passes of one cache: a body edit changes nothing, an added import adds the files behind it. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";

setDefaultTimeout(120000);

const oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),python=process.env.SEMIO_PYTHON??"python3";
const ROUTER=`export class Router{#c=new Map();register(n,C){this.#c.set(n,()=>C);return this}registerLazy(n,l){this.#c.set(n,l);return this}async run(argv){const load=this.#c.get(argv[0]);if(!load)return 1;return new (await load())().run(argv.slice(1));}}
import {A} from "./cmd-a.ts";import {B} from "./cmd-b.ts";
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

test("a closure computed from per-node reach sets equals the walk over the nodes: the same answer and the same support rows, through cycles, builtin imports, resources and router selectors, and after edits between passes",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../🟨️.mjs") as any,root=realpathSync(mkdtempSync(join(tmpdir(),"walk-reach-")));
 try{
  write(root,TREE);writeFileSync(join(root,"package.json"),'{"name":"walk-fixture"}\n');
  const cache=inference.createScriptInputCache({workspaceRoot:root}),question=(entries:string[],selectors:[string,string[]][]=[],resources:string[]=[])=>({version:1,row:{kind:"derived",path:JSON.stringify({walk:1,workspaceRoot:root,roots:entries.map(entry=>join(root,entry)),requiredSources:false,resources:resources.map(entry=>join(root,entry)),selectors:selectors.map(([path,words])=>[join(root,path),words]),provider:cache.providerDesignation??null})}});
  const both=async(entries:string[],selectors:[string,string[]][]=[],resources:string[]=[])=>{
   const asked=question(entries,selectors,resources).row;
   cache.pass={};const plain=await scripts.deriveClosureFactV1(asked,cache,cache.pass);
   const pass={transient:true};const fast=await scripts.deriveClosureFactV1(asked,cache,pass);
   expect(fast.fact,entries.join(",")).toBe(plain.fact);
   expect(new Set(fast.rows.map((row:any)=>JSON.stringify(row))),entries.join(",")+" support").toEqual(new Set(plain.rows.map((row:any)=>JSON.stringify(row))));
   return fast.fact;
  };
  const first=await both(["entry.ts"]);await both(["a.ts","b.ts"]);await both(["cycle2.ts"]);await both(["uses-gen.ts"]);
  await both(["router.ts"],[["router.ts",["a"]]]);await both(["router.ts"],[["router.ts",["a","b"]]]);await both(["entry.ts"],[],["data.json"]);
  writeFileSync(join(root,"cycle2.ts"),'import "./cycle1.ts";import "./leaf.ts";import "./later.ts";export const c2=1;\n');
  expect(await both(["entry.ts"])).not.toBe(first);
  writeFileSync(join(root,"leaf.ts"),"export const leaf=987654321; // body edit\n");
  await both(["entry.ts"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("two closures asked at the same time share the nodes they both need, however their builds interleave",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../🟨️.mjs") as any,root=realpathSync(mkdtempSync(join(tmpdir(),"walk-concurrent-")));
 try{
  write(root,TREE);writeFileSync(join(root,"package.json"),'{"name":"walk-fixture"}\n');
  for(const transient of[false,true]){
   const cache=inference.createScriptInputCache({workspaceRoot:root}),pass:any=transient?{transient:true}:{},ask=(entry:string)=>({kind:"derived",path:JSON.stringify({walk:1,workspaceRoot:root,roots:[join(root,entry)],requiredSources:false,resources:[],selectors:[],provider:cache.providerDesignation??null})});
   const answers=await Promise.all(["entry.ts","a.ts","b.ts","cycle1.ts","uses-gen.ts"].map(entry=>scripts.deriveClosureFactV1(ask(entry),cache,pass)));
   expect(answers.every((answer:any)=>/^[a-f0-9]{64}$/u.test(answer.fact))).toBe(true);
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("with a journal dirty set a pass verifies only the nodes the set can touch and still follows an edit the set names",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../🟨️.mjs") as any,{dirtySummaryV1}=await import("../../../../🧮️derived-nodes/🧩️closure-kinds/🧭️row-paths/🟨️.mjs") as any,root=realpathSync(mkdtempSync(join(tmpdir(),"walk-dirty-")));
 try{
  write(root,TREE);writeFileSync(join(root,"package.json"),'{"name":"walk-fixture"}\n');
  const cache=inference.createScriptInputCache({workspaceRoot:root}),asked={kind:"derived",path:JSON.stringify({walk:1,workspaceRoot:root,roots:[join(root,"entry.ts")],requiredSources:false,resources:[],selectors:[],provider:cache.providerDesignation??null})};
  const before=await scripts.deriveClosureFactV1(asked,cache,{transient:true});
  writeFileSync(join(root,"cycle2.ts"),'import "./cycle1.ts";import "./leaf.ts";import "./later.ts";export const c2=1;\n');
  const clean=await scripts.deriveClosureFactV1(asked,cache,{transient:true,dirty:dirtySummaryV1({paths:["cycle2.ts"]})});
  cache.pass={};const plain=await scripts.deriveClosureFactV1(asked,cache,{});
  expect(clean.fact).not.toBe(before.fact);expect(clean.fact).toBe(plain.fact);expect(clean.files).toContain("{workspaceRoot}/later.ts");
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the bitset reach walk equals the Python reachability on a generated graph with cycles, over two passes of one cache",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../🟨️.mjs") as any,observation=await import("../../../../../👁️observation/🟨️.mjs") as any,root=realpathSync(mkdtempSync(join(tmpdir(),"walk-bitset-")));
 try{
  const count=140;let state=12345;const draw=(limit:number)=>{state=(state*1103515245+12345)%2147483648;return(state>>8)%limit;};
  const files:Record<string,string>={"package.json":'{"name":"walk-bitset"}\n'};
  for(let at=0;at<count;at++){const imports=new Set<number>();for(let item=0;item<1+draw(4);item++)imports.add(draw(count));files["m"+at+".ts"]=[...imports].map(target=>'import "./m'+target+'.ts";').join("")+"export const v"+at+"="+at+";\n";}
  write(root,files);
  const entries=Array.from({length:30},(_,at)=>[join(root,"m"+(at*4)+".ts"),join(root,"m"+((at*7+1)%count)+".ts")]);
  const cache=inference.createScriptInputCache({workspaceRoot:root});
  const ask=async(entry:string[])=>{const row={kind:"derived",path:JSON.stringify({walk:1,workspaceRoot:root,roots:entry,resources:[],selectors:[],requiredSources:false,provider:null}),fact:"none"},pass={transient:true};const answer=await observation.observeInferenceSnapshotV1(()=>scripts.deriveClosureFactV1(row,cache,pass));return(answer.files as string[]).map(path=>path.replace("{workspaceRoot}/","")).filter(path=>path!=="package.json");};
  let cyclic=false;
  for(let round=0;round<2;round++)for(const entry of entries){
   const got=await ask(entry),want=expected(root,entry.map(path=>path.slice(root.length+1)));
   expect(got).toEqual(want);if(got.length>20)cyclic=true;
  }
  expect(cyclic).toBe(true);
  expect(cache.reachState.sets.stats().bytes).toBeLessThan(count*count/8+count*64);
 }finally{rmSync(root,{recursive:true,force:true});}
});
