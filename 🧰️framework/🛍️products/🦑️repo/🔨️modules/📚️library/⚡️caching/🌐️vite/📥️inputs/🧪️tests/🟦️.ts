import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readViteArtifactInputsV1} from "../🟦️.ts";
import storageSchema from "../../💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import requestSchema from "../../../📦️artifacts/👓️reading/🧬️schema/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import groupSchema from "../../../📦️artifacts/👓️reading/📚️group/🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};

test("neutral Vite build and service retain explicit artifact inputs through their actual work",()=>{
 const path=resolve(import.meta.dir,"../../🟦️.ts"),syntax=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),entries=new Map<string,{inputs:boolean;calls:string[]}>();
 const visit=(node:ts.Node):void=>{if(ts.isFunctionDeclaration(node)&&node.name&&fixture.entryPoints.includes(node.name.text)){const calls:string[]=[],walk=(row:ts.Node):void=>{if(ts.isCallExpression(row))calls.push(row.expression.getText(syntax));ts.forEachChild(row,walk);};walk(node);entries.set(node.name.text,{inputs:node.getText(syntax).includes("options."+fixture.property),calls});}ts.forEachChild(node,visit);};visit(syntax);
 expect(new Ajv({strict:true}).validate({const:{version:fixture.version,admission:fixture.admission}},schema.definitions.Protocol.const)).toBe(true);
 for(const entry of fixture.entryPoints){expect(entries.get(entry)?.inputs).toBe(true);expect(entries.get(entry)?.calls).toContain(fixture.requiredCall);expect(entries.get(entry)?.calls).toContain(fixture.intentCall);}
 console.log("[DEBUG] Independent TypeScript AST identifies explicit input retention in both actual Vite entry points");
});

test("neutral Vite input list has a finite copied request inventory and distinct canonical paths",()=>{
 const oracle=new Ajv({strict:true}).addSchema(groupSchema,schema.anyOf[1]!.$ref!),database=new Database(":memory:"),request=(index:number)=>({version:1 as const,path:resolve(process.cwd(),"fixture-"+index),owner:"fixture:vite-input",maximumBytes:1024,maximumFiles:1});
 try{database.exec("CREATE TABLE inputs(path TEXT PRIMARY KEY)");for(const count of fixture.validCounts){const supplied=Array.from({length:count},(_,index)=>request(count-index)),actual=readViteArtifactInputsV1(supplied);expect(oracle.validate(schema,supplied)).toBe(true);database.exec("DELETE FROM inputs");for(const row of supplied){expect(oracle.validate(requestSchema,row)).toBe(true);database.query("INSERT INTO inputs VALUES(?)").run(row.path);}expect(actual.map(row=>row.path)).toEqual((database.query("SELECT path FROM inputs ORDER BY path").all()as {path:string}[]).map(row=>row.path));if(supplied.length){const original=actual[0]!.path;supplied.forEach(row=>{row.path=resolve(process.cwd(),"changed");});expect(actual[0]!.path).toBe(original);}}
 for(const count of fixture.invalidCounts){const rows=Array.from({length:count},(_,index)=>request(index));expect(oracle.validate(schema,rows)).toBe(false);expect(()=>readViteArtifactInputsV1(rows)).toThrow("Finite");}for(const patch of fixture.invalidRequests)expect(()=>readViteArtifactInputsV1([{...request(0),...patch}])).toThrow();expect(()=>readViteArtifactInputsV1([request(0),request(0)])).toThrow("Distinct");console.log("[DEBUG] Ajv and independent SQLite inventory agree with finite copied Vite input DATA; invalid and duplicate requests refuse");}finally{database.close();}
});


test("neutral Vite service retains accepted capacity waiters before cache disposal",()=>{
 const path=resolve(import.meta.dir,"../../🟦️.ts"),syntax=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),calls:string[]=[];const visit=(node:ts.Node):void=>{if(ts.isFunctionDeclaration(node)&&node.name?.text==="serveVite"){const walk=(row:ts.Node):void=>{if(ts.isCallExpression(row))calls.push(row.expression.getText(syntax));ts.forEachChild(row,walk);};walk(node);}else ts.forEachChild(node,visit);};visit(syntax);expect(new Ajv({strict:true}).validate({const:fixture.maximumPendingTransforms},storageSchema.definitions.Protocol.const.maximumPendingTransforms)).toBe(true);for(const name of fixture.serviceCalls)expect(calls).toContain(name);console.log("[DEBUG] Independent AST locates retained accepted transform promises across capacity waits and shutdown");
});


test("neutral Vite full declared products use the genuine bounded group rather than recursive reader frames",()=>{
 const rows=Array.from({length:9},(_,index)=>({version:1,path:resolve(process.cwd(),"fixture-group-"+index),owner:"fixture:vite-group",maximumBytes:1024,maximumFiles:1}));expect(readViteArtifactInputsV1(rows)).toHaveLength(9);const path=resolve(import.meta.dir,"../🟦️.ts"),source=readFileSync(path,"utf8"),tree=ts.createSourceFile(path,source,ts.ScriptTarget.Latest,true),calls=new Set<string>();const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node))calls.add(node.expression.getText(tree));ts.forEachChild(node,visit);};visit(tree);for(const name of["withRepositoryArtifactGroupV1","withRepositoryArtifactGroupTaskV1","readRepositoryArtifactGroupRequestsV1"])expect(calls.has(name)).toBe(true);expect(calls.has("withRepositoryArtifactV1")).toBe(false);expect(source).not.toContain("visit(index+1");
});
