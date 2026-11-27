import {expect,test} from "bun:test";
import Ajv from "ajv";
import ts from "typescript";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {spawn} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import {CATALOG_LANGUAGES_V1,placeholdersV1,textV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import policy from "../🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import catalogue from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {declarationDriftV1,declarationExportsV1,declarationStubsV1,scriptExportsV1} from "../🧾️declarations/🟦️.ts";
import {importKeysV1,planTypecheckV1,readTypecheckPolicyV1,renderTypecheckResultV1,resolveTypecheckPathsV1,runTypecheckV1,typecheckAllPairsV1,typecheckArgumentsV1,typecheckCacheDirectoryV1,typecheckHeapMegabytesV1,typecheckConfigurationV1,typecheckDiagnosticsV1,typecheckDriftV1,typecheckImportersV1,typecheckOptionNamesV1,typecheckProjectOfV1,validateTypecheckDocumentV1,type TypecheckCompileV1} from "../🟦️.ts";
import {TYPECHECK_TEXTS_V1,typecheckTextV1} from "../📖️texts/🟦️.ts";
import {runTypecheckCliV1} from "../⌨️command/🟦️.ts";

const root=(()=>{let directory=import.meta.dir;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);return directory;})();
const ajv=new Ajv({strict:false,allowUnionTypes:true}),validator=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const refusal=(run:()=>unknown):string|undefined=>{try{run();return undefined;}catch(error){return String((error as {key?:string}).key).replace(/^typecheck\./u,"");}};
const scratch=(run:(directory:string)=>void|Promise<void>)=>async()=>{const directory=mkdtempSync(join(tmpdir(),"semio-tc-"));try{await run(directory);}finally{rmSync(directory,{recursive:true,force:true});}};
const write=(directory:string,file:string,text:string)=>{mkdirSync(dirname(join(directory,file)),{recursive:true});writeFileSync(join(directory,file),text);};
const miniPolicy=(overrides:Record<string,unknown>={})=>({...policy,projects:[{id:"mini",roots:["lib"],tsconfig:"tsconfig.json"}],scanRoots:["lib"],...overrides}) as typeof policy;

test("the policy satisfies its closed schema under the in-repo validator and under Ajv, and both reject the same mutations",()=>{
 const check=validator("Policy");
 expect(check(policy),JSON.stringify(check.errors?.[0])).toBe(true);
 expect(validateTypecheckDocumentV1("Policy",policy)).toEqual([]);
 expect(readTypecheckPolicyV1()).toBe(policy as never);
 const mutations:Record<string,(value:any)=>void>={extra:value=>{value.extra=1;},noProjects:value=>{delete value.projects;},badProject:value=>{value.projects[0].id="Nope";},noRoots:value=>{value.projects[0].roots=[];},badLimit:value=>{value.limits.maximumFiles=0;},badBudget:value=>{value.budgetMs=5;},badVariable:value=>{value.cache.variable="lower";},badExtension:value=>{value.checkExtensions=["ts"];}};
 for(const [name,mutate] of Object.entries(mutations)){
  const copy=JSON.parse(JSON.stringify(policy));
  mutate(copy);
  expect(check(copy),name).toBe(false);
  expect(validateTypecheckDocumentV1("Policy",copy).length,name).toBeGreaterThan(0);
 }
});

test("every declared root, project configuration, scan root and the compiler script exist in the checkout",()=>{
 for(const project of policy.projects){expect(existsSync(join(root,project.tsconfig)),project.tsconfig).toBe(true);for(const projectRoot of project.roots)expect(existsSync(join(root,projectRoot)),projectRoot).toBe(true);}
 expect(existsSync(join(root,policy.fallback.tsconfig))).toBe(true);
 for(const scan of policy.scanRoots)expect(existsSync(join(root,scan)),scan).toBe(true);
 expect(existsSync(join(root,policy.compiler.script))).toBe(true);
 expect(typecheckProjectOfV1(policy.projects[0]!.roots[0]+"/x/🟦️.ts").id).toBe("library");
 expect(typecheckProjectOfV1("🧰️framework/📦️packages/🟦️typescript/🟦️.ts").id).toBe(policy.fallback.id);
});

