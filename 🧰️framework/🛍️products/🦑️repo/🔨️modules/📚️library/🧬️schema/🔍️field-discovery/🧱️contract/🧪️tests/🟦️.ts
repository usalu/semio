/** 🧱️ Contract: the field-discovery contract normalises state classes, scalar spellings and snake-case names, and finds a schema declaration only by its declared name; the corpus is language-agnostic and `🐍️.py` recomputes each verdict. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {policyCanonicalScalar,policyCanonicalState,policyFindSchemaDeclaration,policySnakeToCamel} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const row of corpus.state)test("canonical state of "+JSON.stringify(row.raw),()=>{expect(policyCanonicalState(row.raw)).toBe(row.expected);});
for(const row of corpus.scalar)test("canonical scalar of "+JSON.stringify(row.raw),()=>{expect(policyCanonicalScalar(row.raw)).toBe(row.expected);});
for(const row of corpus.scalarDefects)test.failing("canonical scalar defect "+row.raw,()=>{expect(policyCanonicalScalar(row.raw)).toBe(row.expected);});
for(const row of corpus.snake)test("camel case of "+JSON.stringify(row.name),()=>{expect(policySnakeToCamel(row.name)).toBe(row.expected);});
for(const row of corpus.find)test("declaration lookup: "+row.name,()=>{expect(policyFindSchemaDeclaration(row.text,new RegExp(row.pattern,row.flags),row.expected)).toEqual(row.found);});
test("a lookup with a global pattern is repeatable",()=>{const pattern=/\bmessage\s+([A-Za-z_]\w*)\s*\{/g;expect(policyFindSchemaDeclaration("message M {",pattern,"M")).not.toBeNull();expect(policyFindSchemaDeclaration("message M {",pattern,"M")).not.toBeNull();});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
