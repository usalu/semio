import {expect,test} from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {classifyModuleReferencesV1,scanModuleSpecifiersV1,type ModuleReferenceV1} from "../🟦️.ts";
import {nxBuildEngineClosureV1,nxBuildEngineReferencesV1} from "../../../../🟨️.mjs";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🧫️corpus/🔣️.json" with {type:"json"};

const root=fileURLToPath(new URL("../../../../../../../../../",import.meta.url));

const oracle=(source:string):ModuleReferenceV1[]=>{
 const rows:ModuleReferenceV1[]=[],seen=new Set<string>(),add=(kind:ModuleReferenceV1["kind"],specifier:string)=>{const identity=kind+"\0"+specifier;if(!seen.has(identity)){seen.add(identity);rows.push({kind,specifier});}};
 for(const row of ts.preProcessFile(source,true,true).importedFiles)add("module",row.fileName);
 if(!source.includes("URL"))return rows;
 const syntax=ts.createSourceFile("engine.ts",source,ts.ScriptTarget.Latest,false),visit=(node:ts.Node):void=>{
  if(ts.isNewExpression(node)&&ts.isIdentifier(node.expression)&&node.expression.text==="URL"&&node.arguments?.length===2&&ts.isStringLiteralLike(node.arguments[0]!)&&node.arguments[0].text.startsWith(".")&&node.arguments[1]!.getText(syntax)==="import.meta.url"){const specifier=node.arguments[0].text;add(new URL(specifier,"file:///engine.ts").pathname.endsWith("/")?"directory-url":"file-url",specifier);}
  ts.forEachChild(node,visit);
 };
 visit(syntax);return rows;
};
const engineSources=():[string,string][]=>nxBuildEngineClosureV1().files.filter(file=>!file.endsWith(".json")).map(file=>[file,readFileSync(join(root,file),"utf8")]);

test("the corpus is closed by the schema (Ajv as the independent validator)",()=>{
 const validate=new Ajv({strict:false}).compile({$ref:"#/definitions/Corpus",definitions:schema.definitions});
 expect(validate(corpus)).toBe(true);
 expect(new Set(corpus.map(row=>row.name)).size).toBe(corpus.length);
 expect(validate([{name:"x",source:"",references:[{kind:"other",specifier:"a"}]}])).toBe(false);
});

test("the own scanner reads the language-agnostic corpus rows exactly",()=>{
 for(const row of corpus)expect<unknown>({name:row.name,rows:classifyModuleReferencesV1(row.source)}).toEqual({name:row.name,rows:row.references});
});

test("the TypeScript compiler API (the oracle) reads the same corpus rows: comments, strings, templates, regular expressions, dynamic imports, re-exports, type-only imports, URLs",()=>{
 for(const row of corpus)expect<unknown>({name:row.name,rows:oracle(row.source)}).toEqual({name:row.name,rows:row.references});
});

test("the own scanner and the compiler oracle agree on every source file of the engine closure",()=>{
 const sources=engineSources();
 expect(sources.length).toBeGreaterThan(1000);
 const differing=sources.filter(([,source])=>JSON.stringify(classifyModuleReferencesV1(source))!==JSON.stringify(oracle(source))).map(([file])=>file);
 expect(differing).toEqual([]);
 expect(sources.some(([,source])=>classifyModuleReferencesV1(source).some(row=>row.kind==="file-url"))).toBe(true);
});

test("the module pass equals the compiler on every truncation of engine files (seeded cut points reach every lexical state)",()=>{
 const sources=engineSources().filter(([,source])=>source.length>2000).filter((_,index)=>index%40===0);
 expect(sources.length).toBeGreaterThan(10);
 let state=2463534242;const next=()=>{state^=state<<13;state^=state>>>17;state^=state<<5;return(state>>>0)/4294967296;};
 const differing:string[]=[];
 for(const [file,source] of sources)for(let cut=0;cut<12;cut++){
  const text=source.slice(0,Math.floor(next()*source.length));
  if(JSON.stringify(scanModuleSpecifiersV1(text))!==JSON.stringify(ts.preProcessFile(text,true,true).importedFiles.map(row=>row.fileName)))differing.push(`${file}@${text.length}`);
 }
 expect(differing).toEqual([]);
});

test("the engine closure reads its references with the own scanner: no compiler package is loaded by the engine reference classification",()=>{
 const library=readFileSync(new URL("../../../../🟨️.mjs",import.meta.url),"utf8");
 expect(library.includes("preProcessFile")).toBe(false);
 expect(/\("typescript"\)/u.test(library)).toBe(false);
 for(const row of corpus)expect<unknown>(nxBuildEngineReferencesV1(row.source)).toEqual(row.references);
});