test("the texts exist in English and German with the same placeholders, and the catalogue states the operation, its flags and its examples in both languages",()=>{
 for(const [key,message] of Object.entries(texts.messages)){
  expect(CATALOG_LANGUAGES_V1.map(language=>[...placeholdersV1((message as Record<string,string>)[language]!)].sort().join(",")).filter((value,index,all)=>value!==all[0]),key).toEqual([]);
  for(const language of CATALOG_LANGUAGES_V1)expect(textV1(TYPECHECK_TEXTS_V1.catalog,key as never,language,Object.fromEntries(placeholdersV1((message as Record<string,string>)[language]!).map(name=>[name,"x"])))).toBeTypeOf("string");
 }
 const operation=catalogue.operations.find(row=>row.name==="typecheck")!;
 expect(operation.requires).toBe("none");
 expect(operation.summary.en&&operation.summary.de).toBeTruthy();
 expect(operation.examples.length).toBeGreaterThan(1);
 for(const example of operation.examples)expect(example.description.en&&example.description.de).toBeTruthy();
 for(const note of operation.notes??[])expect(note.en&&note.de).toBeTruthy();
 const names=typecheckOptionNamesV1();
 expect([...operation.flags].sort()).toEqual([...names.plain].sort());
 for(const flag of operation.flags){const row=catalogue.flags.find(candidate=>candidate.name===flag&&(candidate.operations as readonly string[]).includes("typecheck"));expect(row,flag).toBeDefined();expect(row!.description.en&&row!.description.de).toBeTruthy();}
 expect(operation.synopsis).toContain("<path>");
});

test("arguments: flags, paths after --, language, and every refusal is data in both languages",()=>{
 const parsed=typecheckArgumentsV1(["--json","--no-importers","--lang=de","a.ts","--","--odd.ts","a.ts"]);
 expect(parsed).toEqual({paths:["a.ts","--odd.ts"],json:true,dryRun:false,noImporters:true,allErrors:false,declarations:false,language:"de"});
 expect(typecheckArgumentsV1(["--declarations"]).paths).toEqual([]);
 expect(refusal(()=>typecheckArgumentsV1([]))).toBe("usage");
 expect(refusal(()=>typecheckArgumentsV1(["--nope","a.ts"]))).toBe("option.unknown");
 expect(refusal(()=>typecheckArgumentsV1(["--json=1","a.ts"]))).toBe("option.unknown");
 expect(refusal(()=>typecheckArgumentsV1(["--lang"]))).toBe("option.value");
 expect(refusal(()=>typecheckArgumentsV1(["--lang=fr","a.ts"]))).toBe("language.unknown");
 expect(refusal(()=>typecheckArgumentsV1(Array.from({length:policy.limits.maximumFiles+1},(_,index)=>"f"+index+".ts")))).toBe("files.too-many");
});

test("path resolution: a file, a folder, a script standing for its declaration, and refusals",scratch(directory=>{
 write(directory,"MODULE.bazel","");
 write(directory,"lib/a/🟦️.ts","export const a=1;");
 write(directory,"lib/a/🟨️.mjs","export const b=1;");
 write(directory,"lib/a/🟨️.d.mts","export const b:1;");
 write(directory,"lib/c/🟨️.mjs","export const c=1;");
 write(directory,"lib/node_modules/x/🟦️.ts","");
 write(directory,"notes.md","");
 const limits=miniPolicy(),resolved=resolveTypecheckPathsV1(directory,directory,["lib/a"],limits);
 expect(resolved.check).toEqual(["lib/a/🟦️.ts","lib/a/🟨️.d.mts"]);
 expect(resolved.pairs).toEqual([{script:"lib/a/🟨️.mjs",declaration:"lib/a/🟨️.d.mts"}]);
 expect(resolveTypecheckPathsV1(directory,join(directory,"lib"),["a/🟨️.mjs"],limits).check).toEqual(["lib/a/🟨️.d.mts"]);
 expect(resolveTypecheckPathsV1(directory,directory,["lib/c/🟨️.mjs"],limits)).toEqual({check:[],pairs:[]});
 expect(refusal(()=>resolveTypecheckPathsV1(directory,directory,["nope.ts"],limits))).toBe("path.missing");
 expect(refusal(()=>resolveTypecheckPathsV1(directory,directory,["notes.md"],limits))).toBe("path.nothing");
 expect(refusal(()=>resolveTypecheckPathsV1(directory,directory,[tmpdir()],limits))).toBe("path.outside");
}));

test("import keys: relative specifiers of every import form resolve to extensionless keys where .mjs, .d.mts and .ts meet; bare packages are ignored",()=>{
 for(const row of corpus.imports)expect([...importKeysV1(row.file,row.text)].sort()).toEqual([...row.keys].sort());
});

