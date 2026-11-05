import{expect,test}from"bun:test";
import{existsSync,readFileSync}from"node:fs";
import{resolve,relative,dirname}from"node:path";
import{pathToFileURL}from"node:url";
import ts from"typescript";
import Ajv from"ajv";
import fixture from"./🧫️fixtures/🔣️.json";
import{closureLiteralSitesV1}from"../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧭️sites/🟦️.ts";
const root=process.cwd(),entry=resolve(root,fixture.publicEntry);
test("Storybook discovery declares only its consumed inventory API",()=>{
 const text=readFileSync(resolve(root,fixture.coordinator),"utf8"),imports=closureLiteralSitesV1(fixture.coordinator,text).filter(row=>row.scanned&&row.specifier.includes("/📇️inventory/"));
 expect(imports.map(row=>resolve(root,dirname(fixture.coordinator),row.specifier))).toEqual([entry]);
});
test("narrow inventory entry preserves original owners and complete executable imports",async()=>{
 expect(existsSync(entry)).toBe(true);
 const source=readFileSync(entry,"utf8"),tree=ts.createSourceFile(entry,source,ts.ScriptTarget.Latest,true),exports:string[]=[],owners:string[]=[];
 for(const row of tree.statements)if(ts.isExportDeclaration(row)&&row.moduleSpecifier&&ts.isStringLiteral(row.moduleSpecifier)){owners.push(row.moduleSpecifier.text);if(row.exportClause&&ts.isNamedExports(row.exportClause))exports.push(...row.exportClause.elements.map(value=>value.name.text));}
 expect(new Ajv({strict:true}).validate({type:"array",items:{type:"string"},uniqueItems:true},exports)).toBe(true);
 expect(exports).toEqual(fixture.exports);expect(owners).toEqual(fixture.owners);
 expect(closureLiteralSitesV1(fixture.publicEntry,source).filter(row=>row.scanned).map(row=>row.specifier)).toEqual(owners);
 const actual=await import(pathToFileURL(entry).href),packages=await import(pathToFileURL(resolve(dirname(entry),fixture.owners[0]!)).href),taxonomy=await import(pathToFileURL(resolve(dirname(entry),fixture.owners[1]!)).href);
 expect(actual.discoverPackages).toBe(packages.discoverPackages);expect(actual.readSemioMarkerSubTable).toBe(packages.readSemioMarkerSubTable);expect(actual.loadTaxonomy).toBe(taxonomy.loadTaxonomy);
 expect(relative(root,entry)).toBe(fixture.publicEntry);
});
