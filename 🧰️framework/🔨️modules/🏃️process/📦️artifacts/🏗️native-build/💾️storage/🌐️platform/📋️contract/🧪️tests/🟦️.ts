import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import ts from "typescript";
import schema from "../../../🧬️schema/🔣️.json";
import corpus from "./🔣️.json";
import {completeNativeImageMaintenanceV1} from "../🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

test("closed maintenance results agree with independent Ajv and SQLite lifetime outcomes",()=>{
 const validate=new Ajv({strict:true}).compile(schema.definitions.ImageMaintenanceResult),db=new Database(":memory:");
 try{for(const row of corpus.outcomes){const value:unknown=row.value,object=JSON.stringify(value),oracle=db.query("SELECT json_type(?1)='object' AND ((json_extract(?1,'$.acquired')=0 AND json_extract(?1,'$.receiver')='released' AND (SELECT count(*) FROM json_each(?1))=2 AND NOT EXISTS(SELECT 1 FROM json_each(?1) WHERE key NOT IN ('acquired','receiver'))) OR (json_extract(?1,'$.acquired')=1 AND json_extract(?1,'$.receiver') IN ('released','retained') AND (SELECT count(*) FROM json_each(?1))=3 AND EXISTS(SELECT 1 FROM json_each(?1) WHERE key='value') AND NOT EXISTS(SELECT 1 FROM json_each(?1) WHERE key NOT IN ('acquired','receiver','value')))) AS accepted").get(object) as {accepted:number|null};expect(Boolean(oracle.accepted)).toBe(row.accepted);expect(validate(value)).toBe(row.accepted);expect(validateJsonSchemaSubset(schema.definitions.ImageMaintenanceResult,value,schema).length===0).toBe(row.accepted);}}
 finally{db.close();}
 console.log("[DEBUG] Closed maintenance receiver outcomes match Ajv and independent SQLite");
});

test("maintenance completion preserves original values and refuses cancellation across both awaits",async()=>{
 const db=new Database(":memory:");
 try{for(const row of corpus.callbacks){const abort=new AbortController(),failure=Error("original maintenance interruption"),value=Object.freeze({owner:Symbol("original")}),control={signal:abort.signal,remainingMilliseconds:()=>1000,publish:async()=>{},yieldContinuation:async()=>{}};let checks=0,calls=0;
  const recheck=async()=>{checks++;await Promise.resolve();if(checks===2&&row.interrupt==="postcheck")abort.abort(failure);},body=async()=>{calls++;await Promise.resolve();if(row.interrupt==="body")abort.abort(failure);if(row.interrupt==="failure")throw failure;return value;};
  const receiver=row.receiver==="retained"?"retained":"released",oracle=db.query("SELECT ?1='none' AS accepted").get(row.interrupt) as {accepted:number};expect(Boolean(oracle.accepted)).toBe(row.accepted);
  if(row.accepted){const result=await completeNativeImageMaintenanceV1(receiver,control,recheck,body);expect(result.acquired).toBe(true);expect(result.receiver).toBe(receiver);if(result.acquired)expect(result.value).toBe(value);expect(checks).toBe(2);}
  else await expect(completeNativeImageMaintenanceV1(receiver,control,recheck,body)).rejects.toBe(failure);
  expect(calls).toBe(1);
 }}finally{db.close();}
 console.log("[DEBUG] Actual maintenance completion preserves original callback value and original cancellation");
});

test("actual opened and provisioned backends bind completion to their original custody and release",()=>{
 const parse=(path:string)=>ts.createSourceFile(path,readFileSync(new URL(path,import.meta.url),"utf8"),ts.ScriptTarget.Latest,true),methods:ts.ArrowFunction[]=[];
 for(const path of ["../../../🍎️image/🟦️.ts","../../../🧱️fixed/🟦️.ts"]){const file=parse(path);const visit=(node:ts.Node)=>{if(ts.isPropertyAssignment(node)&&node.name.getText(file)==="withExclusiveMaintenance"&&ts.isArrowFunction(node.initializer))methods.push(node.initializer);ts.forEachChild(node,visit);};visit(file);}
 expect(methods.length).toBe(3);let retained=0,released=0;
 for(const method of methods){const calls:ts.CallExpression[]=[],finalizers:ts.Block[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node)&&node.expression.getText()==="completeNativeImageMaintenanceV1")calls.push(node);if(ts.isTryStatement(node)&&node.finallyBlock)finalizers.push(node.finallyBlock);ts.forEachChild(node,visit);};visit(method);expect(calls.length).toBe(1);const call=calls[0]!;expect(call.arguments[1]?.getText()).toBe("control");expect(call.arguments[2]?.getText()).toBe("recheck");expect(call.arguments[3]?.getText()).toBe("()=>body(port)");
  const kind=call.arguments[0]?.getText();if(kind==='"released"'){released++;expect(finalizers.some(block=>block.getText().includes("closed=true;exclusive.release();"))).toBe(true);expect(method.getText()).toContain('return{acquired:false,receiver:"released"}');}else{expect(kind).toBe('"retained"');retained++;}
 }
 expect([released,retained]).toEqual([2,1]);
 console.log("[DEBUG] TypeScript AST binds two opened releases and one retained provisioned maintenance result");
});
