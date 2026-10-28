/** ⚖️ Observable aggregate result kinds and multiplicities come from both actual operands. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import cases from "./aggregate-kind-cases.json";
import schema from "./aggregate-kind.schema.json";
import {compileIfc4x3Express,ExpressEvaluator,parseExpressExpression} from "./index.ts";
import {ExpressParser} from "./parser.ts";

test("aggregate operand classification vectors have a closed schema and no native admission claim",()=>{
 expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true);
 expect(cases.nativeExecution).toBe(false);expect(cases.admission).toBe(false);
});
const program=compileIfc4x3Express().then(compiled=>({...compiled.program,functions:cases.functions.map(row=>{const parser=new ExpressParser(row.source);parser.require("FUNCTION");parser.require(row.name);return parser.function(row.name);})}));
for(const row of cases.cases)test("aggregate kind "+row.id,async()=>{
 const result=await new ExpressEvaluator(await program).evaluate(parseExpressExpression(row.expression));
 expect(result.kind).toBe("aggregate");expect(String(result.aggregate)).toBe(row.aggregate);expect((result.value as {value:string}[]).map(row=>row.value).sort()).toEqual(row.values);
});
