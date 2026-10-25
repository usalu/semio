import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import ts from "typescript";
import fixture from "./🔣️.json";

const path=resolve(import.meta.dir,"../../🟦️.ts"),file=ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);
const callsIn=(owner:string,callee:string):number[]=>{
 const positions:number[]=[];
 const visit=(node:ts.Node,inside:boolean):void=>{
  const here=inside||((ts.isFunctionDeclaration(node)||ts.isVariableDeclaration(node))&&node.name!==undefined&&node.name.getText(file)===owner);
  if(here&&ts.isCallExpression(node)&&node.expression.getText(file)===callee)positions.push(node.getStart(file));
  ts.forEachChild(node,child=>visit(child,here));
 };
 visit(file,false);return positions.sort((a,b)=>a-b);
};

test("compiler capacity is taken once and first, so a child under the held marker never acquires it",()=>{
 for(const row of fixture.calls)expect(callsIn(row.owner,row.callee).length,row.id).toBe(row.calls);
 for(const row of fixture.order){
  const first=callsIn(row.owner,row.first)[0];
  expect(first,row.id).toBeDefined();
  for(const later of row.then)for(const position of callsIn(row.owner,later))expect(position,row.id+": "+later).toBeGreaterThan(first!);
 }
});
