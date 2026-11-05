import{test,expect}from "bun:test";
import Ajv from "ajv";
import{Database}from "bun:sqlite";
/** 🪞️ Both independent validators keep transparent protocol identifiers scalar. */
test("native transparent wrappers retain the scalar protocol shape",async()=>{
 const fixture=await Bun.file(new URL("🧫️fixtures/transparent.json",import.meta.url)).json(),schema=await Bun.file(new URL("transparent-schema.json",import.meta.url)).json(),validate=new Ajv({strict:true}).compile(schema);
 const db=new Database(":memory:");try{for(const row of fixture.cases){expect(validate(row.input)).toBe(row.accepted);expect((db.query("SELECT CASE WHEN json_type(?1) = 'integer' AND json_extract(?1,'$') >= 0 THEN 1 ELSE 0 END AS admitted").get(JSON.stringify(row.input))as{admitted:number}).admitted===1).toBe(row.accepted);}}finally{db.close()}
});
