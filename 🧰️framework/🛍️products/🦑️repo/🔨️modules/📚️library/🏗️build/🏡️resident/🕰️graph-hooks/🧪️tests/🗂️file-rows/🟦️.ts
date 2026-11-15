import {afterAll,describe,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {createFrontierV1} from "../../../../🕸️graph/🕰️generations/🧭️frontier/🟨️.mjs";
import {walkFrontierV1} from "../../../../🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs";
import {lstat,readdir} from "node:fs/promises";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {writeFileSync as put} from "node:fs";
import {dirtyIndexV1} from "../../🟨️.mjs";
import {encodeFactIndexV1,openFactIndexV1} from "../../../../🕸️graph/🕰️generations/🧮️fact-codec/🟨️.mjs";
import {journalGraphSeamsV1,rowFilePathV1} from "../../🔌️host/🟨️.mjs";

const sandboxes:string[]=[];
afterAll(()=>sandboxes.forEach(path=>rmSync(path,{recursive:true,force:true})));
const tree:Record<string,string>={"a/x/project.json":"{}","a/x/src/main.ts":"x","a/y/Cargo.toml":"c","b/z.feature":"f","top/project.json":"{}","ignored/q.ts":"q"};
const instance="0123456789abcdef";
const ports=(root:string)=>({
 list:async(directory:string)=>{try{return(await readdir(join(root,directory),{withFileTypes:true})).map(entry=>({name:entry.name,kind:entry.isSymbolicLink()?"other" as const:entry.isDirectory()?"directory" as const:entry.isFile()?"file" as const:"other" as const}));}catch{return undefined;}},
 stat:async(path:string)=>{try{const status=await lstat(join(root,path));return{directory:status.isDirectory(),file:status.isFile(),mtimeMs:status.mtimeMs,ino:status.ino,mode:status.mode};}catch{return undefined;}},
 descend:(name:string)=>name!=="node_modules",
 member:(path:string)=>["project.json","Cargo.toml"].includes(path.slice(path.lastIndexOf("/")+1))||path.endsWith(".feature"),
});
const setup=async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"file-rows-"));sandboxes.push(root);
 for(const[path,content]of Object.entries(tree)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),content);}
 const state={head:5,answers:[] as any[]},map=new Map<string,any>(),held=ports(root),checked:string[][]=[],scans={count:0},notices:any[]=[];
 const journal={async cursor(){return{instance,seq:state.head};},async since(){return state.answers.length?state.answers.shift():{complete:true,cursor:{instance,seq:state.head},rows:[]};},excluded:()=>(path:string)=>path.startsWith("ignored/")};
 const store={get:(key:string)=>map.get(key),set:(key:string,value:any,carry?:any)=>{map.set(key,carry?{...value,carry}:value);}};
 const scan=async()=>{scans.count++;return{digest:"d"+scans.count,frontier:createFrontierV1({capturedAt:Date.now(),...await walkFrontierV1(held)}),changes:[]};};
 const seams=journalGraphSeamsV1({root,journal,store,ports:held,digest:members=>"m"+members.length,membership:scan,holds:async rows=>{checked.push(rows.map(row=>row.path));return true;},notify:notice=>notices.push(notice)});
 return{root,state,map,checked,scans,notices,seams};
};
const abs=(root:string,path:string)=>root+"/"+path;

