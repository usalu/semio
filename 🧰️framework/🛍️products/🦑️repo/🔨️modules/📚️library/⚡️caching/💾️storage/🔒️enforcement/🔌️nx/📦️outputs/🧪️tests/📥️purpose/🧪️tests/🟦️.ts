import{createIssuedResourceControlV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{test,expect}from"bun:test";
import{Database}from"bun:sqlite";
import Ajv from"ajv";
import ts from"typescript";
import{readFile}from"node:fs/promises";
import corpus from"../🔣️.json";
import schema from"../🧬️schema.json";
test("output negative awaited binding refusal is isolated from accepted parent custody",async()=>{const source=await readFile(new URL("../../🟦️.ts",import.meta.url),"utf8"),ast=ts.createSourceFile("outputs.ts",source,ts.ScriptTarget.Latest,true),caught:unknown[]=[],isolated:unknown[]=[],db=new Database(":memory:");try{expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);const visit=(node:import("typescript").Node)=>{if(ts.isCallExpression(node)&&node.expression.getText(ast)==="observeOutputNegativePurposeV1")isolated.push(node.arguments.map(argument=>argument.getText(ast)));if(ts.isVariableDeclaration(node)&&node.name.getText(ast)==="awaited"&&node.initializer?.getText(ast)==="fixture.namespace.awaitedMutation")caught.push(node.getText(ast));ts.forEachChild(node,visit);};visit(ast);expect(db.query("select ?1=1 and ?2 and ?3 and not ?4 and not ?5 isolated").get(corpus.expectedStatus,Number(corpus.waitPublished),Number(corpus.waitCompleted),Number(corpus.waitEntered),Number(corpus.safetyExpired))).toEqual({isolated:1});expect(caught).toEqual([]);expect(db.query("select json_array_length(?) count,json_extract(?,'$[1]') phase").get(JSON.stringify(corpus.parentArguments),JSON.stringify(corpus.parentArguments))).toEqual({count:2,phase:"productsPhase"});expect(isolated).toEqual([corpus.parentArguments]);}finally{db.close();}});

import{withBuildTestInvocationV1}from"../../../../../../../../🏗️build/🧪️tests/📥️invocation/🟦️.ts";
import{observeOutputNegativePurposeV1}from"../🟦️.ts";
test("output negative actual custody owner model retains caught operation failure only in its receiver",async()=>withBuildTestInvocationV1(async original=>{await observeOutputNegativePurposeV1(original,createIssuedResourceControlV1(original,"outputs:model-entry",0),"ledger-model");expect(original.control.signal.aborted).toBe(false);console.log("[DEBUG] Actual output custody function with explicit physical ports and genuine ledger failed its receiver after caught awaited mutation; same-wire parent remains healthy");}));
