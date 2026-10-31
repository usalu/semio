import {expect,test} from "vitest";
import {readFileSync} from "node:fs";
import ts from "typescript";
import Ajv from "ajv";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
const source=ts.createSourceFile("shell-commands.ts",readFileSync(new URL("../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
const constants=new Map<string,string>();
for(const statement of source.statements)if(ts.isVariableStatement(statement))for(const row of statement.declarationList.declarations)if(ts.isIdentifier(row.name)&&row.initializer&&ts.isStringLiteral(row.initializer))constants.set(row.name.text,row.initializer.text);

/** 📋️ Validates the portable request contract against the independent JSON Schema engine. */
test("shell exporter request corpus is closed",()=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const manifest=readFileSync(new URL("../../../📦️packages/🦀️rust/Cargo.toml",import.meta.url),"utf8");
 expect(manifest).not.toMatch(/^\[features\]/mu);
});
for(const row of fixture.cases)test(row.method+" "+row.mode+" uses the declared zero-feature exporter",()=>{
 const method=source.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text===row.method);
 if(!method)throw Error("Actual shell command required");
 const calls:ts.CallExpression[]=[];
 const visit=(node:ts.Node):void=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text===row.callee)calls.push(node);ts.forEachChild(node,visit);};
 visit(method);expect(calls).toHaveLength(1);
 const array=calls[0]!.arguments[row.argument];if(!array||!ts.isArrayLiteralExpression(array))throw Error("Actual closed Cargo request required");
 const value=(node:ts.Expression):string=>{
  if(ts.isStringLiteral(node))return node.text;
  if(ts.isIdentifier(node)){if(node.text==="mode")return row.mode;const found=constants.get(node.text);if(found!==undefined)return found;}
  if(ts.isBinaryExpression(node)&&node.operatorToken.kind===ts.SyntaxKind.PlusToken)return value(node.left)+value(node.right);
  throw Error("Unrecognized Cargo request expression");
 };
 const actual=array.elements.map(value);expect(actual).toEqual(row.expected);
 expect(new Ajv({strict:true}).validate({const:row.expected},actual)).toBe(true);
});
