import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import corpus from"./🔣️.json";
import schema from"./🧬️.json";
test("native UI leaves preserve canonical wire values and reject foreign variants",()=>{const ajv=new Ajv({strict:true});expect(ajv.compile(schema)(corpus)).toBe(true);const db=new Database(":memory:");for(const row of corpus.cases){const value=JSON.parse(row.wire),definition=corpus.definitions[row.family as keyof typeof corpus.definitions];expect(ajv.compile(definition)(value)).toBe(row.accepted);if(row.accepted){const result=db.query("select json(?) actual").get(row.wire)as{actual:string};expect(JSON.parse(result.actual)).toEqual(value)}}db.close()});