test("importers are found, indexed in the cache and re-read only when a file changed",scratch(directory=>{
 write(directory,"lib/leaf/🟦️.ts","export const leaf=1;");
 write(directory,"lib/user/🟦️.ts","import {leaf} from \"../leaf/🟦️.ts\";export const user=leaf;");
 write(directory,"lib/script/🟨️.d.mts","export const s:number;");
 write(directory,"lib/via/🟦️.ts","import {s} from \"../script/🟨️.mjs\";export const v=s;");
 write(directory,"lib/other/🟦️.ts","export const o=1;");
 const limits=miniPolicy(),cache=join(directory,"cache");
 expect(typecheckImportersV1(directory,["lib/leaf/🟦️.ts"],cache,limits)).toEqual(["lib/user/🟦️.ts"]);
 expect(typecheckImportersV1(directory,["lib/script/🟨️.d.mts"],cache,limits)).toEqual(["lib/via/🟦️.ts"]);
 const index=JSON.parse(readFileSync(join(cache,"imports.json"),"utf8"));
 expect(Object.keys(index).length).toBe(5);
 write(directory,"lib/other/🟦️.ts","import {leaf} from \"../leaf/🟦️.ts\";export const o=leaf;");
 expect(typecheckImportersV1(directory,["lib/leaf/🟦️.ts"],cache,limits)).toEqual(["lib/other/🟦️.ts","lib/user/🟦️.ts"]);
}));

test("the plan groups files by project, drops the importers beyond the limit and honours --no-importers",scratch(directory=>{
 write(directory,"lib/leaf/🟦️.ts","export const leaf=1;");
 for(let index=0;index<3;index++)write(directory,`lib/u${index}/🟦️.ts`,"import {leaf} from \"../leaf/🟦️.ts\";");
 const limits=miniPolicy({limits:{...policy.limits,maximumImporters:2}}),touched={check:["lib/leaf/🟦️.ts"],pairs:[]},cache=join(directory,"cache");
 const plan=planTypecheckV1(directory,touched,{noImporters:false},cache,limits);
 expect(plan.truncated).toBe(true);
 expect(plan.found).toBe(3);
 expect(plan.files).toEqual(["lib/leaf/🟦️.ts"]);
 const roomy=planTypecheckV1(directory,touched,{noImporters:false},cache,miniPolicy());
 expect(roomy.files.length).toBe(4);
 expect(roomy.groups.get("mini")!.files.length).toBe(4);
 expect(planTypecheckV1(directory,touched,{noImporters:true},cache,miniPolicy()).files).toEqual(["lib/leaf/🟦️.ts"]);
 expect(validateTypecheckDocumentV1("Plan",{version:1,touched:roomy.touched,importers:roomy.importers,truncated:roomy.truncated,project:roomy.project,files:roomy.files})).toEqual([]);
}));

test("the configuration extends the project, writes its build state into the machine cache and names exactly the files",()=>{
 const configuration=JSON.parse(typecheckConfigurationV1("/checkout",{id:"library",tsconfig:"a/tsconfig.json"},["x/🟦️.ts","y/🟦️.ts"],"/cache/dir",policy));
 expect(configuration).toEqual({extends:"/checkout/a/tsconfig.json",compilerOptions:{noEmit:true,incremental:true,tsBuildInfoFile:"/cache/dir/library.tsbuildinfo",typeRoots:["/checkout/node_modules/@types"],pretty:false},include:[],files:["/checkout/x/🟦️.ts","/checkout/y/🟦️.ts"]});
});

test("the machine cache directory follows the variable first, then the platform's user cache, one folder per checkout",()=>{
 expect(typecheckCacheDirectoryV1("/a",{SEMIO_TYPECHECK_CACHE:"/c"},policy,"linux","/h")).toBe(typecheckCacheDirectoryV1("/a",{SEMIO_TYPECHECK_CACHE:"/c"},policy,"darwin","/h"));
 expect(typecheckCacheDirectoryV1("/a",{},policy,"darwin","/h")).toMatch(/^\/h\/Library\/Caches\/semio\/typecheck\/[0-9a-f]{12}$/u);
 expect(typecheckCacheDirectoryV1("/a",{XDG_CACHE_HOME:"/x"},policy,"linux","/h")).toMatch(/^\/x\/semio\/typecheck\/[0-9a-f]{12}$/u);
 expect(typecheckCacheDirectoryV1("/a",{LOCALAPPDATA:"C:/L"},policy,"win32","/h").replaceAll("\\","/")).toMatch(/^C:\/L\/semio\/typecheck\/[0-9a-f]{12}$/u);
 expect(typecheckCacheDirectoryV1("/a",{},policy,"linux","/h")).not.toBe(typecheckCacheDirectoryV1("/b",{},policy,"linux","/h"));
});

