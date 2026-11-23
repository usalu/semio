import{expect,test,spyOn}from"bun:test";
import ts from"typescript";
import{TranspilerV1}from"../../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🔬️source/🟦️.ts";
import parserFixture from"./🧫️fixtures/📍️parser.json";
import parserSchema from"./🧬️schema/📍️parser.json";
import Ajv from"ajv";
import{spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{readFileSync}from"node:fs";
import{dirname,join,sep}from"node:path";
import{fileURLToPath}from"node:url";
import{closureLiteralSitesV1,closurePackageNameV1,closureRefusalCatalogPathV1,closureRefusalCatalogTableV1,closureRefusalTextV1,closureRefusalMessagesV1,closureSpecifierKindV1}from"../🟦️.ts";
import{astSitesV1}from"./🔭️oracle/🟦️.ts";
import fixture from"./🧫️fixtures/🔣️.json";
import testSchema from"./🧬️schema/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
const here=dirname(fileURLToPath(import.meta.url)),sorted=(values:Iterable<string>)=>[...new Set(values)].sort();

/** 🧪️ The literal-site extractor never misses a site the TypeScript parser and an independent Python tokenizer find. */
test("literal sites equal the TypeScript parser and the Python token oracle",()=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).validate({...testSchema.$defs.Fixture,$defs:testSchema.$defs},fixture)).toBe(true);
 const python=JSON.parse(spawnSync("python3",["-I",join(here,"🐍️oracle","🐍️.py"),join(here,"🧫️fixtures","🔣️.json")],{encoding:"utf8"}).stdout)as Record<string,{specifiers:string[];computed:number}>;
 for(const row of fixture.cases){
  const ast=astSitesV1(row.path,row.source),literals=sorted(ast.filter(site=>site.form==="literal").map(site=>site.specifier!)),computed=ast.filter(site=>site.form==="computed"&&site.kind!=="base").length,actual=closureLiteralSitesV1(row.path,row.source);
  expect(literals,row.id).toEqual(sorted(row.specifiers));expect(computed,row.id).toBe(row.computed);
  expect(python[row.id],row.id).toEqual({specifiers:sorted(row.specifiers),computed:row.computed});
  const found=sorted(actual.map(site=>site.specifier));for(const specifier of row.specifiers)expect(found,row.id+" misses "+specifier).toContain(specifier);
  for(const site of actual)if(!row.specifiers.includes(site.specifier)){expect(row.extra,row.id+" invents "+site.specifier).toContain(site.specifier);expect(site.lenient,row.id+" "+site.specifier).toBe(true);}
 }
});

/** 🧱️ The site bound is a member of the closure family: a file at the bound keeps every site, one more is refused loudly in the language of the process and never truncated, and the TypeScript parser agrees on the count. */
test("a file of exactly the site bound keeps every site and one more is refused instead of truncated",()=>{
 const bound=schema.$defs.Policy.const.maximumSitesPerFile,make=(count:number)=>Array.from({length:count},(_,index)=>"import \"./s"+index+".ts\";").join("\n");
 const kept=closureLiteralSitesV1("big.ts",make(bound));expect(kept).toHaveLength(bound);expect(new Set(kept.map(site=>site.specifier)).size).toBe(bound);
 expect(astSitesV1("big.ts",make(bound)).filter(site=>site.form==="literal")).toHaveLength(bound);expect(astSitesV1("big.ts",make(bound+1)).filter(site=>site.form==="literal")).toHaveLength(bound+1);
 let failure:unknown;try{closureLiteralSitesV1("big.ts",make(bound+1));}catch(error){failure=error;}
 const message=(failure as Error|undefined)?.message??"";expect(message).toContain("big.ts");expect(message).toContain("maximumSitesPerFile="+bound);expect(message).toMatch(/Fix:|Behebung:/u);expect(message).not.toMatch(/\{\w+\}/u);
 for(const language of["en","de"])expect(closureRefusalTextV1(closureRefusalMessagesV1,"sites-bounded",{importer:"big.ts",detail:"maximumSitesPerFile="+bound},{SEMIO_LANGUAGE:language})).toContain("big.ts");
});

/** 🌐️ Every refusal code ships both languages and renders the named site. */
test("refusal texts exist in English and German and name the site and the fix",()=>{
 const validate=new Ajv({strict:true}).compile({...schema.$defs.Message,$defs:schema.$defs});for(const[code,row]of Object.entries(closureRefusalMessagesV1))expect(validate(row),code).toBe(true);
 for(const code of Object.keys(closureRefusalMessagesV1)){
  const fields={specifier:"SITE-X",importer:"OWNER-Y",detail:"DETAIL-Z"},english=closureRefusalTextV1(closureRefusalMessagesV1,code,fields,{SEMIO_LANGUAGE:"en"}),german=closureRefusalTextV1(closureRefusalMessagesV1,code,fields,{LANG:"de_CH.UTF-8"}),both=closureRefusalTextV1(closureRefusalMessagesV1,code,fields,{LANG:"C"});
  expect(english,code).toContain("Fix:");expect(german,code).toContain("Behebung:");expect(both,code).toContain(" | ");expect(both,code).toContain(english);expect(both,code).toContain(german);expect(english,code).not.toBe(german);
  for(const text of[english,german])expect(text,code).not.toMatch(/\{\w+\}/u);
 }
 expect(closureRefusalTextV1(closureRefusalMessagesV1,"import",{specifier:"SITE-X",importer:"OWNER-Y"},{SEMIO_LANGUAGE:"en"})).toContain("SITE-X");
});

