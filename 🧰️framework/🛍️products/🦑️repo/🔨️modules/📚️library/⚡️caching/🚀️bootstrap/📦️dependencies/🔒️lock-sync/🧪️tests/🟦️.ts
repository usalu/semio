/** 🔒️ Contract: a stale lock is refused before any capacity wait, store custody or installer, in every offered language, naming the scope, the members that changed and the command that fixes it; the corpus is language-agnostic, Ajv validates its shape, `🐍️.py` renders the texts and discovers the scopes independently, and the pinned Bun's own refusal text is what the translation recognises. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {checkStagedJavascriptLockV1,javascriptFrozenInstallRefusalV1,javascriptLockRefusalTextV1,readJavascriptLockScopesV1,requireJavascriptLocksInSyncV1,requireStagedJavascriptLocksInSyncV1,type StagedJavascriptLockV1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),encode=(value:unknown)=>new TextEncoder().encode(typeof value==="string"?value:JSON.stringify(value));
const checkout=(files:Readonly<Record<string,string>>):string=>{const root=mkdtempSync(join(tmpdir(),"lock-sync-"));for(const [path,content] of Object.entries(files)){const file=join(root,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,content);}return root;};
const stagedOf=(row:(typeof corpus.staged)[number]):StagedJavascriptLockV1=>({scope:row.scope,members:row.members,directory:"/stage",files:[{path:join("/stage","package.json"),bytes:encode(row.manifest)},{path:join("/stage","bun.lock"),bytes:encode(row.lock)},...Object.entries(row.projected).map(([key,value])=>({path:join("/stage",key,"package.json"),bytes:encode(value)}))]});
const clean=(value:unknown)=>JSON.parse(JSON.stringify(value));

test("the corpus matches its schema",()=>{const validate=new Ajv({strict:false,allErrors:true}).compile(schema);expect(validate(corpus)).toBe(true);expect(validate.errors).toBeNull();});
for(const row of corpus.texts)test("text: "+row.name,()=>{expect(javascriptLockRefusalTextV1(row.kind as "stale"|"refused",row.scope,row.drift as never,row.languages)).toBe(row.expected);});
for(const row of corpus.scopes)test("scopes: "+row.name,()=>{
 const root=checkout(row.files as Record<string,string>);
 try{
  if("error" in row.expected)expect(()=>readJavascriptLockScopesV1(root)).toThrow(row.expected.error);
  else expect(readJavascriptLockScopesV1(root).map(scope=>({scope:scope.scope,manifests:[...scope.manifests.keys()]}))).toEqual(row.expected.scopes);
 }finally{rmSync(root,{recursive:true,force:true});}
});
for(const row of corpus.staged)test("staged: "+row.name,()=>{expect(clean(checkStagedJavascriptLockV1(stagedOf(row))?.drift)).toEqual(row.expected);});
for(const row of corpus.frozen)test("frozen: "+row.name,()=>{
 const error=javascriptFrozenInstallRefusalV1(row.scope,row.stderr,undefined,row.environment);
 if(row.expectedContains===null){expect(error).toBeUndefined();return;}
 expect(error).toBeInstanceOf(Error);
 for(const text of row.expectedContains)expect(error!.message).toContain(text);
 for(const text of (row as {expectedAbsent?:string[]}).expectedAbsent??[])expect(error!.message).not.toContain(text);
 const cause=error!.cause as {purpose:string;scopes:{scope:string;command:string;en:string;de:string}[]};
 expect(cause.purpose).toBe("dependency-lock-refusal");expect(cause.scopes[0]!.scope).toBe(row.scope);expect(cause.scopes[0]!.command).toBe("repo dependencies lock --scope "+row.scope);
});
test("a stale staged scope is refused with both texts and a command in its cause",()=>{
 const stale=corpus.staged[0]!;let refusal:Error|undefined;
 try{requireStagedJavascriptLocksInSyncV1([stagedOf(stale)],{SEMIO_LANGUAGE:"en"});}catch(error){refusal=error as Error;}
 expect(refusal?.message).toContain("b declares dependencies x 1");
 const cause=refusal!.cause as {kind:string;scopes:{drift:unknown[];de:string}[]};
 expect(cause.kind).toBe("stale");expect(cause.scopes[0]!.de).toContain("Die Lock-Datei des Abhängigkeits-Scopes s1 ist veraltet");
 expect(()=>requireStagedJavascriptLocksInSyncV1([stagedOf(corpus.staged[2]!)],{SEMIO_LANGUAGE:"en"})).not.toThrow();
});
test("lock noise Bun accepts is not refused",()=>{
 const noisy={...corpus.staged[2]!,projected:{"w/0":{name:"a",version:"9.9.9",dependencies:{a:"workspace:*"}}}};
 expect(()=>requireStagedJavascriptLocksInSyncV1([stagedOf(noisy as never)],{SEMIO_LANGUAGE:"en"})).not.toThrow();
 expect(clean(checkStagedJavascriptLockV1(stagedOf(noisy as never))?.drift).length).toBeGreaterThan(0);
});
test("the preflight refuses every selected stale scope by directory and passes the unselected ones",()=>{
 const root=checkout(corpus.scopes[0]!.files as Record<string,string>);
 try{
  expect(()=>requireJavascriptLocksInSyncV1(root,undefined,{SEMIO_LANGUAGE:"en"})).toThrow("--scope s1");
  expect(()=>requireJavascriptLocksInSyncV1(root,["s1"],{SEMIO_LANGUAGE:"de"})).toThrow("Abhängigkeits-Scopes s1 ist veraltet");
  expect(()=>requireJavascriptLocksInSyncV1(root,["s2"],{SEMIO_LANGUAGE:"en"})).not.toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("every install path asks the lock check before it queues, installs or reports a frozen failure",()=>{
 const runtime=readFileSync(join(here,"..","..","..","..","..","🏗️build","⌨️command","⚙️runtime","🟦️.ts"),"utf8"),storage=readFileSync(join(here,"..","..","💾️storage","🟦️.ts"),"utf8");
 const guard=runtime.indexOf('buildOperationRequiresV1(command)==="dependencies"');
 expect(guard).toBeGreaterThan(0);
 expect(runtime.indexOf("requireJavascriptLocksInSyncV1(root,undefined,process.env)",guard)).toBeGreaterThan(guard);
 expect(runtime.indexOf("requireJavascriptLocksInSyncV1(root,undefined,process.env)",guard)).toBeLessThan(runtime.indexOf("ensureBuildEnvironmentV1(root,invocation",guard));
 expect(storage.indexOf("requireStagedJavascriptLocksInSyncV1(fresh.map")).toBeGreaterThan(0);
 expect(storage.indexOf("requireStagedJavascriptLocksInSyncV1(fresh.map")).toBeLessThan(storage.indexOf("await admitJavascriptDependencyGenerationV1(workspace,store,control,row.scope)"));
 const refusal=storage.indexOf("javascriptFrozenInstallRefusalV1(row.scope,terminal.stderr,{scope:row.scope,members:row.plan.members,directory:row.plan.directory,files:row.plan.files},process.env)");
 expect(refusal).toBeGreaterThan(0);
 expect(refusal).toBeLessThan(storage.indexOf("await failJavascriptDependencyInstallerV1(row.scope,terminal,control)"));
});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)throw Error("Independent Python oracle required");
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json"),join(here,"..","🧬️schema","🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
