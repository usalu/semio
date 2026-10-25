/** 🔀️ Contract: the imports a router script needs for one registered command are exactly the ones its corpus lists, they cover everything the stubbed execution of that command really touches (the execution-based oracle), and a router the reading does not fully understand keeps all of its imports; on the real scripts of the repository the reading never invents a specifier and prunes at least the routers the corpus bounds. The cases live in `./🔣️.json`. */
import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,normalize,resolve} from "node:path";
import Ajv from "ajv";
import {parserTypescriptV1} from "../../../../../🚀️bootstrap/📦️parsers/🟨️.mjs";
import {routerCommandSpecifiersV1,routerCommandWordV1} from "../🟨️.mjs";
import corpus from "./🔣️.json";
import corpusSchema from "./🧬️schema/🔣️.json";

setDefaultTimeout(60000);

const compiler=parserTypescriptV1(),checkout=resolve(import.meta.dir,corpus.checkoutFromTests);

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,extra:1})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,cases:[]})).toBe(false);
});

for(const item of corpus.cases)test(`router command ${item.case}: ${item.note}`,()=>{
 const kept=routerCommandSpecifiersV1(compiler,"router.ts",item.files["router.ts"]!,item.command);
 expect(kept===undefined?null:[...kept].sort()).toEqual(item.expected);
 if(kept===undefined)return;
 const area=mkdtempSync(join(tmpdir(),"router-"));
 try{
  for(const [name,text] of Object.entries(item.files)){mkdirSync(dirname(join(area,name)),{recursive:true});writeFileSync(join(area,name),text);}
  writeFileSync(join(area,"run.ts"),'await import("./router.ts");console.log(JSON.stringify(globalThis.__touched??[]));\n');
  const run=spawnSync("bun",[join(area,"run.ts"),item.command],{encoding:"utf8",cwd:area});
  expect(run.status,run.stderr).toBe(0);
  const touched=JSON.parse(run.stdout.trim().split("\n").at(-1)!) as string[];
  expect(touched.length,"the selected command does something observable").toBeGreaterThan(0);
  for(const specifier of touched)expect(kept.has(specifier),specifier+" is touched by the command and must be kept").toBe(true);
 }finally{rmSync(area,{recursive:true,force:true});}
});

test("a launcher command line names its registered command behind the script, unless it is an option",()=>{
 const script="/w/p/📜️script.ts",bound=(args:string[])=>({script,arguments:args});
 expect(routerCommandWordV1(bound([script,"generate"]))).toBe("generate");
 expect(routerCommandWordV1(bound(["--cwd","x",script,"build","--flag"]))).toBe("build");
 expect(routerCommandWordV1(bound([script]))).toBeUndefined();
 expect(routerCommandWordV1(bound([script,"--watch"]))).toBeUndefined();
 expect(routerCommandWordV1({script:null,arguments:["x"]})).toBeUndefined();
});

test("on the real scripts of the repository the reading only removes specifiers and prunes the routers the corpus bounds",()=>{
 const listed=spawnSync("git",["ls-files","-z","--",...corpus.scriptRootsFromTests.map(root=>root+"/**/📜️script.ts")],{cwd:checkout,encoding:"utf8",maxBuffer:1<<26});
 expect(listed.status,listed.stderr).toBe(0);
 let pruned=0,routers=0;
 for(const path of listed.stdout.split("\0").filter(Boolean)){
  const text=readFileSync(join(checkout,path),"utf8");
  const names=[...text.matchAll(/\.register(?:Lazy)?\(\s*["']([^"']+)["']/gu)].map(match=>match[1]!);
  if(!names.length)continue;
  routers++;
  const everything=new Set<string>(),tree=compiler.createSourceFile(path,text,compiler.ScriptTarget.Latest,true),visit=(node:any):void=>{if(compiler.isStringLiteralLike(node))everything.add(node.text);compiler.forEachChild(node,visit);};visit(tree);
  for(const name of new Set(names)){
   const kept=routerCommandSpecifiersV1(compiler,path,text,name);
   if(kept===undefined)continue;
   for(const specifier of kept)expect(everything.has(specifier),path+" "+name+" "+specifier).toBe(true);
   if([...everything].some(specifier=>specifier.startsWith(".")&&!kept.has(specifier)))pruned++;
  }
 }
 expect(routers).toBeGreaterThan(0);
 expect(pruned).toBeGreaterThanOrEqual(corpus.minimumPrunedRouters);
});

test("a real router command reaches fewer files than its whole script, never invents one, and reaches scheduling code only through declared ports",async()=>{
 const inference=await import("../../../🟨️.mjs") as any;await inference.libraryBootstrap;
 const scripts=await import("../../🟨️.mjs") as any,{buildSchedulingPortsV1,buildSchedulingPrefixesV1}=await import("../../../../../📥️source/🔏️closure/🗓️scheduling/🟦️.ts"),prefixes=buildSchedulingPrefixesV1(),ports=buildSchedulingPortsV1();
 const specifiers=(path:string):string[]=>{const found:string[]=[],tree=compiler.createSourceFile(path,readFileSync(path,"utf8"),compiler.ScriptTarget.Latest,true),visit=(node:any):void=>{if((compiler.isImportDeclaration(node)||compiler.isExportDeclaration(node))&&node.moduleSpecifier&&compiler.isStringLiteral(node.moduleSpecifier))found.push(node.moduleSpecifier.text);if(compiler.isCallExpression(node)&&node.arguments[0]&&compiler.isStringLiteralLike(node.arguments[0])&&(node.expression.kind===compiler.SyntaxKind.ImportKeyword||compiler.isIdentifier(node.expression)&&node.expression.text==="require"))found.push(node.arguments[0].text);compiler.forEachChild(node,visit);};visit(tree);return found.filter(specifier=>specifier.startsWith("."));};
 for(const item of corpus.realCommands){
  const entry=resolve(checkout,item.script),whole=new Set<string>((await scripts.relativeScriptInputs([entry],checkout,inference.createScriptInputCache({workspaceRoot:checkout}),true)).map((path:string)=>path.replace("{workspaceRoot}/",""))),pruned=new Set<string>((await scripts.relativeScriptInputs([entry],checkout,inference.createScriptInputCache({workspaceRoot:checkout}),true,[],new Map([[entry,[item.command]]]))).map((path:string)=>path.replace("{workspaceRoot}/","")));
  expect(pruned.size,item.script+" "+item.command).toBeLessThanOrEqual(item.maximumFiles);
  expect(pruned.size,item.script+" "+item.command).toBeLessThanOrEqual(whole.size);
  for(const path of pruned)expect(whole.has(path),path).toBe(true);
  const stray:string[]=[];
  if(item.ports)for(const path of pruned){
   if(prefixes.some(prefix=>path.startsWith(prefix))||!/\.[cm]?[jt]sx?$/u.test(path))continue;
   for(const specifier of specifiers(join(checkout,path))){const target=normalize(join(dirname(path),specifier));if(pruned.has(target)&&prefixes.some(prefix=>target.startsWith(prefix))&&!ports.has(target))stray.push(path+" -> "+target);}
  }
  expect(stray,item.script+" "+item.command).toEqual([]);
 }
});
