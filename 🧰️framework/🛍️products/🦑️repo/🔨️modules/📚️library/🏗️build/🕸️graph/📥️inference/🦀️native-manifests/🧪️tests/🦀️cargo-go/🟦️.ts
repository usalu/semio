/** 🦀 Contract: the native manifest readers of the build graph (Cargo path dependencies with aliases, target tables and workspace inheritance; the Cargo workspace a crate belongs to; Go module requirements and replacements) equal the authored cases in `./🔣️.json`, an independent Python derivation, a second TypeScript walk built on smol-toml, and, for Go, `go mod edit -json` of the Go toolchain itself. */
import {afterAll,beforeAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import {parse} from "smol-toml";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {goDependencies,nativeDependencies,nativeWorkspaceV1} from "../../🟨️.mjs";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"./🐍️oracle/🐍️.py");
const ask=(requests:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({requests}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const plain=<T>(value:T):T=>JSON.parse(JSON.stringify(value));
const attempt=<T>(action:()=>T):{value?:T;error?:Error&{code?:string}}=>{try{return{value:action()};}catch(error){return{error:error as Error};}};
let root="";
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"native-manifests-")));});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,dependencies:[{...corpus.dependencies[0],expected:[{name:"a",path:"b",workspace:false,kind:"deps"}]}]})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,workspaces:[{...corpus.workspaces[0],error:"x"}]})).toBe(false);
});

test("path dependencies follow aliases, target tables and workspace inheritance as the authored rows and the oracle say",()=>{
 const requests=corpus.dependencies.map(row=>{const manifest=plain(parse(row.toml));return{manifest,workspace:row.workspace};});
 const answers=ask(requests);
 for(const[index,row]of corpus.dependencies.entries()){
  expect(nativeDependencies(requests[index]!.manifest,row.workspace),row.name).toEqual(row.expected);
  expect(answers[index],row.name).toEqual(row.expected);
 }
});

const independentWorkspace=(directory:string,repository:string):{root:string;declaration:unknown}|{error:string}=>{
 let cursor=directory;const own=parse(readFileSync(join(directory,"Cargo.toml"),"utf8")) as any;
 if(own.package?.workspace)cursor=resolve(directory,own.package.workspace);
 for(;;){
  const rel=relative(repository,cursor);
  if(rel===".."||rel.startsWith(".."+"/")||rel.startsWith(".."+"\\"))return{error:"escapes"};
  const manifest=join(cursor,"Cargo.toml");
  if(existsSync(manifest)){const table=(parse(readFileSync(manifest,"utf8")) as any).workspace;if(table)return{root:cursor,declaration:plain(table)};}
  if(cursor===repository)return{root:cursor,declaration:{}};
  cursor=dirname(cursor);
 }
};

test("a crate belongs to the nearest workspace table above it, or above the directory its workspace key names, and never to one outside the repository",()=>{
 for(const[index,row]of corpus.workspaces.entries()){
  const repository=join(root,"w"+index);
  for(const[path,text]of Object.entries(row.files)){mkdirSync(dirname(join(repository,path)),{recursive:true});writeFileSync(join(repository,path),text);}
  const directory=resolve(repository,row.directory),got=attempt(()=>nativeWorkspaceV1(directory,repository));
  if("error" in row){
   expect(got.error,row.name).toBeDefined();
   const expected=(row.error as string).replace("{directory}",directory);
   if(expected==="ENOENT")expect(got.error!.code,row.name).toBe("ENOENT");else expect(got.error!.message,row.name).toBe(expected);
  }else{
   expect(got.error,row.name+" "+got.error?.message).toBeUndefined();
   const rel=relative(repository,got.value!.root)||".";
   expect({root:rel.replaceAll("\\","/"),declaration:plain(got.value!.declaration)},row.name).toEqual({root:row.root,declaration:row.declaration});
   const second=independentWorkspace(directory,repository);
   expect(second,row.name).toEqual({root:got.value!.root,declaration:plain(got.value!.declaration)});
  }
 }
});

test("Go requirements and replacements equal the authored rows",()=>{
 for(const row of corpus.go)expect<unknown>(goDependencies(row.text),row.name).toEqual({module:row.module,requires:row.requires,replacements:row.replacements});
});

const go=Bun.which("go"),withGo=go?test:test.skip;
withGo("Go module files read the same module, requirements and replacements as `go mod edit -json`",()=>{
 for(const[index,row]of corpus.go.entries()){
  if((row as any).noGo)continue;
  const directory=join(root,"go"+index);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"go.mod"),row.text);
  const run=spawnSync(go!,["mod","edit","-json","go.mod"],{cwd:directory,encoding:"utf8",env:{...process.env,GOFLAGS:"-mod=mod",GOTOOLCHAIN:"local"}});
  expect(run.status,row.name+" "+run.stderr).toBe(0);
  const edit=JSON.parse(run.stdout),actual=goDependencies(row.text);
  expect(actual.module,row.name).toBe(edit.Module?.Path??"");
  expect(actual.requires,row.name).toEqual((edit.Require??[]).map((entry:any)=>entry.Path));
  expect(actual.replacements,row.name).toEqual((edit.Replace??[]).map((entry:any)=>({module:entry.Old.Path,path:entry.New.Path})));
 }
});
