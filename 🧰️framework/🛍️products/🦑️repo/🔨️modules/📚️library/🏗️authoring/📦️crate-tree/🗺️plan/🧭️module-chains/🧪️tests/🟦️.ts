/** 🧭️ Contract: the Rust module-chain reader resolves inline and external module files, lists `crate::` chains, alias chains, root alias modules, `super::` chains, relative heads and path heads of a source, orders a dependency graph into strongly connected components, prefixes module heads with `crate::` and parses `impl` headers; the corpus is language-agnostic, tree-sitter's Rust grammar and graphlib's Tarjan are the independent third-party oracles. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {Graph,alg} from "graphlib";
import {createRequire} from "node:module";
import {dirname,join,resolve} from "node:path";
import {parseRustSourceTree,type RustModuleNode} from "../../../🌳️module-tree/🟦️.ts";
import {aliasChains,crateRelativeChains,externalModuleFile,implHeader,inlineModuleBase,pathHeadNames,qualifyModuleHeads,relativeHeadChains,rootAliasModules,stronglyConnected,superChains} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
const segments=(node:Node):string[]=>{
 if(node.type==="identifier"||node.type==="type_identifier"||node.type==="crate"||node.type==="self"||node.type==="super")return[node.text];
 if(node.type==="scoped_identifier"||node.type==="scoped_type_identifier"){const path=node.childForFieldName("path"),name=node.childForFieldName("name")!;return[...(path?segments(path):[]),name.text];}
 return[];
};
const treeChains=(node:Node,out:string[][]):void=>{
 const rooted=(parts:string[]):string[]|null=>parts[0]==="crate"?parts.slice(1):null;
 const flatten=(current:Node,prefix:string[]):void=>{
  if(current.type==="scoped_use_list"){const path=current.childForFieldName("path"),list=current.childForFieldName("list")!,head=[...prefix,...(path?segments(path):[])];for(const child of list.namedChildren)flatten(child,head);return;}
  if(current.type==="use_list"){for(const child of current.namedChildren)flatten(child,prefix);return;}
  if(current.type==="use_as_clause"){flatten(current.childForFieldName("path")!,prefix);return;}
  if(current.type==="use_wildcard"){const path=current.namedChildren[0];const parts=[...prefix,...(path?segments(path):[])];const chain=rooted(parts);if(chain&&chain.length)out.push(chain);return;}
  const parts=[...prefix,...segments(current)],chain=rooted(parts);if(chain&&chain.length)out.push(chain);
 };
 if(node.type==="use_declaration"){flatten(node.childForFieldName("argument")!,[]);return;}
 if((node.type==="scoped_identifier"||node.type==="scoped_type_identifier")){const parts=segments(node),chain=rooted(parts);if(chain&&chain.length)out.push(chain);return;}
 for(const child of node.children)treeChains(child,out);
};
const sets=(rows:string[][]):string[]=>rows.map(row=>row.join("::")).sort();
const node=(row:{name:string;pathAttribute:string|null}):RustModuleNode=>({name:row.name,pathAttribute:row.pathAttribute} as unknown as RustModuleNode);
for(const row of corpus.bases)test("inline module base "+row.parent+" + "+JSON.stringify(row.node),()=>{expect(inlineModuleBase(resolve(row.parent),node(row.node))).toBe(resolve(row.expected));});
for(const row of corpus.files)test("external module file "+JSON.stringify(row.node)+" in file scope "+row.inFile,()=>{expect(externalModuleFile(resolve(row.fileDir),resolve(row.inline),row.inFile,node(row.node))).toBe(row.expected===null?null:resolve(row.expected));});
for(const row of corpus.chains)test("crate-relative chains: "+row.name,()=>{expect(crateRelativeChains(row.source)).toEqual(row.expected);});
test("tree-sitter reads the same crate-rooted paths",async()=>{
 const parser=await parsed();
 for(const row of corpus.chains){const root=parser.parse(row.source).rootNode;if(root.hasError())continue;const found:string[][]=[];treeChains(root,found);expect(sets(found),row.name).toEqual(sets(row.expected));}
});
for(const row of corpus.aliases)test("alias chains: "+row.name,()=>{expect(aliasChains(row.source)).toEqual(row.expected);});
test("root alias modules keep only pure re-export modules with consistent children",()=>{
 const tree=parseRustSourceTree(corpus.rootAlias.source),found=rootAliasModules(corpus.rootAlias.source,tree.modules);
 expect([...found.entries()].map(([name,value])=>({name,targets:value.targets,text:value.text,nested:[...value.nested].sort()}))).toEqual(corpus.rootAlias.expected);
});
for(const row of corpus.graphs)test("strongly connected components: "+row.name,()=>{
 const edges=new Map(Object.entries(row.edges).map(([from,to])=>[from,new Set(to)] as [string,Set<string>])),found=stronglyConnected(row.nodes,edges);
 expect(found.map(component=>[...component].sort()).sort((a,b)=>a.join()<b.join()?-1:1)).toEqual([...row.components].sort((a,b)=>a.join()<b.join()?-1:1));
 const position=new Map(found.flatMap((component,index)=>component.map(name=>[name,index] as [string,number])));
 for(const[from,to]of edges)for(const target of to)if(position.get(from)!==position.get(target))expect(position.get(target)!,from+"->"+target).toBeLessThan(position.get(from)!);
});
test("graphlib's Tarjan finds the same components",()=>{
 for(const row of corpus.graphs){
  const graph=new Graph({directed:true});for(const name of row.nodes)graph.setNode(name);for(const[from,to]of Object.entries(row.edges))for(const target of to)graph.setEdge(from,target);
  const found=alg.tarjan(graph).map(component=>[...component].sort()).sort((a,b)=>a.join()<b.join()?-1:1);expect(found,row.name).toEqual([...row.components].sort((a,b)=>a.join()<b.join()?-1:1));
 }
});
for(const row of corpus.qualify)test("module heads: "+row.name,()=>{expect(qualifyModuleHeads(row.text,new Set(row.modules),new Set(row.exclude))).toBe(row.expected);});
test("qualified text stays valid Rust and only inserts crate:: prefixes (tree-sitter)",async()=>{
 const parser=await parsed();
 for(const row of corpus.qualify){const before=parser.parse(row.text).rootNode.hasError(),text=qualifyModuleHeads(row.text,new Set(row.modules),new Set(row.exclude));if(!before)expect(parser.parse(text).rootNode.hasError(),row.name).toBe(false);expect(text.split("crate::").join("")).toBe(row.text.split("crate::").join(""));}
});
for(const row of corpus.super)test("super chains: "+row.name,()=>{expect(superChains(row.text,row.from,row.to,row.excluded as [number,number][],row.base)).toEqual(row.expected);});
for(const row of corpus.relative)test("relative head chains: "+row.name,()=>{expect(relativeHeadChains(row.text)).toEqual(row.expected);});
for(const row of corpus.heads)test("path head names: "+row.name,()=>{expect([...pathHeadNames(row.text)].sort()).toEqual(row.expected);});
for(const row of corpus.impls)test("impl header: "+row.name,()=>{expect(implHeader(row.text)).toEqual(row.expected);});
test("tree-sitter reads the same impl headers",async()=>{
 const parser=await parsed(),last=(node:Node|null):string|null=>{if(node===null)return null;if(node.type==="type_identifier"||node.type==="identifier")return node.text;if(node.type==="scoped_type_identifier")return node.childForFieldName("name")!.text;if(node.type==="generic_type")return last(node.childForFieldName("type"));if(node.type==="dynamic_type")return last(node.childForFieldName("trait"));if(node.type==="reference_type")return last(node.childForFieldName("type"));return null;};
 const args=(node:Node|null):string[]=>{if(node===null||node.type!=="generic_type")return[];const out:string[]=[];const visit=(current:Node):void=>{if(current.type==="type_identifier")out.push(current.text);for(const child of current.children)visit(child);};visit(node.childForFieldName("type_arguments")!);return out;};
 for(const row of corpus.impls.filter(item=>item.expected!==null)){
  const root=parser.parse(row.text).rootNode,item=root.namedChildren.find(candidate=>candidate.type==="impl_item"||candidate.type==="unsafe_item"||candidate.type==="function_item");if(item===undefined||item.type!=="impl_item")continue;
  const trait=item.childForFieldName("trait"),self=item.childForFieldName("type");
  expect({trait:last(trait),self:last(self),traitArgs:args(trait)},row.name).toEqual(row.expected as never);
 }
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
