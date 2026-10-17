/** 📦️ Real empty, short and full native pages retain physical backing until its exact release grant. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import fixture from"../../../../🧪️tests/🧫️fixtures/📦️physical-close/🔣️.json";
import schema from"../../🧬️schema/📦️page.json";
import{testJobPayloadPhysicalClose}from"../../../../🧪️tests/📦️physical-close/🟦️.ts";
test("physical page close preserves all four logical lengths against exact native backing",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(fixture)).toBe(true);expect(validate({...fixture,syntheticOwner:true})).toBe(false);testJobPayloadPhysicalClose();const database=new Database(":memory:");try{for(const row of fixture.cases){expect(database.query("select case when ? >= ? then ? else 0 end as released").get(row.insufficientGrant,fixture.pageBytes,fixture.pageBytes)).toEqual({released:0});expect(database.query("select case when ? >= ? then ? else 0 end as released").get(fixture.pageBytes,fixture.pageBytes,fixture.pageBytes)).toEqual({released:fixture.expected.releasedBytes});}}finally{database.close();}});
