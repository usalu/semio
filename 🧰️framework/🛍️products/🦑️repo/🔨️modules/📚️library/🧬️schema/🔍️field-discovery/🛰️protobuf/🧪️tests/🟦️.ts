/** 🛰️ Contract: the Protobuf field extractor reads the fields of a named message (`optional`, `repeated`, `map<string,…>`, `// @state` comments) in declaration order; the corpus is language-agnostic and protobufjs is the independent third-party oracle for names, rules, map and optional flags and comments. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import protobuf from "protobufjs";
import {policyExtractProtobufSchemaFields,policyParseProtoFieldType} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const scalarOf=(name:string):string=>{const table:Record<string,string>={string:"string",bool:"bool",bytes:"bytes"};return Object.hasOwn(table,name)?table[name]!:name;};
const oracle=(text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const parsed=protobuf.parse(text,{alternateCommentMode:true}),type=parsed.root.lookupType((parsed.package?parsed.package+".":"")+name);
 return{typeName:name,fields:type.fieldsArray.map(field=>{
  const map=(field as unknown as {map:boolean}).map===true,state=/@state\s+([a-z0-9_-]+)/u.exec(field.comment??"")?.[1];
  return{name:field.name,optional:(field.options as {proto3_optional?:boolean}|undefined)?.proto3_optional===true,cardinality:map?"map":field.repeated?"list":"scalar",scalar:scalarOf(field.type),state:state?state.toLowerCase().replace(/_/gu,"-"):""};
 })};
};
for(const row of corpus.types)test("proto field type "+JSON.stringify(row.text),()=>{expect<unknown>(policyParseProtoFieldType(row.text,row.optional,row.repeated)).toEqual({optional:row.optional,cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("protobuf extract: "+row.name,()=>{expect<unknown>(policyExtractProtobufSchemaFields(row.text,row.type)).toEqual(row.expected);});
for(const row of corpus.defects)test.failing("protobuf defect: "+row.name,()=>{expect<unknown>(policyExtractProtobufSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("protobufjs parses the same fields as the extractor",()=>{for(const row of corpus.extract.filter(item=>item.oracle))expect(oracle(row.text,row.type!),row.name).toEqual(row.expected);});
test("protobufjs agrees on the nested message the extractor leaks",()=>{for(const row of corpus.defects)expect(oracle(row.text,row.type).fields).toEqual(row.expected.fields);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
