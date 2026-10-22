/** 🏛️ Contract: the artifact I/O architecture scanner finds semantic mutation and diff declarations, physical wire dependencies of a schema (outside `cfg(test)`), physical TypeScript wire exports and semantic diffs that lack a text or binary implementation, ignoring strings and comments; the corpus is language-agnostic, tree-sitter's Rust grammar and the TypeScript compiler API are the independent third-party oracles for the fixtures and the export enumeration. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import ts from "typescript";
import {createRequire} from "node:module";
import {dirname,join} from "node:path";
import {missingArtifactDiffWireTypes,schemaRustWireDependencies,schemaTypeScriptWireSymbols,semanticArtifactIoItems} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const wasm=join(dirname(createRequire(import.meta.url).resolve("tree-sitter-wasms/package.json")),"out","tree-sitter-rust.wasm");
const SYMBOLS=/(?:To|From)(?:Native)?Json(?:Text|Value|Projection)?$|Json(?:Text|Projection)$|^(?:decode|encode|write).*Json(?:Value)?$|^decode.*Protobuf$|^floatLexeme$|^jsonGeometry$|^TxtProtobuf|^txtProtobuf(?:Key|String)$|^decodeRemodeling(?:Snapshot|Diff|Mutation|Artifact)$|^(?:render|infer).*Tikz(?:Plan)?$/u;
const exportsOf=(source:string):string[]=>{
 const file=ts.createSourceFile("x.ts",source,ts.ScriptTarget.ES2022,true),names:string[]=[];
 for(const statement of file.statements){
  const exported=ts.canHaveModifiers(statement)&&(ts.getModifiers(statement)??[]).some(modifier=>modifier.kind===ts.SyntaxKind.ExportKeyword);
  if(exported&&ts.isVariableStatement(statement))for(const declaration of statement.declarationList.declarations)names.push((declaration.name as ts.Identifier).text);
  else if(exported&&(ts.isFunctionDeclaration(statement)||ts.isClassDeclaration(statement))&&statement.name)names.push(statement.name.text);
  else if(ts.isExportDeclaration(statement)&&statement.moduleSpecifier===undefined&&statement.exportClause&&ts.isNamedExports(statement.exportClause))for(const element of statement.exportClause.elements)names.push(element.name.text);
 }
 return names;
};
for(const row of corpus.semantic)test("semantic io items: "+row.name,()=>{expect(semanticArtifactIoItems(row.source)).toEqual(row.expected);});
for(const row of corpus.dependencies)test("schema wire dependencies: "+row.name,()=>{expect(schemaRustWireDependencies(row.source)).toEqual(row.expected);});
for(const row of corpus.symbols)test("typescript wire symbols: "+row.name,()=>{expect(schemaTypeScriptWireSymbols(row.source)).toEqual(row.expected);});
for(const row of corpus.diffs)test("missing diff wire types: "+row.name,()=>{expect(missingArtifactDiffWireTypes(row.semantic,row.text,row.binary)).toEqual(row.expected);});
test("the TypeScript compiler API enumerates the same exports the scanner filters",()=>{for(const row of corpus.symbols)expect(exportsOf(row.source).filter(name=>SYMBOLS.test(name)).sort(),row.name).toEqual(row.expected);});
test("every Rust fixture of the corpus is syntactically valid (tree-sitter)",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 const wrapped=(text:string):string=>text;
 for(const row of[...corpus.semantic,...corpus.dependencies])if(!row.source.includes("todo!()")||true)expect(parser.parse(wrapped(row.source)).rootNode.hasError(),row.name).toBe(false);
 for(const row of corpus.diffs)for(const text of[row.semantic,row.text,row.binary])expect(parser.parse(text).rootNode.hasError(),row.name+": "+text).toBe(false);
});
test("tree-sitter finds the declarations the semantic scanner reports",async()=>{
 await Parser.init();const language=await Parser.Language.load(wasm),parser=new Parser();parser.setLanguage(language);
 for(const row of corpus.semantic.filter(item=>item.expected.length>0)){
  const root=parser.parse(row.source).rootNode,traits=new Set<string>(),functions=new Set<string>();
  const visit=(node:Parser.SyntaxNode):void=>{if(node.type==="impl_item"){const trait=node.childForFieldName("trait");if(trait!==null)traits.add(trait.type==="generic_type"?trait.childForFieldName("type")!.text:trait.text);}if(node.type==="function_item")functions.add(node.childForFieldName("name")!.text);for(const child of node.children)visit(child);};visit(root);
  for(const name of row.expected)expect(traits.has(name)||functions.has(name),row.name+" "+name).toBe(true);
 }
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
