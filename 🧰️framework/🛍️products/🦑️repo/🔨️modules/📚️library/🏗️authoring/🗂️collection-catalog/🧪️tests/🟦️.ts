/** 🗂️ Contract: the collection catalog generator admits only a complete `x-semio-collection-catalog` schema, renders the Rust modules and the TypeScript catalog from it deterministically, finds the module that owns each row type without closing an import cycle, and detects stale generated files; the corpus is language-agnostic, the TypeScript compiler API parses the generated TypeScript and `🐍️oracle.py` re-renders the catalog independently. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import ts from "typescript";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {resolve} from "node:path";
import {generateCollectionCatalogV1,parseCollectionCatalogV1,renderCollectionCatalogRustV1,renderCollectionCatalogTypeScriptV1,resolveCollectionRowOwnersV1} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const clone=<T,>(value:T):T=>JSON.parse(JSON.stringify(value));
const apply=(document:any,change:{op:string;path:string;value?:unknown}):any=>{
 const keys=change.path.split(".");let cursor=document;for(const key of keys.slice(0,-1))cursor=cursor[key];const last=keys[keys.length-1]!;
 if(change.op==="delete")delete cursor[last];else cursor[last]=change.value;return document;
};
const place=(root:string,files:Readonly<Record<string,string|undefined>>):void=>{for(const[path,text]of Object.entries(files)){if(text===undefined)continue;const target=join(root,...path.split("/"));mkdirSync(dirname(target),{recursive:true});writeFileSync(target,text);}};
const reader=(root:string,files:Readonly<Record<string,string|undefined>>)=>{const table=new Map(Object.entries(files).flatMap(([path,text])=>text===undefined?[]:[[resolve(root,...path.split("/")),text] as const]));return(path:string):string|undefined=>table.get(resolve(path));};
const owners=():Map<string,string>=>new Map(Object.entries(corpus.renderTsOwners));
test("a complete document is admitted as the declared catalog",()=>{expect<unknown>(parseCollectionCatalogV1(corpus.document)).toEqual(corpus.parsed);});
for(const row of corpus.invalid)test("parse refuses "+row.name,()=>{expect(()=>parseCollectionCatalogV1(apply(clone(corpus.document),row.patch))).toThrow(row.message);});
for(const[index,value]of corpus.notDocuments.entries())test("parse refuses the non-document "+index,()=>{expect(()=>parseCollectionCatalogV1(value)).toThrow("schema document must be an object");});
test("the TypeScript catalog is rendered byte for byte",()=>{expect(renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),owners())).toBe(corpus.renderTs);expect(renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),new Map([["NodeRow","../rows/index.ts"],["EdgeRow","../rows/index.ts"]]))).toBe(corpus.renderTsSameModule);});
test("a row type without an owning module cannot be rendered",()=>{expect(()=>renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),new Map([["NodeRow","./x.ts"]]))).toThrow("EdgeRow has no owning module");});
test("the generated TypeScript parses cleanly and exports the declared shapes (TypeScript compiler API)",()=>{
 const text=renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),owners()),file=ts.createSourceFile("catalog.ts",text,ts.ScriptTarget.ES2022,true,ts.ScriptKind.TS);
 expect((file as unknown as {parseDiagnostics:unknown[]}).parseDiagnostics).toEqual([]);
 const names:string[]=[];file.forEachChild(node=>{if(ts.isTypeAliasDeclaration(node)||ts.isInterfaceDeclaration(node))names.push(node.name.text);else if(ts.isVariableStatement(node))for(const declaration of node.declarationList.declarations)names.push((declaration.name as ts.Identifier).text);else if(ts.isImportDeclaration(node))names.push("import:"+(node.moduleSpecifier as ts.StringLiteral).text);});
 expect(names.sort()).toEqual(["SHAPES","ShapeKey","ShapeKeyEntry","ShapeRow","import:../rows/edge.ts","import:../rows/node.ts"]);
 const emitted=ts.transpileModule(text,{reportDiagnostics:true,compilerOptions:{module:ts.ModuleKind.ESNext,target:ts.ScriptTarget.ES2022,verbatimModuleSyntax:true}});expect(emitted.diagnostics).toEqual([]);
 expect(emitted.outputText).toContain("export const SHAPES");
});
for(const[name,needles]of Object.entries(corpus.rust))test("the Rust "+(name==="root"?"root":"module "+name)+" holds every generated item",()=>{
 const files=renderCollectionCatalogRustV1(parseCollectionCatalogV1(corpus.document)),file=files.find(item=>item.folder===(name==="root"?null:name));
 expect(file,name).toBeDefined();for(const needle of needles)expect(file!.text.includes(needle),needle).toBe(true);
 expect(file!.text.startsWith("//! ")).toBe(true);
});
test("the Rust catalog is one root and four child modules",()=>{const files=renderCollectionCatalogRustV1(parseCollectionCatalogV1(corpus.document));expect(files.map(item=>item.folder)).toEqual([null,...Object.keys(corpus.rust).filter(name=>name!=="root")]);});
for(const row of corpus.ownerCases)test("row owners: "+row.name,()=>{
 const root=resolve("/w");expect(Object.fromEntries([...resolveCollectionRowOwnersV1({...parseCollectionCatalogV1(corpus.document),typescript:{rows:row.rows}},join(root,...row.output.split("/")),reader(root,row.files)).entries()].sort())).toEqual(row.expected);
});
for(const row of corpus.ownerErrors)test("row owners fail: "+row.name,()=>{
 const root=resolve("/w");expect(()=>resolveCollectionRowOwnersV1({...parseCollectionCatalogV1(corpus.document),typescript:{rows:row.rows}},join(root,...row.output.split("/")),reader(root,row.files))).toThrow(row.message);
});
test("the generator writes every output once, then reports a stale one",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"collection-")));
 try{
  place(root,corpus.generation.files);place(root,{[corpus.generation.schemaPath]:JSON.stringify(corpus.document)});const schemaPath=join(root,...corpus.generation.schemaPath.split("/"));
  expect(generateCollectionCatalogV1(schemaPath,"check").length).toBe(corpus.generation.expectedStaleCount);for(const path of[...corpus.generation.rustPaths,corpus.generation.tsPath])expect(existsSync(join(root,...path.split("/")))).toBe(false);
  expect(generateCollectionCatalogV1(schemaPath,"write").length).toBe(corpus.generation.expectedStaleCount);expect(generateCollectionCatalogV1(schemaPath,"check")).toEqual([]);
  expect(readFileSync(join(root,...corpus.generation.tsPath.split("/")),"utf8")).toBe(renderCollectionCatalogTypeScriptV1(parseCollectionCatalogV1(corpus.document),new Map([["NodeRow","../rows/index.ts"],["EdgeRow","../rows/index.ts"]])));
  expect(readFileSync(join(root,...corpus.generation.rustPaths[0]!.split("/")),"utf8")).toBe(renderCollectionCatalogRustV1(parseCollectionCatalogV1(corpus.document))[0]!.text);
  writeFileSync(join(root,...corpus.generation.rustPaths[2]!.split("/")),"// hand edit\n");const stale=generateCollectionCatalogV1(schemaPath,"check");expect(stale.length).toBe(1);expect(stale[0]!.endsWith(join("🔎\ufe0fkey-base-access","🦀\ufe0f.rs"))).toBe(true);
  expect(generateCollectionCatalogV1(schemaPath,"write").length).toBe(1);expect(generateCollectionCatalogV1(schemaPath,"check")).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
test("the independent Python model reaches the same verdicts",()=>{
 if(!python)return;
 const result=Bun.spawnSync([python,"-I",join(here,"🐍️oracle.py"),join(here,"🔣️.json")]);
 expect(result.stderr.toString()).toBe("");expect(JSON.parse(result.stdout.toString())).toEqual({failures:[]});expect(result.exitCode).toBe(0);
});