describe("file rows (field, digest) are vouched by the cursor",()=>{
 test("rowFilePathV1 maps relative rows, absolute rows, drive paths and escapes",()=>{
  expect(rowFilePathV1("/w/r","a/b/Cargo.toml")).toBe("a/b/Cargo.toml");
  expect(rowFilePathV1("/w/r","/w/r/a/b.json")).toBe("a/b.json");
  expect(rowFilePathV1("/w/r","/etc/hosts")).toBeUndefined();
  expect(rowFilePathV1("/w/r","../x/package.json")).toBeUndefined();
  expect(rowFilePathV1("C:\\w\\r","C:\\w\\r\\a\\b.json")).toBe("a/b.json");
  expect(rowFilePathV1("C:\\w\\r","a\\b.json")).toBe("a/b.json");
 });
 test("only the rows whose file is named by the cursor, lies outside the workspace or in a region the cursor cannot see reach the precise check",async()=>{
  const s=await setup(),first=await s.seams.membership(undefined);s.seams.commit({generation:4,owners:["a"],covered:undefined});
  s.state.head=11;s.state.answers.push({complete:true,cursor:{instance,seq:11},rows:[["m",0,"a/y/Cargo.toml"],["a",0,"a/x/go.mod"]]});
  await s.seams.membership({manifest:{generation:4,membership:first.digest},frontier:first.frontier});
  const rows=[
   {kind:"field",path:"a/x/project.json",fact:{extract:"project-json",value:{}}},
   {kind:"field",path:"a/y/Cargo.toml",fact:{extract:"cargo-package",value:{}}},
   {kind:"field",path:"a/x/go.mod",fact:{extract:"go-mod",value:null}},
   {kind:"field",path:"a/y/go.mod",fact:{extract:"go-mod",value:null}},
   {kind:"digest",path:"top/project.json",fact:{sha256:"x",size:1,mtimeMs:1}},
   {kind:"field",path:abs(s.root,"a/y/Cargo.toml"),fact:{digest:"d",format:"json",pointer:"/x"}},
   {kind:"field",path:abs(s.root,"b/z.feature"),fact:{digest:"d",format:"json",pointer:"/x"}},
   {kind:"field",path:"/usr/lib/outside.json",fact:{}},
   {kind:"digest",path:"ignored/q.ts",fact:{sha256:"x",size:1,mtimeMs:1}},
   {kind:"stamp",path:"a/x/project.json",fact:{}},
  ];
  expect(Array.from(s.seams.doubtful(rows as never)!)).toEqual([0,1,1,0,0,1,0,1,1,1]);
  expect(await s.seams.holds(rows as never)).toBe(true);
  expect(s.checked.at(-1)).toEqual([rows[1],rows[2],rows[5],rows[7],rows[8],rows[9]].map(row=>row.path));
 });
 test("a cursor that reports a file the cursor sees changed never lets its rows through, for every file kind of row",async()=>{
  const s=await setup(),first=await s.seams.membership(undefined);s.seams.commit({generation:4,owners:["a"],covered:undefined});
  const paths=Object.keys(tree).filter(path=>!path.startsWith("ignored/"));
  s.state.answers.push({complete:true,cursor:{instance,seq:6},rows:paths.map(path=>["m",0,path])});
  await s.seams.membership({manifest:{generation:4,membership:first.digest},frontier:first.frontier});
  const rows=paths.flatMap(path=>[{kind:"field",path,fact:{}},{kind:"digest",path:abs(s.root,path),fact:{}},{kind:"file",path:abs(s.root,path),fact:{}}]);
  expect(Array.from(s.seams.doubtful(rows as never)!).every(level=>level===1)).toBe(true);
 });
});

describe("a cold build records its cursor when it publishes",()=>{
 test("a generation published by a cold build has its cursor before the build is sealed, so the next reader needs no verify epoch",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);
  s.seams.scope(true);
  s.seams.published(7);
  expect(s.map.get("g7")).toEqual({instance,seq:5});
  s.state.head=9;
  const again=await s.seams.membership({manifest:{generation:7,membership:first.digest},frontier:first.frontier});
  expect(s.notices).toEqual([]);expect(s.scans.count).toBe(1);expect(again.digest).toBe(first.digest);
 });
 test("a scoped build records nothing, and an edit made after the build started is reported by the recorded cursor",async()=>{
  const s=await setup();
  const first=await s.seams.membership(undefined);
  s.seams.scope(false);s.seams.published(3);
  expect(s.map.size).toBe(0);
  s.seams.scope(true);s.seams.published(4);
  s.state.answers.push({complete:true,cursor:{instance,seq:8},rows:[["m",0,"a/x/src/main.ts"]]});
  await s.seams.membership({manifest:{generation:4,membership:first.digest},frontier:first.frontier});
  const row={kind:"file",path:abs(s.root,"a/x/src/main.ts"),fact:{}};
  expect(Array.from(s.seams.doubtful([row] as never)!)).toEqual([1]);
 });
});

describe("the verdict of a path is remembered for one pass only",()=>{
 test("rows sharing a path get one verdict per pass, and a later pass with another dirty set starts afresh",async()=>{
  const s=await setup(),first=await s.seams.membership(undefined);s.seams.commit({generation:4,owners:["a"],covered:undefined});
  const previous={manifest:{generation:4,membership:first.digest},frontier:first.frontier};
  const rows=[{kind:"file",path:abs(s.root,"a/x/src/main.ts"),fact:{}},{kind:"analysis",path:abs(s.root,"a/x/src/main.ts"),fact:{}},{kind:"field",path:"a/y/Cargo.toml",fact:{}},{kind:"digest",path:abs(s.root,"a/y/Cargo.toml"),fact:{}}];
  s.state.answers.push({complete:true,cursor:{instance,seq:6},rows:[["m",0,"a/x/src/main.ts"],["m",0,"a/y/Cargo.toml"]]});
  await s.seams.membership(previous);
  expect(Array.from(s.seams.doubtful(rows as never)!)).toEqual([1,1,1,1]);
  expect(Array.from(s.seams.doubtful(rows as never)!)).toEqual([1,1,1,1]);
  s.state.answers.push({complete:true,cursor:{instance,seq:7},rows:[["m",0,"b/z.feature"]]});
  await s.seams.membership(previous);
  expect(Array.from(s.seams.doubtful(rows as never)!)).toEqual([0,0,0,0]);
 });
});

