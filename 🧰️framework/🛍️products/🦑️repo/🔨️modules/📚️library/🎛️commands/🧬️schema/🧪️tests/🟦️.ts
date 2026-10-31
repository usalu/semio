import reportNameSchema from "../../../🏗️build/📊️reporting/🧬️schema/🔣️.json";
import ts from "typescript";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import routes from "./🧫️fixtures/📬️route/🔣️.json";
import { expect, test } from "bun:test";
import Ajv from "ajv";
import cases from "./🧫️fixtures/🔣️.json";
import modeSchema from "../../../../../../../🔨️modules/🏃️process/🏗️build/🎚️mode/🧬️schema/🔣️.json";
import { schemaCompilerEnvironmentV1 } from "../🎚️mode/🟦️.ts";
const oracle = new Ajv({ strict: false }).compile(modeSchema);
for (const row of cases.cases) test(row.id, () => {
  const environment = { ...row.environment, ORIGINAL: "retained-original", STORE: "retained-store" }, before = JSON.stringify(environment);
  const selected = ("selected" in row ? row.selected : undefined) ?? environment.SEMIO_BUILD_MODE ?? "prototype";
  expect(oracle(selected), row.id).toBe(row.expected !== null);
  if (row.expected === null) expect(() => schemaCompilerEnvironmentV1(environment, "selected" in row ? row.selected : undefined)).toThrow("Build mode must be prototype or release");
  else {
    const actual = schemaCompilerEnvironmentV1(environment, "selected" in row ? row.selected : undefined);
    expect(actual).toEqual({ ...environment, SEMIO_BUILD_MODE: row.expected });
  }
  expect(JSON.stringify(environment)).toBe(before);
});

interface RouteCase {readonly id:string;readonly kind:string;readonly callee?:string;readonly index?:number;readonly expected?:string}
const validateRoutes=new Ajv({strict:true}).compile<{cases:RouteCase[]}>({type:"object",additionalProperties:false,required:["cases"],properties:{cases:{type:"array",minItems:14,maxItems:14,items:{type:"object",additionalProperties:false,required:["id","kind"],properties:{id:{type:"string",pattern:"^[a-z-]+$"},kind:{enum:["async","argument","call","cancel","default-report","harness-guard","cancellation-refusal","parser-scope","parser-default"]},callee:{type:"string"},index:{type:"integer",minimum:0},expected:{type:"string"}}}}}});
const routeInput:unknown=routes;if(!validateRoutes(routeInput))throw Error(JSON.stringify(validateRoutes.errors));
const commandSource = ts.createSourceFile("schema.ts", readFileSync(new URL("../🟦️.ts", import.meta.url), "utf8"), ts.ScriptTarget.Latest, true);
const schemaClass = commandSource.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "SchemaScript") as ts.ClassDeclaration;
const compileMethod = schemaClass.members.find(node => ts.isMethodDeclaration(node) && node.name?.getText(commandSource) === "compile") as ts.MethodDeclaration;
const calls: ts.CallExpression[] = [];
const visit = (node: ts.Node): void => { if (ts.isCallExpression(node)) calls.push(node); ts.forEachChild(node, visit); };
visit(compileMethod);
let defaultReport:string|undefined;const findDefault=(node:ts.Node):void=>{if(ts.isVariableDeclaration(node)&&node.name.getText(commandSource)==="name"&&node.initializer&&ts.isConditionalExpression(node.initializer)&&ts.isStringLiteral(node.initializer.whenFalse))defaultReport=node.initializer.whenFalse.text;ts.forEachChild(node,findDefault)};findDefault(compileMethod);
const testMethod = schemaClass.members.find(node => ts.isMethodDeclaration(node) && node.name?.getText(commandSource) === "test") as ts.MethodDeclaration;
const testCalls: ts.CallExpression[] = [];
const scanTest = (node: ts.Node): void => { if (ts.isCallExpression(node)) testCalls.push(node); ts.forEachChild(node, scanTest); };
scanTest(testMethod);
const enclosingTry = (node: ts.Node | undefined): ts.TryStatement | undefined => { for (let current = node?.parent; current && current !== testMethod; current = current.parent) if (ts.isTryStatement(current)) return current; return undefined; };
const harnessTry = enclosingTry(testCalls.find(call => call.expression.getText(commandSource) === "runCmdStatus"));
const oracleTry = enclosingTry(testCalls.find(call => call.expression.getText(commandSource) === "this.oracle"));
const cancellationRefusal = harnessTry?.catchClause?.block.statements.find(statement => ts.isIfStatement(statement) && statement.expression.getText(commandSource) === "this.invocation.control.signal.aborted" && ts.isThrowStatement(statement.thenStatement));
const parsersMethod = schemaClass.members.find(node => ts.isMethodDeclaration(node) && node.name?.getText(commandSource) === "parsers") as ts.MethodDeclaration;
let parserScopes: ts.Expression | undefined;
const scanParsers = (node: ts.Node): void => {
 if (ts.isCallExpression(node) && node.expression.getText(commandSource) === "checkParsers" && node.arguments[0] && ts.isObjectLiteralExpression(node.arguments[0])) {
  const property = node.arguments[0].properties.find(property => ts.isPropertyAssignment(property) && property.name.getText(commandSource) === "scopes");
  if (property && ts.isPropertyAssignment(property)) parserScopes = property.initializer;
 }
 ts.forEachChild(node, scanParsers);
};
scanParsers(parsersMethod);
const validReportName=new Ajv({strict:true}).compile(reportNameSchema);
for (const row of routeInput.cases) test("schema command route "+row.id,()=>{
 const db=new Database(":memory:");
 try{
  db.exec("CREATE TABLE observed(id TEXT PRIMARY KEY, valid INTEGER NOT NULL)");
  const call="callee" in row?calls.find(call=>call.expression.getText(commandSource)===row.callee):undefined;
  const actual=row.kind==="parser-scope"?!!parserScopes&&ts.isConditionalExpression(parserScopes)&&parserScopes.condition.getText(commandSource)==='args.includes("--under")'&&parserScopes.whenTrue.getText(commandSource)===row.expected:row.kind==="parser-default"?!!parserScopes&&ts.isConditionalExpression(parserScopes)&&parserScopes.whenFalse.getText(commandSource)===row.expected:row.kind==="harness-guard"?!!harnessTry?.catchClause&&!!oracleTry&&testMethod.body!.statements.indexOf(harnessTry)<testMethod.body!.statements.indexOf(oracleTry):row.kind==="cancellation-refusal"?!!cancellationRefusal:row.kind==="default-report"?validReportName(defaultReport):row.kind==="async"?!!compileMethod.modifiers?.some(modifier=>modifier.kind===ts.SyntaxKind.AsyncKeyword):row.kind==="argument"?call?.arguments[row.index!]?.getText(commandSource)===row.expected:row.kind==="cancel"?!!call&&call.arguments.some(argument=>ts.isObjectLiteralExpression(argument)&&argument.properties.some(property=>ts.isPropertyAssignment(property)&&property.name.getText(commandSource)==="signal"&&property.initializer.getText(commandSource)===row.expected)):!!call;
  db.query("INSERT INTO observed VALUES(?,?)").run(row.id,Number(actual));
  expect(db.query("SELECT id FROM observed WHERE valid<>1").all()).toEqual([]);
 }finally{db.close()}
});
