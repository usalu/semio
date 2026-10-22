/** 🧵️ Contract: the unmirror use-tree module scans every `use` statement of a source (module level and function local, attributes and visibility included, macro variables and malformed trees skipped), names the binding of a leaf and renders leaves back grouped by parent; the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the declarations, leaves and the validity of rendered statements. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {leafBinding,renderUseLeaves,scanSource,scanUseStatements} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const segments=(node:Node):string[]=>{if(node.type==="scoped_identifier"){const path=node.childForFieldName("path"),name=node.childForFieldName("name")!;return[...(path?segments(path):[]),name.text];}return["identifier","crate","self","super"].includes(node.type)?[node.text]:[];};
const leavesOf=(declaration:Node):{path:string[];alias:string|null;glob:boolean}[]=>{
 const out:{path:string[];alias:string|null;glob:boolean}[]=[];
 const walk=(node:Node,prefix:string[]):void=>{
  if(node.type==="scoped_use_list"){const path=node.childForFieldName("path"),list=node.childForFieldName("list")!;for(const child of list.namedChildren)walk(child,[...prefix,...(path?segments(path):[])]);return;}
  if(node.type==="use_list"){for(const child of node.namedChildren)walk(child,prefix);return;}
  if(node.type==="use_wildcard"){const path=node.namedChildren[0];out.push({path:[...prefix,...(path?segments(path):[])],alias:null,glob:true});return;}
  const alias=node.type==="use_as_clause"?node.childForFieldName("alias")!.text:null,target=node.type==="use_as_clause"?node.childForFieldName("path")!:node,path=[...prefix,...segments(target)];
  if(path.at(-1)==="self")path.pop();out.push({path,alias,glob:false});
 };
 walk(declaration.childForFieldName("argument")!,[]);return out;
};
const declarations=(root:Node):Node[]=>{const found:Node[]=[];const visit=(node:Node):void=>{if(node.type==="use_declaration")found.push(node);for(const child of node.children)visit(child);};visit(root);return found;};
test("every use statement is found, with range, visibility, attributes and leaves",()=>{
 const scanned=scanSource(corpus.source),found=scanUseStatements(scanned);
 expect(found.map(row=>({text:corpus.source.slice(row.start,row.end),visibility:row.visibility,attributes:row.attributes,keyword:corpus.source.slice(row.keywordStart,row.keywordStart+row.visibility.length+(row.visibility?1:0)+3),leaves:row.leaves}))).toEqual(corpus.statements);
 expect(scanned.text).toBe(corpus.source);expect(scanned.tokens.length).toBeGreaterThan(10);
});
for(const text of corpus.skipped)test("a statement that is not a complete use tree is skipped: "+JSON.stringify(text),()=>{expect(scanUseStatements(scanSource(text))).toEqual([]);});
test("tree-sitter finds the same use declarations and leaves",async()=>{
 const parser=await parsed(),root=parser.parse(corpus.source).rootNode,real=scanUseStatements(scanSource(corpus.source)),oracle=declarations(root);
 expect(oracle.length).toBe(real.length);for(const[index,declaration]of oracle.entries())expect(leavesOf(declaration),declaration.text).toEqual(real[index]!.leaves);
});
for(const row of corpus.binding)test("binding of "+JSON.stringify(row.leaf),()=>{expect(leafBinding(row.leaf)).toBe(row.expected);});
for(const row of corpus.render)test("render: "+row.name,()=>{expect(renderUseLeaves(row.leaves,row.visibility,row.attributes,row.indent)).toBe(row.expected);});
test("rendered statements are valid Rust that scan back to the same leaves (tree-sitter and the scanner)",async()=>{
 const parser=await parsed();
 for(const row of corpus.render.filter(item=>item.leaves.length>0)){
  const text=renderUseLeaves(row.leaves,row.visibility,row.attributes,row.indent);expect(parser.parse(text).rootNode.hasError(),row.name).toBe(false);
  const leaves=scanUseStatements(scanSource(text)).flatMap(statement=>statement.leaves),key=(leaf:{path:string[];alias:string|null;glob:boolean})=>JSON.stringify([leaf.path,leaf.alias,leaf.glob]);
  expect(new Set(leaves.map(key)),row.name).toEqual(new Set(row.leaves.map(key)));
  expect(declarations(parser.parse(text).rootNode).flatMap(leavesOf).map(key).sort(),row.name).toEqual([...new Set(row.leaves.map(key))].sort());
 }
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
