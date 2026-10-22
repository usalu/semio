/** 🦀️ Contract: the Rust field extractor reads the public fields of a named public struct (renames, state attributes, nested generics, arrays, maps, byte vectors) in declaration order and ignores private fields; the corpus is language-agnostic and the tree-sitter Rust grammar (web-tree-sitter) is the independent third-party oracle. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {policyExtractRustSchemaFields,policyParseRustFieldType} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const snake=(name:string):string=>{const parts=name.split("_");return parts[0]+parts.slice(1).map(part=>part.charAt(0).toUpperCase()+part.slice(1)).join("");};
const scalarOf=(text:string):string=>{const token=text.replace(/\s+/gu,""),table:Record<string,string>={String:"string",bool:"bool",i32:"int32",u32:"uint32",i64:"int64",u64:"uint64",f32:"float32",f64:"float64","Vec<u8>":"bytes"};return Object.hasOwn(table,token)?table[token]!:token;};
const classify=(node:Parser.SyntaxNode):{optional:boolean;cardinality:string;scalar:string}=>{
 let optional=false,type=node;const generic=(candidate:Parser.SyntaxNode,name:string)=>candidate.type==="generic_type"&&candidate.childForFieldName("type")!.text===name;const args=(candidate:Parser.SyntaxNode)=>candidate.childForFieldName("type_arguments")!.namedChildren;
 if(generic(type,"Option")){optional=true;type=args(type)[0]!;}
 if(generic(type,"BTreeMap")||generic(type,"HashMap")){const parts=args(type);if(parts[0]!.text==="String")return{optional,cardinality:"map",scalar:scalarOf(parts[1]!.text)};}
 if(type.type==="array_type")return{optional,cardinality:"fixedList",scalar:scalarOf(type.childForFieldName("element")!.text)};
 if(generic(type,"Vec")){const inner=args(type)[0]!;return inner.text==="u8"?{optional,cardinality:"scalar",scalar:"bytes"}:{optional,cardinality:"list",scalar:scalarOf(inner.text)};}
 return{optional,cardinality:"scalar",scalar:scalarOf(type.text)};
};
const treeFields=(parser:Parser,text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const root=parser.parse(text).rootNode,item=root.namedChildren.find(node=>node.type==="struct_item"&&node.childForFieldName("name")!.text===name&&node.namedChildren.some(child=>child.type==="visibility_modifier"));
 if(item===undefined)return{typeName:"",fields:[]};
 const fields:unknown[]=[];let attributes:string[]=[];
 for(const member of item.childForFieldName("body")!.namedChildren){
  if(member.type==="attribute_item"){attributes.push(member.text);continue;}
  if(member.type!=="field_declaration"){continue;}
  if(!member.namedChildren.some(child=>child.type==="visibility_modifier")){attributes=[];continue;}
  const joined=attributes.join("\n"),state=/#\[state\(([^)]*)\)\]/u.exec(joined)?.[1]?.trim()??"",renamed=/rename\s*=\s*"([^"]*)"/u.exec(joined)?.[1],shape=classify(member.childForFieldName("type")!);
  fields.push({name:renamed??snake(member.childForFieldName("name")!.text),optional:shape.optional,cardinality:shape.cardinality,scalar:shape.scalar,state:state.toLowerCase().replace(/_/gu,"-")});attributes=[];
 }
 return{typeName:name,fields};
};
for(const row of corpus.types)test("rust field type "+JSON.stringify(row.text),()=>{expect<unknown>(policyParseRustFieldType(row.text)).toEqual({optional:row.optional,cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("rust extract: "+row.name,()=>{expect<unknown>(policyExtractRustSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("tree-sitter's Rust grammar yields the same fields as the extractor",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 for(const row of corpus.extract.filter(item=>item.oracle))expect(treeFields(parser,row.text,row.type!),row.name).toEqual(row.expected);
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
