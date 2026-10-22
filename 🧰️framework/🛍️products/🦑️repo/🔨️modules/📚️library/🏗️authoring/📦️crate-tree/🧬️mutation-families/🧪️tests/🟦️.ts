/** 🧬️ Contract: the mutation-family codemod finds every transparent `#[derive(dsl::Mutations)]` aggregate with its snapshot, diff and variant payload paths, reads the names an aggregate module puts in scope, decouples a leaf from its aggregate glob, and converts concrete leaves to leaves generic over their aggregate (keeping concrete builders on the aggregate, leaving unsupported leaves for hand work); the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the aggregates and for the validity of every produced file. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join,relative,resolve} from "node:path";
import {collectDecls} from "../../🧱️decl-tree/🟦️.ts";
import {aggregateScope,convertMutationLeaves,decoupleFromAggregate,findMutationAggregates} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\"):path;
const ROOT=resolve(platform("/w")),relativePath=(absolute:string):string=>relative(ROOT,absolute).split("\\").join("/");
const readerOf=(files:Readonly<Record<string,string|undefined>>)=>(absolute:string):string|null=>files[relativePath(absolute)]??null;
const fileOf=(files:object,path:string):string=>(files as Readonly<Record<string,string|undefined>>)[path]!;
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
const find=(root:Node,type:string):Node[]=>{const found:Node[]=[];const visit=(node:Node):void=>{if(node.type===type)found.push(node);for(const child of node.children)visit(child);};visit(root);return found;};
const mutationEnums=(root:Node):{name:string;variants:{variant:string;path:string}[]}[]=>{
 const out:{name:string;variants:{variant:string;path:string}[]}[]=[];
 for(const item of find(root,"enum_item")){
  let derived=false;for(let sibling=item.previousSibling;sibling&&sibling.type==="attribute_item";sibling=sibling.previousSibling)if(/^#\[derive\(/u.test(sibling.text)&&/(?:^|[^\w:])(?:\w+::)*Mutations(?![\w:])/u.test(sibling.text))derived=true;
  if(!derived)continue;
  const variants:{variant:string;path:string}[]=[];let supported=true;
  for(const variant of item.childForFieldName("body")!.namedChildren){
   if(variant.type!=="enum_variant")continue;
   const body=variant.childForFieldName("body");
   if(!body||body.type!=="ordered_field_declaration_list"||body.namedChildren.filter(child=>child.type!=="attribute_item").length!==1){supported=false;break;}
   variants.push({variant:variant.childForFieldName("name")!.text,path:body.namedChildren.filter(child=>child.type!=="attribute_item")[0]!.text.replace(/\s+/gu,"")});
  }
  if(supported&&variants.length)out.push({name:item.childForFieldName("name")!.text,variants});
 }
 return out;
};
const witness=(row:{defect:string})=>row.defect?test.failing:test;
for(const row of corpus.aggregates)witness(row)("aggregates: "+row.name+(row.defect?" ["+row.defect+"]":""),()=>{
 const reader=readerOf(row.files),tree=collectDecls(resolve(ROOT,"lib.rs"),fileOf(row.files,"lib.rs"),reader),found=findMutationAggregates(tree.decls.values(),reader);
 expect(found.map(entry=>({name:entry.name,file:relativePath(entry.file),effPath:entry.effPath,snapshot:entry.snapshot,diff:entry.diff,variants:entry.variants}))).toEqual(row.expect);
});
for(const row of corpus.aggregates)witness(row)("tree-sitter finds the same aggregates and variants: "+row.name,async()=>{
 const parser=await parsed(),reader=readerOf(row.files),real=findMutationAggregates(collectDecls(resolve(ROOT,"lib.rs"),fileOf(row.files,"lib.rs"),reader).decls.values(),reader),oracle=mutationEnums(parser.parse(fileOf(row.files,"a.rs")).rootNode);
 expect(real.map(entry=>({name:entry.name,variants:entry.variants.map(variant=>[variant.variant,variant.path])}))).toEqual(oracle.map(entry=>({name:entry.name,variants:entry.variants.map(variant=>[variant.variant,variant.path])})));
});
for(const row of corpus.convert)test("conversion: "+row.name,()=>{
 const reader=readerOf(row.files),result=convertMutationLeaves(resolve(ROOT,row.root),fileOf(row.files,row.root),reader);
 const edits:Record<string,string>={},creates:Record<string,string>={};for(const[key,value]of[...result.edits].sort())edits[relativePath(key)]=value;for(const[key,value]of[...result.creates].sort())creates[relativePath(key)]=value;
 expect<unknown>({converted:[...result.converted],hand:result.hand.map(entry=>({payload:entry.payload,reason:entry.reason})),notes:[...result.notes],aggregates:result.aggregates.map(entry=>({name:entry.name,snapshot:entry.snapshot,diff:entry.diff,variants:entry.variants.map(variant=>variant.variant)})),edits,creates}).toEqual(row.expect);
 const again=convertMutationLeaves(resolve(ROOT,row.root),fileOf(row.files,row.root),reader);expect([...again.edits]).toEqual([...result.edits]);expect([...again.creates]).toEqual([...result.creates]);
 for(const[file,text]of Object.entries(edits))expect(text===fileOf(row.files,file),file+" is an unchanged edit").toBe(false);
 for(const file of Object.keys(edits))expect(Object.keys(row.files),file).toContain(file);
 for(const file of Object.keys(creates))expect(Object.keys(row.files),file).not.toContain(file);
 for(const payload of result.converted)expect(result.aggregates.flatMap(entry=>entry.variants.map(variant=>variant.payload)),payload).toContain(payload);
 expect(result.converted.length+result.hand.length+(result.notes.some(note=>note.includes("stays concrete: its module is private"))?result.aggregates[0]!.variants.length:0)).toBe(result.aggregates.reduce((sum,entry)=>sum+entry.variants.length,0));
});
test("every produced file is valid Rust (tree-sitter) and converted leaves no longer name their aggregate type",async()=>{
 const parser=await parsed();
 for(const row of corpus.convert)for(const[file,text]of[...Object.entries(row.expect.edits),...Object.entries(row.expect.creates)]){
  expect(parser.parse(text).rootNode.hasError(),row.name+": "+file).toBe(false);
  if(/\/(?:alpha|beta|delta|epsilon)\/mod\.rs$/u.test(file)&&text.includes("MutationKind<")){
   const impls=find(parser.parse(text).rootNode,"impl_item").filter(item=>/MutationKind/u.test(item.text));expect(impls.length,file).toBeGreaterThan(0);
   for(const impl of impls){expect(impl.text,file).toContain("impl<Op>");expect(impl.text,file).not.toMatch(/\bDemoMutation\b/u);}
  }
 }
});
for(const[index,row]of corpus.scope.entries())test("aggregate scope: "+row.name,()=>{
 const scope=aggregateScope(row.text,row.eff,row.isComponent),names:Record<string,unknown>={};for(const[key,value]of[...scope.names].sort())names[key]=value;
 expect({names,glob:scope.glob}).toEqual(row.expect);void index;
});
test("tree-sitter agrees on the bindings of every imported name and on every import statement",async()=>{
 const parser=await parsed();
 for(const row of corpus.scope){
  const scope=aggregateScope(row.text,row.eff,row.isComponent),root=parser.parse(row.text).rootNode,bound=new Set<string>();
  for(const declaration of find(root,"use_declaration")){
   const previous=declaration.previousSibling;if(previous?.type==="attribute_item"&&/cfg\(\s*test\s*\)/u.test(previous.text))continue;
   for(const leaf of leavesOf(declaration))if(!leaf.glob)bound.add(leaf.alias??leaf.path.at(-1)!);
  }
  expect([...bound].sort(),row.name).toEqual([...scope.names.keys()].filter(key=>bound.has(key)).sort());
  for(const[key,entry]of scope.names){
   if(entry.kind==="item"||entry.statement===undefined)continue;
   const parsedStatement=parser.parse(entry.statement).rootNode;expect(parsedStatement.hasError(),entry.statement).toBe(false);
   const [leaf]=find(parsedStatement,"use_declaration").flatMap(leavesOf);expect(leaf!.alias??leaf!.path.at(-1),entry.statement).toBe(key);
  }
 }
});
for(const row of corpus.decouple)test("decoupling: "+row.name,()=>{
 const scope=aggregateScope(row.aggregate,row.eff,row.isComponent),result=decoupleFromAggregate(row.leaf,row.depth,scope);
 expect(result===null?null:{text:result.text,coupled:[...result.coupled]}).toEqual(row.expect);
});
test("a decoupled leaf is valid Rust, keeps no aggregate glob and imports exactly the used scope names",async()=>{
 const parser=await parsed();
 for(const row of corpus.decouple){
  if(row.expect===null||row.expect.coupled.length)continue;
  const root=parser.parse(row.expect.text).rootNode;expect(root.hasError(),row.name).toBe(false);
  expect(find(root,"use_declaration").flatMap(leavesOf).filter(leaf=>leaf.glob&&leaf.path.every(part=>part==="super")),row.name).toEqual([]);
 }
});

test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
