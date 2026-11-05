import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {readFileSync} from "node:fs";

const base=new URL("../../📦️workspaces/",import.meta.url),read=(path:string)=>JSON.parse(readFileSync(new URL(path,base),"utf8")),schema=read("🧬️schema/🔣️.json"),corpus=read("🧫️fixtures/🔣️.json");
test("workspace link admission matches the independent schema validator",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"links");
 expect(ajv.validate({$ref:"links#/$defs/Corpus"},corpus)).toBe(true);
 const name=ajv.compile({$ref:"links#/$defs/PackageName"}),workspace=ajv.compile({$ref:"links#/$defs/WorkspacePath"});
 for(const row of corpus)expect(Boolean(name(row.name)&&workspace(row.workspace)),row.id).toBe(row.valid);
 expect(schema.$defs.Protocol.const.packageNamePattern).toBe(schema.$defs.PackageName.pattern);
 expect(schema.$defs.Protocol.const.reservedNamePattern).toBe(schema.$defs.PackageName.not.pattern);
 const manifest=ajv.compile({$ref:"links#/$defs/WorkspaceManifest"});
 for(const value of [{},{workspaces:null},{workspaces:"p"}])expect(manifest(value)).toBe(false);
 expect(manifest({workspaces:[]})).toBe(true);
});

test("the portable path budget counts UTF-16 units including supplementary characters",()=>{
 const corpus=read("🧫️fixtures/📏️paths.json");expect(new Ajv({strict:true}).validate(schema.$defs.PathCorpus,corpus)).toBe(true);expect(corpus.maximumUnits).toBe(schema.$defs.Protocol.const.maximumPathLength);
 for(const row of corpus.cases){expect(row.unit.length,row.id).toBe(row.units);const exact=row.unit.repeat(Math.floor(corpus.maximumUnits/row.units))+"x".repeat(corpus.maximumUnits%row.units);expect(exact.length).toBe(corpus.maximumUnits);expect((exact+row.unit).length).toBeGreaterThan(corpus.maximumUnits);}
});
