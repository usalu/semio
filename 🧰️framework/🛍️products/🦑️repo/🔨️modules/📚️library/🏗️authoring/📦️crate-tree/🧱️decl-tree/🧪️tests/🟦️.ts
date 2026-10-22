/** 🧱️ Contract: the crate declaration tree walks every module of a crate through the root file and its `#[path]` mounts with raw and effective paths, scopes, test flags and component globs, measures `super::` reach, groups declarations into slices that keep coupled children and test modules with their parent, and offers a union-find; the corpus is language-agnostic and tree-sitter's Rust grammar is the independent third-party oracle for the module ids and nesting. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join,resolve} from "node:path";
import {UnionFind,buildSlices,collectDecls,superReach,type ModuleDecl} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
type Node=Parser.SyntaxNode;
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\"):path;
const tableOf=(files:Record<string,string>):Map<string,string>=>new Map(Object.entries(files).map(([path,text])=>[resolve(platform(path)),text]));
const reader=(table:Map<string,string>)=>(path:string):string|null=>table.get(resolve(path))??null;
const posix=(path:string|null):string|null=>path===null?null:path.replace(/^[A-Z]:/u,"").split("\\").join("/");
const summary=(decl:ModuleDecl)=>({id:decl.id,raw:[...decl.raw],eff:[...decl.eff],file:posix(decl.file),scope:posix(decl.scope)!,inlineRoot:decl.inlineRoot,testOnly:decl.testOnly,isComponent:decl.isComponent,parentId:decl.parentId,depthInFile:decl.depthInFile,children:[...decl.children],inline:decl.node.inline});
const parsed=async():Promise<Parser>=>{await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);return parser;};
test("every module declaration of the crate is found through #[path] mounts with its effective path and scope",()=>{
 const tree=collectDecls(resolve(platform(corpus.rootFile)),corpus.rootText,reader(tableOf(corpus.files)));
 expect([...tree.decls.values()].map(summary)).toEqual(corpus.decls);expect(tree.roots).toEqual(corpus.decls.filter(decl=>decl.parentId===null).map(decl=>decl.id));
 const component=tree.decls.get("component")!;expect(component.globItem).not.toBeNull();expect(tree.decls.get("a")!.globItem).toBeNull();
});
test("a mounted file that cannot be read is skipped without failing",()=>{const tree=collectDecls(resolve(platform(corpus.rootFile)),corpus.rootText,()=>null);expect(tree.decls.size).toBe(corpus.decls.filter(decl=>decl.parentId===null||decl.id.startsWith("inline")).length);expect(tree.decls.get("a")!.children).toEqual([]);});
test("tree-sitter finds the same module ids and nesting",async()=>{
 const parser=await parsed(),table=tableOf(corpus.files),found:string[]=[];
 const walk=(nodes:Node[],raw:string[]):void=>{
  let attributes:Node[]=[];
  for(const node of nodes){
   if(node.type==="attribute_item"){attributes.push(node);continue;}
   if(node.type!=="mod_item"){attributes=[];continue;}
   const name=node.childForFieldName("name")!.text,id=[...raw,name].join("::"),body=node.childForFieldName("body"),texts=attributes.map(attribute=>attribute.text);attributes=[];
   found.push(id);
   if(body!==null)walk(body.namedChildren,[...raw,name]);else{const path=texts.map(text=>/path\s*=\s*"([^"]*)"/u.exec(text)?.[1]).filter(value=>value!==undefined).pop();if(path!==undefined){const file=resolve(dirname(raw.length===0?resolve(platform(corpus.rootFile)):[...table.keys()].find(key=>key.endsWith(path.replace(/\//gu,"/").split("/").pop()!))!),path);const text=table.get(file);if(text!==undefined)walk(parser.parse(text).rootNode.namedChildren,[...raw,name]);}}
  }
 };
 walk(parser.parse(corpus.rootText).rootNode.namedChildren,[]);
 expect(found.sort()).toEqual(corpus.decls.map(decl=>decl.id).sort());
});
for(const row of corpus.reach)test("super reach of "+JSON.stringify(row.text),()=>{expect(superReach(row.text)).toBe(row.expected);});
for(const row of corpus.unionFind)test("union-find: "+row.name,()=>{
 const set=new UnionFind();for(const[op,a,b]of row.ops)if(op==="union")set.union(a!,b!);else set.find(a!);
 for(const group of row.groups){const roots=new Set(group.map(name=>set.find(name)));expect(roots.size,group.join()).toBe(1);}
 for(const[a,b]of row.separate)expect(set.find(a!)).not.toBe(set.find(b!));
 expect(set.find("never-seen")).toBe("never-seen");
});
test("slices keep coupled children and tests with their parent and everything else apart",()=>{
 const table=tableOf(corpus.slices.files),tree=collectDecls(resolve(platform("/w/src/lib.rs")),corpus.slices.root,reader(table)),result=buildSlices(tree,reader(table));
 expect(result.slices.map(slice=>({members:slice.decls.map(decl=>decl.id).sort(),attach:slice.attach.map(decl=>decl.id).sort()}))).toEqual(corpus.slices.expected);expect([...result.namespaces]).toEqual([]);
 const traces:string[]=[];buildSlices(tree,reader(table),message=>traces.push(message));expect(traces.some(message=>message.includes("union"))).toBe(true);
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