/** 📦️ Specifier classes and package names follow Node's module addressing. */
test("specifier classes and package names",()=>{
 const rows:[string,string,string][]=[["node:fs","builtin","node:fs"],["fs/promises","builtin","fs"],["bun:sqlite","builtin","bun:sqlite"],["./a.ts","relative","."],["../b.json","relative",".."],["/abs/c.js","absolute","abs"],["file:///d.js","scheme","file:"],["typescript","package","typescript"],["typescript/lib/x.js","package","typescript"],["@iarna/toml","package","@iarna/toml"],["@scope/pkg/sub/file.js","package","@scope/pkg"]];
 for(const[specifier,kind,name]of rows){expect<string>(closureSpecifierKindV1(specifier),specifier).toBe(kind);if(kind==="package")expect(closurePackageNameV1(specifier),specifier).toBe(name);}
});

/** 📍️ The catalog path names this very schema file, and the table reader gives its messages only for bytes whose declared hash holds. */
test("the catalog path names the schema file and the table reader refuses altered bytes",()=>{
 const file=join(here,"..","🧬️schema","🔣️.json"),bytes=readFileSync(file),hash=(value:Uint8Array)=>createHash("sha256").update(value).digest("hex"),encoded=bytes.toString("base64"),sha=hash(bytes),bare=Buffer.from("{}"),open=Buffer.from("[");
 expect(file.split(sep).join("/").endsWith("/"+closureRefusalCatalogPathV1),closureRefusalCatalogPathV1).toBe(true);
 expect(closureRefusalCatalogTableV1(encoded,sha,hash)).toEqual(closureRefusalMessagesV1);
 for(const[name,table]of[["wrong hash",closureRefusalCatalogTableV1(encoded,hash(bare),hash)],["absent bytes",closureRefusalCatalogTableV1(undefined,sha,hash)],["absent hash",closureRefusalCatalogTableV1(encoded,undefined,hash)],["not base64",closureRefusalCatalogTableV1("not base64!",sha,hash)],["no messages",closureRefusalCatalogTableV1(bare.toString("base64"),hash(bare),hash)],["not json",closureRefusalCatalogTableV1(open.toString("base64"),hash(open),hash)]] as const)expect(table,name).toBeUndefined();
});

/** 📍️ Parser failures keep their exact original identity and expose the actual Source coordinate. */
test("actual scanner failures preserve engine identity and surface bounded Source coordinates",()=>{
 expect(new Ajv({strict:true}).validate(parserSchema,parserFixture)).toBe(true);
 for(const row of parserFixture.cases){
  const source=ts.createSourceFile(row.path,row.source,ts.ScriptTarget.Latest,true,row.loader==="ts"?ts.ScriptKind.TS:ts.ScriptKind.JS),diagnostics=(source as ts.SourceFile&{parseDiagnostics:readonly ts.Diagnostic[]}).parseDiagnostics;
  expect(diagnostics.length>0,row.path).toBe(row.invalid);
  let original:unknown,caught:unknown,message:string|undefined;const method=row.path.endsWith(".cts")?"scanImports":"scan",implementation=TranspilerV1.prototype[method],witness=spyOn(TranspilerV1.prototype,method).mockImplementation(function(this:TranspilerV1,text:string){try{return implementation.call(this,text)as never;}catch(error){original=error;message=(error as Error).message;throw error;}});
  try{try{closureLiteralSitesV1(row.path,row.source);}catch(error){caught=error;}}finally{witness.mockRestore();}
  if(row.invalid){expect(caught).toBe(original);expect(Object.getPrototypeOf(caught)).toBe(Object.getPrototypeOf(original));const error=caught as Error&{sourcePath:string;sourceLoader:string};expect(error.sourcePath).toBe(row.path);expect(error.sourceLoader).toBe(row.loader);expect(error.message).toStartWith(message!);expect(error.message).toContain(JSON.stringify(row.path));expect(error.message).toContain("loader="+row.loader);expect(error.message.split("\n")).toHaveLength(1);}else expect(caught).toBeUndefined();
 }
});

/** 🪤️ Every specifier the Node type stripper leaves as an import is a site a capture follows, including `import {type A} from "x"`, which the Bun and portable scanners and the TypeScript parser treat as erased; Node keeps the bare `import {} from "x"` and loads it. */
test("inline type-only import and re-export sites are captured as Node loads them",()=>{
 const source=["import {type A} from \"./inline-import.ts\";","import {type B, type C} from \"./inline-many.ts\";","export {type D} from \"./inline-export.ts\";","import {\n type E,\n} from \"./inline-multiline.ts\";","import type {F} from \"./erased-import.ts\";","export type {G} from \"./erased-export.ts\";","import {type H, h} from \"./mixed.ts\";"].join("\n");
 const stripped=spawnSync("node",["--disable-warning=ExperimentalWarning","--input-type=module","-e","import{stripTypeScriptTypes as s}from\"node:module\";process.stdout.write(s(process.argv[1]))","--",source],{encoding:"utf8"}).stdout,loaded=[...stripped.matchAll(/(?:import|export)[^;]*?from\s*"([^"]+)"/gu)].map(match=>match[1]!),found=closureLiteralSitesV1("a.ts",source);
 expect(loaded).toContain("./inline-import.ts");expect(loaded).not.toContain("./erased-import.ts");
 for(const specifier of loaded)expect(found.map(site=>site.specifier),specifier).toContain(specifier);
 for(const specifier of["./erased-import.ts","./erased-export.ts"])expect(found.map(site=>site.specifier)).not.toContain(specifier);
 for(const site of found.filter(row=>/inline/u.test(row.specifier)))expect([site.kind,site.lenient,site.scanned]).toEqual(["static",true,true]);
});
