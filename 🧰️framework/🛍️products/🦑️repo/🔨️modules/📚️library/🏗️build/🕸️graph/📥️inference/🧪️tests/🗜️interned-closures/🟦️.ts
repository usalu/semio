/** 🗜️ Contract: the import closure of a command script reaches a target only through an interned command-source group (one named input per project), never as a literal list repeated in every target; 1,900 literal paths in each of 12 targets of 2,140 crates were 5 GB of graph. */
import {expect,test} from "bun:test";
import {createRequire} from "node:module";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import {readInferenceSourceV1} from "../📖️source/🟦️.ts";

const require=createRequire(import.meta.url),ts=require("typescript"),Ajv=require("ajv");

/** 🔎️ Counts producer calls that flow into the sink outside the wrapper call. */
function violationsV1(source:string):number{
 const tree=ts.createSourceFile("source.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS),name=(call:any)=>ts.isIdentifier(call.expression)?call.expression.text:ts.isPropertyAccessExpression(call.expression)?call.expression.name.text:"";
 let count=0;
 const scan=(node:any,wrapped:boolean):void=>{
  const call=ts.isCallExpression(node)?node:undefined;
  if(call&&fixture.producers.includes(name(call))&&!wrapped)count++;
  const inside=wrapped||call!==undefined&&name(call)===fixture.wrapper;
  ts.forEachChild(node,(child:unknown)=>scan(child,inside));
 };
 const visit=(node:any):void=>{
  if(ts.isCallExpression(node)&&node.expression.getText(tree)===fixture.sink)for(const argument of node.arguments)scan(argument,false);
  else ts.forEachChild(node,visit);
 };
 visit(tree);
 return count;
}

test("interned closure fixture is closed",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);});

test("the checker finds literal closures pushed into target inputs and accepts interned ones",()=>{
 for(const row of fixture.cases)expect(violationsV1(row.source),row.id).toBe(row.violations);
});

test("the actual project assembly never pushes a command closure into a target as literals",()=>{
 expect(violationsV1(readInferenceSourceV1(fixture.component))).toBe(0);
});
