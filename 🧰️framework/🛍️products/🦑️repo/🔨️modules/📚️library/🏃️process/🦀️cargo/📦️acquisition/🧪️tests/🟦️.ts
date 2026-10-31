import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🔣️.json" with {type:"json"};
import {declaredCargoArtifactManifestV1,readRepositoryCargoArtifactAcquisitionV1,readRepositoryCargoArtifactDeclarationV1} from "../🟦️.ts";
import {resolve} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

test("declared Cargo acquisition separates caller selection from repository artifact facts",()=>{
 const validator=new Ajv({strict:true,allErrors:true}).addSchema(schema,"acquisition"),database=new Database(":memory:");
 try{
  expect(validator.validate("acquisition#/definitions/Declaration",fixture.declaration),JSON.stringify(validator.errors)).toBe(true);
  for(const row of fixture.declarations){expect(validateJsonSchemaSubset(schema.definitions.Declaration,row.input).length===0,row.name).toBe(row.valid);expect(validator.validate("acquisition#/definitions/Declaration",row.input),row.name).toBe(row.valid);if(row.valid)expect(readRepositoryCargoArtifactDeclarationV1(row.input)).toEqual(row.input);else expect(()=>readRepositoryCargoArtifactDeclarationV1(row.input),row.name).toThrow();}
  for(const row of fixture.requests){
   const actual=validateJsonSchemaSubset(schema.definitions.Request,row.input).length===0;
   expect(actual,row.name).toBe(row.valid);expect(validator.validate("acquisition#/definitions/Request",row.input),row.name).toBe(row.valid);
   if(row.valid){const admitted=readRepositoryCargoArtifactAcquisitionV1(row.input);expect(admitted).toEqual(row.input);expect(Object.isFrozen(admitted)&&Object.isFrozen(admitted.arguments)).toBe(true);}else expect(()=>readRepositoryCargoArtifactAcquisitionV1(row.input),row.name).toThrow();
   const expected=database.query("SELECT count(*) AS count FROM json_each(?) WHERE key NOT IN ('version','project','target','configuration','arguments')").get(JSON.stringify(row.input)) as {count:number};
   if(expected.count)expect(actual,row.name).toBe(false);
  }
 }finally{database.close();}
});

test("declared manifests remain within the actual project on the current host",()=>{
 expect(declaredCargoArtifactManifestV1(process.cwd(),"owned/project",fixture.declaration)).toBe(resolve(process.cwd(),"owned/project/Cargo.toml"));
 for(const root of["../foreign","C:/foreign","x:y","\\\\server\\share"])expect(()=>declaredCargoArtifactManifestV1(process.cwd(),root,fixture.declaration),root).toThrow();
});
