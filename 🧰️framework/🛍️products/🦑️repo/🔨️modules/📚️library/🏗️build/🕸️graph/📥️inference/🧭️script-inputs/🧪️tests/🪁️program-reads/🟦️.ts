/** 🎐 Contract: the closure of a declared TypeScript program (`metadata.semio.closure.typescript`) depends on the physical identity of the program's files, never on their bytes. The real `declaredProjectClosureV1`, run under the real observation layer with the real physical-source check, records no `file` fact; an edit that keeps a file a regular file reached through real directories keeps every recorded fact (the program's membership is the derived program fact's concern, and the real compiler shows exactly when it changes), while deleting a program file or turning it or an ancestor into a symbolic link or a directory breaks them. An independent Python model states the program and every verdict; the authored rows must agree with both. */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,renameSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,isAbsolute,join,relative,resolve} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {requireTestToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {evaluateExtractedFunctionV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧩️extracted-function/🟦️.ts";
import {declaredTypeScriptProjectsV1} from "../../🟨️.mjs";
import {readPhysicalSource,requirePhysicalSourceV1} from "../../../📍️coordinates/🟨️.mjs";
import {observationLayerHoldsV1,observeDetachedV1} from "../../../../../👁️observation/🟨️.mjs";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);

type Mutation=(typeof corpus.mutations)[number];
type Verdict={id:string;program:string[];holds:boolean;programChanges:boolean};
type Row={kind:string;path:string;fact:unknown};

const python=requireTestToolV1("python"),sandbox=realpathSync(mkdtempSync(join(tmpdir(),"program-reads-")));
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
const oracle=():Verdict[]=>{const run=spawnSync(python,["-I",join(import.meta.dir,"🐍️oracle","🐍️.py")],{input:JSON.stringify(corpus),encoding:"utf8"});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const declaration=():string=>{const text=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),file=ts.createSourceFile("inputs.mjs",text,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),node=file.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="declaredProjectClosureV1");if(!node)throw Error("Actual declared program closure required");return node.getText(file);};
const workspace=(id:string):string=>{const root=join(sandbox,id);for(const[path,text]of Object.entries(corpus.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}return root;};
const listing=(path:string)=>{try{return readdirSync(path,{withFileTypes:true}).map(entry=>({name:entry.name,directory:entry.isDirectory(),file:entry.isFile()}));}catch{return undefined;}};
const programOf=(root:string)=>declaredTypeScriptProjectsV1(["{projectRoot}/"+corpus.config],root,".",{compiler:ts,host:ts.sys,readdir:listing,join,dirname,isAbsolute});
const local=(root:string,paths:readonly string[])=>paths.map(path=>relative(root,path).split("\\").join("/")).sort();
const mutate=(root:string,mutation:Mutation)=>{
 const path=join(root,mutation.path),text="text" in mutation?String(mutation.text):"";
 if(mutation.op==="write")writeFileSync(path,text);
 else if(mutation.op==="append")writeFileSync(path,readFileSync(path,"utf8")+text);
 else if(mutation.op==="rewrite-keep-times"){const before=statSync(path);writeFileSync(path,text);utimesSync(path,before.atime,before.mtime);}
 else if(mutation.op==="replace"){writeFileSync(path+".next",text);renameSync(path+".next",path);}
 else if(mutation.op==="delete")rmSync(path);
 else if(mutation.op==="symlink-file"){const copy=join(root,"copies",mutation.path);mkdirSync(dirname(copy),{recursive:true});writeFileSync(copy,readFileSync(path));rmSync(path);symlinkSync(copy,path);}
 else if(mutation.op==="symlink-directory"){renameSync(path,path+"-real");symlinkSync(path+"-real",path,"dir");}
 else if(mutation.op==="directory"){rmSync(path);mkdirSync(path);}
};
const observeClosure=async(root:string,closure:readonly string[])=>{
 const program=programOf(root),compiledNames=new Map([[closure,new Set<string>()]]),cache={declared:new Map(),compiledNames,generatedImports:new Map()};
 const ports={resolve,memoDigestV1:(...parts:unknown[])=>JSON.stringify(parts),memoizedObservedComputationV1:(store:Map<string,{value:unknown;observations:object}>,key:string,body:()=>unknown)=>{let entry=store.get(key);if(!entry)store.set(key,entry={value:body(),observations:{}});return entry.value;},typescriptProgramV1:()=>program,walkFactsV1:new WeakMap(),programQuestionV1:()=>"question",programAnswerV1:()=>"answer",readPhysicalSource,requirePhysicalSourceV1,closurePathNameV1:(_cache:unknown,base:string,file:string)=>"{workspaceRoot}/"+relative(base,file).split("\\").join("/")};
 const closureOf=evaluateExtractedFunctionV1(declaration(),"declaredProjectClosureV1",ports);
 const {value,rows}=await observeDetachedV1(async()=>closureOf(["{projectRoot}/"+corpus.config],root,".",cache,closure));
 return {program,value:value as {closure:string[]},rows:rows as Row[]};
};

test("the corpus is closed under its schema and the authored verdicts are the independent model's",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).validate(schema,corpus)).toBe(true);
 const verdicts=oracle();expect(verdicts.map(row=>row.id)).toEqual(corpus.mutations.map(row=>row.id));
 for(const[index,mutation]of corpus.mutations.entries()){expect(verdicts[index]!.holds,mutation.id).toBe(mutation.holds);expect(verdicts[index]!.programChanges,mutation.id).toBe(mutation.programChanges);}
});

test("the declared program closure names the real compiler's program and records no byte fact of it",async()=>{
 const root=workspace("names"),verdict=oracle()[0]!,{program,value,rows}=await observeClosure(root,["{workspaceRoot}/command.ts"]);
 expect(local(root,program.files)).toEqual(verdict.program);
 expect(value.closure).toEqual([...new Set(["{workspaceRoot}/command.ts",...verdict.program.map(path=>"{workspaceRoot}/"+path)])].sort());
 expect(rows.filter(row=>row.kind==="file").map(row=>relative(root,row.path)),"no program file is read for its bytes").toEqual([]);
 for(const path of verdict.program)expect(rows.some(row=>row.kind==="lstat"&&row.path===join(root,path)&&row.fact==="file"),path).toBe(true);
});

test("an edit that keeps every program file physical keeps the recorded closure facts, an identity change breaks them",async()=>{
 const verdicts=oracle();
 for(const[index,mutation]of corpus.mutations.entries()){
  const root=workspace(mutation.id),{program,rows}=await observeClosure(root,[]);
  expect(observationLayerHoldsV1(rows),mutation.id+" before").toBe(true);
  await Bun.sleep(5);mutate(root,mutation);
  expect(observationLayerHoldsV1(rows),mutation.id).toBe(verdicts[index]!.holds);
  if(!["symlink-file","symlink-directory","directory"].includes(mutation.op))expect(JSON.stringify(local(root,programOf(root).files))!==JSON.stringify(local(root,program.files)),mutation.id+" real compiler program").toBe(verdicts[index]!.programChanges);
 }
});

test("a program file that is not a physical file fails the closure closed",async()=>{
 for(const op of["symlink-file","symlink-directory"] as const){
  const root=workspace("closed-"+op);mutate(root,{id:"closed",op,path:op==="symlink-file"?"src/util.ts":"src",holds:false,programChanges:false} as Mutation);
  let failure:unknown;try{await observeClosure(root,[]);}catch(error){failure=error;}
  expect(String(failure),op).toContain("Build inference requires real authority");
 }
});
