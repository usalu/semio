/** 🔄️ Tests that the commands which attach the dependency store load without the store: every module they load at start is reachable by relative paths and system modules alone. */
import {expect,test} from "bun:test";
import {existsSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {builtinModules} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import ts from "typescript";

const door=resolve(import.meta.dir,"../../../../🚪️door/📜️script.ts"),dependencies=resolve(import.meta.dir,"../../🟦️.ts");
const system=new Set(builtinModules.flatMap(name=>[name,"node:"+name])),sourceFile=/\.(?:ts|mts|mjs|js)$/u;
const specifiers=(file:string):Readonly<{static:readonly string[];lazy:readonly string[]}>=>{
 const tree=ts.createSourceFile(file,readFileSync(file,"utf8").replace(/^#!.*/u,""),ts.ScriptTarget.Latest,false),statics:string[]=[],lazy:string[]=[];
 const visit=(node:ts.Node)=>{
  if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier)&&!(ts.isImportDeclaration(node)&&node.importClause?.isTypeOnly)&&!(ts.isExportDeclaration(node)&&node.isTypeOnly))statics.push(node.moduleSpecifier.text);
  if(ts.isCallExpression(node)&&node.expression.kind===ts.SyntaxKind.ImportKeyword&&node.arguments[0]&&ts.isStringLiteralLike(node.arguments[0]))lazy.push(node.arguments[0].text);
  ts.forEachChild(node,visit);
 };
 visit(tree);return{static:statics,lazy};
};
const closure=(entry:string)=>{
 const seen=new Set<string>(),bare=new Map<string,string>(),parent=new Map<string,string>();
 const walk=(file:string)=>{
  if(seen.has(file)||!sourceFile.test(file))return;seen.add(file);
  const found=specifiers(file);
  for(const [specifier,deferred] of [...found.static.map(value=>[value,false] as const),...found.lazy.map(value=>[value,true] as const)]){
   if(specifier.startsWith(".")){
    const target=resolve(dirname(file),specifier);
    if(existsSync(target)&&statSync(target).isFile()){if(!parent.has(target))parent.set(target,file);walk(target);}
   }else if(!deferred&&!system.has(specifier)&&!specifier.startsWith("bun:")&&!bare.has(specifier))bare.set(specifier,file);
  }
 };
 walk(entry);
 const chain=(file:string)=>{const rows=[file];while(parent.has(rows.at(-1)!))rows.push(parent.get(rows.at(-1)!)!);return rows.map(row=>row.slice(row.indexOf("🧰️framework")));};
 return{files:seen.size,bare:[...bare].map(([specifier,file])=>({specifier,chain:chain(file)}))};
};
test("the front door and the dependency command load without a single package that resolves through node_modules",()=>{
 for(const entry of[door,dependencies]){
  const result=closure(entry);
  expect(result.files).toBeGreaterThan(100);
  expect(result.bare,"a package imported at start needs node_modules, which dangles while the dependency store is detached and which this very command attaches again").toEqual([]);
 }
});
test("the scan sees what the loader sees: a package imported at start by a module on the path is reported with its chain, a lazy one is not",()=>{
 const directory=mkdtempSync(join(tmpdir(),"bc-"));
 try{
  writeFileSync(join(directory,"entry.ts"),'import {readFileSync} from "node:fs";\nimport "./inner.mjs";\nexport const later=()=>import("lazy-package");\nexport {readFileSync};\n');
  writeFileSync(join(directory,"inner.mjs"),'import pad from "left-pad";\nexport default pad;\n');
  const result=closure(join(directory,"entry.ts"));
  expect(result.bare.map(row=>row.specifier)).toEqual(["left-pad"]);
  expect(result.bare[0]!.chain.length).toBe(2);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
