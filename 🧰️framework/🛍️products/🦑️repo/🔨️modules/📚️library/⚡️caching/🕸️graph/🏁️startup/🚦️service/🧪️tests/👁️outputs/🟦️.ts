import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "./🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import ts from "typescript";

test("closed daemon observation expectations keep watched scope below unrelated generated directories",()=>{
 const validate=new Ajv({strict:false}).compile(schema),db=new Database(":memory:");
 try{expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);const row=db.query("SELECT ?1 > ?2 AND ?1 > ?3 bounded").get(fixture.unrelatedDirectories,fixture.maximumInitialWatches,fixture.maximumRegisteredWatches) as {bounded:number};expect(row.bounded).toBe(1);expect(fixture.initialEnvironment).not.toBe(fixture.changedEnvironment);}
 finally{db.close();}
});


test("public infrastructure dispatch selects declared file and code suites and shares the actual Original",async()=>{
 const library=join(import.meta.dir,"../../../../../.."),source=readFileSync(join(library,"🎛️commands/🧪️test/🟦️.ts"),"utf8"),table=JSON.parse(readFileSync(join(library,"⚡️caching/🗂️suites/🔣️.json"),"utf8")),ast=ts.createSourceFile("build.ts",source,ts.ScriptTarget.Latest,true),owner=ast.statements.find(row=>ts.isClassDeclaration(row)&&row.name?.text==="TestScript") as ts.ClassDeclaration,run=owner.members.find(row=>ts.isMethodDeclaration(row)&&row.name.getText(ast)==="run") as ts.MethodDeclaration,branch=run.body!.statements.find(row=>ts.isIfStatement(row)&&row.expression.getText(ast).includes('rest[0]==="cache"'));
 if(!branch)throw Error("Actual public infrastructure branch required");
 const {selectCanonicalCacheTestV1}=await import("../../../../../🗂️suites/🧮️aggregate/🟦️.ts"),code=ts.transpile("async function execute(rest){"+branch.getText(ast)+"}",{target:ts.ScriptTarget.ESNext}).replace('import("../../⚡️caching/📜️script.ts")',"Promise.resolve(ports.script)"),db=new Database(":memory:");
 try{for(const row of fixture.routes){
  const original={},calls:unknown[][]=[],ports={script:{CacheTestScript:class{constructor(...args:unknown[]){this.original=args[2];}original:unknown;async run(args:string[]){selectCanonicalCacheTestV1(args,table);calls.push([this.original,args]);}}}},execute=new Function("ports","join",code+";return execute;")(ports,join),oracle=db.query("SELECT ?1=2 AND EXISTS(SELECT 1 FROM json_each(?2) WHERE value=?3) accepted").get(row.arguments.length,JSON.stringify([...table.suites.map((suite:{id:string})=>suite.id),...table.codeSuites,"contracts"]),row.arguments[1]??null) as {accepted:number};
  expect(Boolean(oracle.accepted)).toBe(row.accepted);const pending=execute.call({root:"/repo",invocation:original},row.arguments);
  if(row.accepted){await pending;expect(calls[0]![0]).toBe(original);expect(calls[0]![1]).toEqual([row.arguments[1]]);}else{await expect(pending).rejects.toThrow();expect(calls).toEqual([]);}
 }}finally{db.close();}
});
