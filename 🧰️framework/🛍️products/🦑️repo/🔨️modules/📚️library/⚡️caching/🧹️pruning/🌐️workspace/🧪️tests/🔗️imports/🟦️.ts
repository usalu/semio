/** 🔗️ Production retention reaches only canonical symbol owners, checked with TypeScript resolution and SQLite set equality. */
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {dirname,resolve} from "node:path";
import ts from "typescript";
import Ajv from "ajv";
import corpus from "./🧫️cases/🔣️.json";
import schema from "./🧬️schema/🔣️.json";

const subject=resolve(import.meta.dir,corpus.subject);
const read=(path:string)=>ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);

test("production cache retention imports only canonical evidence owners",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(schema,{...corpus,extra:true})).toBe(false);
 expect(ajv.validate(schema,{...corpus,routes:[...corpus.routes,corpus.routes[0]]})).toBe(false);
 const file=read(subject),owner=file.statements.find((node)=>ts.isFunctionDeclaration(node)&&node.name?.text==="pruneTestEvidence");
 expect(owner&&ts.isFunctionDeclaration(owner)&&owner.body).toBeTruthy();
 const actual:{symbol:string;owner:string}[]=[],imports:string[]=[];
 const visit=(node:ts.Node):void=>{
  if(ts.isCallExpression(node)&&node.expression.kind===ts.SyntaxKind.ImportKeyword){expect(node.arguments).toHaveLength(1);expect(ts.isStringLiteral(node.arguments[0]!)).toBe(true);imports.push((node.arguments[0] as ts.StringLiteral).text);}
  if(ts.isVariableDeclaration(node)&&ts.isArrayBindingPattern(node.name)&&node.initializer&&ts.isAwaitExpression(node.initializer)){
   const call=node.initializer.expression;
   expect(ts.isCallExpression(call)&&ts.isPropertyAccessExpression(call.expression)&&call.expression.getText(file)==="Promise.all").toBe(true);
   if(!ts.isCallExpression(call)||!ts.isArrayLiteralExpression(call.arguments[0]!))throw Error("retention imports must be joined as one bounded pair");
   for(const[index,binding]of node.name.elements.entries()){
    expect(ts.isBindingElement(binding)&&ts.isObjectBindingPattern(binding.name)).toBe(true);
    if(!ts.isBindingElement(binding)||!ts.isObjectBindingPattern(binding.name))throw Error("retention owner must bind named exports");
    const entry=call.arguments[0].elements[index];
    if(!entry||!ts.isCallExpression(entry)||entry.expression.kind!==ts.SyntaxKind.ImportKeyword||!ts.isStringLiteral(entry.arguments[0]!))throw Error("retention owner must be a literal module");
    for(const field of binding.name.elements){expect(ts.isIdentifier(field.name)).toBe(true);actual.push({symbol:(field.propertyName??field.name).getText(file),owner:entry.arguments[0].text});}
   }
  }
  ts.forEachChild(node,visit);
 };
 visit(owner!);
 expect(imports).not.toContain(corpus.forbidden);
 expect([...new Set(imports)].sort()).toEqual([...new Set(corpus.routes.map(row=>row.owner))].sort());
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE expected(symbol TEXT PRIMARY KEY,owner TEXT NOT NULL);CREATE TABLE actual(symbol TEXT PRIMARY KEY,owner TEXT NOT NULL)");
  for(const row of corpus.routes)db.query("INSERT INTO expected VALUES(?,?)").run(row.symbol,row.owner);
  for(const row of actual)db.query("INSERT INTO actual VALUES(?,?)").run(row.symbol,row.owner);
  expect(db.query("SELECT * FROM expected EXCEPT SELECT * FROM actual").all()).toEqual([]);
  expect(db.query("SELECT * FROM actual EXCEPT SELECT * FROM expected").all()).toEqual([]);
  expect(db.query("SELECT COUNT(DISTINCT owner) AS n FROM actual").get()).toEqual({n:2});
 }finally{db.close();}
 for(const row of corpus.routes){
  const resolved=ts.resolveModuleName(row.owner,subject,{moduleResolution:ts.ModuleResolutionKind.Bundler,allowImportingTsExtensions:true},ts.sys).resolvedModule;
  expect(resolved?.resolvedFileName).toBe(resolve(dirname(subject),row.owner));
  const declarations=read(resolved!.resolvedFileName).statements.filter((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===row.symbol);
  expect(declarations).toHaveLength(1);
  expect(declarations[0]!.modifiers?.some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword)).toBe(true);
 }
});
