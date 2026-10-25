/** 🔏️ Contract: the static import specifiers of a module are exactly the ones the compiler API reports for its import and export-from declarations, never type-only forms, calls, strings or comments; the authored cases live in `./🔣️.json`. */
import {expect,test} from "bun:test";
import ts from "typescript";
import {staticImportSpecifiersV1} from "../../🟦️.ts";
import corpus from "./🔣️.json";

const compiler=(text:string):string[]=>{
 const file=ts.createSourceFile("case.ts",text,ts.ScriptTarget.Latest,true),found:string[]=[];
 for(const node of file.statements){
  if(ts.isImportDeclaration(node)&&!node.importClause?.isTypeOnly&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);
  if(ts.isExportDeclaration(node)&&!node.isTypeOnly&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);
 }
 return found;
};

test("every authored case names exactly its static specifiers, as the compiler API does",()=>{
 expect(corpus.version).toBe(1);
 for(const row of corpus.cases){
  expect([...staticImportSpecifiersV1(row.text)].sort(),row.name).toEqual([...row.expected].sort());
  expect([...compiler(row.text)].sort(),row.name).toEqual([...row.expected].sort());
 }
});
