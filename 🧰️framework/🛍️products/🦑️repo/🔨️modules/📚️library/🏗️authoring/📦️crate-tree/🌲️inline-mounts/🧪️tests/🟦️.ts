/** 🌲️ Contract: a root-level file module made only of `#[path = "."]` inline mounts is folded back into the root as one inline mount wrapper with its relative `#[path]` attributes rebased to the root directory, while impure, test-only, unreadable or already folded mounts stay; the corpus is language-agnostic and tree-sitter's Rust grammar checks that every folded root is valid Rust. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import {createRequire} from "node:module";
import {dirname,join,resolve} from "node:path";
import {inlineMountTreeFiles} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\"):path;
const table=(files:object):Map<string,string>=>new Map(Object.entries(files).filter((entry):entry is[string,string]=>typeof entry[1]==="string").map(([path,text])=>[resolve(platform(path)),text]));
const reader=(map:Map<string,string>)=>(path:string):string|null=>map.get(resolve(path))??null;
const posix=(path:string):string=>path.replace(/^[A-Z]:/u,"").split("\\").join("/");
const run=(root:string,files:object)=>inlineMountTreeFiles(resolve(platform(corpus.rootFile)),root,reader(table(files)));
for(const row of corpus.cases)test("inline mounts: "+row.name,()=>{const result=run(row.root,row.files);expect(result.rootSource).toBe(row.expected);expect([...result.deletes].map(posix).sort()).toEqual([...row.deletes].sort());expect(result.notes).toEqual(row.notes);});
for(const row of corpus.defects)test.failing("inline mounts defect: "+row.name,()=>{expect(run(row.root,row.files).rootSource).toBe(row.expected);});
test("every folded root is valid Rust (tree-sitter)",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 for(const row of corpus.cases.filter(item=>item.valid))expect(parser.parse(run(row.root,row.files).rootSource).rootNode.hasError(),row.name).toBe(false);
 for(const row of corpus.defects)expect(parser.parse(row.expected).rootNode.hasError(),row.name).toBe(false);
});
test("folding is idempotent: the folded root has nothing left to fold",()=>{for(const row of corpus.cases.filter(item=>item.deletes.length>0)){const again=run(row.expected,{});expect(again.rootSource,row.name).toBe(row.expected);expect(again.deletes.size).toBe(0);}});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
