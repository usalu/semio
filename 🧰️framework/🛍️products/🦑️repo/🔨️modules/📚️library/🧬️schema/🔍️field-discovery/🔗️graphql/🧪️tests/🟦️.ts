/** 🔗️ Contract: the GraphQL field extractor reads the fields of a named object type (non-null as required, list and `…Entry` list as list and map, `@state(class: …)` as state), ignores comments, strings and argument lists and recognises a braceless empty type; the corpus is language-agnostic and graphql-js is the independent third-party oracle. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {Kind,parse,type NamedTypeNode,type ObjectTypeDefinitionNode} from "graphql";
import {policyExtractGraphqlSchemaFields} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const scalarOf=(name:string):string=>{const table:Record<string,string>={String:"string",Int:"int32",Float:"float64",Boolean:"bool"};return Object.hasOwn(table,name)?table[name]!:name;};
const oracle=(text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const definition=parse(text).definitions.find((node):node is ObjectTypeDefinitionNode=>node.kind===Kind.OBJECT_TYPE_DEFINITION&&node.name.value===name);if(definition===undefined)return{typeName:"",fields:[]};
 return{typeName:name,fields:(definition.fields??[]).map(field=>{
  let type=field.type,optional=true,cardinality="scalar",scalar="";
  if(type.kind===Kind.NON_NULL_TYPE){optional=false;type=type.type;}
  if(type.kind===Kind.LIST_TYPE){let inner=type.type;if(inner.kind===Kind.NON_NULL_TYPE)inner=inner.type;const named=(inner as NamedTypeNode).name.value;if(named.endsWith("Entry")){cardinality="map";scalar=scalarOf(named.slice(0,-"Entry".length));}else{cardinality="list";scalar=scalarOf(named);}}
  else scalar=scalarOf((type as NamedTypeNode).name.value);
  const directive=field.directives?.find(item=>item.name.value==="state"),argument=directive?.arguments?.find(item=>item.name.value==="class"),state=argument!==undefined&&argument.value.kind===Kind.ENUM?argument.value.value.toLowerCase().replace(/_/gu,"-"):"";
  return{name:field.name.value,optional,cardinality,scalar,state};
 })};
};
for(const row of corpus.extract)test("graphql extract: "+row.name,()=>{expect<unknown>(policyExtractGraphqlSchemaFields(row.text,row.type)).toEqual(row.expected);});
for(const row of corpus.defects)test.failing("graphql defect: "+row.name,()=>{expect(policyExtractGraphqlSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("graphql-js parses the same fields as the extractor",()=>{for(const row of corpus.extract.filter(item=>item.oracle))expect(oracle(row.text,row.type!),row.name).toEqual(row.expected);});
test("the documents the corpus extracts from are valid GraphQL",()=>{for(const row of [...corpus.extract,...corpus.defects])expect(()=>parse(row.text),row.name).not.toThrow();});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
