/** 🟦️ Contract: the TypeScript field extractor reads the members of an exported interface (optional marks, `| undefined`/`| null`, arrays, `Record`, homogeneous tuples, `@state` JSDoc), follows `extends`, aliases, imports and re-exports through a resolver without executing code and reads relative modules from disk; the corpus is language-agnostic and the TypeScript compiler API is the independent third-party oracle. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {policyExtractTypescriptSchemaFields,policyParseTsFieldType} from "../🟦️.ts";
import {policyExtractTypescriptSchemaFile} from "../📂️module-resolution/🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const scalarOf=(text:string):string=>{const token=text.replace(/\s+/gu,""),table:Record<string,string>={string:"string",boolean:"bool"};return Object.hasOwn(table,token)?table[token]!:token;};
const oracle=(text:string,name:string):{typeName:string;fields:unknown[]}=>{
 const source=ts.createSourceFile("x.ts",text,ts.ScriptTarget.ES2022,true),declaration=source.statements.find((node):node is ts.InterfaceDeclaration=>ts.isInterfaceDeclaration(node)&&node.name.text===name&&(ts.getCombinedModifierFlags(node)&ts.ModifierFlags.Export)!==0);
 if(declaration===undefined)return{typeName:"",fields:[]};
 const fields:unknown[]=[],seen=new Set<string>();
 for(const member of declaration.members){
  if(!ts.isPropertySignature(member))continue;
  const key=member.name.getText(source).replace(/^["']|["']$/gu,"");if(seen.has(key))continue;seen.add(key);
  let type=member.type!,optional=member.questionToken!==undefined;
  if(ts.isUnionTypeNode(type)){const parts=type.types.filter(part=>part.kind!==ts.SyntaxKind.UndefinedKeyword&&!(ts.isLiteralTypeNode(part)&&part.literal.kind===ts.SyntaxKind.NullKeyword));if(parts.length!==type.types.length){optional=true;if(parts.length===1)type=parts[0]!;}}
  let cardinality="scalar",scalar:string;
  if(ts.isTypeReferenceNode(type)&&type.typeName.getText(source)==="Record"&&type.typeArguments?.length===2){cardinality="map";scalar=scalarOf(type.typeArguments[1]!.getText(source));}
  else if(ts.isArrayTypeNode(type)){cardinality="list";scalar=scalarOf(type.elementType.getText(source));}
  else if(ts.isTypeReferenceNode(type)&&type.typeName.getText(source)==="Array"&&type.typeArguments?.length===1){cardinality="list";scalar=scalarOf(type.typeArguments[0]!.getText(source));}
  else if(ts.isTupleTypeNode(type)&&type.elements.length>1&&type.elements.every(element=>element.getText(source)===type.elements[0]!.getText(source))){cardinality="fixedList";scalar=scalarOf(type.elements[0]!.getText(source));}
  else scalar=scalarOf(type.getText(source));
  const tag=ts.getJSDocTags(member).find(item=>item.tagName.text==="state"),state=tag===undefined?"":String(tag.comment??"").trim().toLowerCase().replace(/_/gu,"-");
  fields.push({name:key,optional,cardinality,scalar,state});
 }
 return{typeName:name,fields};
};
const resolver=(files:Record<string,string>)=>(specifier:string,from:string)=>{if(!specifier.startsWith("."))return null;const target=resolve(dirname(from),specifier),found=target in files?target:target+".ts" in files?target+".ts":null;return found===null?null:{moduleId:found,text:files[found]!};};
const posixResolve=(path:string):string=>path;
for(const[index,row]of corpus.types.entries())test("ts field type "+index+": "+JSON.stringify(row.text),()=>{expect<unknown>(policyParseTsFieldType(row.text,row.mark)).toEqual({optional:row.optional,cardinality:row.cardinality,scalar:row.scalar});});
for(const row of corpus.extract)test("typescript extract: "+row.name,()=>{expect<unknown>(policyExtractTypescriptSchemaFields(row.text,row.type)).toEqual(row.expected);});
for(const row of corpus.defects)test.failing("typescript defect: "+row.name,()=>{expect<unknown>(policyExtractTypescriptSchemaFields(row.text,row.type)).toEqual(row.expected);});
test("the TypeScript compiler API reads the same members as the extractor",()=>{for(const row of corpus.extract.filter(item=>item.oracle))expect(oracle(row.text,row.type!),row.name).toEqual(row.expected);});
test("the compiler API agrees with the truth the defect cases expect",()=>{const row=corpus.defects[0]!;expect(oracle(row.text,row.type).fields).toEqual(row.expected.fields);});
for(const row of corpus.modules.cases)test("module resolution: "+row.name,()=>{
 const files:Record<string,string>={};for(const[path,text]of Object.entries(corpus.modules.files))files[resolve(path)]=text;const module=resolve(row.module);
 expect<unknown>(policyExtractTypescriptSchemaFields(files[module]!,row.type,resolver(files),module)).toEqual(row.expected);
});
test("a module cycle terminates",()=>{
 const files:Record<string,string>={[resolve("/w/x.ts")]:'export { Loop as Loop } from "./y.ts";\n',[resolve("/w/y.ts")]:'export { Loop as Loop } from "./x.ts";\n'};
 expect(policyExtractTypescriptSchemaFields(files[resolve("/w/x.ts")]!,"Loop",resolver(files),resolve("/w/x.ts"))).toEqual({typeName:"",fields:[]});
});
test("a file-based extraction follows relative re-exports on disk and ignores package specifiers",()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"tsfields-")));
 try{
  for(const[path,text]of Object.entries(corpus.filePath.files))writeFileSync(join(root,path),text);
  const entry=join(root,corpus.filePath.entry);expect<unknown>(policyExtractTypescriptSchemaFile(entry,readFileSync(entry,"utf8"),corpus.filePath.type)).toEqual(corpus.filePath.expected);
  expect(policyExtractTypescriptSchemaFile(join(root,corpus.filePath.missing.entry),corpus.filePath.missing.text,corpus.filePath.missing.type)).toEqual({typeName:"",fields:[]});
  expect(policyExtractTypescriptSchemaFile(join(root,"pkg.ts"),corpus.filePath.packageSpecifier,"Pkg")).toEqual({typeName:"",fields:[]});
  mkdirSync(join(root,"dir.ts"));expect(policyExtractTypescriptSchemaFile(join(root,"z.ts"),'export { W as Z } from "./dir.ts";\n',"Z")).toEqual({typeName:"",fields:[]});
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
