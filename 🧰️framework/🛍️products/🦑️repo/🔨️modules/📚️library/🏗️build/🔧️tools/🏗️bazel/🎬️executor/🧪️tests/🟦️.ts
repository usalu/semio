/** 🎬️ Contract: the static closure of the executor half of the adapter (the base of every narrow scope, which `../../🎬️scopes` extends with the trace-proven lazy roots and leaves of one action class) holds only the static imports of the executor, the JSON files they name and the schemas beside them, so editing planning, graph or launcher code can never change an action key; the bounds live in `./🔣️.json`, the TypeScript compiler API and an independent Python walk both derive the closure. */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,statSync,writeFileSync} from "node:fs";
import {randomBytes} from "node:crypto";
import {tmpdir} from "node:os";
import {dirname,join,normalize,resolve,sep} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {requireTestToolV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");

const regular=(path:string):boolean=>existsSync(path)&&statSync(path).isFile()&&!path.split(sep).some(part=>part==="node_modules"||part===".cache");

const schemaFiles=(folder:string):string[]=>existsSync(folder)?readdirSync(folder,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1).flatMap(entry=>entry.isDirectory()&&entry.name!=="node_modules"?schemaFiles(join(folder,entry.name)):entry.isFile()&&entry.name.endsWith(".json")?[join(folder,entry.name)]:[]):[];
const schemasBeside=(path:string):string[]=>schemaFiles(join(dirname(path),"🧬️schema"));

/** 🧭️ The static imports of one module through the compiler API: import and export-from declarations that are not type-only. */
const staticSpecifiers=(path:string):string[]=>{
 const file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),found:string[]=[];
 for(const node of file.statements){
  if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);
  if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);
 }
 return found;
};

const jsonLiteral=/(["'])(\.{1,2}\/[^"'\\\r\n]+\.json)\1/gu;

const compilerClosure=(entry:string,roots:readonly string[],leaves:readonly string[]=[]):string[]=>{
 const seen=new Set<string>(),pending=[normalize(entry),...roots.map(root=>normalize(root))];
 while(pending.length){
  const path=pending.pop()!;if(seen.has(path)||!regular(path))continue;seen.add(path);if(path.endsWith(".json"))continue;
  pending.push(...schemasBeside(path));
  for(const specifier of staticSpecifiers(path))if(specifier.startsWith("."))pending.push(normalize(join(dirname(path),specifier)));
  for(const match of readFileSync(path,"utf8").matchAll(jsonLiteral))pending.push(normalize(join(dirname(path),match[2]!)));
 }
 for(const leaf of leaves)if(regular(normalize(leaf)))seen.add(normalize(leaf));
 return [...seen].sort();
};

const checkout=resolve(import.meta.dir,corpus.checkoutFromTests),entry=resolve(import.meta.dir,corpus.entryFromTests),adapter=resolve(import.meta.dir,corpus.adapter),roots:string[]=[],leaves:string[]=[];

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,maximumFiles:0})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,forbidden:[]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,forbidden:[{fragment:"x"}]})).toBe(false);
});

test("the compiler API and the independent Python walk derive the same executor closure",()=>{
 const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({entry,roots,leaves}),encoding:"utf8",maxBuffer:1<<26});
 expect(run.status,run.stderr).toBe(0);
 const oracle=(JSON.parse(run.stdout) as string[]).sort(),actual=compilerClosure(entry,roots,leaves);
 expect(actual.length).toBeGreaterThan(corpus.required.length);
 expect(actual).toEqual(oracle);
});

test("the executor closure stays inside its bound and never reaches planning, graph or launcher modules",()=>{
 const closure=compilerClosure(entry,roots,leaves);
 expect(closure.length,"the executor closure outgrew its bound").toBeLessThanOrEqual(corpus.maximumFiles);
 for(const row of corpus.forbidden)expect(closure.filter(path=>path.includes(row.fragment)),row.reason).toEqual([]);
 for(const row of corpus.required)expect(closure.some(path=>path.includes(row.fragment)),row.reason).toBe(true);
});

test("the adapter's static closure holds the executor closure, so the adapter's process admits it",()=>{
 const wide=new Set(compilerClosure(adapter,[]));
 expect(wide.size).toBeGreaterThan(compilerClosure(entry,[]).length);
 for(const path of compilerClosure(entry,[]))expect(wide.has(path),path).toBe(true);
});

test("launcher placement links on one volume, else keeps one shared copy per output base and every launcher holds the runtime's exact bytes, agreeing with an independent Python sha256",async()=>{
 const {placeBazelLauncherV1}=await import("../🟦️.ts"),root=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"launcher-")),source=join(root,"bun"),bytes=randomBytes(1<<20),refuse=()=>Promise.reject(Object.assign(new Error("cross-device link"),{code:"EXDEV"}));
 writeFileSync(source,bytes,{mode:0o755});
 const place=async(base:string,count:number,link?:typeof refuse):Promise<string[]>=>{const outputs:string[]=[];for(let index=0;index<count;index++){const directory=join(root,base,"execroot","_main","bazel-out","bin");mkdirSync(directory,{recursive:true});const output=join(directory,"t_"+index);await placeBazelLauncherV1(source,output,link as never);outputs.push(output);}return outputs;};
 const linked=await place("linked",3),copied=[...await place("s0",5,refuse),...await place("s1",4,refuse)];
 expect(existsSync(join(root,"linked","launchers"))).toBe(false);
 expect(new Set(linked.map(path=>statSync(path).ino)).size).toBe(1);
 for(const base of["s0","s1"])expect(readdirSync(join(root,base,"launchers")).length,base).toBe(1);
 const digest=spawnSync(python,["-I","-c","import hashlib,sys\nfor path in sys.argv[1:]:\n print(hashlib.sha256(open(path,'rb').read()).hexdigest())",source,...linked,...copied],{encoding:"utf8"});
 expect(digest.status,digest.stderr).toBe(0);
 const rows=digest.stdout.trim().split("\n");expect(rows.length).toBe(1+linked.length+copied.length);expect(new Set(rows).size).toBe(1);
 for(const path of copied)expect(statSync(path).mode&0o100,path).toBeGreaterThan(0);
});
