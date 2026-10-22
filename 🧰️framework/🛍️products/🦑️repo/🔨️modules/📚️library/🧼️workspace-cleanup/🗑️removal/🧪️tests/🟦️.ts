/** 🗑️ Contract: the workspace cleaner removes exactly the discovered misplaced folders, root transients, Cargo targets, closed-ticket output and illegal names, and refuses open tickets, managed storage, native images and protected prefixes in dry and applied runs; the corpus is language-agnostic and `🐍️oracle.py` replays the expected survivors on a rebuilt workspace. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {closeSync,existsSync,ftruncateSync,mkdirSync,mkdtempSync,openSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {sep} from "node:path";
import {cleanRemovePath,runWorkspaceClean} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const sparse=process.platform==="win32"||process.platform==="darwin"&&false;
const posix=(path:string):string=>path.split(sep).join("/");
const roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const world=():string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"removal-")));roots.push(root);for(const node of corpus.nodes){const target=join(root,...node.path.split("/"));mkdirSync(dirname(target),{recursive:true});if((node as {size?:number}).size!==undefined){const fd=openSync(target,"w");ftruncateSync(fd,(node as {size:number}).size);closeSync(fd);}else writeFileSync(target,(node as {text?:string}).text??"");}return root;};
const platformRows=(rows:{kind:string;path:string}[]):{kind:string;path:string}[]=>process.platform==="win32"?rows.filter(row=>row.kind!=="windows-illegal"):rows;
const presence=(root:string,paths:string[]):Record<string,boolean>=>Object.fromEntries(paths.map(path=>[path,existsSync(join(root,...path.split("/")))]));
const savedEnvironment:Record<string,string|undefined>={};
const isolate=():void=>{for(const key of["CARGO_TARGET_DIR","CARGO_BUILD_TARGET_DIR","CARGO_BUILD_BUILD_DIR"]){savedEnvironment[key]=process.env[key];delete process.env[key];}};
const restore=():void=>{for(const[key,value]of Object.entries(savedEnvironment))if(value===undefined)delete process.env[key];else process.env[key]=value;};
test("a dry run lists every removable row, spares open tickets, managed storage and images, and deletes nothing",()=>{
 if(process.platform==="win32")return;
 isolate();const root=world();
 try{
  const report=runWorkspaceClean(root,true);
  expect<unknown>(report.removals.map(row=>({kind:row.kind,path:posix(row.path)})).sort((a,b)=>a.path<b.path?-1:1)).toEqual(platformRows(corpus.dryRun.removals));
  expect(report.removals.every(row=>row.bytes>=0)).toBe(true);
  for(const path of corpus.dryRun.survivors)expect(existsSync(join(root,...path.split("/"))),path).toBe(true);
  for(const needle of corpus.skipped.contains)expect(report.skippedProtected.map(posix).some(path=>path.includes(needle))).toBe(true);
  expect(report.skippedProtected.map(posix).some(path=>path.includes("/OPEN"))).toBe(false);
 }finally{restore();}
});
test("an applied clean deletes exactly the listed rows",()=>{
 if(process.platform==="win32")return;
 isolate();const root=world();
 try{
  const report=runWorkspaceClean(root,false);
  expect(report.removals.length).toBe(corpus.dryRun.removals.length);
  for(const[path,present]of Object.entries(presence(root,corpus.applied.gone)))expect(present,path).toBe(false);
  for(const[path,present]of Object.entries(presence(root,corpus.applied.survivors)))expect(present,path).toBe(true);
  expect(runWorkspaceClean(root,false).removals).toEqual([]);
 }finally{restore();}
});
for(const row of corpus.removePath)test("remove path: "+row.name,()=>{
 if(process.platform==="win32"&&row.illegal)return;
 isolate();const root=world();
 try{
  const target=join(root,...row.path.split("/"));
  expect(cleanRemovePath(root,target,row.dry,[],row.ticket,row.illegal,row.cargo)).toBe(row.expected);
  expect(existsSync(target)).toBe(row.exists);
 }finally{restore();}
});
test("a protected prefix blocks removal of everything inside it",()=>{
 isolate();const root=world();
 try{
  expect(cleanRemovePath(root,join(root,"scratch","plain"),false,[join(root,"scratch")])).toBe(false);expect(existsSync(join(root,"scratch","plain"))).toBe(true);
  expect(cleanRemovePath(root,join(root,"scratch","plain"),false,[join(root,"scratch","plain","a.txt")])).toBe(false);
 }finally{restore();}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
