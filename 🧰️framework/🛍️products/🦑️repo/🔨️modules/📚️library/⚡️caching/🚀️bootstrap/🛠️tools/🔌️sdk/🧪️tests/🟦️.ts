import {test,expect} from "bun:test";
import {createRequire} from "node:module";
import {existsSync,readFileSync,readdirSync} from "node:fs";
import {dirname,join} from "node:path";
import ts from "typescript";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {openNxSdkV1} from "../🟦️.ts";
import policy from "../🔣️policy.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};

const root=join(import.meta.dir,"../../../../../../../../../..").replace(/[\\/]+$/u,""),specifier=/^(?:nx|@nx\/[A-Za-z0-9._-]+|@nrwl\/[A-Za-z0-9._-]+)(?:\/[A-Za-z0-9._\/-]*)?$/u;

/** 🔎️ Own scanner: tokenizes comments and strings, then judges each string by the code that precedes it. */
function ownSpecifiers(source:string):string[]{
 const found:string[]=[],tokens=/\/\/[^\n]*|\/\*[\s\S]*?\*\/|(["'`])((?:\\[\s\S]|(?!\1)[^\\])*)\1|[^"'`\/]+|\//gu,trigger=/(?:\brequire\s*\(|\brequire\s*\.\s*resolve\s*\(|\bimport\s*\(|\bfrom\s*|\bimport\s*|\)\s*\(|(?:\)|\brequire|\bsdk)\s*\.\s*resolve\s*\()\s*$/u;
 let code="";
 for(const match of source.matchAll(tokens)){
  const text=match[0];
  if(text.startsWith("//")||text.startsWith("/*")){code+=" ";continue;}
  if(match[1]!==undefined){if(specifier.test(match[2]!)&&trigger.test(code))found.push(match[2]!);code+="\u0000";continue;}
  code=(code+text).slice(-80);
 }
 return found;
}

/** 🌳️ Third-party oracle: the TypeScript compiler's syntax tree. */
function oracleSpecifiers(source:string):string[]{
 const file=ts.createSourceFile("source.ts",source,ts.ScriptTarget.Latest,true),found:string[]=[],literal=(node:ts.Node|undefined)=>node&&(ts.isStringLiteral(node)||ts.isNoSubstitutionTemplateLiteral(node))?node.text:undefined;
 const visit=(node:ts.Node):void=>{
  if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier)found.push(literal(node.moduleSpecifier)!);
  if(ts.isCallExpression(node)){
   const callee=node.expression,first=literal(node.arguments[0]);
   if(first!==undefined&&(callee.kind===ts.SyntaxKind.ImportKeyword||ts.isIdentifier(callee)&&callee.text==="require"||ts.isCallExpression(callee)||ts.isPropertyAccessExpression(callee)&&callee.name.text==="resolve"&&(ts.isCallExpression(callee.expression)||ts.isIdentifier(callee.expression)&&["require","sdk"].includes(callee.expression.text))))found.push(first);
  }
  ts.forEachChild(node,visit);
 };
 visit(file);
 return found.filter(value=>specifier.test(value));
}

function productionFiles(directory:string):string[]{
 const files:string[]=[];
 for(const entry of readdirSync(directory,{withFileTypes:true})){
  if(["node_modules","🧪️tests","🗑️generated",".git"].includes(entry.name))continue;
  const path=join(directory,entry.name);
  if(entry.isDirectory())files.push(...productionFiles(path));
  else if(entry.isFile()&&/\.(?:ts|tsx|mts|cts|mjs|cjs|js)$/u.test(entry.name)&&!entry.name.endsWith(".d.ts"))files.push(path);
 }
 return files;
}

test("the Nx SDK port policy is a closed, schema-valid catalogue that the installed Nx can resolve",()=>{
 expect(validateJsonSchemaSubset(schema,policy)).toEqual([]);
 const installed=createRequire(join(root,"package.json")),port=readFileSync(join(import.meta.dir,"../🟦️.ts"),"utf8");
 for(const name of policy.modules){expect(()=>installed.resolve(name),name).not.toThrow();expect(port.includes(JSON.stringify(name)),name).toBe(true);}
 for(const path of [...policy.adapterRoots,...policy.scanned])expect(existsSync(join(root,path)),path).toBe(true);
});

test("the Nx SDK port opens exactly the doors Node itself resolves and refuses everything else",()=>{
 const base=join(root,"package.json"),installed=createRequire(base),sdk=openNxSdkV1(base);
 expect(sdk.packageDirectory()).toBe(dirname(installed.resolve(fixture.ports.packageDirectory)));
 expect(sdk.cli()).toBe(installed.resolve(fixture.ports.cli));
 expect(sdk.workspaceRoot).toBeInstanceOf(Function);
 const source=readFileSync(join(import.meta.dir,"../🟦️.ts"),"utf8");
 for(const name of fixture.refused)expect(policy.modules.includes(name),name).toBe(false);
 expect(oracleSpecifiers(source).filter(value=>!policy.modules.includes(value))).toEqual([]);
});

test("the own scanner agrees with the TypeScript syntax tree on every language-agnostic case",()=>{
 for(const row of fixture.scannerCases){expect(oracleSpecifiers(row.source),row.name).toEqual(row.specifiers);expect(ownSpecifiers(row.source),row.name).toEqual(row.specifiers);}
});

test("no production code outside the Nx implementation loads an Nx module",()=>{
 const adapters=policy.adapterRoots.map(path=>join(root,path)),offenders:string[]=[];
 for(const scanned of policy.scanned)for(const file of productionFiles(join(root,scanned))){
  if(adapters.some(adapter=>file===adapter||file.startsWith(adapter+"/")))continue;
  const source=readFileSync(file,"utf8"),own=ownSpecifiers(source),oracle=oracleSpecifiers(source);
  expect(own,file).toEqual(oracle);
  if(oracle.length)offenders.push(file.slice(root.length+1)+" -> "+oracle.join(", "));
 }
 expect(offenders).toEqual([]);
},120_000);
