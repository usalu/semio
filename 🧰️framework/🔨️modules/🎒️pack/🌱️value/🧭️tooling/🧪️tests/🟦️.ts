/** 🧭️ Keeps neutral fixture and native tooling independent of product implementations. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "../🧫️fixtures/🔣️.json";

test("neutral Pack tooling follows the same explicit boundary for computed imports and process commands",()=>{
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema),database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE boundary(owner TEXT NOT NULL,dependency TEXT NOT NULL,allowed INTEGER NOT NULL)");
  for(const row of fixture.cases){
   expect(validate(row)).toBe(true);
   database.query("INSERT INTO boundary VALUES(?,?,?)").run(row.owner,row.dependency,Number(row.allowed));
   expect(database.query("SELECT CASE WHEN ?='general' AND ?='product' THEN 0 ELSE 1 END AS allowed").get(row.owner,row.dependency)).toEqual({allowed:Number(row.allowed)});
   for(const key of Object.keys(row)){const missing={...row} as Record<string,unknown>;delete missing[key];expect(validate(missing)).toBe(false);}
   expect(validate({...row,hiddenDefault:true})).toBe(false);
  }
  expect(database.query("SELECT COUNT(*) AS count FROM boundary WHERE owner='general' AND dependency='product' AND allowed=1").get()).toEqual({count:0});
 }finally{database.close();}
 const owner=resolve(import.meta.dir,"../.."),sources=[resolve(owner,"📜️script.ts"),resolve(owner,"📦️packages/🦀️rust/📜️script.ts")];
 const violations:{source:string;line:number}[]=[];
 for(const file of sources){
  const source=ts.createSourceFile(file,readFileSync(file,"utf8"),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
  const visit=(node:ts.Node):void=>{if(ts.isStringLiteralLike(node)&&node.text.includes("🧰️framework/🛍️products/"))violations.push({source:file,line:source.getLineAndCharacterOfPosition(node.getStart(source)).line+1});ts.forEachChild(node,visit);};visit(source);
 }
 console.log(`[DEBUG] Pack tooling boundary: neutral-vectors=${fixture.cases.length} product-edges=${violations.length}`);
 expect(violations.length).toBe(0);
});
