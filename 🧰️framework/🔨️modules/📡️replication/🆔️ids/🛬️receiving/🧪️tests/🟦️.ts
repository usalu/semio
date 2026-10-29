import{test,expect}from"bun:test";
import Ajv from"ajv/dist/2020";
import toString from"lodash/toString";
import toArray from"lodash/toArray";
import fixture from"./🔣️.json";
import schema from"./📋️schema.json";
/** 🧬️ Independent text and byte owners observe the same exact schema/hash native wrapper values. */
test("replication identifiers admit exact native text or 32 original octets",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);for(const row of fixture.cases){const accepted=row.kind==="schema"?typeof row.input==="string":Array.isArray(row.input)&&row.input.length===32&&row.input.every(value=>Number.isInteger(value)&&Number(value)>=0&&Number(value)<=255);expect(accepted,row.name).toBe(row.accepted);if(accepted)expect(row.kind==="schema"?toString(row.input):toArray(Buffer.from(row.input as number[])),row.name).toEqual(row.expected);}});