describe("the row mapping equals an independent Python model",()=>{
 test("random file rows, dirty paths and subtrees give the same verdict",()=>{
  let seed=12345;const next=(n:number)=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed%n;};
  const names=["a","b","src","x.json","go.mod","Cargo.toml","deep"],pick=()=>Array.from({length:1+next(4)},()=>names[next(names.length)]).join("/");
  const root="/w/r",cases:any[]=[];
  for(let i=0;i<300;i++){
   const exact=Array.from({length:next(5)},pick),subtrees=Array.from({length:next(3)},pick),kind=next(5);
   const path=kind===0?pick():kind===1?root+"/"+pick():kind===2?"/elsewhere/"+pick():kind===3?"../"+pick():root;
   cases.push({root,path,exact,subtrees});
  }
  const file=join(mkdtempSync(join(realpathSync(tmpdir()),"file-rows-py-")),"cases.json");sandboxes.push(dirname(file));put(file,JSON.stringify(cases));
  const python=testPythonV1();
  if(!python)return;
  const out=Bun.spawnSync([python,"-I",join(import.meta.dir,"🐍️oracle.py"),file]);
  if(out.exitCode!==0)throw Error(out.stderr.toString());
  const expected=JSON.parse(out.stdout.toString());
  const actual=cases.map(item=>{const path=rowFilePathV1(item.root,item.path);if(path===undefined)return 1;const index=dirtyIndexV1({exact:item.exact,subtrees:item.subtrees});return index.has(path)?1:0;});
  expect(actual).toEqual(expected);
 });
});

describe("verdicts by id equal verdicts by row",()=>{
 test("for every kind of row and for dirty sets of several shapes, a lazily opened index answers the same without decoding the rows",async()=>{
  const s=await setup(),first=await s.seams.membership(undefined);s.seams.commit({generation:4,owners:["a"],covered:undefined});
  const kinds=["file","exists","readdir","analysis","field","digest","closure","derived","resolve","stamp","lstat"],paths=Object.keys(tree).flatMap(path=>[path,abs(s.root,path),abs(s.root,path.replace(/[^/]*$/u,"")+"gone.ts")]).concat(["/usr/lib/outside.js","top","a/x"]);
  const rows:any[]=[];for(const kind of kinds)for(const path of paths)rows.push({kind,path:kind==="resolve"?JSON.stringify([abs(s.root,"a/x/src/main.ts"),path]):path,fact:kind==="resolve"?path:{}});
  const index=openFactIndexV1(encodeFactIndexV1({version:1,owners:["a"],rows,classes:[{r:rows.map((_,id)=>id),o:[0]}]} as never,{maximumBytes:1<<26}),{maximumBytes:1<<26});
  for(const dirty of [[],[["m",0,"a/x/src/main.ts"]],[["a",0,"a/x/go.mod"],["r",1,"b"]],[["m",0,"top/project.json"],["m",0,"a/y/Cargo.toml"]]]){
   s.state.answers.push({complete:true,cursor:{instance,seq:6},rows:dirty});
   await s.seams.membership({manifest:{generation:4,membership:first.digest},frontier:first.frontier});
   const ids=rows.map((_,id)=>id),byRows=Array.from(s.seams.doubtful(ids.map(id=>index.rows[id]) as never)!),readBefore=index.rowsRead();
   const byIds=Array.from(s.seams.doubtfulIds(index as never,ids)!);
   expect(byIds).toEqual(byRows);
   expect(index.rowsRead()).toBe(readBefore);
  }
 });
 test("an index that cannot answer by id and a pass without dirty set get no verdict by id",async()=>{
  const s=await setup(),first=await s.seams.membership(undefined);
  expect(s.seams.doubtfulIds({rowKindOf:()=>"file",rows:[]} as never,[0])).toBeUndefined();
  const index=openFactIndexV1(encodeFactIndexV1({version:1,owners:[],rows:[],classes:[]} as never,{maximumBytes:1<<20}),{maximumBytes:1<<20});
  expect(s.seams.doubtfulIds(index as never,[])).toBeUndefined();
  expect(first.digest).toBe("d1");
 });
});
