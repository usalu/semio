/** 🌳️ Contract: the Rust module-tree reader parses the module and item skeleton of a file with exact text ranges, attributes, visibilities and `#[path]`, decides whether a `cfg` implies test, demotes dangling doc comments, extends starts over comments, and prunes and names `use` leaves; the corpus is language-agnostic and the tree-sitter Rust grammar is the independent third-party oracle for structure, `use` leaves, validity and identifiers, with `🐍️.py` modelling the cfg logic and doc demotion. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {cfgImpliesTest,demoteDanglingOuterDocs,identifierSet,parseRustSourceTree,prunedUse,referencedItemNames,rustItemStartWithComments,rustOuterDocStart,rustPathAttributeValue,useLeafNames,type RustModuleNode} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const KEYWORDS=new Set(["fn","let","self","crate","super","mod","pub","use","struct","enum","impl","trait","const","static","type","as","mut","ref","in","for","if","else","match","loop","while","return","unsafe","async","await","move","where","dyn","extern","union","macro_rules","true","false"]);
const project=(item:Node,source:string):{kind:string;name:string|null;visibility:string}=>{
 const kinds:Record<string,string>={function_item:"fn",const_item:"const",static_item:"static",struct_item:"struct",union_item:"struct",enum_item:"enum",type_item:"type",trait_item:"trait",impl_item:"impl",macro_definition:"macro",extern_crate_declaration:"extern-crate",use_declaration:"use"};
 const kind=kinds[item.type]??"other",visibility=item.namedChildren.find(child=>child.type==="visibility_modifier")?.text??"private";
 const name=kind==="use"||kind==="impl"||kind==="other"?null:item.childForFieldName("name")?.text??item.namedChildren.find(child=>child.type==="identifier")?.text??null;
 return{kind,name:kind==="impl"?"@impl":name,visibility};
};
const structure=(body:Node[],source:string):{modules:unknown[];items:unknown[]}=>{
 const modules:unknown[]=[],items:unknown[]=[];let attributes:Node[]=[];
 for(const node of body){
  if(node.type==="attribute_item"){attributes.push(node);continue;}
  if(node.type==="line_comment"||node.type==="block_comment")continue;
  if(node.type==="mod_item"){
   const list=node.childForFieldName("body"),inner=list===null?{modules:[],items:[]}:structure(list.namedChildren,source),start=attributes[0]?.startIndex??node.startIndex,texts=attributes.map(attribute=>attribute.text.replace(/^#\[/u,"").replace(/\]$/u,""));
   const path=texts.map(text=>/^path\s*=\s*"((?:[^"\\]|\\.)*)"$/u.exec(text)?.[1]).filter(value=>value!==undefined).pop()??null;
   modules.push({name:node.childForFieldName("name")!.text,visibility:node.namedChildren.find(child=>child.type==="visibility_modifier")?.text??"private",inline:list!==null,attributes:texts,pathAttribute:path,text:source.slice(start,node.endIndex),children:inner.modules,items:inner.items});
  }else items.push(project(node,source));
  attributes=[];
 }
 return{modules,items};
};
const strip=(value:unknown):unknown=>{if(Array.isArray(value))return value.map(strip);if(value&&typeof value==="object"){const copy:Record<string,unknown>={};for(const[key,item]of Object.entries(value))if(key!=="text"||!("kind"in value))copy[key]=strip(item);return copy;}return value;};
const real=(source:string)=>{
 const tree=parseRustSourceTree(source),module=(node:RustModuleNode):unknown=>({name:node.name,visibility:node.visibility,inline:node.inline,conditionalTest:node.conditionalTest,pathAttribute:node.pathAttribute,attributes:[...node.attributeTexts],text:source.slice(node.start,node.end),children:node.children.map(module),items:node.items.map(item=>({kind:item.kind,name:item.name,visibility:item.visibility}))});
 return{innerAttributes:tree.innerAttributes.map(attribute=>attribute.text),modules:tree.modules.map(module),items:tree.items.map(item=>({kind:item.kind,name:item.name,visibility:item.visibility,text:source.slice(item.start,item.end)}))};
};
const sorted=(values:Iterable<string>):string[]=>[...values].sort();
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const leafNames=(node:Node):string[]=>{
 const argument=node.childForFieldName("argument")!;
 const walk=(current:Node):string[]=>{
  if(current.type==="identifier"||current.type==="crate"||current.type==="self"||current.type==="super")return current.type==="identifier"?[current.text]:[];
  if(current.type==="scoped_identifier"){const name=current.childForFieldName("name")!;return name.type==="identifier"?[name.text]:[];}
  if(current.type==="use_as_clause")return[current.childForFieldName("alias")!.text];
  if(current.type==="use_wildcard")return[];
  if(current.type==="scoped_use_list"||current.type==="use_list"){const list=current.type==="use_list"?current:current.childForFieldName("list")!;const path=current.type==="scoped_use_list"?current.childForFieldName("path"):null;return list.namedChildren.flatMap(child=>child.type==="self"?[path?.type==="scoped_identifier"?path.childForFieldName("name")!.text:path?.text??""]:walk(child));}
  return[];
 };
 return walk(argument);
};
for(const row of corpus.sources)test("module tree of "+row.name,()=>{expect(real(row.text)).toEqual(row.tree as never);});
for(const row of corpus.defects)test.failing("module tree defect: "+row.name,()=>{const tree=parseRustSourceTree(row.text);expect<unknown>({kind:tree.items[0]!.kind,name:tree.items[0]!.name,visibility:tree.items[0]!.visibility}).toEqual(row.expectedItem);});
test("tree-sitter's Rust grammar reads the same module and item skeleton",async()=>{
 const parser=await parsed();
 for(const row of corpus.sources.filter(item=>item.oracle)){
  const root=parser.parse(row.text).rootNode;expect(root.hasError(),row.name).toBe(false);const isInner=(node:Node):boolean=>node.type==="inner_attribute_item"||(node.type==="shebang"&&node.text.startsWith("#!["));
  const found=structure(root.namedChildren.filter(node=>!isInner(node)),row.text);
  const expected=real(row.text);
  const withoutCfg=(value:any):any=>{const{conditionalTest:_,...rest}=value;return{...rest,children:value.children.map(withoutCfg),items:value.items.map((item:any)=>({kind:item.kind,name:item.kind==="impl"?"@impl":item.name,visibility:item.visibility}))};};
  const dropCfg=(value:any):any=>{const{conditionalTest:_,...rest}=value;return{...rest,children:value.children.map(dropCfg)};};
  expect((found.modules as any[]).map(dropCfg),row.name).toEqual(expected.modules.map(withoutCfg));
  expect(found.items,row.name).toEqual(expected.items.map(item=>({kind:item.kind,name:item.kind==="impl"?"@impl":item.name,visibility:item.visibility})).map(item=>item.kind==="use"?{...item,name:null}:item));
  expect(root.namedChildren.filter(isInner).map(node=>node.text)).toEqual(expected.innerAttributes);
 }
});
for(const row of corpus.cfg)test("cfg implies test: "+JSON.stringify(row.attribute),()=>{expect(cfgImpliesTest(row.attribute)).toBe(row.expected);});
for(const[index,row]of corpus.docs.entries())test("dangling doc demotion "+index,()=>{expect(demoteDanglingOuterDocs(row.source)).toBe(row.expected);});
for(const row of corpus.itemComments)test("item start over comments: "+JSON.stringify(row.source),()=>{const start=row.source.indexOf(row.needle),at=rustItemStartWithComments(row.source,start);expect(row.source.slice(at,at+row.expected.length)).toBe(row.expected);expect(row.source.indexOf(row.expected)).toBe(at);});
for(const row of corpus.outerDocs)test("outer doc start: "+JSON.stringify(row.source),()=>{const start=row.source.indexOf(row.needle),at=rustOuterDocStart(row.source,start);expect(row.source.slice(at,at+row.expected.length)).toBe(row.expected);});
for(const row of corpus.pathValue)test("path attribute value of "+JSON.stringify(row.attribute),()=>{expect(rustPathAttributeValue(row.attribute)).toBe(row.expected);});
for(const row of corpus.leaves)test("use leaf names of "+JSON.stringify(row.statement),()=>{expect(useLeafNames(row.statement)).toEqual(row.expected);});
for(const row of corpus.leafDefects)test.failing("use leaf names defect "+JSON.stringify(row.statement),()=>{expect(useLeafNames(row.statement)).toEqual(row.expected);});
test("tree-sitter derives the same use leaves",async()=>{
 const parser=await parsed();
 for(const row of corpus.leaves.filter(item=>item.statement.startsWith("use ")||item.statement.includes(" use "))){const root=parser.parse(row.statement).rootNode,declaration=root.namedChildren.find(node=>node.type==="use_declaration")!;expect(leafNames(declaration),row.statement).toEqual(row.expected);}
 for(const row of corpus.leafDefects){const root=parser.parse(row.statement).rootNode;expect(leafNames(root.namedChildren.find(node=>node.type==="use_declaration")!)).toEqual(row.expected);}
});
for(const row of corpus.prune)test("pruned use of "+JSON.stringify(row.statement)+" to "+JSON.stringify(row.leaves),()=>{expect(prunedUse(row.statement,row.leaves)).toBe(row.expected);});
test("every pruned use is valid Rust that binds exactly the kept leaves (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const row of corpus.prune){
  const text=prunedUse(row.statement,row.leaves);if(text==="")continue;const root=parser.parse(text).rootNode;expect(root.hasError(),text).toBe(false);
  const bound=sorted(leafNames(root.namedChildren.find(node=>node.type==="use_declaration")!)),original=sorted(leafNames(parser.parse(row.statement).rootNode.namedChildren.find(node=>node.type==="use_declaration")!).filter(name=>row.leaves.includes(name)));
  expect(bound,text).toEqual(original);
 }
});
for(const row of corpus.identifiers)test("identifier tokens of "+JSON.stringify(row.source),()=>{expect(sorted(identifierSet(row.source))).toEqual(row.expected);});
test("identifier tokens are tree-sitter's identifiers plus Rust keywords",async()=>{
 const parser=await parsed();
 for(const row of corpus.identifiers){const root=parser.parse(row.source).rootNode,named=new Set<string>();const visit=(node:Node):void=>{if(["identifier","type_identifier","field_identifier","primitive_type"].includes(node.type))named.add(node.text);for(const child of node.children)visit(child);};visit(root);const found=identifierSet(row.source);for(const name of named)expect(found.has(name),name).toBe(true);for(const name of found)expect(named.has(name)||KEYWORDS.has(name),name).toBe(true);}
});
for(const row of corpus.references)test("referenced item names of "+JSON.stringify(row.source),()=>{expect(sorted(referencedItemNames(row.source))).toEqual(row.expected);});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
