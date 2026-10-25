/** 🔣️ Contract: the JSON Schema field extractor maps each property to a canonical scalar and cardinality, reads optionality from `required` and the state class from `x-semio-state`, and yields nothing for invalid JSON; the corpus is language-agnostic, Ajv is the third-party oracle for title, property order and the required set, and `🐍️.py` re-derives the shapes. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {policyExtractJsonSchemaFields,policyJsonSchemaScalar,policyParseJsonSchemaProperty} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
for(const[index,row]of corpus.scalar.entries())test("scalar of schema "+index+": "+JSON.stringify(row.schema),()=>{expect(policyJsonSchemaScalar(row.schema as Record<string,unknown>)).toBe(row.expected);});
for(const[index,row]of corpus.property.entries())test("property shape "+index+": "+JSON.stringify(row.schema),()=>{expect<unknown>(policyParseJsonSchemaProperty(row.schema as Record<string,unknown>)).toEqual({cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("extract: "+row.name,()=>{expect<unknown>(policyExtractJsonSchemaFields(row.text)).toEqual(row.expected);});
for(const row of corpus.extract.filter(item=>item.ajv))test("Ajv reads the same title and required set as the extractor: "+row.name,()=>{
 const ajv=new Ajv({strict:false,allErrors:true}),document=JSON.parse(row.text);expect(ajv.validateSchema(document)).toBe(true);
 const validate=ajv.compile(document);expect(validate({})).toBe(Object.values(row.expected.fields).some(field=>!field.optional)?false:true);
 const missing=(validate.errors??[]).filter(error=>error.keyword==="required").map(error=>(error.params as {missingProperty:string}).missingProperty).sort();
 expect(missing).toEqual(row.expected.fields.filter(field=>!field.optional).map(field=>field.name).sort());expect(document.title??"").toBe(row.expected.typeName);
 expect(Object.keys(document.properties??{})).toEqual(row.expected.fields.map(field=>field.name));
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