test("compiler output becomes diagnostics: continuation lines join, files outside the named set are flagged, file-less errors are kept",()=>{
 const rows=typecheckDiagnosticsV1(corpus.diagnostics.output,"/root",new Set(corpus.diagnostics.checked),policy);
 expect(rows).toEqual(corpus.diagnostics.rows as never);
 expect(typecheckDiagnosticsV1("/root/a/b.ts(1,2): error TS1: m\n","/root",new Set(["a/b.ts"]),policy)[0]!.file).toBe("a/b.ts");
});

test("declaration drift: every script export is declared, no phantom function, constant or class, equal parameter counts, comments and strings ignored",()=>{
 for(const row of corpus.pairs){
  const found=declarationDriftV1(row.script,row.declaration).map(item=>item.kind==="arity"?[item.name,item.kind,item.expected,item.found]:[item.name,item.kind]);
  expect(found,row.id).toEqual(row.drift as never);
 }
});

test("generated stubs close the drift they are made for and keep the arity of the script",()=>{
 const script="export function alpha(a,b=1,{c},...rest){}\nexport const beta=(x,y)=>x;\nexport const gamma=3;\nexport class Delta{}\nexport default function(a){}\n",declaration="export const unrelated:1;\n";
 const names=declarationDriftV1(script,declaration).filter(row=>row.kind==="undeclared").map(row=>row.name),stubs=declarationStubsV1(script,names);
 expect(stubs).toContain("export function alpha(a: any, b: any, argument3: any, ...rest: any[]): any;");
 expect(declarationDriftV1(script,declaration+stubs).filter(row=>row.kind!=="phantom")).toEqual([]);
 expect(declarationStubsV1(script,[])).toBe("");
 const barrel="export{x as y,z}from\"./m.mjs\";\nexport{w}from'./n.mjs';\n",rows=declarationDriftV1(barrel,"").map(row=>row.name),barrelStubs=declarationStubsV1(barrel,rows);
 expect(barrelStubs).toBe("export {w} from \"./n.mjs\";\nexport {x as y, z} from \"./m.mjs\";\n");
 expect(declarationDriftV1(barrel,barrelStubs)).toEqual([]);
});

