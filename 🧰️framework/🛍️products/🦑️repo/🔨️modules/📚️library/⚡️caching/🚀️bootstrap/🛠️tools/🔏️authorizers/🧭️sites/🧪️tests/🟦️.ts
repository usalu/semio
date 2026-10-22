import{expect,test}from"bun:test";
import Ajv from"ajv";
import{spawnSync}from"node:child_process";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{closureLiteralSitesV1,closurePackageNameV1,closureRefusalTextV1,closureRefusalMessagesV1,closureSpecifierKindV1}from"../🟦️.ts";
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

/** 🧪️ Every refusal code ships both languages and renders the named site. */
test("refusal texts exist in English and German and name the site and the fix",()=>{
 const validate=new Ajv({strict:true}).compile({...schema.$defs.Message,$defs:schema.$defs});for(const[code,row]of Object.entries(closureRefusalMessagesV1))expect(validate(row),code).toBe(true);
 for(const code of Object.keys(closureRefusalMessagesV1)){
  const fields={specifier:"SITE-X",importer:"OWNER-Y",detail:"DETAIL-Z"},english=closureRefusalTextV1(closureRefusalMessagesV1,code,fields,{SEMIO_LANGUAGE:"en"}),german=closureRefusalTextV1(closureRefusalMessagesV1,code,fields,{LANG:"de_CH.UTF-8"}),both=closureRefusalTextV1(closureRefusalMessagesV1,code,fields,{LANG:"C"});
  expect(english,code).toContain("Fix:");expect(german,code).toContain("Behebung:");expect(both,code).toContain(" | ");expect(both,code).toContain(english);expect(both,code).toContain(german);expect(english,code).not.toBe(german);
  for(const text of[english,german])expect(text,code).not.toMatch(/\{\w+\}/u);
 }
 expect(closureRefusalTextV1(closureRefusalMessagesV1,"import",{specifier:"SITE-X",importer:"OWNER-Y"},{SEMIO_LANGUAGE:"en"})).toContain("SITE-X");
});

/** 🧪️ Specifier classes and package names follow Node's module addressing. */
test("specifier classes and package names",()=>{
 const rows:[string,string,string][]=[["node:fs","builtin","node:fs"],["fs/promises","builtin","fs"],["bun:sqlite","builtin","bun:sqlite"],["./a.ts","relative","."],["../b.json","relative",".."],["/abs/c.js","absolute","abs"],["file:///d.js","scheme","file:"],["typescript","package","typescript"],["typescript/lib/x.js","package","typescript"],["@iarna/toml","package","@iarna/toml"],["@scope/pkg/sub/file.js","package","@scope/pkg"]];
 for(const[specifier,kind,name]of rows){expect<string>(closureSpecifierKindV1(specifier),specifier).toBe(kind);if(kind==="package")expect(closurePackageNameV1(specifier),specifier).toBe(name);}
});
