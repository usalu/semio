/** 🧪 Native fixture source parity does not claim unverified Rust execution. */
import {test,expect} from "bun:test";
import {readFile} from "node:fs/promises";
import {createHash} from "node:crypto";
import Ajv from "ajv";
import schema from "./ast.schema.json";
import cases from "./cases.json";
import statements from "./statements.json";
import constructors from "./constructor-cases.json";
import {ExpressParser,parseExpressExpression} from "./parser.ts";
import {ExpressEvaluator} from "./evaluator.ts";
import type {ExpressValue} from "./evaluator.ts";
const digest=(value:unknown)=>createHash("sha256").update(JSON.stringify(value)).digest("hex");
test("native scalar fixtures retain current schema-admitted language-neutral AST",async()=>{const rows=cases.map(row=>({...row,ast:parseExpressExpression(row.expression)})),admit=new Ajv({strict:true}).compile({...schema,$ref:"#/$defs/Expression"}),source=await readFile(new URL("expression-cases.rs",import.meta.url),"utf8");expect(source.match(/EXPRESSION_CASES_SHA256\s*:\s*&str\s*=\s*"([a-f0-9]{64})"/)?.[1]).toBe(digest(rows));for(const row of rows){expect(admit(row.ast)).toBe(true);const value=await new ExpressEvaluator().evaluate(row.ast);expect(String(value.kind)).toBe(row.kind);expect(String(value.value)).toBe(row.value);}});
test("native statement fixtures retain current schema-admitted flow and shared expected values",async()=>{const rows=statements.map(row=>({...row,ast:new ExpressParser(row.source).function(row.name)})),admit=new Ajv({strict:true}).compile({...schema,$ref:"#/$defs/Function"}),source=await readFile(new URL("statement-cases.rs",import.meta.url),"utf8");expect(source.match(/STATEMENT_CASES_SHA256\s*:\s*&str\s*=\s*"([a-f0-9]{64})"/)?.[1]).toBe(digest(rows));for(const row of rows){expect(admit(row.ast)).toBe(true);const program={schema:"IFC4X3_ADD2" as const,functions:[row.ast],globalRules:[],rules:[],derived:[],unique:[],bounds:[],inverse:[]},value=await new ExpressEvaluator(program).call(row.name,row.arguments.map(value=>({kind:"integer",value:BigInt(value)})));expect(value.kind).toBe("integer");expect(String(value.value)).toBe(row.expected);}});
test("native constructor type fixtures agree with independent ancestry and SELECT closure",async()=>{for(const row of constructors){const evaluator=new ExpressEvaluator(),argument:ExpressValue=row.constructor==="IFCCARTESIANPOINT"?{kind:"aggregate",value:[{kind:"real",value:0},{kind:"real",value:0},{kind:"real",value:0}]}:{kind:"real",value:1},value=await evaluator.call(row.constructor,[argument]),types=await evaluator.call("TYPEOF",[value]);expect((types.value as ExpressValue[]).map(item=>String(item.value)).filter(name=>name.startsWith("IFC4X3_ADD2.")).sort()).toEqual([...row.schemaTypes].sort());}});