const typeScriptExports=(text:string,file:string):Map<string,string>=>{
 const sourceFile=ts.createSourceFile(file,text,ts.ScriptTarget.ESNext,true),table=new Map<string,Set<string>>(),add=(name:string,value:string)=>{(table.get(name)??table.set(name,new Set()).get(name)!).add(value);};
 const modifiers=(node:ts.Node)=>ts.canHaveModifiers(node)?ts.getModifiers(node)??[]:[],exported=(node:ts.Node)=>modifiers(node).some(item=>item.kind===ts.SyntaxKind.ExportKeyword),defaulted=(node:ts.Node)=>modifiers(node).some(item=>item.kind===ts.SyntaxKind.DefaultKeyword);
 const arity=(parameters:readonly ts.ParameterDeclaration[])=>{const own=parameters.filter(item=>!(ts.isIdentifier(item.name)&&item.name.text==="this"));return own.filter(item=>!item.dotDotDotToken).length+(own.some(item=>item.dotDotDotToken)?"+":"");};
 for(const statement of sourceFile.statements){
  if(ts.isFunctionDeclaration(statement)&&exported(statement))add(defaulted(statement)?"default":statement.name?.text??"?","fn:"+arity(statement.parameters));
  else if(ts.isClassDeclaration(statement)&&exported(statement))add(defaulted(statement)?"default":statement.name!.text,"class");
  else if(ts.isVariableStatement(statement)&&exported(statement)){
   for(const declaration of statement.declarationList.declarations){
    if(!ts.isIdentifier(declaration.name))continue;
    const initializer=declaration.initializer;
    if(initializer&&(ts.isArrowFunction(initializer)||ts.isFunctionExpression(initializer)))add(declaration.name.text,"fn:"+arity(initializer.parameters));
    else if(declaration.type&&ts.isFunctionTypeNode(declaration.type))add(declaration.name.text,"fn:"+arity(declaration.type.parameters));
    else add(declaration.name.text,"value");
   }
  }else if(ts.isExportDeclaration(statement)){
   if(statement.exportClause&&ts.isNamedExports(statement.exportClause))for(const item of statement.exportClause.elements)add(item.name.text,statement.isTypeOnly||item.isTypeOnly?"type":"reexport");
   else if(statement.exportClause&&ts.isNamespaceExport(statement.exportClause))add(statement.exportClause.name.text,"reexport");
   else add("*","star");
  }else if(ts.isExportAssignment(statement))add("default","value");
  else if((ts.isInterfaceDeclaration(statement)||ts.isTypeAliasDeclaration(statement)||ts.isEnumDeclaration(statement)||ts.isModuleDeclaration(statement))&&exported(statement))add((statement as any).name.text,"type");
 }
 return new Map([...table].map(([name,values])=>[name,[...new Set([...values].map(value=>value.startsWith("fn:")?"fn":value))].sort().join(",")+"/"+[...new Set([...values].filter(value=>value.startsWith("fn:")).flatMap(value=>value.slice(3).split("|")))].sort().join("|")]));
};
const sources=(directory:string):string[]=>readdirSync(join(root,directory),{withFileTypes:true}).flatMap(entry=>policy.skipDirectories.includes(entry.name)?[]:entry.isDirectory()?sources(join(directory,entry.name)):entry.name.endsWith(".mjs")||entry.name.endsWith(".d.mts")?[join(directory,entry.name)]:[]);
const ours=(table:ReturnType<typeof scriptExportsV1>):Map<string,string>=>{
 const rows=new Map<string,string>();
 for(const [name,row] of table.names)rows.set(name,[...new Set(row.kinds.map(kind=>kind==="function"?"fn":kind))].sort().join(",")+"/"+[...new Set(row.arities.map(arity=>arity.count+(arity.rest?"+":"")))].sort().join("|"));
 if(table.star)rows.set("*","star/");
 return rows;
};

test("the declaration scanner reads every script and declaration of the build library and the process module exactly as the TypeScript parser does (names, kinds, parameter counts)",()=>{
 let files=0;
 const failures:string[]=[];
 for(const scan of policy.scanRoots)for(const file of sources(scan)){
  const text=readFileSync(join(root,file),"utf8"),declaration=file.endsWith(".d.mts"),expected=typeScriptExports(text,file),found=ours(declaration?declarationExportsV1(text):scriptExportsV1(text));
  files++;
  for(const name of new Set([...expected.keys(),...found.keys()])){
   const left=expected.get(name),right=found.get(name);
   if(left===undefined&&right==="type/"||left==="type/"&&right===undefined)continue;
   if(left!==right)failures.push(`${file}: ${name}: ${left} != ${right}`);
  }
 }
 expect(failures).toEqual([]);
 expect(files).toBeGreaterThan(200);
});

test("DECLARATION DRIFT LAW: every hand-written .d.mts of the library declares every export of its .mjs, no function, constant or class the script lacks, with the same parameter count",()=>{
 const pairs=typecheckAllPairsV1(root);
 expect(pairs.length).toBeGreaterThan(200);
 const rows=typecheckDriftV1(root,pairs).map(row=>`${row.declaration}: ${row.kind} ${row.name}${row.expected===undefined?"":" "+row.found+"!="+row.expected}`);
 expect(rows).toEqual([]);
});

const compileWith=(output:string,status=1):TypecheckCompileV1=>async()=>({status,output});

