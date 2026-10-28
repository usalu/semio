/** 🗂️ Contract: workspace discovery lists every directory that carries its own manifest below the root without descending into build, vendor, hidden and scratch trees, drops payload packages owned through an export namespace, rejects duplicate names and malformed manifests, reports progress and honours cancellation, and the membership comparison separates missing, stale and reordered entries; the corpus is language-agnostic and `🐍️.py` walks the same tree with os.walk. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {resolve} from "node:path";
import type {PackagePayloadEntry,PackagePayloadOperations} from "../📦️payload/🟦️.ts";
import {compareWorkspaceMembership} from "../⚖️membership-comparison/🟦️.ts";
import {computeWorkspaces,diffWorkspaces,getWorkspaceRoot} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const roots:string[]=[];afterAll(()=>{process.chdir(original);for(const root of roots)rmSync(root,{recursive:true,force:true});});
const original=process.cwd();
const build=(files:Readonly<Record<string,string|undefined>>):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"workspaces-")));roots.push(root);for(const[path,text]of Object.entries(files)){if(text===undefined)continue;const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}return root;};
const virtual=(files:Record<string,string>,root:string):PackagePayloadOperations=>{
 const table=new Map<string,string>(Object.entries(files).map(([path,text])=>[join(root,...path.split("/")),text])),directories=new Set<string>([root]);
 for(const path of table.keys()){let parent=dirname(path);while(parent.length>=root.length){directories.add(parent);if(parent===root)break;parent=dirname(parent);}}
 return{list:path=>directories.has(path)?[...new Set([...table.keys(),...directories].filter(other=>dirname(other)===path&&other!==path))].sort().map(other=>({name:other.slice(path.length+1),kind:directories.has(other)?"directory":"file"} as PackagePayloadEntry)):[],readText:path=>table.get(path)??"",state:path=>directories.has(path)&&path!==root?"directory":table.has(path)?"file":path===root?"directory":"missing"};
};
test("physical packages are emitted once, payloads and skipped trees never",()=>{const root=build(corpus.tree);expect(computeWorkspaces(root)).toEqual(corpus.expected);});
test("the same discovery over injected operations agrees with the physical one",()=>{const root=resolve("/virtual-root");expect(computeWorkspaces(root,{operations:virtual(corpus.tree,root)})).toEqual(corpus.expected);});
for(const row of corpus.errors)test("discovery fails on "+row.name,()=>{const root=build({...corpus.tree,...row.extra});expect(()=>computeWorkspaces(root)).toThrow(row.message);});
test("a missing or non-directory root is refused",()=>{const root=build({"file.txt":"x"});expect(()=>computeWorkspaces(join(root,"missing"))).toThrow("Workspace root must be a physical directory");expect(()=>computeWorkspaces(join(root,"file.txt"))).toThrow("Workspace root must be a physical directory");});
test("progress reports every scanned directory and a raised signal cancels",()=>{
 const root=build(corpus.tree),seen:string[]=[];computeWorkspaces(root,{onProgress:progress=>seen.push(progress.relativeDirectory)});
 for(const directory of corpus.progressDirs)expect(seen,directory).toContain(directory);expect(seen[0]).toBe("");expect(seen.some(directory=>directory.startsWith("node_modules")||directory.startsWith("target"))).toBe(false);
 expect(()=>computeWorkspaces(root,{signal:{aborted:true}})).toThrow("Workspace discovery cancelled");
 let calls=0;expect(()=>computeWorkspaces(root,{signal:{get aborted(){calls+=1;return calls>3;}}})).toThrow("Workspace discovery cancelled");
});
for(const row of corpus.membership)test("membership comparison: "+row.name,()=>{
 const root=build(corpus.tree),diff=diffWorkspaces(root,row.current),comparison=compareWorkspaceMembership(root,row.current);
 expect(diff.expected).toEqual(corpus.expected);expect(diff.missing).toEqual(row.missing);expect(diff.stale).toEqual(row.stale);
 expect({fresh:comparison.fresh,orderChanged:comparison.orderChanged,missing:comparison.missing,stale:comparison.stale,expected:comparison.expected}).toEqual({fresh:row.fresh,orderChanged:row.orderChanged,missing:row.missing,stale:row.stale,expected:corpus.expected});
});
for(const row of corpus.env)test("workspace root from the environment: "+row.name,()=>{
 const root=build({"package.json":JSON.stringify({name:"unrelated"})});process.chdir(root);
 try{const found=getWorkspaceRoot(row.env);expect(found).toBe(row.expected===null?root:row.expected==="REL"?resolve("rel/root"):resolve(row.expected));}finally{process.chdir(original);}
});
test("without hints the nearest package named workspace above the working directory is the root",()=>{
 const root=build({"package.json":JSON.stringify({name:corpus.workspaceMarker.name}),[corpus.workspaceMarker.nested+"/package.json"]:JSON.stringify({name:"inner"}),[corpus.workspaceMarker.nested+"/x.txt"]:"x"});
 try{process.chdir(join(root,...corpus.workspaceMarker.nested.split("/")));expect(realpathSync(getWorkspaceRoot({}))).toBe(root);}finally{process.chdir(original);}
 const bare=build({"a/b/x.txt":"x"});try{process.chdir(join(bare,"a","b"));expect(realpathSync(getWorkspaceRoot({}))).toBe(realpathSync(process.cwd()));}finally{process.chdir(original);}
 const broken=build({"package.json":"{","a/x.txt":"x"});try{process.chdir(join(broken,"a"));expect(typeof getWorkspaceRoot({})).toBe("string");}finally{process.chdir(original);}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
