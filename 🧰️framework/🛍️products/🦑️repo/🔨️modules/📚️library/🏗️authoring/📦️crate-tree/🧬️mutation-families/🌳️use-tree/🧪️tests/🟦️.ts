/** 🌳️ Contract: the use-tree module flattens a `use` statement into its full leaf paths (glob skipped, `self` naming its parent, aliases as leaves) and prunes named leaves out of groups of any nesting, collapsing single-leaf groups and answering null when nothing is left; the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the leaves and the validity of every pruned statement. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {flattenUse,pruneUseLeaves} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const segments=(node:Node):string[]=>{if(node.type==="scoped_identifier"){const path=node.childForFieldName("path"),name=node.childForFieldName("name")!;return[...(path?segments(path):[]),name.text];}return["identifier","crate","self","super"].includes(node.type)?[node.text]:[];};
const leaves=(statement:string,parser:Parser):{path:string[];leaf:string}[]=>{
 const declaration=parser.parse(statement).rootNode.namedChildren.find(node=>node.type==="use_declaration");if(declaration===undefined)return[];
 const out:{path:string[];leaf:string}[]=[];
 const walk=(node:Node,prefix:string[]):void=>{
  if(node.type==="scoped_use_list"){const path=node.childForFieldName("path"),list=node.childForFieldName("list")!;for(const child of list.namedChildren)walk(child,[...prefix,...(path?segments(path):[])]);return;}
  if(node.type==="use_list"){for(const child of node.namedChildren)walk(child,prefix);return;}
  if(node.type==="use_wildcard")return;
  const alias=node.type==="use_as_clause"?node.childForFieldName("alias")!.text:null,target=node.type==="use_as_clause"?node.childForFieldName("path")!:node,path=[...prefix,...segments(target)],clean=path.at(-1)==="self"?path.slice(0,-1):path;
  out.push({path:clean,leaf:alias??clean.at(-1)!});
 };
 walk(declaration.childForFieldName("argument")!,[]);return out;
};
for(const row of corpus.flatten)test("flatten: "+row.name,()=>{expect(flattenUse(row.statement)).toEqual(row.expected);});
test("tree-sitter flattens the same use leaves",async()=>{const parser=await parsed();for(const row of corpus.flatten.filter(item=>item.statement.includes("use ")&&item.statement.endsWith(";")&&!item.statement.endsWith("::;")))expect(leaves(row.statement,parser),row.name).toEqual(row.expected);});
for(const row of corpus.prune)test("prune: "+row.name,()=>{expect(pruneUseLeaves(row.statement,new Set(row.drop))).toBe(row.expected);});
test("pruned statements are valid Rust that bind exactly the surviving leaves (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const row of corpus.prune.filter(item=>item.statement.endsWith(";")&&item.statement.startsWith(item.statement.includes("pub")?"pub":"use"))){
  const result=pruneUseLeaves(row.statement,new Set(row.drop));if(result===null){expect(leaves(row.statement,parser).filter(item=>!row.drop.includes(item.leaf)),row.name).toEqual([]);continue;}
  expect(parser.parse(result).rootNode.hasError(),result).toBe(false);expect(leaves(result,parser),row.name).toEqual(leaves(row.statement,parser).filter(item=>!row.drop.includes(item.leaf)));
 }
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