test("a run fails on an error in a named file or importer, not on an error elsewhere, unless every error counts; drift fails it too",scratch(async directory=>{
 write(directory,"lib/a/🟦️.ts","export const a=1;");
 write(directory,"lib/b/🟦️.ts","import {a} from \"../a/🟦️.ts\";export const b=a;");
 write(directory,"node_modules/typescript/lib/tsc.js","");
 write(directory,"tsconfig.json","{}");
 const limits=miniPolicy({compiler:{...policy.compiler,script:"node_modules/typescript/lib/tsc.js",typeRoots:"node_modules/@types"}}),touched={check:["lib/a/🟦️.ts"],pairs:[]},options=typecheckArgumentsV1(["lib/a/🟦️.ts"],limits),cache=join(directory,"cache"),lines:string[]=[];
 const run=(output:string,more:Partial<typeof options>={},status=1)=>runTypecheckV1(directory,touched,{...options,...more},"en",{cacheDirectory:cache,compile:compileWith(output,status),notice:line=>lines.push(line)},limits);
 const clean=await run("",{},0);
 expect(clean.status).toBe("passed");
 expect([clean.files,clean.importers]).toEqual([2,1]);
 expect(validateTypecheckDocumentV1("Result",clean)).toEqual([]);
 const bad=await run("lib/b/🟦️.ts(1,1): error TS2305: nope\n");
 expect([bad.status,bad.errors,bad.elsewhere]).toEqual(["failed",1,0]);
 expect(renderTypecheckResultV1(bad,"de")).toContain("Fehler TS2305");
 expect(renderTypecheckResultV1(bad,"en")).toContain("FAILED");
 const elsewhere=await run("zzz/other.ts(1,1): error TS2305: nope\n");
 expect([elsewhere.status,elsewhere.errors,elsewhere.elsewhere]).toEqual(["passed",0,1]);
 const strict=await run("zzz/other.ts(1,1): error TS2305: nope\n",{allErrors:true});
 expect([strict.status,strict.errors]).toEqual(["failed",1]);
 write(directory,"lib/a/🟨️.mjs","export function f(a){}\n");
 write(directory,"lib/a/🟨️.d.mts","export function f():void;\n");
 const drift=await runTypecheckV1(directory,{check:["lib/a/🟨️.d.mts"],pairs:[{script:"lib/a/🟨️.mjs",declaration:"lib/a/🟨️.d.mts"}]},options,"en",{cacheDirectory:cache,compile:compileWith("",0),notice:()=>{}},limits);
 expect([drift.status,drift.drift.map(row=>row.kind)]).toEqual(["failed",["arity"]]);
 expect(renderTypecheckResultV1(drift,"en")).toContain("takes 0 parameters there but 1");
 expect(lines.some(line=>line.includes("checking 2 files"))).toBe(true);
 expect(JSON.parse(readFileSync(join(cache,"mini.tsconfig.json"),"utf8")).compilerOptions.incremental).toBe(true);
 const refused=await runTypecheckV1(directory,touched,options,"en",{cacheDirectory:cache,compile:async()=>({status:null,output:""}),notice:()=>{}},limits).catch(error=>error);
 expect(refused.key).toBe("typecheck.compiler.timeout");
 const crashed=await runTypecheckV1(directory,touched,options,"en",{cacheDirectory:cache,compile:async()=>({status:2,output:"boom"}),notice:()=>{}},limits).catch(error=>error);
 expect(crashed.key).toBe("typecheck.compiler.failed");
 const controller=new AbortController();controller.abort();
 const cancelled=await runTypecheckV1(directory,touched,options,"en",{cacheDirectory:cache,compile:compileWith(""),signal:controller.signal,notice:()=>{}},limits).catch(error=>error);
 expect(cancelled.key).toBe("typecheck.cancelled");
}));

test("the command: dry run prints the plan, a missing compiler is refused with exit 2 in both languages, unknown paths exit 2",scratch(async directory=>{
 write(directory,"MODULE.bazel","");
 write(directory,"lib/a/🟦️.ts","export const a=1;");
 const out:string[]=[],notes:string[]=[],cache=join(directory,"cache");
 const run=async(args:string[],language="en")=>{out.length=0;notes.length=0;return runTypecheckCliV1(directory,args,{SEMIO_LANGUAGE:language,SEMIO_TYPECHECK_CACHE:cache},line=>out.push(line),{directory,notice:line=>notes.push(line)});};
 expect(await run(["--dry-run","--json","lib/a/🟦️.ts"])).toBe(0);
 expect(JSON.parse(out.join("\n"))).toMatchObject({version:1,touched:["lib/a/🟦️.ts"],truncated:false});
 expect(await run(["--dry-run","lib/a/🟦️.ts"])).toBe(0);
 expect(out.join("\n")).toBe("touched  lib/a/🟦️.ts");
 expect(await run(["lib/a/🟦️.ts"])).toBe(2);
 expect(notes.join("\n")).toMatch(/TypeScript compiler is not installed/u);
 expect(await run(["lib/a/🟦️.ts"],"de")).toBe(2);
 expect(notes.join("\n")).toMatch(/TypeScript-Compiler fehlt/u);
 expect(await run(["nope.ts"])).toBe(2);
 expect(await run([])).toBe(2);
}));

