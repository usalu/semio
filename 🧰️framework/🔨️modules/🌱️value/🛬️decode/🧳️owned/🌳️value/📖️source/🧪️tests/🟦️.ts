import{test,expect}from"bun:test";
import Ajv from"ajv/dist/2020";
import toPairs from"lodash/toPairs";
import map from"lodash/map";
import inRange from"lodash/inRange";
import fixture from"./🧫️fixtures/🔣️.json";
import schema from"./📋️schema.json";
/** 👓️ Independent JSON projection observes the same ranked sequence and heterogeneous record entries. */
test("borrowed sequence and record selectors retain their declared original shapes",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);for(const row of fixture.cases){const original=structuredClone(row.input),actual=row.kind==="sequence"?map(row.input as number[],value=>value):toPairs(row.input);expect(actual).toEqual(row.expected);expect(row.input).toEqual(original);}});

test("fixed storage refuses a transfer while returning the exact original value",()=>{expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);for(const row of fixture.storage){expect(inRange(row.index,0,row.capacity)&&!row.occupied).toBe(row.accepted);}});
