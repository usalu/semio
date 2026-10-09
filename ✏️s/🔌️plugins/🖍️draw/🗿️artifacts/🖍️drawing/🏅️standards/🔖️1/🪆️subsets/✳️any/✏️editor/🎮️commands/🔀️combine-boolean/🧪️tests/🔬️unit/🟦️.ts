/** 🧪️ Ajv validates the same Boolean command fixtures as native admission. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import schema from "../../🧬️schema/🔣️.json";
import fixture from "../../🧫️fixtures/🔣️.json";
test("Boolean command fixture schema oracle",()=>{
 const validate=new Ajv({strict:false}).compile({...schema,$schema:"http://json-schema.org/draft-07/schema#"});
 for(const entry of fixture.cases){const structurallyValid=["union","difference","intersection","xor"].includes(entry.operation)&&new Set(entry.ids).size===entry.ids.length;expect(validate({operation:entry.operation,ids:entry.ids})).toBe(structurallyValid);}
});