test("one real run: the compiler checks a file with a type error and a clean file in the machine cache, the second call reuses the build state, and nothing in the checkout is written",scratch(async directory=>{
 write(directory,"MODULE.bazel","");
 write(directory,"tsconfig.json",JSON.stringify({compilerOptions:{strict:true,noEmit:true,target:"ESNext",module:"ESNext",moduleResolution:"bundler",allowImportingTsExtensions:true,types:[]}}));
 write(directory,"lib/ok/🟦️.ts","export const ok:number=1;");
 write(directory,"lib/bad/🟦️.ts","import {ok} from \"../ok/🟦️.ts\";export const bad:string=ok;");
 const compiler=join(root,policy.compiler.script),limits=miniPolicy({compiler:{...policy.compiler,script:relative(directory,compiler),typeRoots:"types"}}),cache=join(directory,"cache"),options=typecheckArgumentsV1(["lib/ok/🟦️.ts"],limits);
 const first=await runTypecheckV1(directory,{check:["lib/ok/🟦️.ts"],pairs:[]},options,"en",{cacheDirectory:cache},limits);
 expect(first.status).toBe("failed");
 expect(first.diagnostics.map(row=>[row.file,row.code])).toEqual([["lib/bad/🟦️.ts","TS2322"]]);
 expect(existsSync(join(cache,"mini.tsbuildinfo"))).toBe(true);
 const second=await runTypecheckV1(directory,{check:["lib/ok/🟦️.ts"],pairs:[]},{...options,noImporters:true},"en",{cacheDirectory:cache},limits);
 expect(second.status).toBe("passed");
 expect(existsSync(resolve(directory,"lib","ok","🟦️.js"))).toBe(false);
 expect(readFileSync(join(directory,"lib/ok/🟦️.ts"),"utf8")).toBe("export const ok:number=1;");
}));
test("the compiler heap follows the declared budget: share of the host, between the declared minimum and maximum",()=>{
 const heap=policy.compiler.heap;
 expect(typecheckHeapMegabytesV1(policy,1_000_000)).toBe(heap.maximumMegabytes);
 expect(typecheckHeapMegabytesV1(policy,1)).toBe(heap.minimumMegabytes);
 const host=Math.floor((heap.minimumMegabytes+heap.maximumMegabytes)/2/heap.hostShare);
 expect(typecheckHeapMegabytesV1(policy,host)).toBe(Math.floor(host*heap.hostShare));
});

import {readTypecheckLibraryReportV1,runTypecheckLibraryV1,summarizeTypecheckLibraryV1,typecheckAreaOfV1} from "../🟦️.ts";
test("the whole-library report counts every error per area, product and tests apart, equals an independent count of the compiler output, validates under Ajv and round-trips through the machine cache",scratch(async directory=>{
 const base=policy.report.areaRoots.find(row=>row.label==="build")!.prefix,process=policy.report.areaRoots.find(row=>row.label==="process")!.prefix;
 const lines=[base+"/🕸️graph/🟦️.ts(1,2): error TS2322: a",base+"/🕸️graph/🧪️tests/🟦️.ts(3,4): error TS2345: b",base+"/🕸️graph/🧪️tests/🟦️.ts(5,6): error TS2345: c",process+"/🎛️owned-execution/🟦️.ts(7,8): error TS2345: d","elsewhere/🟦️.ts(1,1): error TS1005: e"];
 const output=lines.join("\n")+"\n  continuation of e\n";
 write(directory,"node_modules/typescript/lib/tsc.js","");
 const cache=join(directory,"cache"),notices:string[]=[];
 const library=await runTypecheckLibraryV1(directory,"en",{cacheDirectory:cache,notice:line=>notices.push(line),now:()=>1000,compile:async request=>{expect(request.configuration.startsWith(cache)).toBe(true);return {status:2,output};}},{...policy,projects:[{...policy.projects[0]!,tsconfig:"tsconfig.json"}]} as typeof policy);
 expect(library.errors).toBe(lines.length);
 expect(library.product).toBe(lines.filter(line=>!line.includes("/🧪️tests/")).length);
 expect(library.tests).toBe(2);
 expect(Object.fromEntries(library.areas.map(row=>[row.area,[row.errors,row.product]]))).toEqual({"build/🕸️graph":[3,1],"process/🎛️owned-execution":[1,1],other:[1,1]});
 expect(library.areas[0]!.area).toBe("build/🕸️graph");
 expect(validator("LibraryReport")(library)).toBe(true);
 expect(validateTypecheckDocumentV1("LibraryReport",library)).toEqual([]);
 expect(validateTypecheckDocumentV1("LibraryReport",{...library,errors:-1}).length).toBeGreaterThan(0);
 expect(readTypecheckLibraryReportV1(cache)).toEqual(library);
 expect(readTypecheckLibraryReportV1(join(directory,"missing"))).toBeUndefined();
 write(cache,policy.report.file,"{\"version\":1}");
 expect(readTypecheckLibraryReportV1(cache)).toBeUndefined();
 expect(typecheckAreaOfV1(base+"/🟦️.ts")).toBe("build");
 expect(typecheckAreaOfV1("nowhere/x.ts")).toBe("other");
 expect(summarizeTypecheckLibraryV1([],5,0.001).areas).toEqual([]);
 expect(notices.length).toBe(1);
}));

