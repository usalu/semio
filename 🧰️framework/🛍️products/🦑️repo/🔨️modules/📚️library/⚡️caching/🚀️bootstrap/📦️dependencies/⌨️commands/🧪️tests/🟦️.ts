import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {resolve,dirname} from "node:path";
import Ajv from "ajv";
import * as ts from "typescript";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️cases.json" with {type:"json"};

/** 🧩️ Keeps executable dependency commands independent of contract-test infrastructure. */
test("production dependency commands have isolated schema-owned routes",()=>{
 const owner=resolve(import.meta.dir,".."),sourcePath=resolve(owner,"🟦️.ts"),entryPath=resolve(owner,"../📜️script.ts"),testsPath=resolve(owner,"../🧪️tests/🔬️script-commands/🟦️.ts"),parse=(path:string)=>ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true),source=parse(sourcePath),entry=parse(entryPath),contract=parse(testsPath),classes=new Map<string,string>(),imports=new Map<string,string>(),routes=new Map<string,string>();
 expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);
 for(const statement of source.statements){
  if(ts.isImportDeclaration(statement)&&ts.isStringLiteral(statement.moduleSpecifier))expect(statement.moduleSpecifier.text).not.toContain("🧪️tests");
  if(!ts.isClassDeclaration(statement)||!statement.name||!ts.getModifiers(statement)?.some(item=>item.kind===ts.SyntaxKind.ExportKeyword))continue;
  const mode=statement.members.find(item=>ts.isPropertyDeclaration(item)&&item.name?.getText(source)==="mode") as ts.PropertyDeclaration|undefined;let value=mode?.initializer;while(value&&ts.isAsExpression(value))value=value.expression;
  expect(value&&ts.isStringLiteral(value)).toBe(true);classes.set(statement.name.text,(value as ts.StringLiteral).text);
 }
 for(const statement of entry.statements){if(!ts.isImportDeclaration(statement)||!ts.isStringLiteral(statement.moduleSpecifier)||!statement.importClause?.namedBindings||!ts.isNamedImports(statement.importClause.namedBindings))continue;for(const item of statement.importClause.namedBindings.elements)imports.set(item.name.text,resolve(dirname(entryPath),statement.moduleSpecifier.text));}
 const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isPropertyAccessExpression(node.expression)&&node.expression.name.text==="register"&&node.arguments.length===2&&ts.isStringLiteral(node.arguments[0]!)&&ts.isIdentifier(node.arguments[1]!))routes.set(node.arguments[0]!.text,node.arguments[1]!.text);ts.forEachChild(node,visit);};visit(entry);
 expect<unknown>([...classes].sort()).toEqual(schema.definitions.Commands.const.map(row=>[row.export,row.mode]).sort());
 expect(contract.statements.filter(ts.isClassDeclaration).map(row=>row.name?.text)).toEqual(["ContractCheckScript"]);
 expect(contract.statements.some(ts.isExportDeclaration)).toBe(false);
 const db=new Database(":memory:");try{for(const row of fixture.cases){const actual=routes.get(row.route)===row.export&&imports.get(row.export)===sourcePath&&classes.get(row.export)===row.mode,oracle=db.query("SELECT EXISTS(SELECT 1 FROM json_each(?) WHERE json_extract(value,'$.route')=? AND json_extract(value,'$.export')=? AND json_extract(value,'$.mode')=?) AS admitted").get(JSON.stringify(schema.definitions.Commands.const),row.route,row.export,row.mode) as {admitted:number};expect(actual,row.route+":"+row.export).toBe(row.production);expect(Boolean(oracle.admitted)).toBe(actual);}}finally{db.close();}
});
