/** 🧬 Cross-check focused loss-selector contracts without weakening physical vocabularies. */
import assert from "node:assert/strict";
import crypto from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";
import {compileJsonSchemaSubset} from "@semio-tech/schema-validator-ts";

const directory=path.dirname(fileURLToPath(import.meta.url)),modelRoot=path.resolve(directory,"../..");
const require=createRequire(import.meta.url),Ajv=require("ajv/dist/2020").default;
const read=(file:string)=>JSON.parse(fs.readFileSync(file,"utf8"));
const schemaPath=path.join(modelRoot,"🧬️schema/🔣️.schema.json"),dataPath=path.join(modelRoot,"🔣️.json");
const schema=read(schemaPath),data=read(dataPath),fixture=read(path.join(directory,"🔣️.json"));
const hash=(file:string)=>crypto.createHash("sha256").update(fs.readFileSync(file)).digest("hex");
const before=[hash(schemaPath),hash(dataPath)],ajv=new Ajv({strict:true,allErrors:true});
const full=ajv.compile(schema),fullValid=full(data);
assert.deepEqual(schema.$defs.ifc_class.enum,fixture.physical_classes);
assert.deepEqual(schema.$defs.type.properties.class.enum,fixture.physical_type_classes);
const scopes={selector:schema.properties.translation.properties.losses.items.properties.applies_to.properties.class,
 element:schema.$defs.ifc_class,type:schema.$defs.type.properties.class};
const results=[];
for(const test of fixture.cases){
 const contract=scopes[test.scope as keyof typeof scopes],repository=compileJsonSchemaSubset(contract,schema);
 const independent=ajv.compile({$schema:schema.$schema,$defs:schema.$defs,...contract});
 const value=test.from?test.from.reduce((v:any,key:string|number)=>v[key],data):test.value;
 assert.deepEqual(value,test.value,test.id+": authored fixture binding");
 const errors=repository.validate(value),actual=errors.length===0,oracle=independent(value);
 results.push({id:test.id,expected:test.valid,repository:actual,ajv:oracle,errors,ajvErrors:independent.errors});
}
assert.deepEqual([hash(schemaPath),hash(dataPath)],before,"Schema/data moved during selector validation");
console.log(JSON.stringify({cases:results,fullModelAjv:fullValid,fullModelAjvErrors:full.errors,
 schemaSha256:before[0],dataSha256:before[1],repositoryScope:"focused selectors/elements/types; full Draft2020 model checked by Ajv"}));
for(const test of results){assert.equal(test.repository,test.expected,test.id+": repository");assert.equal(test.ajv,test.expected,test.id+": Ajv");}
assert.equal(fullValid,true,JSON.stringify(full.errors));
