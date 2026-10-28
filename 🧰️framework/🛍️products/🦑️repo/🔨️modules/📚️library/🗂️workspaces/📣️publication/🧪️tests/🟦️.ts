/** 📣️ Contract: workspace publication compares the root package document's membership with discovery, reports missing, stale and reordered entries on stderr and fails a stale check, rewrites the root document with the discovered membership and a trailing newline on write, writes nothing when already fresh and refuses a document that is not a regular file or changes during discovery; the corpus is language-agnostic and `🐍️.py` checks the written file against `json.dumps`. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {afterAll} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {publishWorkspaceMembership} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const roots:string[]=[];afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const build=(document:unknown):string=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"publication-")));roots.push(root);for(const[path,text]of Object.entries(corpus.tree)){const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}writeFileSync(join(root,"package.json"),JSON.stringify(document,null,2)+"\n");return root;};
const collect=()=>{const out:{stdout:string[];stderr:string[]}={stdout:[],stderr:[]};return{out,report:(stream:"stderr"|"stdout",message:string)=>{out[stream].push(message);}};};
for(const[name,row]of Object.entries(corpus.scenarios))test("publication: "+name,()=>{
 const root=build(row.document),{out,report}=collect(),before=readFileSync(join(root,"package.json"),"utf8");
 const run=()=>publishWorkspaceMembership(root,row.mode as "check"|"write",{report});
 if(row.throws!==null){expect(run).toThrow(row.throws);expect(readFileSync(join(root,"package.json"),"utf8")).toBe(before);}
 else{const result=run();expect({written:result.written,fresh:result.fresh,expectedCount:result.expectedCount}).toEqual({written:(row as {written:boolean}).written,fresh:(row as {fresh:boolean}).fresh,expectedCount:corpus.expected.length});}
 for(const line of row.stderr)expect(out.stderr,line).toContain(line);for(const line of row.stdout)expect(out.stdout,line).toContain(line);
 if(row.mode==="check")expect(readFileSync(join(root,"package.json"),"utf8")).toBe(before);
});
test("a write regenerates the root document with the discovered membership and a second run is a no-op",()=>{
 const root=build(corpus.rootDocument),first=publishWorkspaceMembership(root,"write",{report:()=>{}});expect(first.written).toBe(true);expect(first.missing).toEqual(["b/c"]);expect(first.stale).toEqual([]);
 const text=readFileSync(join(root,"package.json"),"utf8");expect(JSON.parse(text).workspaces).toEqual(corpus.expected);expect(text.endsWith("\n")).toBe(true);expect(JSON.parse(text).scripts).toEqual(corpus.rootDocument.scripts);
 expect(publishWorkspaceMembership(root,"write",{report:()=>{}}).written).toBe(false);expect(publishWorkspaceMembership(root,"check",{report:()=>{}}).fresh).toBe(true);
 if(python){const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(root,"package.json")]);expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});}
});
test("a document that changes during discovery is refused",()=>{
 const root=build(corpus.rootDocument);let reads=0;
 expect(()=>publishWorkspaceMembership(root,"write",{report:()=>{},operations:{readText:path=>{reads+=1;const text=readFileSync(path,"utf8");return reads>1?text+" ":text;}}})).toThrow("Root package.json changed during workspace discovery");
});
test("a root document that is not a regular file is refused",()=>{
 const root=build(corpus.rootDocument);for(const state of["symlink","other"]as const)expect(()=>publishWorkspaceMembership(root,"check",{report:()=>{},operations:{state:()=>state}})).toThrow("Root package.json must be a physical regular file ("+state+")");
 expect(()=>publishWorkspaceMembership(join(root,"missing"),"check",{report:()=>{}})).toThrow();
});
test("a malformed or ill-typed root document is refused before any discovery",()=>{
 const root=build(corpus.rootDocument);writeFileSync(join(root,"package.json"),"[]");expect(()=>publishWorkspaceMembership(root,"check",{report:()=>{}})).toThrow("Root package.json must contain an object");
 writeFileSync(join(root,"package.json"),'{"workspaces":"a"}');expect(()=>publishWorkspaceMembership(root,"write",{report:()=>{}})).toThrow("workspaces must be a string array");
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