const S="🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server";
test("AMBIENT LAW policy: every ambient declaration exists, names known projects, and the Next.js modules are routed to their own project while the coordinator builder, its tests and its scripts stay out of it",()=>{
 const ids=new Set([...policy.projects.map(project=>project.id),policy.fallback.id]);
 expect(ids.size).toBe(policy.projects.length+1);
 expect(policy.projects[0]!.id).toBe("library");
 for(const entry of policy.ambient){expect(existsSync(join(root,entry.declaration)),entry.declaration).toBe(true);for(const id of entry.projects)expect(ids.has(id),id).toBe(true);}
 const next=policy.ambient.find(entry=>entry.id==="next")!;
 expect(next.projects).toEqual(["server-next"]);
 expect(typecheckProjectOfV1(S+"/🎛️coordinator/🎫️ticket/🟦️.ts").id).toBe("server-next");
 expect(typecheckProjectOfV1(S+"/📚️library/🔌️ports/🟦️.ts").id).toBe("server-next");
 expect(typecheckProjectOfV1(S+"/🎛️coordinator/📦️packages/🟦️typescript/app/api/v1/health/route.ts").id).toBe("server-next");
 expect(typecheckProjectOfV1(S+"/🎛️coordinator/🏗️builder/🛤️task-routes/🟦️.ts").id).toBe("server-build");
 expect(typecheckProjectOfV1(S+"/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts").id).toBe("server-build");
 expect(typecheckProjectOfV1(S+"/🎛️coordinator/🧪️tests/🔬️schema/🟦️.ts").id).toBe("server-build");
 expect(typecheckProjectOfV1(policy.projects[0]!.roots[0]+"/🏗️build/🟦️.ts").id).toBe("library");
});

test("AMBIENT LAW programs: no program that `repo typecheck` composes for the product (the build library, the CLI workspace, the server build tooling) and is not allowed to load an ambient-global package reads its global declaration, and the allowed program does",scratch(async directory=>{
 const product="🧰️framework/🛍️products/🦑️repo",plan=planTypecheckV1(root,resolveTypecheckPathsV1(root,root,[product]),{noImporters:true},directory);
 const programFiles=(id:string,project:Readonly<{id:string;tsconfig:string}>,files:readonly string[])=>new Promise<string[]>(finish=>{
  const configuration=join(directory,id+".law.tsconfig.json");
  writeFileSync(configuration,typecheckConfigurationV1(root,project,files,directory));
  const child=spawn("node",[join(root,policy.compiler.script),"-p",configuration,"--listFilesOnly"],{cwd:root,stdio:["ignore","pipe","pipe"],env:{...process.env,NO_COLOR:"1",NODE_OPTIONS:"--max-old-space-size=4096"}});
  let out="",err="";
  child.stdout.on("data",chunk=>{out+=chunk;});
  child.stderr.on("data",chunk=>{err+=chunk;});
  child.on("close",status=>{expect(status,id+" "+err).toBe(0);finish(out.split(/\r?\n/u).filter(Boolean).map(file=>relative(root,file).split("\\").join("/")));});
 });
 expect([...plan.groups.keys()].sort()).toEqual([...policy.projects.map(row=>row.id),policy.fallback.id].sort());
 for(const [id,group] of plan.groups){
  const files=await programFiles(id,group.project,group.files);
  for(const entry of policy.ambient)expect(files.some(file=>file.endsWith("/"+entry.declaration)),`${id} loads ${entry.declaration}`).toBe(entry.projects.includes(id));
 }
}),600000);
