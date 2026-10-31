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
