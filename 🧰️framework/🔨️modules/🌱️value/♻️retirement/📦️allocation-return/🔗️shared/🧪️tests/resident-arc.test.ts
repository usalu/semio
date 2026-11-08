import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {semioSchemaAjvV1} from "@semio-tech/framework/schema/oracles/validator";
import law from "./resident-arc.json";
import schema from "./resident-arc-schema.json";
test("resident shared acquisition and release preserve the independent native work and allocator marker",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ?+? work,? nullOutput,? successOutput,? retainedOnDenial").get(law.allocation.work,law.release.work,Number(law.allocation.nullOutput),Number(law.allocation.successOutput),Number(law.release.preservesOnDenial))).toEqual({work:2,nullOutput:0,successOutput:1,retainedOnDenial:1});expect(law.denied.length).toBe(6)}finally{db.close()}});

test("resident native publication preserves the original value and records three bounded metadata actions",()=>{expect(semioSchemaAjvV1({strict:true}).validate(schema,law)).toBe(true);const db=new Database(":memory:");try{expect(db.query("select ?+? work,? value,? reallocates").get(law.publication.uniquenessWork,law.publication.transferWork,law.publication.value,Number(law.publication.reallocates))).toEqual({work:law.publication.work,value:73,reallocates:0})}finally{db.close()}});
